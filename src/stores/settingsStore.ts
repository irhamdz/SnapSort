import { create } from 'zustand'

// Store interface for settings state
interface SettingsState {
  // TODO: Implement settings state
  aiProviders: any[]
  watchFolders: any[]
  theme: 'light' | 'dark' | 'system'
  language: string
  notifications: {
    enabled: boolean
    onNewScreenshot: boolean
    onAIComplete: boolean
  }
  lastAIAnalysis: Date | null
}

interface SettingsActions {
  // TODO: Implement settings actions
  addAIProvider: (provider: any) => void
  removeAIProvider: (id: string) => void
  setAIProviderActive: (id: string, active: boolean) => void
  addWatchFolder: (path: string) => void
  removeWatchFolder: (path: string) => void
  setTheme: (theme: 'light' | 'dark' | 'system') => void
  updateNotifications: (settings: any) => void
  setLastAIAnalysis: (date: Date | null) => void
}

export const useSettingsStore = create<SettingsState & SettingsActions>((set) => ({
  // Initial state
  aiProviders: [],
  watchFolders: [],
  theme: 'system',
  language: 'en',
  notifications: {
    enabled: true,
    onNewScreenshot: true,
    onAIComplete: true,
  },
  lastAIAnalysis: null,

  // Actions
  addAIProvider: (provider) =>
    set((state) => ({
      aiProviders: [...state.aiProviders, provider],
    })),

  removeAIProvider: (id) =>
    set((state) => ({
      aiProviders: state.aiProviders.filter((p) => p.id !== id),
    })),

  setAIProviderActive: (id, active) =>
    set((state) => ({
      aiProviders: state.aiProviders.map((p) =>
        p.id === id ? { ...p, is_active: active } : p
      ),
    })),

  addWatchFolder: (path) =>
    set((state) => ({
      watchFolders: [...state.watchFolders, { path, added_at: new Date() }],
    })),

  removeWatchFolder: (path) =>
    set((state) => ({
      watchFolders: state.watchFolders.filter((f) => f.path !== path),
    })),

  setTheme: (theme) => set({ theme }),

  updateNotifications: (settings) =>
    set((state) => ({
      notifications: { ...state.notifications, ...settings },
    })),

  setLastAIAnalysis: (date) => set({ lastAIAnalysis: date }),
}))