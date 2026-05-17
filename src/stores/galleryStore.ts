import { create } from 'zustand'

// Store interface for gallery state
interface GalleryState {
  // TODO: Implement gallery state
  screenshots: any[]
  selectedIds: string[]
  filter: {
    search: string
    category: string | null
    tags: string[]
    dateRange: { start: string | null; end: string | null }
    isArchived: boolean
  }
  filters: {
    categories: string[]
    tags: string[]
    apps: string[]
    dateRanges: string[]
  }
}

interface GalleryActions {
  // TODO: Implement gallery actions
  loadScreenshots: () => Promise<void>
  setFilter: (key: keyof GalleryState['filter'], value: any) => void
  clearFilters: () => void
  toggleFavorite: (id: string) => void
  archive: (id: string, archived: boolean) => void
  delete: (id: string) => Promise<void>
}

export const useGalleryStore = create<GalleryState & GalleryActions>((set) => ({
  // Initial state
  screenshots: [],
  selectedIds: [],
  filter: {
    search: '',
    category: null,
    tags: [],
    dateRange: { start: null, end: null },
    isArchived: false,
  },
  filters: {
    categories: [],
    tags: [],
    apps: [],
    dateRanges: [],
  },

  // Actions
  loadScreenshots: async () => {
    // TODO: Implement
  },

  setFilter: (key, value) =>
    set((state) => ({
      filter: { ...state.filter, [key]: value },
    })),

  clearFilters: () =>
    set({
      filter: {
        search: '',
        category: null,
        tags: [],
        dateRange: { start: null, end: null },
        isArchived: false,
      },
    }),

  toggleFavorite: (id) =>
    set((state) => ({
      screenshots: state.screenshots.map((s) =>
        s.id === id ? { ...s, is_favorite: !s.is_favorite } : s
      ),
    })),

  archive: (id, archived) =>
    set((state) => ({
      screenshots: state.screenshots.map((s) =>
        s.id === id ? { ...s, is_archived: archived } : s
      ),
    })),

  delete: async (id) => {
    // TODO: Implement
  },
}))