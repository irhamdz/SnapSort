// Watch Folder Commands
// Tauri commands for watch folder management

use crate::db::IngestionRepository;
use anyhow::Result;

#[tauri::command]
pub async fn add_watch_folder(path: String) -> Result<()> {
    todo!("Implement add_watch_folder")
}

#[tauri::command]
pub async fn remove_watch_folder(path: String) -> Result<()> {
    todo!("Implement remove_watch_folder")
}

#[tauri::command]
pub async fn list_watch_folders() -> Result<Vec<String>> {
    todo!("Implement list_watch_folders")
}

#[tauri::command]
pub async fn detect_screenshots() -> Result<usize> {
    todo!("Implement detect_screenshots")
}