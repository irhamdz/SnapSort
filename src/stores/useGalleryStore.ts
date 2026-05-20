import { create } from 'zustand'

export interface Screenshot {
  id: string
  filepath: string
  filename: string
  width: number
  height: number
  status: 'detected' | 'queued' | 'ocr_complete' | 'ready' | 'analyzing' | 'enriched' | 'partial' | 'archived' | 'deleted'
  category: string | null
  category_source: 'ai' | 'user'
  tags: string[]
  ocr_text: string | null
  summary: string | null
  thumbnail: string | null
  created_at: string
  updated_at: string
}

interface GalleryState {
  // Raw state
  screenshots: Screenshot[]
  isLoading: boolean
  error: string | null

  // Selection state
  selectedScreenshotIds: string[]

  // UI state
  showBatchActions: boolean
  showDetailPanel: boolean
  detailPanelScreenshot: Screenshot | null

  // Actions
  setScreenshots: (screenshots: Screenshot[]) => void
  addScreenshot: (screenshot: Screenshot) => void
  updateScreenshot: (id: string, updates: Partial<Screenshot>) => void
  removeScreenshot: (id: string) => void
  setLoading: (isLoading: boolean) => void
  setError: (error: string | null) => void
  selectScreenshot: (id: string) => void
  deselectScreenshot: (id: string) => void
  setSelectedScreenshotIds: (ids: string[]) => void
  setShowBatchActions: (show: boolean) => void
  setShowDetailPanel: (show: boolean) => void
  setDetailPanelScreenshot: (screenshot: Screenshot | null) => void
  reset: () => void
}

export const useGalleryStore = create<GalleryState>((set) => ({
  // Initial state
  screenshots: [],
  isLoading: false,
  error: null,
  selectedScreenshotIds: [],
  showBatchActions: false,
  showDetailPanel: false,
  detailPanelScreenshot: null,

  // Actions
  setScreenshots: (screenshots) => set({ screenshots, error: null }),
  addScreenshot: (screenshot) =>
    set((state) => ({
      screenshots: [...state.screenshots, screenshot],
    })),
  updateScreenshot: (id, updates) =>
    set((state) => ({
      screenshots: state.screenshots.map((s) =>
        s.id === id ? { ...s, ...updates } : s
      ),
    })),
  removeScreenshot: (id) =>
    set((state) => ({
      screenshots: state.screenshots.filter((s) => s.id !== id),
    })),
  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),
  selectScreenshot: (id) =>
    set((state) => ({
      selectedScreenshotIds: [...state.selectedScreenshotIds, id],
    })),
  deselectScreenshot: (id) =>
    set((state) => ({
      selectedScreenshotIds: state.selectedScreenshotIds.filter((sid) => sid !== id),
    })),
  setSelectedScreenshotIds: (ids) =>
    set({
      selectedScreenshotIds: ids,
    }),
  setShowBatchActions: (show) => set({ showBatchActions: show }),
  setShowDetailPanel: (show) => set({ showDetailPanel: show }),
  setDetailPanelScreenshot: (screenshot) => set({ detailPanelScreenshot: screenshot }),
  reset: () =>
    set({
      screenshots: [],
      isLoading: false,
      error: null,
      selectedScreenshotIds: [],
      showBatchActions: false,
      showDetailPanel: false,
      detailPanelScreenshot: null,
    }),
}))