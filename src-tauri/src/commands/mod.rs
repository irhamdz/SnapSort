pub mod ai;

use anyhow::Result;
use std::path::PathBuf;
use tauri::State;
use crate::db::{SearchFilters, Screenshot as DbScreenshot};

/// Health check command
#[tauri::command]
pub fn health_check() -> Result<String, String> {
    Ok("SnapSort Tauri 2 shell is running".to_string())
}

/// Add a watch folder to monitor for new screenshot files
#[tauri::command]
pub async fn add_watch_folder(
    path: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(format!("Watch folder does not exist: {}", path_buf.display()));
    }
    if !path_buf.is_dir() {
        return Err(format!("Watch path is not a directory: {}", path_buf.display()));
    }

    let path_str = path_buf.to_string_lossy().to_string();

    {
        let mut ws = state.watch_state.lock().unwrap();
        if ws.watched_folders.contains(&path_str) {
            return Ok(());
        }
        if ws.watched_folders.len() >= ws.max_folders {
            return Err(format!("Maximum of {} watch folders allowed", ws.max_folders));
        }
        ws.watched_folders.push(path_str);
    }

    let watcher_arc = state.watcher.clone();
    tauri::async_runtime::spawn(async move {
        let mut guard = watcher_arc.lock().await;
        if let Some(w) = guard.as_mut() {
            if let Err(e) = w.reinitialize().await {
                eprintln!("Failed to reinitialize watcher: {}", e);
            }
        }
    });

    Ok(())
}

/// Remove a watch folder from monitoring
#[tauri::command]
pub async fn remove_watch_folder(
    path: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    let path_str = path_buf.to_string_lossy().to_string();

    {
        let mut ws = state.watch_state.lock().unwrap();
        if !ws.watched_folders.contains(&path_str) {
            return Err(format!("Path is not being watched: {}", path_buf.display()));
        }
        ws.watched_folders.retain(|f| f != &path_str);
    }

    let watcher_arc = state.watcher.clone();
    tauri::async_runtime::spawn(async move {
        let mut guard = watcher_arc.lock().await;
        if let Some(w) = guard.as_mut() {
            if let Err(e) = w.reinitialize().await {
                eprintln!("Failed to reinitialize watcher: {}", e);
            }
        }
    });

    Ok(())
}

/// List all currently watched folders
#[tauri::command]
pub async fn list_watch_folders(
    state: State<'_, crate::AppState>,
) -> Result<Vec<String>, String> {
    Ok(state.watch_state.lock().unwrap().watched_folders.clone())
}

/// Search screenshots using full-text search with BM25 ranking
#[tauri::command]
pub fn search_screenshots(
    query: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    category: Option<String>,
    tag: Option<String>,
    app_detected: Option<bool>,
    limit: Option<usize>,
    offset: Option<usize>,
    state: State<'_, crate::AppState>,
) -> Result<Vec<DbScreenshot>, String> {
    let filters = crate::db::SearchFilters {
        query,
        start_date,
        end_date,
        category,
        tag,
        app_detected,
        limit,
        offset,
    };

    state.db.search_screenshots(filters)
        .map_err(|e| e.to_string())
}
