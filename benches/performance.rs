use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fsprompt::utils::parallel_fs::{read_files_parallel, scan_directory_parallel};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Create test directory structure
fn create_test_directory(num_dirs: usize, files_per_dir: usize) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    for dir_idx in 0..num_dirs {
        let dir_path = base_path.join(format!("dir_{:04}", dir_idx));
        fs::create_dir(&dir_path).unwrap();

        for file_idx in 0..files_per_dir {
            let file_path = dir_path.join(format!("file_{:04}.txt", file_idx));
            let content = format!("This is file {} in directory {}\n", file_idx, dir_idx);
            fs::write(file_path, content.repeat(100)).unwrap(); // ~10KB per file
        }
    }

    temp_dir
}

// Benchmark directory traversal
fn bench_directory_traversal(c: &mut Criterion) {
    let temp_dir = create_test_directory(100, 50); // 100 dirs, 50 files each = 5000 files
    let root_path = temp_dir.path().to_path_buf();

    c.bench_function("directory_traversal_sequential", |b| {
        b.iter(|| {
            let mut file_count = 0;
            for entry in walkdir::WalkDir::new(&root_path) {
                if entry.unwrap().file_type().is_file() {
                    file_count += 1;
                }
            }
            black_box(file_count)
        })
    });

    c.bench_function("directory_traversal_parallel", |b| {
        b.iter(|| {
            use ignore::WalkBuilder;
            let walker = WalkBuilder::new(&root_path)
                .threads(num_cpus::get())
                .build_parallel();

            let file_count = std::sync::atomic::AtomicUsize::new(0);
            walker.run(|| {
                Box::new(|entry| {
                    if let Ok(entry) = entry {
                        if entry.file_type().map_or(false, |ft| ft.is_file()) {
                            file_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                    ignore::WalkState::Continue
                })
            });
            black_box(file_count.load(std::sync::atomic::Ordering::Relaxed))
        })
    });

    c.bench_function("directory_traversal_optimized", |b| {
        b.iter(|| {
            let entries = scan_directory_parallel(&root_path, None, &[]);
            let file_count = entries.iter().filter(|e| !e.is_dir).count();
            black_box(file_count)
        })
    });
}

// Benchmark file reading
fn bench_file_reading(c: &mut Criterion) {
    let temp_dir = create_test_directory(10, 100); // 1000 files total
    let root_path = temp_dir.path().to_path_buf();

    // Collect all file paths
    let file_paths: Vec<PathBuf> = walkdir::WalkDir::new(&root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    c.bench_function("file_reading_sequential", |b| {
        b.iter(|| {
            let mut total_size = 0;
            for path in &file_paths {
                if let Ok(content) = fs::read_to_string(path) {
                    total_size += content.len();
                }
            }
            black_box(total_size)
        })
    });

    c.bench_function("file_reading_parallel", |b| {
        b.iter(|| {
            let total_size: usize = file_paths
                .par_iter()
                .map(|path| {
                    fs::read_to_string(path)
                        .map(|content| content.len())
                        .unwrap_or(0)
                })
                .sum();
            black_box(total_size)
        })
    });

    c.bench_function("file_reading_optimized", |b| {
        b.iter(|| {
            let results = read_files_parallel(&file_paths, 256 * 1024); // 256KB threshold
            let total_size: usize = results
                .iter()
                .filter_map(|(_, result)| result.as_ref().ok())
                .map(|content| content.len())
                .sum();
            black_box(total_size)
        })
    });

    // Benchmark memory-mapped reading for large files
    let large_file = temp_dir.path().join("large_file.txt");
    let large_content = "x".repeat(10 * 1024 * 1024); // 10MB
    fs::write(&large_file, &large_content).unwrap();

    c.bench_function("large_file_read_standard", |b| {
        b.iter(|| {
            let content = fs::read_to_string(&large_file).unwrap();
            black_box(content.len())
        })
    });

    c.bench_function("large_file_read_mmap", |b| {
        b.iter(|| {
            use memmap2::Mmap;
            let file = fs::File::open(&large_file).unwrap();
            let mmap = unsafe { Mmap::map(&file).unwrap() };
            black_box(mmap.len())
        })
    });
}

// Benchmark output generation
fn bench_output_generation(c: &mut Criterion) {
    let temp_dir = create_test_directory(20, 50); // 1000 files
    let root_path = temp_dir.path().to_path_buf();

    // Prepare test data
    let file_contents: Vec<(PathBuf, String)> = walkdir::WalkDir::new(&root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let path = e.path().to_path_buf();
            let content = fs::read_to_string(&path).unwrap_or_default();
            (path, content)
        })
        .collect();

    c.bench_function("xml_generation", |b| {
        b.iter(|| {
            let mut output = String::with_capacity(1024 * 1024);
            output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root>\n");

            for (path, content) in &file_contents {
                output.push_str(&format!("<file path=\"{}\">\n", path.display()));
                output.push_str("<![CDATA[");
                output.push_str(content);
                output.push_str("]]>\n</file>\n");
            }

            output.push_str("</root>");
            black_box(output.len())
        })
    });

    c.bench_function("markdown_generation", |b| {
        b.iter(|| {
            let mut output = String::with_capacity(1024 * 1024);
            output.push_str("# File Contents\n\n");

            for (path, content) in &file_contents {
                output.push_str(&format!("## {}\n\n```\n", path.display()));
                output.push_str(content);
                output.push_str("\n```\n\n");
            }

            black_box(output.len())
        })
    });
}

// Benchmark glob pattern matching
fn bench_glob_matching(c: &mut Criterion) {
    let patterns = vec![
        "*.rs",
        "**/*.txt",
        "**/target/**",
        "node_modules/**",
        "*.{js,ts,jsx,tsx}",
    ];

    let test_paths = vec![
        "src/main.rs",
        "tests/integration_test.rs",
        "target/debug/build/foo.txt",
        "node_modules/package/index.js",
        "src/components/Button.tsx",
        "docs/readme.md",
        "Cargo.toml",
    ];

    // Compile glob patterns
    let compiled_patterns: Vec<glob::Pattern> = patterns
        .iter()
        .map(|p| glob::Pattern::new(p).unwrap())
        .collect();

    c.bench_function("glob_matching_compiled", |b| {
        b.iter(|| {
            let mut match_count = 0;
            for path in &test_paths {
                for pattern in &compiled_patterns {
                    if pattern.matches(path) {
                        match_count += 1;
                    }
                }
            }
            black_box(match_count)
        })
    });

    // Benchmark regex-based matching (alternative approach)
    let regex_patterns: Vec<regex::Regex> = patterns
        .iter()
        .map(|p| {
            let regex_str = p
                .replace(".", "\\.")
                .replace("*", "[^/]*")
                .replace("**", ".*")
                .replace("{", "(")
                .replace("}", ")")
                .replace(",", "|");
            regex::Regex::new(&format!("^{}$", regex_str)).unwrap()
        })
        .collect();

    c.bench_function("regex_matching_compiled", |b| {
        b.iter(|| {
            let mut match_count = 0;
            for path in &test_paths {
                for pattern in &regex_patterns {
                    if pattern.is_match(path) {
                        match_count += 1;
                    }
                }
            }
            black_box(match_count)
        })
    });
}

criterion_group!(
    benches,
    bench_directory_traversal,
    bench_file_reading,
    bench_output_generation,
    bench_glob_matching
);
criterion_main!(benches);
