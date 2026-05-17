// OCR-related Tauri commands

use crate::AppState;

#[tauri::command]
pub async fn get_ocr_text(screenshot_id: String) -> Result<String, String> {
    // TODO: Implement
    Ok(String::new())
}

#[tauri::command]
pub async fn copy_ocr_text_to_clipboard(screenshot_id: String) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}