-- Migration 004: Add state machine status values and update CHECK constraints
-- Adds: partial, archived, deleted states to the status CHECK constraint
-- Also updates the default status from 'analyzed' to 'detected'

BEGIN;

-- Create new screenshots table with updated status CHECK constraint
-- We must recreate because SQLite cannot ALTER COLUMN a CHECK constraint

CREATE TABLE screenshots_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    filepath TEXT NOT NULL,
    filename TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    -- Updated status CHECK constraint with all 9 state machine states
    status TEXT NOT NULL DEFAULT 'detected'
        CHECK(status IN ('detected', 'queued', 'ocr_complete', 'ready', 'analyzing', 'enriched', 'partial', 'archived', 'deleted')),
    -- spec/0012: category is nullable — screenshots without AI reach 'ready' uncategorized
    category TEXT
        CHECK(category IS NULL OR category IN (
            'code', 'design', 'document', 'web',
            'communication', 'media', 'finance', 'reference', 'system', 'other'
        )),
    -- spec/0012: provenance is always tracked
    category_source TEXT NOT NULL DEFAULT 'user'
        CHECK(category_source IN ('ai', 'user')),
    tags TEXT NOT NULL DEFAULT '[]',
    ocr_text TEXT,
    summary TEXT,
    -- spec/0009: app_detected stores detected app name as text, nullable
    app_detected TEXT,
    user_notes TEXT,
    is_archived INTEGER NOT NULL DEFAULT 0 CHECK(is_archived IN (0, 1)),
    is_favorite INTEGER NOT NULL DEFAULT 0 CHECK(is_favorite IN (0, 1)),
    thumbnail BLOB,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Copy all existing data
INSERT INTO screenshots_new (
    id, filepath, filename, width, height, status, category, category_source,
    tags, ocr_text, summary, app_detected, user_notes, is_archived, is_favorite, thumbnail,
    created_at, updated_at
)
SELECT
    id, filepath, filename, width, height, status, category, category_source,
    tags, ocr_text, summary, app_detected, user_notes, is_archived, is_favorite, thumbnail,
    created_at, updated_at
FROM screenshots;

-- Drop old table
DROP TABLE screenshots;

-- Rename new table
ALTER TABLE screenshots_new RENAME TO screenshots;

-- Recreate FTS5 virtual table
CREATE VIRTUAL TABLE IF NOT EXISTS screenshots_fts USING fts5(
    ocr_text,
    summary,
    category,
    tags,
    content='screenshots',
    content_rowid='id'
);

-- FTS5 sync trigger: INSERT
CREATE TRIGGER IF NOT EXISTS after_screenshots_insert
AFTER INSERT ON screenshots
BEGIN
    INSERT INTO screenshots_fts (rowid, ocr_text, summary, category, tags)
    VALUES (NEW.id, NEW.ocr_text, NEW.summary, NEW.category, NEW.tags);
END;

-- FTS5 sync trigger: UPDATE
CREATE TRIGGER IF NOT EXISTS after_screenshots_update
AFTER UPDATE ON screenshots
BEGIN
    INSERT INTO screenshots_fts(screenshots_fts, rowid, ocr_text, summary, category, tags)
    VALUES ('delete', OLD.id, OLD.ocr_text, OLD.summary, OLD.category, OLD.tags);
    INSERT INTO screenshots_fts(rowid, ocr_text, summary, category, tags)
    VALUES (NEW.id, NEW.ocr_text, NEW.summary, NEW.category, NEW.tags);
END;

-- FTS5 sync trigger: DELETE
CREATE TRIGGER IF NOT EXISTS after_screenshots_delete
AFTER DELETE ON screenshots
BEGIN
    INSERT INTO screenshots_fts(screenshots_fts, rowid, ocr_text, summary, category, tags)
    VALUES ('delete', OLD.id, OLD.ocr_text, OLD.summary, OLD.category, OLD.tags);
END;

-- Update schema_migrations to record this migration
INSERT INTO schema_migrations (version) VALUES (4);

COMMIT;