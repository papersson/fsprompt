# Getting Started with fsPrompt

fsPrompt is a high-performance desktop application that converts local codebases into compact "context prompts" for Large Language Models (LLMs). This guide will help you get up and running quickly.

## What is fsPrompt?

fsPrompt takes your filesystem structure and file contents and formats them into XML or Markdown output that's optimized for LLM consumption. This allows you to easily share your entire codebase context with AI assistants for code review, debugging, or development assistance.

## Installation

### Prerequisites

- **Operating Systems**: Windows 10+, macOS 12+, Linux (glibc 2.31+)
- **Memory**: 512MB minimum, 1GB recommended
- **Rust** (for building from source): 1.86+

### Building from Source

```bash
# Clone the repository
git clone https://github.com/patrikpersson/codext-rs.git
cd codext-rs

# Build release version
cargo build --release

# The binary will be in target/release/fsprompt
```

### Running the Application

```bash
# Run with development settings
cargo run

# Run with optimizations (recommended)
cargo run --release
```

## First Run Experience

When you first launch fsPrompt, you'll see a clean, modern interface with two main panels:

### Left Panel: File Browser & Controls
- **Select Directory** button to choose your project folder
- **Output format** radio buttons (XML or Markdown)
- **Include directory tree** checkbox
- **Ignore patterns** text field for excluding files
- **Search bar** for finding specific files
- **Generate button** to create the output

### Right Panel: Output Preview
- Generated content appears here
- Token count with color-coded indicators
- Copy and Save buttons for exporting

## Your First Context Prompt

1. **Select a Directory**
   - Click the "Select Directory" button
   - Navigate to your project folder
   - Choose the root directory of your codebase

2. **Browse and Select Files**
   - The directory tree will load automatically
   - Expand folders by clicking the arrows
   - Check the boxes next to files you want to include
   - Parent folders show indeterminate state when children are partially selected

3. **Configure Output**
   - Choose between XML or Markdown format
   - Decide whether to include the directory tree structure
   - Adjust ignore patterns if needed (defaults exclude common build/cache folders)

4. **Generate Output**
   - Click the "🚀 Generate" button (or press Ctrl+G)
   - Watch the progress indicator as files are processed
   - Generated output appears in the right panel

5. **Export Your Context**
   - Copy to clipboard with "📋 Copy" button (or Ctrl+C)
   - Save to file with "💾 Save" button (or Ctrl+S)

## Understanding Token Counts

fsPrompt provides real-time token estimation to help you stay within LLM limits:

- **🟢 Low** (0-999 tokens): Ideal for quick questions
- **🟡 Medium** (1,000-9,999 tokens): Good for detailed analysis
- **🔴 High** (10,000+ tokens): May hit model limits, consider reducing selection

## Performance Features

fsPrompt is designed for large codebases:

- **Lazy Loading**: Directories load content only when expanded
- **Parallel Processing**: Multiple files read simultaneously
- **Worker Threads**: UI stays responsive during generation
- **File Watching**: Automatically detects when files change
- **Memory Efficient**: Optimized for repositories with 10,000+ files

## Quick Tips

- Use **Ctrl+F** to search for files in the tree
- **Ctrl+Z/Ctrl+Shift+Z** for undo/redo selections
- Enable performance overlay with **Ctrl+Shift+P**
- Common ignore patterns are pre-configured (node_modules, .git, target, etc.)
- File changes are automatically detected and indicated

## Common Use Cases

- **Code Review**: Share entire project context with reviewers
- **AI Assistance**: Provide comprehensive codebase context to AI assistants
- **Documentation**: Generate structured overviews of project structure
- **Onboarding**: Help new team members understand codebase organization
- **Migration Planning**: Analyze code structure before refactoring

## What's Next?

- Explore the [Features Guide](features.md) for detailed functionality
- Learn [Keyboard Shortcuts](keyboard-shortcuts.md) for efficient navigation
- Customize behavior with [Configuration Options](configuration.md)
- Understand [Output Formats](output-formats.md) for different use cases
- Troubleshoot issues with the [Troubleshooting Guide](troubleshooting.md)

## Need Help?

If you encounter any issues during setup or first use, check the [Troubleshooting Guide](troubleshooting.md) for common solutions and debugging tips.