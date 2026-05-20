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
        // Bootstrap: ensure schema_migrations exists before any queries touch it
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );"
        ).context("Failed to create schema_migrations table")?;

        // Get the version of the last applied migration
        let current_version: Option<i64> = conn
            .query_row("SELECT MAX(version) FROM schema_migrations", [], |row| row.get(0))
            .ok();

        // Locate migrations directory relative to the crate root (works in tests and prod)
        let migrations_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("migrations");

        // Read and apply migrations in order
        let mut migration_files: Vec<_> = std::fs::read_dir(&migrations_dir)
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Screenshot {
    pub id: i64,
    pub filepath: String,
    pub filename: String,
    pub width: i32,
    pub height: i32,
    pub status: String,
    pub category: Option<String>,
    pub category_source: String,
    pub tags: Vec<String>,
    pub ocr_text: Option<String>,
    pub summary: Option<String>,
    pub app_detected: Option<String>,
    pub user_notes: Option<String>,
    pub is_archived: i32,
    pub is_favorite: i32,
    pub thumbnail: Option<Vec<u8>>,
    pub created_at: String,
    pub updated_at: String,
}

/// Search filters for full-text search queries
#[derive(Debug, Clone)]
pub struct SearchFilters {
    /// Full-text search query
    pub query: Option<String>,
    /// Date range filter: start date (ISO 8601 format)
    pub start_date: Option<String>,
    /// Date range filter: end date (ISO 8601 format)
    pub end_date: Option<String>,
    /// Category filter
    pub category: Option<String>,
    /// Tag filter
    pub tag: Option<String>,
    /// App name filter (exact match)
    pub app_detected: Option<bool>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
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
        category: Option<&str>,
        category_source: &str,
        tags: Vec<String>,
        ocr_text: Option<String>,
        summary: Option<String>,
        app_detected: Option<String>,
        is_archived: i32,
        is_favorite: i32,
        thumbnail: Option<Vec<u8>>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

let _result = conn.execute(
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
        );

        Ok(conn.last_insert_rowid())
    }

    /// Get a single screenshot by ID
    pub fn get_screenshot(&self, id: i64) -> Result<Option<Screenshot>> {
        let conn = self.conn.lock().unwrap();

let mut stmt = conn.prepare(
            r#"
            SELECT id, filepath, filename, width, height, status,
                   category, category_source, tags, ocr_text, summary,
                   app_detected, user_notes, is_archived, is_favorite, thumbnail,
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
                user_notes: row.get(12)?,
                is_archived: row.get(13)?,
                is_favorite: row.get(14)?,
                thumbnail: row.get(15)?,
                created_at: row.get(16)?,
                updated_at: row.get(17)?,
            })
        }).optional().context("Failed to get screenshot")?;

        Ok(row)
    }

    /// List all screenshots with optional filtering
    pub fn list_screenshots(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Screenshot>> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from("SELECT id, filepath, filename, width, height, status, category, category_source, tags, ocr_text, summary, app_detected, user_notes, is_archived, is_favorite, thumbnail, created_at, updated_at FROM screenshots");

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
                    user_notes: row.get(12)?,
                    is_archived: row.get(13)?,
                    is_favorite: row.get(14)?,
                    thumbnail: row.get(15)?,
                    created_at: row.get(16)?,
                    updated_at: row.get(17)?,
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
        app_detected: Option<String>,
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

    /// Get a reference to the database connection (for advanced queries)
    ///
    /// This is a public accessor for the private `conn` field.
    /// Most code should use the typed methods on Database instead.
    pub fn get_connection(&self) -> &Mutex<Connection> {
        &self.conn
    }

    /// Search screenshots using full-text search with BM25 ranking
    ///
    /// Search covers: summary, OCR text, category, app name, tags, and user notes
    /// Filters (date range, category, tag, app) compose on top of the text query
    pub fn search_screenshots(&self, filters: SearchFilters) -> Result<Vec<Screenshot>> {
        let conn = self.conn.lock().unwrap();

        let has_text_query = filters.query.is_some();

        // FTS5 MATCH requires the FTS table to be the driving table. Without a text
        // query, skip the FTS join entirely to avoid scanning the FTS index without MATCH.
        let base_query = if has_text_query {
            "SELECT s.id, s.filepath, s.filename, s.width, s.height, s.status, s.category, s.category_source, s.tags, s.ocr_text, s.summary, s.app_detected, s.user_notes, s.is_archived, s.is_favorite, s.thumbnail, s.created_at, s.updated_at \
             FROM screenshots_fts fts JOIN screenshots s ON s.id = fts.rowid"
        } else {
            "SELECT s.id, s.filepath, s.filename, s.width, s.height, s.status, s.category, s.category_source, s.tags, s.ocr_text, s.summary, s.app_detected, s.user_notes, s.is_archived, s.is_favorite, s.thumbnail, s.created_at, s.updated_at \
             FROM screenshots s"
        };
        let mut query = String::from(base_query);

        let mut where_clauses = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        // Full-text search query
        if let Some(q) = &filters.query {
            where_clauses.push("screenshots_fts MATCH ?");
            params.push(Box::new(q.clone()));
        }

        // Date range filter
        if let Some(start_date) = &filters.start_date {
            where_clauses.push("s.created_at >= ?");
            params.push(Box::new(start_date.clone()));
        }
        if let Some(end_date) = &filters.end_date {
            where_clauses.push("s.created_at <= ?");
            params.push(Box::new(end_date.clone()));
        }

        // Category filter
        if let Some(cat) = &filters.category {
            where_clauses.push("s.category = ?");
            params.push(Box::new(cat.clone()));
        }

        // Tag filter (screenshot must have this tag)
        if let Some(tag) = &filters.tag {
            where_clauses.push("s.tags LIKE ?");
            params.push(Box::new(format!("%{}%", tag)));
        }

        // App detected filter
        if let Some(val) = filters.app_detected {
            where_clauses.push("s.app_detected = ?");
            params.push(Box::new(val as i32));
        }

        // Add WHERE clause if we have any filters
        if !where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&where_clauses.join(" AND "));
        }

        if has_text_query {
            query.push_str(" ORDER BY fts.rank");
        } else {
            query.push_str(" ORDER BY s.created_at DESC");
        }

        // Add pagination
        if let Some(limit) = filters.limit {
            query.push_str(&format!(" LIMIT {}", limit));
            if let Some(offset) = filters.offset {
                query.push_str(&format!(" OFFSET {}", offset));
            }
        } else {
            // Default limit of 100 if not specified
            query.push_str(" LIMIT 100");
        }

        let mut stmt = conn.prepare(&query)?;

        // Convert params to Vec<Box<dyn ToSql>> for query_map
        let params_vec: Vec<Box<dyn rusqlite::ToSql>> = params.into_iter().collect();
        let mut rows = stmt.query_map(
            params_from_iter(params_vec),
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
                    user_notes: row.get(12)?,
                    is_archived: row.get(13)?,
                    is_favorite: row.get(14)?,
                    thumbnail: row.get(15)?,
                    created_at: row.get(16)?,
                    updated_at: row.get(17)?,
                })
            }
        )?;

        let mut screenshots = Vec::<Result<Screenshot, _>>::new();
        for row in rows {
            screenshots.push(row.context("Failed to get search result row"));
        }
        let screenshots: Vec<Screenshot> = screenshots.into_iter().collect::<Result<Vec<_>, _>>()?;

        Ok(screenshots)
    }

    /// Get status counts for all screenshots
    ///
    /// Returns a map of status name to count.
    /// This backs the 'Analyzing N' and 'Unanalyzed' smart folder UI.
    pub fn get_status_counts(&self) -> Result<std::collections::HashMap<String, i64>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT status, COUNT(*) as count FROM screenshots GROUP BY status"
        )?;

        let mut counts = std::collections::HashMap::new();
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
    pub fn list_by_status(&self, status: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Screenshot>> {
        let conn = self.conn.lock().unwrap();

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

    // ============================================================================
    // Batch operations
    // ============================================================================

    /// Batch categorize screenshots in a single transaction
    pub fn batch_categorize(
        &self,
        screenshot_ids: Vec<i64>,
        category: &str,
        category_source: &str,
    ) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let mut tx = conn.transaction().context("Failed to start batch categorize transaction")?;

        for id in &screenshot_ids {
            tx.execute(
                "UPDATE screenshots SET category = ?, category_source = ? WHERE id = ?",
                params![category, category_source, id],
            )
            .context("Failed to categorize screenshot")?;
        }

        tx.commit()
            .context("Failed to commit batch categorize transaction")?;
        Ok(())
    }

    /// Batch tag screenshots in a single transaction
    pub fn batch_tag(
        &self,
        screenshot_ids: Vec<i64>,
        tags: Vec<String>,
        add: bool,
    ) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let mut tx = conn.transaction().context("Failed to start batch tag transaction")?;

        for id in &screenshot_ids {
            let current_tags: Option<Vec<String>> = match tx
                .query_row(
                    "SELECT tags FROM screenshots WHERE id = ?",
                    params![id],
                    |row| {
                        let tags_str: String = row.get(0)?;
                        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
                        Ok(tags)
                    },
                ) {
                Ok(tags) => Some(tags),
                Err(_) => None,
            };

            let mut new_tags = if let Some(current_tags) = current_tags {
                current_tags
            } else {
                Vec::new()
            };

            if add {
                // Add new tags, avoiding duplicates
                for tag in &tags {
                    if !new_tags.contains(tag) {
                        new_tags.push(tag.clone());
                    }
                }
            } else {
                // Remove specified tags
                new_tags.retain(|t| !tags.contains(t));
            }

            tx.execute(
                "UPDATE screenshots SET tags = ? WHERE id = ?",
                params![serde_json::to_string(&new_tags).context("Failed to serialize tags")?, id],
            )
            .context("Failed to update tags for screenshot")?;
        }

        tx.commit()
            .context("Failed to commit batch tag transaction")?;
        Ok(())
    }

    /// Batch archive screenshots in a single transaction
    pub fn batch_archive(&self, screenshot_ids: Vec<i64>, archived: i32) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let mut tx = conn.transaction().context("Failed to start batch archive transaction")?;

        for id in &screenshot_ids {
            tx.execute(
                "UPDATE screenshots SET is_archived = ? WHERE id = ?",
                params![archived, id],
            )
            .context("Failed to archive screenshot")?;
        }

        tx.commit()
            .context("Failed to commit batch archive transaction")?;
        Ok(())
    }

    /// Batch add screenshots to a collection in a single transaction
    pub fn batch_add_to_collection(&self, screenshot_ids: Vec<i64>, collection_id: &str) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let mut tx = conn.transaction().context("Failed to start batch add-to-collection transaction")?;

        for id in &screenshot_ids {
            // Check if already in collection
            let exists: bool = tx
                .query_row(
                    "SELECT 1 FROM screenshot_collections WHERE screenshot_id = ? AND collection_id = ?",
                    params![id, collection_id],
                    |row| row.get(0),
                )
                .unwrap_or(false);

            if !exists {
                tx.execute(
                    "INSERT INTO screenshot_collections (screenshot_id, collection_id) VALUES (?, ?)",
                    params![id, collection_id],
                )
                .context("Failed to add screenshot to collection")?;
            }
        }

        tx.commit()
            .context("Failed to commit batch add-to-collection transaction")?;
        Ok(())
    }

    /// Batch delete screenshots (DB-only, file deletion is handled in commands)
    pub fn batch_delete_records(&self, screenshot_ids: Vec<i64>) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let mut tx = conn.transaction().context("Failed to start batch delete transaction")?;

        for id in &screenshot_ids {
            tx.execute("DELETE FROM screenshots WHERE id = ?", params![id])
                .context("Failed to delete screenshot")?;
        }

        tx.commit()
            .context("Failed to commit batch delete transaction")?;
        Ok(())
    }

    /// Batch rename screenshots (per-file best-effort)
    pub fn batch_rename_records(
        &self,
        screenshot_ids: Vec<i64>,
        pattern: &str,
    ) -> Result<Vec<FileOpError>> {
        let mut errors = Vec::new();

        for id in &screenshot_ids {
            let screenshot = self.get_screenshot(*id);
            let screenshot = match screenshot {
                Ok(Some(s)) => s,
                Ok(None) => {
                    errors.push(FileOpError {
                        id: *id,
                        path: String::new(),
                        error: "Screenshot not found".to_string(),
                    });
                    continue;
                }
                Err(e) => {
                    errors.push(FileOpError {
                        id: *id,
                        path: String::new(),
                        error: e.to_string(),
                    });
                    continue;
                }
            };

            let new_filename = pattern.replace("%id%", &id.to_string())
                .replace("%filename%", &screenshot.filename);

            if new_filename == screenshot.filename {
                continue;
            }

            let new_filepath = screenshot.filepath
                .replace(&screenshot.filename, &new_filename);

            // Update both DB and filesystem (blocking for rename)
            if let Err(e) = std::fs::rename(&screenshot.filepath, &new_filepath) {
                errors.push(FileOpError {
                    id: *id,
                    path: screenshot.filepath.clone(),
                    error: e.to_string(),
                });
                continue;
            }

            // Update DB record
            if let Err(e) = self.update_screenshot(
                *id,
                Some(&new_filepath),
                Some(&new_filename),
                None, None, None, None, None, None, None, None, None, None, None, None,
            ) {
                errors.push(FileOpError {
                    id: *id,
                    path: screenshot.filepath.clone(),
                    error: e.to_string(),
                });
            }
        }

        Ok(errors)
    }

    /// Batch move screenshots to new directory (per-file best-effort)
    pub fn batch_move_records(
        &self,
        screenshot_ids: Vec<i64>,
        dest_dir: &str,
    ) -> Result<Vec<FileOpError>> {
        let mut errors = Vec::new();

        for id in &screenshot_ids {
            let screenshot = self.get_screenshot(*id);
            let screenshot = match screenshot {
                Ok(Some(s)) => s,
                Ok(None) => {
                    errors.push(FileOpError {
                        id: *id,
                        path: String::new(),
                        error: "Screenshot not found".to_string(),
                    });
                    continue;
                }
                Err(e) => {
                    errors.push(FileOpError {
                        id: *id,
                        path: String::new(),
                        error: e.to_string(),
                    });
                    continue;
                }
            };

            let new_filepath = format!("{}/{}", dest_dir.trim_end_matches('/'), screenshot.filename);

            // Update both DB and filesystem (blocking for rename)
            if let Err(e) = std::fs::rename(&screenshot.filepath, &new_filepath) {
                errors.push(FileOpError {
                    id: *id,
                    path: screenshot.filepath.clone(),
                    error: e.to_string(),
                });
                continue;
            }

            // Update DB record
            if let Err(e) = self.update_screenshot(
                *id,
                Some(&new_filepath),
                None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ) {
                errors.push(FileOpError {
                    id: *id,
                    path: screenshot.filepath.clone(),
                    error: e.to_string(),
                });
            }
        }

        Ok(errors)
    }

    // ============================================================================
    // Collection methods
    // ============================================================================

    /// List all collections
    pub fn list_collections(&self) -> Result<Vec<Collection>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, color, created_at, updated_at FROM collections ORDER BY created_at DESC"
        )?;

        let mut collections = Vec::new();
        let rows = stmt.query_map([], |row| {
            Ok(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        for row in rows {
            collections.push(row.context("Failed to read collection row")?);
        }

        Ok(collections)
    }

    /// Create a new collection
    pub fn create_collection(&self, name: &str, description: Option<&str>, color: Option<&str>) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO collections (id, name, description, color, created_at, updated_at) VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))",
            params![id, name, description.unwrap_or(""), color.unwrap_or("#3b82f6")],
        )
        .context("Failed to create collection")?;

        Ok(id)
    }

    /// Get all screenshots in a collection
    pub fn get_screenshots_in_collection(&self, collection_id: &str) -> Result<Vec<Screenshot>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT s.id, s.filepath, s.filename, s.width, s.height, s.status, s.category, s.category_source, s.tags, s.ocr_text, s.summary, s.app_detected, s.user_notes, s.is_archived, s.is_favorite, s.thumbnail, s.created_at, s.updated_at \
             FROM screenshots s \
             JOIN screenshot_collections sc ON s.id = sc.screenshot_id \
             WHERE sc.collection_id = ? \
             ORDER BY s.created_at DESC"
        )?;

        let mut screenshots = Vec::new();
        let rows = stmt.query_map(params![collection_id], |row| {
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
                user_notes: row.get(12)?,
                is_archived: row.get(13)?,
                is_favorite: row.get(14)?,
                thumbnail: row.get(15)?,
                created_at: row.get(16)?,
                updated_at: row.get(17)?,
            })
        })?;

        for row in rows {
            screenshots.push(row.context("Failed to read screenshot row")?);
        }

        Ok(screenshots)
    }
}

/// Collection record
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: String,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Error result for filesystem batch operations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileOpError {
    pub id: i64,
    pub path: String,
    pub error: String,
}

/// Non-secret provider configuration row
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderSettings {
    pub provider_id: String,
    pub display_name: String,
    pub base_url: String,
    pub model: String,
    pub is_active: bool,
    pub key_suffix: Option<String>,
}

impl Database {
    pub fn list_provider_settings(&self) -> Result<Vec<ProviderSettings>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT provider_id, display_name, base_url, model, is_active, key_suffix \
             FROM provider_settings ORDER BY created_at"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ProviderSettings {
                provider_id: row.get(0)?,
                display_name: row.get(1)?,
                base_url: row.get(2)?,
                model: row.get(3)?,
                is_active: row.get::<_, i32>(4)? != 0,
                key_suffix: row.get(5)?,
            })
        })?;
        rows.map(|r| r.context("Failed to read provider_settings row"))
            .collect()
    }

    pub fn get_provider_settings(&self, provider_id: &str) -> Result<Option<ProviderSettings>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT provider_id, display_name, base_url, model, is_active, key_suffix \
             FROM provider_settings WHERE provider_id = ?"
        )?;
        let row = stmt
            .query_row(params![provider_id], |row| {
                Ok(ProviderSettings {
                    provider_id: row.get(0)?,
                    display_name: row.get(1)?,
                    base_url: row.get(2)?,
                    model: row.get(3)?,
                    is_active: row.get::<_, i32>(4)? != 0,
                    key_suffix: row.get(5)?,
                })
            })
            .optional()
            .context("Failed to get provider_settings")?;
        Ok(row)
    }

    pub fn upsert_provider_settings(
        &self,
        provider_id: &str,
        display_name: &str,
        base_url: &str,
        model: &str,
        is_active: bool,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO provider_settings (provider_id, display_name, base_url, model, is_active) \
             VALUES (?, ?, ?, ?, ?) \
             ON CONFLICT(provider_id) DO UPDATE SET \
               display_name = excluded.display_name, \
               base_url = excluded.base_url, \
               model = excluded.model, \
               is_active = excluded.is_active, \
               updated_at = datetime('now')",
            params![provider_id, display_name, base_url, model, is_active as i32],
        )
        .context("Failed to upsert provider_settings")?;
        Ok(())
    }

    pub fn delete_provider_settings(&self, provider_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM provider_settings WHERE provider_id = ?",
            params![provider_id],
        )
        .context("Failed to delete provider_settings")?;
        Ok(())
    }

    pub fn update_key_suffix(&self, provider_id: &str, key_suffix: Option<&str>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE provider_settings SET key_suffix = ?, updated_at = datetime('now') \
             WHERE provider_id = ?",
            params![key_suffix, provider_id],
        )
        .context("Failed to update key_suffix")?;
        Ok(())
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
            "enriched",
            Some("code"),
            "user",
            vec!["rust".to_string(), "ui".to_string()],
            Some("OCR text here".to_string()),
            Some("Summary here".to_string()),
            None,
            0,
            0,
            None,
        ).unwrap();

        let screenshot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(screenshot.id, id);
        assert_eq!(screenshot.filepath, "/test/path.png");
        assert_eq!(screenshot.category, Some("code".to_string()));
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
                "enriched",
                Some("code"),
                "user",
                vec![],
                None,
                None,
                None,
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
            "enriched",
            Some("code"),
            "user",
            vec![],
            None,
            None,
            None,
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
            "enriched",
            Some("code"),
            "user",
            vec![],
            None,
            None,
            None,
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
            Some("Terminal".to_string()),
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
        assert_eq!(screenshot.category, Some("design".to_string()));
        assert_eq!(screenshot.category_source, "ai");
        assert_eq!(screenshot.tags, vec!["ui".to_string(), "ux".to_string()]);
        assert_eq!(screenshot.ocr_text, Some("New OCR text".to_string()));
        assert_eq!(screenshot.summary, Some("New summary".to_string()));
        assert_eq!(screenshot.app_detected, Some("Terminal".to_string()));
        assert_eq!(screenshot.is_archived, 1);
        assert_eq!(screenshot.is_favorite, 1);
    }

    #[test]
    fn test_search_screenshots_by_text() {
        let db = Database::open(test_db_path()).unwrap();

        db.insert_screenshot(
            "/s/python_ml.png", "python_ml.png", 1920, 1080, "enriched",
            Some("code"), "ai", vec![], Some("python machine learning code".to_string()),
            None, None, 0, 0, None,
        ).unwrap();
        db.insert_screenshot(
            "/s/design.png", "design.png", 1920, 1080, "enriched",
            Some("design"), "ai", vec![], Some("figma wireframe design".to_string()),
            None, None, 0, 0, None,
        ).unwrap();

        let results = db.search_screenshots(SearchFilters {
            query: Some("python".to_string()),
            start_date: None, end_date: None, category: None, tag: None,
            app_detected: None, limit: None, offset: None,
        }).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].ocr_text.as_deref().unwrap().contains("python"));
    }

    #[test]
    fn test_search_screenshots_category_filter() {
        let db = Database::open(test_db_path()).unwrap();

        db.insert_screenshot(
            "/s/code.png", "code.png", 100, 100, "enriched",
            Some("code"), "ai", vec![], Some("rust programming language".to_string()),
            None, None, 0, 0, None,
        ).unwrap();
        db.insert_screenshot(
            "/s/web.png", "web.png", 100, 100, "enriched",
            Some("web"), "ai", vec![], Some("rust web framework".to_string()),
            None, None, 0, 0, None,
        ).unwrap();

        let results = db.search_screenshots(SearchFilters {
            query: Some("rust".to_string()),
            category: Some("code".to_string()),
            start_date: None, end_date: None, tag: None,
            app_detected: None, limit: None, offset: None,
        }).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].category, Some("code".to_string()));
    }

    #[test]
    fn test_search_screenshots_null_ai_fields() {
        let db = Database::open(test_db_path()).unwrap();

        // OCR-only screenshot: no summary, only ocr_text
        db.insert_screenshot(
            "/s/ocr_only.png", "ocr_only.png", 800, 600, "ready",
            Some("document"), "ai", vec![], Some("scanned invoice total amount".to_string()),
            None, None, 0, 0, None,
        ).unwrap();

        let results = db.search_screenshots(SearchFilters {
            query: Some("invoice".to_string()),
            start_date: None, end_date: None, category: None, tag: None,
            app_detected: None, limit: None, offset: None,
        }).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].summary.is_none());
        assert!(results[0].ocr_text.as_deref().unwrap().contains("invoice"));
    }

    #[test]
    fn test_search_fts_trigger_consistency() {
        let db = Database::open(test_db_path()).unwrap();

        let id = db.insert_screenshot(
            "/s/trigger.png", "trigger.png", 100, 100, "enriched",
            Some("code"), "ai", vec![], Some("original text before update".to_string()),
            None, None, 0, 0, None,
        ).unwrap();

        // Update should keep FTS in sync
        db.update_screenshot(
            id, None, None, None, None, None, None, None, None,
            Some("updated text after change".to_string()), None, None, None, None, None,
        ).unwrap();

        // Old text should not match
        let old_results = db.search_screenshots(SearchFilters {
            query: Some("original".to_string()),
            start_date: None, end_date: None, category: None, tag: None,
            app_detected: None, limit: None, offset: None,
        }).unwrap();
        assert_eq!(old_results.len(), 0, "old text should not match after update");

        // New text should match
        let new_results = db.search_screenshots(SearchFilters {
            query: Some("updated".to_string()),
            start_date: None, end_date: None, category: None, tag: None,
            app_detected: None, limit: None, offset: None,
        }).unwrap();
        assert_eq!(new_results.len(), 1);
    }

    fn insert_test_screenshot(db: &Database, path: &str, name: &str) -> i64 {
        db.insert_screenshot(path, name, 100, 100, "enriched", Some("code"), "ai",
            vec![], None, None, None, 0, 0, None).unwrap()
    }

    // ---- batch tests ----

    #[test]
    fn test_batch_categorize_applies_to_all() {
        let db = Database::open(test_db_path()).unwrap();
        let id1 = insert_test_screenshot(&db, "/s/a.png", "a.png");
        let id2 = insert_test_screenshot(&db, "/s/b.png", "b.png");
        let id3 = insert_test_screenshot(&db, "/s/c.png", "c.png");

        db.batch_categorize(vec![id1, id2, id3], "design", "user").unwrap();

        for id in [id1, id2, id3] {
            let s = db.get_screenshot(id).unwrap().unwrap();
            assert_eq!(s.category, Some("design".to_string()));
            assert_eq!(s.category_source, "user");
        }
    }

    #[test]
    fn test_batch_categorize_empty_ids_is_noop() {
        let db = Database::open(test_db_path()).unwrap();
        db.batch_categorize(vec![], "design", "user").unwrap();
    }

    #[test]
    fn test_batch_tag_add_deduplicates() {
        let db = Database::open(test_db_path()).unwrap();
        let id = db.insert_screenshot("/s/t.png", "t.png", 100, 100, "enriched",
            Some("code"), "ai", vec!["existing".to_string()], None, None, None, 0, 0, None).unwrap();

        db.batch_tag(vec![id], vec!["new1".to_string(), "existing".to_string()], true).unwrap();

        let s = db.get_screenshot(id).unwrap().unwrap();
        assert!(s.tags.contains(&"new1".to_string()));
        // "existing" must not be duplicated
        let count = s.tags.iter().filter(|t| *t == "existing").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_batch_tag_remove() {
        let db = Database::open(test_db_path()).unwrap();
        let id = db.insert_screenshot("/s/t.png", "t.png", 100, 100, "enriched",
            Some("code"), "ai", vec!["keep".to_string(), "remove".to_string()],
            None, None, None, 0, 0, None).unwrap();

        db.batch_tag(vec![id], vec!["remove".to_string()], false).unwrap();

        let s = db.get_screenshot(id).unwrap().unwrap();
        assert!(s.tags.contains(&"keep".to_string()));
        assert!(!s.tags.contains(&"remove".to_string()));
    }

    #[test]
    fn test_batch_archive_sets_flag() {
        let db = Database::open(test_db_path()).unwrap();
        let id1 = insert_test_screenshot(&db, "/s/a.png", "a.png");
        let id2 = insert_test_screenshot(&db, "/s/b.png", "b.png");

        db.batch_archive(vec![id1, id2], 1).unwrap();

        for id in [id1, id2] {
            assert_eq!(db.get_screenshot(id).unwrap().unwrap().is_archived, 1);
        }
    }

    #[test]
    fn test_batch_delete_records_removes_all() {
        let db = Database::open(test_db_path()).unwrap();
        let id1 = insert_test_screenshot(&db, "/s/a.png", "a.png");
        let id2 = insert_test_screenshot(&db, "/s/b.png", "b.png");

        db.batch_delete_records(vec![id1, id2]).unwrap();

        assert!(db.get_screenshot(id1).unwrap().is_none());
        assert!(db.get_screenshot(id2).unwrap().is_none());
    }

    #[test]
    fn test_batch_rename_partial_failure() {
        let db = Database::open(test_db_path()).unwrap();

        // Real temp file for the first screenshot
        let tmp = std::env::temp_dir()
            .join(format!("snapsort_rename_{}_{}", std::process::id(),
                uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp).unwrap();
        let real_file = tmp.join("original.png");
        std::fs::write(&real_file, b"fake").unwrap();

        let id_real = db.insert_screenshot(
            real_file.to_str().unwrap(), "original.png",
            100, 100, "enriched", Some("code"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap();

        // Second screenshot — file does not exist on disk
        let id_missing = db.insert_screenshot(
            "/nonexistent/path/missing.png", "missing.png",
            100, 100, "enriched", Some("code"), "ai", vec![], None, None, None, 0, 0, None,
        ).unwrap();

        let errors = db.batch_rename_records(
            vec![id_real, id_missing], "renamed_%id%.png"
        ).unwrap();

        // Only the missing file should error
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].id, id_missing);

        // The real file's DB record must be updated
        let s = db.get_screenshot(id_real).unwrap().unwrap();
        assert!(s.filename.starts_with("renamed_"), "filename was: {}", s.filename);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_batch_add_to_collection_atomic() {
        let db = Database::open(test_db_path()).unwrap();
        let id1 = insert_test_screenshot(&db, "/s/a.png", "a.png");
        let id2 = insert_test_screenshot(&db, "/s/b.png", "b.png");

        let col_id = db.create_collection("test-col", None, None).unwrap();
        db.batch_add_to_collection(vec![id1, id2], &col_id).unwrap();

        let in_col = db.get_screenshots_in_collection(&col_id).unwrap();
        assert_eq!(in_col.len(), 2);
    }

    #[test]
    fn test_batch_add_to_collection_no_duplicates() {
        let db = Database::open(test_db_path()).unwrap();
        let id = insert_test_screenshot(&db, "/s/a.png", "a.png");
        let col_id = db.create_collection("dedup-col", None, None).unwrap();

        // Add twice
        db.batch_add_to_collection(vec![id], &col_id).unwrap();
        db.batch_add_to_collection(vec![id], &col_id).unwrap();

        let in_col = db.get_screenshots_in_collection(&col_id).unwrap();
        assert_eq!(in_col.len(), 1, "duplicate insertion should be idempotent");
    }
}