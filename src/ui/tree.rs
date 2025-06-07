//! Directory tree UI component with lazy loading and tri-state selection

use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;

/// Selection state for tree items
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionState {
    /// Not selected
    Unchecked,
    /// Partially selected (some children selected)
    Indeterminate,
    /// Fully selected
    Checked,
}

/// A node in the directory tree
#[derive(Debug)]
pub struct TreeNode {
    /// Path to the file or directory
    pub path: PathBuf,
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
}

impl TreeNode {
    /// Creates a new tree node
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        let name = path.file_name().map_or_else(
            || path.to_string_lossy().to_string(),
            |n| n.to_string_lossy().to_string(),
        );

        let is_dir = path.is_dir();

        Self {
            path,
            name,
            is_dir,
            selection: SelectionState::Unchecked,
            expanded: false,
            children_loaded: false,
            children: Vec::new(),
        }
    }

    /// Loads children for this node if it's a directory
    pub fn load_children(&mut self) {
        if !self.is_dir || self.children_loaded {
            return;
        }

        self.children_loaded = true;

        if let Ok(entries) = std::fs::read_dir(&self.path) {
            let mut children: Vec<TreeNode> = entries
                .filter_map(Result::ok)
                .map(|entry| TreeNode::new(entry.path()))
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
        if !self.is_dir || current_depth >= max_depth {
            return;
        }

        // Load immediate children if not already loaded
        if !self.children_loaded {
            self.load_children();
        }

        // Expand this directory to show its contents
        self.expanded = true;

        // Recursively load children of subdirectories
        for child in &mut self.children {
            if child.is_dir {
                child.load_children_recursive(current_depth + 1, max_depth);
            }
        }
    }

    /// Updates selection state recursively
    pub fn set_selection(&mut self, state: SelectionState) {
        self.selection = state;

        // If setting to checked/unchecked, propagate to all children
        if state != SelectionState::Indeterminate {
            // If this is a directory being checked, load all children recursively
            if state == SelectionState::Checked && self.is_dir {
                // Load all descendants up to 20 levels deep (reasonable limit)
                self.load_children_recursive(0, 20);
                // Also expand this node to show what was selected
                self.expanded = true;
            }

            for child in &mut self.children {
                child.set_selection(state);
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
            self.path.display()
        );

        if self.is_dir {
            result.push_str(&format!(
                "{}  (loaded: {}, expanded: {}, {} children)\n",
                indent,
                self.children_loaded,
                self.expanded,
                self.children.len()
            ));
        }

        for child in &self.children {
            result.push_str(&child.debug_tree(depth + 1));
        }
        result
    }
}

/// Directory tree widget
#[derive(Debug)]
pub struct DirectoryTree {
    /// Root nodes of the tree
    pub roots: Vec<TreeNode>,
    /// Map of path to node for quick lookups
    node_map: HashMap<PathBuf, usize>,
}

impl DirectoryTree {
    /// Creates a new empty directory tree
    #[must_use]
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            node_map: HashMap::new(),
        }
    }

    /// Sets the root directory for the tree
    pub fn set_root(&mut self, path: PathBuf) {
        self.roots.clear();
        self.node_map.clear();

        let mut root = TreeNode::new(path);
        root.expanded = true;
        root.load_children();

        self.roots.push(root);
    }

    /// Renders the tree UI
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // We need to handle the recursive rendering carefully
                // to avoid mutable borrow issues
                if !self.roots.is_empty() {
                    let selection_changed = Self::show_nodes(ui, &mut self.roots[0], 0);

                    // If any selection changed, update parent states
                    if selection_changed {
                        Self::update_parent_states_recursive(&mut self.roots[0]);
                    }
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
    pub fn collect_selected_files(&self) -> Vec<PathBuf> {
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
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            
            let child_count = node.children.len();
            for (index, child) in node.children.iter().enumerate() {
                let is_last_child = index == child_count - 1;
                Self::generate_tree_string_recursive(child, output, &new_prefix, is_last_child);
            }
        }
    }

    /// Helper function to collect selected files from a node recursively
    fn collect_selected_from_node(node: &TreeNode, selected: &mut Vec<PathBuf>) {
        match node.selection {
            SelectionState::Checked => {
                if node.is_dir {
                    // For directories, collect all files recursively
                    for child in &node.children {
                        Self::collect_selected_from_node(child, selected);
                    }
                } else {
                    // For files, add to the selected list
                    selected.push(node.path.clone());
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

    /// Renders nodes recursively
    fn show_nodes(ui: &mut egui::Ui, node: &mut TreeNode, depth: usize) -> bool {
        // Safety: prevent stack overflow on extremely deep directories
        const MAX_DEPTH: usize = 50;
        if depth > MAX_DEPTH {
            ui.label("⚠️ Directory too deep to display");
            return false;
        }
        let indent = depth as f32 * 20.0;
        let mut any_selection_changed = false;

        ui.horizontal(|ui| {
            ui.add_space(indent);

            // Expansion toggle for directories
            if node.is_dir {
                let arrow = if node.expanded { "▼" } else { "▶" };
                if ui.small_button(arrow).clicked() {
                    node.expanded = !node.expanded;
                    if node.expanded && !node.children_loaded {
                        node.load_children();
                    }
                }
            } else {
                // Spacer for files to align with directories
                ui.add_space(ui.spacing().button_padding.x * 2.0 + 16.0);
            }

            // Tri-state checkbox
            let selection_changed = match node.selection {
                SelectionState::Unchecked => {
                    if ui.checkbox(&mut false, "").clicked() {
                        node.set_selection(SelectionState::Checked);
                        true
                    } else {
                        false
                    }
                }
                SelectionState::Checked => {
                    if ui.checkbox(&mut true, "").clicked() {
                        node.set_selection(SelectionState::Unchecked);
                        true
                    } else {
                        false
                    }
                }
                SelectionState::Indeterminate => {
                    // Show as partially checked
                    if ui.checkbox(&mut true, "").clicked() {
                        node.set_selection(SelectionState::Checked);
                        true
                    } else {
                        false
                    }
                }
            };

            if selection_changed {
                any_selection_changed = true;
            }

            // Icon and name
            let icon = if node.is_dir { "📁" } else { "📄" };
            ui.label(format!("{} {}", icon, node.name));
        });

        // Show children if expanded
        if node.is_dir && node.expanded {
            for child in &mut node.children {
                if Self::show_nodes(ui, child, depth + 1) {
                    any_selection_changed = true;
                }
            }
        }

        any_selection_changed
    }
}

impl Default for DirectoryTree {
    fn default() -> Self {
        Self::new()
    }
}
