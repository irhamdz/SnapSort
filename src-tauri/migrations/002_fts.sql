-- Migration 002: Consolidate FTS5 schema
-- user_notes column is handled in 001; this migration ensures FTS triggers
-- reflect the final column set (ocr_text, summary, category, tags).

-- Drop old FTS triggers if they exist under legacy names
DROP TRIGGER IF EXISTS screenshots_ai_insert;
DROP TRIGGER IF EXISTS screenshots_ai_update;
DROP TRIGGER IF EXISTS screenshots_ai_delete;

-- Drop and recreate FTS table to ensure it matches the intended schema
DROP TABLE IF EXISTS screenshots_fts;

CREATE VIRTUAL TABLE screenshots_fts USING fts5(
    ocr_text,
    summary,
    category,
    tags,
    content='screenshots',
    content_rowid='id'
);

-- Backfill FTS with existing rows (no-op on fresh database)
INSERT INTO screenshots_fts (rowid, ocr_text, summary, category, tags)
SELECT id, ocr_text, summary, category, tags FROM screenshots;

-- FTS5 sync trigger: INSERT
DROP TRIGGER IF EXISTS after_screenshots_insert;
CREATE TRIGGER after_screenshots_insert
AFTER INSERT ON screenshots
BEGIN
    INSERT INTO screenshots_fts (rowid, ocr_text, summary, category, tags)
    VALUES (NEW.id, NEW.ocr_text, NEW.summary, NEW.category, NEW.tags);
END;

-- FTS5 sync trigger: UPDATE
DROP TRIGGER IF EXISTS after_screenshots_update;
CREATE TRIGGER after_screenshots_update
AFTER UPDATE ON screenshots
BEGIN
    INSERT INTO screenshots_fts(screenshots_fts, rowid, ocr_text, summary, category, tags)
    VALUES ('delete', OLD.id, OLD.ocr_text, OLD.summary, OLD.category, OLD.tags);
    INSERT INTO screenshots_fts(rowid, ocr_text, summary, category, tags)
    VALUES (NEW.id, NEW.ocr_text, NEW.summary, NEW.category, NEW.tags);
END;

-- FTS5 sync trigger: DELETE
DROP TRIGGER IF EXISTS after_screenshots_delete;
CREATE TRIGGER after_screenshots_delete
AFTER DELETE ON screenshots
BEGIN
    INSERT INTO screenshots_fts(screenshots_fts, rowid, ocr_text, summary, category, tags)
    VALUES ('delete', OLD.id, OLD.ocr_text, OLD.summary, OLD.category, OLD.tags);
END;
