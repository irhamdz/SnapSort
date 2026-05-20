import React from 'react'

interface ScreenshotCardProps {
  filename: string
  thumbnail?: string | null
  isSelected: boolean
  onSelect: (e?: React.MouseEvent, id?: string) => void
}

export function ScreenshotCard({
  filename,
  thumbnail,
  isSelected,
  onSelect,
}: ScreenshotCardProps) {
  const [showDetail, setShowDetail] = React.useState(false)

  const handleClick = (e: React.MouseEvent) => {
    onSelect(e)
    setShowDetail(true)
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault()
      setShowDetail(true)
    }
    if (e.key === ' ') {
      e.preventDefault()
      // Don't call onSelect on space, it's for scrolling
    }
  }

  return (
    <div
      className={`relative group rounded-lg overflow-hidden border-2 transition-all ${
        isSelected
          ? 'border-blue-500 shadow-lg shadow-blue-500/25'
          : 'border-transparent hover:border-gray-300 dark:hover:border-gray-700'
      } ${showDetail ? 'ring-2 ring-blue-500' : ''}`}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      role="button"
      tabIndex={0}
    >
      {/* Checkbox overlay for selection */}
      <div
        className="absolute top-2 left-2 z-10"
        onClick={(e) => {
          e.stopPropagation()
          onSelect()
        }}
      >
        <input
          type="checkbox"
          checked={isSelected}
          onChange={(e) => {
            e.stopPropagation()
            onSelect()
          }}
          onClick={(e) => e.stopPropagation()}
          className="w-5 h-5 rounded border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 cursor-pointer"
        />
      </div>

      {/* Thumbnail or placeholder */}
      <div className="aspect-video bg-muted flex items-center justify-center">
        {thumbnail ? (
          <img
            src={thumbnail}
            alt={filename}
            className="w-full h-full object-contain"
            onError={(e) => {
              const target = e.target as HTMLImageElement
              target.style.display = 'none'
              target.nextElementSibling?.remove()
            }}
          />
        ) : (
          <svg
            className="w-12 h-12 text-muted-foreground"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
            />
          </svg>
        )}
      </div>

      {/* Filename overlay */}
      <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-2">
        <p className="text-xs text-white truncate" title={filename}>
          {filename}
        </p>
      </div>

      {/* Hover actions */}
      <div className="absolute top-2 right-2 z-10 opacity-0 group-hover:opacity-100 transition-opacity">
        <button
          onClick={(e) => {
            e.stopPropagation()
            setShowDetail(true)
          }}
          className="p-1.5 bg-black/50 rounded-md hover:bg-black/70 text-white"
          title="View details"
        >
          <svg
            className="w-4 h-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
            />
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
            />
          </svg>
        </button>
      </div>
    </div>
  )
}