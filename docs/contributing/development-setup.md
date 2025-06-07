# Development Environment Setup

This guide walks you through setting up a complete development environment for fsPrompt.

## Table of Contents

- [System Requirements](#system-requirements)
- [Rust Installation](#rust-installation)
- [Project Setup](#project-setup)
- [Development Tools](#development-tools)
- [IDE Configuration](#ide-configuration)
- [Platform-Specific Setup](#platform-specific-setup)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## System Requirements

### Minimum Requirements
- **Operating System**: Windows 10+, macOS 12+, or Linux with glibc 2.31+
- **Memory**: 4GB RAM (8GB recommended for large codebases)
- **Storage**: 2GB free space for Rust toolchain and dependencies
- **Network**: Internet connection for downloading dependencies

### Recommended Development Environment
- **Memory**: 8GB+ RAM for smooth development experience
- **Storage**: SSD for faster compilation times
- **CPU**: Multi-core processor for parallel compilation and testing

## Rust Installation

### Install Rust via rustup

```bash
# Install rustup (Rust toolchain installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows, download and run: https://rustup.rs/
```

### Configure Rust Toolchain

```bash
# Install the latest stable Rust (required: 1.86+)
rustup update stable
rustup default stable

# Add useful components
rustup component add clippy rustfmt rust-src

# Install cargo extensions
cargo install cargo-watch cargo-audit cargo-outdated
```

### Verify Installation

```bash
# Check Rust version (should be 1.86+)
rustc --version

# Check cargo version
cargo --version

# Check clippy
cargo clippy --version

# Check rustfmt
cargo fmt --version
```

## Project Setup

### Clone the Repository

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/codext-rs.git
cd codext-rs

# Add upstream remote
git remote add upstream https://github.com/patrikpersson/codext-rs.git

# Verify remotes
git remote -v
```

### Initial Build

```bash
# Build the project (this will download and compile dependencies)
cargo build

# This may take 5-10 minutes on first run due to dependency compilation
# Subsequent builds will be much faster
```

### Verify Development Environment

```bash
# Run the full verification suite
cargo fmt && cargo clippy && cargo test && cargo check --all-targets

# If all commands succeed, your environment is ready!
```

## Development Tools

### Essential Tools

1. **cargo-watch** - Automatic rebuilding on file changes
   ```bash
   cargo install cargo-watch
   
   # Usage examples:
   cargo watch -x "check"                    # Check on changes
   cargo watch -x "test"                     # Run tests on changes
   cargo watch -x "clippy -- -D warnings"   # Lint on changes
   ```

2. **cargo-audit** - Security vulnerability scanning
   ```bash
   cargo install cargo-audit
   cargo audit  # Check for known vulnerabilities
   ```

3. **cargo-outdated** - Check for outdated dependencies
   ```bash
   cargo install cargo-outdated
   cargo outdated  # List outdated dependencies
   ```

### Performance Tools

1. **Criterion** - Benchmarking (already included in dev-dependencies)
   ```bash
   # Run benchmarks
   cargo bench
   
   # Run specific benchmark
   cargo bench performance
   
   # Generate HTML reports
   cargo bench -- --output-format html
   ```

2. **Heaptrack** (Linux) - Memory profiling
   ```bash
   # Install on Ubuntu/Debian
   sudo apt-get install heaptrack
   
   # Profile your application
   heaptrack cargo run --release
   ```

3. **Instruments** (macOS) - System-level profiling
   - Use Xcode's Instruments app for detailed performance analysis
   - Particularly useful for memory and CPU profiling

### Code Quality Tools

1. **cargo-expand** - Macro expansion debugging
   ```bash
   cargo install cargo-expand
   cargo expand  # Expand macros in your code
   ```

2. **cargo-tree** - Dependency tree visualization
   ```bash
   cargo tree  # Show dependency tree
   cargo tree --duplicates  # Find duplicate dependencies
   ```

## IDE Configuration

### Visual Studio Code (Recommended)

1. **Install extensions**:
   - `rust-analyzer` - Rust language server
   - `CodeLLDB` - Debugging support
   - `crates` - Cargo.toml dependency management
   - `Better TOML` - TOML syntax highlighting

2. **Settings** (`.vscode/settings.json`):
   ```json
   {
     "rust-analyzer.checkOnSave.command": "clippy",
     "rust-analyzer.checkOnSave.allTargets": false,
     "rust-analyzer.cargo.features": "all",
     "rust-analyzer.procMacro.enable": true,
     "editor.formatOnSave": true,
     "files.watcherExclude": {
       "**/target/**": true
     }
   }
   ```

3. **Tasks** (`.vscode/tasks.json`):
   ```json
   {
     "version": "2.0.0",
     "tasks": [
       {
         "label": "Rust: Verify All",
         "type": "shell",
         "command": "cargo",
         "args": ["fmt", "&&", "cargo", "clippy", "&&", "cargo", "test", "&&", "cargo", "check", "--all-targets"],
         "group": {
           "kind": "build",
           "isDefault": true
         },
         "presentation": {
           "echo": true,
           "reveal": "always",
           "focus": false,
           "panel": "shared"
         }
       }
     ]
   }
   ```

### IntelliJ IDEA / CLion

1. **Install Rust plugin**
2. **Configure Clippy** in settings
3. **Enable format on save**
4. **Set up run configurations** for tests and benchmarks

### Vim/Neovim

1. **Use rust-analyzer** with your preferred LSP client
2. **Configure formatting** with `rustfmt`
3. **Set up linting** with `clippy`

Example configuration for Neovim with `nvim-lspconfig`:
```lua
require'lspconfig'.rust_analyzer.setup{
  settings = {
    ["rust-analyzer"] = {
      checkOnSave = {
        command = "clippy"
      }
    }
  }
}
```

## Platform-Specific Setup

### Windows

1. **Install Visual Studio Build Tools**:
   - Download from Microsoft's website
   - Select "C++ build tools" workload
   - Required for compiling native dependencies

2. **PowerShell Configuration**:
   ```powershell
   # Enable execution of scripts
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```

3. **Windows-specific testing**:
   ```bash
   # Test clipboard functionality
   cargo test clipboard_large_content
   
   # Test file dialog integration
   cargo test file_dialog_integration
   ```

### macOS

1. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```

2. **macOS-specific considerations**:
   - File dialogs must run on the main thread (handled by `rfd`)
   - Test with paths containing unicode characters
   - Verify clipboard operations with large content

3. **Testing**:
   ```bash
   # Test macOS-specific functionality
   cargo test macos_file_dialogs
   cargo test unicode_paths
   ```

### Linux

1. **Install system dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install build-essential pkg-config libssl-dev
   
   # For GUI development (optional, for testing)
   sudo apt-get install libgtk-3-dev libxcb-xfixes0-dev
   
   # Fedora/RHEL
   sudo dnf install gcc pkg-config openssl-devel
   sudo dnf install gtk3-devel libxcb-devel
   
   # Arch Linux
   sudo pacman -S base-devel pkg-config openssl
   sudo pacman -S gtk3 libxcb
   ```

2. **Linux-specific testing**:
   ```bash
   # Test X11/Wayland clipboard integration
   cargo test clipboard_integration
   
   # Test file permissions and symlinks
   cargo test symlink_handling
   ```

## Verification

### Quick Verification
```bash
# Basic functionality check
cargo check

# Code formatting
cargo fmt --check

# Linting
cargo clippy -- -D warnings

# Tests
cargo test
```

### Full Verification Suite
```bash
# Complete verification (run before committing)
cargo fmt && cargo clippy && cargo test && cargo check --all-targets

# With benchmarks (optional, takes longer)
cargo bench
```

### Performance Verification
```bash
# Ensure performance requirements are met
cargo bench performance

# Check token counting performance
cargo bench token_thresholds

# UI responsiveness tests
cargo bench ui_performance
```

## Troubleshooting

### Common Issues

1. **Compilation errors with native dependencies**:
   ```bash
   # Clear cargo cache and rebuild
   cargo clean
   cargo build
   
   # Update Rust toolchain
   rustup update
   ```

2. **Clippy warnings as errors**:
   ```bash
   # Fix all clippy warnings before committing
   cargo clippy --fix
   
   # Some warnings may need manual fixes
   cargo clippy -- -D warnings
   ```

3. **Test failures**:
   ```bash
   # Run tests with output
   cargo test -- --nocapture
   
   # Run specific test
   cargo test test_name -- --exact
   
   # Run tests single-threaded
   cargo test -- --test-threads=1
   ```

4. **Performance issues during development**:
   ```bash
   # Use release mode for testing large datasets
   cargo build --release
   cargo test --release
   
   # Check if debug symbols are causing slowness
   CARGO_PROFILE_DEV_DEBUG=0 cargo build
   ```

### Environment Issues

1. **PATH problems**:
   ```bash
   # Ensure cargo is in PATH
   echo $PATH | grep cargo
   
   # Source cargo environment
   source ~/.cargo/env
   ```

2. **Permission issues (Linux/macOS)**:
   ```bash
   # Fix cargo directory permissions
   sudo chown -R $USER:$USER ~/.cargo
   ```

3. **Network/proxy issues**:
   ```bash
   # Configure cargo for proxy
   # Edit ~/.cargo/config.toml
   [http]
   proxy = "http://proxy:port"
   
   [https]
   proxy = "https://proxy:port"
   ```

### Platform-Specific Issues

#### Windows
- **MSVC vs GNU toolchain**: Use MSVC for better compatibility
- **Long path support**: Enable in Windows settings if needed
- **Antivirus**: Add target directory to exclusions for faster builds

#### macOS
- **Xcode updates**: May require reinstalling command line tools
- **Permission dialogs**: Grant necessary permissions for file access
- **Rosetta 2**: Intel Macs may need Rosetta for some dependencies

#### Linux
- **Missing libraries**: Install system development packages
- **Display server**: Test both X11 and Wayland if available
- **AppImage/Flatpak**: Consider packaging considerations

## Next Steps

Once your environment is set up:

1. **Read the type system** - Study `src/core/types.rs` thoroughly
2. **Review testing guide** - See [testing-guide.md](./testing-guide.md)
3. **Pick an issue** - Look for "good first issue" labels
4. **Join discussions** - Participate in GitHub Discussions
5. **Start coding** - Follow the type-driven development workflow

## Getting Help

- **Environment issues**: Open a GitHub issue with the "setup" label
- **Build problems**: Include full error output and system information
- **IDE configuration**: Check existing issues or start a discussion
- **Performance problems**: Use GitHub Discussions for optimization questions

Remember: A properly configured development environment is crucial for a smooth contribution experience. Take time to set it up correctly!