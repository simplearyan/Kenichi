// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn attach_wgpu_renderer() -> bool {
    println!("Backend: WGPU Surface Attached (Simulation)");
    // TODO: Actual WGPU initialization will go here
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, attach_wgpu_renderer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
