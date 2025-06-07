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
mod ui;
mod workers;

use core::types::{OutputFormat, TokenCount};
use workers::{WorkerCommand, WorkerEvent, WorkerHandle};

/// Output viewer tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputTab {
    /// Raw output as generated
    Raw,
    /// Pretty formatted XML (if XML format)
    PrettyXml,
    /// Rendered Markdown (if Markdown format)
    RenderedMarkdown,
}

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
    /// Currently selected output tab
    selected_tab: OutputTab,
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
}

impl FsPromptApp {
    /// Creates a new instance of the application
    #[must_use]
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected_path: None,
            tree: ui::tree::DirectoryTree::new(),
            split_pos: 0.3, // 30% for left panel
            output_content: String::new(),
            selected_tab: OutputTab::Raw,
            is_generating: false,
            output_format: OutputFormat::Xml,
            token_count: None,
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            include_tree: true,
            ignore_patterns: ".*,node_modules,__pycache__,target,build,dist,_*".to_string(),
            search_query: String::new(),
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

    /// Renders pretty formatted XML
    fn render_pretty_xml(&self, ui: &mut egui::Ui) {
        // For now, just display with syntax highlighting
        // In a full implementation, we'd parse and pretty-print the XML
        ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));

        // Simple XML syntax highlighting
        let mut job = egui::text::LayoutJob::default();

        for line in self.output_content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("<?xml") || trimmed.starts_with("<!") {
                // XML declaration or DOCTYPE
                job.append(
                    line,
                    0.0,
                    egui::TextFormat {
                        color: egui::Color32::from_rgb(128, 128, 128),
                        ..Default::default()
                    },
                );
            } else if trimmed.starts_with('<') && !trimmed.starts_with("</") {
                // Opening tags
                if let Some(end) = line.find('>') {
                    let tag_part = &line[..=end];
                    let rest = &line[end + 1..];

                    job.append(
                        tag_part,
                        0.0,
                        egui::TextFormat {
                            color: egui::Color32::from_rgb(34, 139, 34),
                            ..Default::default()
                        },
                    );
                    job.append(rest, 0.0, egui::TextFormat::default());
                } else {
                    job.append(line, 0.0, egui::TextFormat::default());
                }
            } else if trimmed.starts_with("</") {
                // Closing tags
                job.append(
                    line,
                    0.0,
                    egui::TextFormat {
                        color: egui::Color32::from_rgb(34, 139, 34),
                        ..Default::default()
                    },
                );
            } else {
                // Content
                job.append(line, 0.0, egui::TextFormat::default());
            }
            job.append("\n", 0.0, egui::TextFormat::default());
        }

        ui.label(job);
    }

    /// Renders markdown with basic formatting
    fn render_markdown(&self, ui: &mut egui::Ui) {
        // Basic markdown rendering
        for line in self.output_content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("# ") {
                // H1
                ui.style_mut().override_font_id = Some(egui::FontId::proportional(20.0));
                ui.label(&trimmed[2..]);
            } else if trimmed.starts_with("## ") {
                // H2
                ui.style_mut().override_font_id = Some(egui::FontId::proportional(18.0));
                ui.label(&trimmed[3..]);
            } else if trimmed.starts_with("### ") {
                // H3
                ui.style_mut().override_font_id = Some(egui::FontId::proportional(16.0));
                ui.label(&trimmed[4..]);
            } else if trimmed.starts_with("```") {
                // Code block - just show in monospace
                ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));
                ui.label(line);
            } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                // List item
                ui.horizontal(|ui| {
                    ui.label("•");
                    ui.label(&trimmed[2..]);
                });
            } else {
                // Regular text
                ui.style_mut().override_font_id = Some(egui::FontId::proportional(14.0));
                ui.label(line);
            }
        }
    }
}

impl eframe::App for FsPromptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process worker events
        self.process_worker_events(ctx);

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

                    self.tree.show_with_search(ui, &self.search_query);
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
                                .clicked()
                            {
                                self.copy_to_clipboard();
                            }
                        });
                    } else if !self.output_content.is_empty() {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("📋 Copy").clicked() {
                                self.copy_to_clipboard();
                            }

                            if ui.button("💾 Save").clicked() {
                                self.save_to_file();
                            }
                        });
                    }
                });
                ui.separator();

                // Tab bar
                if !self.output_content.is_empty() {
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(self.selected_tab == OutputTab::Raw, "Raw")
                            .clicked()
                        {
                            self.selected_tab = OutputTab::Raw;
                        }

                        match self.output_format {
                            OutputFormat::Xml => {
                                if ui
                                    .selectable_label(
                                        self.selected_tab == OutputTab::PrettyXml,
                                        "Pretty XML",
                                    )
                                    .clicked()
                                {
                                    self.selected_tab = OutputTab::PrettyXml;
                                }
                            }
                            OutputFormat::Markdown => {
                                if ui
                                    .selectable_label(
                                        self.selected_tab == OutputTab::RenderedMarkdown,
                                        "Rendered",
                                    )
                                    .clicked()
                                {
                                    self.selected_tab = OutputTab::RenderedMarkdown;
                                }
                            }
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
                            match self.selected_tab {
                                OutputTab::Raw => {
                                    // Use monospace font for code output
                                    ui.style_mut().override_font_id =
                                        Some(egui::FontId::monospace(12.0));
                                    ui.add(
                                        egui::TextEdit::multiline(
                                            &mut self.output_content.as_str(),
                                        )
                                        .desired_width(f32::INFINITY)
                                        .interactive(false),
                                    );
                                }
                                OutputTab::PrettyXml => {
                                    self.render_pretty_xml(ui);
                                }
                                OutputTab::RenderedMarkdown => {
                                    self.render_markdown(ui);
                                }
                            }
                        }
                    });
            });
        });
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
            selected_tab: OutputTab::Raw,
            is_generating: false,
            output_format: OutputFormat::Xml,
            token_count: None,
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            include_tree: true,
            ignore_patterns: String::new(),
            search_query: String::new(),
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
            selected_tab: OutputTab::Raw,
            is_generating: false,
            output_format: OutputFormat::Xml,
            token_count: None,
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            include_tree: true,
            ignore_patterns: String::new(),
            search_query: String::new(),
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
            selected_tab: OutputTab::Raw,
            is_generating: false,
            output_format: OutputFormat::Xml,
            token_count: None,
            worker: WorkerHandle::new(),
            current_progress: None,
            error_message: None,
            include_tree: true,
            ignore_patterns: String::new(),
            search_query: String::new(),
        };

        // Test that Debug is implemented correctly
        let debug_str = format!("{:?}", app);
        assert!(debug_str.contains("FsPromptApp"));
        assert!(debug_str.contains("selected_path"));
    }
}
