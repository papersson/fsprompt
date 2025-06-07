# Core Types API Reference

Complete reference for all newtypes and domain types in fsPrompt's type system (`src/core/types.rs`).

## Philosophy

fsPrompt follows type-driven development principles to ensure correctness and prevent bugs through the type system. This module provides domain-specific newtypes that:

- Make illegal states unrepresentable
- Provide clear APIs with meaningful names
- Include built-in validation and constraints
- Separate data from UI state

## Path Types

### `CanonicalPath`

A validated, canonical path that prevents path traversal attacks.

```rust
pub struct CanonicalPath(PathBuf);
```

#### Methods

```rust
impl CanonicalPath {
    /// Creates a new canonical path, resolving symlinks and normalizing
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self>
    
    /// Get the inner path
    pub fn as_path(&self) -> &Path
    
    /// Get the inner PathBuf  
    pub fn to_path_buf(&self) -> PathBuf
    
    /// Get the file name
    pub fn file_name(&self) -> Option<&std::ffi::OsStr>
    
    /// Get parent directory
    pub fn parent(&self) -> Option<Self>
    
    /// Check if this path is contained within the given root path
    pub fn is_contained_within(&self, root: &CanonicalPath) -> bool
    
    /// Create a new canonical path that is guaranteed to be within the root
    pub fn new_within_root(path: impl AsRef<Path>, root: &CanonicalPath) -> std::io::Result<Self>
}
```

#### Usage Example

```rust
use crate::core::types::CanonicalPath;

// Safe path creation with validation
let root = CanonicalPath::new("/home/user/project")?;
let file = CanonicalPath::new_within_root("../../../etc/passwd", &root)?; // Error!

// Safe path operations
if file.is_contained_within(&root) {
    println!("File: {}", file.as_path().display());
}
```

#### When to Use
- All file/directory operations
- Path storage in data structures
- Security-sensitive path handling

### `SerializableCanonicalPath`

Serializable wrapper for `CanonicalPath` used in configuration persistence.

```rust
pub struct SerializableCanonicalPath(PathBuf);
```

#### Methods

```rust
impl SerializableCanonicalPath {
    /// Create from a canonical path
    pub fn from_canonical(path: &CanonicalPath) -> Self
    
    /// Try to convert to a canonical path
    pub fn to_canonical(&self) -> Result<CanonicalPath, std::io::Error>
}
```

#### Usage Example

```rust
// Save to config
let config_path = SerializableCanonicalPath::from_canonical(&canonical_path);
serde_json::to_string(&config_path)?;

// Load from config
let canonical = serializable_path.to_canonical()?;
```

## Size and Count Types

### `TokenCount`

Type-safe token counting with level categorization.

```rust
pub struct TokenCount(usize);
```

#### Methods

```rust
impl TokenCount {
    /// Creates a new token count
    pub const fn new(count: usize) -> Self
    
    /// Estimates tokens from character count (roughly 1 token = 4 chars)
    pub const fn from_chars(chars: usize) -> Self
    
    /// Gets the raw count
    pub const fn get(&self) -> usize
    
    /// Gets the estimation level
    pub const fn level(&self) -> TokenLevel
}
```

#### Usage Example

```rust
let content = "Large file content...";
let tokens = TokenCount::from_chars(content.len());

match tokens.level() {
    TokenLevel::Low => println!("Small file ({} tokens)", tokens.get()),
    TokenLevel::Medium => println!("Medium file ({} tokens)", tokens.get()),
    TokenLevel::High => println!("Large file ({} tokens)", tokens.get()),
}
```

### `TokenLevel`

Categorizes token counts for UI display.

```rust
pub enum TokenLevel {
    Low,    // <1,000 tokens
    Medium, // 1,000-10,000 tokens  
    High,   // >10,000 tokens
}
```

### `FileSize`

Type-safe file size with read strategy determination.

```rust
pub struct FileSize(u64);
```

#### Methods

```rust
impl FileSize {
    /// Creates a new file size
    pub const fn from_bytes(bytes: u64) -> Self
    
    /// Gets the size in bytes
    pub const fn bytes(&self) -> u64
    
    /// Determines read strategy based on size
    pub const fn read_strategy(&self) -> FileReadStrategy
}
```

#### Usage Example

```rust
let size = FileSize::from_bytes(metadata.len());
match size.read_strategy() {
    FileReadStrategy::Direct => {
        // Read entire file into memory
        std::fs::read_to_string(path)?
    }
    FileReadStrategy::MemoryMapped => {
        // Use memory mapping for large files
        read_with_mmap(path)?
    }
}
```

### `FileCount`

Type-safe file counting.

```rust
pub struct FileCount(usize);
```

#### Methods

```rust
impl FileCount {
    /// Create a new file count
    pub const fn new(count: usize) -> Self
    
    /// Get the raw count
    pub const fn get(&self) -> usize
    
    /// Increment the count
    pub fn increment(&mut self)
}
```

### `ProgressCount`

Progress tracking with percentage calculation.

```rust
pub struct ProgressCount {
    current: usize,
    total: usize,
}
```

#### Methods

```rust
impl ProgressCount {
    /// Create a new progress count
    pub const fn new(current: usize, total: usize) -> Self
    
    /// Get the current count
    pub const fn current(&self) -> usize
    
    /// Get the total count
    pub const fn total(&self) -> usize
    
    /// Get progress as a percentage (0.0 to 100.0)
    pub fn percentage(&self) -> f32
    
    /// Check if complete
    pub const fn is_complete(&self) -> bool
    
    /// Increment current count
    pub fn increment(&mut self)
}
```

#### Usage Example

```rust
let mut progress = ProgressCount::new(0, 100);
while !progress.is_complete() {
    // Do work...
    progress.increment();
    println!("Progress: {:.1}%", progress.percentage());
}
```

## Memory and Performance Types

### `MemorySize`

Memory size with unit conversions.

```rust
pub struct MemorySize(usize);
```

#### Methods

```rust
impl MemorySize {
    /// Create from bytes
    pub const fn from_bytes(bytes: usize) -> Self
    
    /// Create from megabytes
    pub const fn from_mb(mb: usize) -> Self
    
    /// Create from kilobytes
    pub const fn from_kb(kb: usize) -> Self
    
    /// Get as bytes
    pub const fn as_bytes(&self) -> usize
    
    /// Get as megabytes (rounded down)
    pub const fn as_mb(&self) -> usize
}
```

### `GenerationTime`

Duration tracking for generation operations.

```rust
pub struct GenerationTime(std::time::Duration);
```

#### Methods

```rust
impl GenerationTime {
    /// Create from a duration
    pub fn from_duration(duration: std::time::Duration) -> Self
    
    /// Get as milliseconds
    pub fn as_millis(&self) -> u128
    
    /// Get the inner duration
    pub fn as_duration(&self) -> std::time::Duration
}
```

### `TreeDepth`

Tree traversal depth tracking.

```rust
pub struct TreeDepth(usize);
```

#### Methods

```rust
impl TreeDepth {
    /// Create a new tree depth
    pub const fn new(depth: usize) -> Self
    
    /// Get the raw depth
    pub const fn get(&self) -> usize
    
    /// Increment depth (for traversing deeper)
    pub fn increment(&self) -> Self
    
    /// Check if at or beyond a limit
    pub const fn exceeds(&self, limit: usize) -> bool
}
```

## Validation Types

### `FontSize`

Validated font size with range constraints.

```rust
pub struct FontSize(f32);

impl FontSize {
    pub const MIN: f32 = 8.0;
    pub const MAX: f32 = 24.0;
}
```

#### Methods

```rust
impl FontSize {
    /// Create a new font size with validation
    pub fn new(size: f32) -> Result<Self, ValidationError>
    
    /// Get the raw size
    pub const fn get(&self) -> f32
}
```

### `WindowRatio`

Validated window split ratio (0.0 to 1.0).

```rust
pub struct WindowRatio(f32);
```

#### Methods

```rust
impl WindowRatio {
    /// Create a new window ratio with validation
    pub fn new(ratio: f32) -> Result<Self, ValidationError>
    
    /// Get the raw ratio
    pub const fn get(&self) -> f32
}
```

### `NonEmptyString`

String that cannot be empty or whitespace-only.

```rust
pub struct NonEmptyString(String);
```

#### Methods

```rust
impl NonEmptyString {
    /// Create a new non-empty string
    pub fn new(s: String) -> Result<Self, ValidationError>
    
    /// Get the inner string
    pub fn as_str(&self) -> &str
    
    /// Convert to owned String
    pub fn into_string(self) -> String
}
```

## Filesystem Entry Types

### `FsEntry`

Filesystem entry metadata (pure data, no UI state).

```rust
pub struct FsEntry {
    pub path: CanonicalPath,
    pub name: String,
    pub entry_type: FsEntryType,
}
```

#### Methods

```rust
impl FsEntry {
    /// Check if this is a directory
    pub const fn is_dir(&self) -> bool
    
    /// Get file size if this is a file
    pub const fn file_size(&self) -> Option<FileSize>
    
    /// Check if this entry matches a pattern
    pub fn matches(&self, pattern: &IgnorePattern) -> bool
}
```

### `FsEntryType`

Type of filesystem entry with associated data.

```rust
pub enum FsEntryType {
    File { size: FileSize },
    Directory,
}
```

## Selection and UI State

### `SelectionState`

Tri-state selection for UI components.

```rust
pub enum SelectionState {
    Unchecked,     // Not selected
    Checked,       // Fully selected
    Indeterminate, // Partially selected (directories with mixed children)
}
```

#### Methods

```rust
impl SelectionState {
    /// Check if any form of selection
    pub const fn is_selected(&self) -> bool
    
    /// Convert from a simple boolean
    pub const fn from_bool(selected: bool) -> Self
}
```

#### Usage Example

```rust
let state = SelectionState::from_bool(true);
assert!(state.is_selected());

// For tri-state checkboxes
match selection_state {
    SelectionState::Unchecked => render_unchecked_box(),
    SelectionState::Checked => render_checked_box(),
    SelectionState::Indeterminate => render_mixed_box(),
}
```

## Pattern Matching

### `IgnorePattern`

Compiled ignore pattern with multiple pattern types.

```rust
pub struct IgnorePattern {
    pub pattern: String,
    pub pattern_type: PatternType,
    // compiled: Arc<dyn Fn(&Path) -> bool + Send + Sync>,
}
```

#### Methods

```rust
impl IgnorePattern {
    /// Create from a pattern string, auto-detecting type
    pub fn from_str(pattern: &str) -> Result<Self, String>
}
```

### `PatternType`

Type of pattern matching.

```rust
pub enum PatternType {
    Exact,  // Exact file/directory name
    Glob,   // Glob pattern (*, ?)
    Regex,  // Regular expression
}
```

### `PatternString`

Comma-separated pattern string for multiple patterns.

```rust
pub struct PatternString(String);
```

#### Methods

```rust
impl PatternString {
    /// Create from a string
    pub fn new(s: String) -> Self
    
    /// Create from a slice of patterns
    pub fn from_patterns(patterns: &[String]) -> Self
    
    /// Split into individual patterns
    pub fn split(&self) -> Vec<String>
    
    /// Get as string
    pub fn as_str(&self) -> &str
    
    /// Check if empty
    pub fn is_empty(&self) -> bool
}
```

## Output Types

### `OutputFormat`

Output format options.

```rust
pub enum OutputFormat {
    Xml,      // XML format
    Markdown, // Markdown format  
}
```

### `ClipboardContent`

Clipboard content with metadata.

```rust
pub struct ClipboardContent {
    content: String,
    format: OutputFormat,
}
```

#### Methods

```rust
impl ClipboardContent {
    /// Create new clipboard content
    pub fn new(content: String, format: OutputFormat) -> Self
    
    /// Get the content
    pub fn content(&self) -> &str
    
    /// Get the format
    pub const fn format(&self) -> OutputFormat
    
    /// Convert to owned String
    pub fn into_string(self) -> String
}
```

## Application State Types

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

### `SelectionTracker`

Tracks selections with undo/redo support.

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

## Configuration Types

### `AppConfig`

Application configuration with builder pattern.

```rust
pub struct AppConfig {
    pub window: WindowConfig,
    pub ui: UiConfig,
    pub ignore_patterns: Vec<String>,
    pub performance: PerformanceConfig,
}
```

### `AppConfigBuilder`

Builder for constructing `AppConfig`.

```rust
impl AppConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self
    
    /// Set window configuration
    pub fn window(mut self, window: WindowConfig) -> Self
    
    /// Set UI configuration
    pub fn ui(mut self, ui: UiConfig) -> Self
    
    /// Set ignore patterns
    pub fn ignore_patterns(mut self, patterns: Vec<String>) -> Self
    
    /// Add a single ignore pattern
    pub fn add_ignore_pattern(mut self, pattern: String) -> Self
    
    /// Set performance configuration
    pub fn performance(mut self, perf: PerformanceConfig) -> Self
    
    /// Build the final AppConfig
    pub fn build(self) -> AppConfig
}
```

#### Usage Example

```rust
let config = AppConfigBuilder::new()
    .add_ignore_pattern("*.log".to_string())
    .add_ignore_pattern("tmp/".to_string())
    .window(WindowConfigBuilder::new()
        .dimensions(1920.0, 1080.0)
        .left_pane_ratio(0.4)
        .build())
    .build();
```

### `WindowConfig`

Window configuration with builder.

```rust
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub left_pane_ratio: f32,
}
```

### `WindowConfigBuilder`

Builder for `WindowConfig`.

```rust
impl WindowConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self
    
    /// Set window dimensions
    pub fn dimensions(mut self, width: f32, height: f32) -> Self
    
    /// Set window width
    pub fn width(mut self, width: f32) -> Self
    
    /// Set window height
    pub fn height(mut self, height: f32) -> Self
    
    /// Set left pane ratio
    pub fn left_pane_ratio(mut self, ratio: f32) -> Self
    
    /// Build the final WindowConfig
    pub fn build(self) -> WindowConfig
}
```

## Error Types

### `ValidationError`

Validation error types for constrained values.

```rust
pub enum ValidationError {
    FontSizeOutOfRange,  // Font size is out of valid range
    RatioOutOfRange,     // Window ratio is out of valid range
    EmptyString,         // String is empty or whitespace only
}
```

### `SystemError`

System operation errors.

```rust
pub enum SystemError {
    TimeError(String),     // Time operation failed
    MutexPoisoned(String), // Mutex was poisoned
}
```

## Toast Notification Types

### `Toast`

Type-safe toast notifications.

```rust
pub struct Toast {
    pub variant: ToastVariant,
    pub duration_secs: u8,
}
```

#### Methods

```rust
impl Toast {
    /// Create a success toast
    pub fn success(message: impl Into<String>) -> Self
    
    /// Create an error toast
    pub fn error(message: impl Into<String>) -> Self
}
```

### `ToastVariant`

Toast variants with associated data.

```rust
pub enum ToastVariant {
    Success(String),
    Warning(String),
    Error {
        message: String,
        details: Option<String>,
    },
    Progress {
        message: String,
        percentage: f32,
    },
}
```

## Common Patterns

### Type-Driven Development

Always prefer domain-specific newtypes over primitives:

```rust
// Bad
fn process_files(paths: Vec<PathBuf>, token_limit: usize) -> usize

// Good  
fn process_files(paths: Vec<CanonicalPath>, token_limit: TokenCount) -> TokenCount
```

### Builder Pattern

Use builders for complex configuration:

```rust
let config = AppConfigBuilder::new()
    .window(WindowConfigBuilder::new()
        .dimensions(1200.0, 800.0)
        .build())
    .add_ignore_pattern("*.log".to_string())
    .build();
```

### Error Handling

Use typed errors for different failure modes:

```rust
match CanonicalPath::new_within_root(user_path, &root) {
    Ok(safe_path) => process_file(safe_path),
    Err(_) => return Err("Path traversal attack detected"),
}
```

### Validation

Use validated types to prevent invalid states:

```rust
let font_size = FontSize::new(user_input)?; // Validates range
let ratio = WindowRatio::new(split_pos)?;   // Validates 0.0-1.0
```