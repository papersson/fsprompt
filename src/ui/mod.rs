//! User interface components for fsPrompt

pub mod app_ui;
/// Theme and styling constants
pub mod theme;
pub mod toast;
pub mod tree;

pub use crate::core::types::OutputFormat;
pub use theme::{BgLevel, TextEmphasis, Theme};

use std::path::Path;

/// Generate a tree string representation of a directory
pub fn generate_tree_string(root_path: &Path) -> String {
    let mut output = String::new();
    generate_tree_recursive(root_path, &mut output, "", true, 0);
    output
}

fn generate_tree_recursive(
    path: &Path,
    output: &mut String,
    prefix: &str,
    is_last: bool,
    depth: usize,
) {
    const MAX_DEPTH: usize = 10;

    // Prevent infinite recursion
    if depth > MAX_DEPTH {
        return;
    }

    // Get the file/folder name
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or("?"));

    // Add the current node
    let connector = if is_last { "└── " } else { "├── " };
    let icon = if path.is_dir() { "📁" } else { "📄" };

    output.push_str(prefix);
    output.push_str(connector);
    output.push_str(icon);
    output.push(' ');
    output.push_str(name);
    output.push('\n');

    // Process directory children
    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();

            // Sort entries: directories first, then alphabetically
            entries.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            });

            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            let entry_count = entries.len();

            for (index, entry) in entries.iter().enumerate() {
                let is_last_child = index == entry_count - 1;
                generate_tree_recursive(&entry, output, &new_prefix, is_last_child, depth + 1);
            }
        }
    }
}
