use crate::{
    app::FsPromptApp,
    ui::{
        components::{Button, ButtonSize},
        icons::IconType,
        theme::Theme as UiTheme,
    },
};
use eframe::egui;

/// Output panel component that displays the generated output
pub struct OutputPanel<'a> {
    app: &'a mut FsPromptApp,
}

impl<'a> OutputPanel<'a> {
    pub fn new(app: &'a mut FsPromptApp) -> Self {
        Self { app }
    }

    /// Shows the complete output panel
    pub fn show(mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        let tokens = UiTheme::design_tokens(ui.visuals().dark_mode);

        // Add proper padding and elevation
        egui::Frame::new()
            .inner_margin(egui::Margin::same(tokens.spacing.lg as i8))
            .show(ui, |ui| {
                // Show header
                self.show_header(ui);

                ui.add_space(tokens.spacing.sm);

                // Subtle separator
                ui.painter().hline(
                    ui.available_rect_before_wrap().x_range(),
                    ui.cursor().min.y,
                    egui::Stroke::new(1.0, tokens.colors.outline_variant),
                );
                ui.add_space(tokens.spacing.md);

                // Show content
                self.show_content(ui);
            });
    }

    /// Shows the output panel header with title and action buttons
    fn show_header(&mut self, ui: &mut egui::Ui) {
        let tokens = UiTheme::design_tokens(ui.visuals().dark_mode);

        ui.horizontal(|ui| {
            // Ensure vertical center alignment for all elements
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Output Preview")
                        .heading()
                        .color(tokens.colors.on_surface),
                );
            });

            // Push action buttons to the right
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Only show Copy/Save buttons when output exists
                if self.app.state.output.content.is_some() {
                    let save_button = Button::icon_only(IconType::Save)
                        .size(ButtonSize::Medium)
                        .tooltip("Save to file (Ctrl+S)");

                    if save_button.show(ui, &mut self.app.icon_manager).clicked() {
                        self.app.save_to_file();
                    }

                    let copy_button = Button::icon_only(IconType::Copy)
                        .size(ButtonSize::Medium)
                        .tooltip("Copy to clipboard (Ctrl+C)");

                    if copy_button.show(ui, &mut self.app.icon_manager).clicked() {
                        self.app.copy_to_clipboard();
                    }
                }
            });
        });
    }

    /// Shows the output content area
    fn show_content(&self, ui: &mut egui::Ui) {
        let tokens = UiTheme::design_tokens(ui.visuals().dark_mode);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(content) = &self.app.state.output.content {
                    // Enhanced code display with better styling
                    egui::Frame::new()
                        .fill(tokens.colors.surface_variant)
                        .inner_margin(egui::Margin::same(tokens.spacing.md as i8))
                        .corner_radius(tokens.radius.md)
                        .show(ui, |ui| {
                            // Use monospace font for code output
                            ui.style_mut().override_font_id = Some(egui::FontId::monospace(13.0));
                            ui.add(
                                egui::TextEdit::multiline(&mut content.as_str())
                                    .desired_width(f32::INFINITY)
                                    .interactive(false)
                                    .font(egui::TextStyle::Monospace),
                            );
                        });
                } else {
                    // Better empty state
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() / 3.0);
                            ui.label(
                                egui::RichText::new("📄")
                                    .size(48.0)
                                    .color(tokens.colors.on_surface_variant.gamma_multiply(0.5)),
                            );
                            ui.add_space(tokens.spacing.lg);
                            ui.label(
                                egui::RichText::new("No output generated yet")
                                    .size(16.0)
                                    .color(tokens.colors.on_surface_variant),
                            );
                            ui.add_space(tokens.spacing.sm);
                            ui.label(
                                egui::RichText::new(
                                    "Select files and click Generate to create output",
                                )
                                .size(14.0)
                                .color(tokens.colors.on_surface_variant.gamma_multiply(0.7)),
                            );
                        });
                    });
                }
            });
    }
}
