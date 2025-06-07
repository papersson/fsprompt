# fsPrompt Customization Guide

This guide covers theme customization, configuration options, and advanced customization techniques for fsPrompt.

## Configuration Overview

fsPrompt stores configuration in `~/.config/fsprompt/config.json` and automatically saves your preferences as you use the application.

### Configuration File Location

**Platform-specific paths**:
- **macOS**: `~/Library/Application Support/fsprompt/config.json`
- **Linux**: `~/.config/fsprompt/config.json`  
- **Windows**: `%APPDATA%\fsprompt\config.json`

### Configuration Structure

```json
{
  "version": "0.1.0",
  "window": {
    "width": 1200,
    "height": 800,
    "split_ratio": 0.3,
    "theme": "Auto"
  },
  "behavior": {
    "last_directory": "/Users/username/projects/my-app",
    "ignore_patterns": ["node_modules", "coverage", "*.log"],
    "default_format": "Markdown",
    "include_tree_by_default": true,
    "auto_refresh": true
  },
  "ui": {
    "show_performance_overlay": false,
    "toast_duration_ms": 3000,
    "tree_indent_size": 20,
    "font_size": 14
  }
}
```

## Theme Customization

### Built-in Themes

fsPrompt includes three theme options:

**Auto Theme (Default)**:
- Automatically matches system appearance
- Switches between light and dark based on OS settings
- Updates dynamically when system theme changes

**Light Theme**:
- Clean, minimal appearance with high contrast
- Optimized for bright environments
- Professional appearance for presentations

**Dark Theme**:
- Easy on the eyes for extended coding sessions
- Reduces eye strain in low-light environments
- Popular among developers

### Theme Selection

**Via UI**:
1. Click the theme toggle button in the top panel
2. Cycle through: Auto → Light → Dark → Auto

**Via Configuration**:
```json
{
  "window": {
    "theme": "Dark"  // Options: "Auto", "Light", "Dark"
  }
}
```

### Color Customization

While fsPrompt doesn't currently support custom color schemes, the themes are designed with specific use cases in mind:

**Light Theme Colors**:
```rust
// Reference colors (not user-configurable in v0.1.0)
Background: #FFFFFF
Text: #1F2937  
Accent: #3B82F6
Success: #10B981
Warning: #F59E0B
Error: #EF4444
```

**Dark Theme Colors**:
```rust
// Reference colors (not user-configurable in v0.1.0)
Background: #1F2937
Text: #F9FAFB
Accent: #60A5FA
Success: #34D399
Warning: #FBBF24
Error: #F87171
```

## UI Customization

### Window Layout

**Split Panel Ratio**:
The split between left (controls) and right (output) panels is customizable:

```json
{
  "window": {
    "split_ratio": 0.25  // 25% left, 75% right (minimum: 0.2, maximum: 0.5)
  }
}
```

**Window Size**:
```json
{
  "window": {
    "width": 1400,   // Minimum: 800
    "height": 900    // Minimum: 600
  }
}
```

### Tree Appearance

**Indentation**:
```json
{
  "ui": {
    "tree_indent_size": 24  // Pixels per nesting level (default: 20)
  }
}
```

**Font Size**:
```json
{
  "ui": {
    "font_size": 16  // UI font size (default: 14, range: 10-24)
  }
}
```

### Performance Overlay

**Enable by Default**:
```json
{
  "ui": {
    "show_performance_overlay": true  // Show FPS/memory on startup
  }
}
```

## Behavior Customization

### Default Settings

**Output Format**:
```json
{
  "behavior": {
    "default_format": "XML"  // Options: "XML", "Markdown"
  }
}
```

**Include Directory Tree**:
```json
{
  "behavior": {
    "include_tree_by_default": false  // Don't include tree in output by default
  }
}
```

**Auto-Refresh**:
```json
{
  "behavior": {
    "auto_refresh": false  // Disable filesystem watching
  }
}
```

### Ignore Patterns

**Global Ignore Patterns**:
```json
{
  "behavior": {
    "ignore_patterns": [
      "node_modules",
      "target", 
      "coverage",
      "*.log",
      "*.tmp",
      ".git",
      "dist",
      "build"
    ]
  }
}
```

**Project-Type Specific Configurations**:

Create separate configuration profiles for different project types:

**Frontend Project Config**:
```json
{
  "behavior": {
    "ignore_patterns": [
      "node_modules",
      "dist", 
      "build",
      ".next",
      ".nuxt",
      "coverage",
      "*.log",
      ".cache"
    ],
    "default_format": "Markdown",
    "include_tree_by_default": true
  }
}
```

**Rust Project Config**:
```json
{
  "behavior": {
    "ignore_patterns": [
      "target",
      "Cargo.lock",
      "*.pdb",
      "*.exe", 
      "*.so",
      "*.dylib",
      "coverage"
    ],
    "default_format": "XML",
    "include_tree_by_default": true
  }
}
```

**Python Project Config**:
```json
{
  "behavior": {
    "ignore_patterns": [
      "__pycache__",
      "*.pyc",
      "*.pyo",
      "venv",
      "env",
      ".pytest_cache",
      "dist",
      "build",
      "*.egg-info"
    ],
    "default_format": "Markdown",
    "include_tree_by_default": true
  }
}
```

### Toast Notifications

**Duration Customization**:
```json
{
  "ui": {
    "toast_duration_ms": 5000,  // Show toasts for 5 seconds (default: 3000)
    "toast_position": "TopRight"  // Future: Position options
  }
}
```

## Advanced Customization

### Configuration Management

**Backup Configuration**:
```bash
# Create backup of current settings
cp ~/.config/fsprompt/config.json ~/.config/fsprompt/config.backup.json

# Or on Windows:
copy "%APPDATA%\fsprompt\config.json" "%APPDATA%\fsprompt\config.backup.json"
```

**Reset to Defaults**:
```bash
# Remove config file to reset to defaults
rm ~/.config/fsprompt/config.json

# fsPrompt will recreate with defaults on next launch
```

**Share Team Configuration**:
```bash
# Export team-standard configuration
cp ~/.config/fsprompt/config.json ~/projects/team-config/fsprompt-config.json

# Team members can import:
cp ~/projects/team-config/fsprompt-config.json ~/.config/fsprompt/config.json
```

### Environment Variables

**Override Configuration Directory**:
```bash
# Use custom config location
export FSPROMPT_CONFIG_DIR="/custom/path/config"
cargo run

# Useful for testing different configurations
```

**Development Mode**:
```bash
# Enable development features (future)
export FSPROMPT_DEV_MODE=1
export RUST_LOG=debug
cargo run
```

### Keyboard Shortcuts

Current shortcuts are fixed, but future versions will support customization:

**Current Shortcuts**:
```json
{
  "shortcuts": {
    "generate": "Ctrl+G",
    "copy": "Ctrl+C", 
    "save": "Ctrl+S",
    "search_tree": "Ctrl+K",
    "search_output": "Ctrl+F",
    "undo": "Ctrl+Z",
    "redo": "Ctrl+Shift+Z",
    "performance_overlay": "Ctrl+Shift+P",
    "cancel": "Escape"
  }
}
```

**Future Customization** (planned):
```json
{
  "shortcuts": {
    "generate": "F5",           // Custom key binding
    "copy": "Ctrl+C",           // Keep default
    "vim_mode": true,           // Enable Vim-style navigation
    "custom_commands": {
      "Ctrl+1": "select_all_source_files",
      "Ctrl+2": "select_all_test_files"
    }
  }
}
```

## Project-Specific Customization

### Project Configuration Files

**Future Feature**: Project-specific settings in `.fsprompt.json`:

```json
{
  "name": "My Web App",
  "type": "frontend",
  "ignore_patterns": [
    "node_modules",
    "dist",
    "coverage",
    "e2e/screenshots"
  ],
  "presets": {
    "review": {
      "include_patterns": ["src/**/*.{js,ts,tsx}"],
      "exclude_tests": true,
      "format": "Markdown"
    },
    "testing": {
      "include_patterns": ["**/*.test.{js,ts}", "**/*.spec.{js,ts}"],
      "include_related": true,
      "format": "XML"
    },
    "architecture": {
      "include_patterns": ["src/**/*.{js,ts}", "config/**/*", "*.json"],
      "include_tree": true,
      "format": "Markdown"
    }
  }
}
```

### Team Standards

**Shared Ignore Patterns**:
```bash
# Create team standards file
echo '["node_modules", "coverage", "dist", "*.log"]' > .fsprompt-ignore

# Team members can import:
# (Future feature - CLI support)
fsprompt --import-ignore .fsprompt-ignore
```

**Configuration Templates**:
```bash
# Organization-wide templates
/company-configs/
├── frontend-config.json      # React/Vue/Angular projects
├── backend-config.json       # Node.js/Python/Go APIs  
├── mobile-config.json        # React Native/Flutter
└── data-science-config.json  # Jupyter/Python ML projects
```

## Performance Customization

### Memory Management

**Large Repository Settings**:
```json
{
  "performance": {
    "max_file_size_mb": 1,           // Skip files larger than 1MB
    "max_files_per_selection": 1000, // Limit selection size
    "enable_memory_mapping": true,    // Use mmap for large files
    "worker_threads": 8              // Override CPU core detection
  }
}
```

**UI Performance**:
```json
{
  "ui": {
    "tree_virtualization": true,     // Enable for very large trees
    "lazy_syntax_highlighting": true, // Defer highlighting
    "reduce_animations": false        // Disable for accessibility/performance
  }
}
```

### File Processing

**Concurrency Settings**:
```json
{
  "performance": {
    "parallel_file_reading": true,
    "max_concurrent_reads": 16,
    "chunk_size_bytes": 65536
  }
}
```

## Export and Import

### Configuration Export

**Full Configuration Export**:
```bash
# Export all settings (future CLI feature)
fsprompt --export-config ~/my-fsprompt-settings.json

# Include window state, ignore patterns, theme, etc.
```

**Selective Export**:
```bash
# Export only ignore patterns
fsprompt --export-ignore ~/ignore-patterns.json

# Export only theme settings  
fsprompt --export-theme ~/theme-config.json
```

### Configuration Import

**Import Full Configuration**:
```bash
# Import complete settings
fsprompt --import-config ~/downloaded-config.json

# Merge with existing (don't overwrite everything)
fsprompt --import-config ~/partial-config.json --merge
```

**Import Specific Settings**:
```bash
# Import only ignore patterns
fsprompt --import-ignore ~/team-ignore-patterns.json

# Import theme settings
fsprompt --import-theme ~/company-theme.json
```

## Development and Debugging

### Debug Configuration

**Verbose Logging**:
```json
{
  "debug": {
    "enable_logging": true,
    "log_level": "debug",           // trace, debug, info, warn, error
    "log_file": "~/.config/fsprompt/debug.log",
    "performance_profiling": true
  }
}
```

**Development Features**:
```json
{
  "development": {
    "show_debug_info": true,        // Show internal state info
    "enable_experimental": true,    // Enable experimental features
    "hot_reload_config": true,      // Reload config without restart
    "mock_large_files": false       // Simulate large files for testing
  }
}
```

### Configuration Validation

**Validate Configuration**:
```bash
# Check configuration file validity (future)
fsprompt --validate-config

# Output:
# ✓ Configuration is valid
# ✗ Invalid theme name: "InvalidTheme"
# ✗ Split ratio must be between 0.2 and 0.5
```

**Auto-Repair**:
```bash
# Fix common configuration issues
fsprompt --repair-config

# Creates backup and fixes:
# - Invalid values reset to defaults
# - Missing required fields added
# - Deprecated settings updated
```

## Customization Examples

### Example 1: Minimal UI Setup

For users who prefer a clean, minimal interface:

```json
{
  "window": {
    "theme": "Light",
    "split_ratio": 0.25
  },
  "ui": {
    "show_performance_overlay": false,
    "toast_duration_ms": 2000,
    "font_size": 13
  },
  "behavior": {
    "include_tree_by_default": false,
    "default_format": "XML"
  }
}
```

### Example 2: Power User Setup

For developers working with large codebases:

```json
{
  "window": {
    "theme": "Dark",
    "split_ratio": 0.3,
    "width": 1600,
    "height": 1000
  },
  "ui": {
    "show_performance_overlay": true,
    "tree_indent_size": 24,
    "font_size": 15
  },
  "behavior": {
    "ignore_patterns": [
      "node_modules", "target", "coverage", "*.log", 
      "dist", "build", ".git", "__pycache__", "vendor"
    ],
    "include_tree_by_default": true,
    "default_format": "Markdown"
  }
}
```

### Example 3: Team Collaboration Setup

For teams sharing configurations:

```json
{
  "window": {
    "theme": "Auto"
  },
  "behavior": {
    "ignore_patterns": [
      "node_modules", "coverage", "dist", "*.log",
      "e2e/screenshots", "cypress/videos"
    ],
    "default_format": "Markdown",
    "include_tree_by_default": true
  },
  "team": {
    "organization": "ACME Corp",
    "standards_version": "1.2.0",
    "required_patterns": ["node_modules", "coverage"]
  }
}
```

## Future Customization Features

### Planned Enhancements

**Custom Themes**:
- Full color scheme customization
- Import/export custom themes
- Theme marketplace/sharing

**Advanced Ignore Patterns**:
- Regex support beyond glob patterns
- Conditional patterns based on project type
- Smart pattern suggestions

**UI Customization**:
- Custom keyboard shortcuts
- Configurable toolbar
- Plugin system for custom widgets

**Workflow Presets**:
- Saved selection presets
- Quick-access preset buttons
- Project-specific automation

---

*This completes the comprehensive documentation for fsPrompt. For additional help, see the [Basic Usage Guide](basic-usage.md) or [Advanced Patterns](advanced-patterns.md).*