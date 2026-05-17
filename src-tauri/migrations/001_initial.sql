-- Migration 001: Initial schema for screenshots storage with FTS5
-- Creates the core screenshots table, FTS5 virtual table, and sync triggers

-- Create schema_migrations table to track applied migrations
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create screenshots table
CREATE TABLE IF NOT EXISTS screenshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    filepath TEXT NOT NULL,
    filename TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('detected', 'queued', 'ocr_complete', 'analyzed')),
    category TEXT NOT NULL CHECK(category IN ('code', 'design', 'document', 'web', 'communication', 'media', 'finance', 'reference', 'system', 'other')),
    category_source TEXT NOT NULL CHECK(category_source IN ('ai', 'user')),
    tags TEXT NOT NULL DEFAULT '[]',
    ocr_text TEXT,
    summary TEXT,
    app_detected INTEGER NOT NULL DEFAULT 0 CHECK(app_detected IN (0, 1)),
    is_archived INTEGER NOT NULL DEFAULT 0 CHECK(is_archived IN (0, 1)),
    is_favorite INTEGER NOT NULL DEFAULT 0 CHECK(is_favorite IN (0, 1)),
    thumbnail BLOB,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create FTS5 virtual table for full-text search
CREATE VIRTUAL TABLE IF NOT EXISTS screenshots_fts USING fts5(
    id UNINDEXED,
    filepath,
    filename,
    ocr_text,
    summary,
    content='screenshots',
    content_rowid='id'
);

-- FTS5 sync trigger: INSERT
CREATE TRIGGER IF NOT EXISTS after_screenshots_insert
AFTER INSERT ON screenshots
BEGIN
    INSERT INTO screenshots_fts (id, filepath, filename, ocr_text, summary)
    VALUES (NEW.id, NEW.filepath, NEW.filename, NEW.ocr_text, NEW.summary);
END;

-- FTS5 sync trigger: UPDATE (content tables require delete-then-reinsert)
CREATE TRIGGER IF NOT EXISTS after_screenshots_update
AFTER UPDATE ON screenshots
BEGIN
    INSERT INTO screenshots_fts(screenshots_fts, rowid, filepath, filename, ocr_text, summary)
    VALUES('delete', OLD.id, OLD.filepath, OLD.filename, OLD.ocr_text, OLD.summary);
    INSERT INTO screenshots_fts(rowid, filepath, filename, ocr_text, summary)
    VALUES(NEW.id, NEW.filepath, NEW.filename, NEW.ocr_text, NEW.summary);
END;

-- FTS5 sync trigger: DELETE
CREATE TRIGGER IF NOT EXISTS after_screenshots_delete
AFTER DELETE ON screenshots
BEGIN
    INSERT INTO screenshots_fts(screenshots_fts, rowid, filepath, filename, ocr_text, summary)
    VALUES('delete', OLD.id, OLD.filepath, OLD.filename, OLD.ocr_text, OLD.summary);
END;