# fsPrompt Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Performance
- Incremental token counting for faster updates
- Chunked clipboard operations for very large outputs
- Lazy syntax highlighting to improve generation speed
- Pattern cache integration for faster ignore processing

### Features
- Drag-and-drop support for files and folders
- Multi-root workspace support
- Advanced content search capabilities

## [0.1.0] - 2025-01-07

Initial release of fsPrompt, a high-performance desktop application for generating LLM context prompts from local codebases.

### Features

#### Core Functionality
- **Native Folder Picker** - Select directories using system file dialog
- **Interactive Directory Tree** - Browse files with expand/collapse functionality
- **Tri-state Checkboxes** - Full parent/child selection propagation with indeterminate states
- **Split-pane Interface** - Resizable 30/70 layout for controls and output
- **Lazy Loading** - Directories load content only when expanded

#### Output Generation
- **XML/Markdown Generation** - Create LLM-ready context prompts in two formats
- **Token Estimation** - Accurate token count with Low/Medium/High visual indicators
- **Real-time Preview** - View generated output immediately
- **Directory Tree in Output** - Include full codebase structure for LLM context
- **Parallel Processing** - Handle large codebases with worker threads

#### File Management
- **Search & Filtering** - Find files quickly with fuzzy search
- **Ignore Patterns** - Skip node_modules, .git, and custom patterns
- **File Watching** - Auto-refresh prompts when files change

#### User Interface
- **Dark/Light Themes** - Auto-detect system theme with manual override
- **Clipboard Integration** - Copy output directly to clipboard (Ctrl+C)
- **Save to File** - Export generated output (Ctrl+S)
- **Toast Notifications** - Success/error feedback for user actions
- **Keyboard Shortcuts** - Full set of productivity shortcuts
- **Performance Overlay** - Real-time FPS and memory monitoring (Ctrl+Shift+P)

#### State Management
- **Undo/Redo** - Selection history with Ctrl+Z/Ctrl+Shift+Z (20 levels)
- **Configuration Persistence** - Auto-save settings and window layout
- **Responsive Design** - Tab-based UI for narrow windows

#### Performance Features
- **Virtualized Tree Rendering** - Smooth 60 FPS performance with 50k+ files
- **Memory-mapped I/O** - Efficient handling of large files (>256KB)
- **Worker Thread Architecture** - Non-blocking operations with progress tracking

### Technical Details

#### Architecture
- Built with Rust using egui/eframe for native cross-platform GUI
- Type-driven development with comprehensive newtype system
- Modular code structure with clean separation of concerns
- Parallel file processing using rayon for multi-core utilization

#### Dependencies
- `egui/eframe 0.31.1` - Immediate mode GUI framework
- `rayon 1.10.0` - Data parallelism
- `tokio 1.45.1` - Async runtime for I/O operations
- `arboard 3.5.0` - Cross-platform clipboard access
- `notify 8.0.0` - Filesystem watching
- `rfd 0.15.3` - Native file dialogs

#### Performance Benchmarks
- Handles repositories with 50,000+ files smoothly
- Maintains 60 FPS during tree navigation
- Sub-second response for file selection operations
- Memory usage scales efficiently with repository size

### Code Quality
- Strict linting with Clippy pedantic, nursery, and cargo lints
- Comprehensive test suite with unit and integration tests
- Performance benchmarks with criterion
- Type safety with domain-specific newtypes

### Platform Support
- **Windows** 10 and later
- **macOS** 12 and later (Intel and Apple Silicon)
- **Linux** with glibc 2.31+

### System Requirements
- **Memory**: 512MB minimum, 1GB recommended
- **Storage**: 50MB for application
- **CPU**: Any modern processor (optimized for multi-core)

### Installation
Currently available as source build only:
```bash
git clone https://github.com/patrikpersson/codext-rs.git
cd codext-rs
cargo build --release
```

### Known Limitations
- No drag-and-drop support yet
- Manual directory selection only
- Single-root workspace limitation
- English language only

### Development
This release establishes the foundation for a professional desktop application with:
- Clean, maintainable codebase
- Comprehensive type system preventing entire classes of bugs
- Performance-first architecture
- Modern design system with excellent UX

---

## Version History Summary

- **v0.1.0** (2025-01-07): Initial release with core functionality, professional UI, and excellent performance

---

*For detailed technical changes, see the git commit history. For planned features, see the [roadmap](roadmap.md).*