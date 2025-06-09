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

### Recommended: Install via Cargo

The simplest way to install fsPrompt is using Cargo (Rust's package manager):

```bash
# Install directly from GitHub
cargo install --git https://github.com/patrikpersson/codext-rs.git

# The binary will be installed to ~/.cargo/bin/fsprompt
fsprompt
```

### Pre-built Binaries

Download from the [latest release](https://github.com/patrikpersson/codext-rs/releases/latest).

> **⚠️ Security Notice**: Pre-built binaries are not code-signed. Your OS will show security warnings. This is normal for open-source projects. See [why do I get security warnings?](#security-warnings) below.

#### macOS

**Package Installer (.pkg)**
1. Download `fsprompt-{version}-{arch}-apple-darwin.pkg`
   - Use `x86_64` for Intel Macs
   - Use `aarch64` for Apple Silicon (M1/M2/M3)
2. **To bypass Gatekeeper warning**:
   - Go to System Settings → Privacy & Security
   - Look for "fsprompt.pkg was blocked"
   - Click "Open Anyway"

**Manual Installation**
```bash
# Download and extract
tar xzf fsprompt-{version}-{arch}-apple-darwin.tar.gz

# Install to system
sudo cp fsprompt /usr/local/bin/
sudo chmod +x /usr/local/bin/fsprompt
```

#### Windows

**Installer (.exe)**
1. Download `fsprompt-{version}-x86_64-pc-windows-msvc-setup.exe`
2. **To bypass SmartScreen**:
   - Click "More info"
   - Click "Run anyway"
3. Follow the installer (adds to PATH automatically)

**Manual Installation**
```powershell
# Extract the zip file
Expand-Archive fsprompt-{version}-x86_64-pc-windows-msvc.zip

# Create directory and copy
mkdir "C:\Program Files\fsprompt"
copy fsprompt\fsprompt.exe "C:\Program Files\fsprompt\"

# Add to PATH (run as Administrator)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\fsprompt", [EnvironmentVariableTarget]::Machine)
```

#### Linux

```bash
# Download and extract
tar xzf fsprompt-{version}-x86_64-unknown-linux-gnu.tar.gz

# Install to system
sudo cp fsprompt /usr/local/bin/
sudo chmod +x /usr/local/bin/fsprompt

# Or install to user directory
mkdir -p ~/.local/bin
cp fsprompt ~/.local/bin/
chmod +x ~/.local/bin/fsprompt
# Add ~/.local/bin to PATH if not already there
```

### Build from Source

```bash
git clone https://github.com/patrikpersson/codext-rs.git
cd codext-rs
cargo build --release

# Install to cargo bin directory
cargo install --path .

# Or copy manually
sudo cp target/release/fsprompt /usr/local/bin/  # Unix-like
# or
copy target\release\fsprompt.exe "C:\Program Files\fsprompt\"  # Windows
```

### Security Warnings

**Why do I get security warnings?**

fsPrompt's binaries are not code-signed because:
- **Apple Developer ID**: $99/year
- **Windows Code Signing**: $200-500/year
- **Open source projects** typically can't afford these fees

The warnings you see (Gatekeeper on macOS, SmartScreen on Windows) are normal for unsigned open-source software. The binaries are built automatically by GitHub Actions from the public source code.

**To avoid warnings entirely**, install via:
1. `cargo install` (recommended)
2. Build from source
3. Use a package manager that handles signing (when available)

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