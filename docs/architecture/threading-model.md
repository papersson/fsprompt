# Threading Model & Concurrency Architecture

## Overview: UI Thread Protection

fsPrompt uses a **UI-thread-first** architecture where the main thread is dedicated to UI responsiveness, while all heavy operations are delegated to worker threads. This ensures the interface never freezes, even when processing thousands of files.

## Threading Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Main UI Thread                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Event Loop  │  │ State Mgmt  │  │ UI Rendering        │  │
│  │ (egui)      │  │ (AppState)  │  │ (60+ FPS)           │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
          │                    ▲
          │ Commands           │ Events & Progress
          ▼                    │
┌─────────────────────────────────────────────────────────────┐
│                   Worker Thread Pool                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Generator   │  │ File I/O    │  │ Filesystem          │  │
│  │ Worker      │  │ Pool        │  │ Watcher             │  │
│  │             │  │ (Rayon)     │  │ (notify)            │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┐
```

## Worker Communication

### Message Passing Architecture

The system uses **crossbeam channels** for type-safe, lock-free communication:

```rust
/// Commands sent to worker threads
#[derive(Debug, Clone)]
pub enum WorkerCommand {
    GenerateOutput {
        root_path: CanonicalPath,
        selected_files: Vec<CanonicalPath>,
        format: OutputFormat,
        include_tree: bool,
        ignore_patterns: PatternString,
    },
    Cancel,
}

/// Events sent from worker threads
#[derive(Debug, Clone)]
pub enum WorkerEvent {
    Progress {
        stage: ProgressStage,
        progress: ProgressCount,
    },
    OutputReady {
        content: String,
        token_count: TokenCount,
    },
    Error(String),
    Cancelled,
}
```

### Worker Handle Pattern

```rust
pub struct WorkerHandle {
    sender: Sender<WorkerCommand>,
    receiver: Receiver<WorkerEvent>,
}

impl WorkerHandle {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = crossbeam::channel::unbounded();
        let (event_tx, event_rx) = crossbeam::channel::unbounded();

        // Spawn worker thread
        std::thread::spawn(move || {
            generator::run_worker(cmd_rx, event_tx);
        });

        Self {
            sender: cmd_tx,
            receiver: event_rx,
        }
    }
    
    pub fn send_command(&self, command: WorkerCommand) -> Result<()> {
        self.sender.send(command)
    }
    
    pub fn try_recv_event(&self) -> Option<WorkerEvent> {
        self.receiver.try_recv().ok()
    }
}
```

**Design Benefits**:
- **Non-blocking**: UI thread never waits for workers
- **Type-safe**: All messages are strongly typed
- **Error isolation**: Worker failures don't crash UI
- **Cancellation**: Graceful operation termination

## Generation Worker Detail

### Multi-Stage Pipeline

The output generation follows a structured pipeline with progress reporting:

```rust
pub fn run_worker(cmd_rx: Receiver<WorkerCommand>, event_tx: Sender<WorkerEvent>) {
    while let Ok(command) = cmd_rx.recv() {
        match command {
            WorkerCommand::GenerateOutput { root_path, selected_files, .. } => {
                // Stage 1: Scanning
                send_progress(ProgressStage::ScanningFiles, 0, total);
                
                // Stage 2: Parallel file reading
                let file_contents = read_files_parallel(selected_files);
                
                // Stage 3: Output generation
                send_progress(ProgressStage::BuildingOutput, 0, 1);
                let output = generate_output_format(file_contents, format);
                
                // Send result
                event_tx.send(WorkerEvent::OutputReady { content: output, .. });
            }
            WorkerCommand::Cancel => {
                cancelled.store(true, Ordering::Relaxed);
                event_tx.send(WorkerEvent::Cancelled);
            }
        }
    }
}
```

### Parallel File I/O

File reading uses **Rayon** for data parallelism:

```rust
fn generate_output(selected_files: Vec<CanonicalPath>, cancelled: Arc<AtomicBool>) {
    let processed = Arc::new(AtomicUsize::new(0));
    
    let file_contents: Vec<(CanonicalPath, Result<String, String>)> = selected_files
        .par_iter()  // Parallel iterator
        .map(|path| {
            // Check for cancellation
            if cancelled.load(Ordering::Relaxed) {
                return (path.clone(), Err("Cancelled".to_string()));
            }
            
            // Read file
            let result = fs::read_to_string(path.as_path())
                .map_err(|e| format!("Failed to read file: {}", e));
            
            // Update progress atomically
            let current = processed.fetch_add(1, Ordering::Relaxed) + 1;
            send_progress(ProgressStage::ReadingFiles, current, total);
            
            (path.clone(), result)
        })
        .collect();
}
```

**Performance Characteristics**:
- **CPU Utilization**: Scales with available cores
- **Memory Efficiency**: Processes files incrementally
- **Progress Granularity**: Per-file progress updates
- **Cancellation**: Checked at file boundaries

## Filesystem Watcher

### Async Change Detection

The filesystem watcher runs in its own thread using the `notify` crate:

```rust
pub struct FsWatcher {
    watcher: Option<notify::RecommendedWatcher>,
    receiver: Option<Receiver<WatcherEvent>>,
}

impl FsWatcher {
    pub fn watch_directory(&mut self, path: &CanonicalPath) -> Result<()> {
        let (tx, rx) = crossbeam::channel::unbounded();
        
        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(event) => {
                    let paths: Vec<CanonicalPath> = event.paths
                        .into_iter()
                        .filter_map(|p| CanonicalPath::new(p).ok())
                        .collect();
                    
                    let _ = tx.send(WatcherEvent::Changed(paths));
                }
                Err(e) => {
                    let _ = tx.send(WatcherEvent::Error(e.to_string()));
                }
            }
        })?;
        
        watcher.watch(path.as_path(), RecursiveMode::Recursive)?;
        
        self.watcher = Some(watcher);
        self.receiver = Some(rx);
        Ok(())
    }
    
    pub fn check_events(&mut self) -> Option<WatcherEvent> {
        self.receiver.as_ref()?.try_recv().ok()
    }
}
```

**Integration with UI**:
```rust
impl FsPromptApp {
    pub fn check_fs_changes(&mut self, ctx: &egui::Context) {
        if let Some(event) = self.fs_watcher.check_events() {
            match event {
                WatcherEvent::Changed(paths) => {
                    self.files_changed = true;
                    self.toast_manager.info(format!("{} files changed", paths.len()));
                    ctx.request_repaint(); // Trigger UI update
                }
                WatcherEvent::Error(e) => {
                    self.toast_manager.error(format!("Watcher error: {}", e));
                }
            }
        }
    }
}
```

## Cancellation Strategy

### Cooperative Cancellation

Workers use atomic flags for graceful cancellation:

```rust
fn process_files(files: &[CanonicalPath], cancelled: Arc<AtomicBool>) {
    for file in files {
        // Check cancellation at boundaries
        if cancelled.load(Ordering::Relaxed) {
            return Err("Operation cancelled");
        }
        
        // Process file...
    }
}
```

### Cancellation Points

Strategic cancellation checks at:
- **File boundaries**: Between file processing
- **Progress updates**: During progress reporting
- **I/O operations**: Before expensive operations
- **Loop iterations**: In long-running loops

## Memory Management

### Shared Data Strategy

```rust
/// Shared output content (reference counted)
pub struct OutputState {
    pub content: Option<Arc<String>>,  // Shared ownership
    pub tokens: Option<TokenCount>,
    pub generating: bool,
}
```

**Benefits**:
- **Zero-copy sharing**: UI displays without duplicating large strings
- **Memory efficiency**: Single allocation for output content
- **Thread safety**: `Arc` provides safe sharing across threads

### Memory Pressure Handling

```rust
/// Memory size tracking for large operations
#[derive(Debug, Clone, Copy)]
pub struct MemorySize(usize);

impl MemorySize {
    pub const fn from_mb(mb: usize) -> Self {
        Self(mb * 1024 * 1024)
    }
    
    pub const fn exceeds_limit(&self, limit: MemorySize) -> bool {
        self.0 > limit.0
    }
}

// Usage in file reading
if estimated_size.exceeds_limit(MemorySize::from_mb(100)) {
    // Switch to streaming/mmap strategy
    use_memory_mapped_approach(path)
} else {
    // Direct read is fine
    fs::read_to_string(path)
}
```

## Performance Monitoring

### Thread-Safe Metrics

```rust
pub struct FrameTimer {
    frame_times: Arc<[AtomicU64; 120]>,  // Lock-free circular buffer
    position: Arc<AtomicUsize>,
    last_frame: Arc<AtomicU64>,
}

impl FrameTimer {
    pub fn frame_start(&self) {
        let now = get_timestamp();
        let last = self.last_frame.swap(now, Ordering::Relaxed);
        
        if last > 0 {
            let frame_time = now - last;
            let pos = self.position.fetch_add(1, Ordering::Relaxed) % 120;
            self.frame_times[pos].store(frame_time, Ordering::Relaxed);
        }
    }
}
```

**Characteristics**:
- **Lock-free**: No blocking between UI and monitoring
- **Low overhead**: Single atomic operation per frame
- **Rolling metrics**: Maintains recent performance history

## Error Propagation

### Thread-Safe Error Handling

```rust
pub enum WorkerEvent {
    Error(String),  // Simplified error representation
    // ...
}

// In worker thread
match file_operation() {
    Ok(result) => process_result(result),
    Err(e) => {
        // Convert complex errors to simple strings for transport
        let _ = event_tx.send(WorkerEvent::Error(e.to_string()));
        continue; // Don't stop processing other files
    }
}

// In UI thread
match worker_event {
    WorkerEvent::Error(msg) => {
        self.toast_manager.error(msg);
        // UI continues running
    }
    // ...
}
```

## Thread Lifecycle Management

### Graceful Shutdown

```rust
impl FsPromptApp {
    pub fn on_exit(&mut self) {
        // Stop filesystem watcher
        self.fs_watcher.stop();
        
        // Cancel any running operations
        let _ = self.worker.send_command(WorkerCommand::Cancel);
        
        // Save configuration
        self.save_config();
        
        // Worker threads will terminate when main thread exits
    }
}
```

## Concurrency Benefits

1. **Responsive UI**: Never blocks on file operations
2. **Scalable Performance**: Utilizes all available CPU cores
3. **Graceful Degradation**: Partial failures don't stop the application
4. **Real-time Feedback**: Progress updates during long operations
5. **Resource Efficiency**: Memory-conscious file processing

This threading model provides excellent performance while maintaining UI responsiveness and system stability.