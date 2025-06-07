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

use core::types::{OutputFormat, TokenCount};

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
            is_generating: false,
            output_format: OutputFormat::Xml,
            token_count: None,
        }
    }

    /// Generates output from selected files
    fn generate_output(&mut self) {
        self.is_generating = true;
        self.output_content.clear();

        let selected_files = self.tree.collect_selected_files();

        if selected_files.is_empty() {
            self.output_content =
                "No files selected. Please select some files to generate output.".to_string();
            self.is_generating = false;
            return;
        }

        match self.output_format {
            OutputFormat::Xml => self.generate_xml(&selected_files),
            OutputFormat::Markdown => self.generate_markdown(&selected_files),
        }

        // Calculate token count
        self.token_count = Some(TokenCount::from_chars(self.output_content.chars().count()));

        self.is_generating = false;
    }

    /// Generates XML output
    fn generate_xml(&mut self, selected_files: &[std::path::PathBuf]) {
        self.output_content
            .push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        self.output_content.push_str("<codebase>\n");
        
        // Add directory tree
        self.output_content.push_str("  <directory-tree>\n");
        self.output_content.push_str("    <![CDATA[\n");
        let tree_string = self.tree.generate_tree_string();
        self.output_content.push_str(&tree_string);
        self.output_content.push_str("    ]]>\n");
        self.output_content.push_str("  </directory-tree>\n\n");

        for file_path in selected_files {
            match std::fs::read_to_string(file_path) {
                Ok(content) => {
                    self.output_content
                        .push_str(&format!("  <file path=\"{}\">\n", file_path.display()));
                    self.output_content.push_str("    <content><![CDATA[\n");
                    self.output_content.push_str(&content);
                    self.output_content.push_str("\n    ]]></content>\n");
                    self.output_content.push_str("  </file>\n");
                }
                Err(e) => {
                    eprintln!("Failed to read file {}: {}", file_path.display(), e);
                    self.output_content.push_str(&format!(
                        "  <!-- Error reading file {}: {} -->\n",
                        file_path.display(),
                        e
                    ));
                }
            }
        }

        self.output_content.push_str("</codebase>\n");
    }

    /// Generates Markdown output
    fn generate_markdown(&mut self, selected_files: &[std::path::PathBuf]) {
        self.output_content.push_str("# Codebase Export\n\n");
        self.output_content
            .push_str(&format!("Generated {} files\n\n", selected_files.len()));
        
        // Add directory tree
        self.output_content.push_str("## Directory Structure\n\n");
        self.output_content.push_str("```\n");
        let tree_string = self.tree.generate_tree_string();
        self.output_content.push_str(&tree_string);
        self.output_content.push_str("```\n\n");
        
        self.output_content.push_str("## File Contents\n\n");

        for file_path in selected_files {
            match std::fs::read_to_string(file_path) {
                Ok(content) => {
                    self.output_content
                        .push_str(&format!("## File: {}\n\n", file_path.display()));

                    // Determine file extension for syntax highlighting
                    let extension = file_path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("");

                    self.output_content.push_str(&format!("```{}\n", extension));
                    self.output_content.push_str(&content);
                    self.output_content.push_str("\n```\n\n");
                }
                Err(e) => {
                    eprintln!("Failed to read file {}: {}", file_path.display(), e);
                    self.output_content.push_str(&format!(
                        "> ⚠️ Error reading file {}: {}\n\n",
                        file_path.display(),
                        e
                    ));
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
}

impl eframe::App for FsPromptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

                    ui.separator();

                    // Generate button
                    ui.horizontal(|ui| {
                        let button_enabled = !self.is_generating;
                        if ui
                            .add_enabled(button_enabled, egui::Button::new("🚀 Generate"))
                            .clicked()
                        {
                            self.generate_output();
                        }

                        if self.is_generating {
                            ui.spinner();
                            ui.label("Generating...");
                        } else {
                            ui.label("Select files to include");
                        }
                    });

                    ui.separator();

                    self.tree.show(ui);
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
                            
                            // Add copy button
                            if ui
                                .add_enabled(!self.output_content.is_empty(), egui::Button::new("📋 Copy"))
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
                        });
                    }
                });
                ui.separator();

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
        };

        // Test that Debug is implemented correctly
        let debug_str = format!("{:?}", app);
        assert!(debug_str.contains("FsPromptApp"));
        assert!(debug_str.contains("selected_path"));
    }
}
