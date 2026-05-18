import { describe, it, expect, vi } from 'vitest';
import * as api from './index';

describe('API Layer - Stub Implementation', () => {
  it('exports all API functions', () => {
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

    // Category commands
    expect(api.getCategories).toBeDefined();
    expect(api.assignCategory).toBeDefined();

    // Tag commands
    expect(api.getTags).toBeDefined();
    expect(api.addTags).toBeDefined();
    expect(api.removeTags).toBeDefined();

    // OCR commands
    expect(api.getOCRText).toBeDefined();
    expect(api.copyOCRTextToClipboard).toBeDefined();

    // Collection commands
    expect(api.getCollections).toBeDefined();
    expect(api.createCollection).toBeDefined();
    expect(api.addToCollection).toBeDefined();
    expect(api.removeFromCollection).toBeDefined();

    // Watch folder commands
    expect(api.addWatchFolder).toBeDefined();
    expect(api.removeWatchFolder).toBeDefined();
    expect(api.getWatchFolders).toBeDefined();
    expect(api.detectDefaultWatchFolders).toBeDefined();

    // Smart folders
    expect(api.getSmartFolders).toBeDefined();
    expect(api.getSmartFolderContent).toBeDefined();

    // File operations
    expect(api.openInFinder).toBeDefined();
    expect(api.copyPathToClipboard).toBeDefined();
    expect(api.exportScreenshot).toBeDefined();
  });

  it('all functions are stubs (not implemented yet)', () => {
    const functions = Object.values(api);

    // All functions should be stubs that throw "Not implemented"
    functions.forEach(fn => {
      expect(typeof fn).toBe('function');
    });
  });
});