// Settings commands stub
use anyhow::Result;

#[tauri::command]
pub async fn get_settings() -> Result<serde_json::Value> {
    Ok(serde_json::json!({}))
}

#[tauri::command]
pub async fn save_settings(_settings: serde_json::Value) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn get_categories() -> Result<Vec<String>> {
    Ok(vec![])
}