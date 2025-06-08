//! Directory tree UI component with lazy loading and tri-state selection

use eframe::egui;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use glob::Pattern;
use std::collections::{HashMap, HashSet};

use crate::core::types::{CanonicalPath, FileSize};
use crate::ui::Theme;

// Using SelectionState from core::types
pub use crate::core::types::SelectionState;

/// A node in the directory tree
#[derive(Debug)]
pub struct TreeNode {
    /// Canonical path to the file or directory
    pub canonical_path: CanonicalPath,
    /// Display name (file/folder name)
    pub name: String,
    /// Whether this is a directory
    pub is_dir: bool,
    /// Selection state
    pub selection: SelectionState,
    /// Whether the node is expanded (for directories)
    pub expanded: bool,
    /// Whether children have been loaded
    pub children_loaded: bool,
    /// Child nodes (lazy loaded)
    pub children: Vec<TreeNode>,
    /// File size if this is a file
    pub file_size: Option<FileSize>,
}

impl TreeNode {
    /// Creates a new tree node from a `CanonicalPath`
    ///
    /// # Errors
    ///
    /// Returns an error if the path metadata cannot be accessed
    pub fn new(canonical_path: CanonicalPath) -> std::io::Result<Self> {
        let name = canonical_path.file_name().map_or_else(
            || canonical_path.as_path().to_string_lossy().to_string(),
            |n| n.to_string_lossy().to_string(),
        );

        let is_dir = canonical_path.as_path().is_dir();

        // Get file size if it's a file
        let file_size = if is_dir {
            None
        } else {
            canonical_path
                .as_path()
                .metadata()
                .ok()
                .map(|m| FileSize::from_bytes(m.len()))
        };

        Ok(Self {
            canonical_path,
            name,
            is_dir,
            selection: SelectionState::Unchecked,
            expanded: false,
            children_loaded: false,
            children: Vec::new(),
            file_size,
        })
    }

    /// Loads children for this node if it's a directory
    pub fn load_children(&mut self) {
        self.load_children_with_patterns(&[]);
    }

    /// Loads children for this node with ignore patterns
    pub fn load_children_with_patterns(&mut self, ignore_patterns: &[glob::Pattern]) {
        if !self.is_dir || self.children_loaded {
            return;
        }

        self.children_loaded = true;

        if let Ok(entries) = std::fs::read_dir(self.canonical_path.as_path()) {
            let mut children: Vec<Self> = entries
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let path = entry.path();
                    let name = path.file_name()?.to_str()?;

                    // Check if this entry should be ignored
                    for pattern in ignore_patterns {
                        if pattern.matches(name) {
                            return None;
                        }
                    }

                    CanonicalPath::new(path)
                        .ok()
                        .and_then(|cp| Self::new(cp).ok())
                })
                .collect();

            // Sort directories first, then alphabetically
            children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });

            self.children = children;
        }
    }

    /// Loads all children recursively up to a maximum depth
    pub fn load_children_recursive(&mut self, current_depth: usize, max_depth: usize) {
        self.load_children_recursive_with_patterns(current_depth, max_depth, &[]);
    }

    /// Loads all children recursively up to a maximum depth with ignore patterns
    pub fn load_children_recursive_with_patterns(
        &mut self,
        current_depth: usize,
        max_depth: usize,
        ignore_patterns: &[glob::Pattern],
    ) {
        if !self.is_dir || current_depth >= max_depth {
            return;
        }

        // Load immediate children if not already loaded
        if !self.children_loaded {
            self.load_children_with_patterns(ignore_patterns);
        }

        // Expand this directory to show its contents
        self.expanded = true;

        // Recursively load children of subdirectories
        for child in &mut self.children {
            if child.is_dir {
                child.load_children_recursive_with_patterns(
                    current_depth + 1,
                    max_depth,
                    ignore_patterns,
                );
            }
        }
    }

    /// Updates selection state recursively
    pub fn set_selection(&mut self, state: SelectionState) {
        self.set_selection_with_patterns(state, &[]);
    }

    /// Updates selection state recursively with ignore patterns
    pub fn set_selection_with_patterns(&mut self, state: SelectionState, patterns: &[Pattern]) {
        self.selection = state;

        // If setting to checked/unchecked, propagate to all children
        if state != SelectionState::Indeterminate {
            // If this is a directory being checked, load all children recursively
            if state == SelectionState::Checked && self.is_dir {
                // Load all descendants up to 20 levels deep (reasonable limit)
                self.load_children_recursive_with_patterns(0, 20, patterns);
                // Also expand this node to show what was selected
                self.expanded = true;
            }

            for child in &mut self.children {
                child.set_selection_with_patterns(state, patterns);
            }
        }
    }

    /// Updates parent selection based on children
    pub fn update_parent_selection(&mut self) {
        if !self.is_dir || self.children.is_empty() {
            return;
        }

        let all_checked = self
            .children
            .iter()
            .all(|c| c.selection == SelectionState::Checked);
        let all_unchecked = self
            .children
            .iter()
            .all(|c| c.selection == SelectionState::Unchecked);

        self.selection = if all_checked {
            SelectionState::Checked
        } else if all_unchecked {
            SelectionState::Unchecked
        } else {
            SelectionState::Indeterminate
        };
    }

    /// Debug helper to print tree structure with selection states
    pub fn debug_tree(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let mut result = format!(
            "{}[{}] {} - {:?} (path: {})\n",
            indent,
            if self.is_dir { "DIR" } else { "FILE" },
            self.name,
            self.selection,
            self.canonical_path.as_path().display()
        );

        if self.is_dir {
            use std::fmt::Write;
            let _ = writeln!(
                result,
                "{indent}  (loaded: {}, expanded: {}, {} children)",
                self.children_loaded,
                self.expanded,
                self.children.len()
            );
        }

        for child in &self.children {
            result.push_str(&child.debug_tree(depth + 1));
        }
        result
    }
}

/// A flattened view of a tree node for efficient rendering
#[derive(Debug, Clone)]
struct FlattenedNode {
    /// Reference to the actual node (using index path)
    node_path: Vec<usize>,
    /// Depth in the tree (for indentation)
    depth: usize,
    /// Display name
    name: String,
    /// Whether this is a directory
    is_dir: bool,
    /// Whether the node is expanded
    is_expanded: bool,
    /// Selection state
    selection: SelectionState,
}

/// Directory tree widget
#[derive(Debug)]
pub struct DirectoryTree {
    /// Root nodes of the tree
    pub roots: Vec<TreeNode>,
    /// Map of path to node for quick lookups
    node_map: HashMap<CanonicalPath, usize>,
    /// Ignore patterns to filter files/directories
    ignore_patterns: Vec<Pattern>,
    /// Flattened view of visible nodes (cached)
    flattened_nodes: Vec<FlattenedNode>,
    /// Whether the flattened view needs rebuilding
    needs_flattening: bool,
}

impl DirectoryTree {
    /// Creates a new empty directory tree
    #[must_use]
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            node_map: HashMap::new(),
            ignore_patterns: Vec::new(),
            flattened_nodes: Vec::new(),
            needs_flattening: true,
        }
    }

    /// Sets the root directory for the tree
    pub fn set_root(&mut self, path: CanonicalPath) {
        self.roots.clear();
        self.node_map.clear();
        self.needs_flattening = true;

        if let Ok(mut root) = TreeNode::new(path) {
            root.expanded = true;
            root.load_children_with_patterns(&self.ignore_patterns);
            self.roots.push(root);
        }
    }

    /// Updates the ignore patterns from a comma-separated string
    pub fn set_ignore_patterns(&mut self, patterns_str: &str) {
        self.ignore_patterns = patterns_str
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|pattern| Pattern::new(pattern).ok())
            .collect();

        // Reload all expanded directories with new patterns
        if !self.roots.is_empty() {
            Self::reload_with_patterns(&mut self.roots[0], &self.ignore_patterns);
            self.needs_flattening = true;
        }
    }

    /// Recursively reloads expanded directories with new patterns
    fn reload_with_patterns(node: &mut TreeNode, patterns: &[Pattern]) {
        if node.is_dir && node.children_loaded {
            // Clear children and reload with patterns
            node.children.clear();
            node.children_loaded = false;
            node.load_children_with_patterns(patterns);

            // If node was expanded, reload children recursively
            if node.expanded {
                for child in &mut node.children {
                    if child.is_dir {
                        Self::reload_with_patterns(child, patterns);
                    }
                }
            }

            // Update selection state based on children
            node.update_parent_selection();
        }
    }

    /// Renders the tree UI
    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.show_with_search(ui, "");
    }

    /// Flattens the tree into a linear list of visible nodes
    fn flatten_tree(&mut self, search_query: &str) {
        self.flattened_nodes.clear();

        if self.roots.is_empty() {
            return;
        }

        let root = &self.roots[0];

        // If root's children aren't loaded yet, nothing to show
        if !root.children_loaded {
            return;
        }

        // If the directory is empty, nothing to show
        if root.children.is_empty() {
            return;
        }

        let matcher = if search_query.is_empty() {
            None
        } else {
            Some(SkimMatcherV2::default())
        };

        // Flatten each child of the root directly, skipping the root node itself
        for (index, child) in root.children.iter().enumerate() {
            Self::flatten_node_recursive(
                child,
                &mut self.flattened_nodes,
                &[0, index], // Path that includes root (0) and child index
                0,           // Start children at depth 0 for proper display
                search_query,
                matcher.as_ref(),
            );
        }

        self.needs_flattening = false;
    }

    /// Recursively flattens a node and its visible children
    fn flatten_node_recursive(
        node: &TreeNode,
        flattened: &mut Vec<FlattenedNode>,
        node_path: &[usize],
        depth: usize,
        search_query: &str,
        matcher: Option<&SkimMatcherV2>,
    ) {
        // Check if this node matches the search
        #[allow(clippy::unnecessary_map_or)]
        let should_show = matcher.map_or(true, |m| {
            m.fuzzy_match(&node.name, search_query).is_some()
                || (node.is_dir && Self::has_matching_child(node, search_query, m))
        });

        if !should_show {
            return;
        }

        // Add this node to the flattened list
        flattened.push(FlattenedNode {
            node_path: node_path.to_vec(),
            depth,
            name: node.name.clone(),
            is_dir: node.is_dir,
            is_expanded: node.expanded,
            selection: node.selection,
        });

        // If expanded, add children
        if node.is_dir && node.expanded && node.children_loaded {
            for (i, child) in node.children.iter().enumerate() {
                let mut child_path = node_path.to_vec();
                child_path.push(i);
                Self::flatten_node_recursive(
                    child,
                    flattened,
                    &child_path,
                    depth + 1,
                    search_query,
                    matcher,
                );
            }
        }
    }

    /// Gets a mutable reference to a node by its path
    fn get_node_by_path_mut(&mut self, path: &[usize]) -> Option<&mut TreeNode> {
        if path.is_empty() || self.roots.is_empty() {
            return None;
        }

        let mut current = &mut self.roots[0];

        for &index in &path[1..] {
            if index >= current.children.len() {
                return None;
            }
            current = &mut current.children[index];
        }

        Some(current)
    }

    /// Selects all files in the tree
    pub fn select_all(&mut self) {
        for root in &mut self.roots {
            Self::set_selection_recursive(root, SelectionState::Checked);
        }
        self.needs_flattening = true;
    }

    /// Deselects all files in the tree
    pub fn deselect_all(&mut self) {
        for root in &mut self.roots {
            Self::set_selection_recursive(root, SelectionState::Unchecked);
        }
        self.needs_flattening = true;
    }

    /// Recursively sets selection state
    fn set_selection_recursive(node: &mut TreeNode, state: SelectionState) {
        node.selection = state;
        if node.children_loaded {
            for child in &mut node.children {
                Self::set_selection_recursive(child, state);
            }
        }
    }

    /// Renders the tree UI with search filtering
    pub fn show_with_search(&mut self, ui: &mut egui::Ui, search_query: &str) {
        // Rebuild flattened view if needed
        if self.needs_flattening || !search_query.is_empty() {
            self.flatten_tree(search_query);
        }

        let total_rows = self.flattened_nodes.len();

        if total_rows == 0 {
            // Provide more specific feedback based on the state
            if self.roots.is_empty() {
                ui.label("No directory selected");
            } else {
                let root = &self.roots[0];
                if !root.children_loaded {
                    ui.label("Loading directory contents...");
                } else if root.children.is_empty() {
                    ui.label("Directory is empty");
                } else if !search_query.is_empty() {
                    ui.label("No files match your search");
                } else {
                    ui.label("No files to display");
                }
            }
            return;
        }

        // DEBUG: Add a visual indicator that we're about to render
        ui.label(format!("🌳 Tree with {total_rows} items"));
        ui.separator();

        // Use egui's built-in row virtualization for uniform height items
        let row_height = Theme::ROW_HEIGHT;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, total_rows, |ui, row_range| {
                let patterns = self.ignore_patterns.clone();
                let mut any_selection_changed = false;
                let mut any_expansion_changed = false;

                for row in row_range {
                    if row >= self.flattened_nodes.len() {
                        break;
                    }

                    let flat_node = self.flattened_nodes[row].clone();
                    #[allow(clippy::cast_precision_loss)]
                    let indent = flat_node.depth as f32 * Theme::INDENT_SIZE;

                    ui.horizontal(|ui| {
                        ui.add_space(indent);

                        // Expansion toggle for directories
                        if flat_node.is_dir {
                            let arrow = if flat_node.is_expanded { "▼" } else { "▶" };
                            if ui.small_button(arrow).clicked() {
                                if let Some(node) = self.get_node_by_path_mut(&flat_node.node_path)
                                {
                                    node.expanded = !node.expanded;
                                    if node.expanded && !node.children_loaded {
                                        node.load_children_with_patterns(&patterns);
                                    }
                                    any_expansion_changed = true;
                                }
                            }
                        } else {
                            // Spacer for files
                            ui.add_space(
                                ui.spacing().button_padding.x.mul_add(2.0, Theme::ICON_SIZE),
                            );
                        }

                        // Tri-state checkbox
                        let (mut checked, new_state) = match flat_node.selection {
                            SelectionState::Unchecked => (false, SelectionState::Checked),
                            SelectionState::Checked => (true, SelectionState::Unchecked),
                            SelectionState::Indeterminate => (true, SelectionState::Checked),
                        };

                        if ui.checkbox(&mut checked, "").clicked() {
                            if let Some(node) = self.get_node_by_path_mut(&flat_node.node_path) {
                                node.set_selection_with_patterns(new_state, &patterns);
                                any_selection_changed = true;
                            }
                        }

                        // Icon and name
                        let icon = if flat_node.is_dir { "📁" } else { "📄" };
                        ui.label(format!("{icon} {}", flat_node.name));
                    });
                }

                // Update parent states if selections changed
                if any_selection_changed && !self.roots.is_empty() {
                    Self::update_parent_states_recursive(&mut self.roots[0]);
                    self.needs_flattening = true;
                }

                // Mark for re-flattening if expansions changed
                if any_expansion_changed {
                    self.needs_flattening = true;
                }
            });
    }

    /// Updates parent selection states recursively based on children
    fn update_parent_states_recursive(node: &mut TreeNode) {
        if !node.is_dir || node.children.is_empty() {
            return;
        }

        // First, recursively update all child directories
        for child in &mut node.children {
            if child.is_dir {
                Self::update_parent_states_recursive(child);
            }
        }

        // Then update this node based on its children
        node.update_parent_selection();
    }

    /// Collects all selected file paths recursively
    pub fn collect_selected_files(&self) -> Vec<CanonicalPath> {
        let mut selected = Vec::new();
        for root in &self.roots {
            Self::collect_selected_from_node(root, &mut selected);
        }
        selected
    }

    /// Generates a string representation of the entire directory tree
    pub fn generate_tree_string(&self) -> String {
        let mut output = String::new();
        for root in &self.roots {
            Self::generate_tree_string_recursive(root, &mut output, "", true);
        }
        output
    }

    /// Recursively generates tree string with proper formatting
    fn generate_tree_string_recursive(
        node: &TreeNode,
        output: &mut String,
        prefix: &str,
        is_last: bool,
    ) {
        // Add the current node
        let connector = if is_last { "└── " } else { "├── " };
        let icon = if node.is_dir { "📁" } else { "📄" };

        output.push_str(prefix);
        output.push_str(connector);
        output.push_str(icon);
        output.push(' ');
        output.push_str(&node.name);
        output.push('\n');

        // Only process children if this is a directory with loaded children
        if node.is_dir && node.children_loaded && !node.children.is_empty() {
            let new_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });

            let child_count = node.children.len();
            for (index, child) in node.children.iter().enumerate() {
                let is_last_child = index == child_count - 1;
                Self::generate_tree_string_recursive(child, output, &new_prefix, is_last_child);
            }
        }
    }

    /// Helper function to collect selected files from a node recursively
    fn collect_selected_from_node(node: &TreeNode, selected: &mut Vec<CanonicalPath>) {
        match node.selection {
            SelectionState::Checked => {
                if node.is_dir {
                    // For directories, collect all files recursively
                    for child in &node.children {
                        Self::collect_selected_from_node(child, selected);
                    }
                } else {
                    // For files, add to the selected list
                    selected.push(node.canonical_path.clone());
                }
            }
            SelectionState::Indeterminate => {
                // For indeterminate directories, check children
                if node.is_dir {
                    for child in &node.children {
                        Self::collect_selected_from_node(child, selected);
                    }
                }
            }
            SelectionState::Unchecked => {
                // Skip unchecked nodes
            }
        }
    }

    /// Checks if a node has any children that match the search query
    fn has_matching_child(node: &TreeNode, search_query: &str, matcher: &SkimMatcherV2) -> bool {
        // If children aren't loaded yet, we can't check
        if !node.children_loaded {
            return true; // Assume there might be matches
        }

        for child in &node.children {
            // Check if this child matches
            if matcher.fuzzy_match(&child.name, search_query).is_some() {
                return true;
            }

            // Recursively check child directories
            if child.is_dir && Self::has_matching_child(child, search_query, matcher) {
                return true;
            }
        }

        false
    }

    /// Gets all selected file paths as a set
    pub fn get_selected_files(&self) -> HashSet<String> {
        let mut selected = HashSet::new();
        for root in &self.roots {
            Self::collect_selected_paths_recursive(root, &mut selected);
        }
        selected
    }

    /// Gets all expanded directory paths as a set
    pub fn get_expanded_dirs(&self) -> HashSet<String> {
        let mut expanded = HashSet::new();
        for root in &self.roots {
            Self::collect_expanded_paths_recursive(root, &mut expanded);
        }
        expanded
    }

    /// Restores selection and expansion state
    pub fn restore_selection(
        &mut self,
        selected_files: &HashSet<String>,
        expanded_dirs: &HashSet<String>,
    ) {
        for root in &mut self.roots {
            Self::restore_node_state_recursive(root, selected_files, expanded_dirs);
        }

        // Update parent states after restoring
        for root in &mut self.roots {
            Self::update_parent_states_recursive(root);
        }

        self.needs_flattening = true;
    }

    /// Collects selected file paths recursively
    fn collect_selected_paths_recursive(node: &TreeNode, selected: &mut HashSet<String>) {
        if node.selection == SelectionState::Checked && !node.is_dir {
            selected.insert(node.canonical_path.as_path().to_string_lossy().to_string());
        }

        for child in &node.children {
            Self::collect_selected_paths_recursive(child, selected);
        }
    }

    /// Collects expanded directory paths recursively
    fn collect_expanded_paths_recursive(node: &TreeNode, expanded: &mut HashSet<String>) {
        if node.is_dir && node.expanded {
            expanded.insert(node.canonical_path.as_path().to_string_lossy().to_string());
        }

        for child in &node.children {
            Self::collect_expanded_paths_recursive(child, expanded);
        }
    }

    /// Restores node state recursively
    fn restore_node_state_recursive(
        node: &mut TreeNode,
        selected_files: &HashSet<String>,
        expanded_dirs: &HashSet<String>,
    ) {
        let path_str = node.canonical_path.as_path().to_string_lossy().to_string();

        // Restore expansion state
        if node.is_dir && expanded_dirs.contains(&path_str) {
            node.expanded = true;
            // Load children if not already loaded
            if !node.children_loaded {
                node.load_children();
            }
        }

        // Restore selection state
        if !node.is_dir && selected_files.contains(&path_str) {
            node.selection = SelectionState::Checked;
        }

        // Recursively restore children
        for child in &mut node.children {
            Self::restore_node_state_recursive(child, selected_files, expanded_dirs);
        }
    }
}

impl Default for DirectoryTree {
    fn default() -> Self {
        Self::new()
    }
}
