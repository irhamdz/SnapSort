// Collection-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn get_collections() -> Result<Vec<serde_json::Value>, String> {
    // TODO: Implement
    Ok(vec![])
}

#[tauri::command]
pub async fn create_collection(name: String) -> Result<String, String> {
    // TODO: Implement
    Ok(String::new())
}

#[tauri::command]
pub async fn add_to_collection(
    screenshot_id: String,
    collection_id: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn remove_from_collection(
    screenshot_id: String,
    collection_id: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}