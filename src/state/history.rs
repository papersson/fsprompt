//! Undo/Redo history management for fsPrompt

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SelectionSnapshot {
    /// Set of selected file paths
    pub selected_files: HashSet<String>,
    /// Set of expanded directories
    pub expanded_dirs: HashSet<String>,
}

#[derive(Debug)]
pub struct HistoryManager {
    /// Past states (for undo)
    past: Vec<SelectionSnapshot>,
    /// Future states (for redo)
    future: Vec<SelectionSnapshot>,
    /// Maximum history depth
    max_depth: usize,
}

impl HistoryManager {
    pub fn new(max_depth: usize) -> Self {
        Self {
            past: Vec::new(),
            future: Vec::new(),
            max_depth,
        }
    }

    /// Record a new state, clearing any redo history
    pub fn push(&mut self, snapshot: SelectionSnapshot) {
        // Clear redo history when new action is performed
        self.future.clear();

        // Add to past
        self.past.push(snapshot);

        // Trim if exceeds max depth
        if self.past.len() > self.max_depth {
            self.past.remove(0);
        }
    }

    /// Undo the last action, returns the previous state if available
    pub fn undo(&mut self, current: SelectionSnapshot) -> Option<SelectionSnapshot> {
        if let Some(previous) = self.past.pop() {
            // Move current state to future
            self.future.push(current);

            // Trim future if needed
            if self.future.len() > self.max_depth {
                self.future.remove(0);
            }

            Some(previous)
        } else {
            None
        }
    }

    /// Redo the last undone action, returns the next state if available
    pub fn redo(&mut self, current: SelectionSnapshot) -> Option<SelectionSnapshot> {
        if let Some(next) = self.future.pop() {
            // Move current state to past
            self.past.push(current);

            // Trim past if needed
            if self.past.len() > self.max_depth {
                self.past.remove(0);
            }

            Some(next)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }
}
