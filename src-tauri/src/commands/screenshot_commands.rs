// Screenshot-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn take_screenshot() -> Result<String, String> {
    // TODO: Implement screenshot capture using OS APIs
    Ok("screenshot_path".to_string())
}

#[tauri::command]
pub async fn load_screenshots() -> Result<Vec<serde_json::Value>, String> {
    // TODO: Implement
    Ok(vec![])
}

#[tauri::command]
pub async fn get_screenshot(id: String) -> Result<serde_json::Value, String> {
    // TODO: Implement
    Ok(serde_json::json!(null))
}

#[tauri::command]
pub async fn delete_screenshot(id: String) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn archive_screenshot(
    id: String,
    archived: bool,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn update_screenshot_metadata(
    id: String,
    metadata: serde_json::Value,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}