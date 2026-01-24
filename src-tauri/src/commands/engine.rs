use tauri::{Window, Runtime};
use crate::AppState;

#[tauri::command]
pub async fn attach_wgpu_renderer<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    println!("Requesting WGPU Attachment for Window: {}", window.label());
    
    let mut engine = state.engine.lock().await;
    
    match engine.init_surface(window).await {
        Ok(_) => {
             println!("Backend: WGPU Surface Attached Successfully");
             Ok(true)
        },
        Err(e) => {
            eprintln!("Backend Error: {}", e);
            Err(e.to_string())
        }
    }
}
