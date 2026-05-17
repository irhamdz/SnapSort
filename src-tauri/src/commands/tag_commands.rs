// Tag-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn get_tags() -> Result<Vec<String>, String> {
    // TODO: Implement
    Ok(vec![])
}

#[tauri::command]
pub async fn add_tags(
    screenshot_id: String,
    tags: Vec<String>,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn remove_tags(
    screenshot_id: String,
    tags: Vec<String>,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}