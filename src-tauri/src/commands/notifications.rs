// Notifications stub
use anyhow::Result;

#[tauri::command]
pub async fn show(_title: String, _body: String) -> Result<()> {
    Ok(())
}