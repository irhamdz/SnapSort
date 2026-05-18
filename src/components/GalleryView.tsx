import React from 'react'

export function GalleryView() {
  return (
    <div className="flex-1 flex flex-col h-full">
      {/* TODO: Add search bar, filters, and toolbar */}
      <div className="p-4 border-b bg-card">
        <input
          type="text"
          placeholder="Search screenshots..."
          className="w-full px-4 py-2 rounded-md border bg-background"
        />
      </div>

      <div className="flex-1 overflow-auto p-4">
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          {/* TODO: Render screenshot grid */}
        </div>
      </div>
    </div>
  )
}