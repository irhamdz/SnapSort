import { create } from 'zustand'

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

interface SettingsState {
  // Raw state
  settings: Settings

  // Actions
  setSettings: (settings: Settings) => void
  updateSettings: (updates: Partial<Settings>) => void
  setWatchFolders: (folders: string[]) => void
  setDefaultCategory: (category: string) => void
  setOcrEnabled: (enabled: boolean) => void
  setAiEnabled: (enabled: boolean) => void
  setOcrLanguage: (language: string) => void
  setAiProvider: (provider: string) => void
  setAiModel: (model: string) => void
  setTheme: (theme: 'light' | 'dark') => void
  reset: () => void
}

export const useSettingsStore = create<SettingsState>((set) => ({
  // Initial state
  settings: {
    watchFolders: [],
    defaultCategory: 'uncategorized',
    ocrEnabled: true,
    aiEnabled: false,
    ocrLanguage: 'eng',
    aiProvider: 'ollama',
    aiModel: 'llama3.2',
    theme: 'dark',
  },

  // Actions
  setSettings: (settings) => set({ settings }),
  updateSettings: (updates) =>
    set((state) => ({
      settings: { ...state.settings, ...updates },
    })),
  setWatchFolders: (folders) =>
    set((state) => ({
      settings: { ...state.settings, watchFolders: folders },
    })),
  setDefaultCategory: (category) =>
    set((state) => ({
      settings: { ...state.settings, defaultCategory: category },
    })),
  setOcrEnabled: (enabled) =>
    set((state) => ({
      settings: { ...state.settings, ocrEnabled: enabled },
    })),
  setAiEnabled: (enabled) =>
    set((state) => ({
      settings: { ...state.settings, aiEnabled: enabled },
    })),
  setOcrLanguage: (language) =>
    set((state) => ({
      settings: { ...state.settings, ocrLanguage: language },
    })),
  setAiProvider: (provider) =>
    set((state) => ({
      settings: { ...state.settings, aiProvider: provider },
    })),
  setAiModel: (model) =>
    set((state) => ({
      settings: { ...state.settings, aiModel: model },
    })),
  setTheme: (theme) =>
    set((state) => ({
      settings: { ...state.settings, theme },
    })),
  reset: () =>
    set({
      settings: {
        watchFolders: [],
        defaultCategory: 'uncategorized',
        ocrEnabled: true,
        aiEnabled: false,
        ocrLanguage: 'eng',
        aiProvider: 'ollama',
        aiModel: 'llama3.2',
        theme: 'dark',
      },
    }),
}))