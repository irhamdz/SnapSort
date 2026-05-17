import { describe, it, expect, beforeEach } from 'vitest'
import { useGalleryStore } from '../stores/useGalleryStore'
import { useBatchStore } from '../stores/useBatchStore'
import { useSettingsStore } from '../stores/useSettingsStore'

describe('useGalleryStore', () => {
  beforeEach(() => {
    useGalleryStore.getState().setScreenshots([])
  })

  it('should initialize with empty screenshots', () => {
    const state = useGalleryStore.getState()
    expect(state.screenshots).toEqual([])
    expect(state.isLoading).toBe(false)
    expect(state.error).toBeNull()
  })

  it('should set screenshots', () => {
    const screenshots: any[] = [
      {
        id: '1',
        filepath: '/path/to/image.png',
        width: 1920,
        height: 1080,
        status: 'detected',
        category: 'uncategorized',
        category_source: 'user',
        tags: [],
        ocr_text: '',
        summary: '',
        thumbnail: null,
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
      },
    ]

    useGalleryStore.getState().setScreenshots(screenshots)

    const state = useGalleryStore.getState()
    expect(state.screenshots).toHaveLength(1)
    expect(state.screenshots[0].id).toBe('1')
  })

  it('should add a screenshot', () => {
    useGalleryStore.getState().addScreenshot({
      id: '1',
      filepath: '/path/to/image.png',
      width: 1920,
      height: 1080,
      status: 'detected',
      category: 'uncategorized',
      category_source: 'user',
      tags: [],
      ocr_text: '',
      summary: '',
      thumbnail: null,
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-01T00:00:00Z',
    })

    const state = useGalleryStore.getState()
    expect(state.screenshots).toHaveLength(1)
  })

  it('should update a screenshot', () => {
    useGalleryStore.getState().addScreenshot({
      id: '1',
      filepath: '/path/to/image.png',
      width: 1920,
      height: 1080,
      status: 'detected',
      category: 'uncategorized',
      category_source: 'user',
      tags: [],
      ocr_text: '',
      summary: '',
      thumbnail: null,
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-01T00:00:00Z',
    })

    useGalleryStore.getState().updateScreenshot('1', { status: 'ocr_complete' })

    const state = useGalleryStore.getState()
    expect(state.screenshots[0].status).toBe('ocr_complete')
  })

  it('should remove a screenshot', () => {
    useGalleryStore.getState().addScreenshot({
      id: '1',
      filepath: '/path/to/image.png',
      width: 1920,
      height: 1080,
      status: 'detected',
      category: 'uncategorized',
      category_source: 'user',
      tags: [],
      ocr_text: '',
      summary: '',
      thumbnail: null,
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-01T00:00:00Z',
    })

    useGalleryStore.getState().removeScreenshot('1')

    const state = useGalleryStore.getState()
    expect(state.screenshots).toHaveLength(0)
  })

  it('should set loading state', () => {
    useGalleryStore.getState().setLoading(true)
    expect(useGalleryStore.getState().isLoading).toBe(true)

    useGalleryStore.getState().setLoading(false)
    expect(useGalleryStore.getState().isLoading).toBe(false)
  })

  it('should set error state', () => {
    useGalleryStore.getState().setError('Test error')
    expect(useGalleryStore.getState().error).toBe('Test error')

    useGalleryStore.getState().setError(null)
    expect(useGalleryStore.getState().error).toBeNull()
  })

  it('should maintain error nullness when setting screenshots', () => {
    useGalleryStore.getState().setError('Test error')
    useGalleryStore.getState().setScreenshots([])

    expect(useGalleryStore.getState().error).toBeNull()
  })
})

describe('useBatchStore', () => {
  beforeEach(() => {
    useBatchStore.getState().clearSelection()
  })

  it('should initialize with empty selection', () => {
    const state = useBatchStore.getState()
    expect(state.selection.selectedIds).toEqual([])
    expect(state.selection.isSelectAll).toBe(false)
  })

  it('should toggle selection for a single item', () => {
    useBatchStore.getState().toggleSelection('1')

    const state = useBatchStore.getState()
    expect(state.selection.selectedIds).toContain('1')
    expect(state.selection.isSelectAll).toBe(false)
  })

  it('should toggle off selection for a single item', () => {
    useBatchStore.getState().toggleSelection('1')
    useBatchStore.getState().toggleSelection('1')

    const state = useBatchStore.getState()
    expect(state.selection.selectedIds).not.toContain('1')
  })

  it('should toggle select all', () => {
    useBatchStore.getState().toggleSelectAll()

    const state = useBatchStore.getState()
    expect(state.selection.selectedIds).toEqual(['all'])
    expect(state.selection.isSelectAll).toBe(true)

    useBatchStore.getState().toggleSelectAll()

    const stateAfter = useBatchStore.getState()
    expect(stateAfter.selection.selectedIds).toEqual([])
    expect(stateAfter.selection.isSelectAll).toBe(false)
  })

  it('should clear selection', () => {
    useBatchStore.getState().toggleSelection('1')
    useBatchStore.getState().toggleSelection('2')
    useBatchStore.getState().toggleSelectAll()

    const stateBefore = useBatchStore.getState()
    expect(stateBefore.selection.selectedIds.length).toBeGreaterThan(0)

    useBatchStore.getState().clearSelection()

    const stateAfter = useBatchStore.getState()
    expect(stateAfter.selection.selectedIds).toEqual([])
    expect(stateAfter.selection.isSelectAll).toBe(false)
  })

  it('should set selected ids directly', () => {
    useBatchStore.getState().setSelectedIds(['1', '2', '3'])

    const state = useBatchStore.getState()
    expect(state.selection.selectedIds).toEqual(['1', '2', '3'])
    expect(state.selection.isSelectAll).toBe(false)
  })

  it('should set select all directly', () => {
    useBatchStore.getState().setSelectAll(true)

    const state = useBatchStore.getState()
    expect(state.selection.selectedIds).toEqual(['all'])
    expect(state.selection.isSelectAll).toBe(true)

    useBatchStore.getState().setSelectAll(false)

    const stateAfter = useBatchStore.getState()
    expect(stateAfter.selection.selectedIds).toEqual([])
    expect(stateAfter.selection.isSelectAll).toBe(false)
  })

  it('should deselect all when setting single ids', () => {
    useBatchStore.getState().setSelectAll(true)
    useBatchStore.getState().setSelectedIds(['1'])

    const state = useBatchStore.getState()
    expect(state.selection.isSelectAll).toBe(false)
  })
})

describe('useSettingsStore', () => {
  it('should initialize with default settings', () => {
    const state = useSettingsStore.getState()
    expect(state.settings.watchFolders).toEqual([])
    expect(state.settings.defaultCategory).toBe('uncategorized')
    expect(state.settings.ocrEnabled).toBe(true)
    expect(state.settings.aiEnabled).toBe(false)
    expect(state.settings.ocrLanguage).toBe('eng')
    expect(state.settings.aiProvider).toBe('ollama')
    expect(state.settings.aiModel).toBe('llama3.2')
    expect(state.settings.theme).toBe('dark')
  })

  it('should set settings', () => {
    const newSettings: any = {
      watchFolders: ['/path/to/folder'],
      defaultCategory: 'work',
      ocrEnabled: false,
      aiEnabled: true,
      ocrLanguage: 'eng',
      aiProvider: 'openai',
      aiModel: 'gpt-4',
      theme: 'light',
    }

    useSettingsStore.getState().setSettings(newSettings)

    const state = useSettingsStore.getState()
    expect(state.settings).toEqual(newSettings)
  })

  it('should update settings partially', () => {
    const initial = useSettingsStore.getState().settings

    useSettingsStore.getState().updateSettings({ ocrEnabled: false })

    const state = useSettingsStore.getState()
    expect(state.settings.ocrEnabled).toBe(false)
    expect(state.settings.watchFolders).toBe(initial.watchFolders)
  })

  it('should update settings with multiple updates', () => {
    useSettingsStore.getState().updateSettings({
      ocrEnabled: false,
      aiEnabled: true,
      theme: 'light',
    })

    const state = useSettingsStore.getState()
    expect(state.settings.ocrEnabled).toBe(false)
    expect(state.settings.aiEnabled).toBe(true)
    expect(state.settings.theme).toBe('light')
  })

  it('should set watch folders', () => {
    const folders = ['/path/1', '/path/2', '/path/3']

    useSettingsStore.getState().setWatchFolders(folders)

    const state = useSettingsStore.getState()
    expect(state.settings.watchFolders).toEqual(folders)
  })

  it('should set default category', () => {
    useSettingsStore.getState().setDefaultCategory('work')

    const state = useSettingsStore.getState()
    expect(state.settings.defaultCategory).toBe('work')
  })

  it('should set OCR enabled', () => {
    useSettingsStore.getState().setOcrEnabled(false)

    const state = useSettingsStore.getState()
    expect(state.settings.ocrEnabled).toBe(false)
  })

  it('should set AI enabled', () => {
    useSettingsStore.getState().setAiEnabled(true)

    const state = useSettingsStore.getState()
    expect(state.settings.aiEnabled).toBe(true)
  })

  it('should set OCR language', () => {
    useSettingsStore.getState().setOcrLanguage('fra')

    const state = useSettingsStore.getState()
    expect(state.settings.ocrLanguage).toBe('fra')
  })

  it('should set AI provider', () => {
    useSettingsStore.getState().setAiProvider('anthropic')

    const state = useSettingsStore.getState()
    expect(state.settings.aiProvider).toBe('anthropic')
  })

  it('should set AI model', () => {
    useSettingsStore.getState().setAiModel('claude-3')

    const state = useSettingsStore.getState()
    expect(state.settings.aiModel).toBe('claude-3')
  })

  it('should set theme', () => {
    useSettingsStore.getState().setTheme('light')

    const state = useSettingsStore.getState()
    expect(state.settings.theme).toBe('light')
  })
})