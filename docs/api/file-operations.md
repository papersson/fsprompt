# File Operations API Reference

Complete reference for parallel filesystem utilities and operations in fsPrompt (`src/utils/parallel_fs.rs`).

## Overview

fsPrompt provides high-performance, parallel filesystem operations designed for scanning large codebases efficiently. The utilities include parallel directory scanning, secure file reading with memory mapping, pattern-based filtering, and hierarchical tree building. All operations include built-in security measures to prevent path traversal attacks.

## Core Types

### `DirectoryEntry`

Represents a single filesystem entry from a parallel directory scan.

```rust
pub struct DirectoryEntry {
    pub path: CanonicalPath,
    pub is_dir: bool,
    pub name: String,
    pub parent: Option<CanonicalPath>,
}
```

#### Fields

- `path: CanonicalPath` - The full canonical path to the entry
- `is_dir: bool` - Whether this is a directory
- `name: String` - The file/directory name
- `parent: Option<CanonicalPath>` - Parent directory path (if any)

#### Usage Example

```rust
use crate::utils::parallel_fs::{scan_directory_parallel, DirectoryEntry};

let entries = scan_directory_parallel(
    Path::new("/project/root"),
    Some(3), // max depth
    &["*.log".to_string(), "node_modules".to_string()], // ignore patterns
);

for entry in entries {
    println!("{} {} ({})", 
        if entry.is_dir { "📁" } else { "📄" },
        entry.name,
        entry.path.as_path().display()
    );
}
```

## Directory Scanning

### `scan_directory_parallel`

Performs a parallel directory scan up to a specified depth with ignore patterns.

```rust
pub fn scan_directory_parallel(
    root: &Path,
    max_depth: Option<usize>,
    ignore_patterns: &[String],
) -> Vec<DirectoryEntry>
```

#### Parameters

- `root: &Path` - Root directory to scan
- `max_depth: Option<usize>` - Maximum depth to traverse (None for unlimited)
- `ignore_patterns: &[String]` - Patterns to ignore (glob format)

#### Features

- **Parallel Processing**: Uses up to 8 threads for optimal performance
- **Path Validation**: Ensures all paths are within the root directory
- **Pattern Filtering**: Supports glob patterns for ignoring files/directories
- **Depth Limiting**: Prevents runaway recursion with configurable depth limits
- **Security**: Built-in path traversal protection

#### Usage Example

```rust
use std::path::Path;
use crate::utils::parallel_fs::scan_directory_parallel;

// Scan with depth limit and ignore patterns
let entries = scan_directory_parallel(
    Path::new("/home/user/project"),
    Some(5), // Don't go deeper than 5 levels
    &[
        "target".to_string(),      // Rust build artifacts
        "node_modules".to_string(), // NPM dependencies
        "*.log".to_string(),       // Log files
        ".git".to_string(),        // Git directory
        "__pycache__".to_string(), // Python cache
    ]
);

println!("Found {} entries", entries.len());

// Filter for specific types
let rust_files: Vec<_> = entries.iter()
    .filter(|e| !e.is_dir && e.name.ends_with(".rs"))
    .collect();

println!("Found {} Rust files", rust_files.len());
```

#### Performance Characteristics

- **Thread Pool**: Uses `num_cpus::get().min(8)` threads
- **Memory Usage**: Efficient streaming with bounded memory usage
- **Large Directories**: Handles directories with thousands of entries
- **Network Filesystems**: Optimized for both local and network storage

## Tree Building

### `build_tree_from_entries`

Builds a hierarchical tree structure from flat directory entries.

```rust
pub fn build_tree_from_entries(
    entries: Vec<DirectoryEntry>,
) -> HashMap<CanonicalPath, Vec<DirectoryEntry>>
```

#### Parameters

- `entries: Vec<DirectoryEntry>` - Flat list of directory entries

#### Returns

- `HashMap<CanonicalPath, Vec<DirectoryEntry>>` - Tree structure mapping parent directories to their children

#### Features

- **Automatic Sorting**: Children are sorted with directories first, then alphabetically
- **Hierarchical Organization**: Groups entries by parent directory
- **Efficient Lookup**: HashMap provides O(1) parent-to-children lookup

#### Usage Example

```rust
use crate::utils::parallel_fs::{scan_directory_parallel, build_tree_from_entries};
use std::collections::HashMap;

// Scan directory
let entries = scan_directory_parallel(
    Path::new("/project/src"),
    Some(3),
    &["*.o".to_string(), "*.tmp".to_string()]
);

// Build tree structure
let tree = build_tree_from_entries(entries);

// Navigate tree
for (parent_path, children) in &tree {
    println!("📁 {}/", parent_path.as_path().display());
    
    for child in children {
        let icon = if child.is_dir { "📁" } else { "📄" };
        println!("  {} {}", icon, child.name);
    }
}
```

## File Reading

### `read_files_parallel`

Reads multiple files in parallel with automatic strategy selection based on file size.

```rust
pub fn read_files_parallel(
    file_paths: &[CanonicalPath],
    use_mmap_threshold: usize,
) -> Vec<(CanonicalPath, Result<String, String>)>
```

#### Parameters

- `file_paths: &[CanonicalPath]` - List of files to read
- `use_mmap_threshold: usize` - Size threshold for using memory mapping (in bytes)

#### Returns

- `Vec<(CanonicalPath, Result<String, String>)>` - Results with path and either content or error message

#### Usage Example

```rust
use crate::utils::parallel_fs::read_files_parallel;
use crate::core::types::CanonicalPath;

let files = vec![
    CanonicalPath::new("/project/src/main.rs")?,
    CanonicalPath::new("/project/src/lib.rs")?,
    CanonicalPath::new("/project/README.md")?,
];

// Read files with 256KB threshold for memory mapping
let results = read_files_parallel(&files, 256 * 1024);

for (path, result) in results {
    match result {
        Ok(content) => {
            println!("✓ {} ({} bytes)", 
                path.as_path().display(), 
                content.len()
            );
        }
        Err(error) => {
            eprintln!("✗ {}: {}", 
                path.as_path().display(), 
                error
            );
        }
    }
}
```

### `read_files_parallel_secure`

Secure version of parallel file reading with additional path validation.

```rust
pub fn read_files_parallel_secure(
    file_paths: &[CanonicalPath],
    root: &CanonicalPath,
    use_mmap_threshold: usize,
) -> Vec<(CanonicalPath, Result<String, String>)>
```

#### Parameters

- `file_paths: &[CanonicalPath]` - List of files to read
- `root: &CanonicalPath` - Root directory for validation
- `use_mmap_threshold: usize` - Size threshold for using memory mapping

#### Security Features

- **Path Traversal Protection**: Validates all paths are within the root directory
- **Path Canonicalization**: Resolves symlinks and normalizes paths
- **Access Control**: Prevents reading files outside the allowed directory tree

#### Usage Example

```rust
use crate::utils::parallel_fs::read_files_parallel_secure;

let root = CanonicalPath::new("/project/root")?;
let files = vec![
    CanonicalPath::new("/project/root/src/main.rs")?,
    // This would be rejected:
    // CanonicalPath::new("/etc/passwd")?,
];

let results = read_files_parallel_secure(&files, &root, 256 * 1024);

// All results are guaranteed to be from within the root directory
for (path, result) in results {
    match result {
        Ok(content) => println!("Read: {}", path.as_path().display()),
        Err(error) => eprintln!("Failed: {}", error),
    }
}
```

## Pattern Matching

### `PatternCache`

High-performance pattern cache for efficient glob matching.

```rust
pub struct PatternCache {
    globs: Vec<glob::Pattern>,
    regexes: Vec<regex::Regex>,
}
```

#### Methods

```rust
impl PatternCache {
    /// Create a new pattern cache from glob patterns
    pub fn new(patterns: &[String]) -> Self
    
    /// Check if a path matches any pattern
    pub fn matches(&self, path: &str) -> bool
}
```

#### Features

- **Dual Strategy**: Uses both glob patterns and regex for maximum compatibility
- **Compiled Patterns**: Pre-compiles patterns for fast matching
- **Fallback Support**: Automatically converts between pattern types
- **Performance Optimization**: Optimized for repeated matching operations

#### Usage Example

```rust
use crate::utils::parallel_fs::PatternCache;

// Create pattern cache
let patterns = vec![
    "*.rs".to_string(),        // Rust files
    "target/**".to_string(),   // Build directory
    "*.{txt,md}".to_string(),  // Documentation
    ".git".to_string(),        // Git directory
];

let cache = PatternCache::new(&patterns);

// Test matches
assert!(cache.matches("main.rs"));           // *.rs
assert!(cache.matches("target/debug/app"));  // target/**
assert!(cache.matches("README.md"));         // *.{txt,md}
assert!(cache.matches(".git"));              // .git
assert!(!cache.matches("main.py"));          // No match
```

## Memory Mapping

### `read_file_mmap`

Internal function for memory-mapped file reading (used automatically by parallel readers).

```rust
fn read_file_mmap(path: &Path) -> Result<String, String>
```

#### Features

- **Large File Support**: Efficiently handles files larger than available RAM
- **UTF-8 Validation**: Ensures content is valid UTF-8
- **Error Handling**: Provides detailed error messages for debugging
- **Cross-Platform**: Works on Linux, macOS, and Windows

#### Automatic Selection

The parallel file readers automatically choose between standard reading and memory mapping:

```rust
// Automatic strategy selection
let result = if metadata.len() as usize > use_mmap_threshold {
    // Use memory-mapped reading for large files
    read_file_mmap(path.as_path())
} else {
    // Use standard reading for small files
    std::fs::read_to_string(path.as_path())
        .map_err(|e| format!("Failed to read file: {}", e))
};
```

## Integration Patterns

### Worker Integration

```rust
use crate::utils::parallel_fs::{scan_directory_parallel, read_files_parallel_secure};
use crate::workers::{WorkerEvent, ProgressStage, ProgressCount};

fn generate_output_worker(
    root_path: CanonicalPath,
    selected_files: Vec<CanonicalPath>,
    event_tx: Sender<WorkerEvent>,
) {
    // Phase 1: Scan for tree generation
    let _ = event_tx.send(WorkerEvent::Progress {
        stage: ProgressStage::ScanningFiles,
        progress: ProgressCount::new(0, 1),
    });
    
    let entries = scan_directory_parallel(
        root_path.as_path(),
        Some(10), // reasonable depth limit
        &["target".to_string(), "node_modules".to_string()]
    );
    
    // Phase 2: Read selected files
    let _ = event_tx.send(WorkerEvent::Progress {
        stage: ProgressStage::ReadingFiles,
        progress: ProgressCount::new(0, selected_files.len()),
    });
    
    let file_contents = read_files_parallel_secure(
        &selected_files,
        &root_path,
        256 * 1024 // 256KB threshold
    );
    
    // Process results...
}
```

### Configuration-Driven Scanning

```rust
use crate::core::types::PerformanceConfig;

struct FileScanner {
    config: PerformanceConfig,
}

impl FileScanner {
    fn scan_with_config(&self, root: &Path, ignore_patterns: &[String]) -> Vec<DirectoryEntry> {
        scan_directory_parallel(
            root,
            Some(8), // configurable depth limit
            ignore_patterns
        )
    }
    
    fn read_files_with_config(&self, files: &[CanonicalPath], root: &CanonicalPath) -> Vec<(CanonicalPath, Result<String, String>)> {
        let threshold = if self.config.use_mmap {
            64 * 1024  // 64KB threshold when mmap enabled
        } else {
            usize::MAX // Disable mmap
        };
        
        read_files_parallel_secure(files, root, threshold)
    }
}
```

### Progress Reporting

```rust
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

fn read_files_with_progress(
    files: &[CanonicalPath],
    root: &CanonicalPath,
    progress_callback: impl Fn(usize, usize) + Send + Sync,
) -> Vec<(CanonicalPath, Result<String, String>)> {
    let processed = Arc::new(AtomicUsize::new(0));
    let total_files = files.len();
    
    files
        .par_iter()
        .map(|path| {
            let result = read_file_secure(path, root);
            
            // Report progress
            let current = processed.fetch_add(1, Ordering::Relaxed) + 1;
            progress_callback(current, total_files);
            
            (path.clone(), result)
        })
        .collect()
}
```

## Error Handling

### Common Error Types

```rust
// File reading errors
match result {
    Ok(content) => {
        // Process successful read
    }
    Err(error) => {
        if error.contains("Permission denied") {
            // Handle permission errors
        } else if error.contains("No such file") {
            // Handle missing files
        } else if error.contains("UTF-8 error") {
            // Handle binary files
        } else if error.contains("Security error") {
            // Handle path traversal attempts
        } else {
            // Handle other I/O errors
        }
    }
}
```

### Robust File Processing

```rust
fn process_files_robustly(files: &[CanonicalPath], root: &CanonicalPath) {
    let results = read_files_parallel_secure(files, root, 256 * 1024);
    let mut successful_files = Vec::new();
    let mut failed_files = Vec::new();
    
    for (path, result) in results {
        match result {
            Ok(content) => {
                successful_files.push((path, content));
            }
            Err(error) => {
                failed_files.push((path, error));
                eprintln!("Failed to read {}: {}", path.as_path().display(), error);
            }
        }
    }
    
    println!("Successfully read {} files", successful_files.len());
    if !failed_files.is_empty() {
        println!("Failed to read {} files", failed_files.len());
        
        // Optionally retry failed files with different strategy
        retry_failed_files(&failed_files, root);
    }
}
```

## Performance Optimization

### Tuning Parameters

```rust
// Optimal thread count for your system
let thread_count = num_cpus::get().min(8);

// Memory mapping threshold based on available RAM
let available_memory = get_available_memory();
let mmap_threshold = (available_memory / 100).max(64 * 1024); // 1% of RAM, min 64KB

// Depth limit based on expected directory structure
let depth_limit = match project_type {
    ProjectType::SmallLibrary => 5,
    ProjectType::LargeMonorepo => 10,
    ProjectType::DeepNested => 15,
};
```

### Batch Processing

```rust
fn process_large_fileset(all_files: &[CanonicalPath], root: &CanonicalPath) {
    const BATCH_SIZE: usize = 100;
    
    for batch in all_files.chunks(BATCH_SIZE) {
        let results = read_files_parallel_secure(batch, root, 256 * 1024);
        
        // Process batch results
        for (path, result) in results {
            // Handle each file...
        }
        
        // Optional: yield to other tasks between batches
        std::thread::yield_now();
    }
}
```

### Memory Management

```rust
fn scan_with_memory_limit(root: &Path, max_entries: usize) -> Vec<DirectoryEntry> {
    let mut all_entries = scan_directory_parallel(
        root,
        Some(8),
        &["target".to_string(), "node_modules".to_string()]
    );
    
    // Limit memory usage by truncating results if needed
    if all_entries.len() > max_entries {
        all_entries.truncate(max_entries);
        eprintln!("Warning: Truncated results to {} entries", max_entries);
    }
    
    all_entries
}
```

## Testing Utilities

### Test Helpers

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_structure() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test directory structure
        fs::create_dir(root.join("src")).unwrap();
        fs::create_dir(root.join("target")).unwrap();
        fs::write(root.join("Cargo.toml"), "test content").unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(root.join("target/debug"), "binary data").unwrap();

        temp_dir
    }

    #[test]
    fn test_parallel_scan() {
        let temp_dir = create_test_structure();
        let entries = scan_directory_parallel(
            temp_dir.path(),
            Some(3),
            &["target".to_string()]
        );

        // Should find root, src/, Cargo.toml, src/main.rs (target filtered out)
        assert!(entries.len() >= 3);
        
        let has_cargo_toml = entries.iter()
            .any(|e| e.name == "Cargo.toml");
        assert!(has_cargo_toml);

        let has_target = entries.iter()
            .any(|e| e.name == "target");
        assert!(!has_target); // Should be filtered out
    }
}
```