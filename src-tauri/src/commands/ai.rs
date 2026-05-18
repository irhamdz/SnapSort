use crate::ai::ProviderInfo;

#[tauri::command]
pub async fn get_providers() -> Result<Vec<ProviderInfo>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn add_provider(_provider: serde_json::Value) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn remove_provider(_id: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn test_connection(_id: String) -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub async fn set_active_provider(_id: String) -> Result<(), String> {
    Ok(())
}
