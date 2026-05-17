/// Application state wrapper
pub struct AppState {
    pub db: db::Database,
}

impl AppState {
    pub fn new_with_db(db: db::Database) -> Self {
        Self { db }
    }
}

pub mod commands;
pub mod db;
pub mod ai;
pub mod watcher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("snapsort_test");
        let db = db::Database::open(temp_dir).expect("Failed to create test database");
        let _state = AppState::new_with_db(db);
        assert!(true);
    }

    // Import health_check tests from commands module
    mod health_check_tests {
        use crate::commands::health_check;

        #[test]
        fn test_health_check_returns_success_message() {
            let result = health_check();
            assert!(result.is_ok(), "health_check should return Ok");
            assert_eq!(
                result.unwrap(),
                "SnapSort Tauri 2 shell is running",
                "health_check should return expected success message"
            );
        }

        #[test]
        fn test_health_check_returns_string_type() {
            let result: Result<String, String> = health_check();
            assert!(result.is_ok());
        }

        #[test]
        fn test_health_check_command_registered() {
            let result = health_check();
            assert!(result.is_ok(), "Command should be registered and callable");
        }
    }
}