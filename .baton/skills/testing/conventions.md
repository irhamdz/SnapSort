# Testing Conventions — SnapSort

## Rust tests
- Unit tests in the same file as the module (`#[cfg(test)]` at the bottom)
- Integration tests in `src-tauri/tests/`
- Use `rusqlite` with an in-memory database for DB tests

## Frontend tests
- Use Vitest + React Testing Library
- Test user interactions, not implementation details
- Mock Tauri `invoke` in `src/__mocks__/tauri.ts`

## What to test
- Ingestion pipeline state transitions
- FTS5 search correctness
- Batch rename pattern token resolution
- Batch operation DB transaction atomicity
- Screenshot detection heuristic logic

## What NOT to test
- Tauri framework internals
- OS-specific file system behavior (test with a temp dir instead)
