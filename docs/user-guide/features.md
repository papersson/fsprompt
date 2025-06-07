# Features Guide

fsPrompt offers a comprehensive set of features designed for efficient codebase analysis and context generation. This guide provides detailed explanations of all available functionality.

## Core Features

### 🗂️ Native Directory Selection

fsPrompt uses your operating system's native file dialog for directory selection, providing a familiar and reliable experience.

**How it works:**
- Click "Select Directory" to open native file picker
- Navigate to your project root directory
- Selected directory becomes the base for all file operations
- Supports all standard filesystem paths and symbolic links

**Benefits:**
- Familiar interface across all platforms
- Proper permission handling
- Support for network drives and mounted volumes

### 🌳 Interactive Directory Tree

The file browser provides a rich, interactive tree view of your codebase.

**Features:**
- **Expand/Collapse**: Click arrows to navigate directory structure
- **Tri-state Checkboxes**: Full, partial, or no selection with visual indicators
- **Parent/Child Propagation**: Selecting a folder selects all children
- **Lazy Loading**: Directory contents load only when expanded
- **Visual Icons**: 📁 for folders, 📄 for files

**Selection Behavior:**
- ✓ **Checked**: Item and all children selected
- ⬜ **Unchecked**: Item and all children not selected
- ➖ **Indeterminate**: Some children selected, others not

### 📐 Split-Pane Interface

The interface uses a resizable 30/70 split layout for optimal workspace organization.

**Layout:**
- **Left Pane (30%)**: File browser, controls, and configuration
- **Right Pane (70%)**: Output preview and token information
- **Resizable**: Drag the divider to adjust proportions
- **Responsive**: Adapts to different window sizes

### 🚀 High-Performance Processing

fsPrompt is optimized for large codebases with advanced performance features.

**Parallel Processing:**
- Multiple files read simultaneously using worker threads
- Non-blocking UI during generation
- Progress tracking with detailed status updates
- Cancellation support for long-running operations

**Memory Optimization:**
- Lazy loading prevents unnecessary memory usage
- Efficient string handling for large outputs
- Automatic cleanup of unused resources

**Performance Monitoring:**
- Real-time FPS display (Ctrl+Shift+P)
- Memory usage tracking
- Generation time measurements

## File Management Features

### 🔍 Smart Search & Filtering

Powerful search capabilities help you find files quickly in large codebases.

**Tree Search:**
- **Fuzzy Search**: Type partial names to find matching files
- **Real-time Results**: Results update as you type
- **Clear Function**: ✕ button to clear search
- **Keyboard Shortcut**: Ctrl+F to focus search

**Output Search:**
- **In-content Search**: Find text within generated output
- **Match Navigation**: ↑/↓ buttons to jump between results
- **Match Counter**: Shows current match and total count
- **Escape to Close**: Press Escape to exit search mode

### 🚫 Ignore Patterns

Flexible pattern system to exclude unwanted files and directories.

**Default Patterns:**
- `.*` - Hidden files (starting with dot)
- `node_modules` - Node.js dependencies
- `__pycache__` - Python cache files
- `target` - Rust build directory
- `build` - Generic build directories
- `dist` - Distribution directories
- `_*` - Files starting with underscore

**Pattern Types:**
- **Exact**: Literal string matching
- **Glob**: Wildcard patterns with * and ?
- **Custom**: Add your own patterns separated by commas

**Configuration:**
- Edit patterns in the text field
- Changes apply immediately to the tree view
- Patterns persist between sessions

### 📁 File Watching

Automatic detection of filesystem changes keeps your view current.

**Features:**
- **Real-time Monitoring**: Detects file and directory changes
- **Visual Indicators**: ⚠️ warning when files have changed
- **Refresh Option**: One-click refresh to reload directory
- **Background Processing**: Monitoring doesn't impact performance

**Supported Changes:**
- File creation and deletion
- Directory structure modifications
- File content updates
- Permission changes

## Output Generation

### 📄 Dual Format Support

Choose between XML and Markdown formats based on your needs.

**XML Format:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<codebase>
  <directory_tree>
    <![CDATA[
    📁 src/
    ├── 📄 main.rs
    └── 📁 modules/
        └── 📄 parser.rs
    ]]>
  </directory_tree>
  <files>
    <file path="src/main.rs">
      <![CDATA[
      // File contents here
      ]]>
    </file>
  </files>
</codebase>
```

**Markdown Format:**
```markdown
# Codebase Export

## Directory Structure
```
📁 src/
├── 📄 main.rs
└── 📁 modules/
    └── 📄 parser.rs
```

## Files

### src/main.rs
```rust
// File contents here
```
```

### 🌲 Directory Tree Generation

Optional directory tree inclusion provides structural context.

**Features:**
- **Visual Tree**: Unicode characters for clean presentation
- **Icon Support**: 📁 folders, 📄 files for easy scanning
- **Depth Limiting**: Prevents infinite recursion
- **Pattern Filtering**: Respects ignore patterns in tree generation
- **Sorting**: Directories first, then alphabetical

**Benefits:**
- LLMs understand project structure at a glance
- Helps with navigation and file relationship questions
- Provides context for relative imports and dependencies

### 📊 Token Estimation

Real-time token counting helps you optimize prompt size for different LLMs.

**Estimation Method:**
- Approximately 4 characters per token
- Real-time calculation as you select files
- Visual indicators for different token levels

**Token Levels:**
- **🟢 Low (0-999)**: Quick questions, specific issues
- **🟡 Medium (1,000-9,999)**: Detailed analysis, code review
- **🔴 High (10,000+)**: Comprehensive analysis, may hit limits

**Model Considerations:**
- GPT-3.5: ~4,000 token limit
- GPT-4: ~8,000-32,000 tokens depending on version
- Claude: ~100,000+ tokens for recent versions

## User Experience Features

### ↩️ Undo/Redo System

Complete selection history with unlimited undo/redo capability.

**Features:**
- **Automatic Snapshots**: Selection changes recorded automatically
- **Keyboard Shortcuts**: Ctrl+Z (undo), Ctrl+Shift+Z (redo)
- **History Limit**: 20 operations maintained for performance
- **Visual Feedback**: Selections update immediately

**Use Cases:**
- Recover from accidental bulk selections
- Experiment with different file combinations
- Step through selection refinement process

### 🎨 Theme Support

Automatic theme detection with manual override options.

**Theme Options:**
- **System**: Follows OS light/dark mode preference
- **Light**: Force light theme regardless of system setting
- **Dark**: Force dark theme regardless of system setting

**Features:**
- **Automatic Detection**: Respects system preferences by default
- **Instant Switching**: Theme changes apply immediately
- **Persistent Settings**: Choice saved between sessions
- **Consistent Styling**: All UI elements adapt to selected theme

### 🔔 Toast Notifications

Non-intrusive feedback system for user actions and system events.

**Notification Types:**
- **Success** (🟢): Directory loaded, file saved, etc.
- **Warning** (🟡): File changes detected, potential issues
- **Error** (🔴): Failed operations, permission issues
- **Progress** (🔵): Long-running operation status

**Features:**
- **Auto-dismiss**: Notifications fade after appropriate duration
- **Non-blocking**: Don't interfere with workflow
- **Contextual**: Relevant information for each action

### 📋 Export Options

Multiple ways to export your generated context prompts.

**Clipboard Integration:**
- **One-click Copy**: 📋 button or Ctrl+C
- **Format Preserved**: Maintains formatting for paste operations
- **Cross-platform**: Works on Windows, macOS, and Linux

**File Export:**
- **Save Dialog**: Native file picker for save location
- **Format Extension**: Automatically adds .xml or .md extension
- **Overwrite Protection**: Confirmation for existing files

## Advanced Features

### 📈 Performance Overlay

Developer-focused performance monitoring for optimization and debugging.

**Metrics Displayed:**
- **FPS**: Real-time frame rate
- **Memory Usage**: Current application memory consumption
- **Generation Time**: How long output generation takes
- **File Count**: Number of files processed

**Access:**
- **Keyboard Shortcut**: Ctrl+Shift+P to toggle
- **Non-intrusive**: Overlay doesn't block functionality
- **Debug Information**: Helpful for performance analysis

### 🔧 Configuration Persistence

All settings and preferences persist between application sessions.

**Saved Settings:**
- Window size and position
- Split pane ratio
- Last used directory
- Theme preference
- Ignore patterns
- Output format preference

**Storage Location:**
- **Windows**: `%APPDATA%/fsprompt/config.json`
- **macOS**: `~/Library/Application Support/fsprompt/config.json`
- **Linux**: `~/.config/fsprompt/config.json`

## Accessibility Features

### Keyboard Navigation

Full keyboard support for efficient operation without mouse.

**Global Shortcuts:**
- **Ctrl+G**: Generate output
- **Ctrl+C**: Copy to clipboard
- **Ctrl+S**: Save to file
- **Ctrl+F**: Focus search
- **Ctrl+Z**: Undo selection
- **Ctrl+Shift+Z**: Redo selection
- **Ctrl+Shift+P**: Toggle performance overlay

**Navigation:**
- **Tab**: Move between interface elements
- **Space**: Toggle checkboxes
- **Enter**: Activate buttons
- **Escape**: Close search/dialogs

### Visual Indicators

Clear visual feedback for all interactive elements.

**Selection States:**
- **Hover Effects**: Visual feedback on mouse over
- **Focus Indicators**: Clear focus rings for keyboard navigation
- **State Colors**: Different colors for different selection states
- **Progress Bars**: Visual progress during generation

## Platform-Specific Features

### Windows
- Native file dialogs with Windows styling
- Windows theme integration
- Proper handling of Windows paths and drives

### macOS
- macOS native file picker
- System theme detection
- Retina display optimization

### Linux
- GTK file dialogs where available
- Desktop environment theme integration
- Proper handling of symbolic links and permissions

---

For specific configuration details, see the [Configuration Guide](configuration.md). For keyboard shortcuts reference, see [Keyboard Shortcuts](keyboard-shortcuts.md).