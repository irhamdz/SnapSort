// Single-screenshot commands

use tauri::State;

/// Delete a screenshot: move the file to OS Trash, then remove the DB record.
/// File-to-Trash is best-effort (warns on failure); DB deletion is definitive.
#[tauri::command]
pub async fn delete_screenshot(
    id: i64,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let screenshot = state
        .db
        .get_screenshot(id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Screenshot {id} not found"))?;

    if let Err(e) = trash::delete(&screenshot.filepath) {
        tracing::warn!("Failed to move {} to Trash: {e}", screenshot.filepath);
    }

    state.db.delete_screenshot(id).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::Database;
    use std::path::PathBuf;

    fn test_db() -> Database {
        let tmp = std::env::temp_dir().join(format!(
            "snapsort_screenshots_cmd_{}_{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        Database::open(tmp).unwrap()
    }

    #[test]
    fn delete_screenshot_removes_db_record() {
        let db = test_db();
        let id = db
            .insert_screenshot(
                "/tmp/snap_test.png",
                "snap_test.png",
                800,
                600,
                "detected",
                None,
                "ai",
                vec![],
                None,
                None,
                None,
                0,
                0,
                None,
            )
            .unwrap();

        // Verify it exists
        assert!(db.get_screenshot(id).unwrap().is_some());

        // Delete via DB (file won't exist on disk; trash will silently fail)
        db.delete_screenshot(id).unwrap();

        assert!(db.get_screenshot(id).unwrap().is_none());
    }
}
