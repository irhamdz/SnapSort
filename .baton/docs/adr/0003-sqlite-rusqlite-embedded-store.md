# ADR-0003: SQLite (via rusqlite) as the embedded data store

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.1, §1.5, §2

## Context

SnapSort is offline-first with no accounts and no cloud sync
(see [ADR-0015](0015-privacy-by-default.md)). All screenshot metadata —
filepath, dimensions, status, category, tags, OCR text, summary, thumbnail —
lives on the user's machine and must survive restarts, support full-text
search, and handle libraries of 10,000+ records with bulk transactional
operations. A zero-setup, embedded store is required: the user installs one
binary, not a database server.

## Decision

Use **SQLite** as the single local store, accessed from Rust via **rusqlite**.
All SQL is confined to a repository layer (`src-tauri/src/db/`). Schema changes
ship as sequentially numbered migration files applied at startup and tracked in
a `schema_migrations` table.

## Consequences

**Positive**

- Zero configuration; one file in the OS app-data directory, trivially
  backed up (see [ADR-0015](0015-privacy-by-default.md)).
- ACID transactions make batch operations atomic
  (see [ADR-0014](0014-batch-operations-first-class-transactional.md)).
- Built-in FTS5 enables local search with no extra service
  (see [ADR-0004](0004-sqlite-fts5-full-text-search.md)).

**Negative / Trade-offs**

- Single-writer concurrency model; the watcher, OCR, and AI pipelines must
  serialize writes or use short transactions to avoid lock contention.
- Migrations are forward-only and must be hand-authored and ordered carefully.
- Centralizing SQL in the repository layer is a discipline, not an enforced
  boundary.
