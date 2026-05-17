# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Development
npm run tauri dev        # Start frontend (Vite) + Tauri native window together
npm run dev              # Frontend only (no native window)

# Production build — outputs .dmg (macOS) / .exe (Windows)
npm run tauri build

# Frontend-only checks
npm run build            # tsc + vite build
```

There are no test or lint scripts configured yet (`npm test` / `npm run lint` are stubs in README).

## Architecture

SnapSort is a Tauri 2 desktop app: a **Rust backend** exposes typed commands over Tauri IPC; a **React 19 / TypeScript frontend** calls them through typed wrappers.

### Backend (`src-tauri/src/`)

| Module | Role |
|--------|------|
| `lib.rs` | Declares public modules; holds `AppState` (wraps `db::Database`) |
| `commands/` | Tauri `#[command]` handlers — one module per domain (screenshots, batch, ai, settings, etc.) |
| `db/` | Repository layer over `rusqlite`; all SQL lives here |
| `ai/` | `AIProvider` trait + implementations (Ollama, OpenAI-compat, Anthropic) |
| `watcher.rs` | `notify`-based file watcher; detects new .png/.jpg/.jpeg/.webp files |
| `migrations.rs` | Reads `migrations/*.sql` in order; tracks applied versions in `schema_migrations` table |

**Key conventions:**
- Command handlers use `anyhow::Result` for error propagation
- All file I/O is async via `tokio`
- New migrations go in `src-tauri/migrations/` numbered sequentially (e.g., `002_...sql`)
- `category_source` must always be `"ai"` or `"user"` — never omit it

### Database schema (`migrations/001_initial.sql`)

Core table: `screenshots` — stores filepath, dimensions, status, category, tags (JSON array), ocr_text, summary, thumbnail (BLOB).

FTS5 virtual table `screenshots_fts` is kept in sync via triggers. When updating `screenshots`, the triggers handle `screenshots_fts` automatically — do not write to it directly.

Status lifecycle: `detected` → (OCR) → `ocr_complete` → (AI) → `analyzed`.

### Frontend (`src/`)

| Path | Role |
|------|------|
| `src/api/index.ts` | All Tauri IPC calls — typed wrappers around `invoke()`. Every new command needs a stub here |
| `src/stores/` | Zustand stores: `galleryStore`, `batchStore`, `settingsStore` |
| `src/components/` | React components (PascalCase `.tsx` files) |
| `src/styles.css` | Tailwind CSS v4 entry point |

**Key conventions:**
- All `invoke()` calls must go through `src/api/index.ts` — never call `invoke` directly from components
- Tailwind CSS only — no inline styles or CSS modules
- Zustand hooks use `use` prefix (e.g., `useGalleryStore`)

### IPC contract

When adding a new feature end-to-end:
1. Add the SQL (new migration if schema changes)
2. Add db repository method in `src-tauri/src/db/`
3. Add Tauri command in `src-tauri/src/commands/`
4. Register it in `lib.rs` / `main.rs` `tauri::Builder`
5. Add typed wrapper in `src/api/index.ts`
6. Consume from Zustand store, then component

### AI provider pattern

AI is entirely optional. The `AIProvider` trait in `src-tauri/src/ai/` must be implemented for each provider. The app must work fully (OCR, search, batch ops) with no provider configured.

## Reference Docs
- `.baton/docs/orchestrator-prompt.md` — system prompt template for LLMs using baton as orchestrator
- `.baton/docs/adr/` — architecture decision records
