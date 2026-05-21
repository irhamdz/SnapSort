// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use snapsort::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            let database = snapsort::db::Database::open(app_data_dir)
                .expect("Failed to open database");

            #[cfg(not(mobile))]
            app.manage(AppState::new(database));

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
            snapsort::commands::health_check,
            snapsort::commands::delete_screenshot,
            snapsort::commands::add_watch_folder,
            snapsort::commands::remove_watch_folder,
            snapsort::commands::list_watch_folders,
            snapsort::commands::search_screenshots,
            snapsort::commands::get_status_counts,
            snapsort::commands::retry_partial_analysis,
            snapsort::commands::ai::get_providers,
            snapsort::commands::ai::add_provider,
            snapsort::commands::ai::remove_provider,
            snapsort::commands::ai::test_connection,
            snapsort::commands::ai::set_active_provider,
            snapsort::commands::ai::set_api_key,
            snapsort::commands::ai::has_api_key,
            snapsort::commands::ai::delete_api_key,
            // Batch operations
            snapsort::commands::batch_delete,
            snapsort::commands::batch_categorize,
            snapsort::commands::batch_rename,
            snapsort::commands::batch_tag,
            snapsort::commands::batch_archive,
            snapsort::commands::batch_move,
            snapsort::commands::batch_add_to_collection,
            // Collections
            snapsort::commands::get_collections,
            snapsort::commands::create_collection,
            snapsort::commands::add_to_collection,
            snapsort::commands::remove_from_collection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
