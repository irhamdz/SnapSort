-- Migration 003: AI Providers table
--
-- Store provider non-secret configuration (endpoint URL, model name, is_active)
-- API keys are stored in OS keychain via tauri-plugin-store

CREATE TABLE IF NOT EXISTS provider_settings (
    provider_id   TEXT PRIMARY KEY,
    display_name  TEXT NOT NULL,
    base_url      TEXT NOT NULL,
    model         TEXT NOT NULL,
    is_active     INTEGER NOT NULL DEFAULT 0,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Insert default Ollama provider (auto-detected at localhost:11434)
INSERT OR IGNORE INTO provider_settings (provider_id, display_name, base_url, model, is_active)
VALUES ('ollama', 'Ollama (Local)', 'http://localhost:11434', 'llama2', 1);