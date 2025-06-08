//! UI rendering logic for the main application

use crate::app::FsPromptApp;
use crate::ui::{
    components::{Button, ButtonSize, ButtonVariant},
    footer::Footer,
    icons::IconType,
    output_panel::OutputPanel,
    theme::TextEmphasis,
    Theme as UiTheme,
};
use eframe::egui;

impl FsPromptApp {
    /// Shows just the action bar (for global positioning) - single row layout
    pub fn show_action_bar(&mut self, ui: &mut egui::Ui) {
        let mut generate_requested = false;

        Footer::new(
            &mut self.state,
            &self.tree,
            &self.worker,
            &mut self.icon_manager,
            &self.current_progress,
        )
        .on_generate(|| generate_requested = true)
        .show(ui);

        if generate_requested {
            self.generate_output();
        }
    }

    /// Shows the file tree and settings content  
    #[allow(clippy::too_many_lines)]
    pub fn show_files_content(&mut self, ui: &mut egui::Ui) {
        // Show settings popover if active
        if self.state.config.ui.show_settings {
            self.show_settings_popover(ui.ctx());
        }

        // Main content area with file tree - no nested panels to avoid resizing issues
        ui.vertical(|ui| {
            // Show empty state if no directory selected
            if self.state.root.is_none() {
                self.show_empty_state(ui);
                return;
            }

            // Removed search bar - not worth the complexity

            // File changes notification
            if self.files_changed {
                ui.horizontal(|ui| {
                    ui.add_space(UiTheme::SPACING_SM);
                    ui.colored_label(UiTheme::WARNING, "⚠ Files changed");

                    let refresh_button = Button::new("Refresh")
                        .variant(ButtonVariant::Secondary)
                        .size(ButtonSize::Small)
                        .icon(IconType::Refresh)
                        .tooltip("Reload directory contents");

                    if refresh_button.show(ui, &mut self.icon_manager).clicked() {
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

            // Track selection state before showing tree
            let snapshot_before = self.capture_snapshot();

            // The tree now has all remaining space
            // Note: The tree component has its own ScrollArea, so we don't need another one here
            self.tree.show(ui, &mut self.icon_manager);

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
            .exact_height(44.0)
            .show_inside(ui, |ui| {
                self.show_action_bar(ui);
            });

        // File content takes remaining space
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.show_files_content(ui);
        });
    }

    // Removed show_token_info and show_output_actions as they were creating redundancy
    // All actions are now consolidated in the bottom action bar

    // Removed show_output_search method - search functionality removed

    /// Renders the output panel UI
    pub fn show_output_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        OutputPanel::new(self).show(ui, ctx);
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

    /// Shows the settings popover window
    fn show_settings_popover(&mut self, ctx: &egui::Context) {
        let mut show_settings = self.state.config.ui.show_settings;
        let dark_mode = ctx.style().visuals.dark_mode;
        let tokens = UiTheme::design_tokens(dark_mode);

        egui::Window::new("Settings")
            .id(egui::Id::new("settings_popover"))
            .open(&mut show_settings)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .default_width(400.0)
            .frame(
                egui::Frame::new()
                    .fill(tokens.colors.surface)
                    .shadow(tokens.shadows.lg)
                    .corner_radius(tokens.radius.lg)
                    .inner_margin(egui::Margin::same({
                        #[allow(clippy::cast_possible_truncation)]
                        {
                            tokens.spacing.lg as i8
                        }
                    })),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Include tree checkbox
                    ui.checkbox(
                        &mut self.state.config.ui.include_tree,
                        "Include directory tree in output",
                    );

                    ui.separator();

                    // Ignore patterns section
                    ui.label(egui::RichText::new("Ignore Patterns").heading());
                    ui.add_space(UiTheme::SPACING_SM);

                    // Pattern list
                    egui::ScrollArea::vertical()
                        .max_height(150.0)
                        .show(ui, |ui| {
                            let mut patterns_to_remove = Vec::new();
                            for (idx, pattern) in
                                self.state.config.ignore_patterns.iter().enumerate()
                            {
                                ui.horizontal(|ui| {
                                    ui.label(pattern);

                                    let remove_button = Button::icon_only(IconType::Close)
                                        .size(ButtonSize::Small)
                                        .tooltip("Remove pattern");

                                    if remove_button.show(ui, &mut self.icon_manager).clicked() {
                                        patterns_to_remove.push(idx);
                                    }
                                });
                            }
                            for &idx in patterns_to_remove.iter().rev() {
                                self.state.config.ignore_patterns.remove(idx);
                            }
                        });

                    ui.add_space(UiTheme::SPACING_SM);

                    // Add pattern input
                    ui.horizontal(|ui| {
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.new_pattern_input)
                                .hint_text("Add pattern (e.g., *.log)")
                                .desired_width(250.0),
                        );

                        let add_button = Button::new("+")
                            .variant(ButtonVariant::Primary)
                            .size(ButtonSize::Small)
                            .tooltip("Add pattern");

                        if (add_button.show(ui, &mut self.icon_manager).clicked()
                            || (response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                            && !self.new_pattern_input.trim().is_empty()
                        {
                            self.state
                                .config
                                .ignore_patterns
                                .push(self.new_pattern_input.trim().to_string());
                            self.new_pattern_input.clear();
                            self.apply_patterns();
                        }
                    });

                    ui.add_space(UiTheme::SPACING_SM);

                    // Preset buttons
                    ui.horizontal(|ui| {
                        ui.label("Presets:");

                        let common_button = Button::new("Common")
                            .variant(ButtonVariant::Secondary)
                            .size(ButtonSize::Small)
                            .tooltip("Common ignore patterns");

                        if common_button.show(ui, &mut self.icon_manager).clicked() {
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

                        let minimal_button = Button::new("Minimal")
                            .variant(ButtonVariant::Secondary)
                            .size(ButtonSize::Small)
                            .tooltip("Minimal ignore patterns");

                        if minimal_button.show(ui, &mut self.icon_manager).clicked() {
                            self.state.config.ignore_patterns =
                                vec![".*".to_string(), "node_modules".to_string()];
                            self.apply_patterns();
                        }

                        let clear_button = Button::new("Clear All")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Small)
                            .tooltip("Remove all patterns");

                        if clear_button.show(ui, &mut self.icon_manager).clicked() {
                            self.state.config.ignore_patterns.clear();
                            self.apply_patterns();
                        }
                    });
                });
            });

        // Update settings state based on window visibility
        self.state.config.ui.show_settings = show_settings;

        // Close on Escape key
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.state.config.ui.show_settings = false;
        }
    }

    /// Shows an empty state when no directory is selected
    fn show_empty_state(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(UiTheme::SPACING_XL * 2.0);

                // Large icon
                ui.label(
                    egui::RichText::new("📁")
                        .size(64.0)
                        .color(UiTheme::text_color(ui.visuals().dark_mode, TextEmphasis::Secondary))
                );

                ui.add_space(UiTheme::SPACING_LG);

                // Primary message
                ui.label(
                    egui::RichText::new("No Directory Selected")
                        .size(20.0)
                        .strong()
                        .color(UiTheme::text_color(ui.visuals().dark_mode, TextEmphasis::Primary))
                );

                ui.add_space(UiTheme::SPACING_SM);

                // Helper text
                ui.label(
                    egui::RichText::new("Choose a directory to get started with generating\nyour codebase prompt for LLMs.")
                        .size(14.0)
                        .color(UiTheme::text_color(ui.visuals().dark_mode, TextEmphasis::Secondary))
                );

                ui.add_space(UiTheme::SPACING_LG);

                // CTA button
                let select_button = Button::new("Select Directory")
                    .variant(ButtonVariant::Primary)
                    .size(ButtonSize::Large)
                    .icon(IconType::Folder)
                    .min_width(UiTheme::PRIMARY_BUTTON_MIN_WIDTH);

                if select_button.show(ui, &mut self.icon_manager).clicked() {
                    self.handle_directory_selection();
                }

                ui.add_space(UiTheme::SPACING_SM);

            });
        });
    }
}
