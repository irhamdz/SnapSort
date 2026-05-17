// Watch folder-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn add_watch_folder(path: String) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn remove_watch_folder(path: String) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn get_watch_folders() -> Result<Vec<String>, String> {
    // TODO: Implement
    Ok(vec![])
}

#[tauri::command]
pub async fn detect_default_watch_folders() -> Result<Vec<String>, String> {
    // TODO: Implement
    Ok(vec![])
}