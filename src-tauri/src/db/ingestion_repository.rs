// Ingestion Repository
// Handles ingestion events and queue management

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionEvent {
    pub id: i64,
    pub file_path: String,
    pub detected_at: DateTime<Utc>,
    pub status: String, // "detected", "queued", "processing", "completed", "failed"
}

pub struct IngestionRepository {
    conn: rusqlite::Connection,
}

impl IngestionRepository {
    pub fn new(conn: rusqlite::Connection) -> Self {
        Self { conn }
    }

    /// Create an ingestion event
    pub fn insert(&self, event: IngestionEvent) -> Result<i64> {
        let result = self.conn.execute(
            "INSERT INTO ingestion_events (file_path, detected_at, status) VALUES (?1, ?2, ?3)",
            params![
                &event.file_path,
                event.detected_at,
                &event.status,
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Get pending ingestion events
    pub fn find_pending(&self) -> Result<Vec<IngestionEvent>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM ingestion_events WHERE status = 'queued' ORDER BY detected_at ASC"
        )?;

        let events = stmt.query_map([], |row| {
            Ok(IngestionEvent {
                id: row.get(0)?,
                file_path: row.get(1)?,
                detected_at: row.get(2)?,
                status: row.get(3)?,
            })
        })?;

        let mut results = Vec::new();
        for result in events {
            results.push(result?);
        }
        Ok(results)
    }

    /// Update event status
    pub fn update_status(&self, id: i64, status: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE ingestion_events SET status = ?1 WHERE id = ?2",
            params![status, id],
        )?;
        Ok(())
    }
}