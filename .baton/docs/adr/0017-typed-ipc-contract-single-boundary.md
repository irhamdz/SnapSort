# ADR-0017: Typed IPC contract as the single frontend/backend boundary

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.5; CLAUDE.md (IPC contract conventions)

## Context

Tauri splits the app across a Rust core and a React UI
(see [ADR-0001](0001-tauri-2-application-framework.md)). Every feature —
gallery, search, batch ops, settings, AI config — must cross this process
boundary. Ad-hoc `invoke()` calls scattered through components would make the
contract implicit, untyped, and impossible to evolve safely.

## Decision

All backend functionality is exposed as Tauri `#[command]` handlers
(`src-tauri/src/commands/`, registered in the `tauri::Builder`) and consumed in
the frontend **only** through typed wrappers in `src/api/index.ts`. Components
never call `invoke()` directly; they go store → API layer → command. Adding a
feature follows a fixed contract path: SQL/migration → db repository → command →
builder registration → typed API wrapper → store → component.

## Consequences

**Positive**

- One enumerable, typed surface for the entire frontend/backend boundary;
  TypeScript types mirror Rust signatures and catch drift at compile time.
- The fixed end-to-end recipe makes the codebase predictable for contributors
  and keeps SQL confined to the repository layer
  (see [ADR-0003](0003-sqlite-rusqlite-embedded-store.md)).

**Negative / Trade-offs**

- Every feature pays boilerplate across ~6 layers, even trivial ones.
- The "no direct `invoke()`" and "all SQL in db/" rules are conventions, not
  compiler-enforced; they require review discipline.
- Type parity between Rust and TypeScript is maintained by hand unless codegen
  is later introduced.
