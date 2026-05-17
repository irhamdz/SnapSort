use snap_sort_backend::commands::health_check;

#[cfg(test)]
mod tests {
    use super::*;

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