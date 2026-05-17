# ADR-0014: Batch operations as first-class, transactional features

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.3, §2.1, §4.2 (F-36..F-43), US-090..097

## Context

v1.1 elevated batch delete, categorize, and rename to P0 and made "batch-first"
a design principle: managing 500 screenshots must be as easy as managing 5.
Batch operations are always user-driven and AI-independent. They span both
pure DB changes (categorize, tag, archive, add-to-collection) and on-disk
filesystem changes (delete→Trash, rename, move). Partial failure must not
corrupt the library.

## Decision

Treat batch operations as **first-class features with their own UX surface**
(a Selection Set, a Batch Action Bar, dedicated popovers/modals). Apply all
DB-only batch mutations in a **single SQLite transaction** so they are atomic.
Filesystem batch operations (rename/move/delete) execute per-file, are **not**
rolled back on partial failure, and report a per-file error summary; successful
files keep their result.

## Consequences

**Positive**

- Bulk management is fast and consistent; DB batch writes are all-or-nothing.
- The Selection Set + Action Bar is one reusable interaction model across all
  batch verbs.

**Negative / Trade-offs**

- DB transactions and filesystem operations have **different** failure
  semantics (atomic vs. best-effort partial) — the UI must clearly communicate
  this asymmetry (e.g. per-file rename error report).
- Large batch transactions amplify single-writer contention and trigger cost
  (see [ADR-0003](0003-sqlite-rusqlite-embedded-store.md),
  [ADR-0004](0004-sqlite-fts5-full-text-search.md)).
- Destructive batch actions need strong confirmation guards (typed/explicit
  click, not Enter) — see [ADR-0016](0016-destructive-deletes-route-to-os-trash.md).
