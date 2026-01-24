use wgpu::{Adapter, Device, Instance, Queue, Surface};

pub mod export_utils;
pub mod model;
pub mod proxy_manager;
pub mod renderer;

pub struct KinetixEngine {
    pub instance: Instance,
    pub surface: Option<Surface<'static>>,
    pub adapter: Option<Adapter>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub render_pipeline: Option<wgpu::RenderPipeline>,

    // State
    pub current_file: Option<String>,

    // Config
    pub config: Option<wgpu::SurfaceConfiguration>, // Changed to Option for safety
    pub width: u32,
    pub height: u32,
}

impl KinetixEngine {
    pub fn new() -> Self {
        let instance = Instance::new(&wgpu::InstanceDescriptor::default());

        Self {
            instance,
            surface: None,
            adapter: None,
            device: None,
            queue: None,
            render_pipeline: None,
            current_file: None,
            config: None,
            width: 1920,
            height: 1080,
        }
    }
}
