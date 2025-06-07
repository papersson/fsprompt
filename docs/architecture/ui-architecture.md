# UI Architecture: Immediate Mode with egui

## Philosophy: Reactive Immediate Mode

fsPrompt uses **egui**, an immediate mode GUI framework that rebuilds the UI every frame based on application state. This approach provides excellent developer experience and naturally reactive interfaces.

## UI Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    egui Context                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Input       │  │ Layout      │  │ Rendering           │  │
│  │ Handling    │  │ Engine      │  │ Backend             │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                 Application UI Layer                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Top Panel   │  │ Side Panel  │  │ Central Panel       │  │
│  │ (Header)    │  │ (File Tree) │  │ (Output View)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Component System                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Directory   │  │ Toast       │  │ Performance         │  │
│  │ Tree        │  │ Manager     │  │ Overlay             │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Application State                       │
│  All UI renders from single source of truth                │
└─────────────────────────────────────────────────────────────┘
```

## Panel-Based Layout System

### Responsive Design Strategy

fsPrompt adapts to different screen sizes using conditional layouts:

```rust
impl eframe::App for FsPromptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let window_width = ctx.available_rect().width();
        let is_narrow = window_width < 800.0;

        if is_narrow {
            // Mobile/narrow layout: Tabs
            self.show_tabbed_layout(ctx);
        } else {
            // Desktop layout: Side-by-side panels
            self.show_panel_layout(ctx);
        }
    }
}
```

### Panel Layout (Desktop)

```rust
fn show_panel_layout(&mut self, ctx: &egui::Context) {
    // Fixed top panel
    egui::TopBottomPanel::top("top_panel")
        .exact_height(UiTheme::TOP_BAR_HEIGHT)
        .show(ctx, |ui| {
            self.show_header_ui(ui);
        });

    // Resizable left panel
    let panel_response = egui::SidePanel::left("left_panel")
        .default_width(self.state.config.window.left_pane_ratio * ctx.available_rect().width())
        .width_range(UiTheme::SIDEBAR_MIN_WIDTH..=UiTheme::SIDEBAR_MAX_WIDTH)
        .resizable(true)
        .show(ctx, |ui| {
            self.show_files_panel(ui);
        });

    // Update configuration when panel is resized
    let new_ratio = panel_response.response.rect.width() / ctx.available_rect().width();
    if (new_ratio - self.state.config.window.left_pane_ratio).abs() > 0.01 {
        self.state.config.window.left_pane_ratio = new_ratio;
    }

    // Central panel takes remaining space
    egui::CentralPanel::default().show(ctx, |ui| {
        self.show_output_panel(ui, ctx);
    });
}
```

### Tabbed Layout (Mobile)

```rust
fn show_tabbed_layout(&mut self, ctx: &egui::Context) {
    // Tab bar
    egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_tab, TabView::Files, "📁 Files");
            ui.selectable_value(&mut self.active_tab, TabView::Output, "📄 Output");
        });
    });

    // Content based on active tab
    egui::CentralPanel::default().show(ctx, |ui| {
        match self.active_tab {
            TabView::Files => self.show_files_panel(ui),
            TabView::Output => self.show_output_panel(ui, ctx),
        }
    });
}
```

## Component Architecture

### 1. Directory Tree Component

The directory tree is a custom widget that manages hierarchical file display:

```rust
pub struct DirectoryTree {
    root: Option<CanonicalPath>,
    expanded: HashSet<CanonicalPath>,
    selected: HashSet<CanonicalPath>,
    entries: Vec<FsEntry>,
}

impl DirectoryTree {
    pub fn show_with_search(&mut self, ui: &mut egui::Ui, search_query: &str) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(root) = &self.root {
                    self.show_tree_recursive(ui, root, 0, search_query);
                }
            });
    }

    fn show_tree_recursive(&mut self, ui: &mut egui::Ui, path: &CanonicalPath, depth: usize, search: &str) {
        for entry in &self.entries {
            if self.matches_search(entry, search) {
                self.show_tree_node(ui, entry, depth);
            }
        }
    }

    fn show_tree_node(&mut self, ui: &mut egui::Ui, entry: &FsEntry, depth: usize) {
        let indent = depth as f32 * UiTheme::TREE_INDENT;
        
        ui.horizontal(|ui| {
            ui.add_space(indent);
            
            // Expand/collapse button for directories
            if entry.is_dir() {
                let expanded = self.expanded.contains(&entry.path);
                if ui.selectable_label(expanded, if expanded { "▼" } else { "▶" }).clicked() {
                    self.toggle_expanded(&entry.path);
                }
            } else {
                ui.add_space(20.0); // Align with expand buttons
            }
            
            // Selection checkbox
            let mut selected = self.selected.contains(&entry.path);
            if ui.checkbox(&mut selected, "").changed() {
                self.toggle_selection(&entry.path, selected);
            }
            
            // File/folder icon and name
            let icon = if entry.is_dir() { "📁" } else { "📄" };
            ui.label(format!("{} {}", icon, entry.name));
            
            // File size for files
            if let Some(size) = entry.file_size() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(self.format_file_size(size));
                });
            }
        });
    }
}
```

**Key Features**:
- **Lazy loading**: Only loads visible directories
- **Search filtering**: Real-time search with highlighting
- **Selection state**: Hierarchical selection with indeterminate states
- **Performance**: Virtualized rendering for large directories

### 2. Toast Notification System

Type-safe notifications with automatic dismissal:

```rust
pub struct ToastManager {
    toasts: Vec<ActiveToast>,
}

struct ActiveToast {
    toast: Toast,
    created_at: Instant,
    id: u64,
}

impl ToastManager {
    pub fn show_ui(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        
        // Remove expired toasts
        self.toasts.retain(|toast| {
            now.duration_since(toast.created_at).as_secs() < toast.toast.duration_secs as u64
        });
        
        // Show active toasts
        for (index, active_toast) in self.toasts.iter().enumerate() {
            self.show_toast(ctx, &active_toast.toast, index);
        }
    }

    fn show_toast(&self, ctx: &egui::Context, toast: &Toast, index: usize) {
        let screen_rect = ctx.screen_rect();
        let toast_height = 60.0;
        let margin = 10.0;
        
        let pos = egui::pos2(
            screen_rect.max.x - 300.0 - margin,
            margin + index as f32 * (toast_height + margin),
        );

        egui::Area::new(egui::Id::new(format!("toast_{}", index)))
            .fixed_pos(pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let (bg_color, text_color) = match &toast.variant {
                    ToastVariant::Success(_) => (UiTheme::SUCCESS_BG, UiTheme::SUCCESS_TEXT),
                    ToastVariant::Warning(_) => (UiTheme::WARNING_BG, UiTheme::WARNING_TEXT),
                    ToastVariant::Error { .. } => (UiTheme::ERROR_BG, UiTheme::ERROR_TEXT),
                    ToastVariant::Progress { .. } => (UiTheme::INFO_BG, UiTheme::INFO_TEXT),
                };

                let frame = egui::Frame::popup(ui.style())
                    .fill(bg_color)
                    .stroke(egui::Stroke::new(1.0, text_color));

                frame.show(ui, |ui| {
                    ui.set_min_width(280.0);
                    match &toast.variant {
                        ToastVariant::Success(msg) => {
                            ui.colored_label(text_color, format!("✓ {}", msg));
                        }
                        ToastVariant::Error { message, details } => {
                            ui.colored_label(text_color, format!("✗ {}", message));
                            if let Some(details) = details {
                                ui.colored_label(text_color.gamma_multiply(0.8), details);
                            }
                        }
                        ToastVariant::Progress { message, percentage } => {
                            ui.colored_label(text_color, message);
                            ui.add(egui::ProgressBar::new(*percentage / 100.0));
                        }
                        ToastVariant::Warning(msg) => {
                            ui.colored_label(text_color, format!("⚠ {}", msg));
                        }
                    }
                });
            });
    }
}
```

### 3. Performance Overlay Component

Development-time performance monitoring:

```rust
impl PerfOverlay {
    pub fn show(&self, ctx: &egui::Context) {
        if !self.show { return; }

        let stats = self.frame_timer.stats();
        
        egui::Area::new(egui::Id::new("perf_overlay"))
            .fixed_pos(egui::pos2(
                ctx.screen_rect().max.x - 220.0,
                ctx.screen_rect().max.y - 160.0,
            ))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let frame = egui::Frame::window(ui.style())
                    .fill(egui::Color32::from_rgba_premultiplied(30, 30, 30, 220));

                frame.show(ui, |ui| {
                    ui.label("Performance (Dev)");
                    ui.separator();

                    // Color-coded FPS
                    let fps_color = match stats.avg_fps {
                        fps if fps >= 120.0 => egui::Color32::GREEN,
                        fps if fps >= 60.0 => egui::Color32::YELLOW,
                        _ => egui::Color32::RED,
                    };
                    ui.colored_label(fps_color, format!("FPS: {:.0}", stats.avg_fps));
                    
                    // Frame timing percentiles
                    ui.label(format!("P50: {:.1}ms", stats.p50_ms));
                    ui.label(format!("P95: {:.1}ms", stats.p95_ms));
                    ui.label(format!("Max: {:.1}ms", stats.max_ms));
                    
                    // Memory usage
                    let mem_growth = self.memory_tracker.growth_mb();
                    ui.label(format!("Memory: +{:.1}MB", mem_growth));
                });
            });
    }
}
```

## Theme System

### Centralized Visual Design

```rust
pub struct UiTheme;

impl UiTheme {
    // Layout constants
    pub const TOP_BAR_HEIGHT: f32 = 50.0;
    pub const SIDEBAR_MIN_WIDTH: f32 = 250.0;
    pub const SIDEBAR_MAX_WIDTH: f32 = 600.0;
    pub const BUTTON_HEIGHT: f32 = 32.0;
    
    // Spacing system
    pub const SPACING_SM: f32 = 4.0;
    pub const SPACING_MD: f32 = 8.0;
    pub const SPACING_LG: f32 = 16.0;
    
    // Color palette
    pub const SUCCESS: egui::Color32 = egui::Color32::from_rgb(34, 197, 94);
    pub const WARNING: egui::Color32 = egui::Color32::from_rgb(251, 191, 36);
    pub const ERROR: egui::Color32 = egui::Color32::from_rgb(239, 68, 68);
    
    pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
        let mut style = if dark_mode {
            egui::Style {
                visuals: egui::Visuals::dark(),
                ..Default::default()
            }
        } else {
            egui::Style {
                visuals: egui::Visuals::light(),
                ..Default::default()
            }
        };
        
        // Custom styling
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
        style.spacing.item_spacing = egui::vec2(8.0, 4.0);
        
        ctx.set_style(style);
    }
}
```

## Input Handling

### Global Keyboard Shortcuts

```rust
impl FsPromptApp {
    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Global shortcuts
            if i.modifiers.ctrl {
                if i.key_pressed(egui::Key::G) {
                    self.generate_output();
                }
                if i.key_pressed(egui::Key::S) && self.state.output.content.is_some() {
                    self.save_to_file();
                }
                if i.key_pressed(egui::Key::C) && self.state.output.content.is_some() {
                    self.copy_to_clipboard();
                }
                if i.key_pressed(egui::Key::F) {
                    self.state.search.output_search.active = true;
                }
                if i.key_pressed(egui::Key::Z) {
                    self.undo();
                }
                if i.key_pressed(egui::Key::Y) || (i.modifiers.shift && i.key_pressed(egui::Key::Z)) {
                    self.redo();
                }
            }
            
            // Escape key handling
            if i.key_pressed(egui::Key::Escape) {
                self.state.search.output_search.active = false;
                self.state.search.output_search.query.clear();
            }
        });
    }
}
```

## State-UI Synchronization

### Reactive Updates

The UI automatically reflects state changes through egui's retained mode:

```rust
impl FsPromptApp {
    pub fn show_files_panel(&mut self, ui: &mut egui::Ui) {
        // Selection state snapshot for undo/redo
        let snapshot_before = self.capture_snapshot();
        
        // Show tree (may modify selection)
        self.tree.show_with_search(ui, &self.state.search.tree_search.query);
        
        // Check if selection changed
        let snapshot_after = self.capture_snapshot();
        if snapshot_before.selected_files != snapshot_after.selected_files {
            self.record_state(); // Add to undo history
        }
        
        // UI automatically reflects new state
    }
}
```

### Progress Visualization

Real-time progress updates from worker threads:

```rust
if self.state.output.generating {
    ui.spinner();
    
    if let Some((stage, progress)) = &self.current_progress {
        let stage_text = match stage {
            ProgressStage::ScanningFiles => "Scanning files",
            ProgressStage::ReadingFiles => "Reading files", 
            ProgressStage::BuildingOutput => "Building output",
        };
        
        ui.label(format!(
            "{}: {}/{} ({:.0}%)",
            stage_text,
            progress.current(),
            progress.total(),
            progress.percentage()
        ));
        
        // Progress bar
        ui.add(egui::ProgressBar::new(progress.percentage() / 100.0));
    }
}
```

## Performance Considerations

### Frame Budget Management

```rust
impl FsPromptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Record frame start for performance monitoring
        self.perf_overlay.frame_start();
        
        // Performance budget for heavy operations
        perf_budget!("worker_events", 2, {
            self.process_worker_events(ctx);
        });
        
        perf_budget!("fs_changes", 1, {
            self.check_fs_changes(ctx);
        });
        
        // UI rendering (should be fast)
        self.show_ui(ctx);
    }
}
```

### Efficient Rendering

- **Lazy evaluation**: Only compute UI elements when visible
- **Minimal redraws**: egui's retained mode minimizes work
- **Clipping**: Scroll areas only render visible content
- **Caching**: Expensive calculations cached between frames

## Accessibility & UX

### Visual Feedback
- **Loading states**: Spinners and progress bars
- **Color coding**: Semantic colors for different states
- **Icons**: Intuitive visual representations
- **Hover states**: Interactive feedback

### Keyboard Navigation
- **Tab order**: Logical focus progression
- **Shortcuts**: Common operations accessible via keyboard
- **Search**: Quick file finding with Ctrl+F
- **Escape**: Consistent cancellation behavior

This UI architecture provides a responsive, performant, and maintainable user interface that scales from mobile to desktop screen sizes.