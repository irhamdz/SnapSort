import { describe, it, expect, beforeEach, vi } from 'vitest';

// Mock the @tauri-apps/api/core module
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

import * as api from './index';
import { invoke } from '@tauri-apps/api/core';

describe('API Layer - invokeTyped', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (invoke as any).mockResolvedValue({});
  });

  it('should call invoke with correct parameters and return result', async () => {
    const mockResult = { id: '1', title: 'Test' };
    (invoke as any).mockResolvedValueOnce(mockResult);
    
    const result = await api.invokeTyped<{ id: string; title: string }>('test_command', { 
      param1: 'value1',
      param2: 42
    });
    
    expect(invoke).toHaveBeenCalledWith('test_command', { 
      param1: 'value1',
      param2: 42
    });
    expect(result).toEqual(mockResult);
  });

  it('should handle empty args', async () => {
    (invoke as any).mockResolvedValue([]);
    
    const result = await api.invokeTyped<number[]>('get_screenshots');
    
    expect(invoke).toHaveBeenCalledWith('get_screenshots', undefined);
    expect(result).toEqual([]);
  });
});

describe('API Layer - Gallery Commands', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (invoke as any).mockResolvedValue({ id: '1', path: '/path/1.png' });
  });

  it('getScreenshots should call invoke with correct arguments', async () => {
    (invoke as any).mockResolvedValue([
      { id: '1', path: '/path/1.png' },
      { id: '2', path: '/path/2.png' }
    ]);
    
    const result = await api.getScreenshots();
    
    expect(invoke).toHaveBeenCalledWith('get_screenshots', undefined);
    expect(result).toHaveLength(2);
    expect(result[0]).toEqual({ id: '1', path: '/path/1.png' });
  });

  it('searchScreenshots should call invoke with query parameter', async () => {
    (invoke as any).mockResolvedValue([
      { id: '1', path: '/path/1.png', ocr_text: 'test search term' }
    ]);
    
    const result = await api.searchScreenshots('test search term');
    
    expect(invoke).toHaveBeenCalledWith('search_screenshots', { query: 'test search term' });
    expect(result).toHaveLength(1);
  });

  it('getScreenshotById should call invoke with id parameter', async () => {
    (invoke as any).mockResolvedValue({ 
      id: '1', 
      path: '/path/1.png',
      width: 1920,
      height: 1080
    });
    
    const result = await api.getScreenshotById('1');
    
    expect(invoke).toHaveBeenCalledWith('get_screenshot_by_id', { id: '1' });
    expect(result).toEqual({ 
      id: '1', 
      path: '/path/1.png',
      width: 1920,
      height: 1080
    });
  });
});

describe('API Layer - Batch Commands', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (invoke as any).mockResolvedValue(undefined);
  });

  it('selectForBatch should call invoke with itemIds array', async () => {
    await api.selectForBatch(['id1', 'id2', 'id3']);
    
    expect(invoke).toHaveBeenCalledWith('select_for_batch', { 
      itemIds: ['id1', 'id2', 'id3'] 
    });
  });

  it('deselectFromBatch should call invoke with itemIds array', async () => {
    await api.deselectFromBatch(['id1', 'id2']);
    
    expect(invoke).toHaveBeenCalledWith('deselect_from_batch', { 
      itemIds: ['id1', 'id2'] 
    });
  });

  it('deleteBatchItems should call invoke with itemIds array', async () => {
    await api.deleteBatchItems(['id1', 'id2', 'id3']);
    
    expect(invoke).toHaveBeenCalledWith('delete_batch_items', { 
      itemIds: ['id1', 'id2', 'id3'] 
    });
  });
});

describe('API Layer - Settings Commands', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (invoke as any).mockResolvedValue({
      watchPaths: ['/path1', '/path2'],
      ocrEnabled: true,
      theme: 'dark'
    });
  });

  it('getSettings should call invoke without parameters', async () => {
    const result = await api.getSettings();
    
    expect(invoke).toHaveBeenCalledWith('get_settings', undefined);
    expect(result).toEqual({
      watchPaths: ['/path1', '/path2'],
      ocrEnabled: true,
      theme: 'dark'
    });
  });

  it('saveSettings should call invoke with settings object', async () => {
    const settings = { 
      watchPaths: ['/path1'], 
      ocrEnabled: false,
      theme: 'light'
    };
    
    await api.saveSettings(settings);
    
    expect(invoke).toHaveBeenCalledWith('save_settings', { settings });
  });
});

describe('API Layer - AI Commands', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (invoke as any).mockResolvedValue({
      category: 'code',
      tags: ['programming', 'example'],
      summary: 'AI analysis result'
    });
  });

  it('analyzeWithAI should call invoke with imagePath', async () => {
    const result = await api.analyzeWithAI('/path/to/image.png');
    
    expect(invoke).toHaveBeenCalledWith('analyze_with_ai', { imagePath: '/path/to/image.png' });
    expect(result).toEqual({
      category: 'code',
      tags: ['programming', 'example'],
      summary: 'AI analysis result'
    });
  });

  it('generateSummary should call invoke with ocrText', async () => {
    (invoke as any).mockResolvedValue({
      summary: 'Generated summary text',
      category: 'documentation'
    });
    
    const result = await api.generateSummary('Some OCR text here');
    
    expect(invoke).toHaveBeenCalledWith('generate_summary', { ocrText: 'Some OCR text here' });
    expect(result).toEqual({
      summary: 'Generated summary text',
      category: 'documentation'
    });
  });
});

describe('API Layer - Error Handling', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    const error = new Error('Tauri command failed');
    (invoke as any).mockRejectedValue(error);
  });

  it('should propagate errors from invoke', async () => {
    await expect(api.getScreenshots()).rejects.toThrow('Tauri command failed');
  });

  it('should propagate errors from all command wrappers', async () => {
    await expect(api.searchScreenshots('test')).rejects.toThrow('Tauri command failed');
    await expect(api.selectForBatch(['id'])).rejects.toThrow('Tauri command failed');
    await expect(api.getSettings()).rejects.toThrow('Tauri command failed');
    await expect(api.analyzeWithAI('/path.png')).rejects.toThrow('Tauri command failed');
  });
});