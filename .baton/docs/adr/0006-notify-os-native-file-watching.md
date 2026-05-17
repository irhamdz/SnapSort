# ADR-0006: `notify` crate for OS-native file watching

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.5, §4.2 (F-01), US-060/061

## Context

Zero-friction ingestion is a core design principle: users never manually
import screenshots. The app must detect new files in up to 20 user-configured
Watch Folders, on both macOS and Windows, with low latency and low idle CPU,
while folders are added and removed at runtime. Polling directories would burn
CPU and add detection lag, especially with large folders.

## Decision

Use the Rust **`notify`** crate to subscribe to OS-native filesystem events
(FSEvents on macOS, ReadDirectoryChangesW on Windows). The watcher emits a
candidate event per created file, which then enters the detection heuristics
(see [ADR-0007](0007-heuristic-screenshot-detection.md)).

## Consequences

**Positive**

- Near-real-time detection with negligible idle cost; satisfies the "<10 s to
  analyzed" target budget by removing polling latency.
- One cross-platform API; watch folders can be registered/unregistered
  dynamically as the user edits the list.

**Negative / Trade-offs**

- OS event semantics differ (coalescing, partial-write/rename events); the
  watcher must debounce and confirm the file is fully written before ingesting.
- File-created events do not by themselves prove the file is a screenshot,
  necessitating a separate heuristic stage
  (see [ADR-0007](0007-heuristic-screenshot-detection.md)).
- Some network/virtual filesystems deliver unreliable events; such paths are
  best-effort.
