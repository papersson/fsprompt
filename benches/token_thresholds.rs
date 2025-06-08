use criterion::{criterion_group, criterion_main, Criterion};
use fsprompt::core::types::CanonicalPath;
use fsprompt::utils::parallel_fs::{read_files_parallel, scan_directory_parallel};
use std::fs;
use tempfile::TempDir;

/// Create a realistic project structure for benchmarking
fn create_realistic_project(
    name: &str,
    num_files: usize,
    avg_file_size: usize,
    max_depth: usize,
) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create a realistic directory structure
    let dirs_per_level = (num_files as f64).sqrt() as usize / max_depth;
    let files_per_dir = num_files / (dirs_per_level * max_depth).max(1);

    // Common project structure
    let common_dirs = ["src", "tests", "docs", "examples", "benches"];

    for (i, dir) in common_dirs.iter().enumerate() {
        if i >= max_depth {
            break;
        }

        let dir_path = base_path.join(dir);
        fs::create_dir(&dir_path).unwrap();

        // Create subdirectories
        for j in 0..dirs_per_level {
            let subdir = dir_path.join(format!("module_{j}"));
            fs::create_dir(&subdir).unwrap();

            // Create files
            for k in 0..files_per_dir {
                let file_path = subdir.join(format!("file_{k}.rs"));
                let content = generate_realistic_code(avg_file_size);
                fs::write(file_path, content).unwrap();
            }
        }
    }

    // Create root files
    let root_files = ["Cargo.toml", "README.md", "LICENSE", ".gitignore"];
    for file in root_files {
        let content = generate_realistic_content(file, avg_file_size / 2);
        fs::write(base_path.join(file), content).unwrap();
    }

    println!("{name}: Created {num_files} files in {max_depth} levels");
    temp_dir
}

/// Generate realistic code content
fn generate_realistic_code(size: usize) -> String {
    let template = r#"use std::collections::HashMap;

/// This module handles important business logic
pub struct DataProcessor {
    cache: HashMap<String, Vec<u8>>,
    config: Config,
}

impl DataProcessor {
    pub fn new(config: Config) -> Self {
        Self {
            cache: HashMap::new(),
            config,
        }
    }
    
    pub fn process(&mut self, input: &str) -> Result<String, Error> {
        // Check cache first
        if let Some(cached) = self.cache.get(input) {
            return Ok(String::from_utf8_lossy(cached).to_string());
        }
        
        // Process the input
        let result = self.do_processing(input)?;
        
        // Cache the result
        self.cache.insert(input.to_string(), result.as_bytes().to_vec());
        
        Ok(result)
    }
    
    fn do_processing(&self, input: &str) -> Result<String, Error> {
        // Simulate some complex processing
        let processed = input
            .chars()
            .map(|c| match c {
                'a'..='z' => ((c as u8 - b'a' + 13) % 26 + b'a') as char,
                'A'..='Z' => ((c as u8 - b'A' + 13) % 26 + b'A') as char,
                _ => c,
            })
            .collect();
        
        Ok(processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_processing() {
        let config = Config::default();
        let mut processor = DataProcessor::new(config);
        
        assert_eq!(processor.process("hello").unwrap(), "uryyb");
        assert_eq!(processor.process("WORLD").unwrap(), "JBEYQ");
    }
}
"#;

    // Repeat template to reach desired size
    let repeat_count = size / template.len() + 1;
    template.repeat(repeat_count)[..size].to_string()
}

/// Generate realistic non-code content
fn generate_realistic_content(filename: &str, size: usize) -> String {
    match filename {
        "Cargo.toml" => {
            format!(
                r#"[package]
name = "example-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
tokio = {{ version = "1.0", features = ["full"] }}
anyhow = "1.0"

[dev-dependencies]
criterion = "0.5"
{}
"#,
                " ".repeat(size.saturating_sub(200))
            )
        }
        "README.md" => {
            format!(
                r#"# Example Project

This is a realistic example project for benchmarking fsPrompt.

## Features
- Fast processing
- Caching support
- Comprehensive tests

## Usage
```rust
let processor = DataProcessor::new(Config::default());
let result = processor.process("input")?;
```
{}
"#,
                " ".repeat(size.saturating_sub(250))
            )
        }
        _ => " ".repeat(size),
    }
}

/// Benchmark 20K token threshold (microservice/small library)
fn bench_20k_tokens(c: &mut Criterion) {
    let temp_dir = create_realistic_project("20k_tokens", 80, 1024, 4);
    let root_path = temp_dir.path().to_path_buf();

    let mut group = c.benchmark_group("20k_tokens");

    group.bench_function("full_workflow", |b| {
        b.iter(|| {
            // 1. Scan directory
            let entries = scan_directory_parallel(&root_path, None, &[]);

            // 2. Collect file paths
            let file_paths: Vec<CanonicalPath> = entries
                .iter()
                .filter(|e| !e.is_dir)
                .map(|e| e.path.clone())
                .collect();

            // 3. Read files
            let contents = read_files_parallel(&file_paths, 256 * 1024);

            // 4. Generate output
            let mut output = String::with_capacity(100 * 1024);
            for (path, content) in contents {
                if let Ok(content) = content {
                    output.push_str(&format!(
                        "## {}\n```\n{}\n```\n",
                        path.as_path().display(),
                        content
                    ));
                }
            }

            output.len()
        });
    });

    group.finish();
}

/// Benchmark 100K token threshold (full application)
fn bench_100k_tokens(c: &mut Criterion) {
    let temp_dir = create_realistic_project("100k_tokens", 400, 1024, 6);
    let root_path = temp_dir.path().to_path_buf();

    let mut group = c.benchmark_group("100k_tokens");
    group.sample_size(20); // Fewer samples for longer benchmarks

    group.bench_function("full_workflow", |b| {
        b.iter(|| {
            let entries = scan_directory_parallel(&root_path, None, &[]);
            let file_paths: Vec<CanonicalPath> = entries
                .iter()
                .filter(|e| !e.is_dir)
                .map(|e| e.path.clone())
                .collect();

            let contents = read_files_parallel(&file_paths, 256 * 1024);

            let mut output = String::with_capacity(500 * 1024);
            for (path, content) in contents {
                if let Ok(content) = content {
                    output.push_str(&format!(
                        "## {}\n```\n{}\n```\n",
                        path.as_path().display(),
                        content
                    ));
                }
            }

            output.len()
        });
    });

    group.finish();
}

/// Benchmark 200K token threshold (monorepo section)
fn bench_200k_tokens(c: &mut Criterion) {
    let temp_dir = create_realistic_project("200k_tokens", 800, 1024, 8);
    let root_path = temp_dir.path().to_path_buf();

    let mut group = c.benchmark_group("200k_tokens");
    group.sample_size(10); // Even fewer samples

    group.bench_function("full_workflow", |b| {
        b.iter(|| {
            let entries = scan_directory_parallel(&root_path, None, &[]);
            let file_paths: Vec<CanonicalPath> = entries
                .iter()
                .filter(|e| !e.is_dir)
                .map(|e| e.path.clone())
                .collect();

            let contents = read_files_parallel(&file_paths, 256 * 1024);

            let mut output = String::with_capacity(1024 * 1024);
            for (path, content) in contents {
                if let Ok(content) = content {
                    output.push_str(&format!(
                        "## {}\n```\n{}\n```\n",
                        path.as_path().display(),
                        content
                    ));
                }
            }

            output.len()
        });
    });

    group.finish();
}

/// Benchmark individual operations at different scales
fn bench_operations_by_scale(c: &mut Criterion) {
    let scales = [
        ("small", 100, 1024),
        ("medium", 500, 1024),
        ("large", 1000, 1024),
        ("xlarge", 5000, 1024),
    ];

    for (name, num_files, file_size) in scales {
        let temp_dir = create_realistic_project(name, num_files, file_size, 6);
        let root_path = temp_dir.path().to_path_buf();

        let mut group = c.benchmark_group(format!("operations_{name}"));

        // Benchmark directory scanning
        group.bench_function("scan_directory", |b| {
            b.iter(|| scan_directory_parallel(&root_path, None, &[]));
        });

        // Pre-collect file paths for file reading benchmark
        let entries = scan_directory_parallel(&root_path, None, &[]);
        let file_paths: Vec<CanonicalPath> = entries
            .iter()
            .filter(|e| !e.is_dir)
            .map(|e| e.path.clone())
            .collect();

        // Benchmark file reading
        group.bench_function("read_files", |b| {
            b.iter(|| read_files_parallel(&file_paths, 256 * 1024));
        });

        group.finish();
    }
}

criterion_group!(
    benches,
    bench_20k_tokens,
    bench_100k_tokens,
    bench_200k_tokens,
    bench_operations_by_scale
);
criterion_main!(benches);
