# Design Decisions Reference

This document explains the architectural choices, trade-offs, and rationale behind key design decisions in fsPrompt.

## Type System Architecture

### Newtype Pattern for Domain Safety

**Decision**: Use newtypes extensively instead of primitive types

**Rationale**:
```rust
// Instead of:
fn process_file(path: PathBuf, size: u64, tokens: usize) -> Result<String, Error>

// We use:
fn process_file(path: CanonicalPath, size: FileSize, tokens: TokenCount) -> Result<String, Error>
```

**Benefits**:
- **Compile-time safety**: Impossible to mix up parameters
- **Domain clarity**: Code expresses intent clearly
- **Refactoring safety**: Changes to one type don't affect others
- **Validation boundaries**: Each type enforces its own invariants

**Trade-offs**:
- Slightly more verbose code
- Additional memory overhead (minimal)
- Learning curve for new contributors

**Examples from `src/core/types.rs`**:
```rust
pub struct CanonicalPath(PathBuf);    // Security-validated paths
pub struct TokenCount(usize);         // LLM token estimates  
pub struct FileSize(u64);             // File size with read strategy
pub struct GenerationTime(Duration);  // Performance tracking
```

### Separation of Data and UI State

**Decision**: Separate pure data types from UI state

**Rationale**: 
```rust
// Pure data (no UI concerns)
pub struct FsEntry {
    pub path: CanonicalPath,
    pub name: String,
    pub entry_type: FsEntryType,
}

// UI state (separate)
pub enum SelectionState {
    Unchecked,
    Checked,
    Indeterminate,
}
```

**Benefits**:
- **Testability**: Data logic independent of UI framework
- **Serialization**: Pure data can be saved/loaded easily
- **Performance**: UI state changes don't affect data processing
- **Maintainability**: Clear separation of concerns

**Implementation Strategy**:
- Data types in `core/types.rs` are UI-agnostic
- UI state types manage display and interaction
- Conversion methods bridge the gap when needed

### Builder Pattern for Complex Configuration

**Decision**: Use builder pattern for configuration objects

**Rationale**:
```rust
let config = AppConfigBuilder::new()
    .window(WindowConfigBuilder::new()
        .dimensions(1920.0, 1080.0)
        .left_pane_ratio(0.4)
        .build())
    .add_ignore_pattern("*.log".to_string())
    .performance(PerformanceConfig {
        max_concurrent_reads: 32,
        cache_size_mb: 200,
        use_mmap: true,
    })
    .build();
```

**Benefits**:
- **Flexibility**: Can construct configurations incrementally
- **Defaults**: Sensible defaults with selective overrides
- **Validation**: Builders can validate during construction
- **Readability**: Self-documenting configuration code

**Alternative Considered**: Direct struct initialization
**Why Rejected**: Too verbose and error-prone for complex configurations

## Security Architecture

### Canonical Path Validation Strategy

**Decision**: All user-provided paths must use `CanonicalPath` type

**Rationale**:
```rust
impl CanonicalPath {
    pub fn new_within_root(path: impl AsRef<Path>, root: &CanonicalPath) -> Result<Self> {
        let canonical = Self::new(path)?;
        if !canonical.is_contained_within(root) {
            return Err(/* path traversal error */);
        }
        Ok(canonical)
    }
}
```

**Benefits**:
- **Security**: Prevents path traversal attacks at type level
- **Consistency**: All paths are normalized and validated
- **Performance**: Canonicalization happens once per path
- **Debugging**: Easier to track path-related issues

**Trade-offs**:
- **I/O overhead**: Canonicalization requires filesystem access
- **Error handling**: More complex error paths
- **Platform differences**: Canonicalization behavior varies

**Alternative Considered**: Runtime validation at usage points
**Why Rejected**: Error-prone and doesn't prevent all attack vectors

### Symlink Handling Policy

**Decision**: Disable symlink following in directory traversal

**Rationale**:
```rust
builder.follow_links(false); // Security and performance
```

**Benefits**:
- **Security**: Prevents symlink-based directory escape
- **Performance**: Avoids potential infinite loops
- **Predictability**: Consistent behavior across platforms
- **Simplicity**: Easier to reason about file access patterns

**Trade-offs**:
- **Functionality**: Some legitimate use cases not supported
- **User expectations**: May surprise users expecting symlink support

**Alternative Considered**: Selective symlink following with depth limits
**Why Rejected**: Complex implementation with marginal benefit

## Performance Architecture

### Memory Mapping Strategy

**Decision**: Automatic memory mapping based on file size threshold

**Rationale**:
```rust
const MEMORY_MAP_THRESHOLD: u64 = 256 * 1024; // 256KB

impl FileSize {
    pub const fn read_strategy(&self) -> FileReadStrategy {
        if self.0 < MEMORY_MAP_THRESHOLD {
            FileReadStrategy::Direct
        } else {
            FileReadStrategy::MemoryMapped
        }
    }
}
```

**Benefits**:
- **Performance**: Optimal strategy for different file sizes
- **Memory efficiency**: Large files don't consume heap space
- **OS integration**: Leverages OS-level caching
- **Simplicity**: Automatic selection requires no user configuration

**Trade-offs**:
- **Platform variance**: Memory mapping behavior differs across OS
- **Complexity**: Two code paths to maintain
- **Edge cases**: Threshold may not be optimal for all scenarios

**Alternative Considered**: Always use one strategy
**Why Rejected**: Significant performance penalty for large files

### Parallel Processing Design

**Decision**: Use Rayon for data parallelism with bounded thread pools

**Rationale**:
```rust
// Thread count optimization
builder.threads(num_cpus::get().min(8));

// Parallel file processing
file_paths.par_iter().map(|path| {
    // Process each file in parallel
}).collect()
```

**Benefits**:
- **Performance**: Scales with available CPU cores
- **Simplicity**: Rayon handles work distribution automatically
- **Safety**: Data parallelism avoids shared mutable state
- **Resource management**: Bounded thread pools prevent resource exhaustion

**Trade-offs**:
- **Memory usage**: Multiple threads increase memory pressure
- **Complexity**: Debugging parallel code is harder
- **Platform differences**: Optimal thread count varies

**Alternative Considered**: Manual thread management
**Why Rejected**: Complex implementation with little benefit over Rayon

### UI Rendering Optimization

**Decision**: Implement viewport culling for large directory trees

**Rationale**:
```rust
fn traverse_visible_with_culling(
    node: &TreeNode,
    viewport_top: f32,
    viewport_bottom: f32,
) {
    // Only render items visible in viewport
    if item_bottom >= viewport_top && current_y <= viewport_bottom {
        render_item(node);
    }
}
```

**Benefits**:
- **Performance**: Scales to very large directory trees
- **Responsiveness**: UI remains smooth with thousands of items
- **Memory efficiency**: Only visible items consume UI resources

**Trade-offs**:
- **Complexity**: More complex rendering logic
- **Accuracy**: Scroll position calculations must be precise
- **Testing**: Harder to test viewport culling logic

**Alternative Considered**: Virtual scrolling with fixed item heights
**Why Rejected**: Less flexible for variable-height items

## Error Handling Strategy

### Type-Safe Error Propagation

**Decision**: Use specific error types instead of generic errors

**Rationale**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    FontSizeOutOfRange,
    RatioOutOfRange,
    EmptyString,
}

impl std::error::Error for ValidationError {}
```

**Benefits**:
- **Type safety**: Compile-time guarantee of error handling
- **Debugging**: Specific error types aid troubleshooting
- **Recovery**: Different error types enable different recovery strategies
- **API clarity**: Functions clearly indicate possible failure modes

**Trade-offs**:
- **Verbosity**: More code required for error definitions
- **Maintenance**: Error types must evolve with code changes

**Alternative Considered**: Generic `anyhow::Error` throughout
**Why Rejected**: Loses type information and compile-time guarantees

### Graceful Degradation Policy

**Decision**: Prefer empty results over errors for security violations

**Rationale**:
```rust
let canonical_root = match CanonicalPath::new(root) {
    Ok(cr) => cr,
    Err(_) => return Vec::new(), // Return empty instead of error
};
```

**Benefits**:
- **Security**: Prevents information disclosure through error messages
- **User experience**: Application continues to function
- **Robustness**: Handles edge cases gracefully

**Trade-offs**:
- **Debugging**: Silent failures can be harder to diagnose
- **User feedback**: Users may not understand why results are empty

## Concurrency Model

### Worker Thread Architecture

**Decision**: Use dedicated worker threads for I/O operations

**Rationale**:
```rust
pub struct WorkerHandle {
    sender: Option<mpsc::Sender<WorkerCommand>>,
    receiver: mpsc::Receiver<WorkerEvent>,
}
```

**Benefits**:
- **Responsiveness**: UI thread never blocks on I/O
- **Cancellation**: Long operations can be cancelled
- **Progress reporting**: Worker can report progress incrementally
- **Error isolation**: Worker thread errors don't crash UI

**Trade-offs**:
- **Complexity**: Message passing between threads
- **Latency**: Communication overhead for small operations
- **Resource usage**: Additional threads consume memory

**Alternative Considered**: Async/await with tokio
**Why Rejected**: egui doesn't integrate well with async runtimes

### State Management Strategy

**Decision**: Centralized state with immutable snapshots for undo/redo

**Rationale**:
```rust
pub struct HistoryManager<T> {
    undo_stack: Vec<T>,
    redo_stack: Vec<T>,
    max_size: HistorySize,
}

// Capture state snapshot
let snapshot = self.capture_snapshot();
self.history_manager.push(snapshot);
```

**Benefits**:
- **Simplicity**: Single source of truth for application state
- **Undo/redo**: Natural support for history operations
- **Testing**: Easy to test with known state inputs
- **Debugging**: State changes are explicit and traceable

**Trade-offs**:
- **Memory usage**: History snapshots consume memory
- **Performance**: Large state snapshots can be expensive
- **Granularity**: Need to choose appropriate snapshot points

**Alternative Considered**: Event sourcing with command pattern
**Why Rejected**: Overly complex for this application's needs

## UI Framework Selection

### egui Choice Rationale

**Decision**: Use egui for immediate mode GUI

**Rationale**:
- **Simplicity**: Immediate mode is easier to reason about
- **Performance**: Good performance for text-heavy applications
- **Cross-platform**: Single codebase for all platforms
- **Rust ecosystem**: Native Rust with good integration

**Trade-offs**:
- **Flexibility**: Less flexible than retained mode GUIs
- **Styling**: Limited theming compared to web technologies
- **Ecosystem**: Smaller ecosystem than established GUI frameworks

**Alternatives Considered**:
- **Tauri**: Web technologies in Rust wrapper
- **iced**: Retained mode with Elm architecture
- **gtk-rs**: Native GTK bindings

**Why egui chosen**: Best balance of simplicity, performance, and Rust integration

## Configuration and Persistence

### Configuration File Strategy

**Decision**: Use TOML configuration files with builder pattern

**Rationale**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub ui: UiConfig,
    pub ignore_patterns: Vec<String>,
    pub performance: PerformanceConfig,
}
```

**Benefits**:
- **Human readable**: TOML is easy to edit manually
- **Type safety**: Serde provides compile-time validation
- **Versioning**: Struct evolution with backwards compatibility
- **Defaults**: Builder pattern provides sensible defaults

**Trade-offs**:
- **Migration**: Config schema changes require migration logic
- **Validation**: Runtime validation needed for some constraints

**Alternative Considered**: JSON configuration
**Why Rejected**: Less human-friendly for manual editing

### State Persistence Policy

**Decision**: Persist only user preferences, not application state

**Rationale**:
- **Privacy**: Don't save potentially sensitive file paths
- **Simplicity**: Fresh start for each session
- **Security**: Avoid storing information that could leak data

**Benefits**:
- **Security**: No sensitive data in config files
- **Simplicity**: No complex state migration logic
- **Performance**: Faster startup without state restoration

**Trade-offs**:
- **User experience**: Users must reselect directories each session
- **Productivity**: Some workflow optimization lost

## Future Design Considerations

### Extensibility Planning

**Prepared for**:
- Plugin system through trait objects
- Additional output formats via enum extension
- Custom ignore pattern types
- Platform-specific optimizations

**Architecture Supports**:
- Multiple worker threads for different operations
- Streaming output for very large codebases
- Custom UI themes through configuration
- Alternative storage backends for configuration

### Performance Scaling

**Current Limits**:
- ~50,000 files in single directory tree
- ~500MB total file content in memory
- ~10 concurrent worker threads

**Scaling Strategies**:
- Lazy loading for very large trees
- Streaming output generation
- Database backend for very large codebases
- Distributed processing for enterprise use

These design decisions reflect the current requirements and constraints of fsPrompt. As the application evolves, some decisions may need to be revisited based on new requirements, performance data, or user feedback.