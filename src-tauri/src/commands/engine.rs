#[tauri::command]
pub fn attach_wgpu_renderer() -> bool {
    println!("Backend: WGPU Surface Attached (Simulation)");
    true
}
