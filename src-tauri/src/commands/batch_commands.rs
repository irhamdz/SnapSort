// Batch operations Tauri commands

use crate::db::FileOpError;
use anyhow::Result;
use tauri::State;

/// Batch delete screenshots.
/// Files are sent to OS Trash per-file best-effort; failures are reported.
/// DB records are removed atomically after the Trash pass.
#[tauri::command]
pub async fn batch_delete(
    screenshot_ids: Vec<String>,
    state: State<'_, crate::AppState>,
) -> Result<Vec<FileOpError>, String> {
    let id_vec: Vec<i64> = screenshot_ids
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    let mut trash_errors: Vec<FileOpError> = Vec::new();

    // Per-file best-effort: send each file to OS Trash.
    for &id in &id_vec {
        match state.db.get_screenshot(id) {
            Ok(Some(s)) => {
                if let Err(e) = trash::delete(&s.filepath) {
                    tracing::warn!("Failed to move {} to Trash: {e}", s.filepath);
                    trash_errors.push(FileOpError {
                        id,
                        path: s.filepath.clone(),
                        error: e.to_string(),
                    });
                }
            }
            Ok(None) => tracing::warn!("Screenshot {id} not found; skipping Trash step"),
            Err(e) => tracing::warn!("Failed to fetch screenshot {id}: {e}"),
        }
    }

    // Remove DB records atomically regardless of any Trash failures.
    state
        .db
        .batch_delete_records(id_vec)
        .map_err(|e| format!("Failed to delete screenshots from database: {}", e))?;

    Ok(trash_errors)
}

/// Batch categorize screenshots
/// All-or-nothing: all updates succeed or none do
#[tauri::command]
pub async fn batch_categorize(
    screenshot_ids: Vec<String>,
    category: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let id_vec: Vec<i64> = screenshot_ids
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    if category.is_empty() {
        return Err("Category cannot be empty".to_string());
    }

    state
        .db
        .batch_categorize(id_vec, &category, "user")
        .map_err(|e| format!("Failed to categorize screenshots: {}", e))
}

/// Batch rename screenshots
/// Per-file best-effort: successful renames keep result, failed ones reported
#[tauri::command]
pub async fn batch_rename(
    screenshot_ids: Vec<String>,
    pattern: String,
    state: State<'_, crate::AppState>,
) -> Result<Vec<FileOpError>, String> {
    let id_vec: Vec<i64> = screenshot_ids
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    if pattern.is_empty() {
        return Ok(vec![]);
    }

    state
        .db
        .batch_rename_records(id_vec, &pattern)
        .map_err(|e| format!("Failed to rename screenshots: {}", e))
}

/// Batch tag screenshots
/// All-or-nothing: all updates succeed or none do
#[tauri::command]
pub async fn batch_tag(
    screenshot_ids: Vec<String>,
    tags: Vec<String>,
    add: bool,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let id_vec: Vec<i64> = screenshot_ids
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    if tags.is_empty() {
        return Ok(());
    }

    state
        .db
        .batch_tag(id_vec, tags, add)
        .map_err(|e| format!("Failed to tag screenshots: {}", e))
}

/// Batch archive/unarchive screenshots
/// All-or-nothing: all updates succeed or none do
#[tauri::command]
pub async fn batch_archive(
    screenshot_ids: Vec<String>,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let id_vec: Vec<i64> = screenshot_ids
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    // Archive = 1, Unarchive = 0
    let archived = 1;

    state
        .db
        .batch_archive(id_vec, archived)
        .map_err(|e| format!("Failed to archive screenshots: {}", e))
}

/// Batch move screenshots to new directory
/// Per-file best-effort: successful moves keep result, failed ones reported
#[tauri::command]
pub async fn batch_move(
    screenshot_ids: Vec<String>,
    dest_path: String,
    state: State<'_, crate::AppState>,
) -> Result<Vec<FileOpError>, String> {
    let id_vec: Vec<i64> = screenshot_ids
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    if dest_path.is_empty() {
        return Ok(vec![]);
    }

    state
        .db
        .batch_move_records(id_vec, &dest_path)
        .map_err(|e| format!("Failed to move screenshots: {}", e))
}

/// Batch add screenshots to a collection
/// All-or-nothing: all additions succeed or none do
#[tauri::command]
pub async fn batch_add_to_collection(
    screenshot_ids: Vec<String>,
    collection_id: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let id_vec: Vec<i64> = screenshot_ids
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid screenshot ID format: {}", e))?;

    if collection_id.is_empty() {
        return Err("Collection ID cannot be empty".to_string());
    }

    state
        .db
        .batch_add_to_collection(id_vec, &collection_id)
        .map_err(|e| format!("Failed to add screenshots to collection: {}", e))
}