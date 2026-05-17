# ADR-0002: React 19 + TypeScript + Tailwind v4 + Zustand frontend stack

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.5

## Context

The frontend renders a virtualized gallery of potentially 10,000+ thumbnails,
a detail panel, batch-selection UX, settings, and an onboarding flow. It must
be approachable to open-source contributors and integrate cleanly with Tauri's
command-based IPC model (see [ADR-0001](0001-tauri-2-application-framework.md)).
State is mostly server-owned (the Rust/SQLite side); the UI holds transient
view state (filters, selection set, modals) plus cached query results.

## Decision

Build the UI with **React 19 + TypeScript**, style exclusively with
**Tailwind CSS v4**, and manage client state with **Zustand** stores
(`galleryStore`, `batchStore`, `settingsStore`).

## Consequences

**Positive**

- Largest OSS talent pool and component ecosystem lowers the contribution bar.
- TypeScript types mirror the Rust IPC contract, catching boundary mismatches
  at compile time.
- Zustand's minimal, hook-based model fits the "fetch via Tauri command, cache
  in a store" pattern without Redux boilerplate or excessive React context.
- Tailwind-only enforces visual consistency and avoids a parallel CSS-module or
  inline-style system.

**Negative / Trade-offs**

- Tailwind v4 is recent; some tooling/plugins may lag.
- Discipline required: all IPC must flow through the typed API layer and all
  styling through Tailwind utilities — conventions, not compiler-enforced.
- Zustand offers little built-in structure; store boundaries must be governed
  by convention to avoid a tangle of cross-store reads.
