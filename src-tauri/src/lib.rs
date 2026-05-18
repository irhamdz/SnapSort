// Library entry point for Tauri commands
pub mod commands;
pub mod db;
pub mod ai;
pub mod ocr;
pub mod watcher;
pub mod pipeline;
pub mod thumbnail;

use std::sync::Arc;

pub struct WatchState {
    pub watched_folders: Vec<String>,
    pub max_folders: usize,
}

pub struct AppState {
    pub db: db::Database,
    pub watch_state: Arc<std::sync::Mutex<WatchState>>,
    pub watcher: Arc<tokio::sync::Mutex<Option<watcher::FileWatcher>>>,
}

impl AppState {
    pub fn new(db: db::Database) -> Self {
        Self {
            db,
            watch_state: Arc::new(std::sync::Mutex::new(WatchState {
                watched_folders: Vec::new(),
                max_folders: 10,
            })),
            watcher: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }
}