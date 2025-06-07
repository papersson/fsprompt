# fsPrompt Basic Usage Guide

This guide covers the fundamental workflows for using fsPrompt to generate LLM context prompts from your codebase.

## Getting Started

### First Launch

1. **Build and Run**
   ```bash
   cd fsPrompt
   cargo run --release
   ```

2. **Initial Window**
   - Left panel: Directory tree and controls
   - Right panel: Output preview
   - Split is resizable by dragging the divider

### Quick Start Workflow

The fastest way to generate a prompt:

1. Click **"Select Directory"** → Choose your project folder
2. Check files you want to include
3. Click **"🚀 Generate"** 
4. Copy output with **Ctrl+C** or save with **Ctrl+S**

## Core Workflows

### Workflow 1: Single File Analysis

**Use Case**: Get LLM help with a specific file  
**Time**: ~30 seconds

1. **Select Directory**
   - Click "Select Directory"
   - Navigate to your project root
   - Click "Select Folder"

2. **Find Your File**
   - Use fuzzy search: press **Ctrl+K** and type filename
   - Or manually expand folders in the tree
   - Click the checkbox next to your target file

3. **Generate and Copy**
   - Choose format: **XML** (cleaner) or **Markdown** (readable)
   - Click "🚀 Generate"
   - Press **Ctrl+C** to copy to clipboard

**Result**: Clean prompt with just your file and its context

### Workflow 2: Feature-Focused Prompt

**Use Case**: Working on a specific feature across multiple files  
**Time**: ~2 minutes

1. **Select Project Directory**
   - Choose your project root (e.g., `~/projects/my-app`)

2. **Configure Ignore Patterns**
   - In "Ignore Patterns" field, add: `node_modules,*.log,dist,build`
   - This excludes build artifacts and dependencies

3. **Select Feature Files**
   - Expand relevant directories (e.g., `src/components/auth/`)
   - Check individual files related to your feature
   - Use search to find related files: type "auth" to find authentication-related files

4. **Include Directory Tree** 
   - Check "Include directory tree" to give LLM project context
   - This shows the full project structure (respecting ignore patterns)

5. **Generate**
   - Click "🚀 Generate"
   - Review token count (aim for Medium/Low for better LLM performance)
   - Copy with **Ctrl+C**

**Result**: Focused prompt with feature files plus project structure context

### Workflow 3: Whole Project Overview

**Use Case**: Getting LLM to understand your entire codebase  
**Time**: ~5 minutes

1. **Select Root Directory**
   - Choose project root

2. **Set Comprehensive Ignore Patterns**
   ```
   node_modules,target,dist,build,*.log,*.tmp,.git,.env,coverage,__pycache__
   ```

3. **Smart Selection Strategy**
   - **Don't select everything at once** (too many tokens)
   - Start with core directories: `src/`, `lib/`, `components/`
   - Check key configuration files: `package.json`, `Cargo.toml`, `README.md`

4. **Use Directory Selection**
   - Click folder checkboxes to select entire directories
   - fsPrompt automatically expands and selects all children
   - Watch token count - aim to stay under 32k tokens

5. **Generate with Tree**
   - Enable "Include directory tree"
   - Choose **Markdown** format for better readability
   - Generate and review output

**Result**: Comprehensive codebase overview perfect for architecture discussions

### Workflow 4: Code Review Preparation

**Use Case**: Preparing context for code review or documentation  
**Time**: ~3 minutes

1. **Select Changed Files**
   - Navigate to your project
   - Use git to identify changed files: `git diff --name-only main`
   - Manually select these files in fsPrompt tree

2. **Add Related Context**
   - Include test files for the changed code
   - Include interface/type definition files
   - Add documentation files if they exist

3. **Use Markdown Format**
   - Choose Markdown for better readability in review tools
   - Include directory tree for structural context

4. **Save for Later**
   - Use **Ctrl+S** to save output to file
   - Name it descriptively: `feature-review-2025-01-07.md`

**Result**: Well-organized code review document with full context

## Interface Deep Dive

### Directory Tree Navigation

**Expanding Folders**
- Click the **▶** arrow or double-click folder name
- Use **→** key when folder is selected
- **Shift+→** expands all children recursively

**Selecting Files**
- **Single files**: Click the checkbox next to filename
- **Entire folders**: Click folder checkbox (selects all children)
- **Mixed selection**: Folders show indeterminate state (gray box) when some children selected

**Keyboard Navigation**
- **↑/↓**: Navigate between items
- **Space**: Toggle selection of current item
- **→/←**: Expand/collapse folders
- **Ctrl+A**: Select all visible files
- **Ctrl+D**: Deselect all

### Search and Filtering

**Fuzzy Search** (Ctrl+K)
- Type partial filename: `comp` finds `component.js`, `complex.py`
- Search is case-insensitive
- Press Enter to jump to first match
- Esc to clear search

**Ignore Patterns**
- Comma-separated glob patterns: `*.log,node_modules,dist`
- Affects both tree display and directory tree in output
- Changes take effect immediately
- Patterns are saved automatically

### Output Formats

**XML Format**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<codebase>
  <file path="src/main.rs">
    <content><![CDATA[
fn main() {
    println!("Hello, world!");
}
    ]]></content>
  </file>
</codebase>
```

**Benefits**: Clean, structured, easy for LLMs to parse
**Best for**: Code analysis, formal documentation

**Markdown Format**
```markdown
# Codebase Export

Generated 1 file, estimated 15 tokens

## Directory Structure
```
src/
├── main.rs
└── lib.rs
```

## File: src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```
```

**Benefits**: Human-readable, great for documentation
**Best for**: Code reviews, documentation, sharing with humans

### Token Management

**Token Count Display**
- **Green (Low)**: 0-8,000 tokens - Perfect for most LLMs
- **Yellow (Medium)**: 8,000-32,000 tokens - Good for larger context windows
- **Red (High)**: 32,000+ tokens - May hit LLM limits

**Token Optimization Tips**
1. **Use ignore patterns** to exclude irrelevant files
2. **Select specific files** rather than entire large directories
3. **Exclude generated code** (build outputs, dependencies)
4. **Break large projects** into multiple prompts by feature area
5. **Remove comments** from files if just analyzing logic (manual preprocessing)

## Advanced Features

### Undo/Redo (Ctrl+Z/Ctrl+Shift+Z)

- 20 levels of selection history
- Works across directory changes
- Remembers exact checkbox states
- Useful when experimenting with different file selections

### Auto-Refresh

- fsPrompt watches your filesystem for changes
- Shows toast notification when files change
- Click "Refresh" button to reload the tree
- Maintains your current selections where possible

### Performance Monitoring

- Press **Ctrl+Shift+P** to toggle performance overlay
- Shows FPS, memory usage, and generation times
- Useful for diagnosing performance with large repositories
- Disable when not needed to maximize performance

### Configuration Persistence

**Automatically Saved**:
- Last selected directory
- Window size and position
- Split panel ratio
- Theme preference
- Ignore patterns

**Config Location**: `~/.config/fsprompt/config.json`

## Troubleshooting Common Issues

### "No files selected" Message
- Ensure at least one file checkbox is checked
- Try expanding folders to see file contents
- Check if ignore patterns are hiding your files

### Slow Performance
- Add ignore patterns for large directories: `node_modules,target,.git`
- Use search instead of scrolling through large trees
- Close performance overlay if enabled

### Large Token Counts
- Use more aggressive ignore patterns
- Select specific files instead of entire directories
- Consider breaking project into multiple prompts
- Focus on core logic files only

### Memory Issues
- Restart fsPrompt after processing very large repositories
- Exclude binary files and large data files
- Use XML format (lower memory usage)

## Best Practices

### Preparation
1. **Know your goal** - What specific help do you need from the LLM?
2. **Clean your workspace** - Remove temporary files before selection
3. **Use .gitignore as reference** - Good patterns for excluding irrelevant files

### Selection Strategy
1. **Start small** - Begin with core files, expand as needed
2. **Think like an LLM** - Include files that provide necessary context
3. **Be selective** - More files ≠ better results
4. **Include interfaces** - Type definitions and contracts are valuable context

### Output Management
1. **Save important prompts** - Use descriptive filenames
2. **Version your prompts** - Include dates for tracking
3. **Test token limits** - Verify your LLM can handle the output size
4. **Backup configurations** - Save your ignore patterns and frequently used selections

---

*Next: [Advanced Patterns](advanced-patterns.md) for complex workflows and power-user features*