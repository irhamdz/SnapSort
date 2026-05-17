// Screenshot commands stub
use anyhow::Result;

#[tauri::command]
pub async fn detect_screenshots() -> Result<Vec<String>> {
    Ok(vec![])
}

#[tauri::command]
pub async fn create_thumbnail(_file_path: String) -> Result<String> {
    Ok(String::new())
}

#[tauri::command]
pub async fn get_thumbnail(_id: i64) -> Result<Option<String>> {
    Ok(None)
}