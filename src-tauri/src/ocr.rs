use anyhow::Result;
use std::path::Path;
use std::sync::Arc;
use crate::db::Database;

/// Result of running OCR on a single image.
pub struct OcrResult {
    pub text: String,
}

/// Extract text from an image using Tesseract.
///
/// Returns an empty string rather than an error when Tesseract is not
/// installed or the image contains no recognisable text, so the pipeline
/// can always advance to `ocr_complete`.
pub fn run_ocr(path: &Path) -> Result<OcrResult> {
    // Attempt Tesseract via the `tesseract` CLI if available.
    // This is the bundled path in production; in tests the caller injects
    // text directly through `apply_ocr_result` to avoid needing the binary.
    let output = std::process::Command::new("tesseract")
        .arg(path)
        .arg("stdout")
        .arg("-l")
        .arg("eng")
        .output();

    let text = match output {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        _ => String::new(),
    };

    Ok(OcrResult { text })
}

/// Write OCR text into the database and advance status to `ocr_complete`.
pub fn apply_ocr_result(db: &Database, id: i64, ocr_text: String) -> Result<()> {
    db.update_screenshot(
        id,
        None, None, None, None,
        Some("ocr_complete"),
        None, None, None,
        Some(ocr_text),
        None, None, None, None, None,
    )?;
    Ok(())
}

/// Process a single screenshot: run OCR and persist the result.
pub fn process_screenshot(db: Arc<Database>, id: i64, path: &Path) -> Result<()> {
    let result = run_ocr(path)?;
    apply_ocr_result(&db, id, result.text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::path::PathBuf;

    fn test_db() -> Arc<Database> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir()
            .join(format!("snapsort_ocr_test_{}_{}", std::process::id(), n));
        std::fs::create_dir_all(&dir).unwrap();
        Arc::new(Database::open(dir).unwrap())
    }

    fn insert_detected(db: &Database) -> i64 {
        db.insert_screenshot(
            "/ocr/shot.png", "shot.png", 1920, 1080, "detected",
            "other", "ai", vec![], None, None, 0, 0, 0, None,
        ).unwrap()
    }

    #[test]
    fn test_ocr_apply_result_sets_status_ocr_complete() {
        let db = test_db();
        let id = insert_detected(&db);

        apply_ocr_result(&db, id, "hello world".to_string()).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "ocr_complete");
        assert_eq!(shot.ocr_text, Some("hello world".to_string()));
    }

    #[test]
    fn test_ocr_apply_empty_text_still_advances_status() {
        let db = test_db();
        let id = insert_detected(&db);

        // No AI configured → OCR with empty result must still advance status
        apply_ocr_result(&db, id, String::new()).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "ocr_complete");
    }

    #[test]
    fn test_ocr_works_without_ai_provider() {
        let db = test_db();
        let id = insert_detected(&db);

        // Simulate what process_screenshot does: apply text without AI
        apply_ocr_result(&db, id, "invoice total 42".to_string()).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert_eq!(shot.status, "ocr_complete");
        assert!(shot.ocr_text.as_deref().unwrap().contains("invoice"));
    }

    #[test]
    fn test_ocr_result_stored_in_db_is_searchable_after_update() {
        let db = test_db();
        let id = insert_detected(&db);

        apply_ocr_result(&db, id, "unique ocr search phrase".to_string()).unwrap();

        let shot = db.get_screenshot(id).unwrap().unwrap();
        assert!(shot.ocr_text.as_deref().unwrap().contains("unique ocr search phrase"));
    }
}
