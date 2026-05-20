import { describe, it, expect, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useGalleryStore } from './useGalleryStore';

describe('useGalleryStore', () => {
  beforeEach(() => {
    useGalleryStore.getState().reset();
  });

  it('initializes with default state', () => {
    const { result } = renderHook(() => useGalleryStore());
    expect(result.current.screenshots).toEqual([]);
    expect(result.current.isLoading).toBe(false);
    expect(result.current.error).toBeNull();
  });

  it('has expected state shape', () => {
    const { result } = renderHook(() => useGalleryStore());
    const state = result.current;
    expect(state).toHaveProperty('screenshots');
    expect(state).toHaveProperty('isLoading');
    expect(state).toHaveProperty('error');
    expect(state).toHaveProperty('setScreenshots');
    expect(state).toHaveProperty('setLoading');
    expect(state).toHaveProperty('setError');
    expect(state).toHaveProperty('reset');
  });

  it('has no derived state (as per conventions)', () => {
    const { result } = renderHook(() => useGalleryStore());
    const state = result.current;
    expect(state).not.toHaveProperty('filteredScreenshots');
    expect(state).not.toHaveProperty('searchQuery');
  });

  it('should not update screenshot when ID does not match', () => {
    const { result } = renderHook(() => useGalleryStore());

    // Add two screenshots with different IDs
    act(() => {
      result.current.addScreenshot({
        id: '1',
        filepath: '/path/1.png',
        filename: 'image1.png',
        width: 1920,
        height: 1080,
        status: 'detected',
        category: 'uncategorized',
        category_source: 'user',
        tags: [],
        ocr_text: '',
        summary: '',
        thumbnail: null,
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
      });
    });

    act(() => {
      result.current.addScreenshot({
        id: '2',
        filepath: '/path/2.png',
        filename: 'image2.png',
        width: 1920,
        height: 1080,
        status: 'detected',
        category: 'uncategorized',
        category_source: 'user',
        tags: [],
        ocr_text: '',
        summary: '',
        thumbnail: null,
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
      });
    });

    // Try to update a screenshot with a non-existent ID
    act(() => {
      result.current.updateScreenshot('3', { status: 'ocr_complete' });
    });

    expect(result.current.screenshots[0].status).toBe('detected');
    expect(result.current.screenshots[1].status).toBe('detected');
  });
});