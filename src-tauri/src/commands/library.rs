use tauri::State;
use crate::AppState;

#[tauri::command]
pub async fn load_file(path: String, state: State<'_, AppState>) -> Result<String, String> {
    println!("Backend: Loading File: {}", path);
    // Request proxy generation (async)
    // Returns original path if proxy not ready, or proxy path if ready/exists
    let effective_path = state.proxy_manager.ensure_proxy(path.clone()).await;
    
    // For now, we just return the path to Frontend, which might call another command or we trust Frontend to handle it.
    // If the frontend loads the video into the Viewport, it calls `attach_wgpu_renderer`.
    // The engine itself needs to have a method `load_video(path)`.
    {
        let mut engine = state.engine.lock().await;
        engine.load_video(&effective_path);
    }
    
    Ok(effective_path)
}
