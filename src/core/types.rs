#![warn(missing_docs)]

//! Redesigned type system for fsPrompt with improved expressiveness and type safety

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// ===== Newtypes for Domain Concepts =====

/// A validated, canonical path
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalPath(PathBuf);

impl CanonicalPath {
    /// Creates a new canonical path, resolving symlinks and normalizing
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        Ok(Self(path.as_ref().canonicalize()?))
    }

    /// Get the inner path
    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Get the inner PathBuf
    #[must_use]
    pub fn to_path_buf(&self) -> PathBuf {
        self.0.clone()
    }

    /// Get the file name
    #[must_use]
    pub fn file_name(&self) -> Option<&std::ffi::OsStr> {
        self.0.file_name()
    }

    /// Get parent directory
    pub fn parent(&self) -> Option<Self> {
        self.0.parent().and_then(|p| Self::new(p).ok())
    }
}

/// Serializable wrapper for CanonicalPath
///
/// This type exists to bridge the gap between type safety and persistence.
/// Use CanonicalPath for runtime operations and SerializableCanonicalPath for config storage.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SerializableCanonicalPath(PathBuf);

impl SerializableCanonicalPath {
    /// Create from a canonical path
    pub fn from_canonical(path: &CanonicalPath) -> Self {
        Self(path.as_path().to_path_buf())
    }

    /// Try to convert to a canonical path
    pub fn to_canonical(&self) -> Result<CanonicalPath, std::io::Error> {
        CanonicalPath::new(&self.0)
    }
}

impl From<&CanonicalPath> for SerializableCanonicalPath {
    fn from(path: &CanonicalPath) -> Self {
        SerializableCanonicalPath::from_canonical(path)
    }
}

impl TryFrom<SerializableCanonicalPath> for CanonicalPath {
    type Error = std::io::Error;

    fn try_from(path: SerializableCanonicalPath) -> Result<Self, Self::Error> {
        path.to_canonical()
    }
}

impl TryFrom<&SerializableCanonicalPath> for CanonicalPath {
    type Error = std::io::Error;

    fn try_from(path: &SerializableCanonicalPath) -> Result<Self, Self::Error> {
        path.to_canonical()
    }
}

/// Token count with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TokenCount(usize);

impl TokenCount {
    /// Creates a new token count
    #[must_use]
    pub const fn new(count: usize) -> Self {
        Self(count)
    }

    /// Estimates tokens from character count (roughly 1 token = 4 chars)
    #[must_use]
    pub const fn from_chars(chars: usize) -> Self {
        Self((chars + 3) / 4)
    }

    /// Gets the raw count
    #[must_use]
    pub const fn get(&self) -> usize {
        self.0
    }

    /// Gets the estimation level
    #[must_use]
    pub const fn level(&self) -> TokenLevel {
        match self.0 {
            0..=999 => TokenLevel::Low,
            1000..=9999 => TokenLevel::Medium,
            _ => TokenLevel::High,
        }
    }
}

/// Token count levels for UI display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenLevel {
    /// <1,000 tokens
    Low,
    /// 1,000-10,000 tokens  
    Medium,
    /// >10,000 tokens
    High,
}

/// File size with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileSize(u64);

impl FileSize {
    /// Creates a new file size
    #[must_use]
    pub const fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    /// Gets the size in bytes
    #[must_use]
    pub const fn bytes(&self) -> u64 {
        self.0
    }

    /// Determines read strategy based on size
    #[must_use]
    pub const fn read_strategy(&self) -> FileReadStrategy {
        const MEMORY_MAP_THRESHOLD: u64 = 256 * 1024; // 256KB

        if self.0 < MEMORY_MAP_THRESHOLD {
            FileReadStrategy::Direct
        } else {
            FileReadStrategy::MemoryMapped
        }
    }
}

// ===== Pure Data Types (No UI State) =====

/// File system entry metadata (pure data, no UI state)
#[derive(Debug, Clone)]
pub struct FsEntry {
    /// Canonical path to the entry
    pub path: CanonicalPath,
    /// Display name
    pub name: String,
    /// Entry type
    pub entry_type: FsEntryType,
}

/// Type of filesystem entry with associated data
#[derive(Debug, Clone)]
pub enum FsEntryType {
    /// Regular file with size
    File {
        /// Size of the file
        size: FileSize,
    },
    /// Directory
    Directory,
}

impl FsEntry {
    /// Check if this is a directory
    #[must_use]
    pub const fn is_dir(&self) -> bool {
        matches!(self.entry_type, FsEntryType::Directory)
    }

    /// Get file size if this is a file
    #[must_use]
    pub const fn file_size(&self) -> Option<FileSize> {
        match &self.entry_type {
            FsEntryType::File { size } => Some(*size),
            _ => None,
        }
    }

    /// Check if this entry matches a pattern
    pub fn matches(&self, pattern: &IgnorePattern) -> bool {
        (pattern.compiled)(self.path.as_path())
    }
}

// ===== UI State Types (Separate from Data) =====

/// Selection state for a file/directory in the UI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionState {
    /// Not selected
    #[default]
    Unchecked,
    /// Fully selected
    Checked,
    /// Partially selected (directories with mixed children)
    Indeterminate,
}

impl SelectionState {
    /// Check if any form of selection
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        !matches!(self, Self::Unchecked)
    }

    /// Convert from a simple boolean
    #[must_use]
    pub const fn from_bool(selected: bool) -> Self {
        if selected {
            Self::Checked
        } else {
            Self::Unchecked
        }
    }
}

// ===== Pattern Types =====

/// Type of ignore pattern
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternType {
    /// Exact file/directory name
    Exact,
    /// Glob pattern (*, ?)
    Glob,
    /// Regular expression
    Regex,
}

/// Compiled ignore pattern
#[derive(Clone)]
pub struct IgnorePattern {
    /// Original pattern string
    pub pattern: String,
    /// Pattern type
    pub pattern_type: PatternType,
    /// Compiled pattern (opaque to avoid exposing regex)
    compiled: Arc<dyn Fn(&Path) -> bool + Send + Sync>,
}

impl IgnorePattern {
    /// Create from a pattern string, auto-detecting type
    pub fn from_str(pattern: &str) -> Result<Self, String> {
        let pattern_type = if pattern.contains('*') || pattern.contains('?') {
            PatternType::Glob
        } else if pattern.starts_with('^') || pattern.ends_with('$') {
            PatternType::Regex
        } else {
            PatternType::Exact
        };

        // Compile the pattern based on its type
        let compiled = match pattern_type {
            PatternType::Exact => {
                let pattern = pattern.to_string();
                Arc::new(move |path: &Path| -> bool {
                    path.to_str().map(|p| p.contains(&pattern)).unwrap_or(false)
                }) as Arc<dyn Fn(&Path) -> bool + Send + Sync>
            }
            PatternType::Glob => {
                let glob_pattern = glob::Pattern::new(pattern)
                    .map_err(|e| format!("Invalid glob pattern: {}", e))?;
                Arc::new(move |path: &Path| -> bool {
                    path.to_str()
                        .map(|p| glob_pattern.matches(p))
                        .unwrap_or(false)
                }) as Arc<dyn Fn(&Path) -> bool + Send + Sync>
            }
            PatternType::Regex => {
                let regex = regex::Regex::new(pattern)
                    .map_err(|e| format!("Invalid regex pattern: {}", e))?;
                Arc::new(move |path: &Path| -> bool {
                    path.to_str().map(|p| regex.is_match(p)).unwrap_or(false)
                }) as Arc<dyn Fn(&Path) -> bool + Send + Sync>
            }
        };

        Ok(Self {
            pattern: pattern.to_string(),
            pattern_type,
            compiled,
        })
    }
}

impl std::fmt::Debug for IgnorePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IgnorePattern")
            .field("pattern", &self.pattern)
            .field("compiled", &"<compiled>")
            .finish()
    }
}

// ===== Output Types =====

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// XML format
    #[default]
    Xml,
    /// Markdown format  
    Markdown,
}

/// File reading strategy
#[derive(Debug, Clone, Copy)]
pub enum FileReadStrategy {
    /// Read entire file into memory
    Direct,
    /// Use memory mapping
    MemoryMapped,
}

// ===== Domain-Specific Newtypes =====

/// Represents a duration of time for generation operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenerationTime(std::time::Duration);

impl GenerationTime {
    /// Create from a duration
    pub fn from_duration(duration: std::time::Duration) -> Self {
        Self(duration)
    }

    /// Get as milliseconds
    pub fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    /// Get the inner duration
    pub fn as_duration(&self) -> std::time::Duration {
        self.0
    }
}

/// Count of files with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FileCount(usize);

impl FileCount {
    /// Create a new file count
    pub const fn new(count: usize) -> Self {
        Self(count)
    }

    /// Get the raw count
    pub const fn get(&self) -> usize {
        self.0
    }

    /// Increment the count
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

/// Progress tracking with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ProgressCount {
    current: usize,
    total: usize,
}

impl ProgressCount {
    /// Create a new progress count
    pub const fn new(current: usize, total: usize) -> Self {
        Self { current, total }
    }

    /// Get the current count
    pub const fn current(&self) -> usize {
        self.current
    }

    /// Get the total count
    pub const fn total(&self) -> usize {
        self.total
    }

    /// Get progress as a percentage (0.0 to 100.0)
    pub fn percentage(&self) -> f32 {
        if self.total == 0 {
            100.0
        } else {
            (self.current as f32 / self.total as f32) * 100.0
        }
    }

    /// Check if complete
    pub const fn is_complete(&self) -> bool {
        self.current >= self.total
    }

    /// Increment current count
    pub fn increment(&mut self) {
        if self.current < self.total {
            self.current += 1;
        }
    }
}

/// Maximum history size for undo/redo operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HistorySize(usize);

impl HistorySize {
    /// Create a new history size
    pub const fn new(size: usize) -> Self {
        Self(size)
    }

    /// Get the raw size
    pub const fn get(&self) -> usize {
        self.0
    }

    /// Default history size
    pub const fn default() -> Self {
        Self(20)
    }
}

impl Default for HistorySize {
    fn default() -> Self {
        Self::default()
    }
}

/// Memory size in bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MemorySize(usize);

impl MemorySize {
    /// Create from bytes
    pub const fn from_bytes(bytes: usize) -> Self {
        Self(bytes)
    }

    /// Create from megabytes
    pub const fn from_mb(mb: usize) -> Self {
        Self(mb * 1024 * 1024)
    }

    /// Create from kilobytes
    pub const fn from_kb(kb: usize) -> Self {
        Self(kb * 1024)
    }

    /// Get as bytes
    pub const fn as_bytes(&self) -> usize {
        self.0
    }

    /// Get as megabytes (rounded down)
    pub const fn as_mb(&self) -> usize {
        self.0 / (1024 * 1024)
    }
}

/// Tree traversal depth
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TreeDepth(usize);

impl TreeDepth {
    /// Create a new tree depth
    pub const fn new(depth: usize) -> Self {
        Self(depth)
    }

    /// Get the raw depth
    pub const fn get(&self) -> usize {
        self.0
    }

    /// Increment depth (for traversing deeper)
    pub fn increment(&self) -> Self {
        Self(self.0 + 1)
    }

    /// Check if at or beyond a limit
    pub const fn exceeds(&self, limit: usize) -> bool {
        self.0 >= limit
    }
}

/// Validation error types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Font size is out of valid range
    FontSizeOutOfRange,
    /// Window ratio is out of valid range
    RatioOutOfRange,
    /// String is empty or whitespace only
    EmptyString,
}

/// System operation errors
#[derive(Debug, Clone, PartialEq)]
pub enum SystemError {
    /// Time operation failed
    TimeError(String),
    /// Mutex was poisoned
    MutexPoisoned(String),
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeError(msg) => write!(f, "Time operation failed: {}", msg),
            Self::MutexPoisoned(msg) => write!(f, "Mutex poisoned: {}", msg),
        }
    }
}

impl std::error::Error for SystemError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FontSizeOutOfRange => write!(f, "Font size must be between 8.0 and 24.0"),
            Self::RatioOutOfRange => write!(f, "Ratio must be between 0.0 and 1.0"),
            Self::EmptyString => write!(f, "String cannot be empty"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Font size with validation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontSize(f32);

impl FontSize {
    /// Minimum allowed font size
    pub const MIN: f32 = 8.0;
    /// Maximum allowed font size
    pub const MAX: f32 = 24.0;

    /// Create a new font size with validation
    pub fn new(size: f32) -> Result<Self, ValidationError> {
        if size >= Self::MIN && size <= Self::MAX {
            Ok(Self(size))
        } else {
            Err(ValidationError::FontSizeOutOfRange)
        }
    }

    /// Get the raw size
    pub const fn get(&self) -> f32 {
        self.0
    }
}

/// Window split ratio with validation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowRatio(f32);

impl WindowRatio {
    /// Create a new window ratio with validation
    pub fn new(ratio: f32) -> Result<Self, ValidationError> {
        if (0.0..=1.0).contains(&ratio) {
            Ok(Self(ratio))
        } else {
            Err(ValidationError::RatioOutOfRange)
        }
    }

    /// Get the raw ratio
    pub const fn get(&self) -> f32 {
        self.0
    }
}

/// Non-empty string validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Create a new non-empty string
    pub fn new(s: String) -> Result<Self, ValidationError> {
        if s.trim().is_empty() {
            Err(ValidationError::EmptyString)
        } else {
            Ok(Self(s))
        }
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to owned String
    pub fn into_string(self) -> String {
        self.0
    }
}

/// Clipboard content with metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardContent {
    /// The actual content
    content: String,
    /// Format of the content
    format: OutputFormat,
}

impl ClipboardContent {
    /// Create new clipboard content
    pub fn new(content: String, format: OutputFormat) -> Self {
        Self { content, format }
    }

    /// Get the content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the format
    pub const fn format(&self) -> OutputFormat {
        self.format
    }

    /// Convert to owned String
    pub fn into_string(self) -> String {
        self.content
    }
}

/// Pattern string for ignore patterns (comma-separated)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PatternString(String);

impl PatternString {
    /// Create from a string
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Create from a slice of patterns
    pub fn from_patterns(patterns: &[String]) -> Self {
        Self(patterns.join(","))
    }

    /// Split into individual patterns
    pub fn split(&self) -> Vec<String> {
        self.0
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Get as string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }
}

// ===== Application State =====

/// Main application state with clear separation of concerns
#[derive(Debug)]
pub struct AppState {
    /// Current root directory
    pub root: Option<CanonicalPath>,
    /// Expanded directories
    pub expanded: HashSet<CanonicalPath>,
    /// Selection tracking
    pub selections: SelectionTracker,
    /// Search state
    pub search: SearchState,
    /// Output state
    pub output: OutputState,
    /// Application configuration
    pub config: AppConfig,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            root: None,
            expanded: HashSet::new(),
            selections: SelectionTracker::default(),
            search: SearchState::default(),
            output: OutputState::default(),
            config: AppConfig::default(),
        }
    }
}

/// Tracks selections with undo/redo support
#[derive(Debug, Default)]
pub struct SelectionTracker {
    /// Current selections
    pub selected: HashSet<CanonicalPath>,
    /// Undo stack
    pub undo_stack: Vec<HashSet<CanonicalPath>>,
    /// Redo stack
    pub redo_stack: Vec<HashSet<CanonicalPath>>,
}

impl SelectionTracker {
    /// Maximum undo history size
    pub const MAX_HISTORY: usize = 20;

    /// Records current state for undo
    pub fn checkpoint(&mut self) {
        if self.undo_stack.len() >= Self::MAX_HISTORY {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(self.selected.clone());
        self.redo_stack.clear();
    }
}

/// Search state with separate tree and output search
#[derive(Debug, Default)]
pub struct SearchState {
    /// Tree search
    pub tree_search: TreeSearch,
    /// Output search
    pub output_search: OutputSearch,
}

/// Tree/file search state
#[derive(Debug, Default)]
pub struct TreeSearch {
    /// Current search query
    pub query: String,
    /// Search results (matching paths)
    pub results: HashSet<CanonicalPath>,
    /// Is search active
    pub active: bool,
}

/// Output content search state
#[derive(Debug, Default)]
pub struct OutputSearch {
    /// Current search query
    pub query: String,
    /// Number of matches
    pub match_count: usize,
    /// Current match index (0-based)
    pub current_match: usize,
    /// Is search active
    pub active: bool,
}

impl OutputSearch {
    /// Move to next match
    pub fn next_match(&mut self) {
        if self.match_count > 0 {
            self.current_match = (self.current_match + 1) % self.match_count;
        }
    }

    /// Move to previous match
    pub fn prev_match(&mut self) {
        if self.match_count > 0 {
            self.current_match = if self.current_match == 0 {
                self.match_count - 1
            } else {
                self.current_match - 1
            };
        }
    }
}

/// Output generation state
#[derive(Debug, Default)]
pub struct OutputState {
    /// Current output format
    pub format: OutputFormat,
    /// Generated content
    pub content: Option<Arc<String>>,
    /// Token count
    pub tokens: Option<TokenCount>,
    /// Is generation in progress
    pub generating: bool,
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Window settings
    pub window: WindowConfig,
    /// UI preferences
    pub ui: UiConfig,
    /// Default ignore patterns
    pub ignore_patterns: Vec<String>,
    /// Performance settings
    pub performance: PerformanceConfig,
}

/// Builder for AppConfig
#[derive(Debug, Default)]
pub struct AppConfigBuilder {
    window: Option<WindowConfig>,
    ui: Option<UiConfig>,
    ignore_patterns: Option<Vec<String>>,
    performance: Option<PerformanceConfig>,
}

impl AppConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set window configuration
    pub fn window(mut self, window: WindowConfig) -> Self {
        self.window = Some(window);
        self
    }

    /// Set UI configuration
    pub fn ui(mut self, ui: UiConfig) -> Self {
        self.ui = Some(ui);
        self
    }

    /// Set ignore patterns
    pub fn ignore_patterns(mut self, patterns: Vec<String>) -> Self {
        self.ignore_patterns = Some(patterns);
        self
    }

    /// Add a single ignore pattern
    pub fn add_ignore_pattern(mut self, pattern: String) -> Self {
        self.ignore_patterns
            .get_or_insert_with(|| {
                vec![
                    ".*".to_string(),
                    "node_modules".to_string(),
                    "__pycache__".to_string(),
                    "target".to_string(),
                    "build".to_string(),
                    "dist".to_string(),
                    "_*".to_string(),
                ]
            })
            .push(pattern);
        self
    }

    /// Set performance configuration
    pub fn performance(mut self, perf: PerformanceConfig) -> Self {
        self.performance = Some(perf);
        self
    }

    /// Build the final AppConfig
    pub fn build(self) -> AppConfig {
        AppConfig {
            window: self.window.unwrap_or_default(),
            ui: self.ui.unwrap_or_default(),
            ignore_patterns: self.ignore_patterns.unwrap_or_else(|| {
                vec![
                    ".*".to_string(),
                    "node_modules".to_string(),
                    "__pycache__".to_string(),
                    "target".to_string(),
                    "build".to_string(),
                    "dist".to_string(),
                    "_*".to_string(),
                ]
            }),
            performance: self.performance.unwrap_or_default(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            ui: UiConfig::default(),
            ignore_patterns: vec![
                ".*".to_string(),
                "node_modules".to_string(),
                "__pycache__".to_string(),
                "target".to_string(),
                "build".to_string(),
                "dist".to_string(),
                "_*".to_string(),
            ],
            performance: PerformanceConfig::default(),
        }
    }
}

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Window width
    pub width: f32,
    /// Window height
    pub height: f32,
    /// Left pane ratio (0.0-1.0)
    pub left_pane_ratio: f32,
}

/// Builder for WindowConfig
#[derive(Debug)]
pub struct WindowConfigBuilder {
    width: Option<f32>,
    height: Option<f32>,
    left_pane_ratio: Option<f32>,
}

impl Default for WindowConfigBuilder {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            left_pane_ratio: None,
        }
    }
}

impl WindowConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set window dimensions
    pub fn dimensions(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set window width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set window height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set left pane ratio
    pub fn left_pane_ratio(mut self, ratio: f32) -> Self {
        self.left_pane_ratio = Some(ratio.clamp(0.0, 1.0));
        self
    }

    /// Build the final WindowConfig
    pub fn build(self) -> WindowConfig {
        WindowConfig {
            width: self.width.unwrap_or(1200.0),
            height: self.height.unwrap_or(800.0),
            left_pane_ratio: self.left_pane_ratio.unwrap_or(0.3),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1200.0,
            height: 800.0,
            left_pane_ratio: 0.3,
        }
    }
}

/// UI configuration
#[derive(Debug, Clone)]
pub struct UiConfig {
    /// Theme preference
    pub theme: Theme,
    /// Font size
    pub font_size: f32,
    /// Show hidden files by default
    pub show_hidden: bool,
    /// Include directory tree in output
    pub include_tree: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            font_size: 12.0,
            show_hidden: false,
            include_tree: true,
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Maximum concurrent file reads
    pub max_concurrent_reads: usize,
    /// File cache size limit in MB
    pub cache_size_mb: usize,
    /// Use memory mapping for large files
    pub use_mmap: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_reads: 32,
            cache_size_mb: 100,
            use_mmap: false,
        }
    }
}

/// UI Theme options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// Follow system
    #[default]
    System,
}

// ===== Toast Notifications =====

/// Type-safe toast notifications
#[derive(Debug, Clone)]
pub struct Toast {
    /// Toast variant
    pub variant: ToastVariant,
    /// Display duration in seconds
    pub duration_secs: u8,
}

/// Toast variants with associated data
#[derive(Debug, Clone)]
pub enum ToastVariant {
    /// Success message
    Success(String),
    /// Warning message
    Warning(String),
    /// Error with optional details
    Error {
        /// Error message
        message: String,
        /// Optional error details
        details: Option<String>,
    },
    /// Progress notification
    Progress {
        /// Progress message
        message: String,
        /// Progress percentage (0-100)
        percentage: f32,
    },
}

impl Toast {
    /// Create a success toast
    #[must_use]
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            variant: ToastVariant::Success(message.into()),
            duration_secs: 2,
        }
    }

    /// Create an error toast
    #[must_use]
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            variant: ToastVariant::Error {
                message: message.into(),
                details: None,
            },
            duration_secs: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_count() {
        let tokens = TokenCount::from_chars(4000);
        assert_eq!(tokens.get(), 1000);
        assert_eq!(tokens.level(), TokenLevel::Medium);
    }

    #[test]
    fn test_file_size_strategy() {
        let small = FileSize::from_bytes(1024);
        assert!(matches!(small.read_strategy(), FileReadStrategy::Direct));

        let large = FileSize::from_bytes(512 * 1024);
        assert!(matches!(
            large.read_strategy(),
            FileReadStrategy::MemoryMapped
        ));
    }

    #[test]
    fn test_selection_tracker() {
        let tracker = SelectionTracker::default();
        assert!(tracker.selected.is_empty());

        // Would add more tests here for checkpoint/undo/redo
    }

    #[test]
    fn test_app_config_builder() {
        let config = AppConfigBuilder::new()
            .add_ignore_pattern("*.log".to_string())
            .add_ignore_pattern("tmp/".to_string())
            .build();

        assert!(config.ignore_patterns.contains(&"*.log".to_string()));
        assert!(config.ignore_patterns.contains(&"tmp/".to_string()));
        assert!(config.ignore_patterns.contains(&"node_modules".to_string())); // default
    }

    #[test]
    fn test_window_config_builder() {
        let window = WindowConfigBuilder::new()
            .dimensions(1920.0, 1080.0)
            .left_pane_ratio(0.4)
            .build();

        assert_eq!(window.width, 1920.0);
        assert_eq!(window.height, 1080.0);
        assert_eq!(window.left_pane_ratio, 0.4);
    }

    #[test]
    fn test_window_config_builder_clamps_ratio() {
        let window = WindowConfigBuilder::new()
            .left_pane_ratio(1.5) // Out of range
            .build();

        assert_eq!(window.left_pane_ratio, 1.0); // Clamped to max
    }
}
