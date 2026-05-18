-- Migration 001: Initial schema for screenshots storage with FTS5
-- Incorporates spec/0012 taxonomy constraints and spec/0009 AI-optional relaxations

-- Track applied migrations
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Core screenshots table
CREATE TABLE IF NOT EXISTS screenshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    filepath TEXT NOT NULL,
    filename TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    -- spec/0009: 'ready' is the terminal usable state after OCR with no AI provider
    status TEXT NOT NULL DEFAULT 'detected'
        CHECK(status IN ('detected', 'queued', 'ready', 'ocr_complete', 'analyzed', 'enriched')),
    -- spec/0009: category is nullable — screenshots without AI reach 'ready' uncategorized
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

-- FTS5 virtual table — indexes all searchable text fields
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
