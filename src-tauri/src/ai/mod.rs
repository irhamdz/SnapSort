// AI Provider trait
// Pluggable AI integration for optional screenshot enrichment

use anyhow::Result;

pub trait AIProvider: Send + Sync {
    /// Unique identifier for the provider
    fn id(&self) -> &str;

    /// Provider display name
    fn name(&self) -> &str;

    /// Check if the provider is properly configured
    fn is_configured(&self) -> bool;

    /// Send a screenshot to the AI for analysis
    /// Returns enriched metadata including category, tags, summary, and app name
    async fn analyze_screenshot(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<AnalysisResult>;

    /// Generate a one-line summary of the screenshot
    async fn generate_summary(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<String>;

    /// Detect the application name shown in the screenshot
    async fn detect_app_name(
        &self,
        screenshot_path: &str,
    ) -> Result<Option<String>>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AnalysisResult {
    pub category: Option<String>,
    pub category_source: String, // "ai" or "user"
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub app_name: Option<String>,
}

// Placeholder for Ollama provider
pub struct OllamaProvider {
    pub base_url: String,
    pub model: String,
    pub is_active: bool,
}

impl AIProvider for OllamaProvider {
    fn id(&self) -> &str {
        "ollama"
    }

    fn name(&self) -> &str {
        "Ollama (Local)"
    }

    fn is_configured(&self) -> bool {
        self.is_active
    }

    async fn analyze_screenshot(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<AnalysisResult> {
        // TODO: Implement actual Ollama API call
        Ok(AnalysisResult {
            category: Some("other".to_string()),
            category_source: "ai".to_string(),
            tags: vec![],
            summary: None,
            app_name: None,
        })
    }

    async fn generate_summary(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<String> {
        // TODO: Implement
        Ok(String::new())
    }

    async fn detect_app_name(
        &self,
        screenshot_path: &str,
    ) -> Result<Option<String>> {
        // TODO: Implement
        Ok(None)
    }
}

// Placeholder for OpenAI-compatible provider
pub struct OpenAICompatibleProvider {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub is_active: bool,
}

impl AIProvider for OpenAICompatibleProvider {
    fn id(&self) -> &str {
        "openai-compatible"
    }

    fn name(&self) -> &str {
        "OpenAI Compatible"
    }

    fn is_configured(&self) -> bool {
        self.is_active && !self.api_key.is_empty()
    }

    async fn analyze_screenshot(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<AnalysisResult> {
        // TODO: Implement actual API call
        Ok(AnalysisResult {
            category: Some("other".to_string()),
            category_source: "ai".to_string(),
            tags: vec![],
            summary: None,
            app_name: None,
        })
    }

    async fn generate_summary(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<String> {
        // TODO: Implement
        Ok(String::new())
    }

    async fn detect_app_name(
        &self,
        screenshot_path: &str,
    ) -> Result<Option<String>> {
        // TODO: Implement
        Ok(None)
    }
}