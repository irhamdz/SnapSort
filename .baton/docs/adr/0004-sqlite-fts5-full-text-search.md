# ADR-0004: SQLite FTS5 for local full-text search

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §4.2 (F-12), US-011

## Context

Search is a P0 capability and must work **without AI**: users type natural
language and expect ranked results across OCR text, summary, category,
detected app name, tags, and user notes — with a 300 ms-debounced,
keystroke-driven experience and no network calls. An external search engine
(e.g. Tantivy as a separate index, or a hosted service) would add an operational
component, conflicting with the zero-setup, offline-first, privacy goals.

## Decision

Use **SQLite FTS5** as the search engine. A `screenshots_fts` virtual table is
kept in sync with the `screenshots` table via SQL triggers; results are ranked
by **BM25**. Structured filters (date range, category, tag, app) compose on top
of the text query.

## Consequences

**Positive**

- No additional service or index process — search ships with the database
  (see [ADR-0003](0003-sqlite-rusqlite-embedded-store.md)).
- Trigger-maintained sync keeps the index transactionally consistent with
  writes, including batch operations.
- BM25 relevance ranking satisfies the "ranked, not just date-sorted" requirement.

**Negative / Trade-offs**

- The FTS mirror must **never** be written to directly; all updates go through
  `screenshots` so triggers fire — a correctness-critical convention.
- FTS5 tokenization/stemming is less sophisticated than a dedicated search
  engine; advanced relevance tuning is limited.
- Trigger overhead is paid on every write, including large batch updates.
