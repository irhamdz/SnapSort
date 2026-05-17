// Category-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn get_categories() -> Result<Vec<String>, String> {
    // TODO: Implement - return system-defined categories
    Ok(vec![
        "code".to_string(),
        "design".to_string(),
        "document".to_string(),
        "web".to_string(),
        "communication".to_string(),
        "media".to_string(),
        "finance".to_string(),
        "reference".to_string(),
        "system".to_string(),
        "other".to_string(),
    ])
}

#[tauri::command]
pub async fn assign_category(
    screenshot_id: String,
    category: String,
    source: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}