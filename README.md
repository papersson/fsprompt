# fsPrompt

A high-performance desktop application that generates "context prompts" from local codebases for use with Large Language Models (LLMs). Convert filesystem structures and file contents into XML or Markdown format optimized for AI consumption.

## Features

- 🗂️ **Native folder picker** - Select directories using your system's file dialog
- 🌳 **Interactive directory tree** - Browse and select files with expand/collapse functionality  
- ☑️ **Smart selection** - Tri-state checkboxes with parent/child propagation
- 📄 **Dual output formats** - Generate XML or Markdown optimized for LLMs
- 📊 **Token estimation** - Real-time token count with visual indicators
- ⚡ **High performance** - Parallel processing handles large codebases efficiently
- 🔍 **Search & filtering** - Fuzzy search and customizable ignore patterns
- 📋 **Export options** - Copy to clipboard or save to file
- 🌲 **Directory tree inclusion** - Optional codebase structure for LLM context
- ↩️ **Undo/Redo** - Full selection history
- 🎨 **Theme support** - Dark/light themes with system detection
- 📁 **Auto-refresh** - File watching with automatic prompt updates

## Installation

**Pre-built binaries coming soon!**

Currently, build from source:

```bash
git clone https://github.com/patrikpersson/codext-rs.git
cd codext-rs
cargo build --release
./target/release/fsprompt
```

## How to Use

1. **Select Directory** - Click "Select Directory" to choose your codebase
2. **Browse & Select** - Expand folders and check files to include
3. **Choose Format** - Pick XML or Markdown output format
4. **Generate** - Click "🚀 Generate" to create your prompt
5. **Export** - Copy to clipboard or save to file

### Keyboard Shortcuts

- `Ctrl+F` - Search files
- `Ctrl+G` - Generate output  
- `Ctrl+C` - Copy to clipboard
- `Ctrl+S` - Save to file
- `Ctrl+Z` / `Ctrl+Shift+Z` - Undo/Redo

### Output Formats

**XML** - Structured format, ideal for Claude and GPT
```xml
<codebase>
  <file path="src/main.rs">
    <content><![CDATA[fn main() { ... }]]></content>
  </file>
</codebase>
```

**Markdown** - Human-readable, works with all LLMs
```markdown
## File: src/main.rs
```rust
fn main() { ... }
```

## Documentation

Comprehensive documentation is available in the [`/docs`](docs/) folder:

- **[User Guide](docs/user-guide/)** - Installation, features, and usage
- **[Architecture](docs/architecture/)** - System design and technical details  
- **[API Reference](docs/api/)** - Complete API documentation
- **[Examples](docs/examples/)** - Real-world usage patterns

## System Requirements

- **Windows** 10+, **macOS** 12+, or **Linux** (glibc 2.31+)
- **Memory**: 512MB minimum, 1GB+ for large codebases
- **Rust** 1.86+ (for building from source)

## License

MIT