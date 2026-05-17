# IPC Contract — SnapSort

## Overview

The typed IPC contract is the **single frontend/backend boundary** in SnapSort. All backend functionality is exposed as Tauri `#[command]` handlers and consumed from the frontend **only** through typed wrappers in `src/api/index.ts`.

This contract ensures:
- **Type safety**: TypeScript types mirror Rust signatures and catch drift at compile time
- **Predictability**: A fixed end-to-end recipe makes the codebase maintainable
- **Reviewability**: One enumerable surface for the entire boundary

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend Layer                           │
├─────────────────────────────────────────────────────────────────┤
│  Components  →  Stores  →  src/api/index.ts  →  Tauri IPC      │
│  (React)      (Zustand)   (Typed wrappers)       (typed)        │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                        Backend Layer                            │
├─────────────────────────────────────────────────────────────────┤
│  src-tauri/src/commands/     ←  Tauri #[command] handlers       │
│  src-tauri/src/db/           ←  Repository layer (SQL)          │
│  src-tauri/src/main.rs       ←  tauri::Builder registration     │
└─────────────────────────────────────────────────────────────────┘
```

## Fixed Feature Recipe

When adding a new feature end-to-end, follow this contract path:

```
1. SQL (migrations)
   └─ Add schema changes in src-tauri/migrations/*.sql

2. Database Repository
   └─ Add repository method in src-tauri/src/db/*.rs
      (SQL queries remain here, never in commands)

3. Tauri Command Handler
   └─ Add #[command] in src-tauri/src/commands/*.rs
      (use repository methods, no inline SQL)

4. Builder Registration
   └─ Register command in src-tauri/src/main.rs
      (tauri::generate_handler![...])

5. Typed API Wrapper
   └─ Add typed wrapper in src/api/index.ts
      (export function wrapper, calls invokeTyped)

6. Store Consumption
   └─ Update Zustand store in src/stores/*.ts
      (call API wrapper from actions)

7. Component UI
   └─ Update React component
      (call store actions)
```

## Contract Rules

### Frontend (TypeScript / React)

**Rule 1: All invoke() calls go through src/api/index.ts**
- ❌ Bad: `await invoke('command_name', { args })`
- ✅ Good: `import { commandName } from './api'; await commandName(args)`

**Rule 2: No direct type imports from backend**
- ❌ Bad: `import { Screenshot } from '../api/index'` (if types are backend-generated)
- ✅ Good: Types should be co-located in the store or api layer

**Rule 3: Tailwind only — no inline styles**
- ❌ Bad: `style={{ width: '100%' }}`
- ✅ Good: `<div className="w-full">`

### Backend (Rust)

**Rule 1: All SQL lives in the db/ repository layer**
- ❌ Bad: `conn.execute("SELECT * FROM screenshots", ...)` in command
- ✅ Good: Use `db::get_screenshots()` repository method

**Rule 2: Command handlers use anyhow::Result**
- ❌ Bad: `pub fn command() -> Result<String>`
- ✅ Good: `pub fn command() -> Result<String, String>`

**Rule 3: AI provider must implement AIProvider trait**
- ❌ Bad: Direct AI provider usage in commands
- ✅ Good: Use the AIProvider trait abstraction

**Rule 4: category_source must always be recorded**
- ❌ Always record category source as `"ai"` or `"user"`

## Enforced Checks

### TypeScript Constraint Tests (`src/constraints.test.ts`)

The following checks are enforced in CI:

1. **No CSS modules**: No `.module.css` files
2. **No inline styles**: No `style={{}}` in TS/TSX files
3. **No Redux/MobX in stores**: Only Zustand allowed
4. **No direct invoke() calls**: Components and stores must use `src/api/index.ts`
5. **All three store slices exist**: `useGalleryStore`, `useBatchStore`, `useSettingsStore`
6. **Single invoke wrapper**: Only `src/api/index.ts` should export invoke wrappers
7. **No inline SQL in Rust commands**: Raw SQL strings detected in `src-tauri/src/commands/`

### Running Checks

```bash
# Run all constraint tests
npm test

# Run specific IPC contract tests
npm run lint:ipc
```

## Type Parity Strategy

TypeScript types and Rust command signatures are currently maintained by hand. For future evolution:

- **Option A (current)**: Manual type matching — acceptable for initial phase
- **Option B (medium-term)**: Extract types to shared file — useful when multiple consumers
- **Option C (long-term)**: Tauri codegen — full type extraction from Rust to TS

For now, maintain type parity in `src/api/index.ts` exports.

## References

- [ADR-0017: Typed IPC contract as the single frontend/backend boundary](.baton/docs/adr/0017-typed-ipc-contract-single-boundary.md)
- [ADR-0001: Tauri 2 (Rust + WebView) as the application framework](.baton/docs/adr/0001-tauri-2-application-framework.md)
- [CLAUDE.md - IPC contract section](../CLAUDE.md#-ipc-contract)