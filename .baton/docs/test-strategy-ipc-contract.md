# Test Strategy — IPC Contract Coverage

## Overview

This document describes the testing strategy for the typed IPC contract, ensuring that the frontend/backend boundary is properly tested and enforced.

## Test Categories

### 1. Unit Tests

**API Layer Tests** (`src/api/index.test.ts`)
- Tests invokeTyped wrapper functionality
- Tests all command wrappers for gallery, batch, settings, and AI operations
- Tests error propagation from Tauri invoke to API layer

**Store Tests** (`src/stores/*Store.test.ts`)
- Tests Zustand store state management
- Tests store actions and state transitions
- Verifies no derived/computed state in stores (as per conventions)

**Constraint Compliance Tests** (`src/constraints.test.ts`)
- Enforces no CSS modules, inline styles, Redux/MobX, direct invoke() calls
- Verifies single invoke wrapper in src/api/
- Checks for all three store slices
- Validates no inline SQL in Rust commands

### 2. Integration Tests

**IPC Contract Integration Tests** (`src/ipc-contract.integration.test.ts`)
- Verifies end-to-end flow: Component → Store → API → Command
- Tests API layer contract enforcement
- Validates store type imports from API layer
- Tests error propagation through the contract
- Tests batch store state management
- Tests settings store state management
- Tests gallery store state management

### 3. Rust Tests

**Backend Command Tests** (`src-tauri/src/commands/*_test.rs`)
- Tests health_check command
- Tests other backend commands as they're implemented

**Backend Library Tests** (`src-tauri/src/lib_test.rs`)
- Tests AppState creation and initialization

## Coverage Goals

### Frontend Coverage
- **API Layer**: 100% coverage of all invoke wrappers and type exports
- **Stores**: 100% coverage of state management and actions
- **Components**: 100% coverage of component logic (when components exist)

### Backend Coverage
- **Commands**: 100% coverage of all Tauri command handlers
- **Database Layer**: 100% coverage of repository methods
- **AI Provider**: 100% coverage of AI provider trait implementations

## Running Tests

### Development
```bash
# Run all tests
npm test

# Run tests with coverage
npm run test:coverage

# Run specific test files
npm test src/api/index.test.ts
npm test src/stores/useGalleryStore.test.ts
npm test src/ipc-contract.integration.test.ts
npm test src/constraints.test.ts

# Run IPC contract linting
npm run lint:ipc
```

### CI Integration
- All tests must pass in CI
- Coverage must be at or above 100% for frontend and backend
- Constraint tests must pass (lint:ipc)

## Test Patterns

### API Layer Tests
```typescript
// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

import { invoke } from '@tauri-apps/api/core';
import * as api from './index';

describe('API Layer - Gallery Commands', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (invoke as any).mockResolvedValue({ id: '1', path: '/path/1.png' });
  });

  it('getScreenshots should call invoke with correct arguments', async () => {
    const result = await api.getScreenshots();

    expect(invoke).toHaveBeenCalledWith('get_screenshots', undefined);
    expect(result).toHaveLength(1);
  });
});
```

### Store Tests
```typescript
import { renderHook, act } from '@testing-library/react';
import { useGalleryStore } from './useGalleryStore';

describe('useGalleryStore', () => {
  beforeEach(() => {
    useGalleryStore.getState().reset();
  });

  it('initializes with default state', () => {
    const { result } = renderHook(() => useGalleryStore());
    expect(result.current.screenshots).toEqual([]);
  });

  it('should not have derived state', () => {
    const { result } = renderHook(() => useGalleryStore());
    expect(result.current).not.toHaveProperty('filteredScreenshots');
  });
});
```

### Integration Tests
```typescript
describe('IPC Contract Integration Tests', () => {
  beforeEach(() => {
    useGalleryStore.getState().reset();
    useBatchStore.getState().reset();
  });

  it('should allow store actions to call API wrappers', async () => {
    const mockScreenshots = [{ id: '1', path: '/path/1.png' }];
    (invoke as any).mockResolvedValue(mockScreenshots);

    const { result } = renderHook(() => useGalleryStore());

    act(async () => {
      await result.current.loadScreenshots();
    });

    expect(invoke).toHaveBeenCalledWith('get_screenshots', undefined);
  });
});
```

## Constraint Enforcement

### Automated Checks
1. **No direct invoke() calls**: Constraint test greps components and stores
2. **No inline SQL**: Constraint test greps Rust commands for SQL keywords
3. **All API wrappers exist**: Constraint test verifies src/api/index.ts exports

### Manual Review
- Type parity between Rust and TypeScript
- API wrapper consistency with command handlers
- Store actions calling API wrappers (not direct invoke)

## Coverage Reports

Coverage reports are generated automatically with:
- HTML report: `coverage/index.html`
- LCOV report: `coverage/lcov.info` (for CI)
- JSON report: `coverage/coverage-final.json`

### CI Thresholds
- Frontend statements: 100%
- Frontend branches: 100%
- Frontend functions: 100%
- Frontend lines: 100%
- Backend commands: 100%
- Backend repository: 100%

## Test Maintenance

### When Adding New Commands
1. Add unit tests in `src/api/index.test.ts`
2. Add integration tests in `src/ipc-contract.integration.test.ts`
3. Add Rust tests in `src-tauri/src/commands/*_test.rs`
4. Update coverage thresholds if needed

### When Adding New Stores
1. Add unit tests in `src/stores/*Store.test.ts`
2. Add integration tests in `src/ipc-contract.integration.test.ts`
3. Ensure store types are imported from API layer

### When Adding New Features
1. Verify all new code paths have tests
2. Ensure constraint compliance (no direct invoke, no inline SQL)
3. Run full test suite
4. Generate coverage report
5. Verify 100% coverage for modified files

## References

- [IPC Contract Documentation](.baton/docs/ipc-contract.md)
- [ADR-0017: Typed IPC contract](.baton/docs/adr/0017-typed-ipc-contract-single-boundary.md)
- [CLAUDE.md - IPC contract section](../CLAUDE.md#-ipc-contract)