// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use snapsort::AppState;

mod commands;
mod db;
mod ai;
mod watcher;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize AppState (placeholder for now)
            #[cfg(not(mobile))]
            app.manage(AppState::new());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::health_check
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}