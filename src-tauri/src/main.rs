// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use snapsort::AppState;
use tauri::Manager;

mod commands;
mod db;
mod ai;
mod watcher;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(not(mobile))]
            app.manage(AppState::new());

            #[cfg(not(mobile))]
            {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    let state = app_handle.state::<AppState>();
                    let mut file_watcher = snapsort::watcher::FileWatcher::new(
                        state.watch_state.clone()
                    );
                    if let Err(e) = file_watcher.initialize().await {
                        eprintln!("Failed to initialize file watcher: {}", e);
                        return;
                    }
                    *state.watcher.lock().await = Some(file_watcher);
                    loop {
                        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::health_check,
            commands::add_watch_folder,
            commands::remove_watch_folder,
            commands::list_watch_folders,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
