# Backend Conventions — SnapSort (Rust/Tauri)

## Project layout
- Tauri commands: `src-tauri/src/commands/`
- Repository layer: `src-tauri/src/db/`  (all SQL goes here, not in commands)
- AI provider trait + implementations: `src-tauri/src/ai/`
- File watcher: `src-tauri/src/watcher.rs`
- DB migrations: `src-tauri/migrations/` (numbered, sequential)

## Error handling
- Use `anyhow::Result` in Tauri command handlers
- Propagate errors with `?`; don't panic or unwrap in command handlers
- Return structured errors to the frontend via Tauri's error serialization

## Database (SQLite / rusqlite)
- All queries go through the repository layer — no inline SQL in commands
- Batch operations must run in a single transaction
- FTS5 virtual table `screenshots_fts` must stay in sync with `screenshots`
- Always record category source: `"ai"` or `"user"` in the `category_source` column

## File operations
- All file I/O is async — use `tokio::fs`
- Never block the Tauri main thread
- Use Tauri path API (`tauri::api::path`) for cross-platform directories
- Screenshot files go to OS Trash (not hard delete) via the `trash` crate

## Screenshot detection heuristics (F-01)
A file must pass: extension check (.png/.jpg/.jpeg/.webp) AND (known directory OR filename pattern OR process metadata OR dimensions ≥ 400×300px)

## AI provider pattern
- Implement the `AIProvider` trait for any new provider
- Providers are optional — the pipeline must function without any configured provider
- API keys stored in OS keychain via `tauri-plugin-store`; never logged
