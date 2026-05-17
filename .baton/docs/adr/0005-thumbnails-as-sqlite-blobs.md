# ADR-0005: Store thumbnails as SQLite BLOBs

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §2.1, §4.1 (F-03)

## Context

The gallery virtualizes thousands of cards and must render previews without
loading full-size source images (which may be multi-megabyte 4K captures, and
which can be moved or renamed by the user or by Rules). Thumbnails could live
as a sidecar file cache on disk or as BLOBs inside the database.

## Decision

Generate a downsized thumbnail at ingestion and store it as a **BLOB in the
`screenshots` table** in SQLite.

## Consequences

**Positive**

- The library is fully self-contained in one DB file: backing up or moving the
  database carries the previews with it
  (see [ADR-0003](0003-sqlite-rusqlite-embedded-store.md)).
- Thumbnail availability is decoupled from the source file's location, so
  Batch Move / Batch Rename and external file moves don't break the gallery.
- One storage and consistency model instead of DB-plus-filesystem-cache
  invalidation.

**Negative / Trade-offs**

- The database file grows with the library; thumbnail encoding/size budget
  must be deliberately constrained.
- Large BLOBs increase row size and I/O; thumbnail columns should be excluded
  from list queries that don't render them.
- Regenerating thumbnails (e.g. a quality change) requires a data migration,
  not just clearing a cache directory.
