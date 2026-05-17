import { describe, it, expect, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useBatchStore } from './useBatchStore';

describe('useBatchStore', () => {
  beforeEach(() => {
    useBatchStore.getState().reset();
  });

  it('initializes with default state', () => {
    const { result } = renderHook(() => useBatchStore());
    expect(result.current.selection.selectedIds).toEqual([]);
    expect(result.current.selection.isSelectAll).toBe(false);
  });

  it('has expected state shape', () => {
    const { result } = renderHook(() => useBatchStore());
    const state = result.current;
    expect(state).toHaveProperty('selection');
    expect(state).toHaveProperty('toggleSelection');
    expect(state).toHaveProperty('toggleSelectAll');
    expect(state).toHaveProperty('clearSelection');
    expect(state).toHaveProperty('setSelectedIds');
    expect(state).toHaveProperty('setSelectAll');
    expect(state).toHaveProperty('reset');
  });

  it('can toggle selection', () => {
    const { result } = renderHook(() => useBatchStore());
    const id = 'test-id-123';

    act(() => {
      result.current.toggleSelection(id);
    });

    expect(result.current.selection.selectedIds).toContain(id);

    act(() => {
      result.current.toggleSelection(id);
    });

    expect(result.current.selection.selectedIds).not.toContain(id);
  });

  it('can select all and clear selection', () => {
    const { result } = renderHook(() => useBatchStore());

    act(() => {
      result.current.toggleSelectAll();
    });

    expect(result.current.selection.selectedIds).toContain('all');
    expect(result.current.selection.isSelectAll).toBe(true);

    act(() => {
      result.current.clearSelection();
    });

    expect(result.current.selection.selectedIds).toEqual([]);
    expect(result.current.selection.isSelectAll).toBe(false);
  });
});