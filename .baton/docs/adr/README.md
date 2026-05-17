# Architecture Decision Records

This directory captures the significant architectural decisions for SnapSort,
reverse-engineered from `SnapSort_PRD.md` v1.1.0.

Each ADR follows the Nygard format: **Context → Decision → Consequences**.
ADRs are immutable once accepted; to change a decision, add a new ADR that
supersedes the old one (and update the `Status` line of the superseded record).

## Status legend

- **Accepted** — the decision is in force.
- **Superseded by ADR-XXXX** — replaced by a later decision.
- **Proposed** — under discussion, not yet binding.

## Index

| ADR | Title | Status |
|---|---|---|
| [0001](0001-tauri-2-application-framework.md) | Tauri 2 (Rust + WebView) as the application framework | Accepted |
| [0002](0002-react-typescript-tailwind-zustand-frontend.md) | React 19 + TypeScript + Tailwind v4 + Zustand frontend stack | Accepted |
| [0003](0003-sqlite-rusqlite-embedded-store.md) | SQLite (via rusqlite) as the embedded data store | Accepted |
| [0004](0004-sqlite-fts5-full-text-search.md) | SQLite FTS5 for local full-text search | Accepted |
| [0005](0005-thumbnails-as-sqlite-blobs.md) | Store thumbnails as SQLite BLOBs | Accepted |
| [0006](0006-notify-os-native-file-watching.md) | `notify` crate for OS-native file watching | Accepted |
| [0007](0007-heuristic-screenshot-detection.md) | Heuristic-based screenshot detection | Accepted |
| [0008](0008-bundled-tesseract-offline-ocr.md) | Bundle Tesseract for offline OCR | Accepted |
| [0009](0009-ai-optional-progressive-enhancement.md) | AI as an optional progressive enhancement (offline-first) | Accepted |
| [0010](0010-pluggable-aiprovider-trait-byok.md) | Pluggable `AIProvider` trait with Bring-Your-Own-Key | Accepted |
| [0011](0011-ingestion-pipeline-state-machine.md) | Ingestion pipeline modeled as an explicit state machine | Accepted |
| [0012](0012-fixed-category-taxonomy-provenance.md) | Fixed system Category taxonomy with provenance | Accepted |
| [0013](0013-api-keys-in-os-keychain.md) | Store provider API keys in the OS keychain | Accepted |
| [0014](0014-batch-operations-first-class-transactional.md) | Batch operations as first-class, transactional features | Accepted |
| [0015](0015-privacy-by-default.md) | Privacy by default — no telemetry, accounts, or cloud sync | Accepted |
| [0016](0016-destructive-deletes-route-to-os-trash.md) | Destructive deletes route to OS Trash | Accepted |
| [0017](0017-typed-ipc-contract-single-boundary.md) | Typed IPC contract as the single frontend/backend boundary | Accepted |
| [0018](0018-tauri-bundler-github-actions-release.md) | Tauri bundler + GitHub Actions release pipeline | Accepted |
