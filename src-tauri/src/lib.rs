use crate::watcher::{FileWatcher, WatcherState};
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub db: Arc<db::Database>,
    pub watch_state: Arc<Mutex<WatcherState>>,
    pub watcher: Arc<tokio::sync::Mutex<Option<FileWatcher>>>,
}

impl AppState {
    pub fn new(db: db::Database) -> Self {
        Self {
            db: Arc::new(db),
            watch_state: Arc::new(Mutex::new(WatcherState::new())),
            watcher: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }
}

pub mod commands;
pub mod db;
pub mod thumbnail;
pub mod ai;
pub mod watcher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_returns_success_message() {
        let result = commands::health_check();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "SnapSort Tauri 2 shell is running");
    }
}
