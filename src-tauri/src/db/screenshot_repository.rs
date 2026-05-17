// Screenshot Repository
// Handles all database operations for screenshots

use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screenshot {
    pub id: i64,
    pub file_path: String,
    pub filename: String,
    pub file_size: i64,
    pub width: i32,
    pub height: i32,
    pub created_at: DateTime<Utc>,
    pub ingested_at: Option<DateTime<Utc>>,
    pub status: String, // "detected", "queued", "ready", "analyzing", "enriched", "partial", "archived", "deleted"
    pub category: Option<String>,
    pub category_source: String, // "ai" or "user"
    pub tags: String, // JSON array of strings
    pub ocr_text: Option<String>,
    pub summary: Option<String>,
    pub app_detected: Option<String>,
    pub is_archived: bool,
    pub is_favorite: bool,
    pub thumbnail: Option<Vec<u8>>, // BLOB
}

pub struct ScreenshotRepository {
    conn: Connection,
}

impl ScreenshotRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Create a new screenshot record
    pub fn insert(&self, mut screenshot: Screenshot) -> Result<i64> {
        let tags_json = serde_json::to_string(&serde_json::from_str::<Vec<String>>(
            &screenshot.tags
        ).unwrap_or_default())?;

        let now = Utc::now();
        screenshot.ingested_at = Some(now);

        let result = self.conn.execute(
            r#"
            INSERT INTO screenshots
            (file_path, filename, file_size, width, height, created_at, ingested_at,
             status, category, category_source, tags, ocr_text, summary, app_detected,
             is_archived, is_favorite, thumbnail)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
            "#,
            params![
                &screenshot.file_path,
                &screenshot.filename,
                screenshot.file_size,
                screenshot.width,
                screenshot.height,
                screenshot.created_at,
                screenshot.ingested_at.unwrap(),
                &screenshot.status,
                &screenshot.category,
                &screenshot.category_source,
                tags_json,
                &screenshot.ocr_text,
                &screenshot.summary,
                &screenshot.app_detected,
                screenshot.is_archived,
                screenshot.is_favorite,
                screenshot.thumbnail,
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Get all screenshots (optionally filtered)
    pub fn find_all(&self) -> Result<Vec<Screenshot>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM screenshots ORDER BY created_at DESC"
        )?;

        let screenshots = stmt.query_map([], |row| {
            Ok(Screenshot {
                id: row.get(0)?,
                file_path: row.get(1)?,
                filename: row.get(2)?,
                file_size: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                created_at: row.get(6)?,
                ingested_at: row.get(7)?,
                status: row.get(8)?,
                category: row.get(9)?,
                category_source: row.get(10)?,
                tags: row.get(11)?,
                ocr_text: row.get(12)?,
                summary: row.get(13)?,
                app_detected: row.get(14)?,
                is_archived: row.get(15)?,
                is_favorite: row.get(16)?,
                thumbnail: row.get(17)?,
            })
        })?;

        let mut results = Vec::new();
        for result in screenshots {
            results.push(result?);
        }
        Ok(results)
    }

    /// Get a screenshot by ID
    pub fn find_by_id(&self, id: i64) -> Result<Option<Screenshot>> {
        let mut stmt = self.conn.prepare("SELECT * FROM screenshots WHERE id = ?1")?;

        let screenshot = stmt.query_row(params![id], |row| {
            Ok(Screenshot {
                id: row.get(0)?,
                file_path: row.get(1)?,
                filename: row.get(2)?,
                file_size: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                created_at: row.get(6)?,
                ingested_at: row.get(7)?,
                status: row.get(8)?,
                category: row.get(9)?,
                category_source: row.get(10)?,
                tags: row.get(11)?,
                ocr_text: row.get(12)?,
                summary: row.get(13)?,
                app_detected: row.get(14)?,
                is_archived: row.get(15)?,
                is_favorite: row.get(16)?,
                thumbnail: row.get(17)?,
            })
        })?;

        Ok(Some(screenshot))
    }

    /// Update screenshot metadata
    pub fn update(&self, screenshot: &Screenshot) -> Result<()> {
        let tags_json = serde_json::to_string(&serde_json::from_str::<Vec<String>>(
            &screenshot.tags
        ).unwrap_or_default())?;

        self.conn.execute(
            r#"
            UPDATE screenshots
            SET category = ?1, category_source = ?2, tags = ?3,
                ocr_text = ?4, summary = ?5, app_detected = ?6,
                is_archived = ?7, is_favorite = ?8, status = ?9
            WHERE id = ?10
            "#,
            params![
                &screenshot.category,
                &screenshot.category_source,
                tags_json,
                &screenshot.ocr_text,
                &screenshot.summary,
                &screenshot.app_detected,
                screenshot.is_archived,
                screenshot.is_favorite,
                &screenshot.status,
                screenshot.id,
            ],
        )?;

        Ok(())
    }

    /// Delete a screenshot
    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM screenshots WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Batch update screenshots (single transaction)
    pub fn batch_update(&self, screenshots: &[Screenshot]) -> Result<()> {
        let tx = self.conn.transaction()?;

        for screenshot in screenshots {
            let tags_json = serde_json::to_string(&serde_json::from_str::<Vec<String>>(
                &screenshot.tags
            ).unwrap_or_default())?;

            tx.execute(
                r#"
                UPDATE screenshots
                SET category = ?1, category_source = ?2, tags = ?3,
                    ocr_text = ?4, summary = ?5, app_detected = ?6,
                    is_archived = ?7, is_favorite = ?8, status = ?9
                WHERE id = ?10
                "#,
                params![
                    screenshot.category,
                    screenshot.category_source,
                    tags_json,
                    screenshot.ocr_text,
                    screenshot.summary,
                    screenshot.app_detected,
                    screenshot.is_archived,
                    screenshot.is_favorite,
                    screenshot.status,
                    screenshot.id,
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Get all screenshots matching conditions
    pub fn find_filtered(&self, include_archived: bool) -> Result<Vec<Screenshot>> {
        let sql = if include_archived {
            "SELECT * FROM screenshots ORDER BY created_at DESC"
        } else {
            "SELECT * FROM screenshots WHERE is_archived = 0 ORDER BY created_at DESC"
        };

        let mut stmt = self.conn.prepare(sql)?;

        let screenshots = stmt.query_map([], |row| {
            Ok(Screenshot {
                id: row.get(0)?,
                file_path: row.get(1)?,
                filename: row.get(2)?,
                file_size: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                created_at: row.get(6)?,
                ingested_at: row.get(7)?,
                status: row.get(8)?,
                category: row.get(9)?,
                category_source: row.get(10)?,
                tags: row.get(11)?,
                ocr_text: row.get(12)?,
                summary: row.get(13)?,
                app_detected: row.get(14)?,
                is_archived: row.get(15)?,
                is_favorite: row.get(16)?,
                thumbnail: row.get(17)?,
            })
        })?;

        let mut results = Vec::new();
        for result in screenshots {
            results.push(result?);
        }
        Ok(results)
    }
}