# State Management API Reference

Complete reference for AppState and history management in fsPrompt (`src/state/`).

## Overview

fsPrompt uses a centralized state management system that separates data from UI state, provides undo/redo functionality, and handles configuration persistence. The state system is designed for type safety, immutability where possible, and clear separation of concerns.

## Core State Types

### `AppState`

Main application state with clear separation of concerns.

```rust
pub struct AppState {
    pub root: Option<CanonicalPath>,
    pub expanded: HashSet<CanonicalPath>,
    pub selections: SelectionTracker,
    pub search: SearchState,
    pub output: OutputState,
    pub config: AppConfig,
}
```

#### Usage Example

```rust
use crate::core::types::AppState;

// Initialize application state
let mut app_state = AppState::default();

// Set root directory
app_state.root = Some(CanonicalPath::new("/project/root")?);

// Track expanded directories
app_state.expanded.insert(root_path.clone());

// Access selection state
let selected_files = app_state.selections.selected.clone();

// Update configuration
app_state.config.ui.show_hidden = true;
```

### `SelectionTracker`

Tracks file selections with built-in undo/redo support.

```rust
pub struct SelectionTracker {
    pub selected: HashSet<CanonicalPath>,
    pub undo_stack: Vec<HashSet<CanonicalPath>>,
    pub redo_stack: Vec<HashSet<CanonicalPath>>,
}
```

#### Methods

```rust
impl SelectionTracker {
    pub const MAX_HISTORY: usize = 20;
    
    /// Records current state for undo
    pub fn checkpoint(&mut self)
}
```

#### Usage Example

```rust
// Before making selection changes, create a checkpoint
app_state.selections.checkpoint();

// Make selection changes
app_state.selections.selected.insert(file_path);
app_state.selections.selected.remove(&other_path);

// Undo is now available for these changes
```

### `SearchState`

Manages search functionality with separate tree and output search.

```rust
pub struct SearchState {
    pub tree_search: TreeSearch,
    pub output_search: OutputSearch,
}
```

#### Tree Search

```rust
pub struct TreeSearch {
    pub query: String,
    pub results: HashSet<CanonicalPath>,
    pub active: bool,
}
```

#### Output Search

```rust
pub struct OutputSearch {
    pub query: String,
    pub match_count: usize,
    pub current_match: usize,
    pub active: bool,
}
```

##### Methods

```rust
impl OutputSearch {
    /// Move to next match
    pub fn next_match(&mut self)
    
    /// Move to previous match
    pub fn prev_match(&mut self)
}
```

#### Usage Example

```rust
// Tree search
app_state.search.tree_search.query = "main.rs".to_string();
app_state.search.tree_search.active = true;

// Output search with navigation
app_state.search.output_search.query = "function".to_string();
app_state.search.output_search.match_count = 5;
app_state.search.output_search.next_match(); // Navigate matches
```

### `OutputState`

Manages generated output and generation status.

```rust
pub struct OutputState {
    pub format: OutputFormat,
    pub content: Option<Arc<String>>,
    pub tokens: Option<TokenCount>,
    pub generating: bool,
}
```

#### Usage Example

```rust
// Start generation
app_state.output.generating = true;
app_state.output.content = None;

// Update when generation completes
app_state.output.generating = false;
app_state.output.content = Some(Arc::new(generated_content));
app_state.output.tokens = Some(TokenCount::from_chars(content.len()));
```

## Configuration Management

### `ConfigManager`

Handles loading and saving of application configuration with platform-specific paths.

```rust
pub struct ConfigManager {
    config_path: PathBuf,
}
```

#### Methods

```rust
impl ConfigManager {
    /// Creates a new config manager with platform-specific config path
    pub fn new() -> Self
    
    /// Load configuration from disk, returns default if not found or invalid
    pub fn load(&self) -> AppConfig
    
    /// Save configuration to disk
    pub fn save(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>>
}
```

#### Usage Example

```rust
use crate::state::config::ConfigManager;

// Initialize config manager
let config_manager = ConfigManager::new();

// Load configuration at startup
let app_config = config_manager.load();

// Modify configuration
let mut updated_config = app_config;
updated_config.ui.theme = Theme::Dark;
updated_config.window.width = 1920.0;

// Save configuration
config_manager.save(&updated_config)?;
```

#### Platform-Specific Paths

The configuration is stored in platform-appropriate locations:

- **Linux**: `~/.config/fsprompt/config.json`
- **macOS**: `~/Library/Application Support/fsprompt/config.json`
- **Windows**: `%APPDATA%\fsprompt\config.json`

### `SerializableConfig`

Serializable configuration for persistence with backward compatibility.

```rust
pub struct SerializableConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub split_position: f32,
    pub last_directory: Option<PathBuf>,
    pub ignore_patterns: String,
    pub include_tree: bool,
    pub output_format: String,
    pub theme: String,
}
```

#### Methods

```rust
impl SerializableConfig {
    /// Convert to AppConfig with defaults for missing fields
    pub fn to_app_config(&self) -> AppConfig
}

impl From<&AppConfig> for SerializableConfig {
    fn from(config: &AppConfig) -> Self
}
```

#### Usage Example

```rust
// Convert for saving
let serializable = SerializableConfig::from(&app_config);
let json = serde_json::to_string_pretty(&serializable)?;
std::fs::write(config_path, json)?;

// Convert when loading
let serializable: SerializableConfig = serde_json::from_str(&content)?;
let app_config = serializable.to_app_config();
```

## History Management

### `HistoryManager`

Manages undo/redo history for file selections with configurable depth.

```rust
pub struct HistoryManager {
    past: Vec<SelectionSnapshot>,
    future: Vec<SelectionSnapshot>,
    max_depth: HistorySize,
}
```

#### Methods

```rust
impl HistoryManager {
    /// Creates a new history manager with specified maximum depth
    pub fn new(max_depth: HistorySize) -> Self
    
    /// Record a new state, clearing any redo history
    pub fn push(&mut self, snapshot: SelectionSnapshot)
    
    /// Undo the last action, returns the previous state if available
    pub fn undo(&mut self, current: SelectionSnapshot) -> Option<SelectionSnapshot>
    
    /// Redo the last undone action, returns the next state if available
    pub fn redo(&mut self, current: SelectionSnapshot) -> Option<SelectionSnapshot>
    
    /// Check if undo is available
    pub fn can_undo(&self) -> bool
    
    /// Check if redo is available
    pub fn can_redo(&self) -> bool
}
```

#### Usage Example

```rust
use crate::state::history::{HistoryManager, SelectionSnapshot};
use crate::core::types::HistorySize;

// Initialize history manager
let mut history = HistoryManager::new(HistorySize::new(20));

// Create snapshot of current state
let current_snapshot = SelectionSnapshot {
    selected_files: app_state.get_selected_files(),
    expanded_dirs: app_state.get_expanded_dirs(),
};

// Record state before making changes
history.push(current_snapshot.clone());

// Make changes to selection...
// (modify app state)

// Undo operation
if history.can_undo() {
    let new_current = SelectionSnapshot {
        selected_files: app_state.get_selected_files(),
        expanded_dirs: app_state.get_expanded_dirs(),
    };
    
    if let Some(previous_state) = history.undo(new_current) {
        // Restore previous state
        app_state.restore_selection(&previous_state.selected_files, &previous_state.expanded_dirs);
    }
}

// Redo operation
if history.can_redo() {
    let new_current = SelectionSnapshot {
        selected_files: app_state.get_selected_files(),
        expanded_dirs: app_state.get_expanded_dirs(),
    };
    
    if let Some(next_state) = history.redo(new_current) {
        // Restore next state
        app_state.restore_selection(&next_state.selected_files, &next_state.expanded_dirs);
    }
}
```

### `SelectionSnapshot`

Snapshot of selection state for undo/redo operations.

```rust
pub struct SelectionSnapshot {
    pub selected_files: HashSet<String>,
    pub expanded_dirs: HashSet<String>,
}
```

#### Usage Example

```rust
// Create snapshot from current state
let snapshot = SelectionSnapshot {
    selected_files: directory_tree.get_selected_files(),
    expanded_dirs: directory_tree.get_expanded_dirs(),
};

// Restore state from snapshot
directory_tree.restore_selection(&snapshot.selected_files, &snapshot.expanded_dirs);
```

## State Management Patterns

### State Updates with History

```rust
impl App {
    fn update_selection_with_history(&mut self, new_selection: HashSet<CanonicalPath>) {
        // Create snapshot before changes
        let snapshot = SelectionSnapshot {
            selected_files: self.tree.get_selected_files(),
            expanded_dirs: self.tree.get_expanded_dirs(),
        };
        
        // Record for undo
        self.history.push(snapshot);
        
        // Apply changes
        self.app_state.selections.selected = new_selection;
        
        // Update UI state
        self.tree.restore_selection(&self.get_selected_files_as_strings(), &HashSet::new());
    }
}
```

### Configuration Persistence

```rust
impl App {
    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Update config with current state
        let mut config = self.app_state.config.clone();
        config.window.width = self.window_width;
        config.window.height = self.window_height;
        config.window.left_pane_ratio = self.split_position;
        
        // Save to disk
        self.config_manager.save(&config)?;
        Ok(())
    }
    
    fn load_config(&mut self) {
        let config = self.config_manager.load();
        
        // Apply configuration
        self.app_state.config = config;
        self.window_width = self.app_state.config.window.width;
        self.window_height = self.app_state.config.window.height;
        self.split_position = self.app_state.config.window.left_pane_ratio;
        
        // Apply theme
        Theme::apply_theme(&self.ctx, self.app_state.config.ui.theme == crate::core::types::Theme::Dark);
    }
}
```

### Search State Management

```rust
impl App {
    fn handle_tree_search(&mut self, query: String) {
        self.app_state.search.tree_search.query = query.clone();
        self.app_state.search.tree_search.active = !query.is_empty();
        
        if !query.is_empty() {
            // Perform search and update results
            self.app_state.search.tree_search.results = self.perform_tree_search(&query);
        } else {
            self.app_state.search.tree_search.results.clear();
        }
    }
    
    fn handle_output_search(&mut self, query: String) {
        self.app_state.search.output_search.query = query.clone();
        self.app_state.search.output_search.active = !query.is_empty();
        
        if let Some(content) = &self.app_state.output.content {
            // Count matches in output
            self.app_state.search.output_search.match_count = content.matches(&query).count();
            self.app_state.search.output_search.current_match = 0;
        }
    }
    
    fn navigate_search_results(&mut self, forward: bool) {
        if forward {
            self.app_state.search.output_search.next_match();
        } else {
            self.app_state.search.output_search.prev_match();
        }
    }
}
```

### Output State Management

```rust
impl App {
    fn start_generation(&mut self) {
        // Clear previous output
        self.app_state.output.content = None;
        self.app_state.output.tokens = None;
        self.app_state.output.generating = true;
        
        // Start worker
        let command = WorkerCommand::GenerateOutput {
            root_path: self.app_state.root.clone().unwrap(),
            selected_files: self.app_state.selections.selected.iter().cloned().collect(),
            format: self.app_state.output.format,
            include_tree: self.app_state.config.ui.include_tree,
            ignore_patterns: PatternString::from_patterns(&self.app_state.config.ignore_patterns),
        };
        
        let _ = self.worker.send_command(command);
    }
    
    fn handle_generation_complete(&mut self, content: String, token_count: TokenCount) {
        self.app_state.output.generating = false;
        self.app_state.output.content = Some(Arc::new(content));
        self.app_state.output.tokens = Some(token_count);
        
        // Clear any search state in output
        self.app_state.search.output_search.query.clear();
        self.app_state.search.output_search.active = false;
        self.app_state.search.output_search.match_count = 0;
        self.app_state.search.output_search.current_match = 0;
    }
}
```

## Best Practices

### State Validation

```rust
impl AppState {
    /// Validates the current state for consistency
    pub fn validate(&self) -> Result<(), String> {
        // Check root directory exists if set
        if let Some(root) = &self.root {
            if !root.as_path().exists() {
                return Err("Root directory no longer exists".to_string());
            }
        }
        
        // Validate expanded paths are under root
        if let Some(root) = &self.root {
            for expanded_path in &self.expanded {
                if !expanded_path.is_contained_within(root) {
                    return Err("Expanded path outside root directory".to_string());
                }
            }
        }
        
        // Validate selected paths
        for selected_path in &self.selections.selected {
            if !selected_path.as_path().exists() {
                return Err("Selected file no longer exists".to_string());
            }
        }
        
        Ok(())
    }
}
```

### State Synchronization

```rust
impl App {
    /// Synchronizes UI state with app state
    fn sync_state(&mut self) {
        // Sync tree selection with app state
        let selected_strings: HashSet<String> = self.app_state.selections.selected
            .iter()
            .map(|p| p.as_path().to_string_lossy().to_string())
            .collect();
            
        let expanded_strings: HashSet<String> = self.app_state.expanded
            .iter()
            .map(|p| p.as_path().to_string_lossy().to_string())
            .collect();
            
        self.tree.restore_selection(&selected_strings, &expanded_strings);
        
        // Sync ignore patterns
        let patterns_str = self.app_state.config.ignore_patterns.join(",");
        self.tree.set_ignore_patterns(&patterns_str);
    }
}
```

### Error Recovery

```rust
impl App {
    fn handle_state_error(&mut self, error: String) {
        eprintln!("State error: {}", error);
        
        // Try to recover by resetting problematic state
        if error.contains("Root directory") {
            self.app_state.root = None;
            self.app_state.expanded.clear();
            self.app_state.selections.selected.clear();
        }
        
        if error.contains("Selected file") {
            // Remove non-existent files from selection
            self.app_state.selections.selected.retain(|path| path.as_path().exists());
        }
        
        // Resync UI
        self.sync_state();
        
        // Show error to user
        self.toast_manager.error(format!("State error: {}", error));
    }
}
```

### Memory Management

```rust
impl App {
    fn cleanup_state(&mut self) {
        // Limit history size
        while self.app_state.selections.undo_stack.len() > SelectionTracker::MAX_HISTORY {
            self.app_state.selections.undo_stack.remove(0);
        }
        
        while self.app_state.selections.redo_stack.len() > SelectionTracker::MAX_HISTORY {
            self.app_state.selections.redo_stack.remove(0);
        }
        
        // Clear old search results
        if !self.app_state.search.tree_search.active {
            self.app_state.search.tree_search.results.clear();
        }
        
        // Release large output content if not displayed
        if self.app_state.output.generating {
            self.app_state.output.content = None;
        }
    }
}
```