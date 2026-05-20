use crate::ai::ProviderInfo;
use crate::keychain;
use reqwest::Client;
use tauri::State;

#[tauri::command]
pub async fn get_providers(
    state: State<'_, crate::AppState>,
    app: tauri::AppHandle,
) -> Result<Vec<ProviderInfo>, String> {
    let settings = state.db.list_provider_settings().map_err(|e| e.to_string())?;
    let mut providers = Vec::with_capacity(settings.len());
    for ps in settings {
        let has_key = keychain::has_api_key(&app, &ps.provider_id).unwrap_or(false);
        let key_suffix = if has_key {
            ps.key_suffix
                .or_else(|| keychain::get_key_suffix(&app, &ps.provider_id).ok().flatten())
        } else {
            None
        };
        providers.push(ProviderInfo {
            id: ps.provider_id,
            name: ps.display_name,
            is_configured: ps.is_active && has_key,
            is_active: ps.is_active,
            has_key,
            key_suffix,
        });
    }
    Ok(providers)
}

#[tauri::command]
pub async fn add_provider(
    provider_id: String,
    display_name: String,
    base_url: String,
    model: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    state
        .db
        .upsert_provider_settings(&provider_id, &display_name, &base_url, &model, false)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_provider(
    provider_id: String,
    state: State<'_, crate::AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let _ = keychain::delete_api_key(&app, &provider_id);
    state
        .db
        .delete_provider_settings(&provider_id)
        .map_err(|e| e.to_string())
}

/// Store an API key in the OS keychain. The key is write-only: it is never
/// returned to the frontend. Only the last 4 characters are cached in the DB
/// for masked display.
#[tauri::command]
pub async fn set_api_key(
    provider_id: String,
    api_key: String,
    app: tauri::AppHandle,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    keychain::set_api_key(&app, &provider_id, &api_key).map_err(|e| e.to_string())?;
    let suffix = if api_key.len() >= 4 {
        Some(&api_key[api_key.len() - 4..])
    } else {
        None
    };
    state
        .db
        .update_key_suffix(&provider_id, suffix)
        .map_err(|e| e.to_string())?;
    // Immediately overwrite local binding so the key doesn't linger in scope.
    drop(api_key);
    Ok(())
}

#[tauri::command]
pub async fn has_api_key(
    provider_id: String,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    keychain::has_api_key(&app, &provider_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_api_key(
    provider_id: String,
    app: tauri::AppHandle,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    keychain::delete_api_key(&app, &provider_id).map_err(|e| e.to_string())?;
    state
        .db
        .update_key_suffix(&provider_id, None)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn set_active_provider(
    provider_id: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let all = state.db.list_provider_settings().map_err(|e| e.to_string())?;
    for ps in &all {
        let active = ps.provider_id == provider_id;
        state
            .db
            .upsert_provider_settings(&ps.provider_id, &ps.display_name, &ps.base_url, &ps.model, active)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Test connectivity to the provider using the keychain-stored key.
/// The key is never logged or returned; only the boolean result is.
#[tauri::command]
pub async fn test_connection(
    provider_id: String,
    state: State<'_, crate::AppState>,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let settings = state
        .db
        .get_provider_settings(&provider_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Provider not found: {}", provider_id))?;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    if provider_id == "ollama" {
        let url = format!("{}/api/tags", settings.base_url.trim_end_matches('/'));
        return Ok(client
            .get(&url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false));
    }

    let api_key = keychain::get_api_key(&app, &provider_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No API key configured for this provider".to_string())?;

    let ok = if provider_id == "anthropic" {
        let resp = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": settings.model,
                "max_tokens": 1,
                "messages": [{"role": "user", "content": "hi"}]
            }))
            .send()
            .await;
        match resp {
            Ok(r) => r.status().as_u16() != 401 && r.status().as_u16() != 403,
            Err(_) => false,
        }
    } else {
        let url = format!("{}/v1/models", settings.base_url.trim_end_matches('/'));
        let resp = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await;
        match resp {
            Ok(r) => r.status().as_u16() != 401 && r.status().as_u16() != 403,
            Err(_) => false,
        }
    };

    drop(api_key);
    Ok(ok)
}
