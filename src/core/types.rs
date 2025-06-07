#![warn(missing_docs)]

//! Redesigned type system for fsPrompt with improved expressiveness and type safety

use std::collections::HashSet;
use std::marker::PhantomData;
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
}

/// Token count with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    File { size: FileSize },
    /// Directory
    Directory,
    /// Symbolic link (not followed)
    Symlink { target: PathBuf },
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

/// Loading state with phantom type for compile-time state tracking
#[derive(Debug)]
pub struct LoadingState<S> {
    _phantom: PhantomData<S>,
}

/// Marker types for loading states
pub mod loading {
    /// Not yet loaded
    pub struct NotLoaded;
    /// Currently loading
    pub struct Loading;
    /// Successfully loaded
    pub struct Loaded;
    /// Failed to load
    pub struct Failed;
}

/// UI node that combines filesystem data with UI state
#[derive(Debug, Clone)]
pub struct UiNode {
    /// The filesystem entry
    pub entry: FsEntry,
    /// Selection state
    pub selection: SelectionState,
    /// Whether this node matches current search
    pub matches_search: bool,
    /// Children nodes (for directories)
    pub children: Vec<UiNode>,
    /// Loading state for children
    pub children_loaded: bool,
}

// ===== Thread Communication Types =====

/// Messages that can be sent to worker threads
#[derive(Debug, Clone)]
pub enum WorkerRequest {
    /// Scan a directory for entries
    ScanDirectory {
        /// Directory to scan
        path: CanonicalPath,
        /// Include hidden files
        include_hidden: bool,
    },
    /// Generate output from selected files
    GenerateOutput {
        /// Root directory
        root: CanonicalPath,
        /// Selected paths
        selections: Arc<HashSet<CanonicalPath>>,
        /// Output configuration
        config: OutputConfig,
    },
    /// Cancel current operation
    Cancel,
}

/// Output generation configuration
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// Output format
    pub format: OutputFormat,
    /// Ignore patterns
    pub ignore_patterns: Arc<Vec<IgnorePattern>>,
    /// Include file contents
    pub include_contents: bool,
    /// Maximum file size to include
    pub max_file_size: Option<FileSize>,
}

/// Compiled ignore pattern
#[derive(Clone)]
pub struct IgnorePattern {
    /// Original pattern string
    pub pattern: String,
    /// Compiled pattern (opaque to avoid exposing regex)
    compiled: Arc<dyn Fn(&Path) -> bool + Send + Sync>,
}

impl std::fmt::Debug for IgnorePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IgnorePattern")
            .field("pattern", &self.pattern)
            .field("compiled", &"<compiled>")
            .finish()
    }
}

/// Worker thread responses
#[derive(Debug, Clone)]
pub enum WorkerResponse {
    /// Directory entries found
    DirectoryEntries {
        /// Path that was scanned
        path: CanonicalPath,
        /// Found entries
        entries: Vec<FsEntry>,
    },
    /// Progress update
    Progress(ProgressUpdate),
    /// Output generated
    OutputReady {
        /// Generated content
        content: Arc<String>,
        /// Token count
        tokens: TokenCount,
        /// Generation time in milliseconds
        generation_time_ms: u32,
    },
    /// Error occurred
    Error(WorkerError),
}

/// Worker errors with specific variants
#[derive(Debug, Clone)]
pub enum WorkerError {
    /// I/O error with path context
    Io { path: PathBuf, error: String },
    /// Invalid UTF-8 in file
    InvalidUtf8 { path: PathBuf },
    /// Pattern compilation failed
    InvalidPattern { pattern: String, error: String },
    /// Directory not found
    NotFound { path: PathBuf },
    /// Permission denied
    PermissionDenied { path: PathBuf },
    /// Operation cancelled
    Cancelled,
}

/// Progress update with detailed information
#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    /// Current stage
    pub stage: ProgressStage,
    /// Items completed
    pub completed: usize,
    /// Total items (if known)
    pub total: Option<usize>,
    /// Current item being processed
    pub current_item: Option<String>,
}

impl ProgressUpdate {
    /// Calculate percentage if total is known
    #[must_use]
    pub fn percentage(&self) -> Option<f32> {
        self.total.map(|total| {
            if total == 0 {
                100.0
            } else {
                (self.completed as f32 / total as f32) * 100.0
            }
        })
    }
}

/// Progress stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressStage {
    /// Discovering files
    Discovery,
    /// Reading file contents
    Reading,
    /// Building output
    Formatting,
    /// Complete
    Complete,
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

// ===== Application State =====

/// Main application state with clear separation of concerns
#[derive(Debug)]
pub struct AppState {
    /// Current root directory
    pub root: Option<CanonicalPath>,
    /// UI tree representation
    pub tree: Option<UiNode>,
    /// Expanded directories
    pub expanded: HashSet<CanonicalPath>,
    /// Selection tracking
    pub selections: SelectionTracker,
    /// Search state
    pub search: SearchState,
    /// Output state
    pub output: OutputState,
    /// Worker communication
    pub worker: WorkerState,
    /// Application configuration
    pub config: AppConfig,
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

/// Search state
#[derive(Debug, Default)]
pub struct SearchState {
    /// Current search query
    pub query: String,
    /// Search results (matching paths)
    pub results: HashSet<CanonicalPath>,
    /// Is search active
    pub active: bool,
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

/// Worker thread state
#[derive(Debug)]
pub struct WorkerState {
    /// Channel sender to worker
    pub sender: crossbeam::channel::Sender<WorkerRequest>,
    /// Current task handle
    pub task_handle: Option<tokio::task::JoinHandle<()>>,
    /// Is task running
    pub busy: bool,
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

/// UI configuration
#[derive(Debug, Clone)]
pub struct UiConfig {
    /// Theme preference
    pub theme: Theme,
    /// Font size
    pub font_size: f32,
    /// Show hidden files by default
    pub show_hidden: bool,
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
        message: String,
        details: Option<String>,
    },
    /// Progress notification
    Progress { message: String, percentage: f32 },
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
    fn test_progress_percentage() {
        let progress = ProgressUpdate {
            stage: ProgressStage::Reading,
            completed: 50,
            total: Some(100),
            current_item: None,
        };
        assert_eq!(progress.percentage(), Some(50.0));

        let unknown = ProgressUpdate {
            stage: ProgressStage::Discovery,
            completed: 10,
            total: None,
            current_item: None,
        };
        assert_eq!(unknown.percentage(), None);
    }

    #[test]
    fn test_selection_tracker() {
        let tracker = SelectionTracker::default();
        assert!(tracker.selected.is_empty());

        // Would add more tests here for checkpoint/undo/redo
    }
}
