use super::KinetixEngine;

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
        let frame_data = match decoder.decode_next_frame() {
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
        self.current_file = Some(path.to_string());

        // 8. Force Render
        self.render();
    }
}
