use crate::AppState;
use crate::engine::timeline::Clip;

#[tauri::command]
pub async fn add_clip(
    state: tauri::State<'_, AppState>,
    clip: Clip,
) -> Result<(), String> {
    let mut engine = state.engine.lock().await;
    engine.timeline_manager.add_clip(clip);
    Ok(())
}

#[tauri::command]
pub async fn remove_clip(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mut engine = state.engine.lock().await;
    engine.timeline_manager.remove_clip(&id);
    Ok(())
}

// Legacy/Bulk Sync (Optional, but good for "Load Project")
#[tauri::command]
pub async fn update_composition(
    state: tauri::State<'_, AppState>,
    new_clips: Vec<Clip>,
) -> Result<(), String> {
    let mut engine = state.engine.lock().await;
    
    // Naively clear and re-add
    // Real implementation should Diff, but this is safe for now
    engine.timeline_manager.timeline.clips.clear();
    for clip in new_clips {
        engine.timeline_manager.add_clip(clip);
    }
    
    Ok(())
}
