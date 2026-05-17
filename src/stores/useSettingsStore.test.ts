import { describe, it, expect, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useSettingsStore } from './useSettingsStore';

describe('useSettingsStore', () => {
  beforeEach(() => {
    useSettingsStore.getState().reset();
  });

  it('initializes with default state', () => {
    const { result } = renderHook(() => useSettingsStore());
    expect(result.current.settings.watchFolders).toEqual([]);
    expect(result.current.settings.defaultCategory).toBe('uncategorized');
    expect(result.current.settings.ocrEnabled).toBe(true);
    expect(result.current.settings.aiEnabled).toBe(false);
    expect(result.current.settings.ocrLanguage).toBe('eng');
    expect(result.current.settings.aiProvider).toBe('ollama');
    expect(result.current.settings.aiModel).toBe('llama3.2');
    expect(result.current.settings.theme).toBe('dark');
  });

  it('has expected state shape', () => {
    const { result } = renderHook(() => useSettingsStore());
    const state = result.current;
    expect(state).toHaveProperty('settings');
    expect(state).toHaveProperty('updateSettings');
    expect(state).toHaveProperty('setWatchFolders');
    expect(state).toHaveProperty('setDefaultCategory');
    expect(state).toHaveProperty('setOcrEnabled');
    expect(state).toHaveProperty('setAiEnabled');
    expect(state).toHaveProperty('setOcrLanguage');
    expect(state).toHaveProperty('setAiProvider');
    expect(state).toHaveProperty('setAiModel');
    expect(state).toHaveProperty('setTheme');
    expect(state).toHaveProperty('reset');
  });

  it('can update settings', () => {
    const { result } = renderHook(() => useSettingsStore());

    act(() => {
      result.current.updateSettings({ ocrEnabled: false });
    });

    expect(result.current.settings.ocrEnabled).toBe(false);
  });

  it('can manage watch folders', () => {
    const { result } = renderHook(() => useSettingsStore());
    const folders = ['/Users/test/Pictures', '/Users/test/Desktop'];

    act(() => {
      result.current.setWatchFolders(folders);
    });

    expect(result.current.settings.watchFolders).toEqual(folders);
  });

  it('can set AI provider', () => {
    const { result } = renderHook(() => useSettingsStore());
    const provider = 'ollama';

    act(() => {
      result.current.updateSettings({ aiProvider: provider });
    });

    expect(result.current.settings.aiProvider).toBe(provider);
  });

  it('resets to default state', () => {
    const { result } = renderHook(() => useSettingsStore());

    act(() => {
      result.current.updateSettings({
        ocrEnabled: false,
        watchFolders: ['/test'],
        aiProvider: 'ollama',
      });
    });

    expect(result.current.settings.ocrEnabled).toBe(false);
    expect(result.current.settings.watchFolders).toEqual(['/test']);
    expect(result.current.settings.aiProvider).toBe('ollama');

    act(() => {
      result.current.reset();
    });

    expect(result.current.settings.ocrEnabled).toBe(true);
    expect(result.current.settings.watchFolders).toEqual([]);
    expect(result.current.settings.aiProvider).toBe('ollama');
  });
});