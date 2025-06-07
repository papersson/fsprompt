# UI Components API Reference

Complete reference for reusable UI components in fsPrompt's interface system (`src/ui/`).

## Overview

fsPrompt's UI system is built on egui and provides a set of reusable, type-safe components for building consistent interfaces. The components follow a design system with standardized spacing, colors, and interaction patterns.

## Theme System

### `Theme`

Central theme configuration with constants for colors, spacing, and dimensions.

```rust
pub struct Theme;
```

#### Color Constants

```rust
impl Theme {
    // Dark theme colors
    pub const DARK_BG_PRIMARY: Color32 = Color32::from_rgb(26, 26, 26);
    pub const DARK_BG_SECONDARY: Color32 = Color32::from_rgb(35, 35, 35);
    pub const DARK_BG_TERTIARY: Color32 = Color32::from_rgb(45, 45, 45);
    pub const DARK_BORDER: Color32 = Color32::from_rgb(58, 58, 58);
    pub const DARK_TEXT_PRIMARY: Color32 = Color32::from_rgb(236, 236, 236);
    pub const DARK_TEXT_SECONDARY: Color32 = Color32::from_rgb(155, 155, 155);
    pub const DARK_ACCENT: Color32 = Color32::from_rgb(30, 144, 255);

    // Light theme colors
    pub const LIGHT_BG_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);
    pub const LIGHT_BG_SECONDARY: Color32 = Color32::from_rgb(247, 247, 247);
    pub const LIGHT_BG_TERTIARY: Color32 = Color32::from_rgb(235, 235, 235);
    pub const LIGHT_BORDER: Color32 = Color32::from_rgb(227, 227, 227);
    pub const LIGHT_TEXT_PRIMARY: Color32 = Color32::from_rgb(32, 32, 32);
    pub const LIGHT_TEXT_SECONDARY: Color32 = Color32::from_rgb(110, 110, 110);
    pub const LIGHT_ACCENT: Color32 = Color32::from_rgb(0, 102, 204);

    // Status colors
    pub const SUCCESS: Color32 = Color32::from_rgb(16, 185, 129);
    pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11);
    pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);
    pub const INFO: Color32 = Color32::from_rgb(59, 130, 246);
}
```

#### Spacing Constants

```rust
impl Theme {
    pub const SPACING_XS: f32 = 4.0;   // Extra small spacing
    pub const SPACING_SM: f32 = 8.0;   // Small spacing
    pub const SPACING_MD: f32 = 16.0;  // Medium spacing
    pub const SPACING_LG: f32 = 24.0;  // Large spacing
    pub const SPACING_XL: f32 = 32.0;  // Extra large spacing

    pub const RADIUS_SM: f32 = 4.0;    // Small corner radius
    pub const RADIUS_MD: f32 = 6.0;    // Medium corner radius
    pub const RADIUS_LG: f32 = 8.0;    // Large corner radius
}
```

#### Layout Constants

```rust
impl Theme {
    pub const TOP_BAR_HEIGHT: f32 = 48.0;
    pub const BOTTOM_BAR_HEIGHT: f32 = 56.0;
    pub const SIDEBAR_DEFAULT_WIDTH: f32 = 280.0;
    pub const SIDEBAR_MIN_WIDTH: f32 = 200.0;
    pub const SIDEBAR_MAX_WIDTH: f32 = 400.0;

    pub const BUTTON_HEIGHT: f32 = 36.0;
    pub const BUTTON_PADDING_H: f32 = 16.0;
    pub const BUTTON_PADDING_V: f32 = 8.0;

    pub const ROW_HEIGHT: f32 = 28.0;
    pub const INDENT_SIZE: f32 = 20.0;
    pub const ICON_SIZE: f32 = 16.0;
    pub const ICON_SPACING: f32 = 4.0;
}
```

#### Methods

```rust
impl Theme {
    /// Apply theme to egui context
    pub fn apply_theme(ctx: &egui::Context, dark_mode: bool)
    
    /// Get background color for given theme and level
    pub fn bg_color(dark_mode: bool, level: BgLevel) -> Color32
    
    /// Get text color for given theme and emphasis
    pub fn text_color(dark_mode: bool, emphasis: TextEmphasis) -> Color32
    
    /// Get accent color for given theme
    pub fn accent_color(dark_mode: bool) -> Color32
    
    /// Get border color for given theme
    pub fn border_color(dark_mode: bool) -> Color32
}
```

#### Usage Example

```rust
// Apply theme to egui context
Theme::apply_theme(&ctx, dark_mode);

// Use themed colors
let bg_color = Theme::bg_color(dark_mode, BgLevel::Secondary);
let text_color = Theme::text_color(dark_mode, TextEmphasis::Primary);

// Use spacing constants
ui.add_space(Theme::SPACING_MD);
```

### `BgLevel`

Background color levels for consistent hierarchy.

```rust
pub enum BgLevel {
    Primary,   // Primary background
    Secondary, // Secondary background
    Tertiary,  // Tertiary background
}
```

### `TextEmphasis`

Text emphasis levels for typography hierarchy.

```rust
pub enum TextEmphasis {
    Primary,   // Primary text
    Secondary, // Secondary text
}
```

## Toast Notification System

### `ToastManager`

Manages toast notifications with automatic dismissal.

```rust
pub struct ToastManager {
    current_toast: Option<Toast>,
}
```

#### Methods

```rust
impl ToastManager {
    /// Creates a new toast manager
    pub fn new() -> Self
    
    /// Shows a new toast notification
    pub fn show(&mut self, toast: Toast)
    
    /// Shows a success toast
    pub fn success(&mut self, message: impl Into<String>)
    
    /// Shows a warning toast
    pub fn warning(&mut self, message: impl Into<String>)
    
    /// Shows an error toast
    pub fn error(&mut self, message: impl Into<String>)
    
    /// Shows an info toast (uses warning variant)
    pub fn info(&mut self, message: impl Into<String>)
    
    /// Updates the toast state (removes expired toasts)
    pub fn update(&mut self)
    
    /// Renders the toast UI
    pub fn show_ui(&mut self, ctx: &egui::Context)
}
```

#### Usage Example

```rust
use crate::ui::toast::ToastManager;

struct App {
    toast_manager: ToastManager,
}

impl App {
    fn new() -> Self {
        Self {
            toast_manager: ToastManager::new(),
        }
    }
    
    fn update(&mut self, ctx: &egui::Context) {
        // Show toasts (call this every frame)
        self.toast_manager.show_ui(ctx);
        
        // Example notifications
        if copy_successful {
            self.toast_manager.success("Output copied to clipboard");
        }
        
        if generation_failed {
            self.toast_manager.error("Failed to generate output");
        }
        
        if files_ignored {
            self.toast_manager.warning("Some files were ignored");
        }
    }
}
```

### `Toast`

Individual toast notification with variant-specific styling.

```rust
pub struct Toast {
    pub message: String,
    pub variant: ToastVariant,
    pub created_at: Instant,
}
```

#### Methods

```rust
impl Toast {
    /// Creates a new success toast
    pub fn success(message: impl Into<String>) -> Self
    
    /// Creates a new warning toast
    pub fn warning(message: impl Into<String>) -> Self
    
    /// Creates a new error toast
    pub fn error(message: impl Into<String>) -> Self
    
    /// Checks if the toast should be dismissed
    pub fn should_dismiss(&self) -> bool
    
    /// Gets the remaining time as a fraction (0.0 to 1.0)
    pub fn remaining_fraction(&self) -> f32
}
```

### `ToastVariant`

Toast variants with automatic duration and styling.

```rust
pub enum ToastVariant {
    Success, // Success message (green, 2 seconds)
    Warning, // Warning message (yellow, 3 seconds)
    Error,   // Error message (red, 4 seconds)
}
```

#### Methods

```rust
impl ToastVariant {
    /// Gets the color for this variant
    fn color(&self) -> egui::Color32
    
    /// Gets the icon for this variant
    fn icon(&self) -> &'static str
    
    /// Gets the auto-dismiss duration
    fn dismiss_duration(&self) -> Duration
}
```

## Directory Tree Component

### `DirectoryTree`

High-performance directory tree widget with lazy loading, filtering, and tri-state selection.

```rust
pub struct DirectoryTree {
    pub roots: Vec<TreeNode>,
    // ... private fields
}
```

#### Methods

```rust
impl DirectoryTree {
    /// Creates a new empty directory tree
    pub fn new() -> Self
    
    /// Sets the root directory for the tree
    pub fn set_root(&mut self, path: CanonicalPath)
    
    /// Updates the ignore patterns from a comma-separated string
    pub fn set_ignore_patterns(&mut self, patterns_str: &str)
    
    /// Renders the tree UI
    pub fn show(&mut self, ui: &mut egui::Ui)
    
    /// Renders the tree UI with search filtering
    pub fn show_with_search(&mut self, ui: &mut egui::Ui, search_query: &str)
    
    /// Collects all selected file paths recursively
    pub fn collect_selected_files(&self) -> Vec<CanonicalPath>
    
    /// Generates a string representation of the entire directory tree
    pub fn generate_tree_string(&self) -> String
    
    /// Gets all selected file paths as a set
    pub fn get_selected_files(&self) -> HashSet<String>
    
    /// Gets all expanded directory paths as a set
    pub fn get_expanded_dirs(&self) -> HashSet<String>
    
    /// Restores selection and expansion state
    pub fn restore_selection(&mut self, selected_files: &HashSet<String>, expanded_dirs: &HashSet<String>)
}
```

#### Usage Example

```rust
use crate::ui::tree::DirectoryTree;
use crate::core::types::CanonicalPath;

struct App {
    tree: DirectoryTree,
    search_query: String,
}

impl App {
    fn show_file_browser(&mut self, ui: &mut egui::Ui) {
        // Search bar
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
        });
        
        // Directory tree with search
        ui.separator();
        self.tree.show_with_search(ui, &self.search_query);
        
        // Selection info
        let selected_files = self.tree.collect_selected_files();
        ui.label(format!("Selected: {} files", selected_files.len()));
    }
    
    fn set_root_directory(&mut self, path: &str) -> Result<(), std::io::Error> {
        let canonical_path = CanonicalPath::new(path)?;
        self.tree.set_root(canonical_path);
        Ok(())
    }
    
    fn apply_ignore_patterns(&mut self, patterns: &str) {
        self.tree.set_ignore_patterns(patterns);
    }
}
```

### `TreeNode`

Individual node in the directory tree with lazy loading.

```rust
pub struct TreeNode {
    pub canonical_path: CanonicalPath,
    pub name: String,
    pub is_dir: bool,
    pub selection: SelectionState,
    pub expanded: bool,
    pub children_loaded: bool,
    pub children: Vec<TreeNode>,
    pub file_size: Option<FileSize>,
}
```

#### Methods

```rust
impl TreeNode {
    /// Creates a new tree node from a CanonicalPath
    pub fn new(canonical_path: CanonicalPath) -> std::io::Result<Self>
    
    /// Loads children for this node if it's a directory
    pub fn load_children(&mut self)
    
    /// Loads children for this node with ignore patterns
    pub fn load_children_with_patterns(&mut self, ignore_patterns: &[glob::Pattern])
    
    /// Loads all children recursively up to a maximum depth
    pub fn load_children_recursive(&mut self, current_depth: usize, max_depth: usize)
    
    /// Loads all children recursively up to a maximum depth with ignore patterns
    pub fn load_children_recursive_with_patterns(&mut self, current_depth: usize, max_depth: usize, ignore_patterns: &[glob::Pattern])
    
    /// Updates selection state recursively
    pub fn set_selection(&mut self, state: SelectionState)
    
    /// Updates selection state recursively with ignore patterns
    pub fn set_selection_with_patterns(&mut self, state: SelectionState, patterns: &[Pattern])
    
    /// Updates parent selection based on children
    pub fn update_parent_selection(&mut self)
    
    /// Debug helper to print tree structure with selection states
    pub fn debug_tree(&self, depth: usize) -> String
}
```

## Tree Generation Utilities

### `generate_tree_string`

Generates a visual tree representation of a directory structure.

```rust
pub fn generate_tree_string(root_path: &Path) -> String
```

#### Usage Example

```rust
use crate::ui::generate_tree_string;

let tree_output = generate_tree_string(Path::new("/project/root"));
println!("{}", tree_output);
// Output:
// 📁 root/
// ├── 📁 src/
// │   ├── 📄 main.rs
// │   └── 📄 lib.rs
// └── 📄 Cargo.toml
```

## Component Integration Patterns

### Standard Layout Pattern

```rust
fn show_main_ui(&mut self, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Apply consistent spacing
        ui.spacing_mut().item_spacing = egui::vec2(Theme::SPACING_SM, Theme::SPACING_SM);
        
        // Main content area
        ui.vertical(|ui| {
            // Header section
            ui.horizontal(|ui| {
                ui.heading("fsPrompt");
                ui.add_space(Theme::SPACING_MD);
                
                // Status indicators
                if self.is_generating {
                    ui.spinner();
                    ui.label("Generating...");
                }
            });
            
            ui.separator();
            
            // Content area with proper spacing
            ui.add_space(Theme::SPACING_SM);
            
            // Split panel layout
            let available_rect = ui.available_rect_before_wrap();
            let split_position = available_rect.width() * self.config.window.left_pane_ratio;
            
            ui.horizontal(|ui| {
                // Left panel (file tree)
                ui.allocate_ui_with_layout(
                    egui::vec2(split_position, available_rect.height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        self.show_file_tree(ui);
                    },
                );
                
                ui.separator();
                
                // Right panel (output)
                ui.allocate_ui_with_layout(
                    egui::vec2(available_rect.width() - split_position - Theme::SPACING_SM, available_rect.height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        self.show_output_area(ui);
                    },
                );
            });
        });
    });
    
    // Show toasts on top
    self.toast_manager.show_ui(ctx);
}
```

### Button Style Pattern

```rust
fn show_action_buttons(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        // Primary action button
        let generate_button = egui::Button::new("Generate Output")
            .min_size(egui::vec2(120.0, Theme::BUTTON_HEIGHT));
            
        if ui.add_enabled(!self.is_generating, generate_button).clicked() {
            self.start_generation();
        }
        
        ui.add_space(Theme::SPACING_SM);
        
        // Secondary action button
        let cancel_button = egui::Button::new("Cancel")
            .min_size(egui::vec2(80.0, Theme::BUTTON_HEIGHT));
            
        if ui.add_enabled(self.is_generating, cancel_button).clicked() {
            self.cancel_generation();
        }
        
        ui.add_space(Theme::SPACING_MD);
        
        // Utility buttons
        if ui.button("Copy").clicked() {
            self.copy_to_clipboard();
        }
        
        if ui.button("Save").clicked() {
            self.save_to_file();
        }
    });
}
```

### Progress Display Pattern

```rust
fn show_progress(&self, ui: &mut egui::Ui) {
    if let Some(progress) = &self.progress {
        ui.horizontal(|ui| {
            // Progress bar
            let progress_bar = egui::ProgressBar::new(progress.percentage() / 100.0)
                .desired_width(200.0)
                .text(format!("{}/{}", progress.current(), progress.total()));
            ui.add(progress_bar);
            
            ui.add_space(Theme::SPACING_SM);
            
            // Stage indicator
            if let Some(stage) = &self.current_stage {
                let stage_text = match stage {
                    ProgressStage::ScanningFiles => "Scanning files...",
                    ProgressStage::ReadingFiles => "Reading file contents...", 
                    ProgressStage::BuildingOutput => "Building output...",
                };
                ui.label(stage_text);
            }
        });
    }
}
```

### Error Display Pattern

```rust
fn show_status_area(&self, ui: &mut egui::Ui) {
    // Status messages with appropriate colors
    if let Some(error_msg) = &self.last_error {
        ui.horizontal(|ui| {
            ui.colored_label(Theme::ERROR, "✕");
            ui.colored_label(Theme::ERROR, error_msg);
        });
    } else if let Some(success_msg) = &self.last_success {
        ui.horizontal(|ui| {
            ui.colored_label(Theme::SUCCESS, "✓");
            ui.colored_label(Theme::text_color(self.dark_mode, TextEmphasis::Primary), success_msg);
        });
    }
    
    // Token count display with level-based coloring
    if let Some(token_count) = &self.token_count {
        let (color, level_text) = match token_count.level() {
            TokenLevel::Low => (Theme::SUCCESS, "Low"),
            TokenLevel::Medium => (Theme::WARNING, "Medium"),
            TokenLevel::High => (Theme::ERROR, "High"),
        };
        
        ui.horizontal(|ui| {
            ui.label("Tokens:");
            ui.colored_label(color, format!("{} ({})", token_count.get(), level_text));
        });
    }
}
```

## Accessibility and UX Patterns

### Keyboard Navigation

```rust
// Handle keyboard shortcuts
if ctx.input(|i| i.key_pressed(egui::Key::Escape)) && self.is_generating {
    self.cancel_generation();
}

if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::C)) {
    self.copy_to_clipboard();
}

if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
    self.save_to_file();
}
```

### Loading States

```rust
fn show_content_area(&mut self, ui: &mut egui::Ui) {
    if self.is_generating {
        // Show loading state
        ui.vertical_centered(|ui| {
            ui.add_space(Theme::SPACING_XL);
            ui.spinner();
            ui.add_space(Theme::SPACING_MD);
            ui.label("Generating output...");
            
            if let Some(progress) = &self.progress {
                ui.add_space(Theme::SPACING_SM);
                self.show_progress(ui);
            }
        });
    } else if let Some(content) = &self.output_content {
        // Show generated content
        self.show_output_text(ui, content);
    } else {
        // Show empty state
        ui.vertical_centered(|ui| {
            ui.add_space(Theme::SPACING_XL);
            ui.label("Select files and click 'Generate Output' to begin");
        });
    }
}
```

### Responsive Design

```rust
fn show_responsive_layout(&mut self, ui: &mut egui::Ui) {
    let available_width = ui.available_width();
    
    if available_width < 600.0 {
        // Mobile/narrow layout - stack vertically
        ui.vertical(|ui| {
            self.show_file_tree(ui);
            ui.separator();
            self.show_output_area(ui);
        });
    } else {
        // Desktop layout - side by side
        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(available_width * 0.4, ui.available_height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| self.show_file_tree(ui),
            );
            
            ui.separator();
            
            ui.allocate_ui_with_layout(
                egui::vec2(available_width * 0.6 - Theme::SPACING_SM, ui.available_height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| self.show_output_area(ui),
            );
        });
    }
}
```