// Privacy guard: static analysis that enforces the "nothing leaves the machine"
// guarantee at the source level. Fails CI if a disallowed pattern is added.

use std::fs;
use std::path::{Path, PathBuf};

fn collect_rs_sources() -> Vec<(String, String)> {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut results = Vec::new();
    collect_rs_recursive(&src_dir, &mut results);
    results
}

fn collect_rs_recursive(dir: &Path, out: &mut Vec<(String, String)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_rs_recursive(&path, out);
            } else if path.extension().map_or(false, |ext| ext == "rs")
                && !path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map_or(false, |n| n.ends_with(".bak") || n.ends_with(".bak2"))
            {
                if let Ok(content) = fs::read_to_string(&path) {
                    out.push((path.to_string_lossy().into_owned(), content));
                }
            }
        }
    }
}

fn canonical_path(path: &str) -> String {
    PathBuf::from(path)
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

/// reqwest (HTTP client) may ONLY appear in AI-provider modules.
/// Any other module making network calls is a privacy violation.
const ALLOWED_REQWEST_FILES: &[&str] = &["src/ai/mod.rs", "src/commands/ai.rs"];

#[test]
fn privacy_reqwest_only_in_ai_modules() {
    let sources = collect_rs_sources();
    let mut violations: Vec<String> = Vec::new();

    for (path, content) in &sources {
        let norm = canonical_path(path);
        let is_allowed = ALLOWED_REQWEST_FILES
            .iter()
            .any(|suffix| norm.contains(suffix));

        if !is_allowed && content.contains("reqwest") {
            violations.push(path.clone());
        }
    }

    assert!(
        violations.is_empty(),
        "reqwest (HTTP client) found outside AI modules — potential privacy violation.\n\
         Only src/ai/mod.rs and src/commands/ai.rs may make network calls.\n\
         Affected files:\n{}",
        violations.join("\n")
    );
}

const TELEMETRY_PATTERNS: &[&str] = &[
    "segment",
    "amplitude",
    "sentry",
    "mixpanel",
    "google-analytics",
    "firebase",
    "crashlytics",
    "newrelic",
    "datadog",
];

#[test]
fn privacy_no_telemetry_in_rust_sources() {
    let sources = collect_rs_sources();
    let mut violations: Vec<String> = Vec::new();

    for (path, content) in &sources {
        let content_lower = content.to_lowercase();
        for pattern in TELEMETRY_PATTERNS {
            if content_lower.contains(pattern) {
                violations.push(format!("{path}: contains '{pattern}'"));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Telemetry/analytics references found in Rust sources:\n{}",
        violations.join("\n")
    );
}

#[test]
fn privacy_no_telemetry_in_cargo_toml() {
    let cargo_toml = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .expect("Failed to read Cargo.toml");

    let lower = cargo_toml.to_lowercase();
    for pattern in TELEMETRY_PATTERNS {
        assert!(
            !lower.contains(pattern),
            "Telemetry dependency '{pattern}' found in Cargo.toml"
        );
    }
}

#[test]
fn privacy_no_hardcoded_absolute_paths() {
    let sources = collect_rs_sources();
    // Hard-coded user-space absolute paths that should use the Tauri path API instead.
    let forbidden = &["/home/", "/Users/", "C:\\Users\\", "C:/Users/"];
    let mut violations: Vec<String> = Vec::new();

    for (path, content) in &sources {
        for pattern in forbidden {
            if content.contains(pattern) {
                violations.push(format!("{path}: contains '{pattern}'"));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Hardcoded absolute user-space paths found — use Tauri path API:\n{}",
        violations.join("\n")
    );
}

#[test]
fn privacy_database_opened_via_tauri_app_data_dir() {
    // The database must be opened using a path derived from Tauri's app_data_dir(),
    // not a hard-coded or env-resolved location.
    let main_rs = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.rs"),
    )
    .expect("Failed to read src/main.rs");

    assert!(
        main_rs.contains("app_data_dir"),
        "src/main.rs does not open the database via Tauri app_data_dir() — \
         data must live in the OS app-data dir"
    );
}
