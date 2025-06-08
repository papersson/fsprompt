use crate::{
    core::types::{AppState, OutputFormat, ProgressCount},
    ui::{
        components::{Button, ButtonSize, ButtonVariant, SegmentedControl},
        icons::{IconManager, IconType},
        theme::Theme as UiTheme,
        tree::DirectoryTree,
    },
    workers::{ProgressStage, WorkerCommand, WorkerHandle},
};
use eframe::egui;

/// Footer/action bar component for the application
pub struct Footer<'a> {
    state: &'a mut AppState,
    tree: &'a DirectoryTree,
    worker: &'a WorkerHandle,
    icon_manager: &'a mut IconManager,
    current_progress: &'a Option<(ProgressStage, ProgressCount)>,
    on_generate: Option<Box<dyn FnOnce() + 'a>>,
}

impl<'a> Footer<'a> {
    pub fn new(
        state: &'a mut AppState,
        tree: &'a DirectoryTree,
        worker: &'a WorkerHandle,
        icon_manager: &'a mut IconManager,
        current_progress: &'a Option<(ProgressStage, ProgressCount)>,
    ) -> Self {
        Self {
            state,
            tree,
            worker,
            icon_manager,
            current_progress,
            on_generate: None,
        }
    }

    /// Sets the callback to run when generate is clicked
    pub fn on_generate(mut self, callback: impl FnOnce() + 'a) -> Self {
        self.on_generate = Some(Box::new(callback));
        self
    }

    /// Shows the footer/action bar
    pub fn show(mut self, ui: &mut egui::Ui) {
        let tokens = &UiTheme::design_tokens(ui.visuals().dark_mode);

        // Main container with consistent height - no extra frame needed since we're already in a panel
        ui.horizontal_centered(|ui| {
            ui.add_space(tokens.spacing.lg);

            // Left side: info and format
            self.show_left_side(ui, &tokens);

            // Right side: actions (push to the right)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(tokens.spacing.lg);
                self.show_right_side(ui, &tokens);
            });
        });
    }

    fn show_left_side(&mut self, ui: &mut egui::Ui, tokens: &crate::ui::theme::DesignTokens) {
        let selected_count = self.tree.get_selected_files().len();
        let token_estimate = self.estimate_tokens_for_selection();

        // Use horizontal layout to ensure all items are vertically centered
        ui.horizontal(|ui| {
            // Selection info with consistent height
            egui::Frame::new()
                .fill(tokens.colors.surface_container)
                .inner_margin(egui::Margin::symmetric(
                    tokens.spacing.sm as i8,
                    4, // Reduced vertical padding
                ))
                .corner_radius(tokens.radius.sm)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("{selected_count} files"))
                            .color(tokens.colors.on_surface_variant),
                    );
                });

            ui.add_space(tokens.spacing.sm);

            // Token count with enhanced visual indicator
            let (token_color, token_bg, token_label) = if token_estimate < 10_000 {
                (
                    tokens.colors.success,
                    tokens.colors.success_container,
                    "Low",
                )
            } else if token_estimate < 50_000 {
                (
                    tokens.colors.warning,
                    tokens.colors.warning_container,
                    "Medium",
                )
            } else {
                (tokens.colors.error, tokens.colors.error_container, "High")
            };

            egui::Frame::new()
                .fill(token_bg.gamma_multiply(0.3))
                .inner_margin(egui::Margin::symmetric(
                    tokens.spacing.sm as i8,
                    4, // Match the file count padding
                ))
                .corner_radius(tokens.radius.sm)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "~{} tokens ({})",
                            format_token_count(token_estimate),
                            token_label
                        ))
                        .color(token_color),
                    );
                });

            ui.separator();

            // Output format toggle - ensure proper vertical alignment
            ui.label(egui::RichText::new("Format:").color(tokens.colors.on_surface_variant));

            // Create a custom segmented control with better alignment
            let format_control = SegmentedControl::new(self.state.output.format)
                .option(OutputFormat::Xml, "XML", None)
                .option(OutputFormat::Markdown, "MD", None)
                .size(ButtonSize::Small);

            if let Some(new_format) = format_control.show(ui, self.icon_manager) {
                self.state.output.format = new_format;
            }
        });
    }

    fn show_right_side(&mut self, ui: &mut egui::Ui, _tokens: &crate::ui::theme::DesignTokens) {
        // Settings button (rightmost)
        let settings_button = Button::icon_only(IconType::Settings)
            .size(ButtonSize::Medium)
            .tooltip("Settings");

        if settings_button.show(ui, self.icon_manager).clicked() {
            self.state.config.ui.show_settings = !self.state.config.ui.show_settings;
        }

        // Progress indicator during generation
        if self.state.output.generating {
            let cancel_button = Button::new("Cancel")
                .variant(ButtonVariant::Secondary)
                .size(ButtonSize::Medium)
                .tooltip("Cancel generation");

            if cancel_button.show(ui, self.icon_manager).clicked() {
                let _ = self.worker.send_command(WorkerCommand::Cancel);
            }

            if let Some((stage, progress)) = &self.current_progress {
                let stage_text = match stage {
                    ProgressStage::ScanningFiles => "Scanning",
                    ProgressStage::ReadingFiles => "Reading",
                    ProgressStage::BuildingOutput => "Building",
                };
                ui.label(format!("{stage_text}: {:.0}%", progress.percentage()));
            }

            ui.spinner();
        }

        // Generate button (primary action)
        let selected_count = self.tree.get_selected_files().len();
        let button_enabled =
            !self.state.output.generating && self.state.root.is_some() && selected_count > 0;

        let tooltip_text = if button_enabled {
            "Generate output (Ctrl+G)"
        } else if self.state.root.is_none() {
            "Select a directory first"
        } else if self.tree.get_selected_files().is_empty() {
            "Select files to include"
        } else {
            "Generating..."
        };

        let generate_button = Button::new("Generate")
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Medium)
            .icon(IconType::Generate)
            .min_width(120.0)
            .loading(self.state.output.generating)
            .disabled(!button_enabled)
            .tooltip(tooltip_text);

        if generate_button.show(ui, self.icon_manager).clicked() && button_enabled {
            if let Some(callback) = self.on_generate.take() {
                callback();
            }
        }
    }

    fn estimate_tokens_for_selection(&self) -> usize {
        self.state.output.estimated_tokens.unwrap_or_else(|| {
            let selected_files = self.tree.get_selected_files();
            // Rough estimate: 1 token per 4 characters
            selected_files
                .iter()
                .filter_map(|path| std::fs::metadata(path).ok())
                .map(|metadata| (metadata.len() / 4) as usize)
                .sum()
        })
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
