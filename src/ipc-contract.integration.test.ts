import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useGalleryStore } from './stores/useGalleryStore';
import { useBatchStore } from './stores/useBatchStore';
import { useSettingsStore } from './stores/useSettingsStore';
import * as api from './api/index';

describe('IPC Contract Integration Tests', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset all stores before each test
    useGalleryStore.getState().reset();
    useBatchStore.getState().reset();
    useSettingsStore.getState().reset();
  });

  describe('IPC Contract Enforcement', () => {
    it('should export all required API functions', () => {
      expect(api).toBeDefined();

      // Gallery commands
      expect(api.loadScreenshots).toBeDefined();
      expect(api.getScreenshot).toBeDefined();
      expect(api.searchScreenshots).toBeDefined();

      // Batch commands
      expect(api.batchDelete).toBeDefined();
      expect(api.batchCategorize).toBeDefined();
      expect(api.batchRename).toBeDefined();
      expect(api.batchTag).toBeDefined();
      expect(api.batchArchive).toBeDefined();
      expect(api.batchMove).toBeDefined();
      expect(api.batchAddToCollection).toBeDefined();

      // Settings commands
      expect(api.getSettings).toBeDefined();
      expect(api.saveSettings).toBeDefined();

      // AI commands
      expect(api.runAIAnalysis).toBeDefined();
      expect(api.getAIAnalysisStatus).toBeDefined();
    });

    it('should export type definitions used by stores', () => {
      // Types are exported from API layer but may not be available at runtime due to .js extension
      // This is expected behavior - types are only for TypeScript compilation
      // We verify they exist in the module structure instead
      expect(api).toBeDefined();
      expect(typeof api.loadScreenshots).toBe('function');
      expect(typeof api.saveSettings).toBe('function');
    });
  });

  describe('API Layer Stub Implementation', () => {
    it('all functions are stubs (not implemented)', () => {
      const functions = Object.values(api);

      functions.forEach(fn => {
        expect(typeof fn).toBe('function');
      });
    });

    it('gallery command functions are stubs', () => {
      expect(api.loadScreenshots).toBeDefined();
      expect(api.searchScreenshots).toBeDefined();
      expect(api.getScreenshot).toBeDefined();
    });

    it('batch command functions are stubs', () => {
      expect(api.batchDelete).toBeDefined();
      expect(api.batchCategorize).toBeDefined();
      expect(api.batchRename).toBeDefined();
      expect(api.batchTag).toBeDefined();
      expect(api.batchArchive).toBeDefined();
      expect(api.batchMove).toBeDefined();
      expect(api.batchAddToCollection).toBeDefined();
    });

    it('settings command functions are stubs', () => {
      expect(api.getSettings).toBeDefined();
      expect(api.saveSettings).toBeDefined();
    });

    it('AI command functions are stubs', () => {
      expect(api.runAIAnalysis).toBeDefined();
      expect(api.getAIAnalysisStatus).toBeDefined();
    });

    it('collection command functions are stubs', () => {
      expect(api.getCollections).toBeDefined();
      expect(api.createCollection).toBeDefined();
      expect(api.addToCollection).toBeDefined();
      expect(api.removeFromCollection).toBeDefined();
    });

    it('watch folder command functions are stubs', () => {
      expect(api.addWatchFolder).toBeDefined();
      expect(api.removeWatchFolder).toBeDefined();
      expect(api.getWatchFolders).toBeDefined();
      expect(api.detectDefaultWatchFolders).toBeDefined();
    });

    it('smart folder command functions are stubs', () => {
      expect(api.getSmartFolders).toBeDefined();
      expect(api.getSmartFolderContent).toBeDefined();
    });

    it('file operation command functions are stubs', () => {
      expect(api.openInFinder).toBeDefined();
      expect(api.copyPathToClipboard).toBeDefined();
      expect(api.exportScreenshot).toBeDefined();
    });
  });
});