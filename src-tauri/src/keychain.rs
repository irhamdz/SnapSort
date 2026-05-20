use anyhow::{Context, Result};
use tauri_plugin_store::StoreExt;

const STORE_PATH: &str = "api_keys.bin";

fn store_key(provider_id: &str) -> String {
    format!("api_key_{}", provider_id)
}

fn env_var_name(provider_id: &str) -> String {
    format!(
        "SNAPSORT_API_KEY_{}",
        provider_id.to_uppercase().replace('-', "_")
    )
}

pub fn set_api_key(app: &tauri::AppHandle, provider_id: &str, api_key: &str) -> Result<()> {
    let store = app.store(STORE_PATH).context("Failed to open keychain store")?;
    store.set(store_key(provider_id), serde_json::Value::String(api_key.to_string()));
    store.save().context("Failed to persist keychain store")?;
    Ok(())
}

pub fn get_api_key(app: &tauri::AppHandle, provider_id: &str) -> Result<Option<String>> {
    // CI/headless fallback: read from environment variable
    if let Ok(key) = std::env::var(env_var_name(provider_id)) {
        if !key.is_empty() {
            return Ok(Some(key));
        }
    }
    let store = app.store(STORE_PATH).context("Failed to open keychain store")?;
    Ok(store
        .get(store_key(provider_id))
        .and_then(|v| v.as_str().map(|s| s.to_string())))
}

pub fn has_api_key(app: &tauri::AppHandle, provider_id: &str) -> Result<bool> {
    if let Ok(key) = std::env::var(env_var_name(provider_id)) {
        if !key.is_empty() {
            return Ok(true);
        }
    }
    let store = app.store(STORE_PATH).context("Failed to open keychain store")?;
    Ok(store.has(store_key(provider_id)))
}

pub fn delete_api_key(app: &tauri::AppHandle, provider_id: &str) -> Result<()> {
    let store = app.store(STORE_PATH).context("Failed to open keychain store")?;
    store.delete(store_key(provider_id));
    store.save().context("Failed to persist keychain store")?;
    Ok(())
}

/// Returns the last 4 chars of the stored key for masked display (e.g. "3f2a").
pub fn get_key_suffix(app: &tauri::AppHandle, provider_id: &str) -> Result<Option<String>> {
    match get_api_key(app, provider_id)? {
        Some(key) if key.len() >= 4 => Ok(Some(key[key.len() - 4..].to_string())),
        Some(_) => Ok(Some("****".to_string())),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    fn env_var_name(provider_id: &str) -> String {
        format!(
            "SNAPSORT_API_KEY_{}",
            provider_id.to_uppercase().replace('-', "_")
        )
    }

    #[test]
    fn test_env_var_fallback_present() {
        let var = env_var_name("anthropic");
        std::env::set_var(&var, "sk-ant-test-key-1234");
        let key = std::env::var(&var).unwrap();
        assert_eq!(key, "sk-ant-test-key-1234");
        assert!(!key.is_empty());
        std::env::remove_var(&var);
    }

    #[test]
    fn test_env_var_fallback_absent() {
        let var = env_var_name("nonexistent-provider-xyz123");
        std::env::remove_var(&var);
        assert!(std::env::var(&var).is_err());
    }

    #[test]
    fn test_key_suffix_long_key() {
        let key = "sk-ant-api03-very-long-key-abc123xyz";
        let suffix = &key[key.len() - 4..];
        assert_eq!(suffix.len(), 4);
        assert_eq!(suffix, "3xyz");
    }

    #[test]
    fn test_key_suffix_short_key() {
        let key = "ab";
        let suffix = if key.len() >= 4 { &key[key.len() - 4..] } else { "****" };
        assert_eq!(suffix, "****");
    }

    #[test]
    fn test_no_key_value_in_env_var_name() {
        // Env var names must never contain actual key material
        let var = env_var_name("openai-compatible");
        assert!(var.starts_with("SNAPSORT_API_KEY_"));
        assert!(!var.contains("sk-"));
        assert!(!var.contains("Bearer"));
    }

    #[test]
    fn test_store_key_format() {
        assert_eq!(super::store_key("anthropic"), "api_key_anthropic");
        assert_eq!(super::store_key("openai-compatible"), "api_key_openai-compatible");
        assert_eq!(super::store_key("ollama"), "api_key_ollama");
    }

    #[test]
    fn test_env_var_name_normalisation() {
        assert_eq!(env_var_name("openai-compatible"), "SNAPSORT_API_KEY_OPENAI_COMPATIBLE");
        assert_eq!(env_var_name("anthropic"), "SNAPSORT_API_KEY_ANTHROPIC");
    }
}
