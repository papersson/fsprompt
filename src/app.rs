//! Main application state and core logic

use crate::core::types::{
    AppState, FileCount, HistorySize, OutputFormat, PatternString, ProgressCount, Theme,
};
use crate::state::{ConfigManager, HistoryManager, SelectionSnapshot};
use crate::ui::components::AnimatedButtonManager;
use crate::ui::icons::IconManager;
use crate::ui::toast::ToastManager;
use crate::ui::Theme as UiTheme;
use crate::utils::perf::PerfOverlay;
use crate::watcher::FsWatcher;
use crate::workers::{WorkerCommand, WorkerEvent, WorkerHandle};
use eframe::egui;
use std::sync::Arc;

/// The main application struct that holds all state
#[derive(Debug)]
pub struct FsPromptApp {
    /// Core application state
    pub state: AppState,
    /// Directory tree widget (temporary until fully migrated)
    pub tree: crate::ui::tree::DirectoryTree,
    /// Worker thread handle (temporary until fully migrated)
    pub worker: WorkerHandle,
    /// Current progress stage (temporary)
    pub current_progress: Option<(crate::workers::ProgressStage, ProgressCount)>,
    /// Error message to display (temporary)
    pub error_message: Option<String>,
    /// Configuration manager
    pub config_manager: ConfigManager,
    /// History manager for undo/redo
    pub history_manager: HistoryManager,
    /// Toast notification manager
    pub toast_manager: ToastManager,
    /// Filesystem watcher
    pub fs_watcher: FsWatcher,
    /// Whether files have changed since last generation
    pub files_changed: bool,
    /// Performance overlay
    pub perf_overlay: PerfOverlay,
    /// Active tab for narrow/mobile view
    pub active_tab: TabView,
    /// Input field for new ignore pattern
    pub new_pattern_input: String,
    /// Saved ignore patterns for tracking changes
    pub saved_ignore_patterns: Vec<String>,
    /// Icon manager for SVG icons
    pub icon_manager: IconManager,
    /// Animation manager for smooth UI transitions
    pub animation_manager: AnimatedButtonManager,
    /// Last applied theme to avoid redundant applications
    last_applied_theme: Option<(Theme, bool)>, // (theme_setting, resolved_dark_mode)
}

/// Tab view for narrow/mobile layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabView {
    /// File tree view
    Files,
    /// Output preview view
    Output,
}

impl FsPromptApp {
    /// Creates a new instance of the application
    #[must_use]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config_manager = ConfigManager::new();
        // Load configuration
        let loaded_config = config_manager.load();

        // Create AppState with loaded config
        let state = AppState {
            config: loaded_config,
            ..AppState::default()
        };

        // Save a copy of the loaded ignore patterns
        let saved_patterns = state.config.ignore_patterns.clone();

        Self {
            state,
            tree: crate::ui::tree::DirectoryTree::new(),
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            config_manager,
            history_manager: HistoryManager::new(HistorySize::default()),
            toast_manager: ToastManager::new(),
            fs_watcher: FsWatcher::new(),
            files_changed: false,
            perf_overlay: PerfOverlay::default(),
            active_tab: TabView::Files,
            new_pattern_input: String::new(),
            saved_ignore_patterns: saved_patterns,
            icon_manager: IconManager::new(),
            animation_manager: AnimatedButtonManager::new(),
            last_applied_theme: None,
        }
    }

    /// Detect system theme preference using dark-light crate
    pub fn prefers_dark_theme() -> bool {
        match dark_light::detect() {
            Ok(dark_light::Mode::Dark) => true,
            Ok(dark_light::Mode::Light | dark_light::Mode::Unspecified) | Err(_) => false, // Default to light mode
        }
    }

    /// Reset theme cache (for immediate theme changes)
    pub fn reset_theme_cache(&mut self) {
        self.last_applied_theme = None;
    }

    /// Apply theme if needed (only when it changes)
    pub fn apply_theme_if_needed(&mut self, ctx: &egui::Context) {
        // Always use dark mode for now
        let dark_mode = true;

        // Only apply on first run
        if self.last_applied_theme.is_none() {
            UiTheme::apply_theme(ctx, dark_mode);
            self.last_applied_theme = Some((Theme::Dark, dark_mode));
        }
    }

    /// Check for filesystem changes
    pub fn check_fs_changes(&mut self, ctx: &egui::Context) {
        if let Some(event) = self.fs_watcher.check_events() {
            match event {
                crate::watcher::WatcherEvent::Changed(paths) => {
                    self.files_changed = true;
                    let count = FileCount::new(paths.len());
                    if count.get() == 1 {
                        self.toast_manager
                            .info("1 file changed in the watched directory");
                    } else {
                        self.toast_manager.info(format!(
                            "{} files changed in the watched directory",
                            count.get()
                        ));
                    }
                    ctx.request_repaint();
                }
                crate::watcher::WatcherEvent::Error(e) => {
                    self.toast_manager.error(format!("Watcher error: {e}"));
                }
            }
        }
    }

    /// Generates output from selected files
    pub fn generate_output(&mut self) {
        let selected_files = self.tree.collect_selected_files();

        if selected_files.is_empty() {
            self.error_message =
                Some("No files selected. Please select some files to generate output.".to_string());
            return;
        }

        if let Some(root_path) = &self.state.root {
            self.state.output.generating = true;
            self.state.output.content = None;
            self.state.output.tokens = None;
            self.error_message = None;
            self.current_progress = None;
            self.files_changed = false;

            let command = WorkerCommand::GenerateOutput {
                root_path: root_path.clone(),
                selected_files,
                format: self.state.output.format,
                include_tree: self.state.config.ui.include_tree,
                ignore_patterns: PatternString::from_patterns(&self.state.config.ignore_patterns),
            };

            if let Err(e) = self.worker.send_command(command) {
                self.error_message = Some(format!("Failed to start generation: {e}"));
                self.state.output.generating = false;
            }
        }
    }

    /// Processes events from the worker thread
    pub fn process_worker_events(&mut self, ctx: &egui::Context) {
        while let Some(event) = self.worker.try_recv_event() {
            match event {
                WorkerEvent::Progress { stage, progress } => {
                    self.current_progress = Some((stage, progress));
                    ctx.request_repaint();
                }
                WorkerEvent::OutputReady {
                    content,
                    token_count,
                } => {
                    self.state.output.content = Some(Arc::new(content));
                    self.state.output.tokens = Some(token_count);
                    self.state.output.generating = false;
                    self.current_progress = None;
                    self.toast_manager
                        .success(format!("Generated {} tokens", token_count.get()));
                    ctx.request_repaint();
                }
                WorkerEvent::Error(msg) => {
                    self.error_message = Some(msg.clone());
                    self.toast_manager.error(msg);
                    // Don't stop generation here, as we might still get output
                    ctx.request_repaint();
                }
                WorkerEvent::Cancelled => {
                    self.state.output.generating = false;
                    self.current_progress = None;
                    self.error_message = Some("Generation cancelled".to_string());
                    self.toast_manager.warning("Generation cancelled");
                    ctx.request_repaint();
                }
            }
        }
    }

    /// Copies the output content to clipboard
    pub fn copy_to_clipboard(&mut self) {
        use arboard::Clipboard;

        if let Some(content) = &self.state.output.content {
            match Clipboard::new() {
                Ok(mut clipboard) => match clipboard.set_text(content.as_str()) {
                    Ok(()) => {
                        self.toast_manager.success("Copied to clipboard!");
                    }
                    Err(e) => {
                        self.toast_manager.error(format!("Failed to copy: {e}"));
                    }
                },
                Err(e) => {
                    self.toast_manager
                        .error(format!("Failed to access clipboard: {e}"));
                }
            }
        }
    }

    /// Saves the output content to a file
    pub fn save_to_file(&mut self) {
        let extension = match self.state.output.format {
            OutputFormat::Xml => "xml",
            OutputFormat::Markdown => "md",
        };

        let default_filename = format!("codebase_export.{extension}");

        if let Some(content) = &self.state.output.content {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(&default_filename)
                .add_filter(format!("{} files", extension.to_uppercase()), &[extension])
                .add_filter("All files", &["*"])
                .save_file()
            {
                match std::fs::write(&path, content.as_str()) {
                    Ok(()) => {
                        self.toast_manager.success(format!(
                            "Saved to {}",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        ));
                    }
                    Err(e) => {
                        self.toast_manager
                            .error(format!("Failed to save file: {e}"));
                    }
                }
            }
        }
    }

    /// Updates search match count
    pub fn update_search_matches(&mut self) {
        if self.state.search.output_search.query.is_empty() {
            self.state.search.output_search.match_count = 0;
            self.state.search.output_search.current_match = 0;
            return;
        }

        if let Some(content) = &self.state.output.content {
            let query = self.state.search.output_search.query.to_lowercase();
            let content_lower = content.to_lowercase();

            self.state.search.output_search.match_count = content_lower.matches(&query).count();

            // Reset to first match
            if self.state.search.output_search.match_count > 0 {
                self.state.search.output_search.current_match = 0;
            }
        }
    }

    /// Navigate to next search match
    #[allow(clippy::missing_const_for_fn)] // Cannot be const due to &mut self
    pub fn next_match(&mut self) {
        self.state.search.output_search.next_match();
    }

    /// Navigate to previous search match
    #[allow(clippy::missing_const_for_fn)] // Cannot be const due to &mut self
    pub fn prev_match(&mut self) {
        self.state.search.output_search.prev_match();
    }

    /// Saves the current configuration
    pub fn save_config(&self) {
        let _ = self.config_manager.save(&self.state.config);
    }

    /// Captures current selection state
    pub fn capture_snapshot(&self) -> SelectionSnapshot {
        SelectionSnapshot {
            selected_files: self.tree.get_selected_files(),
            expanded_dirs: self.tree.get_expanded_dirs(),
        }
    }

    /// Restores a selection state
    pub fn restore_snapshot(&mut self, snapshot: &SelectionSnapshot) {
        self.tree
            .restore_selection(&snapshot.selected_files, &snapshot.expanded_dirs);
    }

    /// Records the current state for undo
    pub fn record_state(&mut self) {
        let snapshot = self.capture_snapshot();
        self.history_manager.push(snapshot);
    }

    /// Handles undo operation
    pub fn undo(&mut self) {
        let current = self.capture_snapshot();
        if let Some(previous) = self.history_manager.undo(current) {
            self.restore_snapshot(&previous);
        }
    }

    /// Handles redo operation
    pub fn redo(&mut self) {
        let current = self.capture_snapshot();
        if let Some(next) = self.history_manager.redo(current) {
            self.restore_snapshot(&next);
        }
    }

    /// Stop watching filesystem when exiting
    pub fn on_exit(&mut self) {
        self.fs_watcher.stop();
        self.save_config();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        // Test that we can create an app instance
        // Note: We can't easily test the CreationContext, so we use a simplified test
        let app = FsPromptApp {
            state: AppState::default(),
            tree: crate::ui::tree::DirectoryTree::new(),
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            config_manager: ConfigManager::new(),
            history_manager: HistoryManager::new(HistorySize::default()),
            toast_manager: ToastManager::new(),
            fs_watcher: FsWatcher::new(),
            files_changed: false,
            perf_overlay: PerfOverlay::default(),
            active_tab: TabView::Files,
            new_pattern_input: String::new(),
            saved_ignore_patterns: Vec::new(),
            icon_manager: crate::ui::icons::IconManager::new(),
            animation_manager: crate::ui::components::AnimatedButtonManager::new(),
            last_applied_theme: None,
        };

        assert!(app.state.root.is_none());
        assert!(app.state.output.content.is_none());
        assert!(!app.state.output.generating);
    }

    #[test]
    fn test_app_with_path() {
        // Since CanonicalPath requires the path to exist, we'll test the structure
        let mut app = FsPromptApp {
            state: AppState::default(),
            tree: crate::ui::tree::DirectoryTree::new(),
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            config_manager: ConfigManager::new(),
            history_manager: HistoryManager::new(HistorySize::default()),
            toast_manager: ToastManager::new(),
            fs_watcher: FsWatcher::new(),
            files_changed: false,
            perf_overlay: PerfOverlay::default(),
            active_tab: TabView::Files,
            new_pattern_input: String::new(),
            saved_ignore_patterns: Vec::new(),
            icon_manager: crate::ui::icons::IconManager::new(),
            animation_manager: crate::ui::components::AnimatedButtonManager::new(),
            last_applied_theme: None,
        };

        // Test that we can set output format
        app.state.output.format = OutputFormat::Markdown;
        assert_eq!(app.state.output.format, OutputFormat::Markdown);
    }

    #[test]
    fn test_app_debug_impl() {
        let app = FsPromptApp {
            state: AppState::default(),
            tree: crate::ui::tree::DirectoryTree::new(),
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            config_manager: ConfigManager::new(),
            history_manager: HistoryManager::new(HistorySize::default()),
            toast_manager: ToastManager::new(),
            fs_watcher: FsWatcher::new(),
            files_changed: false,
            perf_overlay: PerfOverlay::default(),
            active_tab: TabView::Files,
            new_pattern_input: String::new(),
            saved_ignore_patterns: Vec::new(),
            icon_manager: crate::ui::icons::IconManager::new(),
            animation_manager: crate::ui::components::AnimatedButtonManager::new(),
            last_applied_theme: None,
        };

        // Test that Debug is implemented correctly
        let debug_str = format!("{app:?}");
        assert!(debug_str.contains("FsPromptApp"));
        assert!(debug_str.contains("state"));
    }
}
