use anyhow::{Context, Result};
use notify::Watcher;
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

/// Known default screenshot directories
const DEFAULT_SCREENSHOT_DIRS: &[&str] = &[
    "~/Desktop",
    "~/Pictures/Screenshots",
    "~/Downloads",
    "%USERPROFILE%/Desktop",
    "%USERPROFILE%/Pictures/Screenshots",
    "%USERPROFILE%/Downloads",
];

/// Known screenshot filename patterns (case-insensitive)
const SCREENSHOT_PATTERNS: &[&str] = &[
    "Screenshot",
    "Capture",
    "Screen Shot",
    "screen",
    "capture",
    "printscreen",
];

/// Known screenshot process names
const SCREENSHOT_PROCESSES: &[&str] = &[
    "com.apple.screencaptureui",
    "SnippingTool.exe",
    "ScreenClippingHost.exe",
];

/// Minimum screenshot size in bytes (filters icons/avatars)
const MIN_SCREENSHOT_SIZE: u64 = 1024; // 1KB minimum

/// Heuristic-based screenshot detection
fn detect_screenshot(path: &PathBuf) -> bool {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase());

    // Rule 1: Extension check
    if !matches!(extension.as_deref(), Some("png") | Some("jpg") | Some("jpeg") | Some("webp")) {
        return false;
    }

    // Rule 2: Known default screenshot directory
    let expanded_path = expand_path(path);
    if DEFAULT_SCREENSHOT_DIRS.iter().any(|dir| {
        expanded_path.starts_with(dir)
            || expanded_path.to_string_lossy().contains(dir)
    }) {
        return true;
    }

    // Rule 3: Filename pattern
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if SCREENSHOT_PATTERNS
        .iter()
        .any(|pattern| filename.to_lowercase().contains(pattern))
    {
        return true;
    }

    // Rule 4: Known screenshot process (checked via file metadata on creation)
    // Note: This is a simplified check - actual process detection would require
    // additional metadata tracking. For now, we rely on heuristics 2-5.
    // Future enhancement: track process ID from file creation events.

    // Rule 5: Minimum file size (filters icons/avatars)
    if let Ok(metadata) = fs::metadata(path) {
        let size = metadata.len();
        if size >= MIN_SCREENSHOT_SIZE {
            return true;
        }
    }

    false
}

/// Expand environment variables in paths (macOS %USERPROFILE%, Windows %USERPROFILE%)
fn expand_path(path: &PathBuf) -> PathBuf {
    let path_str = path.to_string_lossy().to_string();

    // Replace %USERPROFILE% with home directory
    let expanded = if path_str.contains("%USERPROFILE%") {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        path_str.replace("%USERPROFILE%", &home.to_string_lossy())
    } else if path_str.contains("~/") {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        path_str.replace("~/", &home.to_string_lossy())
    } else {
        path_str
    };

    PathBuf::from(expanded)
}

/// State shared between Tauri commands and the background watcher
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct WatcherState {
    /// Currently watched folders (canonicalized paths)
    pub watched_folders: Vec<String>,
    /// Maximum number of folders to watch
    pub max_folders: usize,
}

impl WatcherState {
    pub fn new() -> Self {
        Self {
            watched_folders: Vec::new(),
            max_folders: 20,
        }
    }

    /// Add a watch folder
    ///
    /// # Errors
    /// - Path doesn't exist or is not a directory
    /// - Path is already being watched
    /// - Maximum 20 folders limit exceeded
    pub async fn add_watch_folder(&mut self, path: PathBuf) -> Result<()> {
        let _path_str = path.to_string_lossy().to_string();

        // Canonicalize path for robust comparison
        let canonical_path = fs::canonicalize(&path)
            .with_context(|| format!("Failed to canonicalize path: {}", path.display()))?;

        let can_path_str = canonical_path.to_string_lossy().to_string();

        // Check if already watching
        if self.watched_folders.contains(&can_path_str) {
            return Ok(());
        }

        // Check max folders limit
        if self.watched_folders.len() >= self.max_folders {
            return Err(anyhow::anyhow!(
                "Maximum of {} watch folders allowed",
                self.max_folders
            ));
        }

        // Verify path exists and is a directory
        if !canonical_path.exists() {
            return Err(anyhow::anyhow!(
                "Watch folder does not exist: {}",
                canonical_path.display()
            ));
        }

        if !canonical_path.is_dir() {
            return Err(anyhow::anyhow!(
                "Watch path is not a directory: {}",
                canonical_path.display()
            ));
        }

        self.watched_folders.push(can_path_str);
        Ok(())
    }

    /// Remove a watch folder
    ///
    /// # Errors
    /// - Path was not in the watched set
    pub async fn remove_watch_folder(&mut self, path: PathBuf) -> Result<()> {
        let canonical_path = fs::canonicalize(&path)
            .with_context(|| format!("Failed to canonicalize path: {}", path.display()))?;

        let can_path_str = canonical_path.to_string_lossy().to_string();

        if !self.watched_folders.contains(&can_path_str) {
            return Err(anyhow::anyhow!(
                "Path is not being watched: {}",
                canonical_path.display()
            ));
        }

        self.watched_folders.retain(|f| f != &can_path_str);
        Ok(())
    }

    /// List all currently watched folders (as PathBuf)
    pub fn list_watch_folders(&self) -> Vec<PathBuf> {
        self.watched_folders.iter().map(|s| PathBuf::from(s)).collect()
    }

    /// Get the number of currently watched folders
    pub fn folder_count(&self) -> usize {
        self.watched_folders.len()
    }

    /// Get max folders limit
    pub fn max_folders() -> usize {
        20
    }
}

/// Background file watcher that detects new screenshot files
pub struct FileWatcher {
    watch_state: Arc<Mutex<WatcherState>>,
    watcher: Option<notify_debouncer_full::Debouncer<
        notify::RecommendedWatcher,
        notify_debouncer_full::FileIdMap,
    >>,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(watch_state: Arc<Mutex<WatcherState>>) -> Self {
        Self {
            watch_state,
            watcher: None,
        }
    }

    /// Get a clone of the Arc for state sharing
    pub fn get(&self) -> Arc<Self> {
        Arc::new(Self {
            watch_state: self.watch_state.clone(),
            watcher: None,
        })
    }

    /// Initialize the watcher with current watched folders
    pub async fn initialize(&mut self) -> Result<()> {
        let folders = self.watch_state.lock().unwrap().list_watch_folders();
        let watch_state_clone = self.watch_state.clone();

        // Create the debouncer with 1-second debounce window
        let mut debouncer = notify_debouncer_full::new_debouncer(
            std::time::Duration::from_secs(1),
            None,
            move |res: std::result::Result<Vec<notify_debouncer_full::DebouncedEvent>, Vec<notify::Error>>| {
                match res {
                    Ok(events) => {
                        for event in events {
                            // Process file creation events
                            if matches!(event.kind, notify::event::EventKind::Create(notify::event::CreateKind::File)) {
                                for path in &event.paths {
                                    // Check if it's a screenshot using heuristic
                                    if detect_screenshot(&path) {
                                        // Defer full ingestion to a separate async task
                                        let path_clone = path.clone();
                                        let watch_state = watch_state_clone.clone();

                                        tokio::spawn(async move {
                                            // Wait for file to be fully written
                                            if verify_file_complete(&path_clone).await {
                                                // File is complete - trigger ingestion
                                                tracing::info!(
                                                    "Screenshot detected: {}",
                                                    path_clone.display()
                                                );

                                                // In a real implementation, this would trigger the
                                                // ingestion pipeline (thumbnail generation, OCR, etc.)
                                                // For now, we just log it.
                                                // TODO: Call ingestion pipeline from here
                                            } else {
                                                tracing::debug!(
                                                    "File not ready (partial write): {}",
                                                    path_clone.display()
                                                );
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Debouncer error: {:?}", e);
                    }
                }
            }
        )
        .context("Failed to create debouncer")?;

        // Watch each folder
        for folder in &folders {
            let path = PathBuf::from(folder);
            debouncer
                .watcher()
                .watch(&path, notify::RecursiveMode::Recursive)
                .with_context(|| {
                    format!("Failed to watch folder: {}", path.display())
                })?;
        }

        self.watcher = Some(debouncer);

        tracing::info!(
            "File watcher initialized with {} folders",
            folders.len()
        );

        Ok(())
    }

    /// Reinitialize the watcher when watched folders change
    pub async fn reinitialize(&mut self) -> Result<()> {
        tracing::info!("Reinitializing file watcher with updated folder list");
        self.initialize().await
    }
}

/// Verify that a file has been fully written by checking size changes
///
/// This implements the sync→async bridge pattern for file verification.
/// It polls the file size until it stabilizes (no change for 2 consecutive polls),
/// indicating the file is fully written.
async fn verify_file_complete(path: &PathBuf) -> bool {
    let mut last_size: Option<u64> = None;
    let mut stable_count = 0;

    for _ in 0..10 {
        // Wait a bit between polls to allow the writer to finish
        tokio::time::sleep(Duration::from_millis(100)).await;

        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => {
                // File doesn't exist yet (was deleted or not created)
                tracing::debug!("File does not exist for verification: {}", path.display());
                return false;
            }
        };

        let size = metadata.len();

        // Initialize last_size on first successful read
        if let None = last_size {
            last_size = Some(size);
            tracing::debug!("File initial size: {} bytes", size);
            continue;
        }

        // Check if size has changed since last poll
        if let Some(prev_size) = last_size {
            if size != prev_size {
                // Size changed - reset stable count
                last_size = Some(size);
                stable_count = 0;
                tracing::debug!("File size changed: {} -> {} bytes", prev_size, size);
            } else {
                // Size hasn't changed - increment stable count
                stable_count += 1;
                tracing::debug!("File size stable (count: {}/{})", stable_count, 2);

                // Need 2 consecutive polls with no size change
                if stable_count >= 2 {
                    tracing::debug!(
                        "File fully written: {} (final size: {} bytes)",
                        path.display(),
                        size
                    );
                    return true;
                }
            }
        }
    }

    // Timeout - file might still be writing
    tracing::debug!(
        "File verification timeout: {} (last known size: {:?} bytes)",
        path.display(),
        last_size
    );
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heuristic_extension() {
        // Image extensions with screenshot keywords are detected
        for ext in &["png", "jpg", "jpeg", "webp"] {
            let name = format!("Screenshot.{}", ext);
            assert!(detect_screenshot(&PathBuf::from(&name)), "{} should be detected", name);
        }

        // Non-image extensions are always rejected, even with screenshot keywords
        for ext in &["txt", "pdf", "docx"] {
            let name = format!("Screenshot.{}", ext);
            assert!(!detect_screenshot(&PathBuf::from(&name)), "{} should be rejected", name);
        }
    }

    #[test]
    fn test_heuristic_pattern() {
        let test_cases = vec![
            ("Screenshot 2026-04-12.png", true),
            ("Capture_2026-04-12.jpg", true),
            ("Screen Shot 1.png", true),
            ("test_screen.png", true),
            ("test_capture.png", true),
            ("normal_file.png", false),
        ];

        for (filename, expected) in test_cases {
            let path = PathBuf::from(filename);
            let result = detect_screenshot(&path);
            assert_eq!(
                result, expected,
                "Heuristic should detect {} as {}",
                filename, expected
            );
        }
    }

    #[test]
    fn test_min_size_filter() {
        // Create a real temp file >= 1KB with no screenshot keywords to isolate size rule
        let tmp_path = std::env::temp_dir().join("snapsort_size_test_only.png");
        std::fs::write(&tmp_path, vec![0u8; 2048]).unwrap();
        let result = detect_screenshot(&tmp_path);
        let _ = std::fs::remove_file(&tmp_path);
        assert!(result, "Image file >= 1KB should be detected by size rule");
    }

    #[test]
    fn test_detect_screenshot_rejects_non_image() {
        assert!(!detect_screenshot(&PathBuf::from("document.pdf")));
        assert!(!detect_screenshot(&PathBuf::from("Screenshot.txt")));
    }

    #[test]
    fn test_detect_screenshot_keyword_in_name() {
        assert!(detect_screenshot(&PathBuf::from("Screenshot 2026-05-01.png")));
        assert!(detect_screenshot(&PathBuf::from("Capture_today.jpg")));
        assert!(!detect_screenshot(&PathBuf::from("photo.png")));
    }
}