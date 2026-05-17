use crate::watcher::{FileWatcher, WatcherState};
use std::sync::{Arc, Mutex};

/// Application state wrapper
/// TODO: Add database connection in later specs
pub struct AppState {
    // Database connection (to be added in later specs)
    // pub db: db::Database,
    /// Watch folder state (mutable at runtime, shared with background watcher)
    pub watch_state: Arc<Mutex<WatcherState>>,
    /// File watcher (tokio Mutex so the guard is Send across .await in commands)
    pub watcher: Arc<tokio::sync::Mutex<Option<FileWatcher>>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            watch_state: self.watch_state.clone(),
            watcher: self.watcher.clone(),
        }
    }
}

impl serde::Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.watch_state.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for AppState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let watch_state = WatcherState::deserialize(deserializer)?;
        Ok(AppState {
            watch_state: Arc::new(Mutex::new(watch_state)),
            watcher: Arc::new(tokio::sync::Mutex::new(None)),
        })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            watch_state: Arc::new(Mutex::new(WatcherState::new())),
            watcher: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }
}

pub mod commands;
pub mod db;
pub mod ai;
pub mod watcher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert!(true); // Placeholder test
    }

    // Import health_check tests from commands module
    mod health_check_tests {
        use crate::commands::health_check;

        #[test]
        fn test_health_check_returns_success_message() {
            let result = health_check();
            assert!(result.is_ok(), "health_check should return Ok");
            assert_eq!(
                result.unwrap(),
                "SnapSort Tauri 2 shell is running",
                "health_check should return expected success message"
            );
        }

        #[test]
        fn test_health_check_returns_string_type() {
            let result: Result<String, String> = health_check();
            assert!(result.is_ok());
        }

        #[test]
        fn test_health_check_command_registered() {
            let result = health_check();
            assert!(result.is_ok(), "Command should be registered and callable");
        }
    }
}