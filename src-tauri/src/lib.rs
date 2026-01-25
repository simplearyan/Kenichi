pub mod commands;
pub mod engine;

use commands::engine::*;
use commands::export::*;

use engine::proxy_manager::ProxyManager;
use engine::KinetixEngine;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub engine: Arc<Mutex<KinetixEngine>>,
    pub proxy_manager: ProxyManager,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = Arc::new(Mutex::new(KinetixEngine::new()));
    let proxy_manager = ProxyManager::new();

    // Spawn Render Loop (60 FPS)
    let loop_engine = engine.clone();
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(16));
        let mut last_time = std::time::Instant::now();

        loop {
            interval.tick().await;
            
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_time).as_secs_f64();
            last_time = now;

            let mut engine_guard = loop_engine.lock().await;
            engine_guard.tick(dt); 
            // Force render effectively happens inside tick() -> seek() -> update_texture() -> render()
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            engine,
            proxy_manager,
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            attach_wgpu_renderer,
            export_video,
            commands::library::load_file,
            commands::timeline::update_composition,
            play,
            pause,
            seek,
            get_playback_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
