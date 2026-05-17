// Watcher commands stub
use anyhow::Result;

#[tauri::command]
pub async fn add_watch_folder(_path: String) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn remove_watch_folder(_id: i64) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn get_watch_folders() -> Result<Vec<String>> {
    Ok(vec![])
}