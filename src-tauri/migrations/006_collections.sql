-- Migration 006: Collections support
-- Adds collections table and junction table for multiple screenshots per collection

-- Create collections table
CREATE TABLE IF NOT EXISTS collections (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    color TEXT DEFAULT '#3b82f6',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create junction table for many-to-many relationship between screenshots and collections
CREATE TABLE IF NOT EXISTS screenshot_collections (
    screenshot_id INTEGER NOT NULL REFERENCES screenshots(id) ON DELETE CASCADE,
    collection_id TEXT NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    PRIMARY KEY (screenshot_id, collection_id)
);

-- Index for faster queries
CREATE INDEX IF NOT EXISTS idx_screenshot_collections_screenshot ON screenshot_collections(screenshot_id);
CREATE INDEX IF NOT EXISTS idx_screenshot_collections_collection ON screenshot_collections(collection_id);