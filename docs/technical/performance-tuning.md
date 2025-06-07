# Performance Tuning Reference

This document provides detailed performance characteristics, optimization strategies, and tuning guidelines for fsPrompt based on benchmarking results and profiling data.

## Performance Benchmarks

### Directory Traversal Performance

Based on `benches/performance.rs` results:

#### Small to Medium Projects (100-1,000 files)
```
Sequential traversal:     ~50-100ms
Parallel traversal:       ~20-40ms  
Optimized (fsPrompt):     ~15-30ms
Speedup: 2-3x over sequential
```

#### Large Projects (5,000-10,000 files)
```
Sequential traversal:     ~500ms-2s
Parallel traversal:       ~150-400ms
Optimized (fsPrompt):     ~100-250ms  
Speedup: 3-5x over sequential
```

#### Configuration for Optimal Traversal

```rust
// Thread count optimization
builder.threads(num_cpus::get().min(8)); // Sweet spot for most systems

// Disable expensive operations
builder
    .standard_filters(false)    // Skip .gitignore parsing
    .hidden(false)             // Show hidden files explicitly  
    .parents(false)            // Don't search parent directories
    .follow_links(false);      // Security and performance
```

### File Reading Performance

#### Memory Mapping Thresholds

```rust
const MEMORY_MAP_THRESHOLD: u64 = 256 * 1024; // 256KB

impl FileSize {
    pub const fn read_strategy(&self) -> FileReadStrategy {
        if self.0 < MEMORY_MAP_THRESHOLD {
            FileReadStrategy::Direct      // std::fs::read_to_string()
        } else {
            FileReadStrategy::MemoryMapped // memmap2::Mmap
        }
    }
}
```

#### Benchmark Results by File Size

**Small Files (<256KB)**
```
Standard read:        ~0.1-1ms per file
Memory mapped:        ~0.5-2ms per file (overhead dominates)
Recommendation: Use standard reading
```

**Large Files (>1MB)**  
```
Standard read:        ~10-100ms per file
Memory mapped:        ~1-5ms per file
Recommendation: Use memory mapping
```

**Parallel vs Sequential Reading**
```
1000 small files:
  Sequential: ~500ms
  Parallel:   ~150ms (3x speedup)

100 large files:  
  Sequential: ~5s
  Parallel:   ~1.2s (4x speedup)
```

### Token-Based Performance Thresholds

From `benches/token_thresholds.rs`:

#### 20K Token Threshold (Microservice/Small Library)
```
Files: ~80 files, 1KB average
Full workflow time: ~50-100ms
Memory usage: ~5-10MB
Recommendation: Real-time generation
```

#### 100K Token Threshold (Full Application)
```  
Files: ~400 files, 1KB average
Full workflow time: ~200-500ms
Memory usage: ~25-50MB
Recommendation: Background generation with progress
```

#### 200K Token Threshold (Monorepo Section)
```
Files: ~800 files, 1KB average  
Full workflow time: ~500ms-1.5s
Memory usage: ~50-100MB
Recommendation: Background generation with cancellation
```

### UI Rendering Performance

From `benches/ui_performance.rs`:

#### Tree Traversal Performance

**Without Viewport Culling**
```
Small tree (120 nodes):    ~0.1ms
Medium tree (1,200 nodes): ~1ms  
Large tree (12,000 nodes): ~10ms
```

**With Viewport Culling**
```
Small tree (120 nodes):    ~0.05ms (50% improvement)
Medium tree (1,200 nodes): ~0.2ms (80% improvement)
Large tree (12,000 nodes): ~0.5ms (95% improvement)
```

#### UI Performance Guidelines

```rust
// Viewport culling implementation
fn traverse_visible_with_culling(
    node: &TreeNode,
    viewport_top: f32,
    viewport_bottom: f32,
) {
    let item_height = 24.0; // Optimized row height
    
    // Only render visible items
    if item_bottom >= viewport_top && current_y <= viewport_bottom {
        render_node(node);
    }
}
```

## Memory Optimization

### Memory Usage Patterns

#### File Content Caching
```rust
pub struct PerformanceConfig {
    pub cache_size_mb: usize,        // Default: 100MB
    pub max_concurrent_reads: usize, // Default: 32
    pub use_mmap: bool,              // Default: false (use auto-detection)
}
```

#### Memory Mapping Strategy

**Advantages of Memory Mapping**
- Reduces memory pressure for large files
- OS handles caching and paging automatically
- Faster for large files (>1MB)
- Shared memory across processes

**Disadvantages**
- Higher overhead for small files
- Platform-specific behavior
- Potential for memory fragmentation

#### Smart Memory Management

```rust
// Use Arc for shared content to reduce memory usage
pub struct OutputState {
    pub content: Option<Arc<String>>, // Shared ownership
    pub tokens: Option<TokenCount>,
    pub generating: bool,
}
```

### Memory Pressure Handling

#### Configurable Limits
```rust
impl MemorySize {
    pub const fn from_mb(mb: usize) -> Self {
        Self(mb * 1024 * 1024)
    }
    
    // Recommended limits by system RAM
    // 8GB RAM:  cache_size_mb: 100
    // 16GB RAM: cache_size_mb: 250  
    // 32GB RAM: cache_size_mb: 500
}
```

## I/O Optimization

### Parallel I/O Configuration

#### Thread Pool Sizing
```rust
// Optimal thread count varies by storage type
match storage_type {
    StorageType::SSD => num_cpus::get().min(8),     // I/O bound
    StorageType::HDD => num_cpus::get().min(4),     // Avoid seek storms
    StorageType::Network => num_cpus::get().min(2), // Network limited
}
```

#### Batch Processing Strategy

```rust
// Process files in batches to balance memory and throughput
const BATCH_SIZE: usize = 32; // Optimal for most systems

for batch in file_paths.chunks(BATCH_SIZE) {
    let results = read_files_parallel(batch, mmap_threshold);
    process_batch(results);
}
```

### Pattern Matching Optimization

#### Compiled Pattern Caching

```rust
pub struct PatternCache {
    globs: Vec<glob::Pattern>,    // Pre-compiled glob patterns
    regexes: Vec<regex::Regex>,   // Pre-compiled regex patterns
}

// Performance comparison:
// Raw string matching:     ~100ns per check
// Compiled glob patterns:  ~50ns per check  
// Compiled regex patterns: ~30ns per check
```

#### Pattern Optimization Guidelines

1. **Use compiled patterns**: Pre-compile all patterns at startup
2. **Prefer glob over regex**: Glob patterns are faster for simple cases
3. **Cache pattern results**: Avoid recompiling patterns
4. **Minimize pattern count**: Combine similar patterns where possible

## Platform-Specific Optimizations

### Windows Optimizations

#### File System Performance
```rust
// Windows-specific optimizations
#[cfg(windows)]
const OPTIMAL_THREAD_COUNT: usize = 6; // Avoid scheduler overhead

#[cfg(windows)]  
const MEMORY_MAP_THRESHOLD: u64 = 512 * 1024; // NTFS optimized
```

#### Windows-Specific Settings
- Enable long path support in manifest
- Use overlapped I/O for network drives
- Configure antivirus exclusions for temp directories

### macOS Optimizations

#### APFS Optimizations
```rust
#[cfg(target_os = "macos")]
const MEMORY_MAP_THRESHOLD: u64 = 128 * 1024; // APFS optimized

// Leverage unified buffer cache
#[cfg(target_os = "macos")]
const PREFERRED_READ_SIZE: usize = 64 * 1024; // 64KB chunks
```

### Linux Optimizations

#### Filesystem-Specific Tuning
```rust
#[cfg(target_os = "linux")]
fn optimize_for_filesystem(fs_type: &str) -> PerformanceConfig {
    match fs_type {
        "ext4" => PerformanceConfig {
            max_concurrent_reads: 32,
            cache_size_mb: 200,
            use_mmap: true,
        },
        "xfs" => PerformanceConfig {
            max_concurrent_reads: 16, // Better for large files
            cache_size_mb: 300,
            use_mmap: true,
        },
        "btrfs" => PerformanceConfig {
            max_concurrent_reads: 8,  // More conservative
            cache_size_mb: 150,
            use_mmap: false, // Avoid COW overhead
        },
        _ => PerformanceConfig::default(),
    }
}
```

## Profiling and Monitoring

### Built-in Performance Monitoring

#### Performance Overlay
```rust
pub struct PerfOverlay {
    pub enabled: bool,
    pub show_memory: bool,
    pub show_timing: bool,
    pub show_threads: bool,
}

// Accessible via Ctrl+Shift+P
impl PerfOverlay {
    pub fn render(&self, ctx: &egui::Context, app: &FsPromptApp) {
        egui::Window::new("Performance")
            .show(ctx, |ui| {
                ui.label(format!("Memory: {:.1}MB", self.memory_usage_mb()));
                ui.label(format!("Generation time: {:.1}ms", self.last_generation_time()));
                ui.label(format!("Active threads: {}", self.active_threads()));
            });
    }
}
```

### External Profiling Tools

#### CPU Profiling
```bash
# Linux: perf profiling
perf record --call-graph=dwarf ./target/release/fsprompt
perf report

# macOS: Instruments profiling  
xcrun xctrace record --template "Time Profiler" --launch ./target/release/fsprompt

# Windows: VS Diagnostic Tools
# Use Visual Studio or standalone profiler
```

#### Memory Profiling
```bash
# Valgrind (Linux/macOS)
valgrind --tool=massif ./target/release/fsprompt

# Heaptrack (Linux)
heaptrack ./target/release/fsprompt
```

## Performance Tuning Guidelines

### Application Configuration

#### Conservative Settings (Low-end hardware)
```rust
PerformanceConfig {
    max_concurrent_reads: 8,
    cache_size_mb: 50,
    use_mmap: false,
}
```

#### Balanced Settings (Typical hardware)
```rust
PerformanceConfig {
    max_concurrent_reads: 16,
    cache_size_mb: 100,
    use_mmap: true,
}
```

#### Aggressive Settings (High-end hardware)
```rust
PerformanceConfig {
    max_concurrent_reads: 32,
    cache_size_mb: 500,
    use_mmap: true,
}
```

### Runtime Optimization

#### Generation Strategy by Project Size

**Small Projects (<1,000 files)**
- Generate synchronously on main thread
- No progress indication needed
- Cache results aggressively

**Medium Projects (1,000-10,000 files)**
- Generate on background thread
- Show progress indicator
- Use incremental updates

**Large Projects (>10,000 files)**
- Generate with worker thread
- Implement cancellation
- Use streaming output
- Provide detailed progress

#### Dynamic Performance Adjustment

```rust
impl FsPromptApp {
    fn adjust_performance_settings(&mut self, project_size: usize) {
        let config = &mut self.state.config.performance;
        
        match project_size {
            0..=1000 => {
                config.max_concurrent_reads = 8;
                config.cache_size_mb = 50;
            }
            1001..=10000 => {
                config.max_concurrent_reads = 16;
                config.cache_size_mb = 100;
            }
            _ => {
                config.max_concurrent_reads = 32;
                config.cache_size_mb = 200;
            }
        }
    }
}
```

## Common Performance Issues

### Anti-patterns to Avoid

1. **Blocking the UI thread**: Always use background threads for I/O
2. **Excessive memory allocation**: Reuse buffers and use references
3. **Synchronous I/O in loops**: Batch operations and use parallel processing
4. **Unbounded recursion**: Implement depth limits for directory traversal
5. **Memory leaks**: Use RAII and avoid circular references

### Troubleshooting Performance Issues

#### Slow Directory Scanning
- Check for network-mounted directories
- Verify antivirus exclusions
- Reduce thread count for HDDs
- Enable pattern filtering

#### High Memory Usage
- Reduce cache size limits
- Enable memory mapping for large files
- Check for memory leaks in file content storage
- Implement streaming for very large outputs

#### UI Responsiveness Issues
- Ensure all I/O is on background threads
- Implement viewport culling for large trees
- Use incremental rendering for output
- Add cancellation points in long operations

## Performance Testing

### Benchmark Setup

```bash
# Run comprehensive benchmarks
cargo bench

# Specific benchmark categories
cargo bench directory_traversal
cargo bench file_reading
cargo bench token_thresholds
cargo bench ui_performance
```

### Creating Custom Benchmarks

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_custom_operation(c: &mut Criterion) {
    c.bench_function("custom_operation", |b| {
        b.iter(|| {
            // Your operation here
        });
    });
}

criterion_group!(benches, benchmark_custom_operation);
criterion_main!(benches);
```

### Performance Regression Testing

1. **Baseline establishment**: Record benchmark results for each release
2. **Automated testing**: Run benchmarks in CI/CD pipeline  
3. **Threshold monitoring**: Alert on >10% performance regressions
4. **Platform comparison**: Compare results across Windows/macOS/Linux