//! Directory tree UI component with lazy loading and tri-state selection

use eframe::egui;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use glob::Pattern;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// State for tracking rendering position and viewport culling
struct RenderState {
    /// Current Y position in the scroll area
    current_y: f32,
    /// Top of the visible viewport
    viewport_top: f32,
    /// Bottom of the visible viewport
    viewport_bottom: f32,
    /// Height of each tree item
    item_height: f32,
    /// Number of items actually rendered
    items_rendered: usize,
    /// Number of items skipped due to viewport culling
    items_skipped: usize,
}

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
        self.load_children_with_patterns(&[]);
    }

    /// Loads children for this node with ignore patterns
    pub fn load_children_with_patterns(&mut self, ignore_patterns: &[glob::Pattern]) {
        if !self.is_dir || self.children_loaded {
            return;
        }

        self.children_loaded = true;

        if let Ok(entries) = std::fs::read_dir(&self.path) {
            let mut children: Vec<TreeNode> = entries
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

                    Some(TreeNode::new(path))
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
    /// Ignore patterns to filter files/directories
    ignore_patterns: Vec<Pattern>,
}

impl DirectoryTree {
    /// Creates a new empty directory tree
    #[must_use]
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            node_map: HashMap::new(),
            ignore_patterns: Vec::new(),
        }
    }

    /// Sets the root directory for the tree
    pub fn set_root(&mut self, path: PathBuf) {
        self.roots.clear();
        self.node_map.clear();

        let mut root = TreeNode::new(path);
        root.expanded = true;
        root.load_children_with_patterns(&self.ignore_patterns);

        self.roots.push(root);
    }

    /// Updates the ignore patterns from a comma-separated string
    pub fn set_ignore_patterns(&mut self, patterns_str: &str) {
        self.ignore_patterns = patterns_str
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter_map(|pattern| Pattern::new(pattern).ok())
            .collect();

        // Reload all expanded directories with new patterns
        if !self.roots.is_empty() {
            Self::reload_with_patterns(&mut self.roots[0], &self.ignore_patterns);
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

    /// Renders the tree UI with search filtering
    pub fn show_with_search(&mut self, ui: &mut egui::Ui, search_query: &str) {
        // Clone patterns to avoid borrowing issues
        let patterns = self.ignore_patterns.clone();
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_viewport(ui, |ui, viewport| {
                // We need to handle the recursive rendering carefully
                // to avoid mutable borrow issues
                if !self.roots.is_empty() {
                    let matcher = if search_query.is_empty() {
                        None
                    } else {
                        Some(SkimMatcherV2::default())
                    };

                    // Track current y position for viewport culling
                    let mut render_state = RenderState {
                        current_y: 0.0,
                        viewport_top: viewport.min.y,
                        viewport_bottom: viewport.max.y,
                        item_height: ui.spacing().interact_size.y,
                        items_rendered: 0,
                        items_skipped: 0,
                    };

                    let selection_changed = Self::show_nodes_with_search_culled(
                        ui,
                        &mut self.roots[0],
                        0,
                        search_query,
                        matcher.as_ref(),
                        &mut render_state,
                        &patterns,
                    );

                    // If any selection changed, update parent states
                    if selection_changed {
                        Self::update_parent_states_recursive(&mut self.roots[0]);
                    }

                    // Debug: log render stats
                    #[cfg(debug_assertions)]
                    if render_state.items_rendered > 0 || render_state.items_skipped > 0 {
                        ui.ctx()
                            .request_repaint_after(std::time::Duration::from_secs(1));
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
    fn show_nodes(
        ui: &mut egui::Ui,
        node: &mut TreeNode,
        depth: usize,
        patterns: &[Pattern],
    ) -> bool {
        Self::show_nodes_with_search(ui, node, depth, "", None, patterns)
    }

    /// Renders nodes recursively with search filtering
    fn show_nodes_with_search(
        ui: &mut egui::Ui,
        node: &mut TreeNode,
        depth: usize,
        search_query: &str,
        matcher: Option<&SkimMatcherV2>,
        patterns: &[Pattern],
    ) -> bool {
        // Safety: prevent stack overflow on extremely deep directories
        const MAX_DEPTH: usize = 50;
        if depth > MAX_DEPTH {
            ui.label("⚠️ Directory too deep to display");
            return false;
        }

        // Check if this node matches the search
        let should_show = if let Some(matcher) = matcher {
            // Check if the node name matches
            let node_matches = matcher.fuzzy_match(&node.name, search_query).is_some();

            // For directories, also check if any child matches
            let children_match = if node.is_dir {
                Self::has_matching_child(node, search_query, matcher)
            } else {
                false
            };

            node_matches || children_match
        } else {
            true // No search, show everything
        };

        if !should_show {
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
                        node.load_children_with_patterns(patterns);
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
                        node.set_selection_with_patterns(SelectionState::Checked, patterns);
                        true
                    } else {
                        false
                    }
                }
                SelectionState::Checked => {
                    if ui.checkbox(&mut true, "").clicked() {
                        node.set_selection_with_patterns(SelectionState::Unchecked, patterns);
                        true
                    } else {
                        false
                    }
                }
                SelectionState::Indeterminate => {
                    // Show as partially checked
                    if ui.checkbox(&mut true, "").clicked() {
                        node.set_selection_with_patterns(SelectionState::Checked, patterns);
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
            // Auto-expand when searching to show matching children
            if matcher.is_some() && !node.children_loaded {
                node.load_children_with_patterns(patterns);
            }

            for child in &mut node.children {
                if Self::show_nodes_with_search(
                    ui,
                    child,
                    depth + 1,
                    search_query,
                    matcher,
                    patterns,
                ) {
                    any_selection_changed = true;
                }
            }
        }

        any_selection_changed
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

    /// Renders nodes recursively with search filtering and viewport culling
    fn show_nodes_with_search_culled(
        ui: &mut egui::Ui,
        node: &mut TreeNode,
        depth: usize,
        search_query: &str,
        matcher: Option<&SkimMatcherV2>,
        render_state: &mut RenderState,
        patterns: &[Pattern],
    ) -> bool {
        // Safety: prevent stack overflow on extremely deep directories
        const MAX_DEPTH: usize = 50;
        if depth > MAX_DEPTH {
            ui.label("⚠️ Directory too deep to display");
            return false;
        }

        // Check if this node matches the search
        let should_show = if let Some(matcher) = matcher {
            // Check if the node name matches
            let node_matches = matcher.fuzzy_match(&node.name, search_query).is_some();

            // For directories, also check if any child matches
            let children_match = if node.is_dir {
                Self::has_matching_child(node, search_query, matcher)
            } else {
                false
            };

            node_matches || children_match
        } else {
            true // No search, show everything
        };

        if !should_show {
            return false;
        }

        // Calculate item position
        let item_top = render_state.current_y;
        let item_bottom = item_top + render_state.item_height;

        // Check if item is within viewport
        let is_visible =
            item_bottom >= render_state.viewport_top && item_top <= render_state.viewport_bottom;

        let mut any_selection_changed = false;

        if is_visible {
            // Item is visible, render it
            render_state.items_rendered += 1;

            let indent = depth as f32 * 20.0;

            ui.horizontal(|ui| {
                ui.add_space(indent);

                // Expansion toggle for directories
                if node.is_dir {
                    let arrow = if node.expanded { "▼" } else { "▶" };
                    if ui.small_button(arrow).clicked() {
                        node.expanded = !node.expanded;
                        if node.expanded && !node.children_loaded {
                            node.load_children_with_patterns(patterns);
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
                            node.set_selection_with_patterns(SelectionState::Checked, patterns);
                            true
                        } else {
                            false
                        }
                    }
                    SelectionState::Checked => {
                        if ui.checkbox(&mut true, "").clicked() {
                            node.set_selection_with_patterns(SelectionState::Unchecked, patterns);
                            true
                        } else {
                            false
                        }
                    }
                    SelectionState::Indeterminate => {
                        // Show as partially checked
                        if ui.checkbox(&mut true, "").clicked() {
                            node.set_selection_with_patterns(SelectionState::Checked, patterns);
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
        } else {
            // Item is not visible, skip rendering but allocate space
            render_state.items_skipped += 1;
            ui.allocate_space(egui::vec2(ui.available_width(), render_state.item_height));
        }

        // Update Y position
        render_state.current_y += render_state.item_height;

        // Show children if expanded
        if node.is_dir && node.expanded {
            // Auto-expand when searching to show matching children
            if matcher.is_some() && !node.children_loaded {
                node.load_children_with_patterns(patterns);
            }

            for child in &mut node.children {
                if Self::show_nodes_with_search_culled(
                    ui,
                    child,
                    depth + 1,
                    search_query,
                    matcher,
                    render_state,
                    patterns,
                ) {
                    any_selection_changed = true;
                }
            }
        }

        any_selection_changed
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
    }

    /// Collects selected file paths recursively
    fn collect_selected_paths_recursive(node: &TreeNode, selected: &mut HashSet<String>) {
        if node.selection == SelectionState::Checked && !node.is_dir {
            selected.insert(node.path.to_string_lossy().to_string());
        }

        for child in &node.children {
            Self::collect_selected_paths_recursive(child, selected);
        }
    }

    /// Collects expanded directory paths recursively
    fn collect_expanded_paths_recursive(node: &TreeNode, expanded: &mut HashSet<String>) {
        if node.is_dir && node.expanded {
            expanded.insert(node.path.to_string_lossy().to_string());
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
        let path_str = node.path.to_string_lossy().to_string();

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
