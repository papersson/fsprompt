//! Main application state and core logic

use crate::core::types::{
    AppState, CanonicalPath, FileCount, HistorySize, OutputFormat, PatternString, ProgressCount,
    Theme,
};
use crate::state::{ConfigManager, HistoryManager, SelectionSnapshot};
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
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config_manager = ConfigManager::new();
        let old_config = config_manager.load();

        // Create AppState from old config
        let mut state = AppState::default();

        // Set root directory
        if let Some(path) = &old_config.last_directory {
            state.root = CanonicalPath::new(path).ok();
        }

        // Configure output
        state.output.format = match old_config.output_format.as_str() {
            "markdown" => OutputFormat::Markdown,
            _ => OutputFormat::Xml,
        };

        // Configure window
        state.config.window.left_pane_ratio = old_config.split_position;

        // Configure UI
        state.config.ui.theme = match old_config.theme.as_str() {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::System,
        };
        state.config.ui.include_tree = old_config.include_tree;

        // Configure ignore patterns
        if !old_config.ignore_patterns.is_empty() {
            state.config.ignore_patterns = old_config
                .ignore_patterns
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // Apply theme based on config
        let theme_str = match state.config.ui.theme {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "auto",
        };
        Self::apply_theme(cc, theme_str);

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
        }
    }

    /// Apply theme to the UI at creation time
    pub fn apply_theme(cc: &eframe::CreationContext<'_>, theme: &str) {
        Self::apply_theme_to_ctx(&cc.egui_ctx, theme);
    }

    /// Apply theme to context
    pub fn apply_theme_to_ctx(ctx: &egui::Context, theme: &str) {
        let dark_mode = match theme {
            "dark" => true,
            "light" => false,
            _ => Self::prefers_dark_theme(),
        };
        UiTheme::apply_theme(ctx, dark_mode);
    }

    /// Detect system theme preference
    pub fn prefers_dark_theme() -> bool {
        // On macOS, we can check the system appearance
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("defaults")
                .args(["read", "-g", "AppleInterfaceStyle"])
                .output()
            {
                return String::from_utf8_lossy(&output.stdout).trim() == "Dark";
            }
        }

        // Default to dark theme if we can't detect
        true
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
                    self.toast_manager.error(format!("Watcher error: {}", e));
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
                self.error_message = Some(format!("Failed to start generation: {}", e));
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
                        self.toast_manager.error(format!("Failed to copy: {}", e));
                    }
                },
                Err(e) => {
                    self.toast_manager
                        .error(format!("Failed to access clipboard: {}", e));
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

        let default_filename = format!("codebase_export.{}", extension);

        if let Some(content) = &self.state.output.content {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(&default_filename)
                .add_filter(&format!("{} files", extension.to_uppercase()), &[extension])
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
                            .error(format!("Failed to save file: {}", e));
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
    pub fn next_match(&mut self) {
        self.state.search.output_search.next_match();
    }

    /// Navigate to previous search match
    pub fn prev_match(&mut self) {
        self.state.search.output_search.prev_match();
    }

    /// Saves the current configuration
    pub fn save_config(&self) {
        let config = crate::state::AppConfig {
            window_width: self.state.config.window.width,
            window_height: self.state.config.window.height,
            split_position: self.state.config.window.left_pane_ratio,
            last_directory: self.state.root.as_ref().map(|p| p.as_path().to_path_buf()),
            ignore_patterns: self.state.config.ignore_patterns.join(", "),
            include_tree: self.state.config.ui.include_tree,
            output_format: match self.state.output.format {
                OutputFormat::Xml => "xml".to_string(),
                OutputFormat::Markdown => "markdown".to_string(),
            },
            theme: match self.state.config.ui.theme {
                Theme::Light => "light".to_string(),
                Theme::Dark => "dark".to_string(),
                Theme::System => "auto".to_string(),
            },
        };

        let _ = self.config_manager.save(&config);
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
        };

        // Test that Debug is implemented correctly
        let debug_str = format!("{:?}", app);
        assert!(debug_str.contains("FsPromptApp"));
        assert!(debug_str.contains("state"));
    }
}
