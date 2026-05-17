import { invoke } from '@tauri-apps/api/core'

// Type-safe IPC wrappers — all invoke calls flow through here

/**
 * Generic invoke wrapper that preserves TypeScript typing
 */
export async function invokeTyped<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  return invoke<T>(command, args)
}

/**
 * Example typed command wrappers (to be expanded as backend commands are added)
 */

// Health check command
export async function healthCheck() {
  return invokeTyped<string>('health_check')
}

// Gallery commands
export async function getScreenshots() {
  return invokeTyped<any[]>('get_screenshots')
}

export async function searchScreenshots(query: string) {
  return invokeTyped<any[]>('search_screenshots', { query })
}

export async function getScreenshotById(id: string) {
  return invokeTyped<any>('get_screenshot_by_id', { id })
}

export async function getThumbnail(id: number) {
  return invokeTyped<number[] | null>('get_thumbnail', { id })
}

// Batch commands (to be added)
export async function selectForBatch(itemIds: string[]) {
  return invokeTyped<void>('select_for_batch', { itemIds })
}

export async function deselectFromBatch(itemIds: string[]) {
  return invokeTyped<void>('deselect_from_batch', { itemIds })
}

export async function deleteBatchItems(itemIds: string[]) {
  return invokeTyped<void>('delete_batch_items', { itemIds })
}

// Settings commands (to be added)
export async function getSettings() {
  return invokeTyped<any>('get_settings')
}

export async function saveSettings(settings: any) {
  return invokeTyped<void>('save_settings', { settings })
}

// AI commands (to be added)
export async function analyzeWithAI(imagePath: string) {
  return invokeTyped<any>('analyze_with_ai', { imagePath })
}

export async function generateSummary(ocrText: string) {
  return invokeTyped<any>('generate_summary', { ocrText })
}

// Type exports for stores to use
export interface Screenshot {
  id: string
  filepath: string
  width: number
  height: number
  status: 'detected' | 'queued' | 'ocr_complete' | 'analyzed'
  category: string
  category_source: 'ai' | 'user'
  tags: string[]
  ocr_text: string
  summary: string
  thumbnail: string | null
  created_at: string
  updated_at: string
}

export interface Settings {
  watchFolders: string[]
  defaultCategory: string
  ocrEnabled: boolean
  aiEnabled: boolean
  ocrLanguage: string
  aiProvider: string
  aiModel: string
  theme: 'light' | 'dark'
}

export interface BatchSelection {
  selectedIds: string[]
  isSelectAll: boolean
}