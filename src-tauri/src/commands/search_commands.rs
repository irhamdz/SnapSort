// Search-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn search_screenshots(query: String) -> Result<Vec<serde_json::Value>, String> {
    // TODO: Implement FTS5 search
    Ok(vec![])
}

#[tauri::command]
pub async fn search_with_filters(filters: serde_json::Value) -> Result<Vec<serde_json::Value>, String> {
    // TODO: Implement filtered search
    Ok(vec![])
}