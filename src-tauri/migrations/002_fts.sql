-- Migration 002: Update FTS5 schema and add user_notes column

-- Add user_notes column to screenshots table
ALTER TABLE screenshots ADD COLUMN user_notes TEXT;

-- Drop existing FTS5 triggers and table
DROP TRIGGER IF EXISTS after_screenshots_insert;
DROP TRIGGER IF EXISTS after_screenshots_update;
DROP TRIGGER IF EXISTS after_screenshots_delete;
DROP TABLE IF EXISTS screenshots_fts;

-- Recreate FTS5 virtual table with updated columns
CREATE VIRTUAL TABLE screenshots_fts USING fts5(
    summary,
    ocr_text,
    category,
    tags,
    content='screenshots',
    content_rowid='id'
);

-- Backfill FTS with existing rows (no-op on a fresh database)
INSERT INTO screenshots_fts (rowid, summary, ocr_text, category, tags)
SELECT id, summary, ocr_text, category, tags FROM screenshots;

-- FTS5 sync trigger: INSERT
CREATE TRIGGER after_screenshots_insert
AFTER INSERT ON screenshots
BEGIN
    INSERT INTO screenshots_fts (rowid, summary, ocr_text, category, tags)
    VALUES (NEW.id, NEW.summary, NEW.ocr_text, NEW.category, NEW.tags);
END;

-- FTS5 sync trigger: UPDATE (content tables require delete-then-reinsert)
CREATE TRIGGER after_screenshots_update
AFTER UPDATE ON screenshots
BEGIN
    INSERT INTO screenshots_fts(screenshots_fts, rowid, summary, ocr_text, category, tags)
    VALUES('delete', OLD.id, OLD.summary, OLD.ocr_text, OLD.category, OLD.tags);
    INSERT INTO screenshots_fts(rowid, summary, ocr_text, category, tags)
    VALUES(NEW.id, NEW.summary, NEW.ocr_text, NEW.category, NEW.tags);
END;

-- FTS5 sync trigger: DELETE
CREATE TRIGGER after_screenshots_delete
AFTER DELETE ON screenshots
BEGIN
    INSERT INTO screenshots_fts(screenshots_fts, rowid, summary, ocr_text, category, tags)
    VALUES('delete', OLD.id, OLD.summary, OLD.ocr_text, OLD.category, OLD.tags);
END;
