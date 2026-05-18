// AI Provider trait and implementations
// Pluggable AI integration for optional screenshot enrichment

use anyhow::{Context, Result};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use reqwest::Client;

#[async_trait]
pub trait AIProvider: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn is_configured(&self) -> bool;
    fn is_active(&self) -> bool;

    async fn analyze_screenshot(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<AnalysisResult>;

    async fn generate_summary(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<String>;

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

// Registry for managing provider instances
pub struct ProviderRegistry {
    providers: Vec<Box<dyn AIProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Box<dyn AIProvider>) {
        self.providers.push(provider);
    }

    pub fn get_provider(&self, id: &str) -> Option<&dyn AIProvider> {
        self.providers.iter().find(|p| p.id() == id).map(|p| p.as_ref())
    }

    pub fn get_active_provider(&self) -> Option<&dyn AIProvider> {
        self.providers.iter().find(|p| p.is_configured() && p.is_active()).map(|p| p.as_ref())
    }

    pub fn list_providers(&self) -> Vec<ProviderInfo> {
        self.providers
            .iter()
            .map(|p| ProviderInfo {
                id: p.id().to_string(),
                name: p.name().to_string(),
                is_configured: p.is_configured(),
                is_active: p.is_active(),
            })
            .collect()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub is_configured: bool,
    pub is_active: bool,
}

// --- Ollama Provider (local, default) ---

pub struct OllamaProvider {
    pub base_url: String,
    pub model: String,
    pub is_active: bool,
}

impl OllamaProvider {
    pub fn new(base_url: String, model: String, is_active: bool) -> Self {
        Self {
            base_url,
            model,
            is_active,
        }
    }

    // Auto-detect Ollama at localhost:11434
    pub fn auto_detect() -> Option<Self> {
        // Check if Ollama is running at default location
        let client = Client::new();
        let base_url = "http://localhost:11434";

        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                let result = handle.block_on(async {
                    // Send a simple request to check if Ollama is running
                    let response = client
                        .get(format!("{}/api/tags", base_url))
                        .timeout(std::time::Duration::from_secs(2))
                        .send()
                        .await;

                    response.map(|r| r.status().is_success()).unwrap_or(false)
                });

                if result {
                    Some(OllamaProvider::new(
                        base_url.to_string(),
                        "llama2".to_string(),
                        true,
                    ))
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

#[async_trait]
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

    fn is_active(&self) -> bool {
        self.is_active
    }

    async fn analyze_screenshot(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<AnalysisResult> {
        let client = Client::new();

        // Build prompt for screenshot analysis
        let prompt = format!(
            "Analyze this screenshot. OCR text: '{}'. Provide the category, tags (comma-separated), and a brief summary (max 3 sentences). Return JSON format: {{\"category\": \"...\", \"tags\": [\"tag1\", \"tag2\"], \"summary\": \"...\"}}",
            ocr_text
        );

        let response = client
            .post(format!("{}/api/generate", self.base_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
            }))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .context("Failed to send request to Ollama")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Ollama request failed: {}",
                response.status()
            ));
        }

        let body: serde_json::Value =
            response.json().await.context("Failed to parse Ollama response")?;

        if let Some(response_str) = body.get("response").and_then(|v| v.as_str()) {
            let parsed = serde_json::from_str::<serde_json::Value>(response_str)
                .context("Failed to parse AI response as JSON")?;

            let category = parsed.get("category").and_then(|v| v.as_str()).map(|s| s.to_string());
            let tags: Vec<String> = parsed
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|v| v.as_slice())
                .unwrap_or(&[])
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            let summary = parsed.get("summary").and_then(|v| v.as_str()).map(|s| s.to_string());

            return Ok(AnalysisResult {
                category,
                category_source: "ai".to_string(),
                tags,
                summary,
                app_name: None,
            });
        }

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
        let client = Client::new();

        let prompt = format!(
            "Summarize this screenshot in one sentence. OCR text: '{}'",
            ocr_text
        );

        let response = client
            .post(format!("{}/api/generate", self.base_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
            }))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .context("Failed to send request to Ollama")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Ollama request failed: {}",
                response.status()
            ));
        }

        let body: serde_json::Value =
            response.json().await.context("Failed to parse Ollama response")?;

        Ok(body
            .get("response")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    async fn detect_app_name(
        &self,
        screenshot_path: &str,
    ) -> Result<Option<String>> {
        // Ollama doesn't have specific app detection endpoint
        // We'll return None and let the UI show "Unknown"
        Ok(None)
    }
}

// --- OpenAI-Compatible Provider ---

pub struct OpenAICompatibleProvider {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub is_active: bool,
}

impl OpenAICompatibleProvider {
    pub fn new(base_url: String, api_key: String, model: String, is_active: bool) -> Self {
        Self {
            base_url,
            api_key,
            model,
            is_active,
        }
    }
}

#[async_trait]
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

    fn is_active(&self) -> bool {
        self.is_active
    }

    async fn analyze_screenshot(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<AnalysisResult> {
        let client = Client::new();

        // Build base64 encoded image
        let image_data = tokio::fs::read(screenshot_path)
            .await
            .context("Failed to read screenshot file")?;

        let image_b64 = BASE64.encode(&image_data);

        let response = client
            .post(&format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/')))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [{
                    "role": "user",
                    "content": [
                        {"type": "text", "text": format!("Analyze this screenshot. OCR text: '{}'. Provide the category, tags (comma-separated), and a brief summary (max 3 sentences). Return JSON format: {{\"category\": \"...\", \"tags\": [\"tag1\", \"tag2\"], \"summary\": \"...\"}}", ocr_text)},
                        {"type": "image_url", "image_url": {"url": format!("data:image/png;base64,{}", image_b64)}}
                    ]
                }]
            }))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .context("Failed to send request to OpenAI-compatible endpoint")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "OpenAI-compatible request failed: {} - {}",
                status,
                error_text
            ));
        }

        let body: serde_json::Value =
            response.json().await.context("Failed to parse API response")?;

        let content = body
            .get("choices")
            .and_then(|arr| arr.get(0))
            .and_then(|obj| obj.get("message"))
            .and_then(|obj| obj.get("content"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let parsed = serde_json::from_str::<serde_json::Value>(content)
            .context("Failed to parse AI response as JSON")?;

        let category = parsed.get("category").and_then(|v| v.as_str()).map(|s| s.to_string());
        let tags: Vec<String> = parsed
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        let summary = parsed.get("summary").and_then(|v| v.as_str()).map(|s| s.to_string());

        Ok(AnalysisResult {
            category,
            category_source: "ai".to_string(),
            tags,
            summary,
            app_name: None,
        })
    }

    async fn generate_summary(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<String> {
        let client = Client::new();

        // Build base64 encoded image
        let image_data = tokio::fs::read(screenshot_path)
            .await
            .context("Failed to read screenshot file")?;

        let image_b64 = BASE64.encode(&image_data);

        let response = client
            .post(&format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/')))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [{
                    "role": "user",
                    "content": [
                        {"type": "text", "text": format!("Summarize this screenshot in one sentence. OCR text: '{}'", ocr_text)},
                        {"type": "image_url", "image_url": {"url": format!("data:image/png;base64,{}", image_b64)}}
                    ]
                }]
            }))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .context("Failed to send request to OpenAI-compatible endpoint")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "OpenAI-compatible request failed: {} - {}",
                status,
                error_text
            ));
        }

        let body: serde_json::Value =
            response.json().await.context("Failed to parse API response")?;

        Ok(body
            .get("choices")
            .and_then(|arr| arr.get(0))
            .and_then(|obj| obj.get("message"))
            .and_then(|obj| obj.get("content"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    async fn detect_app_name(
        &self,
        screenshot_path: &str,
    ) -> Result<Option<String>> {
        // OpenAI-compatible doesn't have specific app detection
        Ok(None)
    }
}

// --- Anthropic Provider (Claude) ---

pub struct AnthropicProvider {
    pub api_key: String,
    pub model: String,
    pub is_active: bool,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: String, is_active: bool) -> Self {
        Self {
            api_key,
            model,
            is_active,
        }
    }
}

#[async_trait]
impl AIProvider for AnthropicProvider {
    fn id(&self) -> &str {
        "anthropic"
    }

    fn name(&self) -> &str {
        "Anthropic (Claude)"
    }

    fn is_configured(&self) -> bool {
        self.is_active && !self.api_key.is_empty()
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    async fn analyze_screenshot(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<AnalysisResult> {
        let client = Client::new();

        // Build base64 encoded image
        let image_data = tokio::fs::read(screenshot_path)
            .await
            .context("Failed to read screenshot file")?;

        let image_b64 = BASE64.encode(&image_data);

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "max_tokens": 1024,
                "messages": [{
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Analyze this screenshot. OCR text: '{}'. Provide the category, tags (comma-separated), and a brief summary (max 3 sentences). Return JSON format: {{\"category\": \"...\", \"tags\": [\"tag1\", \"tag2\"], \"summary\": \"...\"}}", ocr_text)
                        },
                        {
                            "type": "image",
                            "source": {
                                "type": "base64",
                                "media_type": "image/png",
                                "data": image_b64
                            }
                        }
                    ]
                }]
            }))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "Anthropic request failed: {} - {}",
                status,
                error_text
            ));
        }

        let body: serde_json::Value =
            response.json().await.context("Failed to parse Anthropic response")?;

        let content = body
            .get("content")
            .and_then(|arr| arr.get(0))
            .and_then(|obj| obj.get("text"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let parsed = serde_json::from_str::<serde_json::Value>(content)
            .context("Failed to parse AI response as JSON")?;

        let category = parsed.get("category").and_then(|v| v.as_str()).map(|s| s.to_string());
        let tags: Vec<String> = parsed
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        let summary = parsed.get("summary").and_then(|v| v.as_str()).map(|s| s.to_string());

        Ok(AnalysisResult {
            category,
            category_source: "ai".to_string(),
            tags,
            summary,
            app_name: None,
        })
    }

    async fn generate_summary(
        &self,
        screenshot_path: &str,
        ocr_text: &str,
    ) -> Result<String> {
        let client = Client::new();

        // Build base64 encoded image
        let image_data = tokio::fs::read(screenshot_path)
            .await
            .context("Failed to read screenshot file")?;

        let image_b64 = BASE64.encode(&image_data);

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "max_tokens": 1024,
                "messages": [{
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Summarize this screenshot in one sentence. OCR text: '{}'", ocr_text)
                        },
                        {
                            "type": "image",
                            "source": {
                                "type": "base64",
                                "media_type": "image/png",
                                "data": image_b64
                            }
                        }
                    ]
                }]
            }))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "Anthropic request failed: {} - {}",
                status,
                error_text
            ));
        }

        let body: serde_json::Value =
            response.json().await.context("Failed to parse Anthropic response")?;

        Ok(body
            .get("content")
            .and_then(|arr| arr.get(0))
            .and_then(|obj| obj.get("text"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    async fn detect_app_name(
        &self,
        screenshot_path: &str,
    ) -> Result<Option<String>> {
        // Anthropic doesn't have specific app detection
        Ok(None)
    }
}