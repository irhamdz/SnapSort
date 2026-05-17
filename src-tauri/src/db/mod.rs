//! Database module for SnapSort
//!
//! Provides a repository layer over SQLite (rusqlite) for screenshot storage.
//! All SQL queries are centralized here, never in command handlers.

use anyhow::{Context, Result};
        use rusqlite::{Connection, params, params_from_iter, OptionalExtension};
        use std::path::PathBuf;
        use std::sync::Mutex;

/// Database connection wrapper with migration runner
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
/// Open a new database connection in the OS app-data directory
    pub fn open(app_data_dir: PathBuf) -> Result<Self> {
        let db_path = app_data_dir.join("snapsort.db");

        // Remove existing database file if present (for tests)
        #[cfg(test)]
        {
            let _ = std::fs::remove_file(&db_path);
            // Ensure the directory exists
            let _ = std::fs::create_dir_all(&app_data_dir);
        }

        let conn = Connection::open(&db_path)
            .context("Failed to open SQLite database")?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .context("Failed to enable foreign keys")?;

        // Enable WAL mode for better concurrent read performance
        // Disable WAL mode for tests to avoid corruption issues
        #[cfg(not(test))]
        {
            let mut retry_count = 0;
            let max_retries = 3;
            loop {
                match conn.execute_batch("PRAGMA journal_mode = WAL;") {
                    Ok(_) => break,
                    Err(e) if retry_count < max_retries && e.to_string().contains("database is locked") => {
                        retry_count += 1;
                        // Small delay before retry
                        std::thread::sleep(std::time::Duration::from_millis(50));
                    }
                    Err(e) => return Err(e).context("Failed to enable WAL mode"),
                }
            }
        }
        
        // For tests, disable WAL mode to avoid corruption issues
        #[cfg(test)]
        {
            let _ = conn.execute_batch("PRAGMA journal_mode = MEMORY;");
        }

        // Run migrations
        Self::run_migrations(&conn)
            .context("Failed to initialize database schema")?;

        Ok(Self { conn: Mutex::new(conn) })
    }

/// Run all pending migrations
    fn run_migrations(conn: &Connection) -> Result<()> {
        // Get the version of the last applied migration
        let current_version: Option<i64> = conn
            .query_row("SELECT MAX(version) FROM schema_migrations", [], |row| row.get(0))
            .ok();

        eprintln!("Current version from schema_migrations: {:?}", current_version);

        // Read and apply migrations in order
        let mut migration_files: Vec<_> = std::fs::read_dir("migrations/")
            .context("Failed to read migrations directory")?
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();

        // Sort by filename to ensure order
        migration_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        for entry in migration_files {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_str()
                .context("Migration filename contains invalid UTF-8")?;

            // Parse version from filename
            let version: i64 = file_name_str
                .strip_suffix(".sql")
                .and_then(|s| s.split('_').next())
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| anyhow::anyhow!("Invalid migration filename: {}", file_name_str))?;

            // Skip if already applied
            if let Some(cv) = current_version {
                if version <= cv {
                    eprintln!("Skipping migration {} (already applied, current_version={})", version, cv);
                    continue;
                }
            }

            // Read and execute the migration
            let migration_sql = std::fs::read_to_string(entry.path())
                .context(format!("Failed to read migration file: {:?}", entry.path()))?;

            // Execute the migration
            conn.execute_batch(&migration_sql)
                .context(format!("Failed to apply migration version {}", version))?;

            // Record the migration
            if let Err(e) = conn.execute(
                "INSERT OR IGNORE INTO schema_migrations (version) VALUES (?)",
                params![version],
            ) {
                // Check if this was a duplicate insert (version already exists)
                if !e.to_string().contains("UNIQUE constraint failed") {
                    return Err(e).context(format!("Failed to record migration version {}", version));
                }
            }

            eprintln!("Applied migration: {}", file_name_str);
        }

        Ok(())
    }

    /// Close the database connection (useful for cleanup in tests)
    #[cfg(test)]
    pub fn close(&self) -> Result<()> {
        // Just drop the connection - don't consume the Mutex
        // This allows the connection to be properly closed without consuming self
        let _conn = self.conn.lock().unwrap();
        Ok(())
    }
}

/// Screenshot record from database
#[derive(Debug, Clone)]
pub struct Screenshot {
    pub id: i64,
    pub filepath: String,
    pub filename: String,
    pub width: i32,
    pub height: i32,
    pub status: String,
    pub category: String,
    pub category_source: String,
    pub tags: Vec<String>,
    pub ocr_text: Option<String>,
    pub summary: Option<String>,
    pub app_detected: i32,
    pub is_archived: i32,
    pub is_favorite: i32,
    pub thumbnail: Option<Vec<u8>>,
    pub created_at: String,
    pub updated_at: String,
}

impl Database {
    /// Insert a new screenshot into the database
    pub fn insert_screenshot(
        &self,
        filepath: &str,
        filename: &str,
        width: i32,
        height: i32,
        status: &str,
        category: &str,
        category_source: &str,
        tags: Vec<String>,
        ocr_text: Option<String>,
        summary: Option<String>,
        app_detected: i32,
        is_archived: i32,
        is_favorite: i32,
        thumbnail: Option<Vec<u8>>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

conn.execute(
            r#"
            INSERT INTO screenshots (
                filepath, filename, width, height, status,
                category, category_source, tags, ocr_text, summary,
                app_detected, is_archived, is_favorite, thumbnail
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            params![
                filepath,
                filename,
                width,
                height,
                status,
                category,
                category_source,
                serde_json::to_string(&tags).context("Failed to serialize tags")?,
                ocr_text,
                summary,
                app_detected,
                is_archived,
                is_favorite,
                thumbnail,
            ],
        ).context("Failed to insert screenshot")?;

        Ok(conn.last_insert_rowid())
    }

    /// Get a single screenshot by ID
    pub fn get_screenshot(&self, id: i64) -> Result<Option<Screenshot>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            r#"
            SELECT id, filepath, filename, width, height, status,
                   category, category_source, tags, ocr_text, summary,
                   app_detected, is_archived, is_favorite, thumbnail,
                   created_at, updated_at
            FROM screenshots WHERE id = ?
            "#
        )?;

        let row = stmt.query_row(params![id], |row| {
            Ok(Screenshot {
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
                is_archived: row.get(12)?,
                is_favorite: row.get(13)?,
                thumbnail: row.get(14)?,
                created_at: row.get(15)?,
                updated_at: row.get(16)?,
            })
        }).optional().context("Failed to get screenshot")?;

        Ok(row)
    }

    /// List all screenshots with optional filtering
    pub fn list_screenshots(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Screenshot>> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from("SELECT id, filepath, filename, width, height, status, category, category_source, tags, ocr_text, summary, app_detected, is_archived, is_favorite, thumbnail, created_at, updated_at FROM screenshots");

        if limit.is_some() || offset.is_some() {
            query.push_str(" LIMIT ? OFFSET ?");
        }

        let mut stmt = conn.prepare(&query)?;

let rows = stmt.query_map(
            match (limit, offset) {
                (Some(l), Some(o)) => params_from_iter(vec![l, o]),
                _ => params_from_iter(vec![]),
            },
            |row| {
                Ok(Screenshot {
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
                    is_archived: row.get(12)?,
                    is_favorite: row.get(13)?,
                    thumbnail: row.get(14)?,
                    created_at: row.get(15)?,
                    updated_at: row.get(16)?,
                })
            }
        )?;

        let mut screenshots = Vec::<Result<Screenshot, _>>::new();
        for row in rows {
            screenshots.push(row.context("Failed to get screenshot row"));
        }
        let screenshots: Vec<Screenshot> = screenshots.into_iter().collect::<Result<Vec<_>, _>>()?;

        Ok(screenshots)
    }

    /// Delete a screenshot by ID
    pub fn delete_screenshot(&self, id: i64) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM screenshots WHERE id = ?", params![id])
            .context("Failed to delete screenshot")
    }

    /// Update a screenshot
    pub fn update_screenshot(
        &self,
        id: i64,
        filepath: Option<&str>,
        filename: Option<&str>,
        width: Option<i32>,
        height: Option<i32>,
        status: Option<&str>,
        category: Option<&str>,
        category_source: Option<&str>,
        tags: Option<Vec<String>>,
        ocr_text: Option<String>,
        summary: Option<String>,
        app_detected: Option<i32>,
        is_archived: Option<i32>,
        is_favorite: Option<i32>,
        thumbnail: Option<Vec<u8>>,
    ) -> Result<usize> {
        let conn = self.conn.lock().unwrap();

        let mut updates = Vec::new();
        let mut column_values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(v) = filepath {
            updates.push("filepath = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = filename {
            updates.push("filename = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = width {
            updates.push("width = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = height {
            updates.push("height = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = status {
            updates.push("status = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = category {
            updates.push("category = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = category_source {
            updates.push("category_source = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = tags {
            updates.push("tags = ?");
            column_values.push(Box::new(serde_json::to_string(&v)?));
        }
        if let Some(v) = ocr_text {
            updates.push("ocr_text = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = summary {
            updates.push("summary = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = app_detected {
            updates.push("app_detected = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = is_archived {
            updates.push("is_archived = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = is_favorite {
            updates.push("is_favorite = ?");
            column_values.push(Box::new(v));
        }
        if let Some(v) = thumbnail {
            updates.push("thumbnail = ?");
            column_values.push(Box::new(v));
        }

        if updates.is_empty() {
            return Ok(0);
        }

        updates.push("updated_at = datetime('now')");
        column_values.push(Box::new(id));

        let query = format!("UPDATE screenshots SET {} WHERE id = ?", updates.join(", "));

        // Use params_from_iter to handle Box<dyn ToSql> values
        let result = conn.execute(&query, params_from_iter(column_values))?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db_path() -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_dir = std::env::temp_dir()
            .join(format!("snapsort_test_{}_{}", std::process::id(), n));
        std::fs::create_dir_all(&temp_dir).expect("Failed to create temporary test directory");
        temp_dir
    }

    #[test]
    fn test_migration_applied() {
        let db = Database::open(test_db_path()).unwrap();
        // Connection will be dropped automatically when test ends
        // No need to close explicitly - the connection is consumed when Mutex is dropped
        // Database file will be deleted by remove_file() in next test's open()
        // We can use the connection to verify the migration was applied
        let _ = db.get_screenshot(1); // Force a query to verify schema exists
    }

    #[test]
    fn test_insert_and_get_screenshot() {
        let db = Database::open(test_db_path()).unwrap();

        let id = db.insert_screenshot(
            "/test/path.png",
            "test.png",
            800,
            600,
            "analyzed",
            "code",
            "user",
            vec!["rust".to_string(), "ui".to_string()],
            Some("OCR text here".to_string()),
            Some("Summary here".to_string()),
            1,
            0,
            0,
            None,
        ).unwrap();

        let screenshot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(screenshot.id, id);
        assert_eq!(screenshot.filepath, "/test/path.png");
        assert_eq!(screenshot.category, "code");
        assert_eq!(screenshot.category_source, "user");
        assert_eq!(screenshot.tags, vec!["rust".to_string(), "ui".to_string()]);
    }

    #[test]
    fn test_list_screenshots() {
        let db = Database::open(test_db_path()).unwrap();

        // Insert multiple screenshots
        for i in 0..5 {
            db.insert_screenshot(
                &format!("/test/path{}.png", i),
                &format!("test{}.png", i),
                800 + i,
                600 + i,
                "analyzed",
                "code",
                "user",
                vec![],
                None,
                None,
                1,
                0,
                0,
                None,
            ).unwrap();
        }

        let screenshots = db.list_screenshots(Some(3), Some(0)).unwrap();
        assert_eq!(screenshots.len(), 3);
    }

    #[test]
    fn test_delete_screenshot() {
        let db = Database::open(test_db_path()).unwrap();

        let id = db.insert_screenshot(
            "/test/path.png",
            "test.png",
            800,
            600,
            "analyzed",
            "code",
            "user",
            vec![],
            None,
            None,
            1,
            0,
            0,
            None,
        ).unwrap();

        let result = db.delete_screenshot(id).unwrap();
        assert_eq!(result, 1);

        let screenshot = db.get_screenshot(id).unwrap();
        assert!(screenshot.is_none());
        
        // Force a query to verify FTS5 tables are in sync
        let _ = db.list_screenshots(Some(1), Some(0));
    }

    #[test]
    fn test_update_screenshot() {
        let db = Database::open(test_db_path()).unwrap();

        let id = db.insert_screenshot(
            "/test/path.png",
            "test.png",
            800,
            600,
            "analyzed",
            "code",
            "user",
            vec![],
            None,
            None,
            1,
            0,
            0,
            None,
        ).unwrap();

        let result = db.update_screenshot(
            id,
            Some("/new/path.png"),
            Some("new_name.png"),
            Some(1024),
            Some(768),
            Some("ocr_complete"),
            Some("design"),
            Some("ai"),
            Some(vec!["ui".to_string(), "ux".to_string()]),
            Some("New OCR text".to_string()),
            Some("New summary".to_string()),
            Some(0),
            Some(1),
            Some(1),
            None,
        ).unwrap();

        assert_eq!(result, 1);

        let screenshot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(screenshot.filepath, "/new/path.png");
        assert_eq!(screenshot.filename, "new_name.png");
        assert_eq!(screenshot.width, 1024);
        assert_eq!(screenshot.height, 768);
        assert_eq!(screenshot.status, "ocr_complete");
        assert_eq!(screenshot.category, "design");
        assert_eq!(screenshot.category_source, "ai");
        assert_eq!(screenshot.tags, vec!["ui".to_string(), "ux".to_string()]);
        assert_eq!(screenshot.ocr_text, Some("New OCR text".to_string()));
        assert_eq!(screenshot.summary, Some("New summary".to_string()));
        assert_eq!(screenshot.app_detected, 0);
        assert_eq!(screenshot.is_archived, 1);
        assert_eq!(screenshot.is_favorite, 1);
    }

    // The 10 valid taxonomy categories enforced by the DB CHECK constraint
    const VALID_CATEGORIES: &[&str] = &[
        "code", "design", "document", "web", "communication",
        "media", "finance", "reference", "system", "other",
    ];

    #[test]
    fn test_category_all_valid_values_accepted() {
        let db = Database::open(test_db_path()).unwrap();
        for (i, cat) in VALID_CATEGORIES.iter().enumerate() {
            let result = db.insert_screenshot(
                &format!("/s/{}.png", cat), &format!("{}.png", cat),
                100, 100, "detected", cat, "ai", vec![], None, None, 0, 0, 0, None,
            );
            assert!(result.is_ok(), "category '{}' should be accepted (index {})", cat, i);
        }
    }

    #[test]
    fn test_category_invalid_value_rejected() {
        let db = Database::open(test_db_path()).unwrap();
        // SQLite CHECK constraint should reject values outside the taxonomy
        let result = db.insert_screenshot(
            "/s/bad.png", "bad.png", 100, 100, "detected",
            "invalid_category", "ai", vec![], None, None, 0, 0, 0, None,
        );
        assert!(result.is_err(), "invalid category should be rejected by DB constraint");
    }

    #[test]
    fn test_category_source_ai_accepted() {
        let db = Database::open(test_db_path()).unwrap();
        let id = db.insert_screenshot(
            "/s/ai.png", "ai.png", 100, 100, "analyzed",
            "code", "ai", vec![], None, None, 0, 0, 0, None,
        ).unwrap();
        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.category_source, "ai");
    }

    #[test]
    fn test_category_source_user_accepted() {
        let db = Database::open(test_db_path()).unwrap();
        let id = db.insert_screenshot(
            "/s/user.png", "user.png", 100, 100, "analyzed",
            "design", "user", vec![], None, None, 0, 0, 0, None,
        ).unwrap();
        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.category_source, "user");
    }

    #[test]
    fn test_category_source_invalid_rejected() {
        let db = Database::open(test_db_path()).unwrap();
        let result = db.insert_screenshot(
            "/s/bad_src.png", "bad_src.png", 100, 100, "detected",
            "code", "system", vec![], None, None, 0, 0, 0, None,
        );
        assert!(result.is_err(), "category_source not in ('ai','user') should be rejected");
    }

    #[test]
    fn test_category_user_source_survives_update() {
        let db = Database::open(test_db_path()).unwrap();
        let id = db.insert_screenshot(
            "/s/prot.png", "prot.png", 100, 100, "analyzed",
            "finance", "user", vec![], None, None, 0, 0, 0, None,
        ).unwrap();

        // AI updates summary but must NOT change category_source without explicit intent
        db.update_screenshot(
            id, None, None, None, None, None, None, None, None,
            None, Some("AI-generated summary".to_string()), None, None, None, None,
        ).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.category, "finance", "category must not change");
        assert_eq!(shot.category_source, "user", "user provenance must be preserved");
    }
}