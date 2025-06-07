use fsprompt::core::types::{CanonicalPath, SelectionState};
use fsprompt::ui::tree::{DirectoryTree, TreeNode};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_file_selection() {
    // Create a temporary directory structure
    let temp_dir = TempDir::new().unwrap();
    let root_path = temp_dir.path();
    
    // Create some test files
    std::fs::create_dir(root_path.join("src")).unwrap();
    std::fs::write(root_path.join("src/main.rs"), "fn main() {}").unwrap();
    std::fs::write(root_path.join("src/lib.rs"), "// lib").unwrap();
    std::fs::create_dir(root_path.join("tests")).unwrap();
    std::fs::write(root_path.join("tests/test.rs"), "// test").unwrap();
    
    // Create a tree from the temp directory
    let root_node = TreeNode::new(root_path.to_path_buf()).unwrap();
    let mut tree = DirectoryTree::new();
    tree.set_root(root_node);
    
    // Test initial state - no selections
    let selected = tree.collect_selected_files();
    assert_eq!(selected.len(), 0);
    
    // Select a file and verify
    if let Some(root) = tree.roots.first_mut() {
        root.load_children_with_patterns(&[]);
        
        // Find and select src/main.rs
        if let Some(src) = root.children.iter_mut().find(|c| c.name == "src") {
            src.load_children_with_patterns(&[]);
            if let Some(main_rs) = src.children.iter_mut().find(|c| c.name == "main.rs") {
                main_rs.selection = SelectionState::Checked;
            }
        }
    }
    
    // Verify selection
    let selected = tree.collect_selected_files();
    assert_eq!(selected.len(), 1);
    assert!(selected[0].as_path().ends_with("src/main.rs"));
}

#[test]
fn test_directory_selection_propagation() {
    let temp_dir = TempDir::new().unwrap();
    let root_path = temp_dir.path();
    
    // Create nested structure
    std::fs::create_dir(root_path.join("src")).unwrap();
    std::fs::write(root_path.join("src/a.rs"), "").unwrap();
    std::fs::write(root_path.join("src/b.rs"), "").unwrap();
    
    let root_node = TreeNode::new(root_path.to_path_buf()).unwrap();
    let mut tree = DirectoryTree::new();
    tree.set_root(root_node);
    
    // Select entire directory
    if let Some(root) = tree.roots.first_mut() {
        root.load_children_with_patterns(&[]);
        
        if let Some(src) = root.children.iter_mut().find(|c| c.name == "src") {
            src.set_selection_with_patterns(SelectionState::Checked, &[]);
            src.load_children_with_patterns(&[]);
        }
    }
    
    // All files in directory should be selected
    let selected = tree.collect_selected_files();
    assert_eq!(selected.len(), 2);
}

#[test]
fn test_ignore_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let root_path = temp_dir.path();
    
    // Create files that should be ignored
    std::fs::write(root_path.join("file.rs"), "").unwrap();
    std::fs::write(root_path.join(".hidden"), "").unwrap();
    std::fs::create_dir(root_path.join("node_modules")).unwrap();
    std::fs::write(root_path.join("node_modules/package.json"), "{}").unwrap();
    
    let root_node = TreeNode::new(root_path.to_path_buf()).unwrap();
    let mut tree = DirectoryTree::new();
    tree.set_root(root_node);
    
    // Add ignore patterns
    tree.set_ignore_patterns(vec![".*".to_string(), "node_modules".to_string()]);
    
    // Load children with patterns
    if let Some(root) = tree.roots.first_mut() {
        root.load_children_with_patterns(&tree.ignore_patterns);
        
        // Hidden files and node_modules should not be present
        assert!(root.children.iter().find(|c| c.name == ".hidden").is_none());
        assert!(root.children.iter().find(|c| c.name == "node_modules").is_none());
        assert!(root.children.iter().find(|c| c.name == "file.rs").is_some());
    }
}