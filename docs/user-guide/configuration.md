# Configuration Guide

fsPrompt offers extensive configuration options to customize the application behavior, appearance, and performance. This guide covers all available settings and how to modify them.

## Configuration Overview

fsPrompt stores configuration in platform-specific locations using industry-standard paths:

- **Windows**: `%APPDATA%\fsprompt\config.json`
- **macOS**: `~/Library/Application Support/fsprompt/config.json`
- **Linux**: `~/.config/fsprompt/config.json`

All settings are automatically saved when changed through the UI and persist between application sessions.

## Window Configuration

Control the application window behavior and layout.

### Window Dimensions

| Setting | Default | Description |
|---------|---------|-------------|
| `window_width` | 1200.0 | Window width in pixels |
| `window_height` | 800.0 | Window height in pixels |
| `split_position` | 0.3 | Left panel ratio (0.0-1.0) |

**Examples:**
```json
{
  "window_width": 1920.0,
  "window_height": 1080.0,
  "split_position": 0.25
}
```

**Configuration Tips:**
- **Split Position**: 0.3 = 30% left panel, 70% right panel
- **Minimum Width**: 800 pixels for usable interface
- **Recommended**: 1200x800 or larger for optimal experience

### Window Behavior

The application remembers:
- Last window size and position
- Panel split ratio
- Maximized/restored state
- Multi-monitor positioning

## User Interface Settings

Customize the visual appearance and behavior of the interface.

### Theme Configuration

| Setting | Values | Description |
|---------|--------|-------------|
| `theme` | `"auto"`, `"light"`, `"dark"` | UI theme preference |

**Theme Options:**
- **`"auto"`** (default): Follows system light/dark mode
- **`"light"`**: Forces light theme regardless of system setting
- **`"dark"`**: Forces dark theme regardless of system setting

**Theme Features:**
- Instant switching without restart
- Affects all UI elements consistently
- Respects system accessibility settings
- High contrast support where available

### Font and Display

| Setting | Default | Range | Description |
|---------|---------|-------|-------------|
| `font_size` | 14.0 | 8.0-24.0 | Base font size in points |
| `show_hidden` | false | boolean | Show hidden files by default |
| `include_tree` | true | boolean | Include directory tree in output |

**Font Considerations:**
- **Small (8-12)**: More content visible, harder to read
- **Medium (14-16)**: Balanced readability and content density
- **Large (18-24)**: Better accessibility, less content visible

### UI Behavior Settings

```json
{
  "ui": {
    "theme": "auto",
    "font_size": 14.0,
    "show_hidden": false,
    "include_tree": true
  }
}
```

## File Processing Configuration

Configure how fsPrompt handles files and directories.

### Ignore Patterns

Control which files and directories are excluded from the tree view and processing.

**Default Patterns:**
```json
{
  "ignore_patterns": [
    ".*",
    "node_modules",
    "__pycache__",
    "target",
    "build",
    "dist",
    "_*"
  ]
}
```

**Pattern Types:**

1. **Exact Match**: Literal string matching
   - `"node_modules"` - Matches directories named exactly "node_modules"
   - `".git"` - Matches .git directories

2. **Glob Patterns**: Wildcard matching
   - `"*.tmp"` - All files ending with .tmp
   - `"test_*"` - All files starting with test_
   - `"**/*.log"` - All .log files in any subdirectory

3. **Complex Patterns**: Advanced matching
   - `"build/**/cache"` - Cache directories within build directories
   - `"{tmp,temp}/*"` - Files in tmp or temp directories

**Common Ignore Patterns by Language:**

**JavaScript/Node.js:**
```json
["node_modules", "npm-debug.log", "*.log", ".npm", "dist", "build"]
```

**Python:**
```json
["__pycache__", "*.pyc", "*.pyo", ".venv", "venv", ".pytest_cache", "*.egg-info"]
```

**Rust:**
```json
["target", "Cargo.lock", "*.rlib", ".cargo"]
```

**Java:**
```json
["*.class", "target", "*.jar", "*.war", ".gradle", "build"]
```

**General Development:**
```json
[".git", ".svn", ".hg", "*.tmp", "*.swp", "*~", ".DS_Store", "Thumbs.db"]
```

### File Watching

Control how fsPrompt monitors directory changes.

| Setting | Default | Description |
|---------|---------|-------------|
| `watch_files` | true | Monitor directory for changes |
| `watch_debounce_ms` | 500 | Milliseconds to wait before processing changes |

## Performance Configuration

Optimize fsPrompt for your system and use cases.

### Processing Settings

| Setting | Default | Range | Description |
|---------|---------|-------|-------------|
| `max_concurrent_reads` | 32 | 1-128 | Maximum parallel file operations |
| `cache_size_mb` | 100 | 10-1000 | File content cache size in MB |
| `use_mmap` | false | boolean | Use memory mapping for large files |

**Performance Tuning:**

**For SSDs and Fast Systems:**
```json
{
  "performance": {
    "max_concurrent_reads": 64,
    "cache_size_mb": 200,
    "use_mmap": true
  }
}
```

**For HDDs and Slower Systems:**
```json
{
  "performance": {
    "max_concurrent_reads": 16,
    "cache_size_mb": 50,
    "use_mmap": false
  }
}
```

**For Memory-Constrained Systems:**
```json
{
  "performance": {
    "max_concurrent_reads": 8,
    "cache_size_mb": 25,
    "use_mmap": false
  }
}
```

### Memory Management

| Setting | Description | Impact |
|---------|-------------|--------|
| `cache_size_mb` | Limits memory used for file caching | Higher = faster repeated access, more RAM usage |
| `use_mmap` | Memory-map large files instead of loading | Better for very large files, platform-dependent |

## Output Configuration

Configure how output is generated and formatted.

### Format Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `output_format` | `"xml"` | Default output format (`"xml"` or `"markdown"`) |
| `include_tree` | true | Include directory structure in output |
| `tree_max_depth` | 10 | Maximum tree traversal depth |

### Export Behavior

```json
{
  "export": {
    "default_filename": "codebase-export",
    "auto_extension": true,
    "backup_existing": false,
    "confirm_overwrite": true
  }
}
```

## Advanced Configuration

Advanced settings for power users and specific use cases.

### Debug and Logging

| Setting | Default | Description |
|---------|---------|-------------|
| `debug_mode` | false | Enable debug logging |
| `log_level` | `"info"` | Logging verbosity |
| `performance_overlay` | false | Show performance metrics by default |

### Experimental Features

Some features may be experimental and require explicit enabling:

```json
{
  "experimental": {
    "enable_drag_drop": false,
    "enhanced_search": false,
    "custom_themes": false
  }
}
```

## Platform-Specific Settings

Some settings are platform-specific or have platform-specific defaults.

### Windows
```json
{
  "platform": {
    "use_native_dialogs": true,
    "follow_windows_theme": true,
    "enable_acrylic": false
  }
}
```

### macOS
```json
{
  "platform": {
    "use_native_dialogs": true,
    "follow_system_accent": true,
    "enable_vibrancy": true
  }
}
```

### Linux
```json
{
  "platform": {
    "use_native_dialogs": true,
    "follow_gtk_theme": true,
    "prefer_dark_theme": false
  }
}
```

## Configuration File Example

Complete example configuration file with commonly used settings:

```json
{
  "window_width": 1400.0,
  "window_height": 900.0,
  "split_position": 0.35,
  "last_directory": "/home/user/projects/my-app",
  "ignore_patterns": [
    ".*",
    "node_modules",
    "__pycache__",
    "target",
    "build",
    "dist",
    "_*",
    "*.log",
    "*.tmp"
  ],
  "include_tree": true,
  "output_format": "markdown",
  "theme": "auto",
  "ui": {
    "theme": "auto",
    "font_size": 14.0,
    "show_hidden": false,
    "include_tree": true
  },
  "performance": {
    "max_concurrent_reads": 32,
    "cache_size_mb": 100,
    "use_mmap": false
  }
}
```

## Configuration Management

### Resetting Configuration

To reset all settings to defaults:

1. **Through UI**: Use "Reset to Defaults" option if available
2. **Manual**: Delete the config.json file and restart the application
3. **Selective**: Edit the config.json file to remove specific settings

### Backup and Restore

**Backup Configuration:**
```bash
# Windows
copy "%APPDATA%\fsprompt\config.json" "config-backup.json"

# macOS/Linux
cp "~/.config/fsprompt/config.json" "config-backup.json"
```

**Restore Configuration:**
```bash
# Windows
copy "config-backup.json" "%APPDATA%\fsprompt\config.json"

# macOS/Linux
cp "config-backup.json" "~/.config/fsprompt/config.json"
```

### Sharing Configuration

To share settings between machines or users:

1. Export the config.json file
2. Remove machine-specific settings like `last_directory` and window position
3. Share the cleaned configuration file
4. Import by placing in the appropriate config directory

## Troubleshooting Configuration

### Common Issues

**Configuration Not Saving:**
- Check file permissions on config directory
- Ensure adequate disk space
- Verify the application has write access

**Settings Reset on Restart:**
- Check if config file is read-only
- Verify correct config file location
- Look for application permission issues

**Performance Issues:**
- Reduce `max_concurrent_reads` for slower systems
- Decrease `cache_size_mb` if memory-constrained
- Disable `use_mmap` on older systems

### Validation

The application validates configuration on startup and will:
- Use defaults for invalid values
- Show warnings for unknown settings
- Automatically fix format issues where possible

---

For immediate configuration questions, check the [Troubleshooting Guide](troubleshooting.md). For feature-specific settings, see the [Features Guide](features.md).