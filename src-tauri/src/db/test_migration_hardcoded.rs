use anyhow::Result;
use rusqlite::{Connection, params};

fn run_migration(conn: &Connection) -> Result<()> {
    let sql = r#"
-- Migration 001: Initial schema for screenshots storage with FTS5
-- Updated for AI optional enforcement
-- Creates the core screenshots table, FTS5 virtual table, and sync triggers

-- Create schema_migrations table to track applied migrations
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create screenshots table with AI-optional fields
CREATE TABLE IF NOT EXISTS screenshots (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    ingested_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'detected',
    category TEXT,
    category_source TEXT NOT NULL DEFAULT 'user',
    tags TEXT NOT NULL DEFAULT '[]',
    ocr_text TEXT,
    summary TEXT,
    app_detected TEXT,
    is_archived INTEGER NOT NULL DEFAULT 0,
    is_favorite INTEGER NOT NULL DEFAULT 0,
    thumbnail BLOB,
    ai_analyzed_at TEXT
);

-- Create FTS5 virtual table for full-text search
CREATE VIRTUAL TABLE IF NOT EXISTS screenshots_fts USING fts5(
    id,
    filename,
    file_path,
    ocr_text,
    summary,
    category,
    app_detected,
    tags,
    content='screenshots',
    content_rowid='id'
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_screenshots_created_at ON screenshots(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_screenshots_category ON screenshots(category);
CREATE INDEX IF NOT EXISTS idx_screenshots_is_archived ON screenshots(is_archived);
CREATE INDEX IF NOT EXISTS idx_screenshots_is_favorite ON screenshots(is_favorite);
CREATE INDEX IF NOT EXISTS idx_screenshots_status ON screenshots(status);

-- FTS5 triggers to keep search table in sync
CREATE TRIGGER IF NOT EXISTS screenshots_ai_insert AFTER INSERT ON screenshots BEGIN
    INSERT INTO screenshots_fts(rowid, id, filename, file_path, ocr_text, summary, category, app_detected, tags)
    VALUES (new.id, new.id, new.filename, new.file_path, new.ocr_text, new.summary, new.category, new.app_detected, new.tags);
END;

CREATE TRIGGER IF NOT EXISTS screenshots_ai_update AFTER UPDATE ON screenshots BEGIN
    UPDATE screenshots_fts
    SET id = new.id,
        filename = new.filename,
        file_path = new.file_path,
        ocr_text = new.ocr_text,
        summary = new.summary,
        category = new.category,
        app_detected = new.app_detected,
        tags = new.tags
    WHERE rowid = old.id;
END;

CREATE TRIGGER IF NOT EXISTS screenshots_ai_delete AFTER DELETE ON screenshots BEGIN
    DELETE FROM screenshots_fts WHERE rowid = old.id;
END;

-- Watch folders table
CREATE TABLE IF NOT EXISTS watch_folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    added_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Collections table
CREATE TABLE IF NOT EXISTS collections (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Screenshot-Collections junction table
CREATE TABLE IF NOT EXISTS screenshot_collections (
    screenshot_id TEXT NOT NULL,
    collection_id TEXT NOT NULL,
    added_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (screenshot_id, collection_id),
    FOREIGN KEY (screenshot_id) REFERENCES screenshots(id) ON DELETE CASCADE,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE
);

-- AI Providers table
CREATE TABLE IF NOT EXISTS ai_providers (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL,
    name TEXT NOT NULL,
    config TEXT NOT NULL DEFAULT '{}',
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Settings table
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Insert default settings
INSERT INTO settings (key, value) VALUES
    ('theme', '"system"'),
    ('language', '"en"'),
    ('notifications_enabled', '"true"'),
    ('notifications_new_screenshot', '"true"'),
    ('notifications_ai_complete', '"true"') ON CONFLICT(key) DO UPDATE SET value = excluded.value;
"#;

    conn.execute_batch(sql)?;

    let version: i32 = conn.query_row("SELECT MAX(version) FROM schema_migrations", [], |row| row.get(0)).unwrap();
    println!("Migration version: {}", version);

    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    println!("Database opened successfully");
    run_migration(&conn)?;
    println!("Test passed!");
    Ok(())
}