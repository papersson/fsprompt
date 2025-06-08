//! UI rendering logic for the main application

use crate::app::FsPromptApp;
use crate::core::types::{OutputFormat, TokenLevel};
use crate::ui::{TextEmphasis, Theme as UiTheme};
use crate::workers::WorkerCommand;
use eframe::egui;

impl FsPromptApp {
    /// Shows just the action bar (for global positioning)
    #[allow(clippy::too_many_lines)]
    pub fn show_action_bar(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(4.0);
            // Real-time stats
            ui.horizontal(|ui| {
                let selected_count = self.tree.get_selected_files().len();
                let token_estimate = self.estimate_tokens_for_selection();

                // Selection info
                ui.label(
                    egui::RichText::new(format!("📁 {selected_count} files selected")).color(
                        UiTheme::text_color(Self::prefers_dark_theme(), TextEmphasis::Secondary),
                    ),
                );

                ui.separator();

                // Token count with color coding
                let (token_color, token_label) = if token_estimate < 10_000 {
                    (UiTheme::SUCCESS, "Low")
                } else if token_estimate < 50_000 {
                    (UiTheme::WARNING, "Medium")
                } else {
                    (UiTheme::ERROR, "High")
                };

                ui.colored_label(
                    token_color,
                    format!(
                        "🔢 ~{} tokens ({})",
                        format_token_count(token_estimate),
                        token_label
                    ),
                );

                ui.separator();

                // Output format toggle (compact)
                ui.label("Format:");
                if ui
                    .selectable_label(self.state.output.format == OutputFormat::Xml, "XML")
                    .clicked()
                {
                    self.state.output.format = OutputFormat::Xml;
                }
                ui.label("|");
                if ui
                    .selectable_label(
                        self.state.output.format == OutputFormat::Markdown,
                        "Markdown",
                    )
                    .clicked()
                {
                    self.state.output.format = OutputFormat::Markdown;
                }
            });

            ui.add_space(2.0);

            // Primary actions
            ui.horizontal(|ui| {
                let selected_count = self.tree.get_selected_files().len();
                let button_enabled = !self.state.output.generating
                    && self.state.root.is_some()
                    && selected_count > 0;

                // Generate button - Primary CTA
                let generate_button = egui::Button::new("🚀 Generate")
                    .min_size(egui::vec2(140.0, 32.0))
                    .fill(if button_enabled {
                        UiTheme::accent_color(Self::prefers_dark_theme())
                    } else {
                        egui::Color32::GRAY
                    });

                if ui
                    .add_enabled(button_enabled, generate_button)
                    .on_hover_text(if button_enabled {
                        "Generate output (Ctrl+G)"
                    } else if self.state.root.is_none() {
                        "Select a directory first"
                    } else if self.tree.get_selected_files().is_empty() {
                        "Select files to include"
                    } else {
                        "Generating..."
                    })
                    .clicked()
                {
                    self.generate_output();
                }

                // Secondary actions
                if self.state.output.content.is_some() {
                    ui.separator();

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
                }

                // Progress indicator
                if self.state.output.generating {
                    ui.separator();
                    ui.spinner();

                    if let Some((stage, progress)) = &self.current_progress {
                        let stage_text = match stage {
                            crate::workers::ProgressStage::ScanningFiles => "Scanning",
                            crate::workers::ProgressStage::ReadingFiles => "Reading",
                            crate::workers::ProgressStage::BuildingOutput => "Building",
                        };
                        ui.label(format!("{stage_text}: {:.0}%", progress.percentage()));
                    }

                    if ui.small_button("Cancel").clicked() {
                        let _ = self.worker.send_command(WorkerCommand::Cancel);
                    }
                }

                // Settings button (right-aligned)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙ Settings").clicked() {
                        self.state.config.ui.show_settings = !self.state.config.ui.show_settings;
                    }
                });
            });
        });
    }

    /// Shows the file tree and settings content  
    #[allow(clippy::too_many_lines)]
    pub fn show_files_content(&mut self, ui: &mut egui::Ui) {
        // Collapsible settings panel
        if self.state.config.ui.show_settings {
            egui::TopBottomPanel::top("settings_panel")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.add_space(UiTheme::SPACING_SM);
                    ui.collapsing("⚙ Advanced Settings", |ui| {
                        // Include tree checkbox
                        ui.checkbox(
                            &mut self.state.config.ui.include_tree,
                            "Include directory tree in output",
                        );

                        ui.separator();

                        // Ignore patterns in collapsible section
                        ui.collapsing("Ignore Patterns", |ui| {
                            // Compact pattern display
                            egui::ScrollArea::vertical()
                                .max_height(100.0)
                                .show(ui, |ui| {
                                    let mut patterns_to_remove = Vec::new();
                                    for (idx, pattern) in
                                        self.state.config.ignore_patterns.iter().enumerate()
                                    {
                                        ui.horizontal(|ui| {
                                            ui.label(pattern);
                                            if ui.small_button("×").clicked() {
                                                patterns_to_remove.push(idx);
                                            }
                                        });
                                    }
                                    for &idx in patterns_to_remove.iter().rev() {
                                        self.state.config.ignore_patterns.remove(idx);
                                    }
                                });

                            // Compact add pattern input
                            ui.horizontal(|ui| {
                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut self.new_pattern_input)
                                        .hint_text("Add pattern (e.g., *.log)")
                                        .desired_width(150.0),
                                );

                                let add_pattern = |app: &mut Self| {
                                    if !app.new_pattern_input.trim().is_empty() {
                                        app.state
                                            .config
                                            .ignore_patterns
                                            .push(app.new_pattern_input.trim().to_string());
                                        app.new_pattern_input.clear();
                                    }
                                };

                                if response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    add_pattern(self);
                                    response.request_focus();
                                }

                                if ui.small_button("+").clicked() {
                                    add_pattern(self);
                                }
                            });

                            // Preset patterns
                            ui.horizontal(|ui| {
                                ui.label("Presets:");
                                if ui.small_button("Common").clicked() {
                                    self.state.config.ignore_patterns = vec![
                                        ".*".to_string(),
                                        "node_modules".to_string(),
                                        "__pycache__".to_string(),
                                        "target".to_string(),
                                        "build".to_string(),
                                        "dist".to_string(),
                                    ];
                                    self.apply_patterns();
                                }
                                if ui.small_button("Minimal").clicked() {
                                    self.state.config.ignore_patterns =
                                        vec![".*".to_string(), "node_modules".to_string()];
                                    self.apply_patterns();
                                }
                                if ui.small_button("Clear").clicked() {
                                    self.state.config.ignore_patterns.clear();
                                    self.apply_patterns();
                                }
                            });
                        });
                    });
                    ui.add_space(UiTheme::SPACING_SM);
                });
        }

        // Main content area with file tree
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // Search bar at the top
            ui.horizontal(|ui| {
                ui.add_space(UiTheme::SPACING_SM);
                ui.label("🔍");
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.state.search.tree_search.query)
                        .desired_width(ui.available_width() - 80.0)
                        .hint_text("Search files... (Ctrl+F)"),
                );

                if !self.state.search.tree_search.query.is_empty() && ui.small_button("×").clicked() {
                    self.state.search.tree_search.query.clear();
                }

                // Keyboard shortcut
                if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::F)) {
                    response.request_focus();
                }
            });

            ui.separator();

            // File changes notification
            if self.files_changed {
                ui.horizontal(|ui| {
                    ui.add_space(UiTheme::SPACING_SM);
                    ui.colored_label(UiTheme::WARNING, "⚠ Files changed");
                    if ui.small_button("Refresh").clicked() {
                        if let Some(root) = &self.state.root {
                            self.tree.set_root(root.clone());
                            self.files_changed = false;
                            self.toast_manager.success("Directory refreshed");
                        }
                    }
                });
            }

            // Error display
            if let Some(error) = &self.error_message {
                ui.horizontal(|ui| {
                    ui.add_space(UiTheme::SPACING_SM);
                    ui.colored_label(UiTheme::ERROR, format!("⚠ {error}"));
                });
            }

            ui.add_space(UiTheme::SPACING_SM);
            // Track selection state before showing tree
            let snapshot_before = self.capture_snapshot();

            // The tree now has all remaining space
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.tree
                        .show_with_search(ui, &self.state.search.tree_search.query);
                });

            // Check if selection changed and record state
            let snapshot_after = self.capture_snapshot();
            if snapshot_before.selected_files != snapshot_after.selected_files {
                self.record_state();
                // Update real-time token count when selection changes
                self.state.output.estimated_tokens = Some(self.estimate_tokens_for_selection());
            }
        });
    }
    
    /// Renders the complete files panel (for tab/narrow view)
    pub fn show_files_panel(&mut self, ui: &mut egui::Ui) {
        // Fixed bottom action bar
        egui::TopBottomPanel::bottom("local_action_bar")
            .exact_height(80.0)
            .show_inside(ui, |ui| {
                self.show_action_bar(ui);
            });
        
        // File content takes remaining space
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.show_files_content(ui);
        });
    }

    /// Shows the output panel header with title and action buttons
    fn show_output_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Output Preview").heading());

            if let Some(token_count) = self.state.output.tokens {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    Self::show_token_info(ui, token_count);
                    ui.add_space(UiTheme::SPACING_MD);
                    self.show_output_actions(ui);
                });
            } else if self.state.output.content.is_some() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.show_output_actions(ui);
                });
            }
        });
    }

    /// Shows token count information with appropriate styling
    fn show_token_info(ui: &mut egui::Ui, token_count: crate::core::types::TokenCount) {
        let level = token_count.level();
        let (label, color) = match level {
            TokenLevel::Low => ("Low", UiTheme::SUCCESS),
            TokenLevel::Medium => ("Medium", UiTheme::WARNING),
            TokenLevel::High => ("High", UiTheme::ERROR),
        };

        ui.colored_label(color, format!("◆ {} tokens", token_count.get()));
        ui.colored_label(color, format!("[{label}]"));
    }

    /// Shows the output action buttons (save and copy)
    fn show_output_actions(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                self.state.output.content.is_some(),
                egui::Button::new("📋 Copy"),
            )
            .on_hover_text("Copy to clipboard (Ctrl+C)")
            .clicked()
        {
            self.copy_to_clipboard();
        }

        if ui
            .add_enabled(
                self.state.output.content.is_some(),
                egui::Button::new("💾 Save"),
            )
            .on_hover_text("Save to file (Ctrl+S)")
            .clicked()
        {
            self.save_to_file();
        }
    }

    /// Shows the search interface for output content
    fn show_output_search(&mut self, ui: &mut egui::Ui) {
        if self.state.search.output_search.active && self.state.output.content.is_some() {
            ui.horizontal(|ui| {
                ui.label("🔍 Find:");
                let response = ui.text_edit_singleline(&mut self.state.search.output_search.query);

                if response.changed() {
                    self.update_search_matches();
                }

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.state.search.output_search.active = false;
                    self.state.search.output_search.query.clear();
                }

                response.request_focus();

                // Show match count and navigation
                if !self.state.search.output_search.query.is_empty()
                    && self.state.search.output_search.match_count > 0
                {
                    ui.label(format!(
                        "{} / {}",
                        self.state.search.output_search.current_match + 1,
                        self.state.search.output_search.match_count
                    ));

                    if ui.small_button("↑").clicked() {
                        self.prev_match();
                    }

                    if ui.small_button("↓").clicked() {
                        self.next_match();
                    }
                } else if !self.state.search.output_search.query.is_empty() {
                    ui.label("No matches");
                }

                if ui.small_button("✕").clicked() {
                    self.state.search.output_search.active = false;
                    self.state.search.output_search.query.clear();
                }
            });
            ui.add_space(UiTheme::SPACING_MD);
        }
    }

    /// Renders the output panel UI
    pub fn show_output_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.add_space(UiTheme::SPACING_MD);
        ui.vertical(|ui| {
            self.show_output_header(ui);
            ui.separator();

            self.show_output_search(ui);

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if let Some(content) = &self.state.output.content {
                        // Use monospace font for code output
                        ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));
                        ui.add(
                            egui::TextEdit::multiline(&mut content.as_str())
                                .desired_width(f32::INFINITY)
                                .interactive(false),
                        );
                    } else {
                        ui.label("Generated output will appear here...");
                    }
                });
        });
    }

    /// Estimates token count for current selection
    pub fn estimate_tokens_for_selection(&self) -> usize {
        let selected_files = self.tree.get_selected_files();
        // Rough estimate: 1 token per 4 characters
        selected_files
            .iter()
            .filter_map(|path| std::fs::metadata(path).ok())
            .map(|metadata| (metadata.len() / 4) as usize)
            .sum()
    }

    /// Applies current ignore patterns to the tree
    fn apply_patterns(&mut self) {
        self.tree
            .set_ignore_patterns(&self.state.config.ignore_patterns.join(","));
        self.save_config();
        self.saved_ignore_patterns
            .clone_from(&self.state.config.ignore_patterns);
        if let Some(root) = &self.state.root {
            self.tree.set_root(root.clone());
        }
        self.toast_manager.success("Patterns applied");
    }
}

/// Formats token count with K/M suffixes
#[allow(clippy::cast_precision_loss)]
fn format_token_count(count: usize) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}
