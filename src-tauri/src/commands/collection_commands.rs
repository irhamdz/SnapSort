// Collection-related Tauri commands

use crate::db::Collection;
use anyhow::Result;
use tauri::State;

/// List all collections
#[tauri::command]
pub async fn get_collections(state: State<'_, crate::AppState>) -> Result<Vec<Collection>, String> {
    state
        .db
        .list_collections()
        .map_err(|e| format!("Failed to list collections: {}", e))
}

/// Create a new collection
#[tauri::command]
pub async fn create_collection(
    name: String,
    description: Option<String>,
    color: Option<String>,
    state: State<'_, crate::AppState>,
) -> Result<String, String> {
    if name.is_empty() {
        return Err("Collection name cannot be empty".to_string());
    }

    state
        .db
        .create_collection(&name, description.as_deref(), color.as_deref())
        .map_err(|e| format!("Failed to create collection: {}", e))
}

/// Add a screenshot to a collection
#[tauri::command]
pub async fn add_to_collection(
    screenshot_id: String,
    collection_id: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let id = screenshot_id
        .parse::<i64>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    if collection_id.is_empty() {
        return Err("Collection ID cannot be empty".to_string());
    }

    state
        .db
        .batch_add_to_collection(vec![id], &collection_id)
        .map_err(|e| format!("Failed to add screenshot to collection: {}", e))
}

/// Remove a screenshot from a collection
#[tauri::command]
pub async fn remove_from_collection(
    screenshot_id: String,
    collection_id: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let id = screenshot_id
        .parse::<i64>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    if collection_id.is_empty() {
        return Err("Collection ID cannot be empty".to_string());
    }

    state
        .db
        .remove_from_collection(id, &collection_id)
        .map_err(|e| e.to_string())
}