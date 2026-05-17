// AI-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn run_ai_analysis(screenshot_id: String) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn get_ai_analysis_status(screenshot_id: String) -> Result<serde_json::Value, String> {
    // TODO: Implement
    Ok(serde_json::json!(null))
}