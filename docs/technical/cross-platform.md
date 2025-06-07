# Cross-Platform Reference

This document details platform-specific behaviors, limitations, and implementation considerations for fsPrompt across Windows, macOS, and Linux.

## Platform Abstractions

### File System Operations

#### Path Handling

**Windows**
- Uses backslash (`\`) as path separator
- Supports drive letters (C:, D:, etc.)
- Path length limited to 260 characters (legacy) or 32,767 characters (long path aware)
- Case-insensitive filesystem on NTFS (configurable)
- Handles UNC paths (`\\server\share`)

**macOS** 
- Uses forward slash (`/`) as path separator
- Case-insensitive by default (HFS+/APFS), but case-preserving
- Maximum path length: 1024 characters
- Supports resource forks and extended attributes
- Unicode normalization differences (NFD vs NFC)

**Linux**
- Uses forward slash (`/`) as path separator  
- Case-sensitive filesystem (ext4, xfs, etc.)
- Maximum path length: 4096 characters
- Full Unicode support without normalization
- Supports various filesystem types with different characteristics

#### Implementation Strategy

fsPrompt uses Rust's `std::path` module for cross-platform path handling:

```rust
// CanonicalPath handles platform differences automatically
impl CanonicalPath {
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        Ok(Self(path.as_ref().canonicalize()?))
    }
}
```

The `canonicalize()` method handles:
- Path separator normalization
- Relative path resolution  
- Symlink resolution (where supported)
- Case normalization (on case-insensitive filesystems)

### File Dialog Integration

#### Native File Dialogs (`rfd` crate)

**Windows**
- Uses Windows API (`IFileDialog` interface)
- Integrates with Windows Explorer
- Supports recent locations and favorites
- Respects Windows file associations and icons

**macOS**
- Uses Cocoa `NSOpenPanel`/`NSSavePanel`
- Integrates with Finder
- Supports macOS-specific features like tags and previews
- Follows macOS Human Interface Guidelines

**Linux**
- Uses GTK portal or fallback to GTK dialogs
- Integrates with desktop environment file managers
- Supports freedesktop.org standards
- Graceful fallback for headless environments

#### Configuration

```rust
pub fn handle_directory_selection(&mut self) {
    if let Some(path) = rfd::FileDialog::new().pick_folder() {
        // Cross-platform path handling
        if let Ok(canonical_path) = CanonicalPath::new(&path) {
            // Process selected directory
        }
    }
}
```

### Clipboard Operations

#### Cross-Platform Clipboard (`arboard` crate)

**Windows**
- Uses Win32 Clipboard API
- Supports multiple clipboard formats
- Handles clipboard history (Windows 10+)
- Thread-safe implementation

**macOS**
- Uses NSPasteboard API
- Supports multiple pasteboard types
- Integrates with Universal Clipboard (continuity)
- Respects macOS sandboxing requirements

**Linux**
- Uses X11 clipboard protocols (PRIMARY/CLIPBOARD)
- Supports Wayland through wl-clipboard
- Handles multiple display servers
- Requires proper X11/Wayland session

#### Implementation

```rust
pub fn copy_to_clipboard(&mut self) {
    use arboard::Clipboard;
    
    if let Some(content) = &self.state.output.content {
        match Clipboard::new() {
            Ok(mut clipboard) => {
                match clipboard.set_text(content.as_str()) {
                    Ok(()) => self.toast_manager.success("Copied to clipboard!"),
                    Err(e) => self.toast_manager.error(format!("Failed to copy: {}", e)),
                }
            }
            Err(e) => self.toast_manager.error(format!("Failed to access clipboard: {}", e)),
        }
    }
}
```

## Performance Characteristics

### File System Performance

#### Directory Scanning

**Windows**
- NTFS: Fast metadata access, good for large directories
- ReFS: Optimized for large files and volumes
- Network drives: Significant performance penalty
- Antivirus impact: Real-time scanning can slow operations

**macOS**
- APFS: Optimized for SSD, fast metadata operations
- HFS+: Slower on large directories
- Network volumes: Performance varies by protocol (SMB, AFP, NFS)
- Spotlight indexing: Can interfere with file operations

**Linux**
- ext4: Excellent performance for most workloads
- xfs: Better for large files and directories
- btrfs: Good features but variable performance
- Network filesystems: NFS performance depends on configuration

#### Memory Mapping

Platform-specific memory mapping thresholds:

```rust
impl FileSize {
    pub const fn read_strategy(&self) -> FileReadStrategy {
        const MEMORY_MAP_THRESHOLD: u64 = 256 * 1024; // 256KB
        
        if self.0 < MEMORY_MAP_THRESHOLD {
            FileReadStrategy::Direct
        } else {
            FileReadStrategy::MemoryMapped  // Platform-optimized
        }
    }
}
```

**Windows**
- Supports large memory mappings (64-bit)
- File mapping handles are automatically cached
- Good performance on NTFS

**macOS** 
- Unified buffer cache improves performance
- Memory pressure handling affects mapping
- Good integration with virtual memory system

**Linux**
- Excellent memory mapping performance
- `mmap` with `MAP_POPULATE` for better performance
- Transparent huge pages can improve large file handling

### Threading and Concurrency

#### Parallel Operations

```rust
// Platform-aware thread configuration
builder.threads(num_cpus::get().min(8)); // Respect system capabilities
```

**Windows**
- Thread pool integration with I/O completion ports
- NUMA-aware scheduling on multi-socket systems
- Good scaling on high-core-count systems

**macOS**
- Grand Central Dispatch integration
- Efficient context switching
- Power management affects performance

**Linux**
- Native pthread implementation
- Excellent scaling on many-core systems
- CPU affinity and scheduling policies available

## UI Integration

### Native Look and Feel

#### Theme Support

```rust
pub enum Theme {
    Light,
    Dark,
    System,  // Follows platform preferences
}
```

**Windows**
- Respects Windows theme (light/dark mode)
- High DPI scaling support
- Windows 11 visual style integration

**macOS**
- Follows macOS appearance (light/dark/auto)
- Retina display optimization
- macOS-specific UI conventions

**Linux**
- Follows GTK/Qt theme settings
- Desktop environment integration
- Handles various DPI configurations

#### Font Rendering

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontSize(f32);

impl FontSize {
    pub const MIN: f32 = 8.0;
    pub const MAX: f32 = 24.0;  // Adjusted for platform DPI
}
```

**Windows**
- ClearType font rendering
- System font scaling
- Per-monitor DPI awareness

**macOS**
- Subpixel font rendering
- Dynamic Type support
- Retina display optimization

**Linux**
- FreeType font rendering
- Fontconfig integration
- Variable DPI handling

## Platform-Specific Limitations

### File System Limitations

**Windows**
```
- Reserved names: CON, PRN, AUX, NUL, COM1-9, LPT1-9
- Invalid characters: < > : " | ? * \0
- Path length: 260 chars (legacy), 32767 chars (long paths)
- Case insensitive (usually)
```

**macOS**
```
- Colon (:) converted to slash in display
- Case insensitive but preserving
- Unicode normalization (NFD)
- Resource forks and extended attributes
```

**Linux**
```
- Only NUL character forbidden in filenames
- Case sensitive
- No path length limit (filesystem dependent)
- Wide variety of filesystem types
```

### Performance Gotchas

#### Windows-Specific

1. **Antivirus interference**: Real-time scanning can significantly slow file operations
2. **Network drive enumeration**: UNC paths can cause timeouts
3. **Long path limitations**: Requires Windows 10 RS1+ and manifest opt-in
4. **Case sensitivity**: Rare NTFS configurations can cause confusion

#### macOS-Specific

1. **Unicode normalization**: Filenames may appear different than expected
2. **Resource forks**: Can cause size discrepancies  
3. **App sandboxing**: File access may require user permission
4. **Spotlight indexing**: Can slow directory operations

#### Linux-Specific

1. **Filesystem diversity**: Performance varies significantly between filesystems
2. **Permission models**: SELinux/AppArmor can affect file access
3. **Character encoding**: Filename encoding may not be UTF-8
4. **Network filesystems**: NFS/CIFS performance highly variable

## Development and Testing

### Cross-Platform Testing Strategy

1. **Automated CI/CD**: Test on Windows, macOS, and Linux
2. **Path handling tests**: Verify behavior with platform-specific paths
3. **Unicode tests**: Test with various character encodings and normalizations
4. **Performance benchmarks**: Measure on different filesystem types
5. **Integration tests**: Test file dialogs and clipboard operations

### Platform-Specific Build Configuration

```toml
[target.'cfg(windows)'.dependencies]
# Windows-specific optimizations

[target.'cfg(target_os = "macos")'.dependencies]  
# macOS-specific features

[target.'cfg(unix)'.dependencies]
# Unix/Linux specific functionality
```

### Debug and Diagnostics

Platform-specific debugging tools:

**Windows**
- Process Monitor for file access tracing
- Event Viewer for system errors
- Performance Toolkit for profiling

**macOS**
- Console.app for system logs
- Instruments for performance profiling
- fs_usage for filesystem tracing

**Linux**
- strace for system call tracing
- inotify for filesystem events
- perf for performance analysis

## Best Practices

### Cross-Platform Development

1. **Use platform abstractions**: Rely on Rust standard library and well-tested crates
2. **Test early and often**: Don't assume behavior is identical across platforms
3. **Handle edge cases**: Plan for platform-specific limitations and quirks
4. **Graceful degradation**: Provide fallbacks when platform features are unavailable
5. **Performance awareness**: Optimize for the lowest common denominator while allowing platform-specific optimizations

### Deployment Considerations

1. **Target-specific builds**: Optimize binaries for each platform
2. **Bundle dependencies**: Include necessary runtime libraries
3. **Permission handling**: Document required permissions on each platform
4. **Installation packages**: Use platform-appropriate installation mechanisms
5. **Update mechanisms**: Implement platform-appropriate update strategies