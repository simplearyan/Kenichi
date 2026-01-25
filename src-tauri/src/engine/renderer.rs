use super::KinetixEngine;

impl KinetixEngine {
    pub async fn init_surface<W>(&mut self, window: W) -> anyhow::Result<()>
    where
        W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle + Clone + Send + Sync + 'static,
    {
        println!("Initializing WGPU Surface...");

        // 1. Create Surface
        let surface = self.instance.create_surface(window)?;

        // 2. Request Adapter (High Performance)
        // Treated as Result based on compiler feedback
        let adapter = self.instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|_| anyhow::anyhow!("Failed to find an appropriate adapter"))?; 
            // Note: If it's effectively Option inside Result, this might need tweaking. 
            // But if it's strictly Result<Adapter>, this works.

        println!("GPU Adapter: {:?}", adapter.get_info());

        // 3. Request Device & Queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Kenichi Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: wgpu::MemoryHints::default(),
                    ..Default::default()
                },
            )
            .await?;

        // 4. Configure Surface
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: self.width,
            height: self.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);

        self.surface = Some(surface);
        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);

        // 5. Create Texture Bind Group Layout
        let texture_bind_group_layout = self.device.as_ref().unwrap().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        
        // 6. Create Shader Module
        let shader = self.device.as_ref().unwrap().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Video Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../src/lib/shaders/video.wgsl").into()),
        });

        // 7. Create Pipeline Layout
        let pipeline_layout = self.device.as_ref().unwrap().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Video Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            ..Default::default()
        });

        // 8. Create Render Pipeline
        let pipeline = self.device.as_ref().unwrap().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Video Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // explicit entry point
                buffers: &[], 
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
            multiview_mask: None, 
        });

        self.render_pipeline = Some(pipeline);
        self.texture_bind_group_layout = Some(texture_bind_group_layout);

        // Initial Render
        self.render();

        println!("WGPU Surface Initialized Successfully");
        Ok(())
    }

    pub fn render(&mut self) {
        let (Some(surface), Some(device), Some(queue), Some(pipeline)) = (&self.surface, &self.device, &self.queue, &self.render_pipeline) else {
            return;
        };

        // Get Surface Texture
        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated) => return, // Needs resize
            Err(e) => {
                eprintln!("Render error: {:?}", e);
                return;
            }
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1, // Dark Gray Background
                            g: 0.1,
                            b: 0.1,
                            a: 1.0, 
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(pipeline);
            
            // Bind Texture if available
            if let Some(bind_group) = &self.texture_bind_group {
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.draw(0..6, 0..1); // Draw 6 vertices (2 triangles) for Quad
            }
        }
        
        queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
