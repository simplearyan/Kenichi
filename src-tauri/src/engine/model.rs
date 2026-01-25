use super::KinetixEngine;

// Ensure bytemuck is derived
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub position: [f32; 2], // X, Y
    pub scale: f32,
    pub rotation: f32,
    pub opacity: f32,
    pub _padding: [f32; 3], // Necessary for 16-byte alignment in WGSL
}

impl Default for TransformUniform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            scale: 1.0,
            rotation: 0.0,
            opacity: 1.0,
            _padding: [0.0; 3],
        }
    }
}

impl KinetixEngine {
    pub fn load_video(&mut self, path: &str) {
        println!("Engine: Loading Video: {}", path);

        let (Some(device), Some(queue), Some(layout)) =
            (&self.device, &self.queue, &self.texture_bind_group_layout)
        else {
            println!("Engine: WGPU not ready, skipping load.");
            self.current_file = Some(path.to_string());
            return;
        };

        // 1. Create Decoder
        let mut decoder = match crate::engine::decoding::VideoDecoder::new(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to create decoder: {}", e);
                return;
            }
        };

        println!(
            "Engine: Decoder Created. Size: {}x{}",
            decoder.width(),
            decoder.height()
        );

        // 2. Decode First Frame
        let (frame_data, pts) = match decoder.decode_next_frame() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to decode first frame: {}", e);
                return;
            }
        };

        // 3. Create Texture
        let size = wgpu::Extent3d {
            width: decoder.width(),
            height: decoder.height(),
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Video Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, // Compatible with RGBA pixels
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // 4. Upload Data
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &frame_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * decoder.width()),
                rows_per_image: Some(decoder.height()),
            },
            size,
        );

        // 5. Create View & Sampler
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        // 6. Create Bind Group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Video Bind Group"),
        });

        // 7. Store State
        self.decoder = Some(decoder);
        self.texture_bind_group = Some(bind_group);
        self.video_texture = Some(texture); // Store texture for updates
        self.current_file = Some(path.to_string());
        self.playback_state.current_time = pts; // Sync

        // 8. Force Render
        self.render();
    }

    pub fn play(&mut self) {
        self.playback_state.is_playing = true;
        self.playback_state.last_frame_time = Some(std::time::Instant::now());
    }

    pub fn pause(&mut self) {
        self.playback_state.is_playing = false;
        self.playback_state.last_frame_time = None;
    }

    pub fn seek(&mut self, time: f64) {
        self.playback_state.current_time = time;
        self.playback_state.last_frame_time = Some(std::time::Instant::now());
        self.sync_video_to_time(time, true); // true = Force Seek
    }

    // Helper: Internal seek that doesn't mess with Global Timeline Time
    fn seek_decoder_only(&mut self, time: f64) {
        if let Some(decoder) = &mut self.decoder {
            if let Err(e) = decoder.seek(time) {
                eprintln!("Engine Seek Error: {}", e);
                return;
            }
            if let Ok((frame_data, _)) = decoder.decode_next_frame() {
                self.update_texture(&frame_data);
            }
        }
    }

    // Phase 5b: Sync Engine to Timeline
    fn sync_video_to_time(
        &mut self,
        time: f64,
        force_seek: bool,
    ) -> Option<crate::engine::timeline::ClipData> {
        let active_clip =
            crate::engine::timeline::get_active_clip(time, &self.composition).cloned();
        let clip = active_clip.as_ref()?;

        // 1. Switch File if needed
        let needs_load = match &self.current_file {
            Some(path) => *path != clip.path,
            None => true,
        };

        if needs_load {
            println!("Engine: Switching Clip -> {}", clip.path);
            self.load_video(&clip.path);
            // load_video renders frame 0. We might be at offset 50.
            // So we definitely need to seek if offset > 0.
        }

        // 2. Calculate Media Time
        let media_time = (time - clip.start) + clip.offset;

        // 3. Seek if needed
        // If we just loaded (needs_load), we probably need to seek unless media_time is near 0.
        // If force_seek (User scrub), we always seek.
        if force_seek || (needs_load && media_time > 0.1) {
            self.seek_decoder_only(media_time);
        }

        active_clip
    }

    fn update_texture(&self, data: &[u8]) {
        let (Some(queue), Some(texture)) = (&self.queue, &self.video_texture) else {
            return;
        };

        // Calculate dimensions (assuming texture size matches data)
        let width = texture.width();
        let height = texture.height();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // Upload New Data
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        // Note: Render will happen on next frame request (or we could trigger it here if managed)
    }

    pub fn tick(&mut self, dt: f64) {
        if !self.playback_state.is_playing {
            return;
        }

        // 1. Advance Playhead
        self.playback_state.current_time += dt;
        let current_time = self.playback_state.current_time;

        // 2. Sync to Timeline (Switch clips if needed)
        // We pass force_seek = false because we want smooth playback, not jump cuts
        let active_clip = self.sync_video_to_time(current_time, false);

        if active_clip.is_none() {
            return;
        }

        // 3. Decode Next Frame (Pacing)
        let Some(decoder) = &mut self.decoder else {
            return;
        };

        let now = std::time::Instant::now();
        let last_time = self.playback_state.last_frame_time.unwrap_or(now);
        let elapsed = now.duration_since(last_time).as_secs_f64();

        let fps = decoder.fps;
        let frame_duration = if fps > 0.0 { 1.0 / fps } else { 0.033 };

        if elapsed >= frame_duration {
            match decoder.decode_next_frame() {
                Ok((frame_data, _pts)) => {
                    self.update_texture(&frame_data);
                    self.playback_state.last_frame_time = Some(now);
                }
                Err(_) => {
                    // Start of gap or end of file?
                }
            }
        }
    }
}
