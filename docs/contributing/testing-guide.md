# Testing Guide

This guide covers how to write, run, and maintain tests in the fsPrompt project. Testing is crucial for maintaining code quality and ensuring reliable performance across different platforms and use cases.

## Table of Contents

- [Testing Philosophy](#testing-philosophy)
- [Test Categories](#test-categories)
- [Writing Unit Tests](#writing-unit-tests)
- [Integration Testing](#integration-testing)
- [Performance Testing](#performance-testing)
- [Cross-Platform Testing](#cross-platform-testing)
- [Test Data Management](#test-data-management)
- [Running Tests](#running-tests)
- [Debugging Tests](#debugging-tests)
- [CI/CD Integration](#cicd-integration)

## Testing Philosophy

fsPrompt follows a comprehensive testing strategy:

1. **Type-First Testing** - The type system prevents many classes of bugs
2. **Property-Based Testing** - Test invariants and behaviors, not just examples
3. **Performance Testing** - Ensure scalability to large codebases (10,000+ files)
4. **Cross-Platform Testing** - Verify behavior across Windows, macOS, and Linux
5. **Real-World Testing** - Use realistic datasets and scenarios

## Test Categories

### 1. Unit Tests
Test individual components in isolation.

**Location**: `#[cfg(test)]` modules within source files
**Purpose**: Verify individual function and method behavior
**Scope**: Single functions, methods, or small components

### 2. Integration Tests
Test component interactions and complete workflows.

**Location**: `tests/` directory
**Purpose**: Verify system behavior and component integration
**Scope**: Multiple modules working together

### 3. Performance Tests (Benchmarks)
Measure and track performance characteristics.

**Location**: `benches/` directory
**Purpose**: Ensure performance requirements are met
**Scope**: Critical performance paths and scalability

### 4. Property Tests
Test invariants and properties across a range of inputs.

**Status**: Planned for future implementation
**Purpose**: Catch edge cases and verify mathematical properties

## Writing Unit Tests

### Basic Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_canonical_path_creation() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.txt");
        std::fs::write(&path, "test content").unwrap();
        
        let canonical = CanonicalPath::new(&path).unwrap();
        assert_eq!(canonical.as_path(), path.canonicalize().unwrap());
    }
}
```

### Testing Guidelines

1. **Use descriptive test names**:
   ```rust
   #[test]
   fn test_token_count_from_chars_rounds_up() { /* ... */ }
   
   #[test]
   fn test_selection_state_propagates_to_children() { /* ... */ }
   ```

2. **Test both success and failure cases**:
   ```rust
   #[test]
   fn test_font_size_validation_accepts_valid_range() {
       assert!(FontSize::new(12.0).is_ok());
   }
   
   #[test]
   fn test_font_size_validation_rejects_out_of_range() {
       assert!(FontSize::new(5.0).is_err());
       assert!(FontSize::new(30.0).is_err());
   }
   ```

3. **Use temporary directories for filesystem tests**:
   ```rust
   #[test]
   fn test_directory_scanning() {
       let temp_dir = TempDir::new().unwrap();
       // Create test structure
       // Run test
       // temp_dir automatically cleaned up
   }
   ```

4. **Test edge cases**:
   ```rust
   #[test]
   fn test_empty_directory() { /* ... */ }
   
   #[test]
   fn test_deeply_nested_structure() { /* ... */ }
   
   #[test]
   fn test_unicode_filenames() { /* ... */ }
   ```

### Type System Testing

Focus on testing your type constructors and invariants:

```rust
#[test]
fn test_window_ratio_validation() {
    // Valid ratios
    assert!(WindowRatio::new(0.0).is_ok());
    assert!(WindowRatio::new(0.5).is_ok());
    assert!(WindowRatio::new(1.0).is_ok());
    
    // Invalid ratios
    assert!(WindowRatio::new(-0.1).is_err());
    assert!(WindowRatio::new(1.1).is_err());
}

#[test]
fn test_canonical_path_prevents_traversal() {
    let temp_dir = TempDir::new().unwrap();
    let root = CanonicalPath::new(temp_dir.path()).unwrap();
    
    // Should reject paths outside root
    let result = CanonicalPath::new_within_root("../../../etc/passwd", &root);
    assert!(result.is_err());
}
```

## Integration Testing

Integration tests are located in the `tests/` directory and test complete workflows.

### Example: File Selection Test

```rust
// tests/integration/file_selection_test.rs
use fsprompt::core::types::{CanonicalPath, SelectionState};
use fsprompt::ui::tree::{DirectoryTree, TreeNode};
use tempfile::TempDir;

#[test]
fn test_complete_file_selection_workflow() {
    // Setup test directory
    let temp_dir = setup_test_directory();
    
    // Create tree
    let mut tree = DirectoryTree::new();
    tree.set_root(TreeNode::new(temp_dir.path()).unwrap());
    
    // Test selection propagation
    // Test file collection
    // Test output generation
}

fn setup_test_directory() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    
    // Create realistic project structure
    std::fs::create_dir(root.join("src")).unwrap();
    std::fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
    std::fs::write(root.join("src/lib.rs"), "// Library code").unwrap();
    
    std::fs::create_dir(root.join("tests")).unwrap();
    std::fs::write(root.join("tests/integration.rs"), "// Integration tests").unwrap();
    
    temp_dir
}
```

### Testing Worker Communication

```rust
// tests/integration/worker_communication_test.rs
use fsprompt::workers::generator::GenerationWorker;
use crossbeam::channel;
use std::time::Duration;

#[test]
fn test_worker_generation_pipeline() {
    let (sender, receiver) = channel::unbounded();
    
    // Start worker
    let worker = GenerationWorker::new(sender);
    
    // Send generation request
    // Verify response
    // Test cancellation
    // Test error handling
}
```

## Performance Testing

Performance tests use the Criterion framework and are located in `benches/`.

### Benchmark Structure

```rust
// benches/performance.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use fsprompt::core::types::CanonicalPath;

fn bench_file_reading(c: &mut Criterion) {
    let test_data = create_test_files();
    
    let mut group = c.benchmark_group("file_reading");
    
    for size in [1_000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("parallel", size),
            size,
            |b, &size| {
                b.iter(|| {
                    // Benchmark parallel file reading
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_file_reading);
criterion_main!(benches);
```

### Performance Requirements

fsPrompt has specific performance targets:

1. **Directory Scanning**: <2 seconds for 10,000 files
2. **File Reading**: <5 seconds for 100MB of content
3. **Token Counting**: <1 second for 1M characters
4. **UI Responsiveness**: 60 FPS during operations

### Performance Test Examples

```rust
#[test]
fn test_performance_requirements() {
    let test_data = create_large_test_dataset(); // 10,000 files
    
    let start = Instant::now();
    let result = scan_directory_parallel(&test_data.path()).unwrap();
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_secs(2), 
            "Directory scanning took {:?}, expected <2s", duration);
    assert_eq!(result.len(), 10_000);
}
```

### Memory Usage Testing

```rust
#[test]
fn test_memory_usage_stays_bounded() {
    let initial_memory = get_memory_usage();
    
    // Process large dataset
    process_large_codebase();
    
    let final_memory = get_memory_usage();
    let memory_increase = final_memory - initial_memory;
    
    assert!(memory_increase < 100_000_000, // 100MB
            "Memory usage increased by {} bytes", memory_increase);
}
```

## Cross-Platform Testing

### Platform-Specific Tests

```rust
#[cfg(target_os = "windows")]
#[test]
fn test_windows_path_handling() {
    // Test Windows-specific path behaviors
    // Long paths, UNC paths, drive letters
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_file_dialogs() {
    // Test macOS-specific dialog behavior
    // HFS+ vs APFS considerations
}

#[cfg(target_family = "unix")]
#[test]
fn test_unix_permissions() {
    // Test file permissions and symlinks
    // Case sensitivity
}
```

### Cross-Platform Utilities

```rust
fn create_test_file_with_permissions(path: &Path, content: &str, executable: bool) {
    std::fs::write(path, content).unwrap();
    
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path).unwrap().permissions();
        if executable {
            perms.set_mode(0o755);
        } else {
            perms.set_mode(0o644);
        }
        std::fs::set_permissions(path, perms).unwrap();
    }
}
```

## Test Data Management

### Creating Realistic Test Data

```rust
pub fn create_realistic_project_structure(base: &Path, file_count: usize) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    // Source code
    let src_dir = base.join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    
    for i in 0..file_count / 4 {
        let file_path = src_dir.join(format!("module_{}.rs", i));
        let content = generate_rust_code(100 + i * 10); // Varying sizes
        std::fs::write(&file_path, content).unwrap();
        files.push(file_path);
    }
    
    // Tests
    let test_dir = base.join("tests");
    std::fs::create_dir_all(&test_dir).unwrap();
    // ... create test files
    
    // Documentation
    let docs_dir = base.join("docs");
    std::fs::create_dir_all(&docs_dir).unwrap();
    // ... create documentation files
    
    files
}

fn generate_rust_code(lines: usize) -> String {
    let mut content = String::new();
    content.push_str("// Generated test file\n");
    content.push_str("use std::collections::HashMap;\n\n");
    
    for i in 0..lines {
        content.push_str(&format!("// Line {}\n", i));
        if i % 10 == 0 {
            content.push_str(&format!("fn function_{}() {{\n", i / 10));
            content.push_str("    println!(\"test\");\n");
            content.push_str("}\n\n");
        }
    }
    
    content
}
```

### Test Data Cleanup

```rust
pub struct TestEnv {
    temp_dir: TempDir,
    files: Vec<PathBuf>,
}

impl TestEnv {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let files = create_realistic_project_structure(temp_dir.path(), 1000);
        
        Self { temp_dir, files }
    }
    
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }
}

// Automatic cleanup via Drop trait
impl Drop for TestEnv {
    fn drop(&mut self) {
        // TempDir handles cleanup automatically
    }
}
```

## Running Tests

### Basic Test Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests single-threaded (useful for debugging)
cargo test -- --test-threads=1

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'
```

### Performance Tests

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench performance

# Run with specific number of iterations
cargo bench -- --measurement-time 10

# Generate HTML reports
cargo bench -- --output-format html
```

### Test Filtering

```bash
# Run tests matching pattern
cargo test selection

# Run tests in specific module
cargo test core::types

# Exclude expensive tests
cargo test -- --skip expensive

# Run only ignored tests
cargo test -- --ignored
```

## Debugging Tests

### Debug Output

```rust
#[test]
fn test_with_debug_output() {
    env_logger::init(); // Initialize logging
    
    let result = function_under_test();
    
    println!("Result: {:?}", result); // Will show with --nocapture
    eprintln!("Error info: {:?}", result.err()); // Always shows
    
    assert!(result.is_ok());
}
```

### Test-Specific Logging

```rust
#[test]
fn test_with_logging() {
    use tracing_test::traced_test;
    
    #[traced_test]
    fn inner_test() {
        tracing::info!("Starting test");
        // Test implementation
        tracing::debug!("Debug information");
    }
    
    inner_test();
}
```

### Debugging Tips

1. **Use `dbg!` macro** for quick debugging:
   ```rust
   let result = dbg!(function_call());
   ```

2. **Print intermediate values**:
   ```rust
   println!("Intermediate: {:#?}", complex_structure);
   ```

3. **Use `std::hint::black_box`** to prevent optimization in benchmarks:
   ```rust
   b.iter(|| {
       let result = expensive_operation();
       std::hint::black_box(result)
   });
   ```

## CI/CD Integration

### GitHub Actions

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        components: clippy, rustfmt
        override: true
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Run Clippy
      run: cargo clippy --all-targets -- -D warnings
    
    - name: Run tests
      run: cargo test --all-targets
    
    - name: Run benchmarks (smoke test)
      run: cargo bench -- --test
```

### Test Coverage

```bash
# Install cargo-tarpaulin (Linux only)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Upload to codecov (in CI)
cargo tarpaulin --out Xml
bash <(curl -s https://codecov.io/bash)
```

## Best Practices

### Test Organization

1. **Group related tests** in modules
2. **Use descriptive test names** that explain what is being tested
3. **Test one thing per test** function
4. **Use setup/teardown helpers** for common test initialization

### Test Maintenance

1. **Keep tests simple** and focused
2. **Update tests when APIs change**
3. **Remove obsolete tests** when features are removed
4. **Document complex test setups**

### Performance Testing

1. **Test with realistic data** sizes and structures
2. **Use stable benchmarking environment**
3. **Track performance regressions** over time
4. **Test both memory and CPU performance**

### Cross-Platform Testing

1. **Test on all target platforms** when possible
2. **Use platform-specific test conditions** appropriately
3. **Document platform differences** in tests
4. **Test edge cases** specific to each platform

## Common Testing Patterns

### Testing Error Conditions

```rust
#[test]
fn test_error_handling() {
    let result = operation_that_should_fail();
    
    match result {
        Err(ExpectedError::SpecificVariant { .. }) => {
            // Good - got expected error
        }
        Err(other) => panic!("Got unexpected error: {:?}", other),
        Ok(_) => panic!("Expected error but got success"),
    }
}
```

### Testing Async Code

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_operation().await;
    assert!(result.is_ok());
}
```

### Testing with Timeouts

```rust
#[test]
fn test_with_timeout() {
    use std::time::{Duration, Instant};
    
    let start = Instant::now();
    let result = potentially_slow_operation();
    let elapsed = start.elapsed();
    
    assert!(result.is_ok());
    assert!(elapsed < Duration::from_secs(5), "Operation took too long: {:?}", elapsed);
}
```

Remember: Good tests are investments in code quality. They catch bugs early, document expected behavior, and enable confident refactoring. Take time to write comprehensive tests!