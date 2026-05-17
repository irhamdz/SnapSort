import React from 'react'
import { GalleryView } from './GalleryView'
import { Sidebar } from './Sidebar'

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

export function Sidebar() {
  return (
    <div className="w-64 border-r bg-card p-4 flex flex-col gap-6">
      <div className="text-lg font-bold">SnapSort</div>

      <nav className="space-y-1">
        <div className="text-xs font-semibold text-muted-foreground uppercase">Library</div>
        <a href="#" className="block px-3 py-2 rounded-md hover:bg-accent">All Screenshots</a>
        <a href="#" className="block px-3 py-2 rounded-md hover:bg-accent">Favorites</a>
        <a href="#" className="block px-3 py-2 rounded-md hover:bg-accent">Unanalyzed</a>
      </nav>

      <nav className="space-y-1">
        <div className="text-xs font-semibold text-muted-foreground uppercase">Categories</div>
        {/* TODO: Render categories from DB */}
      </nav>

      <nav className="space-y-1">
        <div className="text-xs font-semibold text-muted-foreground uppercase">Collections</div>
        {/* TODO: Render collections from DB */}
      </nav>

      <nav className="space-y-1">
        <div className="text-xs font-semibold text-muted-foreground uppercase">Smart Folders</div>
        {/* TODO: Render smart folders from DB */}
      </nav>
    </div>
  )
}