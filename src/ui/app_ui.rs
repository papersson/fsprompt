//! UI rendering logic for the main application

use crate::app::FsPromptApp;
use crate::core::types::*;
use crate::ui::Theme as UiTheme;
use crate::workers::WorkerCommand;
use eframe::egui;

impl FsPromptApp {
    /// Renders the files panel UI
    pub fn show_files_panel(&mut self, ui: &mut egui::Ui) {
        ui.add_space(UiTheme::SPACING_MD);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Files & Directories").heading());
            ui.add_space(UiTheme::SPACING_MD);

            // Format selection
            ui.horizontal(|ui| {
                ui.label("Output format:");
                ui.radio_value(&mut self.state.output.format, OutputFormat::Xml, "XML");
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
                ui.label("Ignore patterns (comma-separated):");
                let mut patterns_str = self.state.config.ignore_patterns.join(", ");
                if ui
                    .text_edit_singleline(&mut patterns_str)
                    .on_hover_text("e.g., .*, node_modules, __pycache__, target, _*")
                    .changed()
                {
                    self.state.config.ignore_patterns = patterns_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
            });

            ui.add_space(UiTheme::SPACING_MD);

            // Search bar with modern styling
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.spacing_mut().text_edit_width = ui.available_width() - 60.0;
                let response = ui
                    .add(
                        egui::TextEdit::singleline(&mut self.state.search.tree_search.query)
                            .desired_width(f32::INFINITY)
                            .hint_text("Search files..."),
                    )
                    .on_hover_text("Search for files and folders");

                // Clear button
                if !self.state.search.tree_search.query.is_empty() && ui.small_button("✕").clicked()
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
                let button_enabled = !self.state.output.generating && self.state.root.is_some();
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
                            crate::workers::ProgressStage::ScanningFiles => "Scanning files",
                            crate::workers::ProgressStage::ReadingFiles => "Reading files",
                            crate::workers::ProgressStage::BuildingOutput => "Building output",
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
                ui.colored_label(UiTheme::ERROR, format!("⚠️ {}", error));
                ui.add_space(UiTheme::SPACING_MD);
            }

            // Track selection state before showing tree
            let snapshot_before = self.capture_snapshot();

            self.tree
                .show_with_search(ui, &self.state.search.tree_search.query);

            // Check if selection changed and record state
            let snapshot_after = self.capture_snapshot();
            if snapshot_before.selected_files != snapshot_after.selected_files {
                self.record_state();
            }
        });
    }

    /// Renders the output panel UI
    pub fn show_output_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.add_space(UiTheme::SPACING_MD);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Output Preview").heading());

                if let Some(token_count) = self.state.output.tokens {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let level = token_count.level();
                        let (label, color) = match level {
                            TokenLevel::Low => ("Low", UiTheme::SUCCESS),
                            TokenLevel::Medium => ("Medium", UiTheme::WARNING),
                            TokenLevel::High => ("High", UiTheme::ERROR),
                        };

                        ui.colored_label(color, format!("◆ {} tokens", token_count.get()));
                        ui.colored_label(color, format!("[{}]", label));

                        ui.add_space(UiTheme::SPACING_MD);

                        // Add save button
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

                        // Add copy button
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
                    });
                } else if self.state.output.content.is_some() {
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
            if self.state.search.output_search.active && self.state.output.content.is_some() {
                ui.horizontal(|ui| {
                    ui.label("🔍 Find:");
                    let response =
                        ui.text_edit_singleline(&mut self.state.search.output_search.query);

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
