# System Architecture Overview

## High-Level Design

fsPrompt is built as a modern Rust application using a **type-driven architecture** with clear separation of concerns. The system follows these core principles:

- **Type Safety First**: All domain concepts are represented as newtypes to prevent bugs
- **Reactive UI**: Immediate mode GUI that responds to state changes
- **Background Processing**: Heavy operations run on worker threads
- **Performance-Oriented**: Parallel processing and optimized file I/O

## Component Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         UI Layer                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Tree Panel  │  │ Output Panel│  │ Performance Overlay │  │
│  │ (egui)      │  │ (egui)      │  │ (debug builds)      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Application State                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Selection   │  │ Search      │  │ Output              │  │
│  │ Tracker     │  │ State       │  │ State               │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Core Types System                     │
│  All domain concepts as newtypes (CanonicalPath,           │
│  TokenCount, FileSize, ProgressCount, etc.)                │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Worker Thread Layer                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ File        │  │ Generator   │  │ Filesystem          │  │
│  │ Scanner     │  │ Worker      │  │ Watcher             │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Platform & I/O Layer                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ File System │  │ Clipboard   │  │ Configuration       │  │
│  │ Operations  │  │ Access      │  │ Persistence         │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Data Flow

### 1. User Interaction Flow
```
User Action → UI Event → State Mutation → Type-Safe Operation → Background Work → Result
```

### 2. File Processing Pipeline
```
Directory Selection → Parallel Scan → Pattern Filtering → Selection UI → 
Parallel Read → Content Processing → Output Generation → UI Update
```

### 3. State Management
- **Immutable Core Types**: All domain types are immutable value types
- **Mutable Application State**: Centralized in `AppState` with clear ownership
- **History Tracking**: Undo/redo via snapshots of selection state
- **Configuration Persistence**: Type-safe serialization of user preferences

## Key Design Decisions

### Type-Driven Development
Every domain concept gets its own newtype wrapper:
- `CanonicalPath` prevents path injection attacks
- `TokenCount` makes token calculations explicit and type-safe
- `FileSize` enables size-based optimization strategies
- `ProgressCount` provides structured progress reporting

### Separation of Concerns
- **UI Layer**: Pure rendering, no business logic
- **State Layer**: Centralized state management
- **Worker Layer**: Isolated background processing
- **Core Layer**: Domain types and business rules

### Performance Strategy
- **Parallel File I/O**: Using `rayon` for CPU-bound operations
- **Memory-Mapped Files**: For large files (>256KB)
- **Incremental Updates**: Only reprocess changed files
- **UI Thread Protection**: Never block the UI with heavy operations

### Error Handling
- **Type-Safe Errors**: Custom error types for different failure modes
- **Graceful Degradation**: Partial failures don't stop the entire operation
- **User Feedback**: Toast notifications for all operations

## Module Organization

```
src/
├── app.rs              # Main application struct and coordination
├── core/
│   ├── mod.rs          # Core module exports
│   └── types.rs        # All domain types (FOUNDATION)
├── handlers.rs         # Event handlers and user actions
├── state/              # State management
│   ├── config.rs       # Configuration persistence
│   ├── history.rs      # Undo/redo system
│   └── mod.rs
├── ui/                 # User interface components
│   ├── app_ui.rs       # Main UI rendering
│   ├── theme.rs        # Visual theming system
│   ├── toast.rs        # Notification system
│   └── tree.rs         # Directory tree widget
├── utils/              # Utility functions
│   ├── parallel_fs.rs  # Parallel file operations
│   └── perf.rs         # Performance monitoring
├── watcher.rs          # Filesystem change detection
└── workers/            # Background processing
    ├── generator.rs    # Output generation worker
    └── mod.rs          # Worker communication types
```

## Communication Patterns

### UI ↔ State
- Direct mutable access to `AppState`
- Snapshot-based history for undo/redo
- Reactive updates via egui's retained mode

### Main Thread ↔ Workers
- **Commands**: Sent via `crossbeam::channel` to workers
- **Events**: Received from workers with progress and results
- **Cancellation**: Atomic flags for graceful termination

### State ↔ Persistence
- **Configuration**: JSON serialization via `serde`
- **Type Safety**: Custom serialization wrappers for complex types
- **Error Recovery**: Fallback to defaults on corruption

## Quality Assurance

### Type Safety
- All raw primitives wrapped in newtypes
- Builder patterns for complex configuration
- Validation at type construction boundaries

### Performance Monitoring
- Built-in performance overlay (debug builds)
- Benchmark suite for critical paths
- Memory usage tracking

### Testing Strategy
- Unit tests for all core types
- Integration tests for worker communication
- Benchmark tests for performance regression detection

This architecture ensures maintainability, performance, and correctness while providing a smooth user experience.