// Batch operations Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn batch_delete(screenshot_ids: Vec<String>) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn batch_categorize(
    screenshot_ids: Vec<String>,
    category: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn batch_rename(
    screenshot_ids: Vec<String>,
    pattern: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn batch_tag(
    screenshot_ids: Vec<String>,
    tags: Vec<String>,
    add: bool,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn batch_archive(screenshot_ids: Vec<String>) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn batch_move(
    screenshot_ids: Vec<String>,
    dest_path: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn batch_add_to_collection(
    screenshot_ids: Vec<String>,
    collection_id: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}