// AI commands stub
use anyhow::Result;

#[tauri::command]
pub async fn get_providers() -> Result<Vec<String>> {
    Ok(vec![])
}

#[tauri::command]
pub async fn add_provider(_provider: serde_json::Value) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn remove_provider(_id: String) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn test_connection(_id: String) -> Result<bool> {
    Ok(true)
}

#[tauri::command]
pub async fn set_active_provider(_id: String) -> Result<()> {
    Ok(())
}