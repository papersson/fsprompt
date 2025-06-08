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

### Download Pre-built Binaries

Download the latest release from the [GitHub Releases](https://github.com/patrikpersson/codext-rs/releases) page.

#### macOS

**Option 1: Package Installer (.pkg)**
1. Download `fsprompt-v0.1.0-{arch}-apple-darwin.pkg` (where `{arch}` is `x86_64` for Intel or `aarch64` for Apple Silicon)
2. Double-click the `.pkg` file
3. You'll see a warning that the app is from an unidentified developer
4. Right-click the `.pkg` file and select "Open" to bypass Gatekeeper
5. Follow the installation wizard
6. fsPrompt will be installed to `/usr/local/bin` and available in your terminal

**Option 2: Manual Installation**
1. Download `fsprompt-v0.1.0-{arch}-apple-darwin.tar.gz`
2. Extract: `tar xzf fsprompt-v0.1.0-{arch}-apple-darwin.tar.gz`
3. Move to PATH: `sudo mv fsprompt /usr/local/bin/`
4. Make executable: `chmod +x /usr/local/bin/fsprompt`

#### Windows

**Option 1: Installer (.exe)**
1. Download `fsprompt-v0.1.0-x86_64-pc-windows-msvc-setup.exe`
2. Double-click the installer
3. Windows SmartScreen may warn about an unrecognized app
4. Click "More info" → "Run anyway"
5. Follow the installation wizard
6. The installer will add fsPrompt to your system PATH automatically

**Option 2: Manual Installation**
1. Download `fsprompt-v0.1.0-x86_64-pc-windows-msvc.zip`
2. Extract the ZIP file
3. Move `fsprompt.exe` to a directory in your PATH (e.g., `C:\Program Files\fsPrompt\`)
4. Or add the directory containing `fsprompt.exe` to your PATH

### Build from Source

If you prefer to build from source:

```bash
git clone https://github.com/patrikpersson/codext-rs.git
cd codext-rs
cargo build --release
./target/release/fsprompt  # or .\target\release\fsprompt.exe on Windows
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