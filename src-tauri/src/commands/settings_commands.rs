// Settings-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn get_settings() -> Result<serde_json::Value, String> {
    // TODO: Implement
    Ok(serde_json::json!({}))
}

#[tauri::command]
pub async fn save_settings(settings: serde_json::Value) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}