import { useState } from 'react'
import { useGalleryStore } from '../stores/useGalleryStore'
import { useBatchStore } from '../stores/batchStore'

export function BatchActionBar() {
  const { screenshots } = useGalleryStore()
  const { setShowBatchActions, setShowDetailPanel } = useGalleryStore.getState()

  const selectedCount = useBatchStore.getState().selectedCount
  const isSelectAll = useBatchStore.getState().isSelectAll
  const lastErrors = useBatchStore.getState().lastErrors

  const [showCategoryModal, setShowCategoryModal] = useState(false)
  const [showTagModal, setShowTagModal] = useState(false)
  const [showCollectionModal, setShowCollectionModal] = useState(false)
  const [showMoveModal, setShowMoveModal] = useState(false)
  const [category, setCategory] = useState('')
  const [tags, setTags] = useState<string[]>([])
  const [newTag, setNewTag] = useState('')
  const [destinationPath, setDestinationPath] = useState('')

  const handleClearSelection = () => {
    useBatchStore.getState().clearSelection()
    setShowBatchActions(false)
  }

  const handleClose = () => {
    useBatchStore.getState().clearSelection()
    setShowBatchActions(false)
    setShowDetailPanel(false)
  }

  const handleCategorize = async () => {
    await useBatchStore.getState().categorizeSelected(category)
    setShowCategoryModal(false)
    setCategory('')
  }

  const handleTag = async (add: boolean) => {
    await useBatchStore.getState().tagSelected(tags, add)
    setShowTagModal(false)
    setTags([])
    setNewTag('')
  }

  const handleArchive = async () => {
    await useBatchStore.getState().archiveSelected()
    setShowDetailPanel(false)
  }

  const handleMove = async () => {
    await useBatchStore.getState().moveSelected(destinationPath)
    setShowMoveModal(false)
    setDestinationPath('')
  }

  const handleAddToCollection = async (collectionId: string) => {
    await useBatchStore.getState().addToCollectionSelected(collectionId)
    setShowCollectionModal(false)
  }

  const toggleTag = (tag: string) => {
    setTags(prev => prev.includes(tag) ? prev.filter(t => t !== tag) : [...prev, tag])
  }

  if (selectedCount === 0 && !isSelectAll) {
    return null
  }

  return (
    <>
      {/* Last errors display */}
      {lastErrors && lastErrors.length > 0 && (
        <div className="fixed bottom-24 left-0 right-0 z-50">
          <div className="max-w-7xl mx-auto px-4">
            <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-4 text-sm text-red-300">
              <p className="font-medium mb-2">Some operations failed:</p>
              <ul className="space-y-1">
                {lastErrors.map((error, i) => (
                  <li key={i}>
                    {error.path || `Screenshot ${error.id}`}: {error.error}
                  </li>
                ))}
              </ul>
              <button
                onClick={() => useBatchStore.getState().clearSelection()}
                className="mt-2 px-3 py-1 bg-red-500/20 hover:bg-red-500/30 rounded text-red-200"
              >
                Clear Errors
              </button>
            </div>
          </div>
        </div>
      )}

      <div className="fixed bottom-0 left-0 right-0 bg-card border-t shadow-lg z-50">
        <div className="max-w-7xl mx-auto px-4 py-3">
          <div className="flex items-center justify-between gap-4">
            <div className="flex items-center gap-4">
              <span className="text-sm font-medium text-foreground">
                {isSelectAll
                  ? `Select All ${screenshots.length} screenshots`
                  : `Selected ${selectedCount} screenshot${selectedCount !== 1 ? 's' : ''}`}
              </span>

              <div className="flex items-center gap-1">
                <button
                  onClick={() => setShowCategoryModal(true)}
                  className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
                >
                  Categorize
                </button>
                <button
                  onClick={() => setShowTagModal(true)}
                  className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
                >
                  Tag
                </button>
                <button
                  onClick={() => setShowCollectionModal(true)}
                  className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
                >
                  Add to Collection
                </button>
              </div>

              <div className="flex items-center gap-1">
                <button
                  onClick={async () => {
                    if (confirm(`Archive ${selectedCount} screenshots?`)) {
                      await handleArchive()
                    }
                  }}
                  className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
                >
                  Archive
                </button>
                <button
                  onClick={() => setShowMoveModal(true)}
                  className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
                >
                  Move
                </button>
                <button
                  onClick={async () => {
                    if (confirm(`Delete ${selectedCount} screenshots? This cannot be undone.`)) {
                      await useBatchStore.getState().deleteSelected()
                    }
                  }}
                  className="px-3 py-1.5 text-sm rounded-md border border-red-500 hover:bg-red-500/10 text-red-500"
                >
                  Delete
                </button>
              </div>
            </div>

            <div className="flex items-center gap-2">
              <button
                onClick={handleCategorize}
                className="px-3 py-1.5 text-sm rounded-md bg-primary hover:bg-primary/90"
              >
                Apply
              </button>
              <button
                onClick={handleClearSelection}
                className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
              >
                Clear
              </button>
              <button
                onClick={handleClose}
                className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Category Modal */}
      {showCategoryModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onClick={() => setShowCategoryModal(false)}>
          <div className="bg-card rounded-lg p-6 max-w-md w-full" onClick={e => e.stopPropagation()}>
            <h3 className="text-lg font-bold mb-4">Categorize {selectedCount} screenshots</h3>
            <input
              type="text"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              placeholder="Enter category name"
              className="w-full px-3 py-2 rounded-md border mb-4"
            />
            <div className="flex justify-end gap-2">
              <button
                onClick={() => setShowCategoryModal(false)}
                className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
              >
                Cancel
              </button>
              <button
                onClick={handleCategorize}
                className="px-3 py-1.5 text-sm rounded-md bg-primary hover:bg-primary/90"
              >
                Apply
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Tag Modal */}
      {showTagModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onClick={() => setShowTagModal(false)}>
          <div className="bg-card rounded-lg p-6 max-w-md w-full" onClick={e => e.stopPropagation()}>
            <h3 className="text-lg font-bold mb-4">Tag {selectedCount} screenshots</h3>
            <div className="flex gap-2 mb-4">
              <input
                type="text"
                value={newTag}
                onChange={(e) => setNewTag(e.target.value)}
                placeholder="Add new tag"
                onKeyPress={(e) => e.key === 'Enter' && (e.preventDefault(), toggleTag(newTag), setNewTag(''))}
                className="flex-1 px-3 py-2 rounded-md border"
              />
              <button
                onClick={() => {
                  if (newTag.trim()) {
                    toggleTag(newTag.trim())
                    setNewTag('')
                  }
                }}
                className="px-3 py-2 rounded-md bg-primary hover:bg-primary/90"
              >
                Add
              </button>
            </div>
            <div className="flex flex-wrap gap-2 mb-4">
              {tags.map(tag => (
                <span
                  key={tag}
                  onClick={() => toggleTag(tag)}
                  className="px-2 py-1 rounded-md bg-muted hover:bg-muted/80 cursor-pointer"
                >
                  {tag} ×
                </span>
              ))}
            </div>
            <div className="flex justify-end gap-2">
              <button
                onClick={() => {
                  setTags([])
                  setShowTagModal(false)
                }}
                className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
              >
                Clear All
              </button>
              <button
                onClick={() => handleTag(true)}
                className="px-3 py-1.5 text-sm rounded-md bg-primary hover:bg-primary/90"
              >
                Apply
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Collection Modal */}
      {showCollectionModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onClick={() => setShowCollectionModal(false)}>
          <div className="bg-card rounded-lg p-6 max-w-md w-full" onClick={e => e.stopPropagation()}>
            <h3 className="text-lg font-bold mb-4">Add to Collection</h3>
            <div className="mb-4">
              <input
                type="text"
                placeholder="Enter collection name"
                className="w-full px-3 py-2 rounded-md border"
              />
            </div>
            <div className="flex justify-end gap-2">
              <button
                onClick={() => setShowCollectionModal(false)}
                className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
              >
                Cancel
              </button>
              <button
                onClick={() => handleAddToCollection('')}
                className="px-3 py-1.5 text-sm rounded-md bg-primary hover:bg-primary/90"
              >
                Create & Add
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Move Modal */}
      {showMoveModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onClick={() => setShowMoveModal(false)}>
          <div className="bg-card rounded-lg p-6 max-w-md w-full" onClick={e => e.stopPropagation()}>
            <h3 className="text-lg font-bold mb-4">Move {selectedCount} screenshots</h3>
            <input
              type="text"
              value={destinationPath}
              onChange={(e) => setDestinationPath(e.target.value)}
              placeholder="Destination folder path"
              className="w-full px-3 py-2 rounded-md border mb-4"
            />
            <div className="flex justify-end gap-2">
              <button
                onClick={() => setShowMoveModal(false)}
                className="px-3 py-1.5 text-sm rounded-md border hover:bg-muted"
              >
                Cancel
              </button>
              <button
                onClick={handleMove}
                className="px-3 py-1.5 text-sm rounded-md bg-primary hover:bg-primary/90"
              >
                Move
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  )
}