// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use snapsort::{AppState, db::Database};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the OS app-data directory using Tauri's path API
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Create the database in the app-data directory
            let db = Database::open(app_data_dir)
                .expect("Failed to open database");

            // Initialize AppState with the database
            #[cfg(not(mobile))]
            app.manage(AppState::new_with_db(db));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}