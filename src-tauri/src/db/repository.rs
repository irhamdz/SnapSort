use anyhow::Result;
use rusqlite::{Connection, params, OptionalExtension};

pub struct ScreenshotRepository {
    conn: Connection,
}

impl ScreenshotRepository {
    pub fn new(conn: &Connection) -> Self {
        Self { conn: Connection::open(conn.path().unwrap()).unwrap() }
    }

    /// Insert a new screenshot (stub)
    pub fn insert(&self, _screenshot: &crate::db::models::Screenshot) -> Result<i64> {
        Ok(0)
    }

    /// Get a screenshot by ID (stub)
    pub fn get_by_id(&self, _id: i64) -> Result<Option<crate::db::models::Screenshot>> {
        Ok(None)
    }

    /// Get all screenshots (stub)
    pub fn get_all(&self, _limit: Option<usize>, _offset: Option<usize>) -> Result<Vec<crate::db::models::Screenshot>> {
        Ok(vec![])
    }

    /// Update a screenshot (stub)
    pub fn update(&self, _screenshot: &crate::db::models::Screenshot) -> Result<()> {
        Ok(())
    }

    /// Delete a screenshot (stub)
    pub fn delete(&self, _id: i64) -> Result<()> {
        Ok(())
    }

    /// Batch delete screenshots (stub)
    pub fn batch_delete(&self, _ids: &[i64]) -> Result<()> {
        Ok(())
    }

    /// Search screenshots using FTS5 (stub)
    pub fn search(&self, _query: &str, _limit: usize) -> Result<Vec<crate::db::models::Screenshot>> {
        Ok(vec![])
    }

    /// Get screenshots by category (stub)
    pub fn get_by_category(&self, _category: &str, _limit: Option<usize>) -> Result<Vec<crate::db::models::Screenshot>> {
        Ok(vec![])
    }

    /// Get screenshots by tag (stub)
    pub fn get_by_tag(&self, _tag: &str, _limit: Option<usize>) -> Result<Vec<crate::db::models::Screenshot>> {
        Ok(vec![])
    }
}

pub struct WatchFolderRepository {
    conn: Connection,
}

impl WatchFolderRepository {
    pub fn new(conn: &Connection) -> Self {
        Self { conn: conn.clone() }
    }

    pub fn insert(&self, _path: &str) -> Result<i64> {
        Ok(0)
    }

    pub fn get_all(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    pub fn remove(&self, _id: i64) -> Result<()> {
        Ok(())
    }
}

pub struct TagRepository {
    conn: Connection,
}

impl TagRepository {
    pub fn new(conn: &Connection) -> Self {
        Self { conn: conn.clone() }
    }

    pub fn get_all(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
}

pub struct CollectionRepository {
    conn: Connection,
}

impl CollectionRepository {
    pub fn new(conn: &Connection) -> Self {
        Self { conn: conn.clone() }
    }

    pub fn create(&self, _name: &str) -> Result<i64> {
        Ok(0)
    }

    pub fn get_all(&self) -> Result<Vec<crate::db::models::Collection>> {
        Ok(vec![])
    }
}