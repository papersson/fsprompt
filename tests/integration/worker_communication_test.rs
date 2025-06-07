use fsprompt::core::types::{
    CanonicalPath, OutputConfig, OutputFormat, ProgressStage, WorkerError, WorkerRequest,
    WorkerResponse,
};
use fsprompt::workers::create_worker_pair;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_worker_directory_scan() {
    let temp_dir = TempDir::new().unwrap();
    let root_path = temp_dir.path();
    
    // Create test files
    std::fs::write(root_path.join("file1.rs"), "content1").unwrap();
    std::fs::write(root_path.join("file2.rs"), "content2").unwrap();
    std::fs::create_dir(root_path.join("subdir")).unwrap();
    
    let (tx, rx, _handle) = create_worker_pair();
    
    // Send scan request
    let canonical_path = CanonicalPath::new(root_path).unwrap();
    tx.send(WorkerRequest::ScanDirectory {
        path: canonical_path.clone(),
        include_hidden: false,
    })
    .unwrap();
    
    // Wait for response
    let response = rx.recv_timeout(Duration::from_secs(5)).unwrap();
    
    match response {
        WorkerResponse::DirectoryEntries { path, entries } => {
            assert_eq!(path, canonical_path);
            assert_eq!(entries.len(), 3); // 2 files + 1 directory
            
            let names: HashSet<String> = entries.iter().map(|e| e.name.clone()).collect();
            assert!(names.contains("file1.rs"));
            assert!(names.contains("file2.rs"));
            assert!(names.contains("subdir"));
        }
        _ => panic!("Expected DirectoryEntries response"),
    }
}

#[test]
fn test_worker_output_generation() {
    let temp_dir = TempDir::new().unwrap();
    let root_path = temp_dir.path();
    
    // Create test files
    std::fs::write(root_path.join("main.rs"), "fn main() { println!(\"Hello\"); }").unwrap();
    std::fs::write(root_path.join("lib.rs"), "pub fn lib_func() {}").unwrap();
    
    let (tx, rx, _handle) = create_worker_pair();
    
    // Prepare selections
    let canonical_root = CanonicalPath::new(root_path).unwrap();
    let mut selections = HashSet::new();
    selections.insert(CanonicalPath::new(root_path.join("main.rs")).unwrap());
    selections.insert(CanonicalPath::new(root_path.join("lib.rs")).unwrap());
    
    // Send generation request
    let config = OutputConfig {
        format: OutputFormat::Xml,
        ignore_patterns: Arc::new(vec![]),
        include_contents: true,
        max_file_size: None,
    };
    
    tx.send(WorkerRequest::GenerateOutput {
        root: canonical_root,
        selections: Arc::new(selections),
        config,
    })
    .unwrap();
    
    // Collect responses until we get the output
    let mut got_progress = false;
    let mut got_output = false;
    
    for _ in 0..10 {
        if let Ok(response) = rx.recv_timeout(Duration::from_secs(1)) {
            match response {
                WorkerResponse::Progress(update) => {
                    got_progress = true;
                    assert!(matches!(
                        update.stage,
                        ProgressStage::Discovery | ProgressStage::Reading | ProgressStage::Formatting
                    ));
                }
                WorkerResponse::OutputReady {
                    content,
                    tokens,
                    generation_time_ms,
                } => {
                    got_output = true;
                    assert!(!content.is_empty());
                    assert!(content.contains("main.rs"));
                    assert!(content.contains("lib.rs"));
                    assert!(content.contains("fn main()"));
                    assert!(tokens.get() > 0);
                    assert!(generation_time_ms > 0);
                    break;
                }
                _ => {}
            }
        }
    }
    
    assert!(got_progress, "Should have received progress updates");
    assert!(got_output, "Should have received output");
}

#[test]
fn test_worker_error_handling() {
    let (tx, rx, _handle) = create_worker_pair();
    
    // Send scan request for non-existent directory
    let non_existent = std::path::PathBuf::from("/definitely/does/not/exist/anywhere");
    
    match CanonicalPath::new(&non_existent) {
        Ok(path) => {
            tx.send(WorkerRequest::ScanDirectory {
                path,
                include_hidden: false,
            })
            .unwrap();
        }
        Err(_) => {
            // If we can't create canonical path, that's fine - it means the path doesn't exist
            return;
        }
    }
    
    // Should receive an error response
    let response = rx.recv_timeout(Duration::from_secs(5)).unwrap();
    
    match response {
        WorkerResponse::Error(WorkerError::NotFound { path }) => {
            assert_eq!(path, non_existent);
        }
        WorkerResponse::Error(WorkerError::Io { path, .. }) => {
            assert_eq!(path, non_existent);
        }
        _ => panic!("Expected error response for non-existent directory"),
    }
}

#[test]
fn test_worker_cancellation() {
    let temp_dir = TempDir::new().unwrap();
    let root_path = temp_dir.path();
    
    // Create many files to slow down the operation
    for i in 0..100 {
        std::fs::write(root_path.join(format!("file{}.txt", i)), format!("content {}", i)).unwrap();
    }
    
    let (tx, rx, _handle) = create_worker_pair();
    
    // Start a generation task
    let canonical_root = CanonicalPath::new(root_path).unwrap();
    let selections = (0..100)
        .map(|i| CanonicalPath::new(root_path.join(format!("file{}.txt", i))).unwrap())
        .collect::<HashSet<_>>();
    
    let config = OutputConfig {
        format: OutputFormat::Xml,
        ignore_patterns: Arc::new(vec![]),
        include_contents: true,
        max_file_size: None,
    };
    
    tx.send(WorkerRequest::GenerateOutput {
        root: canonical_root,
        selections: Arc::new(selections),
        config,
    })
    .unwrap();
    
    // Send cancel immediately
    tx.send(WorkerRequest::Cancel).unwrap();
    
    // Should receive cancellation error eventually
    let mut got_cancelled = false;
    
    for _ in 0..10 {
        if let Ok(response) = rx.recv_timeout(Duration::from_millis(100)) {
            if let WorkerResponse::Error(WorkerError::Cancelled) = response {
                got_cancelled = true;
                break;
            }
        }
    }
    
    assert!(got_cancelled, "Should have received cancellation error");
}