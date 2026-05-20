export function Sidebar() {
  return (
    <div className="w-64 border-r bg-card p-4 flex flex-col gap-6">
      <div className="text-lg font-bold">SnapSort</div>

      <nav className="space-y-1">
        <div className="text-xs font-semibold text-muted-foreground uppercase">Library</div>
        <a href="#" className="block px-3 py-2 rounded-md hover:bg-accent">All Screenshots</a>
        <a href="#" className="block px-3 py-2 rounded-md hover:bg-accent">Favorites</a>
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