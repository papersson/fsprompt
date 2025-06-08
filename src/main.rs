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
use core::types::Theme;
use ui::Theme as UiTheme;

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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        // Determine current theme mode for styling
        let _dark_mode = match self.state.config.ui.theme {
            Theme::Dark => true,
            Theme::Light => false,
            Theme::System => Self::prefers_dark_theme(),
        };

        // Top panel with title and directory selector
        egui::TopBottomPanel::top("top_panel")
            .exact_height(UiTheme::TOP_BAR_HEIGHT)
            .show(ctx, |ui| {
                ui.add_space(UiTheme::SPACING_SM);
                ui.horizontal(|ui| {
                    ui.heading("fsPrompt");
                    ui.add_space(UiTheme::SPACING_MD);

                    if ui.button("📁 Select Directory").clicked() {
                        self.handle_directory_selection();
                    }

                    if let Some(root) = &self.state.root {
                        ui.add_space(UiTheme::SPACING_SM);
                        ui.label(format!("📂 {}", root.as_path().display()));
                    }

                    // Theme toggle on the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.menu_button("🎨 Theme", |ui| {
                            if ui
                                .radio_value(&mut self.state.config.ui.theme, Theme::System, "Auto")
                                .clicked()
                            {
                                self.handle_theme_selection(ctx, Theme::System);
                            }
                            if ui
                                .radio_value(&mut self.state.config.ui.theme, Theme::Light, "Light")
                                .clicked()
                            {
                                self.handle_theme_selection(ctx, Theme::Light);
                            }
                            if ui
                                .radio_value(&mut self.state.config.ui.theme, Theme::Dark, "Dark")
                                .clicked()
                            {
                                self.handle_theme_selection(ctx, Theme::Dark);
                            }
                        });
                    });
                });
                ui.add_space(UiTheme::SPACING_SM);
            });

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
            let panel_response = egui::SidePanel::left("left_panel")
                .default_width(
                    self.state.config.window.left_pane_ratio * ctx.available_rect().width(),
                )
                .width_range(UiTheme::SIDEBAR_MIN_WIDTH..=UiTheme::SIDEBAR_MAX_WIDTH)
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_files_panel(ui);
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
