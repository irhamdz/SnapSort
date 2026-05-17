# ADR-0013: Store provider API keys in the OS keychain

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §4.2 (F-16), US-051, US-052

## Context

Cloud providers (OpenAI, Anthropic, custom endpoints) require API keys that are
sensitive credentials with real billing exposure. The PRD mandates that keys
are never written to logs and never shown in the UI after entry. Storing keys
in the SQLite database or a plaintext settings file would leak them via backups
(the DB is intentionally user-accessible — see
[ADR-0003](0003-sqlite-rusqlite-embedded-store.md)) and DB inspection.

## Decision

Store provider API keys in the **OS-native secure credential store** — macOS
Keychain / Windows Credential Manager — via `tauri-plugin-store`. Keys are
write-only from the UI's perspective: entered once, masked thereafter, and
never echoed back or logged. Non-secret settings remain in the regular settings
profile.

## Consequences

**Positive**

- Secrets get OS-level encryption and access control, isolated from the
  user-inspectable database and from log output.
- "Test Connection" can use the stored key without ever surfacing it to the UI
  layer.

**Negative / Trade-offs**

- Keychain behavior differs across platforms (prompts, access scopes); both
  must be tested, and headless/CI contexts need a fallback path.
- Backing up the SQLite DB does **not** carry credentials; users must re-enter
  keys on a new machine — an intentional security/usability trade-off.
- Adds a platform-integration dependency surface (the keychain plugin).
