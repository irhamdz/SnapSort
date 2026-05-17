// Tauri IPC wrapper stubs
// These provide type-safe wrappers around Tauri command invocations

import { invoke } from '@tauri-apps/api/core'

// Screenshot-related commands
export async function takeScreenshot(): Promise<string> {
  // TODO: Implement screenshot capture
  throw new Error('Not implemented')
}

export async function loadScreenshots(): Promise<any[]> {
  // TODO: Implement
  return []
}

export async function getScreenshot(id: string): Promise<any> {
  // TODO: Implement
  return null
}

export async function deleteScreenshot(id: string): Promise<void> {
  // TODO: Implement
}

export async function archiveScreenshot(id: string, archived: boolean): Promise<void> {
  // TODO: Implement
}

export async function updateScreenshotMetadata(
  id: string,
  metadata: Partial<any>
): Promise<void> {
  // TODO: Implement
}

// Category commands
export async function getCategories(): Promise<string[]> {
  // TODO: Implement
  return []
}

export async function assignCategory(
  screenshotId: string,
  category: string,
  source: 'ai' | 'user'
): Promise<void> {
  // TODO: Implement
}

// Tag commands
export async function getTags(): Promise<string[]> {
  // TODO: Implement
  return []
}

export async function addTags(screenshotId: string, tags: string[]): Promise<void> {
  // TODO: Implement
}

export async function removeTags(screenshotId: string, tags: string[]): Promise<void> {
  // TODO: Implement
}

// OCR commands
export async function getOCRText(screenshotId: string): Promise<string> {
  // TODO: Implement
  return ''
}

export async function copyOCRTextToClipboard(screenshotId: string): Promise<void> {
  // TODO: Implement
}

// Batch operations
export async function batchDelete(screenshotIds: string[]): Promise<void> {
  // TODO: Implement
}

export async function batchCategorize(
  screenshotIds: string[],
  category: string
): Promise<void> {
  // TODO: Implement
}

export async function batchRename(
  screenshotIds: string[],
  pattern: string
): Promise<void> {
  // TODO: Implement
}

export async function batchTag(
  screenshotIds: string[],
  tags: string[],
  add: boolean
): Promise<void> {
  // TODO: Implement
}

export async function batchArchive(screenshotIds: string[]): Promise<void> {
  // TODO: Implement
}

export async function batchMove(screenshotIds: string[], destPath: string): Promise<void> {
  // TODO: Implement
}

export async function batchAddToCollection(
  screenshotIds: string[],
  collectionId: string
): Promise<void> {
  // TODO: Implement
}

// Search commands
export async function searchScreenshots(query: string): Promise<any[]> {
  // TODO: Implement
  return []
}

export async function searchWithFilters(filters: any): Promise<any[]> {
  // TODO: Implement
  return []
}

// AI commands
export async function runAIAnalysis(screenshotId: string): Promise<void> {
  // TODO: Implement
}

export async function getAIAnalysisStatus(screenshotId: string): Promise<any> {
  // TODO: Implement
  return null
}

// Collection commands
export async function getCollections(): Promise<any[]> {
  // TODO: Implement
  return []
}

export async function createCollection(name: string): Promise<string> {
  // TODO: Implement
  throw new Error('Not implemented')
}

export async function addToCollection(screenshotId: string, collectionId: string): Promise<void> {
  // TODO: Implement
}

export async function removeFromCollection(screenshotId: string, collectionId: string): Promise<void> {
  // TODO: Implement
}

// Watch folder commands
export async function addWatchFolder(path: string): Promise<void> {
  // TODO: Implement
}

export async function removeWatchFolder(path: string): Promise<void> {
  // TODO: Implement
}

export async function getWatchFolders(): Promise<string[]> {
  // TODO: Implement
  return []
}

export async function detectDefaultWatchFolders(): Promise<string[]> {
  // TODO: Implement
  return []
}

// Settings commands
export async function getSettings(): Promise<any> {
  // TODO: Implement
  return {}
}

export async function saveSettings(settings: any): Promise<void> {
  // TODO: Implement
}

// Smart folders
export async function getSmartFolders(): Promise<any[]> {
  // TODO: Implement
  return []
}

export async function getSmartFolderContent(folderId: string): Promise<any[]> {
  // TODO: Implement
  return []
}

// File operations
export async function openInFinder(path: string): Promise<void> {
  // TODO: Implement
}

export async function copyPathToClipboard(path: string): Promise<void> {
  // TODO: Implement
}

export async function exportScreenshot(screenshotId: string, format: string): Promise<void> {
  // TODO: Implement
}