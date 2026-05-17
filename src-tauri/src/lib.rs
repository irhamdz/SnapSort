// Library entry point for Tauri commands
pub mod commands;
pub mod db;
pub mod ai;
pub mod watcher;

// State manager for database access
pub struct AppState {
    pub db: db::Database,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: db::Database::new().expect("Failed to initialize database"),
        }
    }
}