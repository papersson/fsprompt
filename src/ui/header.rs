use crate::{
    core::types::AppState,
    ui::{
        components::{Button, ButtonSize, ButtonVariant},
        icons::{IconManager, IconType},
        logo::Logo,
        theme::{Elevation, Theme as UiTheme},
    },
};
use eframe::egui;

/// App header component
pub struct AppHeader<'a> {
    state: &'a mut AppState,
    icon_manager: &'a mut IconManager,
    on_select_directory: Option<Box<dyn FnOnce() + 'a>>,
}

impl<'a> AppHeader<'a> {
    pub fn new(state: &'a mut AppState, icon_manager: &'a mut IconManager) -> Self {
        Self {
            state,
            icon_manager,
            on_select_directory: None,
        }
    }

    /// Sets the callback to run when select directory is clicked
    pub fn on_select_directory(mut self, callback: impl FnOnce() + 'a) -> Self {
        self.on_select_directory = Some(Box::new(callback));
        self
    }

    /// Shows the app header
    pub fn show(mut self, ctx: &egui::Context) {
        let dark_mode = ctx.style().visuals.dark_mode;
        let tokens = UiTheme::design_tokens(dark_mode);

        egui::TopBottomPanel::top("top_panel")
            .exact_height(UiTheme::TOP_BAR_HEIGHT)
            .frame(
                egui::Frame::new()
                    .fill(tokens.colors.surface)
                    .inner_margin(egui::Margin::symmetric(tokens.spacing.lg as i8, 0))
                    .shadow(Elevation::Level2.shadow(dark_mode)),
            )
            .show(ctx, |ui| {
                // Use horizontal_centered to ensure all items are vertically centered
                ui.horizontal_centered(|ui| {
                    // Show logo with brand name
                    Logo::new()
                        .size(28.0)
                        .show_text(true)
                        .animate_on_hover(true)
                        .show_animated(ui, None);

                    ui.add_space(tokens.spacing.lg);

                    // Show directory controls only when directory is selected
                    if self.state.root.is_some() {
                        // Show Select Directory button with proper width
                        let select_button = Button::new("Select Directory")
                            .variant(ButtonVariant::Primary)
                            .size(ButtonSize::Medium)
                            .icon(IconType::Folder)
                            .min_width(150.0); // Make button wide enough for text

                        if select_button.show(ui, self.icon_manager).clicked() {
                            if let Some(callback) = self.on_select_directory.take() {
                                callback();
                            }
                        }

                        // Show directory path to the right of the button
                        if let Some(root) = &self.state.root {
                            ui.add_space(tokens.spacing.md);

                            let path_str = root.as_path().display().to_string();
                            let max_width = ui.available_width() - 150.0; // Reserve space for theme button

                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.set_max_width(max_width);
                                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                    ui.add(egui::Label::new(format!("📂 {path_str}")).truncate())
                                        .on_hover_text(&path_str);
                                },
                            );
                        }
                    }
                });
            });
    }
}
