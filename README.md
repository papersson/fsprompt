# fsPrompt

A high-performance desktop application that generates compact "context prompts" from local codebases for use with Large Language Models (LLMs). Convert filesystem structures and file contents into XML or Markdown format.

## Features

### ✅ Currently Implemented
- 🗂️ **Native folder picker** - Select directories using your system's file dialog
- 🌳 **Interactive directory tree** - Browse files with expand/collapse functionality  
- ☑️ **Tri-state checkboxes** - Full parent/child selection propagation with indeterminate states
- 📐 **Split-pane interface** - Resizable 30/70 layout for controls and output
- 🚀 **Lazy loading** - Directories load content only when expanded
- 📄 **XML/Markdown generation** - Create LLM-ready context prompts in two formats
- 📊 **Token estimation** - See token count with Low/Medium/High visual indicators
- 🎯 **Real-time preview** - View generated output immediately
- ⚡ **Parallel processing** - Handle large codebases with worker threads
- 🔍 **Search & filtering** - Find files quickly with fuzzy search
- 🚫 **Ignore patterns** - Skip node_modules, .git, and custom patterns
- 📋 **Clipboard integration** - Copy output directly to clipboard (Ctrl+C)
- 💾 **Save to file** - Export generated output (Ctrl+S)
- 🌲 **Directory tree in output** - Include full codebase structure for LLM context
- ↩️ **Undo/Redo** - Selection history with Ctrl+Z/Ctrl+Shift+Z
- 🎨 **Dark/Light themes** - Auto-detect system theme with manual override
- 🔔 **Toast notifications** - Success/error feedback for user actions
- 📁 **File watching** - Auto-refresh prompts when files change
- 📈 **Performance overlay** - Real-time FPS and memory monitoring (Ctrl+Shift+P)

### 🚧 Planned Features
- 🎯 **Drag-and-drop** - Drag files/folders to include
- ♿ **Accessibility** - Full keyboard navigation and screen reader support
- 🌍 **Internationalization** - Support for multiple languages
- 📦 **Installers** - Native installers for Windows, macOS, and Linux

## Quick Start

```bash
# Build and run
cargo build
cargo run

# Run with optimizations
cargo run --release

# Run verification suite
cargo fmt && cargo clippy && cargo test && cargo check --all-targets
```

## Usage

1. **Select Directory** - Click "Select Directory" to choose a folder
2. **Browse Files** - Expand folders and check files you want to include
3. **Choose Format** - Select XML or Markdown output format
4. **Generate** - Click "🚀 Generate" to create the output
5. **View Results** - See the output and token count in the right panel

### Output Formats

**XML Format:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<codebase>
  <file path="/src/main.rs">
    <content><![CDATA[
// File contents here
    ]]></content>
  </file>
</codebase>
```

**Markdown Format:**
```markdown
# Codebase Export

Generated 2 files

## File: /src/main.rs

```rust
// File contents here
```
```

## Development

This project emphasizes code quality through:
- **Strict linting** - Clippy pedantic, nursery, and cargo lints enabled
- **Type safety** - Comprehensive type system with newtypes
- **Performance** - Designed for 10,000+ file repositories
- **Testing** - Unit, integration, and performance tests

### Project Structure

```
src/
├── main.rs          # Application entry point and UI layout
├── ui/
│   ├── mod.rs       # UI module declarations
│   └── tree.rs      # Directory tree widget with selection
├── core/
│   ├── mod.rs       # Core module declarations  
│   └── types.rs     # Domain types and data structures
├── workers/         # (Coming soon) Worker thread implementations
└── utils/           # (Coming soon) Utility functions
```

### Key Technologies

- **GUI**: egui/eframe (immediate mode, native performance)
- **File Dialogs**: rfd (native file/folder selection)
- **Planned**: rayon (parallelism), crossbeam (channels), tokio (async)

## System Requirements

- **Operating Systems**: Windows 10+, macOS 12+, Linux (glibc 2.31+)
- **Memory**: 512MB minimum, 1GB recommended
- **Rust**: 1.86+ (for development)

## Building from Source

```bash
# Clone the repository
git clone https://github.com/patrikpersson/codext-rs.git
cd fsprompt

# Build release version
cargo build --release

# Binary will be in target/release/fsprompt
```

## Contributing

See `.claude/development.md` for development practices and guidelines.

## License

MIT