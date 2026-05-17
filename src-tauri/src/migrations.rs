use crate::db::Database;

/// Run database migrations on startup
pub fn run(db: &Database) -> anyhow::Result<()> {
    // Read all SQL files from migrations directory
    let migrations_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("migrations");

    // Get all migration files and sort them by name
    let mut migration_files = std::fs::read_dir(&migrations_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("sql"))
        .collect::<Vec<_>>();

    migration_files.sort_by_key(|entry| {
        entry.file_name()
            .to_str()
            .unwrap_or("")
            .to_lowercase()
    });

    // Check which migrations have already been run
    let mut stmt = db.conn()
        .prepare("SELECT version FROM schema_migrations ORDER BY version")?;
    let already_run: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    // Apply each migration
    for entry in migration_files {
        let path = entry.path();
        let filename = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if !already_run.contains(&filename.to_string()) {
            let sql = std::fs::read_to_string(&path)?;
            log::info!("Applying migration: {}", filename);

            let tx = db.transaction();
            tx.execute_batch(&sql)?;
            tx.query_row(
                "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, datetime('now'))",
                &[filename],
                |_| Ok(())
            )?;
            tx.commit()?;
        }
    }

    Ok(())
}