// File watcher for detecting new screenshots

use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::path::Path;
use std::time::Duration;

use crate::AppState;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    tx: std::sync::mpsc::Sender<String>,
}

impl FileWatcher {
    pub fn new(_app_state: AppState) -> Self {
        let (tx, _rx) = channel();
        let tx_clone = tx.clone();

        // Initialize watcher
        let watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, _>| {
                match res {
                    Ok(event) => {
                        for path in event.paths {
                            if path.extension().and_then(|e| e.to_str()) == Some("png")
                                || path.extension().and_then(|e| e.to_str()) == Some("jpg")
                                || path.extension().and_then(|e| e.to_str()) == Some("jpeg")
                                || path.extension().and_then(|e| e.to_str()) == Some("webp")
                            {
                                // Send path to main thread for processing
                                let _ = tx_clone.send(path.to_string_lossy().to_string());
                            }
                        }
                    }
                    Err(e) => eprintln!("Watch error: {:?}", e),
                }
            },
            notify::Config::default(),
        )
        .expect("Failed to create watcher");

        Self { watcher, tx }
    }

    pub fn start(&mut self) {
        // TODO: Add logic to add watch folders from app_state
        // For now, this is a stub
        println!("File watcher started");
    }

    pub fn add_folder(&mut self, path: &Path) -> Result<()> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn remove_folder(&mut self, path: &Path) -> Result<()> {
        self.watcher.unwatch(path)?;
        Ok(())
    }
}