# Troubleshooting Guide

This guide helps you resolve common issues with fsPrompt. Issues are organized by category with step-by-step solutions.

## Installation and Startup Issues

### Application Won't Start

**Symptoms**: Double-clicking the executable does nothing, or application crashes immediately.

**Solutions**:

1. **Check System Requirements**
   ```bash
   # Verify OS compatibility
   # Windows: 10+ required
   # macOS: 12+ required  
   # Linux: glibc 2.31+ required
   ldd --version  # Linux only
   ```

2. **Run from Terminal** (for detailed error messages)
   ```bash
   # Navigate to application directory
   ./fsprompt  # Linux/macOS
   fsprompt.exe  # Windows
   ```

3. **Check Permissions**
   ```bash
   # Make executable (Linux/macOS)
   chmod +x fsprompt
   
   # Check file permissions
   ls -la fsprompt
   ```

4. **Graphics Driver Issues**
   - Update graphics drivers
   - Try software rendering: `--software-rendering` flag
   - Check OpenGL support

### Missing Dependencies (Linux)

**Symptoms**: Error messages about missing libraries.

**Solutions**:

**Ubuntu/Debian**:
```bash
sudo apt update
sudo apt install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

**Fedora/RHEL**:
```bash
sudo dnf install gtk3-devel libxcb-devel
```

**Arch Linux**:
```bash
sudo pacman -S gtk3 libxcb
```

### Configuration Directory Issues

**Symptoms**: Settings don't persist, or permission errors.

**Solutions**:

1. **Check Config Directory**
   ```bash
   # Windows
   echo %APPDATA%\fsprompt
   
   # macOS
   echo ~/Library/Application\ Support/fsprompt
   
   # Linux
   echo ~/.config/fsprompt
   ```

2. **Fix Permissions**
   ```bash
   # Linux/macOS
   mkdir -p ~/.config/fsprompt
   chmod 755 ~/.config/fsprompt
   ```

3. **Reset Configuration**
   ```bash
   # Delete config file to reset to defaults
   rm ~/.config/fsprompt/config.json  # Linux/macOS
   del "%APPDATA%\fsprompt\config.json"  # Windows
   ```

## Directory and File Access Issues

### Directory Won't Load

**Symptoms**: "Failed to load directory" or permission errors.

**Solutions**:

1. **Check Directory Permissions**
   ```bash
   # Test directory access
   ls -la /path/to/directory  # Linux/macOS
   dir "C:\path\to\directory"  # Windows
   ```

2. **Path Length Issues** (Windows)
   - Enable long path support in Windows
   - Use shorter directory paths
   - Map long paths to drives

3. **Network Drive Issues**
   - Ensure network drive is properly mounted
   - Try local copy of directory
   - Check network connectivity

4. **Symbolic Link Problems**
   ```bash
   # Check for broken symlinks
   find /path -type l -exec test ! -e {} \; -print  # Linux/macOS
   ```

### Files Not Appearing

**Symptoms**: Expected files don't show in the tree view.

**Solutions**:

1. **Check Ignore Patterns**
   - Review ignore patterns in settings
   - Common patterns: `.*`, `node_modules`, `target`
   - Clear patterns temporarily to test

2. **Hidden Files Setting**
   - Enable "Show hidden files" option
   - Check if files start with `.` (hidden)

3. **File Permissions**
   ```bash
   # Check file readability
   test -r filename && echo "Readable" || echo "Not readable"
   ```

4. **Refresh Directory**
   - Use Ctrl+Shift+R to force refresh
   - Re-select directory if needed

### File Reading Errors

**Symptoms**: "Failed to read file" errors during generation.

**Solutions**:

1. **Encoding Issues**
   - Files with invalid UTF-8 encoding
   - Binary files mistakenly selected
   - Use text-only selections

2. **File Size Limits**
   - Very large files may timeout
   - Adjust performance settings
   - Exclude extremely large files

3. **File Locks**
   - Close files in other applications
   - Check for exclusive file locks
   - Restart applications holding locks

## Performance Issues

### Slow Directory Loading

**Symptoms**: Long delays when expanding directories or selecting folders.

**Solutions**:

1. **Adjust Performance Settings**
   ```json
   {
     "performance": {
       "max_concurrent_reads": 16,  // Reduce for slower systems
       "cache_size_mb": 50,         // Reduce memory usage
       "use_mmap": false           // Disable for HDDs
     }
   }
   ```

2. **Storage Optimization**
   - Use SSD storage when possible
   - Defragment HDD drives
   - Close other disk-intensive applications

3. **Directory Size**
   - Avoid selecting root directories (C:\, /)
   - Use more specific subdirectories
   - Exclude large build/cache directories

### High Memory Usage

**Symptoms**: Application consumes excessive RAM or system becomes slow.

**Solutions**:

1. **Reduce Cache Size**
   ```json
   {
     "performance": {
       "cache_size_mb": 25,  // Lower cache limit
       "max_concurrent_reads": 8
     }
   }
   ```

2. **File Selection**
   - Avoid selecting entire large repositories
   - Focus on specific modules/components
   - Use ignore patterns for large directories

3. **System Resources**
   - Close other memory-intensive applications
   - Increase system virtual memory
   - Consider system RAM upgrade

### UI Responsiveness

**Symptoms**: Interface becomes unresponsive or laggy.

**Solutions**:

1. **Enable Performance Overlay** (Ctrl+Shift+P)
   - Monitor FPS and memory usage
   - Identify performance bottlenecks

2. **Graphics Settings**
   - Update graphics drivers
   - Try software rendering mode
   - Reduce window size if needed

3. **Background Processing**
   - Cancel ongoing generation (Escape)
   - Wait for file watching to complete
   - Close performance-intensive applications

## Output Generation Issues

### Generation Fails or Hangs

**Symptoms**: Generation process starts but never completes or fails with errors.

**Solutions**:

1. **Check File Selection**
   - Ensure files are actually selected
   - Verify files are readable
   - Remove problematic files from selection

2. **Timeout Issues**
   - Cancel and retry with fewer files
   - Close other applications
   - Check system resources

3. **Memory Issues**
   - Reduce file selection size
   - Lower cache settings
   - Free system memory

4. **File Conflicts**
   - Check for locked files
   - Verify no files are being modified
   - Close editors/IDEs temporarily

### Incomplete Output

**Symptoms**: Output is generated but missing expected files or content.

**Solutions**:

1. **Verify Selection**
   - Check that all intended files are selected
   - Look for indeterminate checkboxes (partial selection)
   - Use Ctrl+A to select all visible

2. **Ignore Patterns**
   - Review ignore patterns for unintended exclusions
   - Temporarily clear patterns to test
   - Check for case-sensitivity issues

3. **File Access**
   - Verify read permissions on all files
   - Check for broken symbolic links
   - Ensure files haven't been moved/deleted

### Format-Specific Issues

**XML Output Problems**:
- Malformed XML: Check for special characters in filenames
- Encoding issues: Ensure UTF-8 file encoding
- Size limits: XML overhead can increase file size significantly

**Markdown Output Problems**:
- Code block formatting: Check for backticks in content
- Language detection: Verify file extensions are recognized
- Rendering issues: Test in different Markdown viewers

## Search and Navigation Issues

### Search Not Working

**Symptoms**: Search doesn't find expected files or shows no results.

**Solutions**:

1. **Search Scope**
   - Search only finds currently visible/loaded items
   - Expand directories to make files searchable
   - Use refresh if recently added files aren't found

2. **Search Query**
   - Check for typos in search terms
   - Search is case-insensitive
   - Use partial filenames for better results

3. **Clear Search Cache**
   - Clear search field and retry
   - Restart application if search seems stuck

### Keyboard Shortcuts Not Working

**Symptoms**: Expected keyboard shortcuts don't respond.

**Solutions**:

1. **Focus Issues**
   - Click in the appropriate panel first
   - Some shortcuts are context-sensitive
   - Check if modal dialogs are blocking input

2. **Platform Differences**
   - Use Cmd instead of Ctrl on macOS
   - Some shortcuts may be platform-specific
   - Check system-level shortcut conflicts

3. **Input Method**
   - Disable input methods/IME temporarily
   - Check for sticky keys or accessibility features
   - Try different keyboard if using external keyboard

## Export and Clipboard Issues

### Clipboard Copy Fails

**Symptoms**: Copy operation doesn't work or content is corrupted.

**Solutions**:

1. **System Clipboard**
   - Try pasting into different applications
   - Check clipboard managers/tools
   - Restart clipboard service if needed

2. **Content Size**
   - Large outputs may exceed clipboard limits
   - Try saving to file instead
   - Reduce selection size

3. **Format Issues**
   - Some applications expect specific formats
   - Try pasting as plain text
   - Use file export for complex formatting

### File Save Issues

**Symptoms**: Save dialog doesn't appear or save operation fails.

**Solutions**:

1. **Permissions**
   ```bash
   # Check write permissions
   test -w /path/to/directory && echo "Writable" || echo "Not writable"
   ```

2. **Disk Space**
   - Check available disk space
   - Clean temporary files if needed
   - Choose different save location

3. **File Conflicts**
   - Check if file is open in another application
   - Try different filename
   - Close file locks

## Theme and Display Issues

### Theme Not Applying

**Symptoms**: Theme changes don't take effect or revert unexpectedly.

**Solutions**:

1. **System Theme Detection**
   - Check system theme settings
   - Try manual theme selection
   - Restart application after theme change

2. **Configuration Issues**
   - Verify theme setting in config file
   - Reset configuration if corrupted
   - Check file permissions on config

3. **Graphics Compatibility**
   - Update graphics drivers
   - Try different theme options
   - Check for high contrast mode conflicts

### Display Scaling Problems

**Symptoms**: UI elements appear too large/small or blurry.

**Solutions**:

1. **System DPI Settings**
   - Check display scaling settings
   - Ensure consistent DPI across monitors
   - Restart application after DPI changes

2. **Font Size Adjustment**
   - Adjust font size in settings
   - Use Ctrl+Mouse wheel for temporary scaling
   - Check for system font scaling

3. **Multi-Monitor Issues**
   - Ensure consistent display settings
   - Try moving window to different monitor
   - Check for mixed DPI configurations

## Diagnostic Information

### Gathering Debug Information

When reporting issues, include this information:

1. **System Information**
   ```bash
   # Operating system and version
   uname -a  # Linux/macOS
   ver       # Windows
   
   # Graphics information
   lspci | grep VGA  # Linux
   system_profiler SPDisplaysDataType  # macOS
   dxdiag  # Windows
   ```

2. **Application Logs**
   - Enable debug mode if available
   - Check console output when running from terminal
   - Note exact error messages

3. **Configuration**
   - Export current configuration
   - Note any custom settings
   - Include ignore patterns and performance settings

### Performance Profiling

If experiencing performance issues:

1. **Enable Performance Overlay** (Ctrl+Shift+P)
2. **Monitor Resource Usage**
   - CPU usage during operations
   - Memory consumption
   - Disk I/O activity

3. **Test with Minimal Configuration**
   - Reset to default settings
   - Test with small directory
   - Gradually increase complexity

## Getting Additional Help

### Before Reporting Issues

1. **Update to Latest Version**
   - Check for application updates
   - Review changelog for fixes
   - Test with clean configuration

2. **Reproduce the Issue**
   - Note exact steps to reproduce
   - Test with different directories/files
   - Check if issue is consistent

3. **Check System Requirements**
   - Verify OS compatibility
   - Ensure adequate system resources
   - Update system dependencies

### Reporting Bugs

When reporting issues, include:

- **Environment**: OS, version, hardware specs
- **Steps**: Exact reproduction steps
- **Expected**: What should happen
- **Actual**: What actually happens
- **Logs**: Error messages or debug output
- **Configuration**: Relevant settings

### Community Resources

- Check project documentation
- Search existing issues
- Review FAQ sections
- Community forums or discussions

---

If you can't find a solution here, consider reporting the issue with detailed information about your system and the specific problem you're experiencing.