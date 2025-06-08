//! Event handlers for keyboard shortcuts and directory selection

use crate::app::FsPromptApp;
use crate::core::types::CanonicalPath;
use eframe::egui;

impl FsPromptApp {
    /// Handles global keyboard shortcuts
    pub fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Ctrl+F for output search (only when output is available and not in tree search)
            if i.modifiers.ctrl
                && i.key_pressed(egui::Key::F)
                && self.state.output.content.is_some()
                && !i.focused
            {
                self.state.search.output_search.active = true;
            }

            // Ctrl+G for Generate (when not generating and path is selected)
            if i.modifiers.ctrl
                && i.key_pressed(egui::Key::G)
                && !self.state.output.generating
                && self.state.root.is_some()
                && !self.tree.get_selected_files().is_empty()
            {
                self.generate_output();
            }

            // Ctrl+C for Copy (when output is available)
            if i.modifiers.ctrl
                && i.key_pressed(egui::Key::C)
                && self.state.output.content.is_some()
            {
                self.copy_to_clipboard();
            }

            // Ctrl+S for Save (when output is available)
            if i.modifiers.ctrl
                && i.key_pressed(egui::Key::S)
                && self.state.output.content.is_some()
            {
                self.save_to_file();
            }

            // Ctrl+Z for Undo
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::Z) {
                self.undo();
            }

            // Ctrl+Shift+Z for Redo
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::Z) {
                self.redo();
            }

            // Ctrl+Shift+P for Performance Overlay
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::P) {
                self.perf_overlay.toggle();
            }

            // Ctrl+A for Select All (when in file tree context)
            if i.modifiers.ctrl && i.key_pressed(egui::Key::A) && self.state.root.is_some() {
                self.tree.select_all();
                self.state.output.estimated_tokens = Some(self.estimate_tokens_for_selection());
                self.record_state();
            }

            // Ctrl+D for Deselect All
            if i.modifiers.ctrl && i.key_pressed(egui::Key::D) && self.state.root.is_some() {
                self.tree.deselect_all();
                self.state.output.estimated_tokens = Some(0);
                self.record_state();
            }

            // Ctrl+Comma for Settings toggle
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Comma) {
                self.state.config.ui.show_settings = !self.state.config.ui.show_settings;
            }
        });
    }

    /// Handles directory selection dialog
    pub fn handle_directory_selection(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            if let Ok(canonical_path) = CanonicalPath::new(&path) {
                self.state.root = Some(canonical_path.clone());
                self.tree
                    .set_ignore_patterns(&self.state.config.ignore_patterns.join(","));
                self.tree.set_root(canonical_path.clone());

                // Start watching the directory
                if let Err(e) = self.fs_watcher.watch(&canonical_path) {
                    self.toast_manager
                        .warning(format!("Failed to watch directory: {e}"));
                }

                self.files_changed = false;
                self.toast_manager.success(format!(
                    "Loaded {}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
        }
    }
}
