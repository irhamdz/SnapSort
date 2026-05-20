import { create } from 'zustand'
import { useBatchStore as selectionStore } from './useBatchStore'
import { invoke } from '@tauri-apps/api/core'

// File operation error result
export interface FileOpError {
  id: string
  path: string
  error: string
}

// Store interface for batch operations state
interface BatchState {
  // Selection state (transient, lives in useBatchStore)
  selectedCount: number
  isSelectAll: boolean

  // Last action result
  lastAction: string | null
  lastErrors: FileOpError[] | null

  // Actions
  setSelectionMode: (enabled: boolean) => void
  selectAll: () => void
  deselectAll: () => void
  toggleSelection: (id: string) => void
  clearSelection: () => void
  deleteSelected: () => Promise<void>
  categorizeSelected: (category: string) => Promise<void>
  renameSelected: (pattern: string) => Promise<void>
  tagSelected: (tags: string[], add: boolean) => Promise<void>
  archiveSelected: () => Promise<void>
  moveSelected: (destPath: string) => Promise<void>
  addToCollectionSelected: (collectionId: string) => Promise<void>
}

export const useBatchStore = create<BatchState>((set) => ({
  // Initial state
  selectedCount: 0,
  isSelectAll: false,
  lastAction: null,
  lastErrors: null,

  // Actions
  setSelectionMode: (enabled) =>
    set({
      selectedCount: enabled ? 0 : 0,
      isSelectAll: false,
      lastAction: enabled ? 'Select Mode' : null,
      lastErrors: null,
    }),

  selectAll: () => {
    const { selection } = selectionStore.getState()
    if (selection.selectedIds.includes('all')) {
      selectionStore.getState().toggleSelectAll() // Deselect all
    } else {
      selectionStore.getState().toggleSelectAll() // Select all
    }
    set({ lastAction: 'Select All' })
  },

  deselectAll: () => {
    selectionStore.getState().clearSelection()
    set({ lastAction: 'Deselect All' })
  },

  toggleSelection: (id) => {
    selectionStore.getState().toggleSelection(id)
    const { selection } = selectionStore.getState()
    set({
      selectedCount: selection.selectedIds.includes('all')
        ? -1 // Select all mode
        : selection.selectedIds.length,
      isSelectAll: selection.selectedIds.includes('all'),
      lastAction: selection.selectedIds.includes('all') ? 'Select All' : null,
    })
  },

  clearSelection: () => {
    selectionStore.getState().reset()
    set({
      selectedCount: 0,
      isSelectAll: false,
      lastAction: null,
      lastErrors: null,
    })
  },

  deleteSelected: async () => {
    const { selection } = selectionStore.getState()
    if (selection.selectedIds.length === 0) return

    set({ lastAction: 'Deleting...' })

    try {
      await invoke('batch_delete', { screenshotIds: selection.selectedIds })
      selectionStore.getState().reset()
      set({ lastAction: 'Deleted', lastErrors: null })
    } catch (error) {
      set({
        lastAction: 'Delete Failed',
        lastErrors: [{ id: 'batch', path: '', error: error instanceof Error ? error.message : String(error) }],
      })
    }
  },

  categorizeSelected: async (category) => {
    const { selection } = selectionStore.getState()
    if (selection.selectedIds.length === 0) return

    set({ lastAction: 'Categorizing...' })

    try {
      await invoke('batch_categorize', { screenshotIds: selection.selectedIds, category })
      selectionStore.getState().reset()
      set({ lastAction: 'Categorized', lastErrors: null })
    } catch (error) {
      set({
        lastAction: 'Categorize Failed',
        lastErrors: [{ id: 'batch', path: '', error: error instanceof Error ? error.message : String(error) }],
      })
    }
  },

  renameSelected: async (pattern) => {
    const { selection } = selectionStore.getState()
    if (selection.selectedIds.length === 0) return

    set({ lastAction: 'Renaming...' })

    try {
      const errors = await invoke<FileOpError[]>('batch_rename', { screenshotIds: selection.selectedIds, pattern })
      selectionStore.getState().reset()
      set({ lastAction: errors.length > 0 ? 'Rename Partially Failed' : 'Renamed', lastErrors: errors.length > 0 ? errors : null })
    } catch (error) {
      set({
        lastAction: 'Rename Failed',
        lastErrors: [{ id: 'batch', path: '', error: error instanceof Error ? error.message : String(error) }],
      })
    }
  },

  tagSelected: async (tags, add) => {
    const { selection } = selectionStore.getState()
    if (selection.selectedIds.length === 0) return

    set({ lastAction: 'Tagging...' })

    try {
      await invoke('batch_tag', { screenshotIds: selection.selectedIds, tags, add })
      selectionStore.getState().reset()
      set({ lastAction: 'Tagged', lastErrors: null })
    } catch (error) {
      set({
        lastAction: 'Tag Failed',
        lastErrors: [{ id: 'batch', path: '', error: error instanceof Error ? error.message : String(error) }],
      })
    }
  },

  archiveSelected: async () => {
    const { selection } = selectionStore.getState()
    if (selection.selectedIds.length === 0) return

    set({ lastAction: 'Archiving...' })

    try {
      await invoke('batch_archive', { screenshotIds: selection.selectedIds })
      selectionStore.getState().reset()
      set({ lastAction: 'Archived', lastErrors: null })
    } catch (error) {
      set({
        lastAction: 'Archive Failed',
        lastErrors: [{ id: 'batch', path: '', error: error instanceof Error ? error.message : String(error) }],
      })
    }
  },

  moveSelected: async (destPath) => {
    const { selection } = selectionStore.getState()
    if (selection.selectedIds.length === 0) return

    set({ lastAction: 'Moving...' })

    try {
      const errors = await invoke<FileOpError[]>('batch_move', { screenshotIds: selection.selectedIds, destPath })
      selectionStore.getState().reset()
      set({ lastAction: errors.length > 0 ? 'Move Partially Failed' : 'Moved', lastErrors: errors.length > 0 ? errors : null })
    } catch (error) {
      set({
        lastAction: 'Move Failed',
        lastErrors: [{ id: 'batch', path: '', error: error instanceof Error ? error.message : String(error) }],
      })
    }
  },

  addToCollectionSelected: async (collectionId) => {
    const { selection } = selectionStore.getState()
    if (selection.selectedIds.length === 0) return

    set({ lastAction: 'Adding to collection...' })

    try {
      await invoke('batch_add_to_collection', { screenshotIds: selection.selectedIds, collectionId })
      selectionStore.getState().reset()
      set({ lastAction: 'Added to collection', lastErrors: null })
    } catch (error) {
      set({
        lastAction: 'Add to collection Failed',
        lastErrors: [{ id: 'batch', path: '', error: error instanceof Error ? error.message : String(error) }],
      })
    }
  },
}))