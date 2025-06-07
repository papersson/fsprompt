# fsPrompt Development Roadmap

## Overview

This document outlines the future development plans for fsPrompt, a high-performance filesystem prompt generator for LLMs. The roadmap is organized by priority and estimated implementation complexity.

## Version Milestones

### v0.2.0 - Performance Enhancements (Q1 2025)
**Focus**: Complete remaining performance optimizations for handling massive codebases

#### Features
- **Incremental Token Counting** - Maintain running token diff instead of full recalculation
  - Priority: High
  - Complexity: Medium
  - Impact: 50%+ faster token updates on large files
  
- **Chunked Clipboard Operations** - Handle clipboard operations ≥8MB efficiently
  - Priority: Medium
  - Complexity: Medium
  - Impact: Support for very large outputs without freezing
  
- **Lazy Syntax Highlighting** - Defer highlighting until output is visible
  - Priority: Medium
  - Complexity: Low
  - Impact: Faster initial generation for syntax-highlighted outputs
  
- **Pattern Cache Integration** - Activate already-implemented pattern caching
  - Priority: High
  - Complexity: Low
  - Impact: Faster ignore pattern processing

#### Benchmarks
- Generate Linux kernel (660MB, 55k files) in ≤8s
- Maintain 60 FPS with 50k+ file trees
- Copy 10MB text to clipboard in <150ms

### v0.3.0 - Enhanced Usability (Q2 2025)
**Focus**: Improve user experience with drag-and-drop and better file management

#### Features
- **Drag-and-Drop Support** 
  - Drag files/folders from OS file manager to include
  - Drag nodes within tree to reorganize
  - Visual drop indicators
  - Priority: High
  - Complexity: Medium
  
- **Multi-Root Workspace**
  - Select multiple directories to aggregate
  - Tabbed interface for switching between roots
  - Combined output generation
  - Priority: Medium
  - Complexity: High
  
- **Advanced Search**
  - Search by file content (not just names)
  - Regular expression support
  - Search history
  - Priority: Low
  - Complexity: Medium

### v0.4.0 - Accessibility & Internationalization (Q3 2025)
**Focus**: Make fsPrompt accessible to all users worldwide

#### Features
- **Full Accessibility Support**
  - Complete keyboard navigation (arrow keys, tab order)
  - Screen reader labels (ARIA/AccessKit)
  - High contrast mode enhancements
  - Reduced motion mode improvements
  - Priority: High
  - Complexity: High
  
- **Internationalization (i18n)**
  - Extract all UI strings to `.po` files
  - Support for RTL languages
  - Initial translations: Spanish, Chinese, Japanese, French, German
  - Community translation framework
  - Priority: Medium
  - Complexity: Medium
  
- **Improved Keyboard Shortcuts**
  - Customizable keybindings
  - Vim-style navigation mode
  - Command palette (Cmd+K style)
  - Priority: Low
  - Complexity: Low

### v0.5.0 - Distribution & Installation (Q4 2025)
**Focus**: Professional packaging and distribution

#### Features
- **Native Installers**
  - Windows: MSI installer with Start Menu integration
  - macOS: DMG with drag-to-Applications, code signing, notarization
  - Linux: DEB/RPM packages, Flatpak, AppImage
  - Priority: High
  - Complexity: Medium
  
- **Auto-Update System**
  - Check for updates on startup
  - Background downloads
  - Cryptographic signature verification
  - Rollback capability
  - Priority: Medium
  - Complexity: High
  
- **CI/CD Pipeline**
  - GitHub Actions matrix builds
  - Automated testing on all platforms
  - Release automation
  - Priority: High
  - Complexity: Medium

### v1.0.0 - Enterprise Features (Q1 2026)
**Focus**: Advanced features for power users and teams

#### Features
- **Plugin System**
  - Transform file contents (strip comments, minify, etc.)
  - Custom output formats
  - Language-specific processors
  - WASM plugin support
  - Priority: Low
  - Complexity: Very High
  
- **Live Diff Output**
  - Regenerate only changed sections
  - Side-by-side diff view
  - Git integration for change detection
  - Priority: Low
  - Complexity: High
  
- **Team Features**
  - Shared ignore pattern presets
  - Export/import configuration profiles
  - Usage analytics (opt-in)
  - Priority: Low
  - Complexity: Medium

## Future Considerations (Post v1.0)

### Cloud Features (Requires Privacy Review)
- Cloud sync for settings and presets
- Shareable output links
- Team workspaces
- API for CI/CD integration

### AI Integration
- Smart file selection based on query
- Automatic ignore pattern suggestions
- Token optimization recommendations
- Integration with popular LLM APIs

### Advanced Performance
- GPU-accelerated syntax highlighting
- Distributed processing for massive repos
- Incremental indexing with SQLite
- Memory-mapped virtual filesystem

## Development Principles

1. **Performance First** - Every feature must maintain 60 FPS and sub-3s generation
2. **Type Safety** - Leverage Rust's type system to prevent bugs
3. **Cross-Platform** - Features must work identically on Windows, macOS, and Linux
4. **User Privacy** - No telemetry or network access without explicit opt-in
5. **Backward Compatibility** - Configuration files remain compatible across versions

## Community Involvement

We welcome contributions! Priority areas for community help:
- Translations for internationalization
- Platform-specific testing
- Performance optimization ideas
- Accessibility testing with real screen readers
- Plugin development (once system is ready)

## Version Support Policy

- Latest stable version: Full support
- Previous minor version: Security fixes only
- Older versions: Community support

## How to Contribute

1. Check the GitHub issues for items tagged "help wanted"
2. Read `.claude/development.md` for coding standards
3. Join discussions in GitHub Discussions
4. Submit PRs with comprehensive tests

---

*This roadmap is subject to change based on user feedback and community contributions. Last updated: January 2025*