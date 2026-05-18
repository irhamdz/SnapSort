//! Screenshot ingestion pipeline state machine
//!
//! Models the lifecycle of a screenshot as an explicit, persisted state machine.
//! Each state transition is a short DB transaction to ensure durability.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::db::Database;

/// Status of a screenshot in the ingestion pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenshotStatus {
    Detected,
    Queued,
    OcrComplete,
    Ready,
    Analyzing,
    Enriched,
    Partial,
    Archived,
    Deleted,
}

impl ScreenshotStatus {
    /// Check if this is a terminal state (no further transitions)
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            ScreenshotStatus::Ready | ScreenshotStatus::Enriched | ScreenshotStatus::Archived | ScreenshotStatus::Deleted
        )
    }

    /// Check if this is a terminal state that is fully usable
    pub fn is_usable(self) -> bool {
        matches!(self, ScreenshotStatus::Ready | ScreenshotStatus::Enriched)
    }

    /// Check if this is a terminal state that is not usable
    pub fn is_terminated(self) -> bool {
        matches!(self, ScreenshotStatus::Archived | ScreenshotStatus::Deleted)
    }
}

impl std::fmt::Display for ScreenshotStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Validate that a transition is allowed by the state machine
///
/// Returns an error if the transition is invalid.
pub fn is_valid_transition(from: &str, to: &str) -> Result<()> {
    // All transitions must be explicitly defined
    match (from, to) {
        // Initial transition from file detection
        ("detected", "queued") => Ok(()),

        // OCR stage
        ("queued", "ocr_complete") => Ok(()),

        // AI stage (conditional on AI provider)
        ("ocr_complete", "ready") => Ok(()),
        ("ocr_complete", "analyzing") => Ok(()),

        // AI success and failure
        ("analyzing", "enriched") => Ok(()),
        ("analyzing", "partial") => Ok(()),

        // Retry from failed AI
        ("partial", "analyzing") => Ok(()),

        // Auxiliary terminal states
        ("ready", "archived") => Ok(()),
        ("ready", "deleted") => Ok(()),
        ("ready", "queued") => Ok(()), // Re-queue if needed
        ("enriched", "archived") => Ok(()),
        ("enriched", "deleted") => Ok(()),
        ("enriched", "queued") => Ok(()), // Re-queue if needed
        ("partial", "archived") => Ok(()),
        ("partial", "deleted") => Ok(()),
        ("partial", "queued") => Ok(()), // Re-queue if needed

        // Archive and delete from any state
        (_, "archived") => Ok(()),
        (_, "deleted") => Ok(()),

        _ => Err(anyhow::anyhow!(
            "Invalid status transition: {} -> {}",
            from, to
        )),
    }
}

/// Transition a screenshot to the queued state
///
/// Called by the file watcher when a new screenshot is detected.
pub fn enqueue_screenshot(db: &Database, id: i64) -> Result<()> {
    is_valid_transition("detected", "queued")?;
    db.update_screenshot(id, None, None, None, None, Some("queued"), None, None, None, None, None, None, None, None, None)?;
    Ok(())
}

/// Transition a screenshot to the ocr_complete state
///
/// Called by the OCR processor after extracting text.
pub fn mark_ocr_complete(db: &Database, id: i64) -> Result<()> {
    is_valid_transition("queued", "ocr_complete")?;
    db.update_screenshot(id, None, None, None, None, Some("ocr_complete"), None, None, None, None, None, None, None, None, None)?;
    Ok(())
}

/// Transition a screenshot to the ready state
///
/// Called when no AI provider is configured (OCR-only path).
pub fn mark_ready(db: &Database, id: i64) -> Result<()> {
    is_valid_transition("ocr_complete", "ready")?;
    db.update_screenshot(id, None, None, None, None, Some("ready"), None, None, None, None, None, None, None, None, None)?;
    Ok(())
}

/// Transition a screenshot to the analyzing state
///
/// Called when an AI provider is configured and analysis begins.
pub fn mark_analyzing(db: &Database, id: i64) -> Result<()> {
    is_valid_transition("ocr_complete", "analyzing")?;
    db.update_screenshot(id, None, None, None, None, Some("analyzing"), None, None, None, None, None, None, None, None, None)?;
    Ok(())
}

/// Transition a screenshot to the enriched state
///
/// Called when AI analysis completes successfully.
pub fn mark_enriched(
    db: &Database,
    id: i64,
    category: Option<&str>,
    category_source: &str,
    tags: Vec<String>,
    summary: Option<&str>,
    app_detected: Option<&str>,
) -> Result<()> {
    is_valid_transition("analyzing", "enriched")?;
    let app_detected_value = app_detected.map(|s| s.to_string());
    tracing::debug!("mark_enriched: id={}, category={:?}, app_detected={:?}", id, category, app_detected_value);
    db.update_screenshot(
        id,
        None,
        None,
        None,
        None,
        Some("enriched"),
        category,
        Some(category_source),
        Some(tags),
        None,
        summary.map(|s| s.to_string()),
        app_detected_value,
        None,
        None,
        None,
    )?;
    Ok(())
}

/// Transition a screenshot to the partial state
///
/// Called when AI analysis fails or times out.
/// The screenshot retains OCR data and is retry-eligible.
pub fn mark_partial(
    db: &Database,
    id: i64,
    category: Option<&str>,
    category_source: &str,
    tags: Vec<String>,
    summary: Option<&str>,
    app_detected: Option<&str>,
) -> Result<()> {
    is_valid_transition("analyzing", "partial")?;
    db.update_screenshot(
        id,
        None,
        None,
        None,
        None,
        Some("partial"),
        category,
        Some(category_source),
        Some(tags),
        None,
        summary.map(|s| s.to_string()),
        app_detected.map(|s| s.to_string()),
        None,
        None,
        None,
    )?;
    Ok(())
}

/// Transition a screenshot to the archived state
///
/// Called when a screenshot is archived (hidden from main library).
pub fn mark_archived(db: &Database, id: i64) -> Result<()> {
    is_valid_transition("enriched", "archived")?;
    db.update_screenshot(
        id,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(1i32),
        Some(0i32),
        None,
    )?;
    Ok(())
}

/// Transition a screenshot to the deleted state
///
/// Called when a screenshot is permanently deleted.
pub fn mark_deleted(db: &Database, id: i64) -> Result<()> {
    is_valid_transition("enriched", "deleted")?;
    db.update_screenshot(
        id,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(1i32),
        Some(0i32),
        None,
    )?;
    Ok(())
}

/// Retry a failed AI analysis by transitioning from partial to analyzing
///
/// This resets the AI job while preserving OCR data.
pub fn retry_partial_analysis(db: &Database, id: i64) -> Result<()> {
    is_valid_transition("partial", "analyzing")?;
    db.update_screenshot(id, None, None, None, None, Some("analyzing"), None, None, None, None, None, None, None, None, None)?;
    Ok(())
}

/// Generate thumbnail for a screenshot and return the image bytes
pub fn generate_thumbnail(path: &Path) -> Result<Vec<u8>> {
    use crate::thumbnail::generate_thumbnail;

    let result = generate_thumbnail(path)?;
    Ok(result.bytes)
}

/// Insert a new screenshot with detected status and generate thumbnail
///
/// This is the starting point of the ingestion pipeline.
/// Called by the file watcher when a new screenshot is detected.
pub fn ingest_new_file(
    db: &Database,
    filepath: &str,
    filename: &str,
    width: i32,
    height: i32,
) -> Result<i64> {
    // Insert with detected status
    let id = db.insert_screenshot(
        filepath,
        filename,
        width,
        height,
        "detected",
        None,
        "user",
        vec![],
        None,
        None,
        None,
        0,
        0,
        None,
    )?;

    // Generate thumbnail
    let path = std::path::Path::new(filepath);
    if path.exists() {
        if let Ok(thumbnail_bytes) = generate_thumbnail(path) {
            db.update_screenshot(
                id,
                None, None, None, None,
                None,
                None, None, None,
                None, None, None,
                None, None,
                Some(thumbnail_bytes),
            )?;
        }
    }

    // Transition to queued
    enqueue_screenshot(db, id)?;

    Ok(id)
}

/// Run the full ingestion pipeline for a single screenshot
///
/// This function orchestrates:
/// 1. Thumbnail generation
/// 2. OCR processing
/// 3. AI decision (ready vs analyzing)
///
/// Returns the final status of the screenshot.
pub async fn run_pipeline(
    db: &Database,
    filepath: &str,
    filename: &str,
    width: i32,
    height: i32,
    ai_provider_configured: bool,
) -> Result<String> {
    use crate::ocr::apply_ocr_result;

    // Step 1: Ingest new file (insert with thumbnail, transition to queued)
    let id = ingest_new_file(db, filepath, filename, width, height)?;

    // Step 2: Run OCR (transition to ocr_complete)
    let path = std::path::Path::new(filepath);
    if path.exists() {
        let ocr_result = crate::ocr::run_ocr(path)?;
        apply_ocr_result(db, id, ocr_result.text)?;
    } else {
        tracing::warn!("Screenshot file not found: {}", filepath);
    }

    // Step 3: Decision on AI - either ready (no AI) or analyzing (AI configured)
    if ai_provider_configured {
        mark_analyzing(db, id)?;
        Ok("analyzing".to_string())
    } else {
        mark_ready(db, id)?;
        Ok("ready".to_string())
    }
}

/// Get status counts for all screenshots
    ///
    /// Returns a map of status name to count.
    /// This backs the 'Analyzing N' and 'Unanalyzed' smart folder UI.
    pub fn get_status_counts(db: &Database) -> Result<std::collections::HashMap<String, i64>> {
        let conn = db.get_connection().lock().unwrap();

        // Query counts for each status
        let mut stmt = conn.prepare(
            "SELECT status, COUNT(*) as count FROM screenshots GROUP BY status"
        )?;

        let mut counts = std::collections::HashMap::new();
        for s in &["detected", "queued", "ocr_complete", "ready", "analyzing", "enriched", "partial", "archived", "deleted"] {
            counts.insert(s.to_string(), 0i64);
        }

        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        for row in rows {
            let (status, count) = row?;
            counts.insert(status, count);
        }

        Ok(counts)
    }

    /// List screenshots by status
    ///
    /// Useful for the "Unanalyzed" smart folder and batch operations.
    pub fn list_by_status(db: &Database, status: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<crate::db::Screenshot>> {
        use rusqlite::params_from_iter;

        let conn = db.get_connection().lock().unwrap();

    let mut query = format!(
        "SELECT id, filepath, filename, width, height, status, category, category_source, tags, ocr_text, summary, app_detected, user_notes, is_archived, is_favorite, thumbnail, created_at, updated_at FROM screenshots WHERE status = ?"
    );

    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(status)];

    if limit.is_some() || offset.is_some() {
        query.push_str(" LIMIT ? OFFSET ?");
        params.push(Box::new(limit.unwrap_or(100)));
        params.push(Box::new(offset.unwrap_or(0)));
    }

    let mut stmt = conn.prepare(&query)?;

    let mut screenshots = Vec::new();
    let rows = stmt.query_map(params_from_iter(params), |row| {
        Ok(crate::db::Screenshot {
            id: row.get(0)?,
            filepath: row.get(1)?,
            filename: row.get(2)?,
            width: row.get(3)?,
            height: row.get(4)?,
            status: row.get(5)?,
            category: row.get(6)?,
            category_source: row.get(7)?,
            tags: {
                let tags_str = row.get::<_, String>(8)?;
                serde_json::from_str(&tags_str).unwrap_or_default()
            },
            ocr_text: row.get(9)?,
            summary: row.get(10)?,
            app_detected: row.get(11)?,
            user_notes: row.get(12)?,
            is_archived: row.get(13)?,
            is_favorite: row.get(14)?,
            thumbnail: row.get(15)?,
            created_at: row.get(16)?,
            updated_at: row.get(17)?,
        })
    })?;

    for row in rows {
        screenshots.push(row?);
    }

    Ok(screenshots)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn test_db() -> Database {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_dir = std::env::temp_dir()
            .join(format!("snapsort_pipeline_test_{}_{}", std::process::id(), n));
        // Ensure the directory exists
        let _ = std::fs::create_dir_all(&temp_dir);
        Database::open(temp_dir).unwrap()
    }

    fn insert_detected(db: &Database) -> i64 {
        db.insert_screenshot(
            "/test/path.png", "test.png", 800, 600, "detected",
            Some("other"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap()
    }

    #[test]
    fn test_status_is_valid_transition() {
        assert!(is_valid_transition("detected", "queued").is_ok());
        assert!(is_valid_transition("queued", "ocr_complete").is_ok());
        assert!(is_valid_transition("ocr_complete", "ready").is_ok());
        assert!(is_valid_transition("ocr_complete", "analyzing").is_ok());
        assert!(is_valid_transition("analyzing", "enriched").is_ok());
        assert!(is_valid_transition("analyzing", "partial").is_ok());
        assert!(is_valid_transition("partial", "analyzing").is_ok());
        assert!(is_valid_transition("ready", "archived").is_ok());
        assert!(is_valid_transition("ready", "deleted").is_ok());

        // Invalid transitions
        assert!(is_valid_transition("detected", "ocr_complete").is_err());
        assert!(is_valid_transition("queued", "ready").is_err());
        assert!(is_valid_transition("ready", "enriched").is_err());
        assert!(is_valid_transition("partial", "enriched").is_err());
    }

    #[test]
    fn test_status_is_terminal() {
        assert!(ScreenshotStatus::Ready.is_terminal());
        assert!(ScreenshotStatus::Enriched.is_terminal());
        assert!(!ScreenshotStatus::Queued.is_terminal());
        assert!(!ScreenshotStatus::Analyzing.is_terminal());
    }

    #[test]
    fn test_status_is_usable() {
        assert!(ScreenshotStatus::Ready.is_usable());
        assert!(ScreenshotStatus::Enriched.is_usable());
        assert!(!ScreenshotStatus::Partial.is_usable());
        assert!(!ScreenshotStatus::Archived.is_usable());
    }

    #[test]
    fn test_status_is_terminated() {
        assert!(ScreenshotStatus::Archived.is_terminated());
        assert!(ScreenshotStatus::Deleted.is_terminated());
        assert!(!ScreenshotStatus::Ready.is_terminated());
    }

    #[test]
    fn test_enqueue_screenshot() {
        let db = test_db();
        let id = insert_detected(&db);

        enqueue_screenshot(&db, id).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "queued");
    }

    #[test]
    fn test_mark_ocr_complete() {
        let db = test_db();
        let id = db.insert_screenshot(
            "/test/path.png", "test.png", 800, 600, "queued",
            Some("other"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap();

        mark_ocr_complete(&db, id).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "ocr_complete");
    }

    #[test]
    fn test_mark_ready() {
        let db = test_db();
        let id = db.insert_screenshot(
            "/test/path.png", "test.png", 800, 600, "ocr_complete",
            Some("other"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap();

        mark_ready(&db, id).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "ready");
    }

    #[test]
    fn test_mark_analyzing() {
        let db = test_db();
        let id = db.insert_screenshot(
            "/test/path.png", "test.png", 800, 600, "ocr_complete",
            Some("other"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap();

        mark_analyzing(&db, id).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "analyzing");
    }

    #[test]
    fn test_mark_enriched() {
        let db = test_db();
        let id = db.insert_screenshot(
            "/test/path.png", "test.png", 800, 600, "analyzing",
            Some("other"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap();

        mark_enriched(&db, id, Some("code"), "ai", vec!["rust".to_string()], Some("summary"), Some("Terminal")).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        println!("Shot status: {:?}", shot.status);
        println!("Shot category: {:?}", shot.category);
        println!("Shot app_detected: {:?}", shot.app_detected);
        assert_eq!(shot.status, "enriched");
        assert_eq!(shot.category, Some("code".to_string()));
        assert_eq!(shot.app_detected, Some("Terminal".to_string()));
    }

    #[test]
    fn test_mark_partial() {
        let db = test_db();
        let id = db.insert_screenshot(
            "/test/path.png", "test.png", 800, 600, "analyzing",
            Some("other"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap();

        mark_partial(&db, id, Some("code"), "ai", vec!["rust".to_string()], Some("summary"), Some("Terminal")).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "partial");
        assert_eq!(shot.category, Some("code".to_string()));
    }

    #[test]
    fn test_retry_partial_analysis() {
        let db = test_db();
        let id = db.insert_screenshot(
            "/test/path.png", "test.png", 800, 600, "partial",
            Some("other"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap();

        retry_partial_analysis(&db, id).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "analyzing");
    }

    #[test]
    fn test_get_status_counts() {
        let db = test_db();

        for i in 0..5 {
            db.insert_screenshot(
                &format!("/test/path{}.png", i),
                &format!("test{}.png", i),
                800, 600, "queued",
                Some("code"), "ai", vec![], None, None, None, 0, 0, None,
            ).unwrap();
        }

        for i in 5..10 {
            db.insert_screenshot(
                &format!("/test/path{}.png", i),
                &format!("test{}.png", i),
                800, 600, "analyzing",
                Some("design"), "ai", vec![], None, None, None, 0, 0, None,
            ).unwrap();
        }

        let counts = get_status_counts(&db).unwrap();

        assert_eq!(counts.get("detected"), Some(&0i64));
        assert_eq!(counts.get("queued"), Some(&5i64));
        assert_eq!(counts.get("analyzing"), Some(&5i64));
        assert_eq!(counts.get("enriched"), Some(&0i64));
    }

    #[test]
    fn test_list_by_status() {
        let db = test_db();

        db.insert_screenshot("/test/queued.png", "queued.png", 800, 600, "queued", Some("code"), "ai", vec![], None, None, None, 0, 0, None).unwrap();
        db.insert_screenshot("/test/enriched.png", "enriched.png", 800, 600, "enriched", Some("design"), "ai", vec![], None, None, None, 0, 0, None).unwrap();

        let queued = list_by_status(&db, "queued", Some(10), Some(0)).unwrap();
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].status, "queued");

        let enriched = list_by_status(&db, "enriched", Some(10), Some(0)).unwrap();
        assert_eq!(enriched.len(), 1);
        assert_eq!(enriched[0].status, "enriched");
    }

    #[test]
    fn test_pipeline_persistence_reopen() {
        // Database::open deletes the file in test mode, so we verify persistence by
        // reading the row back through a raw rusqlite connection on the same file.
        use std::sync::atomic::{AtomicU64, Ordering};
        static PERSIST_COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = PERSIST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_dir = std::env::temp_dir()
            .join(format!("snapsort_persist_test_{}_{}", std::process::id(), n));
        let _ = std::fs::create_dir_all(&temp_dir);
        let db_path = temp_dir.join("snapsort.db");

        let id = {
            let db1 = Database::open(temp_dir.clone()).unwrap();
            let id = db1.insert_screenshot(
                "/test/path.png", "test.png", 800, 600, "analyzing",
                Some("other"), "ai", vec![], None, None, None, 0, 0, None,
            ).unwrap();
            mark_enriched(&db1, id, Some("code"), "ai", vec!["rust".to_string()], Some("summary"), Some("Terminal")).unwrap();
            id
        };

        // Read back via raw connection on the same file — simulates a restart
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        let (status, category): (String, Option<String>) = conn
            .query_row(
                "SELECT status, category FROM screenshots WHERE id = ?",
                rusqlite::params![id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(status, "enriched");
        assert_eq!(category, Some("code".to_string()));
    }
}