use wgpu::{Adapter, Device, Instance, Queue, Surface};

pub mod decoding;
pub mod export_utils;
pub mod model;
pub mod proxy_manager;
pub mod renderer;
pub mod timeline;

pub struct KinetixEngine {
    pub instance: Instance,
    pub surface: Option<Surface<'static>>,
    pub adapter: Option<Adapter>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub render_pipeline: Option<wgpu::RenderPipeline>,
    pub texture_bind_group_layout: Option<wgpu::BindGroupLayout>, // [NEW] Layout for creating texture bind groups
    pub texture_bind_group: Option<wgpu::BindGroup>, // [NEW] Holds the active video frame texture
    pub video_texture: Option<wgpu::Texture>,        // [NEW] The actual texture resource
    pub decoder: Option<crate::engine::decoding::VideoDecoder>, // [NEW] Video Decoder

    // State
    pub current_file: Option<String>,
    pub playback_state: PlaybackState, // [NEW] Track playback
    pub timeline_manager: timeline::TimelineManager, // [NEW] Timeline Manager

    // Config
    pub config: Option<wgpu::SurfaceConfiguration>, // Changed to Option for safety
    pub width: u32,
    pub height: u32,
    pub viewport: Option<[f32; 4]>, // [NEW] x, y, width, height
}

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_time: f64, // Seconds
    pub duration: f64,     // Seconds (Total video length)

    // Internal Sync
    #[serde(skip)]
    pub last_frame_time: Option<std::time::Instant>, // When was the last frame shown? (Wall Clock)
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_time: 0.0,
            duration: 0.0,
            last_frame_time: None,
        }
    }
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
            texture_bind_group_layout: None,
            texture_bind_group: None,
            video_texture: None,
            decoder: None,
            current_file: None,
            playback_state: PlaybackState::default(),
            timeline_manager: timeline::TimelineManager::new(),
            config: None,
            width: 1920,
            height: 1080,
            viewport: None,
        }
    }
}
