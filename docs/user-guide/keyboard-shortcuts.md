# Keyboard Shortcuts Reference

fsPrompt provides comprehensive keyboard shortcuts for efficient operation without relying on mouse interaction. This reference covers all available shortcuts organized by functionality.

## Global Shortcuts

These shortcuts work from anywhere in the application:

| Shortcut | Action | Description |
|----------|--------|-------------|
| **Ctrl+G** | Generate Output | Start output generation process |
| **Ctrl+C** | Copy to Clipboard | Copy generated output to clipboard |
| **Ctrl+S** | Save to File | Save generated output to a file |
| **Ctrl+F** | Focus Search | Focus the tree search box |
| **Ctrl+Z** | Undo Selection | Undo the last selection change |
| **Ctrl+Shift+Z** | Redo Selection | Redo the previously undone selection |
| **Ctrl+Shift+P** | Performance Overlay | Toggle performance monitoring display |

### Notes on Global Shortcuts

- **Generate (Ctrl+G)**: Only works when a directory is selected and generation is not already in progress
- **Copy (Ctrl+C)**: Only available when output content exists
- **Save (Ctrl+S)**: Only available when output content exists
- **Search (Ctrl+F)**: Automatically focuses the tree search field; if already focused, moves to output search when output is available

## File Tree Navigation

Navigate and interact with the directory tree efficiently:

| Key | Action | Description |
|-----|--------|-------------|
| **Tab** | Next Element | Move focus to next interactive element |
| **Shift+Tab** | Previous Element | Move focus to previous interactive element |
| **Space** | Toggle Selection | Toggle checkbox state for focused item |
| **Enter** | Expand/Collapse | Toggle folder expansion state |
| **Arrow Keys** | Navigate Tree | Move selection up/down in tree view |
| **Home** | First Item | Jump to first item in tree |
| **End** | Last Item | Jump to last visible item in tree |

### Tree Navigation Tips

- Use **Arrow Keys** to quickly navigate through files and folders
- **Space** provides fastest way to select/deselect items
- **Enter** is useful for expanding large directory structures
- **Tab** moves between tree and other controls

## Search Operations

Efficient searching within the application:

### Tree Search
| Shortcut | Action | Description |
|----------|--------|-------------|
| **Ctrl+F** | Open Tree Search | Focus the file search box |
| **Escape** | Clear Search | Clear search and return to full tree view |
| **Enter** | Navigate Results | Jump to first search result |

### Output Search
| Shortcut | Action | Description |
|----------|--------|-------------|
| **Ctrl+F** | Open Output Search | Open search in output panel (when output exists) |
| **Enter** | Next Match | Jump to next search match |
| **Shift+Enter** | Previous Match | Jump to previous search match |
| **Escape** | Close Search | Close output search and clear highlights |
| **F3** | Find Next | Alternative to Enter for next match |
| **Shift+F3** | Find Previous | Alternative to Shift+Enter for previous match |

### Search Behavior Notes

- **Context-Sensitive**: Ctrl+F opens tree search by default, output search when output panel is focused
- **Real-time Results**: Search results update as you type
- **Case-Insensitive**: All searches ignore case by default
- **Fuzzy Matching**: Tree search supports partial filename matching

## Selection Management

Manage file selections efficiently:

| Shortcut | Action | Description |
|----------|--------|-------------|
| **Ctrl+A** | Select All Visible | Select all currently visible files |
| **Ctrl+Shift+A** | Deselect All | Clear all selections |
| **Ctrl+I** | Invert Selection | Toggle selection state of all visible items |
| **Ctrl+Z** | Undo Selection | Revert to previous selection state |
| **Ctrl+Shift+Z** | Redo Selection | Restore undone selection state |

### Selection Shortcuts Context

- **Select All**: Only affects currently visible items in tree view
- **Undo/Redo**: Maintains 20 levels of selection history
- **Invert**: Useful for excluding specific files from large selections

## Interface Navigation

Move efficiently between different parts of the interface:

| Shortcut | Action | Description |
|----------|--------|-------------|
| **Tab** | Next Control | Move to next interactive element |
| **Shift+Tab** | Previous Control | Move to previous interactive element |
| **Alt+1** | Files Panel | Focus the files/tree panel |
| **Alt+2** | Output Panel | Focus the output preview panel |
| **F6** | Switch Panels | Toggle focus between left and right panels |

## Dialog and Modal Operations

Handle dialogs and modal windows:

| Shortcut | Action | Description |
|----------|--------|-------------|
| **Enter** | Confirm/OK | Confirm current dialog or action |
| **Escape** | Cancel/Close | Cancel current dialog or close modal |
| **Tab** | Next Button | Move between dialog buttons |
| **Space** | Activate Button | Activate focused button or checkbox |

## Output Operations

Work with generated output content:

| Shortcut | Action | Description |
|----------|--------|-------------|
| **Ctrl+C** | Copy All | Copy entire output to clipboard |
| **Ctrl+S** | Save Output | Save output to file with save dialog |
| **Ctrl+F** | Search Output | Open in-content search |
| **Ctrl+Home** | Top of Output | Scroll to beginning of output |
| **Ctrl+End** | End of Output | Scroll to end of output |
| **Page Up** | Scroll Up | Scroll output view up one page |
| **Page Down** | Scroll Down | Scroll output view down one page |

## Application Control

Control the application itself:

| Shortcut | Action | Description |
|----------|--------|-------------|
| **F11** | Toggle Fullscreen | Enter/exit fullscreen mode |
| **Ctrl+Q** | Quit Application | Close application (Linux/macOS) |
| **Alt+F4** | Quit Application | Close application (Windows) |
| **F1** | Help/About | Show application information |

## Advanced Shortcuts

Power-user features and debugging:

| Shortcut | Action | Description |
|----------|--------|-------------|
| **Ctrl+Shift+P** | Performance Overlay | Toggle FPS and memory display |
| **Ctrl+Shift+D** | Debug Mode | Enable debug logging (if available) |
| **Ctrl+Shift+R** | Refresh Tree | Force reload current directory |
| **Ctrl+Shift+C** | Clear Cache | Clear internal file cache |

## Context-Specific Behavior

Some shortcuts behave differently based on current context:

### When Tree Search is Active
- **Escape**: Clears search and returns to full tree view
- **Enter**: Selects first search result
- **Arrow Keys**: Navigate through search results

### When Output Search is Active
- **Escape**: Closes search and removes highlights
- **Enter/F3**: Jump to next match
- **Shift+Enter/Shift+F3**: Jump to previous match

### During Generation
- **Escape**: Cancel current generation process
- **Space**: Pause/resume generation (if supported)
- **Ctrl+C**: Copy partial results (if any)

### When No Directory Selected
- **Enter**: Open directory selection dialog
- **Ctrl+O**: Alternative to open directory selection
- **Drag+Drop**: Accept dropped directories (when supported)

## Platform-Specific Shortcuts

### Windows
| Shortcut | Action |
|----------|--------|
| **Alt+F4** | Quit Application |
| **Ctrl+Shift+N** | New Window |
| **Windows+V** | Paste from History |

### macOS
| Shortcut | Action |
|----------|--------|
| **Cmd+Q** | Quit Application |
| **Cmd+W** | Close Window |
| **Cmd+M** | Minimize Window |
| **Cmd+H** | Hide Application |

### Linux
| Shortcut | Action |
|----------|--------|
| **Ctrl+Q** | Quit Application |
| **Alt+F10** | Maximize Window |
| **Super+Left/Right** | Snap Window |

## Customization

### Modifier Key Equivalents
On different platforms, modifier keys may be mapped differently:

- **Ctrl** (Windows/Linux) ↔ **Cmd** (macOS)
- **Alt** (Windows/Linux) ↔ **Option** (macOS)
- **Shift** remains consistent across platforms

### Accessibility
- All shortcuts support standard accessibility features
- Screen readers announce shortcut availability
- High contrast mode affects shortcut visual indicators
- Sticky keys and other accessibility tools are supported

## Quick Reference Card

### Most Common Operations
```
Ctrl+G  → Generate
Ctrl+C  → Copy
Ctrl+S  → Save
Ctrl+F  → Search
Ctrl+Z  → Undo
Space   → Select/Deselect
Enter   → Expand/Collapse
Tab     → Navigate
```

### Power User Combos
```
Ctrl+Shift+P → Performance
Ctrl+Shift+Z → Redo
Ctrl+Shift+A → Deselect All
Alt+1/2      → Switch Panels
F6           → Panel Focus
```

---

**Tip**: Most shortcuts are also available through tooltips when hovering over buttons and controls. This provides a helpful reminder of available shortcuts while learning the interface.