# Release Process

This document outlines the release process for fsPrompt, including versioning strategy, quality gates, and distribution procedures.

## Table of Contents

- [Versioning Strategy](#versioning-strategy)
- [Release Types](#release-types)
- [Pre-Release Checklist](#pre-release-checklist)
- [Quality Gates](#quality-gates)
- [Release Preparation](#release-preparation)
- [Release Execution](#release-execution)
- [Post-Release Tasks](#post-release-tasks)
- [Hotfix Process](#hotfix-process)
- [Distribution](#distribution)

## Versioning Strategy

fsPrompt follows [Semantic Versioning (SemVer)](https://semver.org/) with the format `MAJOR.MINOR.PATCH`:

- **MAJOR**: Breaking changes, major architectural changes
- **MINOR**: New features, backwards-compatible changes
- **PATCH**: Bug fixes, performance improvements, documentation updates

### Version Examples

- `0.1.0` - Initial release
- `0.2.0` - Added new output format support
- `0.2.1` - Fixed memory leak in file watcher
- `1.0.0` - First stable release with complete feature set
- `1.1.0` - Added drag-and-drop support
- `2.0.0` - Breaking API changes

### Pre-1.0 Versioning

During pre-1.0 development:
- MINOR versions may include breaking changes
- PATCH versions are for bug fixes and small improvements
- Breaking changes are clearly documented

## Release Types

### Development Releases (0.x.x)
- **Frequency**: As needed for feature milestones
- **Stability**: Beta quality, may have known issues
- **Distribution**: GitHub releases, limited distribution

### Stable Releases (1.x.x+)
- **Frequency**: Every 2-3 months for minor versions
- **Stability**: Production ready
- **Distribution**: Full distribution including package managers

### Patch Releases (x.x.1+)
- **Frequency**: As needed for critical bug fixes
- **Stability**: Production ready
- **Distribution**: Same as parent minor version

### Release Candidates (x.x.x-rc.n)
- **Purpose**: Pre-release testing
- **Duration**: 1-2 weeks before stable release
- **Distribution**: GitHub releases only

## Pre-Release Checklist

### Code Quality
- [ ] All CI/CD checks pass on main branch
- [ ] No known critical bugs or security issues
- [ ] Code coverage meets minimum threshold (80%+)
- [ ] Performance benchmarks within acceptable ranges
- [ ] All dependencies are up to date and secure

### Documentation
- [ ] CHANGELOG.md updated with all changes
- [ ] README.md reflects current features and requirements
- [ ] API documentation is complete and accurate
- [ ] User guide updated for new features
- [ ] Migration guide provided for breaking changes

### Testing
- [ ] All automated tests pass
- [ ] Manual testing completed on all target platforms
- [ ] Performance testing with large datasets completed
- [ ] Security scan completed (cargo audit)
- [ ] Cross-platform compatibility verified

### Legal and Compliance
- [ ] License headers present on all source files
- [ ] Third-party license compliance verified
- [ ] Export control requirements met (if applicable)

## Quality Gates

### Automated Quality Gates

1. **Continuous Integration**
   ```bash
   # All must pass
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test --all-targets
   cargo check --all-targets
   cargo audit
   ```

2. **Performance Benchmarks**
   ```bash
   # Performance must not regress by >10%
   cargo bench
   ```

3. **Security Scanning**
   ```bash
   # No high or critical vulnerabilities
   cargo audit
   ```

### Manual Quality Gates

1. **Cross-Platform Testing**
   - Windows 10/11 testing
   - macOS 12+ testing  
   - Linux (Ubuntu LTS, Fedora) testing
   - Large dataset testing (10,000+ files)

2. **User Experience Validation**
   - UI responsiveness verification
   - Error message clarity
   - Installation/setup process
   - Documentation accuracy

3. **Performance Validation**
   - Memory usage within bounds
   - Startup time acceptable
   - File processing performance meets targets
   - UI remains responsive during operations

## Release Preparation

### 1. Version Bump

Update version in `Cargo.toml`:
```toml
[package]
version = "0.2.0"  # New version
```

### 2. Update CHANGELOG.md

Follow the [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
# Changelog

## [0.2.0] - 2024-01-15

### Added
- New markdown output format with syntax highlighting
- Drag-and-drop file selection support
- Performance monitoring overlay (Ctrl+Shift+P)

### Changed
- Improved type safety with comprehensive newtype system
- Enhanced error messages with detailed context
- Updated UI theme with better contrast ratios

### Deprecated
- Legacy XML format will be removed in v1.0.0

### Removed
- Removed experimental async file reading (performance regression)

### Fixed
- Fixed memory leak in file watcher when processing large directories
- Resolved clipboard issues on Linux with large content (>8MB)
- Fixed race condition in worker thread communication

### Security
- Updated regex dependency to fix ReDoS vulnerability (RUSTSEC-2024-0001)
```

### 3. Update Documentation

- [ ] README.md feature list
- [ ] Installation instructions
- [ ] System requirements
- [ ] Known limitations
- [ ] Migration guides for breaking changes

### 4. Pre-Release Testing

```bash
# Create and test release candidate
git checkout -b release/v0.2.0
git commit -am "Prepare v0.2.0 release"

# Build release version
cargo build --release

# Run comprehensive tests
cargo test --release
cargo bench

# Manual testing on each platform
```

## Release Execution

### 1. Create Git Tag

```bash
# Ensure you're on the correct commit
git checkout main
git pull origin main

# Create and push tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### 2. GitHub Release

1. **Go to GitHub Releases page**
2. **Click "Draft a new release"**
3. **Select the version tag** (v0.2.0)
4. **Release title**: "fsPrompt v0.2.0"
5. **Release description**: Copy from CHANGELOG.md
6. **Upload binaries** (if applicable)
7. **Mark as pre-release** if version < 1.0.0
8. **Publish release**

### 3. Cargo Publication

```bash
# Dry run first
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

### 4. Update Documentation Sites

- [ ] Update GitHub Pages documentation
- [ ] Update any external documentation sites
- [ ] Notify documentation mirrors

## Post-Release Tasks

### 1. Verify Release

```bash
# Test installation from crates.io
cargo install fsprompt --version 0.2.0

# Verify functionality
fsprompt --version
```

### 2. Update Development Branch

```bash
# Bump to next development version
# Update Cargo.toml to 0.2.1-dev or 0.3.0-dev
git commit -am "Bump version to 0.3.0-dev"
git push origin main
```

### 3. Communication

- [ ] Update project status in README
- [ ] Post announcement in GitHub Discussions
- [ ] Update any external project listings
- [ ] Notify key users/stakeholders

### 4. Monitor Release

- [ ] Monitor issue reports
- [ ] Watch download/usage metrics
- [ ] Check for security vulnerability reports
- [ ] Gather user feedback

## Hotfix Process

For critical bugs or security issues that need immediate attention:

### 1. Create Hotfix Branch

```bash
# Branch from the latest release tag
git checkout v0.2.0
git checkout -b hotfix/v0.2.1
```

### 2. Implement Fix

```bash
# Make minimal changes to fix the issue
# Include tests that verify the fix
# Update CHANGELOG.md

git commit -am "Fix critical memory leak in file watcher"
```

### 3. Test Hotfix

```bash
# Run targeted tests
cargo test
cargo bench

# Manual verification on affected platforms
```

### 4. Release Hotfix

```bash
# Update version to patch level
# Update Cargo.toml: 0.2.0 -> 0.2.1

# Tag and release
git tag -a v0.2.1 -m "Hotfix v0.2.1"
git push origin v0.2.1

# Publish to crates.io
cargo publish
```

### 5. Merge Back

```bash
# Merge hotfix back to main
git checkout main
git merge hotfix/v0.2.1
git push origin main

# Clean up hotfix branch
git branch -d hotfix/v0.2.1
git push origin --delete hotfix/v0.2.1
```

## Distribution

### Current Distribution Channels

1. **Crates.io** - Primary Rust package distribution
2. **GitHub Releases** - Source code and pre-built binaries
3. **Source builds** - Users can build from source

### Future Distribution Plans

1. **Package Managers**
   - Homebrew (macOS/Linux)
   - Chocolatey (Windows)
   - APT repositories (Debian/Ubuntu)
   - RPM repositories (Fedora/RHEL)

2. **Application Stores**
   - Microsoft Store (Windows)
   - Mac App Store (macOS)
   - Snap Store (Linux)
   - Flatpak (Linux)

3. **Standalone Installers**
   - MSI installer for Windows
   - DMG installer for macOS
   - AppImage for Linux

### Binary Distribution

When providing pre-built binaries:

```bash
# Build for multiple targets
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu

# Create distribution packages
# Include README, LICENSE, CHANGELOG
# Package as zip/tar.gz with version in filename
```

## Release Metrics

Track these metrics for each release:

### Adoption Metrics
- Download counts from crates.io
- GitHub release download counts
- Issue reports and resolution time
- Community feedback and engagement

### Quality Metrics
- Number of critical bugs found post-release
- Performance regression reports
- Security vulnerability reports
- Documentation feedback

### Success Criteria
- Zero critical bugs within 1 week of release
- Performance within 5% of previous version
- Positive community feedback
- Successful adoption by existing users

## Emergency Procedures

### Security Vulnerability Response

1. **Immediate assessment** of vulnerability impact
2. **Private coordination** with security researchers
3. **Develop fix** in private repository
4. **Coordinate disclosure** timeline
5. **Release emergency patch** with security advisory
6. **Public disclosure** after patch availability

### Critical Bug Response

1. **Acknowledge issue** within 24 hours
2. **Assess impact** and affected versions
3. **Prioritize fix** based on severity
4. **Release hotfix** if needed
5. **Communicate** status to users

## Release Calendar

### Target Schedule
- **Major releases**: Annually
- **Minor releases**: Quarterly
- **Patch releases**: As needed
- **Security releases**: Immediately

### Planned Releases
- v0.2.0: Q1 2024 - Enhanced UI and performance
- v0.3.0: Q2 2024 - Drag-and-drop and accessibility
- v1.0.0: Q4 2024 - First stable release

Remember: Quality over speed. It's better to delay a release than to ship known critical issues.