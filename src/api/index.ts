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

export async function getScreenshot(_id: string): Promise<any> {
  // TODO: Implement
  return null
}

export async function deleteScreenshot(id: string): Promise<void> {
  return invoke('delete_screenshot', { id: parseInt(id, 10) })
}

export async function archiveScreenshot(_id: string, _archived: boolean): Promise<void> {
  // TODO: Implement
}

export async function updateScreenshotMetadata(
  _id: string,
  _metadata: Partial<any>
): Promise<void> {
  // TODO: Implement
}

// Category commands
export async function getCategories(): Promise<string[]> {
  // TODO: Implement
  return []
}

export async function assignCategory(
  _screenshotId: string,
  _category: string,
  _source: 'ai' | 'user'
): Promise<void> {
  // TODO: Implement
}

// Tag commands
export async function getTags(): Promise<string[]> {
  // TODO: Implement
  return []
}

export async function addTags(_screenshotId: string, _tags: string[]): Promise<void> {
  // TODO: Implement
}

export async function removeTags(_screenshotId: string, _tags: string[]): Promise<void> {
  // TODO: Implement
}

// OCR commands
export async function getOCRText(_screenshotId: string): Promise<string> {
  // TODO: Implement
  return ''
}

export async function copyOCRTextToClipboard(_screenshotId: string): Promise<void> {
  // TODO: Implement
}

// Batch operations
export type FileOpError = { id: number; path: string; error: string }

export async function batchDelete(screenshotIds: string[]): Promise<FileOpError[]> {
  return invoke('batch_delete', { screenshotIds })
}

export async function batchCategorize(
  screenshotIds: string[],
  category: string
): Promise<void> {
  return invoke('batch_categorize', { screenshotIds, category })
}

export async function batchRename(
  screenshotIds: string[],
  pattern: string
): Promise<void> {
  return invoke('batch_rename', { screenshotIds, pattern })
}

export async function batchTag(
  screenshotIds: string[],
  tags: string[],
  add: boolean
): Promise<void> {
  return invoke('batch_tag', { screenshotIds, tags, add })
}

export async function batchArchive(screenshotIds: string[]): Promise<void> {
  return invoke('batch_archive', { screenshotIds })
}

export async function batchMove(screenshotIds: string[], destPath: string): Promise<void> {
  return invoke('batch_move', { screenshotIds, destPath })
}

export async function batchAddToCollection(
  screenshotIds: string[],
  collectionId: string
): Promise<void> {
  return invoke('batch_add_to_collection', { screenshotIds, collectionId })
}

// Search commands
export async function searchScreenshots(_query: string): Promise<any[]> {
  // TODO: Implement
  return []
}

export async function searchWithFilters(_filters: any): Promise<any[]> {
  // TODO: Implement
  return []
}

// AI provider commands
export interface ProviderInfo {
  id: string
  name: string
  is_configured: boolean
  is_active: boolean
  has_key: boolean
  key_suffix: string | null
}

export async function getProviders(): Promise<ProviderInfo[]> {
  return invoke<ProviderInfo[]>('get_providers')
}

export async function addProvider(provider: {
  provider_id: string
  display_name: string
  base_url: string
  model: string
}): Promise<void> {
  return invoke('add_provider', {
    providerId: provider.provider_id,
    displayName: provider.display_name,
    baseUrl: provider.base_url,
    model: provider.model,
  })
}

export async function removeProvider(providerId: string): Promise<void> {
  return invoke('remove_provider', { providerId })
}

export async function testConnection(providerId: string): Promise<boolean> {
  return invoke<boolean>('test_connection', { providerId })
}

export async function setActiveProvider(providerId: string): Promise<void> {
  return invoke('set_active_provider', { providerId })
}

/** Write-only: stores the API key in the OS keychain. Never echoed back. */
export async function setApiKey(providerId: string, apiKey: string): Promise<void> {
  return invoke('set_api_key', { providerId, apiKey })
}

export async function hasApiKey(providerId: string): Promise<boolean> {
  return invoke<boolean>('has_api_key', { providerId })
}

export async function deleteApiKey(providerId: string): Promise<void> {
  return invoke('delete_api_key', { providerId })
}

// AI analysis commands
export async function runAIAnalysis(_screenshotId: string): Promise<void> {
  // TODO: Implement
}

export async function getAIAnalysisStatus(_screenshotId: string): Promise<any> {
  // TODO: Implement
  return null
}

// Collection commands
export async function getCollections(): Promise<any[]> {
  return invoke('get_collections')
}

export async function createCollection(name: string): Promise<string> {
  return invoke('create_collection', { name })
}

export async function addToCollection(screenshotId: string, collectionId: string): Promise<void> {
  return invoke('add_to_collection', { screenshotId, collectionId })
}

export async function removeFromCollection(screenshotId: string, collectionId: string): Promise<void> {
  return invoke('remove_from_collection', { screenshotId, collectionId })
}

// Watch folder commands
export async function addWatchFolder(_path: string): Promise<void> {
  // TODO: Implement
}

export async function removeWatchFolder(_path: string): Promise<void> {
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

export async function saveSettings(_settings: any): Promise<void> {
  // TODO: Implement
}

// Smart folders
export async function getSmartFolders(): Promise<any[]> {
  // TODO: Implement
  return []
}

export async function getSmartFolderContent(_folderId: string): Promise<any[]> {
  // TODO: Implement
  return []
}

// File operations
export async function openInFinder(_path: string): Promise<void> {
  // TODO: Implement
}

export async function copyPathToClipboard(_path: string): Promise<void> {
  // TODO: Implement
}

export async function exportScreenshot(_screenshotId: string, _format: string): Promise<void> {
  // TODO: Implement
}