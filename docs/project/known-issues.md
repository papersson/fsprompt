# fsPrompt Known Issues & Limitations

This document tracks current limitations, known issues, and their workarounds in fsPrompt v0.1.0.

## Current Limitations

### Feature Limitations

#### 1. No Drag-and-Drop Support
**Issue**: Users cannot drag files or folders from the OS file manager into fsPrompt  
**Impact**: Must use the "Select Directory" button and tree navigation  
**Workaround**: Use the fuzzy search (Ctrl+K) to quickly find files after selecting a directory  
**Planned Fix**: v0.3.0 (Q2 2025)  

#### 2. Single-Root Workspace Only
**Issue**: Can only work with one directory at a time  
**Impact**: Cannot combine files from multiple projects in one output  
**Workaround**: 
- Generate outputs separately and manually combine
- Use symbolic links to create a unified directory structure
**Planned Fix**: v0.3.0 with multi-root workspace support  

#### 3. Manual Directory Selection Required
**Issue**: Must manually navigate to desired directory each session  
**Impact**: Extra clicks for frequently used directories  
**Workaround**: fsPrompt remembers the last selected directory  
**Planned Fix**: Recent directories list and favorites in v0.2.0  

#### 4. English Language Only
**Issue**: UI is only available in English  
**Impact**: Non-English speakers must use English interface  
**Workaround**: None currently available  
**Planned Fix**: v0.4.0 with full internationalization  

### Performance Limitations

#### 5. Large Clipboard Operations May Freeze UI
**Issue**: Copying very large outputs (>10MB) can cause brief UI freeze  
**Impact**: 1-2 second delay when copying massive codebases  
**Workaround**: 
- Use "Save to File" instead for very large outputs
- Break large selections into smaller chunks
**Planned Fix**: v0.2.0 with chunked clipboard operations  

#### 6. Syntax Highlighting Blocks Generation
**Issue**: Syntax highlighting is applied during generation, slowing initial output  
**Impact**: Slower generation for Markdown format with many files  
**Workaround**: 
- Use XML format for faster generation
- Disable syntax highlighting in viewer if implemented
**Planned Fix**: v0.2.0 with lazy syntax highlighting  

### Platform-Specific Issues

#### 7. macOS File Dialog Focus Issues
**Issue**: File dialog may appear behind main window on some macOS versions  
**Impact**: Dialog appears stuck or non-responsive  
**Workaround**: 
- Click on fsPrompt in the dock to bring it forward
- Use Cmd+Tab to cycle to the file dialog
**Status**: Investigating platform-specific behavior  

#### 8. Linux: Some File Managers Don't Update Watch Events
**Issue**: Changes made in certain file managers may not trigger auto-refresh  
**Impact**: User must manually refresh when files change  
**Workaround**: 
- Use manual refresh button when needed
- Switch to a different file manager (Nautilus, Dolphin work well)
**Status**: Depends on file manager's inotify implementation  

#### 9. Windows: Long Path Names May Cause Issues
**Issue**: Windows path length limitations can cause errors  
**Impact**: Cannot process files with very long paths  
**Workaround**: 
- Enable long path support in Windows 10/11 (requires admin)
- Move repositories to shorter paths (e.g., C:\dev\)
**Status**: Investigating UNC path support  

## Development and Build Issues

### 10. Multiple Crate Version Warnings
**Issue**: Cargo shows warnings about multiple versions of some crates  
**Impact**: Slightly larger binary size, harmless warnings during build  
**Workaround**: Warnings can be ignored - they don't affect functionality  
**Status**: Will be resolved as dependencies update  

### 11. Missing Documentation Warnings
**Issue**: Some public items lack documentation comments  
**Impact**: Documentation generation shows warnings  
**Workaround**: Does not affect functionality  
**Planned Fix**: Documentation improvements in ongoing releases  

### 12. Large Memory Usage During Initial Scan
**Issue**: Memory usage spikes when first loading very large directories  
**Impact**: Temporary high memory usage (typically <30 seconds)  
**Workaround**: 
- Close other memory-intensive applications during large scans
- Use ignore patterns to exclude large directories (node_modules, target)
**Status**: Working as designed - memory is released after scan  

## User Interface Issues

### 13. Tree Scrolling Can Be Jumpy with Mouse Wheel
**Issue**: Rapid mouse wheel scrolling in large trees may feel unresponsive  
**Impact**: Less smooth navigation in very large file trees  
**Workaround**: 
- Use Page Up/Page Down keys for large jumps
- Use arrow keys for precise navigation
- Use search to jump to specific files
**Status**: Inherent limitation of immediate-mode GUI  

### 14. No Undo for Directory Selection
**Issue**: Undo/Redo only applies to file selections, not directory changes  
**Impact**: Cannot undo accidentally changing the root directory  
**Workaround**: Manually re-select the previous directory  
**Planned Fix**: Extended undo system in v0.3.0  

### 15. Theme Changes Require Restart
**Issue**: Switching between Auto/Light/Dark themes doesn't always update immediately  
**Impact**: May need to restart application to see theme change  
**Workaround**: Restart fsPrompt after changing theme  
**Status**: Investigating egui theme refresh behavior  

## Performance Monitoring

### 16. Performance Overlay Affects Performance
**Issue**: The performance overlay itself uses CPU and may lower FPS slightly  
**Impact**: FPS may appear lower than actual when overlay is enabled  
**Workaround**: Disable overlay (Ctrl+Shift+P) for maximum performance  
**Status**: Expected behavior - overlay is for development use  

## Data Integrity

### 17. Config File Corruption on Improper Shutdown
**Issue**: Force-quitting the application may corrupt configuration file  
**Impact**: Settings reset to defaults on next startup  
**Workaround**: 
- Always close fsPrompt normally when possible
- Backup `~/.config/fsprompt/config.json` for important settings
**Planned Fix**: Atomic config writes in v0.2.0  

## Network and Security

### 18. No Network Functionality
**Issue**: Cannot fetch remote repositories or sync settings  
**Impact**: Must work with local files only  
**Workaround**: Clone repositories locally first  
**Status**: By design for security and privacy  

## Accessibility

### 19. Limited Screen Reader Support
**Issue**: Some UI elements may not be properly announced by screen readers  
**Impact**: Reduced accessibility for visually impaired users  
**Workaround**: Use keyboard navigation where possible  
**Planned Fix**: v0.4.0 with full AccessKit integration  

### 20. No High Contrast Mode
**Issue**: Dark/Light themes may not provide sufficient contrast for some users  
**Impact**: Reduced visibility for users with visual impairments  
**Workaround**: Use system high contrast mode if supported  
**Planned Fix**: v0.4.0 with dedicated high contrast themes  

## Error Handling

### 21. Cryptic Error Messages for Permission Issues
**Issue**: File permission errors may show technical details instead of user-friendly messages  
**Impact**: Users may not understand why files cannot be read  
**Workaround**: Check file/directory permissions manually  
**Planned Fix**: Improved error messages in ongoing releases  

## Workaround Strategies

### General Performance Tips
1. **Use ignore patterns** to exclude large directories (`node_modules`, `target`, `.git`)
2. **Start with smaller directories** to test performance on your system
3. **Close other resource-intensive applications** during large operations
4. **Use SSD storage** for better file I/O performance

### Memory Management
1. **Restart fsPrompt** after processing very large repositories
2. **Use XML format** for lower memory usage compared to Markdown
3. **Process large codebases in sections** rather than all at once

### UI Responsiveness
1. **Use keyboard shortcuts** for faster navigation
2. **Collapse unused tree sections** to improve rendering performance
3. **Use fuzzy search** instead of manual scrolling for large trees

## Reporting New Issues

If you encounter issues not listed here:

1. **Check the GitHub issues** to see if it's already reported
2. **Include system information**: OS version, Rust version, fsPrompt version
3. **Provide reproduction steps** with minimal test case
4. **Include relevant log output** if available
5. **Test with a clean configuration** (backup and delete config directory)

### Debug Information
To get debug information for issue reports:
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Check config directory
ls -la ~/.config/fsprompt/

# System information
cargo --version
rustc --version
```

---

*This document is updated with each release. Last updated: January 2025 for v0.1.0*