//! Parallel filesystem operations for improved performance

use crate::core::types::CanonicalPath;
use ignore::{overrides::OverrideBuilder, WalkBuilder, WalkState};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Result of a parallel directory scan
#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    /// The full canonical path to the entry
    pub path: CanonicalPath,
    /// Whether this is a directory
    pub is_dir: bool,
    /// The file/directory name
    pub name: String,
    /// Parent directory path (if any)
    pub parent: Option<CanonicalPath>,
}

/// Performs a parallel directory scan up to a specified depth
///
/// # Errors
///
/// Returns an empty vector if the root path cannot be canonicalized
pub fn scan_directory_parallel(
    root: &Path,
    max_depth: Option<usize>,
    ignore_patterns: &[String],
) -> Vec<DirectoryEntry> {
    // Create canonical root for path validation
    let Ok(canonical_root) = CanonicalPath::new(root) else {
        return Vec::new(); // Return empty if root is invalid
    };

    let entries = Arc::new(Mutex::new(Vec::new()));
    let entries_clone = Arc::clone(&entries);

    let mut builder = WalkBuilder::new(root);

    // Configure the walker
    builder
        .standard_filters(false) // Don't use .gitignore by default
        .hidden(false) // Show hidden files
        .parents(false) // Don't look for .gitignore in parent dirs
        .follow_links(false) // Don't follow symlinks
        .threads(num_cpus::get().min(8)); // Use up to 8 threads

    if let Some(depth) = max_depth {
        builder.max_depth(Some(depth));
    }

    // Add ignore patterns
    if !ignore_patterns.is_empty() {
        let mut override_builder = OverrideBuilder::new(root);
        for pattern in ignore_patterns {
            if let Err(_e) = override_builder.add(&format!("!{pattern}")) {
                // Silently ignore invalid patterns to avoid debug output
            }
        }
        if let Ok(overrides) = override_builder.build() {
            builder.overrides(overrides);
        }
    }

    let walker = builder.build_parallel();

    walker.run(|| {
        let entries = Arc::clone(&entries_clone);
        let canonical_root = canonical_root.clone();
        Box::new(move |result| {
            if let Ok(entry) = result {
                let path_buf = entry.path().to_path_buf();
                // Validate path is within root to prevent traversal attacks
                if let Ok(canonical_path) =
                    CanonicalPath::new_within_root(&path_buf, &canonical_root)
                {
                    let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());
                    let name = entry.file_name().to_string_lossy().into_owned();
                    let parent = path_buf
                        .parent()
                        .and_then(|p| CanonicalPath::new_within_root(p, &canonical_root).ok());

                    let dir_entry = DirectoryEntry {
                        path: canonical_path,
                        is_dir,
                        name,
                        parent,
                    };

                    if let Ok(mut entries) = entries.lock() {
                        entries.push(dir_entry);
                    }
                }
            }
            WalkState::Continue
        })
    });

    entries
        .lock()
        .map_or_else(|_| Vec::new(), |entries| entries.clone())
}

/// Builds a hierarchical tree structure from flat entries
pub fn build_tree_from_entries(
    entries: Vec<DirectoryEntry>,
) -> HashMap<CanonicalPath, Vec<DirectoryEntry>> {
    let mut tree: HashMap<CanonicalPath, Vec<DirectoryEntry>> = HashMap::new();

    // Group entries by parent
    for entry in entries {
        if let Some(parent) = &entry.parent {
            tree.entry(parent.clone()).or_default().push(entry);
        }
    }

    // Sort children in each group (directories first, then alphabetically)
    for children in tree.values_mut() {
        children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
    }

    tree
}

/// Read multiple files in parallel with path validation
/// Validates all paths are within the root directory before reading
///
/// # Errors
///
/// Returns errors for:
/// - Path traversal attempts (security error)
/// - File read failures
/// - UTF-8 decoding errors
pub fn read_files_parallel_secure(
    file_paths: &[CanonicalPath],
    root: &CanonicalPath,
    use_mmap_threshold: usize,
) -> Vec<(CanonicalPath, Result<String, String>)> {
    file_paths
        .par_iter()
        .map(|path| {
            // Validate path is within root
            if !path.is_contained_within(root) {
                return (
                    path.clone(),
                    Err("Security error: path traversal detected".to_string()),
                );
            }

            let result = std::fs::metadata(path.as_path()).map_or_else(
                |_| {
                    Err(format!(
                        "Failed to get metadata for {}: file may not exist or be inaccessible",
                        path.as_path().display()
                    ))
                },
                |metadata| {
                    if usize::try_from(metadata.len()).unwrap_or(usize::MAX) > use_mmap_threshold {
                        // Use memory-mapped reading for large files
                        read_file_mmap(path.as_path())
                    } else {
                        // Use standard reading for small files
                        std::fs::read_to_string(path.as_path())
                            .map_err(|e| format!("Failed to read file: {e}"))
                    }
                },
            );

            (path.clone(), result)
        })
        .collect()
}

/// Parallel file reading with memory mapping for large files
///
/// # Errors
///
/// Returns errors for:
/// - File read failures
/// - UTF-8 decoding errors
pub fn read_files_parallel(
    file_paths: &[CanonicalPath],
    use_mmap_threshold: usize,
) -> Vec<(CanonicalPath, Result<String, String>)> {
    file_paths
        .par_iter()
        .map(|path| {
            let result = std::fs::metadata(path.as_path()).map_or_else(
                |_| {
                    Err(format!(
                        "Failed to get metadata for {}: file may not exist or be inaccessible",
                        path.as_path().display()
                    ))
                },
                |metadata| {
                    if usize::try_from(metadata.len()).unwrap_or(usize::MAX) > use_mmap_threshold {
                        // Use memory-mapped reading for large files
                        read_file_mmap(path.as_path())
                    } else {
                        // Use standard reading for small files
                        std::fs::read_to_string(path.as_path())
                            .map_err(|e| format!("Failed to read file: {e}"))
                    }
                },
            );

            (path.clone(), result)
        })
        .collect()
}

/// Read a file using memory mapping
///
/// # Errors
///
/// Returns errors for:
/// - File open failures
/// - Memory mapping failures
/// - UTF-8 decoding errors
fn read_file_mmap(path: &Path) -> Result<String, String> {
    use memmap2::Mmap;
    use std::fs::File;

    let file =
        File::open(path).map_err(|e| format!("Failed to open file for memory mapping: {e}"))?;
    let mmap =
        unsafe { Mmap::map(&file) }.map_err(|e| format!("Failed to create memory map: {e}"))?;

    // Convert to string, handling UTF-8 errors
    String::from_utf8(mmap.to_vec()).map_err(|e| format!("UTF-8 error: {e}"))
}

/// Pattern cache for improved glob matching performance
pub struct PatternCache {
    /// Compiled glob patterns
    globs: Vec<glob::Pattern>,
    /// Compiled regex patterns (as alternative)
    regexes: Vec<regex::Regex>,
}

impl PatternCache {
    /// Create a new pattern cache from glob patterns
    pub fn new(patterns: &[String]) -> Self {
        let globs = patterns
            .iter()
            .filter_map(|p| glob::Pattern::new(p).ok())
            .collect();

        let regexes = patterns
            .iter()
            .filter_map(|p| {
                // Convert glob to regex
                let regex_str = p
                    .replace('.', "\\.")
                    .replace('*', "[^/]*")
                    .replace("**", ".*")
                    .replace('{', "(")
                    .replace('}', ")")
                    .replace(',', "|");
                regex::Regex::new(&format!("^{regex_str}$")).ok()
            })
            .collect();

        Self { globs, regexes }
    }

    /// Check if a path matches any pattern
    pub fn matches(&self, path: &str) -> bool {
        // Try glob patterns first
        for pattern in &self.globs {
            if pattern.matches(path) {
                return true;
            }
        }

        // Fall back to regex if needed
        for pattern in &self.regexes {
            if pattern.is_match(path) {
                return true;
            }
        }

        false
    }
}

impl std::fmt::Debug for PatternCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PatternCache")
            .field("globs", &format!("{} patterns", self.globs.len()))
            .field("regexes", &format!("{} patterns", self.regexes.len()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parallel_scan() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test structure
        fs::create_dir(root.join("dir1")).unwrap();
        fs::create_dir(root.join("dir2")).unwrap();
        fs::write(root.join("file1.txt"), "content").unwrap();
        fs::write(root.join("dir1/file2.txt"), "content").unwrap();

        let entries = scan_directory_parallel(root, Some(2), &[]);

        assert!(entries.len() >= 4); // root + 2 dirs + 2 files

        let tree = build_tree_from_entries(entries);
        // Check if any entry has root as parent
        let root_canonical = CanonicalPath::new(root).unwrap();
        assert!(tree.contains_key(&root_canonical));
    }

    #[test]
    fn test_pattern_cache() {
        let patterns = vec![
            "*.rs".to_string(),
            "target/**".to_string(),
            "*.{txt,md}".to_string(),
        ];

        let cache = PatternCache::new(&patterns);

        assert!(cache.matches("main.rs"));
        assert!(cache.matches("target/debug/build"));
        assert!(cache.matches("readme.txt"));
        assert!(cache.matches("doc.md"));
        assert!(!cache.matches("main.py"));
    }
}
