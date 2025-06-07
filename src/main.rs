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
}

impl FsPromptApp {
    /// Creates a new instance of the application
    #[must_use]
    const fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected_path: None,
        }
    }
}

impl eframe::App for FsPromptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("fsPrompt");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Select Directory").clicked() {
                    ui.label("Directory selection will be implemented next");
                }

                if let Some(path) = &self.selected_path {
                    ui.label(format!("Selected: {}", path.display()));
                }
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("File tree will appear here");
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
        };

        assert!(app.selected_path.is_none());
    }

    #[test]
    fn test_app_with_path() {
        let test_path = std::path::PathBuf::from("/test/path");
        let app = FsPromptApp {
            selected_path: Some(test_path.clone()),
        };

        assert_eq!(app.selected_path, Some(test_path));
    }

    #[test]
    fn test_app_debug_impl() {
        let app = FsPromptApp {
            selected_path: None,
        };

        // Test that Debug is implemented correctly
        let debug_str = format!("{:?}", app);
        assert!(debug_str.contains("FsPromptApp"));
        assert!(debug_str.contains("selected_path"));
    }
}
