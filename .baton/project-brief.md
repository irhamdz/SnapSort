# Project Brief — SnapSort

Project: SnapSort
Language: Rust (backend/Tauri commands), TypeScript/React (frontend)
Framework: Tauri 2, React 19, Tailwind CSS v4, Zustand, SQLite (rusqlite + FTS5), Tesseract OCR

## Overview

SnapSort is an offline-first, open-source desktop app for macOS and Windows that automatically
detects new screenshots and surfaces them through a searchable library. AI enrichment is fully
optional — every core feature (OCR, batch operations, search) works without any AI provider.

## Tech Stack

| Layer          | Choice                             |
|----------------|------------------------------------|
| App framework  | Tauri 2 (Rust + WebView)           |
| Frontend       | React 19, TypeScript, Tailwind v4  |
| State          | Zustand                            |
| Database       | SQLite via rusqlite + FTS5         |
| File watching  | notify crate (Rust)                |
| OCR            | Tesseract (bundled, offline)       |
| AI (optional)  | Provider trait — Ollama, OpenAI, Anthropic |
| Packaging      | Tauri bundler + GitHub Actions     |

## Key Conventions

### Rust (backend)
- All Tauri commands live in `src-tauri/src/commands/`
- Database access goes through a repository layer, not inline SQL in commands
- Use `anyhow::Result` for error propagation in Tauri commands
- AI provider integrations must implement the `AIProvider` trait
- File operations always use async; never block the Tauri main thread

### TypeScript / React (frontend)
- Component files: PascalCase `.tsx`
- Hooks: `use` prefix, camelCase
- Zustand stores: one file per domain slice under `src/stores/`
- Tailwind only — no inline styles, no CSS modules
- All Tauri IPC calls go through typed wrappers in `src/api/`

### Database
- Schema migrations are numbered and live in `src-tauri/migrations/`
- FTS5 virtual table: `screenshots_fts` — keep in sync with the `screenshots` table
- Category source must always be recorded: `"ai"` or `"user"`

### General
- Privacy by default: no telemetry, no network calls except user-configured AI providers
- Never hard-code paths; use Tauri's path API for cross-platform directories
- Batch operations must execute in a single DB transaction

## Current Focus / Priority

Phase 1 — P0 features:
- F-01 Automatic screenshot detection (file watcher + heuristics)
- F-02 Ingestion pipeline (state machine: detected → queued → OCR → ready/enriched)
- F-03 Thumbnail generation
- F-04 SQLite storage & indexing
- F-05 Searchable gallery view
- F-06 Detail panel
- F-11 OCR text extraction (Tesseract bundled)
- F-17 System tray
- F-18 Watch folder management
- F-27 Onboarding flow
- F-36–F-39 Batch selection, delete, categorize, rename (P0 batch ops)

## Baton CLI (orchestrator must use these)

**Dispatch:**      `baton run --spec .baton/specs/<task>.yaml --task-id <id>`
**Read output:**   `baton result <id> --output`  (ALWAYS read after task completes)
**Wait:**          `baton wait <id1> <id2>`  (block until done, use for parallel tasks)
**Blocked tasks:** `baton result <id> --clarify-context`
                   → then `baton respond <id> --answer "..." --resume`
                   → or   `baton escalate <id> --reason "..."`
**Park:**          `baton defer <id>`
**Kill:**          `baton kill <id>`
**Cost:**          `baton cost`
**Pipeline:**      `baton pipeline run .baton/specs/<task>.yaml`
**Status:**        `baton status`

## Agent Workflow

- **Orchestrator** (claude-code / claude-opus-4-6): Plans, designs, reviews, delegates via baton
- **Frontend workers** (opencode / kimi-k2.5): React components, Tailwind UI, TypeScript hooks
- **Backend workers** (opencode / deepseek-chat-v3-0324): Rust Tauri commands, SQLite, OCR integration
- **Test workers** (opencode / gemini-flash-1.5): Unit tests, integration tests, boilerplate
