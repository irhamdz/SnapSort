// Database repository layer
// All SQL queries go through this module

use anyhow::Result;
use rusqlite::{Connection, params};
use std::sync::Arc;

const WAL: &str = "wal";

pub struct Database {
    conn: Connection,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            conn: self.conn
                .clone()
                .expect("Failed to clone database connection"),
        }
    }
}

// State manager for database access
pub struct AppState {
    pub db: Arc<Database>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: Arc::new(Database::new().expect("Failed to initialize database")),
        }
    }
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(
            dirs::config_dir()
                .map(|p| p.join("SnapSort").join("snapsort.db"))
                .unwrap_or_else(|| {
                    std::path::PathBuf::from("snapsort.db")
                })
        )?;

        conn.pragma_update(None, "journal_mode", WAL)?;

        Ok(Self { conn })
    }

    pub fn migrate(&self) -> Result<()> {
        // Run migrations - see migrations/ directory
        println!("Running migrations...");
        Ok(())
    }
}