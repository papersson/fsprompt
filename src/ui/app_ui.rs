//! UI rendering logic for the main application

use crate::app::FsPromptApp;
use crate::core::types::{OutputFormat, Theme, TokenLevel};
use crate::ui::{TextEmphasis, Theme as UiTheme};
use crate::workers::WorkerCommand;
use eframe::egui;

impl FsPromptApp {
    /// Renders the files panel UI
    pub fn show_files_panel(&mut self, ui: &mut egui::Ui) {
        // Use a scrollable area for controls to ensure everything is visible
        egui::TopBottomPanel::top("file_controls")
            .resizable(true)
            .default_height(350.0)
            .height_range(300.0..=500.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(UiTheme::SPACING_MD);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("Files & Directories").heading());
                            ui.add_space(UiTheme::SPACING_MD);

                            // Format selection
                            ui.horizontal(|ui| {
                                ui.label("Output format:");
                                ui.radio_value(
                                    &mut self.state.output.format,
                                    OutputFormat::Xml,
                                    "XML",
                                );
                                ui.radio_value(
                                    &mut self.state.output.format,
                                    OutputFormat::Markdown,
                                    "Markdown",
                                );
                            });

                            // Include tree checkbox
                            ui.checkbox(
                                &mut self.state.config.ui.include_tree,
                                "Include directory tree in output",
                            );

                            // Ignore patterns
                            ui.vertical(|ui| {
                                ui.label("Ignore patterns:");

                                // Pattern list with remove buttons
                                let mut patterns_to_remove = Vec::new();
                                ui.group(|ui| {
                                    ui.set_width(ui.available_width());

                                    if self.state.config.ignore_patterns.is_empty() {
                                        // Determine dark mode
                                        let dark_mode = match self.state.config.ui.theme {
                                            Theme::Dark => true,
                                            Theme::Light => false,
                                            Theme::System => Self::prefers_dark_theme(),
                                        };
                                        ui.colored_label(
                                            UiTheme::text_color(dark_mode, TextEmphasis::Secondary),
                                            "No patterns configured",
                                        );
                                    } else {
                                        for (idx, pattern) in
                                            self.state.config.ignore_patterns.iter().enumerate()
                                        {
                                            ui.horizontal(|ui| {
                                                ui.label(pattern);
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(
                                                        egui::Align::Center,
                                                    ),
                                                    |ui| {
                                                        if ui.small_button("✕").clicked() {
                                                            patterns_to_remove.push(idx);
                                                        }
                                                    },
                                                );
                                            });
                                        }
                                    }
                                });

                                // Remove patterns that were marked for deletion
                                for &idx in patterns_to_remove.iter().rev() {
                                    self.state.config.ignore_patterns.remove(idx);
                                }

                                // Add new pattern input
                                ui.add_space(UiTheme::SPACING_SM);
                                ui.horizontal(|ui| {
                                    ui.label("Add pattern:");
                                    let response = ui
                                        .text_edit_singleline(&mut self.new_pattern_input)
                                        .on_hover_text("Enter a pattern (e.g., *.log, temp/*, _*)");

                                    // Add pattern on Enter key
                                    if response.lost_focus()
                                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                        && !self.new_pattern_input.trim().is_empty()
                                    {
                                        self.state
                                            .config
                                            .ignore_patterns
                                            .push(self.new_pattern_input.trim().to_string());
                                        self.new_pattern_input.clear();
                                        response.request_focus();
                                    }

                                    if ui.button("Add").clicked()
                                        && !self.new_pattern_input.trim().is_empty()
                                    {
                                        self.state
                                            .config
                                            .ignore_patterns
                                            .push(self.new_pattern_input.trim().to_string());
                                        self.new_pattern_input.clear();
                                    }
                                });

                                // Action buttons
                                ui.add_space(UiTheme::SPACING_SM);
                                ui.horizontal(|ui| {
                                    // Track if patterns have been modified
                                    let patterns_modified = self.state.config.ignore_patterns
                                        != self.saved_ignore_patterns;

                                    // Reset button
                                    if ui.button("Reset to Defaults").clicked() {
                                        self.state.config.ignore_patterns = vec![
                                            ".*".to_string(),
                                            "node_modules".to_string(),
                                            "__pycache__".to_string(),
                                            "target".to_string(),
                                            "build".to_string(),
                                            "dist".to_string(),
                                            "_*".to_string(),
                                        ];
                                        self.toast_manager.info("Reset to default ignore patterns");
                                    }

                                    // Save button - only enabled if patterns have been modified
                                    ui.add_enabled_ui(patterns_modified, |ui| {
                                        if ui.button("Save").clicked() {
                                            // Update the tree with new patterns
                                            self.tree.set_ignore_patterns(
                                                &self.state.config.ignore_patterns.join(","),
                                            );

                                            // Save configuration
                                            self.save_config();

                                            // Update saved patterns to match current
                                            self.saved_ignore_patterns
                                                .clone_from(&self.state.config.ignore_patterns);

                                            self.toast_manager.success("Ignore patterns saved");

                                            // If we have a root directory, refresh the tree
                                            if let Some(root) = &self.state.root {
                                                self.tree.set_root(root.clone());
                                            }
                                        }
                                    });

                                    // Visual indicator if patterns have been modified
                                    if patterns_modified {
                                        ui.colored_label(UiTheme::WARNING, "⚠ Unsaved changes");
                                    }
                                });
                            });

                            ui.add_space(UiTheme::SPACING_MD);

                            // Search bar with modern styling
                            ui.horizontal(|ui| {
                                ui.label("🔍");
                                ui.spacing_mut().text_edit_width = ui.available_width() - 60.0;
                                let response = ui
                                    .add(
                                        egui::TextEdit::singleline(
                                            &mut self.state.search.tree_search.query,
                                        )
                                        .desired_width(f32::INFINITY)
                                        .hint_text("Search files..."),
                                    )
                                    .on_hover_text("Search for files and folders");

                                // Clear button
                                if !self.state.search.tree_search.query.is_empty()
                                    && ui.small_button("✕").clicked()
                                {
                                    self.state.search.tree_search.query.clear();
                                }

                                // Focus on Ctrl+F
                                if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::F)) {
                                    response.request_focus();
                                }
                            });

                            ui.add_space(UiTheme::SPACING_MD);

                            // Show refresh notification if files have changed
                            if self.files_changed {
                                ui.horizontal(|ui| {
                                    ui.colored_label(
                                        UiTheme::WARNING,
                                        "⚠️ Files have changed since last generation",
                                    );
                                    if ui.small_button("Refresh").clicked() {
                                        // Reload the tree
                                        if let Some(root) = &self.state.root {
                                            self.tree.set_root(root.clone());
                                            self.files_changed = false;
                                            self.toast_manager.success("Directory refreshed");
                                        }
                                    }
                                });
                                ui.add_space(UiTheme::SPACING_MD);
                            }

                            // Generate button - make it prominent
                            ui.add_space(UiTheme::SPACING_SM);
                            ui.horizontal_centered(|ui| {
                                let button_enabled =
                                    !self.state.output.generating && self.state.root.is_some();
                                let generate_button = egui::Button::new("🚀 Generate")
                                    .min_size(egui::vec2(120.0, UiTheme::BUTTON_HEIGHT));

                                if ui
                                    .add_enabled(button_enabled, generate_button)
                                    .on_hover_text("Generate output (Ctrl+G)")
                                    .clicked()
                                {
                                    self.generate_output();
                                }

                                if self.state.output.generating {
                                    ui.spinner();

                                    if let Some((stage, progress)) = &self.current_progress {
                                        let stage_text = match stage {
                                            crate::workers::ProgressStage::ScanningFiles => {
                                                "Scanning files"
                                            }
                                            crate::workers::ProgressStage::ReadingFiles => {
                                                "Reading files"
                                            }
                                            crate::workers::ProgressStage::BuildingOutput => {
                                                "Building output"
                                            }
                                        };
                                        ui.label(format!(
                                            "{}: {}/{} ({:.0}%)",
                                            stage_text,
                                            progress.current(),
                                            progress.total(),
                                            progress.percentage()
                                        ));
                                    } else {
                                        ui.label("Starting...");
                                    }

                                    if ui.button("Cancel").clicked() {
                                        let _ = self.worker.send_command(WorkerCommand::Cancel);
                                    }
                                } else if self.state.root.is_none() {
                                    ui.label("Select a directory first");
                                } else {
                                    ui.label("Select files to include");
                                }
                            });

                            ui.add_space(UiTheme::SPACING_MD);

                            // Show error message if any
                            if let Some(error) = &self.error_message {
                                ui.colored_label(UiTheme::ERROR, format!("⚠️ {error}"));
                                ui.add_space(UiTheme::SPACING_MD);
                            }
                        });
                    });
            });

        // Now use CentralPanel for the tree - this guarantees it gets remaining space
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // Track selection state before showing tree
            let snapshot_before = self.capture_snapshot();

            // The tree now has all remaining space
            self.tree
                .show_with_search(ui, &self.state.search.tree_search.query);

            // Check if selection changed and record state
            let snapshot_after = self.capture_snapshot();
            if snapshot_before.selected_files != snapshot_after.selected_files {
                self.record_state();
            }
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
}
