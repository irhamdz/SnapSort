use anyhow::Result;
use tauri::State;
use crate::AppState;

/// Get thumbnail bytes for a screenshot by ID
#[tauri::command]
pub async fn get_thumbnail(
    id: i64,
    state: State<'_, AppState>,
) -> Result<Option<Vec<u8>>, String> {
    state.db.get_thumbnail(id)
        .map_err(|e| format!("Failed to get thumbnail: {}", e))
}