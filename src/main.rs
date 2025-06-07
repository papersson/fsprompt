#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs
)]
#![allow(clippy::module_name_repetitions)] // Common in Rust APIs
#![allow(clippy::must_use_candidate)] // We'll add these selectively

//! fsPrompt - A high-performance filesystem prompt generator for LLMs
//!
//! This application allows users to generate context prompts from codebases,
//! producing XML or Markdown output containing directory structure and file contents.

use eframe::egui;

mod core;
mod state;
mod ui;
mod workers;

use core::types::{OutputFormat, TokenCount};
use state::{AppConfig, ConfigManager, HistoryManager, SelectionSnapshot};
use workers::{WorkerCommand, WorkerEvent, WorkerHandle};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "fsPrompt",
        native_options,
        Box::new(|cc| Ok(Box::new(FsPromptApp::new(cc)))),
    )
}

/// The main application struct that holds all state
#[derive(Debug)]
struct FsPromptApp {
    /// The currently selected directory path
    selected_path: Option<std::path::PathBuf>,
    /// Directory tree widget
    tree: ui::tree::DirectoryTree,
    /// Split position (percentage of left panel width)
    split_pos: f32,
    /// Generated output content
    output_content: String,
    /// Whether we're currently generating output
    is_generating: bool,
    /// Current output format
    output_format: OutputFormat,
    /// Estimated token count
    token_count: Option<TokenCount>,
    /// Worker thread handle
    worker: WorkerHandle,
    /// Current progress stage
    current_progress: Option<(workers::ProgressStage, usize, usize)>,
    /// Error message to display
    error_message: Option<String>,
    /// Whether to include directory tree in output
    include_tree: bool,
    /// Ignore patterns input
    ignore_patterns: String,
    /// Search query for filtering the tree
    search_query: String,
    /// Output search state
    output_search_active: bool,
    /// Output search query
    output_search_query: String,
    /// Current search match index
    output_search_match_index: usize,
    /// Total number of search matches
    output_search_match_count: usize,
    /// Configuration manager
    config_manager: ConfigManager,
    /// History manager for undo/redo
    history_manager: HistoryManager,
}

impl FsPromptApp {
    /// Creates a new instance of the application
    #[must_use]
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config_manager = ConfigManager::new();
        let config = config_manager.load();

        // Apply saved window size
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        // Note: Window size is set via NativeOptions in main()

        let output_format = match config.output_format.as_str() {
            "markdown" => OutputFormat::Markdown,
            _ => OutputFormat::Xml,
        };

        Self {
            selected_path: config.last_directory.clone(),
            tree: ui::tree::DirectoryTree::new(),
            split_pos: config.split_position,
            output_content: String::new(),
            is_generating: false,
            output_format,
            token_count: None,
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            include_tree: config.include_tree,
            ignore_patterns: if config.ignore_patterns.is_empty() {
                ".*,node_modules,__pycache__,target,build,dist,_*".to_string()
            } else {
                config.ignore_patterns
            },
            search_query: String::new(),
            output_search_active: false,
            output_search_query: String::new(),
            output_search_match_index: 0,
            output_search_match_count: 0,
            config_manager,
            history_manager: HistoryManager::new(20),
        }
    }

    /// Generates output from selected files
    fn generate_output(&mut self) {
        let selected_files = self.tree.collect_selected_files();

        if selected_files.is_empty() {
            self.error_message =
                Some("No files selected. Please select some files to generate output.".to_string());
            return;
        }

        if let Some(root_path) = &self.selected_path {
            self.is_generating = true;
            self.output_content.clear();
            self.token_count = None;
            self.error_message = None;
            self.current_progress = None;

            let command = WorkerCommand::GenerateOutput {
                root_path: root_path.clone(),
                selected_files,
                format: self.output_format,
                include_tree: self.include_tree,
                ignore_patterns: self.ignore_patterns.clone(),
            };

            if let Err(e) = self.worker.send_command(command) {
                self.error_message = Some(format!("Failed to start generation: {}", e));
                self.is_generating = false;
            }
        }
    }

    /// Processes events from the worker thread
    fn process_worker_events(&mut self, ctx: &egui::Context) {
        while let Some(event) = self.worker.try_recv_event() {
            match event {
                WorkerEvent::Progress {
                    stage,
                    current,
                    total,
                } => {
                    self.current_progress = Some((stage, current, total));
                    ctx.request_repaint();
                }
                WorkerEvent::OutputReady {
                    content,
                    token_count,
                } => {
                    self.output_content = content;
                    self.token_count = Some(TokenCount::from_chars(token_count * 4)); // Convert back from token count
                    self.is_generating = false;
                    self.current_progress = None;
                    ctx.request_repaint();
                }
                WorkerEvent::Error(msg) => {
                    self.error_message = Some(msg);
                    // Don't stop generation here, as we might still get output
                    ctx.request_repaint();
                }
                WorkerEvent::Cancelled => {
                    self.is_generating = false;
                    self.current_progress = None;
                    self.error_message = Some("Generation cancelled".to_string());
                    ctx.request_repaint();
                }
            }
        }
    }

    /// Copies the output content to clipboard
    fn copy_to_clipboard(&self) {
        use arboard::Clipboard;

        match Clipboard::new() {
            Ok(mut clipboard) => {
                match clipboard.set_text(&self.output_content) {
                    Ok(()) => {
                        // TODO: Show success toast when toast system is implemented
                        println!("Copied to clipboard!");
                    }
                    Err(e) => {
                        eprintln!("Failed to copy to clipboard: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to access clipboard: {}", e);
            }
        }
    }

    /// Saves the output content to a file
    fn save_to_file(&self) {
        let extension = match self.output_format {
            OutputFormat::Xml => "xml",
            OutputFormat::Markdown => "md",
        };

        let default_filename = format!("codebase_export.{}", extension);

        if let Some(path) = rfd::FileDialog::new()
            .set_file_name(&default_filename)
            .add_filter(&format!("{} files", extension.to_uppercase()), &[extension])
            .add_filter("All files", &["*"])
            .save_file()
        {
            match std::fs::write(&path, &self.output_content) {
                Ok(()) => {
                    // TODO: Show success toast when toast system is implemented
                    println!("Saved to: {}", path.display());
                }
                Err(e) => {
                    eprintln!("Failed to save file: {}", e);
                }
            }
        }
    }

    /// Updates search match count
    fn update_search_matches(&mut self) {
        if self.output_search_query.is_empty() {
            self.output_search_match_count = 0;
            self.output_search_match_index = 0;
            return;
        }

        let query = self.output_search_query.to_lowercase();
        let content = self.output_content.to_lowercase();

        self.output_search_match_count = content.matches(&query).count();

        // Reset to first match
        if self.output_search_match_count > 0 {
            self.output_search_match_index = 0;
        }
    }

    /// Navigate to next search match
    fn next_match(&mut self) {
        if self.output_search_match_count > 0 {
            self.output_search_match_index =
                (self.output_search_match_index + 1) % self.output_search_match_count;
        }
    }

    /// Navigate to previous search match
    fn prev_match(&mut self) {
        if self.output_search_match_count > 0 {
            if self.output_search_match_index == 0 {
                self.output_search_match_index = self.output_search_match_count - 1;
            } else {
                self.output_search_match_index -= 1;
            }
        }
    }

    /// Saves the current configuration
    fn save_config(&self) {
        let config = AppConfig {
            window_width: 1200.0, // We'll get actual size later
            window_height: 800.0,
            split_position: self.split_pos,
            last_directory: self.selected_path.clone(),
            ignore_patterns: self.ignore_patterns.clone(),
            include_tree: self.include_tree,
            output_format: match self.output_format {
                OutputFormat::Xml => "xml".to_string(),
                OutputFormat::Markdown => "markdown".to_string(),
            },
        };

        let _ = self.config_manager.save(&config);
    }

    /// Captures current selection state
    fn capture_snapshot(&self) -> SelectionSnapshot {
        SelectionSnapshot {
            selected_files: self.tree.get_selected_files(),
            expanded_dirs: self.tree.get_expanded_dirs(),
        }
    }

    /// Restores a selection state
    fn restore_snapshot(&mut self, snapshot: &SelectionSnapshot) {
        self.tree
            .restore_selection(&snapshot.selected_files, &snapshot.expanded_dirs);
    }

    /// Records the current state for undo
    fn record_state(&mut self) {
        let snapshot = self.capture_snapshot();
        self.history_manager.push(snapshot);
    }

    /// Handles undo operation
    fn undo(&mut self) {
        let current = self.capture_snapshot();
        if let Some(previous) = self.history_manager.undo(current) {
            self.restore_snapshot(&previous);
        }
    }

    /// Handles redo operation
    fn redo(&mut self) {
        let current = self.capture_snapshot();
        if let Some(next) = self.history_manager.redo(current) {
            self.restore_snapshot(&next);
        }
    }
}

impl eframe::App for FsPromptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process worker events
        self.process_worker_events(ctx);

        // Global keyboard shortcuts
        ctx.input(|i| {
            // Ctrl+F for output search (only when output is available and not in tree search)
            if i.modifiers.ctrl
                && i.key_pressed(egui::Key::F)
                && !self.output_content.is_empty()
                && !i.focused
            {
                self.output_search_active = true;
            }

            // Ctrl+G for Generate (when not generating and path is selected)
            if i.modifiers.ctrl
                && i.key_pressed(egui::Key::G)
                && !self.is_generating
                && self.selected_path.is_some()
            {
                self.generate_output();
            }

            // Ctrl+C for Copy (when output is available)
            if i.modifiers.ctrl && i.key_pressed(egui::Key::C) && !self.output_content.is_empty() {
                self.copy_to_clipboard();
            }

            // Ctrl+S for Save (when output is available)
            if i.modifiers.ctrl && i.key_pressed(egui::Key::S) && !self.output_content.is_empty() {
                self.save_to_file();
            }

            // Ctrl+Z for Undo
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::Z) {
                self.undo();
            }

            // Ctrl+Shift+Z for Redo
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::Z) {
                self.redo();
            }
        });

        // Top panel with title and directory selector
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading("fsPrompt");
                ui.separator();

                if ui.button("Select Directory").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.selected_path = Some(path.clone());
                        self.tree.set_root(path);
                    }
                }

                if let Some(path) = &self.selected_path {
                    ui.label(format!("Selected: {}", path.display()));
                }
            });
            ui.add_space(4.0);
        });

        // Calculate panel widths
        let available_width = ctx.available_rect().width();
        let left_width = available_width * self.split_pos;

        // Left panel with directory tree and controls
        egui::SidePanel::left("left_panel")
            .default_width(left_width)
            .width_range(200.0..=available_width - 200.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Files & Directories");
                    ui.separator();

                    // Format selection
                    ui.horizontal(|ui| {
                        ui.label("Output format:");
                        ui.radio_value(&mut self.output_format, OutputFormat::Xml, "XML");
                        ui.radio_value(&mut self.output_format, OutputFormat::Markdown, "Markdown");
                    });

                    // Include tree checkbox
                    ui.checkbox(&mut self.include_tree, "Include directory tree in output");

                    // Ignore patterns
                    ui.vertical(|ui| {
                        ui.label("Ignore patterns (comma-separated):");
                        ui.text_edit_singleline(&mut self.ignore_patterns)
                            .on_hover_text("e.g., .*, node_modules, __pycache__, target, _*");
                    });

                    ui.separator();

                    // Search bar
                    ui.horizontal(|ui| {
                        ui.label("🔍");
                        let response = ui
                            .text_edit_singleline(&mut self.search_query)
                            .on_hover_text("Search for files and folders");

                        // Clear button
                        if !self.search_query.is_empty() && ui.small_button("✕").clicked() {
                            self.search_query.clear();
                        }

                        // Focus on Ctrl+F
                        if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::F)) {
                            response.request_focus();
                        }
                    });

                    ui.separator();

                    // Generate button
                    ui.horizontal(|ui| {
                        let button_enabled = !self.is_generating && self.selected_path.is_some();
                        if ui
                            .add_enabled(button_enabled, egui::Button::new("🚀 Generate"))
                            .on_hover_text("Generate output (Ctrl+G)")
                            .clicked()
                        {
                            self.generate_output();
                        }

                        if self.is_generating {
                            ui.spinner();

                            if let Some((stage, current, total)) = &self.current_progress {
                                let stage_text = match stage {
                                    workers::ProgressStage::ScanningFiles => "Scanning files",
                                    workers::ProgressStage::ReadingFiles => "Reading files",
                                    workers::ProgressStage::BuildingOutput => "Building output",
                                };
                                ui.label(format!("{}: {}/{}", stage_text, current, total));
                            } else {
                                ui.label("Starting...");
                            }

                            if ui.button("Cancel").clicked() {
                                let _ = self.worker.send_command(WorkerCommand::Cancel);
                            }
                        } else if self.selected_path.is_none() {
                            ui.label("Select a directory first");
                        } else {
                            ui.label("Select files to include");
                        }
                    });

                    ui.separator();

                    // Show error message if any
                    if let Some(error) = &self.error_message {
                        ui.colored_label(
                            egui::Color32::from_rgb(244, 67, 54),
                            format!("⚠️ {}", error),
                        );
                        ui.separator();
                    }

                    // Track selection state before showing tree
                    let snapshot_before = self.capture_snapshot();

                    self.tree.show_with_search(ui, &self.search_query);

                    // Check if selection changed and record state
                    let snapshot_after = self.capture_snapshot();
                    if snapshot_before.selected_files != snapshot_after.selected_files {
                        self.record_state();
                    }
                });
            });

        // Right panel with output
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Output Preview");

                    if let Some(token_count) = self.token_count {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let level = token_count.level();
                            let (label, color) = match level {
                                core::types::TokenLevel::Low => {
                                    ("Low", egui::Color32::from_rgb(76, 175, 80))
                                }
                                core::types::TokenLevel::Medium => {
                                    ("Medium", egui::Color32::from_rgb(255, 152, 0))
                                }
                                core::types::TokenLevel::High => {
                                    ("High", egui::Color32::from_rgb(244, 67, 54))
                                }
                            };

                            ui.colored_label(color, format!("◆ {} tokens", token_count.get()));
                            ui.colored_label(color, format!("[{}]", label));

                            ui.separator();

                            // Add save button
                            if ui
                                .add_enabled(
                                    !self.output_content.is_empty(),
                                    egui::Button::new("💾 Save"),
                                )
                                .on_hover_text("Save to file (Ctrl+S)")
                                .clicked()
                            {
                                self.save_to_file();
                            }

                            // Add copy button
                            if ui
                                .add_enabled(
                                    !self.output_content.is_empty(),
                                    egui::Button::new("📋 Copy"),
                                )
                                .on_hover_text("Copy to clipboard (Ctrl+C)")
                                .clicked()
                            {
                                self.copy_to_clipboard();
                            }
                        });
                    } else if !self.output_content.is_empty() {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button("📋 Copy")
                                .on_hover_text("Copy to clipboard (Ctrl+C)")
                                .clicked()
                            {
                                self.copy_to_clipboard();
                            }

                            if ui
                                .button("💾 Save")
                                .on_hover_text("Save to file (Ctrl+S)")
                                .clicked()
                            {
                                self.save_to_file();
                            }
                        });
                    }
                });
                ui.separator();

                // Search bar for output
                if self.output_search_active && !self.output_content.is_empty() {
                    ui.horizontal(|ui| {
                        ui.label("🔍 Find:");
                        let response = ui.text_edit_singleline(&mut self.output_search_query);

                        if response.changed() {
                            self.update_search_matches();
                        }

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                            self.output_search_active = false;
                            self.output_search_query.clear();
                        }

                        response.request_focus();

                        // Show match count and navigation
                        if !self.output_search_query.is_empty()
                            && self.output_search_match_count > 0
                        {
                            ui.label(format!(
                                "{} / {}",
                                self.output_search_match_index + 1,
                                self.output_search_match_count
                            ));

                            if ui.small_button("↑").clicked() {
                                self.prev_match();
                            }

                            if ui.small_button("↓").clicked() {
                                self.next_match();
                            }
                        } else if !self.output_search_query.is_empty() {
                            ui.label("No matches");
                        }

                        if ui.small_button("✕").clicked() {
                            self.output_search_active = false;
                            self.output_search_query.clear();
                        }
                    });
                    ui.separator();
                }

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.output_content.is_empty() {
                            ui.label("Generated output will appear here...");
                        } else {
                            // Use monospace font for code output
                            ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));
                            ui.add(
                                egui::TextEdit::multiline(&mut self.output_content.as_str())
                                    .desired_width(f32::INFINITY)
                                    .interactive(false),
                            );
                        }
                    });
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save configuration when exiting
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
            selected_path: None,
            tree: ui::tree::DirectoryTree::new(),
            split_pos: 0.3,
            output_content: String::new(),
            is_generating: false,
            output_format: OutputFormat::Xml,
            token_count: None,
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            include_tree: true,
            ignore_patterns: String::new(),
            search_query: String::new(),
            output_search_active: false,
            output_search_query: String::new(),
            output_search_match_index: 0,
            output_search_match_count: 0,
            config_manager: ConfigManager::new(),
            history_manager: HistoryManager::new(20),
        };

        assert!(app.selected_path.is_none());
        assert!(app.output_content.is_empty());
        assert!(!app.is_generating);
    }

    #[test]
    fn test_app_with_path() {
        let test_path = std::path::PathBuf::from("/test/path");
        let app = FsPromptApp {
            selected_path: Some(test_path.clone()),
            tree: ui::tree::DirectoryTree::new(),
            split_pos: 0.3,
            output_content: String::new(),
            is_generating: false,
            output_format: OutputFormat::Xml,
            token_count: None,
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            include_tree: true,
            ignore_patterns: String::new(),
            search_query: String::new(),
            output_search_active: false,
            output_search_query: String::new(),
            output_search_match_index: 0,
            output_search_match_count: 0,
            config_manager: ConfigManager::new(),
            history_manager: HistoryManager::new(20),
        };

        assert_eq!(app.selected_path, Some(test_path));
    }

    #[test]
    fn test_app_debug_impl() {
        let app = FsPromptApp {
            selected_path: None,
            tree: ui::tree::DirectoryTree::new(),
            split_pos: 0.3,
            output_content: String::new(),
            is_generating: false,
            output_format: OutputFormat::Xml,
            token_count: None,
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            include_tree: true,
            ignore_patterns: String::new(),
            search_query: String::new(),
            output_search_active: false,
            output_search_query: String::new(),
            output_search_match_index: 0,
            output_search_match_count: 0,
            config_manager: ConfigManager::new(),
            history_manager: HistoryManager::new(20),
        };

        // Test that Debug is implemented correctly
        let debug_str = format!("{:?}", app);
        assert!(debug_str.contains("FsPromptApp"));
        assert!(debug_str.contains("selected_path"));
    }
}
