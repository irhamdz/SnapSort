-- Migration 005: Add key_suffix to provider_settings for masked display
-- Stores the last 4 chars of the API key so the UI can show "••••3f2a"
-- without a keychain round-trip on every load.
ALTER TABLE provider_settings ADD COLUMN key_suffix TEXT;
