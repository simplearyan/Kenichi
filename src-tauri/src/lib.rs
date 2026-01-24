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
            commands::library::load_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
