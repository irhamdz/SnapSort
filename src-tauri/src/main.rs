// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod ai;
mod watcher;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(db::AppState::new())
        .setup(|app| {
            // Initialize database and migrations on startup
            let app_state = app.state::<db::AppState>();
            let db_state = app_state.inner();
            db_state.migrate().expect("Failed to run migrations");

            // Start file watcher
            let watcher = watcher::FileWatcher::new(db_state.clone());
            watcher.start();

            println!("SnapSort started successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}