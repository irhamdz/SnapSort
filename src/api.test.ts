import { describe, it, expect, vi } from 'vitest';
import * as api from './api';

// Mock the Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('API Layer', () => {
  it('exports invokeTyped function', () => {
    expect(api).toBeDefined();
    expect(typeof api.invokeTyped).toBe('function');
  });

  it('has invokeTyped as function', () => {
    expect(typeof api.invokeTyped).toBe('function');
  });

  it('invokeTyped is callable', () => {
    expect(() => api.invokeTyped('test-command')).not.toThrow();
  });

  it('has multiple typed functions', () => {
    const apiKeys = Object.keys(api);
    expect(apiKeys.length).toBeGreaterThan(0);
    expect(apiKeys).toContain('invokeTyped');
    expect(apiKeys).toContain('getScreenshots');
    expect(apiKeys).toContain('searchScreenshots');
  });
});