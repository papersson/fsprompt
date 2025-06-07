# Worker API Reference

Complete reference for the worker command/event protocol used in fsPrompt's background processing system (`src/workers/`).

## Overview

fsPrompt uses a worker thread system to perform CPU-intensive operations (file scanning, content generation) without blocking the UI. The worker API provides a type-safe, channel-based communication protocol between the main thread and worker threads.

## Core Components

### `WorkerHandle`

Main interface for communicating with worker threads.

```rust
pub struct WorkerHandle {
    sender: Sender<WorkerCommand>,
    receiver: Receiver<WorkerEvent>,
}
```

#### Methods

```rust
impl WorkerHandle {
    /// Create a new worker handle and spawn worker thread
    pub fn new() -> Self
    
    /// Send command to worker thread
    pub fn send_command(&self, command: WorkerCommand) -> Result<(), crossbeam::channel::SendError<WorkerCommand>>
    
    /// Try to receive event from worker thread
    pub fn try_recv_event(&self) -> Option<WorkerEvent>
}
```

#### Usage Example

```rust
use crate::workers::{WorkerHandle, WorkerCommand, OutputFormat};

// Create worker
let worker = WorkerHandle::new();

// Send generation command
let command = WorkerCommand::GenerateOutput {
    root_path: root_directory,
    selected_files: vec![file1, file2, file3],
    format: OutputFormat::Xml,
    include_tree: true,
    ignore_patterns: PatternString::new("*.log,node_modules".to_string()),
};

worker.send_command(command)?;

// Poll for events in main loop
while let Some(event) = worker.try_recv_event() {
    match event {
        WorkerEvent::Progress { stage, progress } => {
            update_progress_ui(stage, progress);
        }
        WorkerEvent::OutputReady { content, token_count } => {
            display_output(content, token_count);
            break;
        }
        WorkerEvent::Error(msg) => {
            show_error(msg);
            break;
        }
        WorkerEvent::Cancelled => {
            show_cancelled_message();
            break;
        }
    }
}
```

## Commands

### `WorkerCommand`

Commands sent to worker threads to initiate operations.

```rust
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
```

#### `WorkerCommand::GenerateOutput`

Initiates output generation from selected files.

**Parameters:**
- `root_path: CanonicalPath` - Root directory for relative path calculation
- `selected_files: Vec<CanonicalPath>` - List of files to include in output  
- `format: OutputFormat` - Output format (XML or Markdown)
- `include_tree: bool` - Whether to include directory tree in output
- `ignore_patterns: PatternString` - Comma-separated ignore patterns for tree generation

**Example:**

```rust
let command = WorkerCommand::GenerateOutput {
    root_path: CanonicalPath::new("/project/root")?,
    selected_files: vec![
        CanonicalPath::new("/project/root/src/main.rs")?,
        CanonicalPath::new("/project/root/src/lib.rs")?,
    ],
    format: OutputFormat::Markdown,
    include_tree: true,
    ignore_patterns: PatternString::new("target,*.log,.git".to_string()),
};

worker.send_command(command)?;
```

#### `WorkerCommand::Cancel`

Cancels the currently running operation.

**Example:**

```rust
// User clicked cancel button
worker.send_command(WorkerCommand::Cancel)?;
```

## Events

### `WorkerEvent`

Events sent from worker threads to report progress and results.

```rust
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

#### `WorkerEvent::Progress`

Reports progress during operation execution.

**Fields:**
- `stage: ProgressStage` - Current operation stage
- `progress: ProgressCount` - Progress within current stage

**Usage:**

```rust
match event {
    WorkerEvent::Progress { stage, progress } => {
        let percentage = progress.percentage();
        let stage_name = match stage {
            ProgressStage::ScanningFiles => "Scanning files...",
            ProgressStage::ReadingFiles => "Reading file contents...",
            ProgressStage::BuildingOutput => "Building output...",
        };
        
        update_progress_bar(percentage);
        update_status_text(format!("{} ({}/{})", stage_name, progress.current(), progress.total()));
    }
}
```

#### `WorkerEvent::OutputReady`

Indicates successful completion with generated output.

**Fields:**
- `content: String` - Generated output content
- `token_count: TokenCount` - Estimated token count for the content

**Usage:**

```rust
match event {
    WorkerEvent::OutputReady { content, token_count } => {
        // Display the generated content
        output_text_area.set_text(content);
        
        // Update token count display
        token_display.set_text(format!("Tokens: {} ({})", 
            token_count.get(), 
            match token_count.level() {
                TokenLevel::Low => "Low",
                TokenLevel::Medium => "Medium", 
                TokenLevel::High => "High",
            }
        ));
        
        // Enable copy/save buttons
        copy_button.set_enabled(true);
        save_button.set_enabled(true);
    }
}
```

#### `WorkerEvent::Error`

Reports an error during operation execution.

**Fields:**
- `String` - Error message

**Usage:**

```rust
match event {
    WorkerEvent::Error(message) => {
        // Show error to user
        show_error_toast(format!("Generation failed: {}", message));
        
        // Reset UI state
        progress_bar.set_visible(false);
        generate_button.set_enabled(true);
    }
}
```

#### `WorkerEvent::Cancelled`

Confirms that the operation was successfully cancelled.

**Usage:**

```rust
match event {
    WorkerEvent::Cancelled => {
        // Show cancellation feedback
        show_info_toast("Operation cancelled");
        
        // Reset UI state
        progress_bar.set_visible(false);
        cancel_button.set_enabled(false);
        generate_button.set_enabled(true);
    }
}
```

## Progress Stages

### `ProgressStage`

Represents different stages of the output generation process.

```rust
pub enum ProgressStage {
    ScanningFiles,   // Scanning filesystem
    ReadingFiles,    // Reading file contents
    BuildingOutput,  // Building final output
}
```

#### Stage Descriptions

- **`ScanningFiles`**: Walking the directory tree to find files (when generating tree view)
- **`ReadingFiles`**: Reading contents of selected files in parallel
- **`BuildingOutput`**: Formatting content into final XML/Markdown output

#### Progress Reporting Pattern

Each stage reports progress differently:

```rust
// Stage 1: File count determined upfront
WorkerEvent::Progress {
    stage: ProgressStage::ScanningFiles,
    progress: ProgressCount::new(0, selected_files.len()),
}

// Stage 2: Progress increments as files are read
WorkerEvent::Progress {
    stage: ProgressStage::ReadingFiles, 
    progress: ProgressCount::new(files_read, total_files),
}

// Stage 3: Single step for output formatting
WorkerEvent::Progress {
    stage: ProgressStage::BuildingOutput,
    progress: ProgressCount::new(0, 1),
}
// Then immediately:
WorkerEvent::Progress {
    stage: ProgressStage::BuildingOutput,
    progress: ProgressCount::new(1, 1),
}
```

## Worker Implementation

### `run_worker`

Main worker thread function that processes commands.

```rust
pub fn run_worker(cmd_rx: Receiver<WorkerCommand>, event_tx: Sender<WorkerEvent>)
```

**Parameters:**
- `cmd_rx` - Channel to receive commands from main thread
- `event_tx` - Channel to send events back to main thread

**Behavior:**
- Runs in infinite loop receiving commands
- Handles cancellation via atomic boolean flag
- Automatically sends progress updates during long operations
- Uses parallel processing for file reading

## Output Generation Process

### File Reading Strategy

The worker uses parallel file reading with automatic strategy selection:

```rust
// Files are read in parallel using rayon
let file_contents: Vec<(CanonicalPath, Result<String, String>)> = selected_files
    .par_iter()
    .map(|path| {
        // Check for cancellation
        if cancelled.load(Ordering::Relaxed) {
            return (path.clone(), Err("Cancelled".to_string()));
        }
        
        // Read file content
        let result = fs::read_to_string(path.as_path())
            .map_err(|e| format!("Failed to read file: {}", e));
            
        // Report progress
        let current = processed.fetch_add(1, Ordering::Relaxed) + 1;
        let _ = event_tx.send(WorkerEvent::Progress {
            stage: ProgressStage::ReadingFiles,
            progress: ProgressCount::new(current, total_files),
        });
        
        (path.clone(), result)
    })
    .collect();
```

### Tree Generation

When `include_tree` is true, the worker generates a filtered directory tree:

```rust
fn generate_filtered_tree_string(root_path: &Path, ignore_patterns: &[String]) -> String {
    // Compile patterns for efficient matching
    let patterns: Vec<Pattern> = ignore_patterns
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();
    
    let mut output = String::new();
    generate_filtered_tree_recursive(root_path, &mut output, "", true, 0, &patterns);
    output
}
```

### Output Formatting

The worker supports two output formats:

#### XML Format

```xml
<?xml version="1.0" encoding="UTF-8"?>
<codebase>
  <directory_tree>
<![CDATA[
📁 project-root/
├── 📁 src/
│   ├── 📄 main.rs
│   └── 📄 lib.rs
└── 📄 Cargo.toml
]]>
  </directory_tree>

  <files>
    <file path="src/main.rs">
<![CDATA[
fn main() {
    println!("Hello, world!");
}
]]>
    </file>
  </files>
</codebase>
```

#### Markdown Format

```markdown
# Codebase Export

## Directory Structure

```
📁 project-root/
├── 📁 src/
│   ├── 📄 main.rs
│   └── 📄 lib.rs
└── 📄 Cargo.toml
```

## Files

### src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```
```

### Language Detection

The worker automatically detects programming languages for syntax highlighting:

```rust
let lang = match extension {
    "rs" => "rust",
    "js" => "javascript", 
    "ts" => "typescript",
    "py" => "python",
    "java" => "java",
    // ... more languages
    _ => "",
};
```

## Error Handling

### Common Error Scenarios

1. **File Read Errors**: Files that cannot be read are reported but don't stop the process
2. **Permission Errors**: Handled gracefully with descriptive error messages
3. **Cancellation**: Operations can be cancelled at any point
4. **Memory Limits**: Large files are handled efficiently

### Error Reporting Pattern

```rust
// Collect failed files for summary reporting
let mut failed_files = Vec::new();

for (path, content_result) in &file_contents {
    match content_result {
        Ok(content) => {
            // Process successful file
        }
        Err(e) => {
            failed_files.push(format!("{}: {}", path_str, e));
        }
    }
}

// Report summary of failures
if !failed_files.is_empty() {
    let error_msg = format!(
        "Warning: Failed to read {} file(s): {}",
        failed_files.len(),
        failed_files.join(", ")
    );
    let _ = event_tx.send(WorkerEvent::Error(error_msg));
}
```

## Best Practices

### Command Handling

```rust
// Always handle cancellation
if cancelled.load(Ordering::Relaxed) {
    let _ = event_tx.send(WorkerEvent::Cancelled);
    return;
}

// Report progress frequently during long operations
let current = processed.fetch_add(1, Ordering::Relaxed) + 1;
let _ = event_tx.send(WorkerEvent::Progress {
    stage: current_stage,
    progress: ProgressCount::new(current, total),
});
```

### Event Sending

```rust
// Always use `let _ =` for event sending since the receiver might be dropped
let _ = event_tx.send(WorkerEvent::Progress { /* ... */ });

// Send final success event only if not cancelled
if !cancelled.load(Ordering::Relaxed) {
    let _ = event_tx.send(WorkerEvent::OutputReady {
        content: output,
        token_count,
    });
}
```

### Resource Management

```rust
// Use parallel iterators for CPU-bound work
selected_files
    .par_iter()
    .map(|path| {
        // Work done in parallel
    })
    .collect()

// Check cancellation in long loops
for item in large_collection {
    if cancelled.load(Ordering::Relaxed) {
        return;
    }
    // Process item
}
```

## Integration Example

Complete example of integrating the worker API in a UI application:

```rust
use crate::workers::{WorkerHandle, WorkerCommand, WorkerEvent, ProgressStage};

struct App {
    worker: WorkerHandle,
    is_generating: bool,
    progress: Option<ProgressCount>,
    current_stage: Option<ProgressStage>,
    output_content: Option<String>,
    token_count: Option<TokenCount>,
}

impl App {
    fn new() -> Self {
        Self {
            worker: WorkerHandle::new(),
            is_generating: false,
            progress: None,
            current_stage: None,
            output_content: None,
            token_count: None,
        }
    }
    
    fn start_generation(&mut self, selected_files: Vec<CanonicalPath>) {
        let command = WorkerCommand::GenerateOutput {
            root_path: self.root_path.clone(),
            selected_files,
            format: self.output_format,
            include_tree: self.include_tree,
            ignore_patterns: self.ignore_patterns.clone(),
        };
        
        if self.worker.send_command(command).is_ok() {
            self.is_generating = true;
            self.progress = None;
            self.output_content = None;
        }
    }
    
    fn cancel_generation(&mut self) {
        let _ = self.worker.send_command(WorkerCommand::Cancel);
    }
    
    fn update(&mut self) {
        // Process all pending events
        while let Some(event) = self.worker.try_recv_event() {
            match event {
                WorkerEvent::Progress { stage, progress } => {
                    self.current_stage = Some(stage);
                    self.progress = Some(progress);
                }
                WorkerEvent::OutputReady { content, token_count } => {
                    self.is_generating = false;
                    self.progress = None;
                    self.current_stage = None;
                    self.output_content = Some(content);
                    self.token_count = Some(token_count);
                }
                WorkerEvent::Error(msg) => {
                    self.is_generating = false;
                    self.progress = None;
                    self.current_stage = None;
                    self.show_error(&msg);
                }
                WorkerEvent::Cancelled => {
                    self.is_generating = false;
                    self.progress = None;
                    self.current_stage = None;
                    self.show_info("Generation cancelled");
                }
            }
        }
    }
}
```