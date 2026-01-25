use crate::AppState;
use crate::engine::timeline::ClipData;

#[tauri::command]
pub async fn update_composition(
    state: tauri::State<'_, AppState>,
    new_clips: Vec<ClipData>,
) -> Result<(), String> {
    let mut engine = state.engine.lock().await;

    println!("Timeline: Received {} clips.", new_clips.len());
    engine.composition = new_clips;

    Ok(())
}
