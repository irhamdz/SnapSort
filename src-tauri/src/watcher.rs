// File watcher for detecting new screenshots

use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::{Arc, Mutex, mpsc::channel};
use std::path::Path;

use crate::WatchState;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    tx: std::sync::mpsc::Sender<String>,
    watch_state: Arc<Mutex<WatchState>>,
}

impl FileWatcher {
    pub fn new(watch_state: Arc<Mutex<WatchState>>) -> Self {
        let (tx, _rx) = channel();
        let tx_clone = tx.clone();

        let watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, _>| {
                match res {
                    Ok(event) => {
                        for path in event.paths {
                            let ext = path.extension().and_then(|e| e.to_str());
                            if matches!(ext, Some("png") | Some("jpg") | Some("jpeg") | Some("webp")) {
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

        Self { watcher, tx, watch_state }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let folders = self.watch_state.lock().unwrap().watched_folders.clone();
        for folder in folders {
            self.watcher.watch(Path::new(&folder), RecursiveMode::Recursive)?;
        }
        Ok(())
    }

    pub async fn reinitialize(&mut self) -> Result<()> {
        self.initialize().await
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