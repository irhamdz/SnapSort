// Gallery commands stub
use anyhow::Result;

#[tauri::command]
pub async fn get_screenshots(_limit: Option<i64>, _offset: Option<i64>) -> Result<Vec<String>> {
    Ok(vec![])
}

#[tauri::command]
pub async fn search_screenshots(_query: String, _limit: Option<i64>) -> Result<Vec<String>> {
    Ok(vec![])
}

#[tauri::command]
pub async fn delete_screenshot(_id: i64) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn batch_delete_screenshots(_ids: Vec<i64>) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn archive_screenshot(_id: i64) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn batch_archive_screenshots(_ids: Vec<i64>) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn update_screenshot_category(
    _id: i64,
    _category: String,
    _category_source: String
) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn batch_update_categories(
    _ids: Vec<i64>,
    _category: String
) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn update_screenshot_tags(_id: i64, _tags: Vec<String>) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn batch_update_tags(_ids: Vec<i64>, _tags: Vec<String>) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn move_screenshot_file(_id: i64, _destination: String) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn batch_move_files(_ids: Vec<i64>, _destination: String) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn add_to_collection(_id: i64, _collection_id: i64) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn batch_add_to_collection(_ids: Vec<i64>, _collection_id: i64) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn rename_screenshot_file(_id: i64, _new_path: String) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn batch_rename_files(_ids: Vec<i64>, _pattern: String) -> Result<()> {
    Ok(())
}