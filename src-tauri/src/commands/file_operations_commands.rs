// File operations Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn open_in_finder(path: String) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn copy_path_to_clipboard(path: String) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn export_screenshot(
    screenshot_id: String,
    format: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}