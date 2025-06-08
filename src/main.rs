#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms,
    missing_debug_implementations
)]
#![allow(clippy::module_name_repetitions)] // Common in Rust APIs
#![allow(clippy::must_use_candidate)] // We'll add these selectively
#![allow(clippy::multiple_crate_versions)] // Transitive dependency conflicts we don't control

//! fsPrompt - A high-performance filesystem prompt generator for LLMs
//!
//! This application allows users to generate context prompts from codebases,
//! producing XML or Markdown output containing directory structure and file contents.

use eframe::egui;

pub mod app;
pub mod core;
pub mod handlers;
pub mod state;
pub mod ui;
pub mod utils;
pub mod watcher;
/// Worker thread management for background tasks
pub mod workers;

use app::{FsPromptApp, TabView};
use ui::{
    components::{Button, ButtonSize, ButtonVariant},
    header::AppHeader,
    icons::IconType,
    logo::Logo,
    Theme as UiTheme,
};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "fsPrompt",
        native_options,
        Box::new(|cc| Ok(Box::new(FsPromptApp::new(cc)))),
    )
}

impl eframe::App for FsPromptApp {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // FIRST: Apply theme before ANY UI rendering
        self.apply_theme_if_needed(ctx);

        // Record frame start for performance monitoring
        self.perf_overlay.frame_start();

        // Process worker events
        self.process_worker_events(ctx);

        // Check for filesystem changes
        self.check_fs_changes(ctx);

        // Check if window is narrow for responsive design
        let window_width = ctx.available_rect().width();
        let is_narrow = window_width < 800.0;

        // Global keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);

        // Show app header
        let mut directory_selected = false;

        AppHeader::new(&mut self.state, &mut self.icon_manager)
            .on_select_directory(|| directory_selected = true)
            .show(ctx);

        if directory_selected {
            self.handle_directory_selection();
        }

        // Show welcome screen if no directory is selected
        if self.state.root.is_none() {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.show_welcome_screen(ui);
            });

            // Show toast notifications and performance overlay
            self.toast_manager.show_ui(ctx);
            self.perf_overlay.show(ctx);
            return;
        }

        // Responsive UI: Use tabs for narrow windows, side-by-side for wide windows
        if is_narrow {
            // Tab bar for narrow windows
            egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.active_tab, TabView::Files, "📁 Files");
                    ui.selectable_value(&mut self.active_tab, TabView::Output, "📄 Output");
                });
            });

            // Show content based on active tab
            egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
                TabView::Files => self.show_files_panel(ui),
                TabView::Output => self.show_output_panel(ui, ctx),
            });
        } else {
            // Normal side-by-side layout for wide windows
            // First, create the action bar at the bottom
            egui::TopBottomPanel::bottom("global_action_bar")
                .exact_height(44.0)
                .show(ctx, |ui| {
                    self.show_action_bar(ui);
                });

            // Then create the side panels
            let panel_response = egui::SidePanel::left("left_panel")
                .default_width(
                    self.state.config.window.left_pane_ratio * ctx.available_rect().width(),
                )
                .width_range(UiTheme::SIDEBAR_MIN_WIDTH..=UiTheme::SIDEBAR_MAX_WIDTH)
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_files_content(ui);
                });

            // Update panel width ratio if resized
            let panel_rect = panel_response.response.rect;
            let new_ratio = panel_rect.width() / ctx.available_rect().width();
            if (new_ratio - self.state.config.window.left_pane_ratio).abs() > 0.01 {
                self.state.config.window.left_pane_ratio = new_ratio;
            }

            // Right panel with output
            egui::CentralPanel::default().show(ctx, |ui| {
                self.show_output_panel(ui, ctx);
            });
        }

        // Show toast notifications
        self.toast_manager.show_ui(ctx);

        // Show performance overlay
        self.perf_overlay.show(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.on_exit();
    }
}

impl FsPromptApp {
    /// Shows the welcome screen when no directory is selected
    fn show_welcome_screen(&mut self, ui: &mut egui::Ui) {
        let tokens = UiTheme::design_tokens(ui.visuals().dark_mode);

        // Center content vertically and horizontally
        ui.vertical_centered(|ui| {
            // Add fixed vertical centering space
            ui.add_space(100.0);

            // Large centered logo
            Logo::new()
                .size(120.0)
                .show_text(false)
                .animate_on_hover(true)
                .show_animated(ui, None);

            ui.add_space(tokens.spacing.xl);

            // Welcome title
            ui.label(
                egui::RichText::new("Welcome to fsPrompt")
                    .size(tokens.typography.headline_large.size)
                    .color(tokens.colors.on_surface),
            );

            ui.add_space(tokens.spacing.md);

            // Simple subtitle
            ui.label(
                egui::RichText::new("Generate LLM-ready prompts from your codebase")
                    .size(tokens.typography.body_large.size)
                    .color(tokens.colors.on_surface_variant),
            );

            ui.add_space(tokens.spacing.xxxl);

            // Clean, properly sized button
            let start_button = Button::new("Select Directory")
                .variant(ButtonVariant::Primary)
                .size(ButtonSize::Large)
                .icon(IconType::Folder)
                .min_width(220.0);

            if start_button
                .show_animated(
                    ui,
                    &mut self.icon_manager,
                    Some(&mut self.animation_manager),
                )
                .clicked()
            {
                self.handle_directory_selection();
            }
        });
    }
}
