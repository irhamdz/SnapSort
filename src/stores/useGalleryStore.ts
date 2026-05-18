import { create } from 'zustand'

export interface Screenshot {
  id: string
  filepath: string
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

  // Actions
  setScreenshots: (screenshots: Screenshot[]) => void
  addScreenshot: (screenshot: Screenshot) => void
  updateScreenshot: (id: string, updates: Partial<Screenshot>) => void
  removeScreenshot: (id: string) => void
  setLoading: (isLoading: boolean) => void
  setError: (error: string | null) => void
  reset: () => void
}

export const useGalleryStore = create<GalleryState>((set) => ({
  // Initial state
  screenshots: [],
  isLoading: false,
  error: null,

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
  reset: () =>
    set({
      screenshots: [],
      isLoading: false,
      error: null,
    }),
}))