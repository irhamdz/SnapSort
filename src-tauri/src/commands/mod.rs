use anyhow::Result;

/// Health check command
/// Returns OK if the Tauri shell is running successfully
#[tauri::command]
pub fn health_check() -> Result<String, String> {
    Ok("SnapSort Tauri 2 shell is running".to_string())
}

// Placeholder for AI provider trait and implementations
// TODO: Add AIProvider trait, Ollama, OpenAI-compat, Anthropic implementations in later specs