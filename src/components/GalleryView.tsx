import { useEffect, useCallback } from 'react'
import { useGalleryStore } from '../stores/useGalleryStore'
import { useBatchStore } from '../stores/useBatchStore'
import { ScreenshotCard } from './ScreenshotCard'
import { BatchActionBar } from './BatchActionBar'

export function GalleryView() {
  const { screenshots, selectedScreenshotIds, setShowBatchActions, setShowDetailPanel } =
    useGalleryStore()
  const { clearSelection, selection } = useBatchStore()

  const isSelectAll = useBatchStore.getState().selection.isSelectAll
  const selectedCount = selection.selectedIds.length

  // Track selected screenshot IDs for the gallery store
  useEffect(() => {
    useGalleryStore.setState({
      selectedScreenshotIds: isSelectAll ? screenshots.map((s) => s.id) : selection.selectedIds,
    })
  }, [selection.selectedIds, isSelectAll, screenshots])

  const handleSelectAll = useCallback((e?: React.ChangeEvent<HTMLInputElement>) => {
    e?.preventDefault()
    useBatchStore.getState().toggleSelectAll()
  }, [])

  // Show batch actions when selection exists
  useEffect(() => {
    setShowBatchActions(selectedCount > 0 || isSelectAll)
  }, [selectedCount, isSelectAll, setShowBatchActions])

  // Keyboard shortcuts for batch selection
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl/Cmd+A: Select all
      if ((e.ctrlKey || e.metaKey) && e.key === 'a') {
        e.preventDefault()
        useBatchStore.getState().toggleSelectAll()
      }

      // Escape: Clear selection
      if (e.key === 'Escape') {
        e.preventDefault()
        clearSelection()
        setShowDetailPanel(false)
      }

      // Ctrl/Cmd+/: Select none
      if ((e.ctrlKey || e.metaKey) && e.key === '/') {
        e.preventDefault()
        useBatchStore.getState().setSelectAll(false)
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [clearSelection, setShowDetailPanel])

  const handleSelectScreenshot = useCallback(
    (_e: React.MouseEvent, id?: string) => {
      if (id) {
        useBatchStore.getState().toggleSelection(id)
      }
    }
  )

  return (
    <div className="flex-1 flex flex-col h-full">
      {/* Search bar and toolbar */}
      <div className="p-4 border-b bg-card">
        <div className="flex gap-2 mb-2">
          <input
            type="text"
            placeholder="Search screenshots..."
            className="flex-1 px-4 py-2 rounded-md border bg-background"
          />
          <button className="px-4 py-2 rounded-md border bg-background hover:bg-muted">
            Filter
          </button>
        </div>

        {/* Batch action bar header */}
        {selectedCount > 0 || isSelectAll ? (
          <div className="flex items-center justify-between text-sm">
            <div className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={isSelectAll}
                onChange={handleSelectAll}
                className="w-4 h-4 rounded border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800"
              />
              <span className="text-muted-foreground">
                {isSelectAll
                  ? `Select All ${screenshots.length} screenshots`
                  : `Selected ${selectedCount} screenshot${selectedCount !== 1 ? 's' : ''}`}
              </span>
            </div>
          </div>
        ) : null}
      </div>

      {/* Gallery grid */}
      <div className="flex-1 overflow-auto p-4">
        {screenshots.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
            <svg className="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1}
                d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
              />
            </svg>
            <p className="text-lg">No screenshots yet</p>
            <p className="text-sm">Screenshots will appear here as they're detected</p>
          </div>
        ) : (
          <div
            className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4"
            role="grid"
            aria-label="Screenshot gallery"
          >
            {screenshots.map((screenshot) => (
              <ScreenshotCard
                key={screenshot.id}
                id={screenshot.id}
                filename={screenshot.filename}
                filepath={screenshot.filepath}
                width={screenshot.width}
                height={screenshot.height}
                thumbnail={screenshot.thumbnail}
                isSelected={selectedScreenshotIds.includes(screenshot.id)}
                onSelect={handleSelectScreenshot}
                isSelectAll={isSelectAll}
                onToggleSelectAll={handleSelectAll}
              />
            ))}
          </div>
        )}
      </div>

      {/* Batch Action Bar */}
      <BatchActionBar />
    </div>
  )
}