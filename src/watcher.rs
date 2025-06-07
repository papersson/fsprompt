//! Filesystem watcher for auto-refresh functionality

use crate::core::types::CanonicalPath;
use notify::{Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};

/// Events from the filesystem watcher
#[derive(Debug, Clone)]
pub enum WatcherEvent {
    /// Files have changed in the watched directory
    Changed(Vec<PathBuf>),
    /// An error occurred while watching
    Error(String),
}

/// Manages filesystem watching with debouncing
#[derive(Debug)]
pub struct FsWatcher {
    /// The actual watcher instance
    watcher: Option<RecommendedWatcher>,
    /// Channel for receiving events
    rx: Receiver<WatcherEvent>,
    /// Channel for sending events (used by watcher)
    tx: Sender<WatcherEvent>,
    /// Last event time for debouncing
    last_event: Option<Instant>,
    /// Debounce duration
    debounce_duration: Duration,
}

impl FsWatcher {
    /// Create a new filesystem watcher
    pub fn new() -> Self {
        let (tx, rx) = channel();

        Self {
            watcher: None,
            rx,
            tx,
            last_event: None,
            debounce_duration: Duration::from_millis(500),
        }
    }

    /// Start watching a directory
    pub fn watch(&mut self, path: &CanonicalPath) -> Result<()> {
        // Stop any existing watcher
        self.stop();

        let tx = self.tx.clone();

        // Create a new watcher
        let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
            match res {
                Ok(event) => {
                    // Filter out events we don't care about
                    if event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove() {
                        let paths: Vec<PathBuf> = event.paths;
                        let _ = tx.send(WatcherEvent::Changed(paths));
                    }
                }
                Err(e) => {
                    let _ = tx.send(WatcherEvent::Error(e.to_string()));
                }
            }
        })?;

        // Start watching the path recursively
        watcher.watch(path.as_path(), RecursiveMode::Recursive)?;

        self.watcher = Some(watcher);
        self.last_event = None;

        Ok(())
    }

    /// Stop watching
    pub fn stop(&mut self) {
        self.watcher = None;
    }

    /// Check for events with debouncing
    pub fn check_events(&mut self) -> Option<WatcherEvent> {
        let now = Instant::now();

        // Drain all pending events
        let mut events = Vec::new();
        while let Ok(event) = self.rx.try_recv() {
            events.push(event);
        }

        if events.is_empty() {
            return None;
        }

        // Check debounce
        if let Some(last) = self.last_event {
            if now.duration_since(last) < self.debounce_duration {
                // Still in debounce period, don't report yet
                return None;
            }
        }

        // Update last event time
        self.last_event = Some(now);

        // Merge all change events
        let mut all_paths = Vec::new();
        let mut errors = Vec::new();

        for event in events {
            match event {
                WatcherEvent::Changed(paths) => all_paths.extend(paths),
                WatcherEvent::Error(e) => errors.push(e),
            }
        }

        // Return error if any occurred
        if !errors.is_empty() {
            return Some(WatcherEvent::Error(errors.join(", ")));
        }

        // Return changed paths if any
        if !all_paths.is_empty() {
            // Deduplicate paths
            all_paths.sort();
            all_paths.dedup();
            return Some(WatcherEvent::Changed(all_paths));
        }

        None
    }
}

impl Default for FsWatcher {
    fn default() -> Self {
        Self::new()
    }
}
