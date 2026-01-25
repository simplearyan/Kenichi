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

#[tauri::command]
pub async fn play(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut engine = state.engine.lock().await;
    engine.play();
    Ok(())
}

#[tauri::command]
pub async fn pause(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut engine = state.engine.lock().await;
    engine.pause();
    Ok(())
}

#[tauri::command]
pub async fn seek(time: f64, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut engine = state.engine.lock().await;
    engine.seek(time);
    Ok(())
}

#[tauri::command]
pub async fn get_playback_state(state: tauri::State<'_, AppState>) -> Result<crate::engine::PlaybackState, String> {
    let engine = state.engine.lock().await;
    Ok(engine.playback_state)
}
