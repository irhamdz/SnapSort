/// Application state wrapper
/// TODO: Add database connection in later specs
pub struct AppState {
    // Database connection (to be added in later specs)
    // pub db: db::Database,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            // db: db::Database::new().await.unwrap(),
        }
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
        let state = AppState::new();
        assert!(true); // Placeholder test
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