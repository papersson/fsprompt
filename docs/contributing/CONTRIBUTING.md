# Contributing to fsPrompt

Welcome to the fsPrompt contributor community! This guide will help you understand our development practices, code standards, and contribution process.

## Table of Contents

- [Quick Start](#quick-start)
- [Type-Driven Development](#type-driven-development)
- [Code Style Guidelines](#code-style-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing Requirements](#testing-requirements)
- [Performance Considerations](#performance-considerations)
- [Cross-Platform Compatibility](#cross-platform-compatibility)
- [Code Review Process](#code-review-process)

## Quick Start

1. **Fork and clone** the repository
2. **Set up your environment** - See [development-setup.md](./development-setup.md)
3. **Read the type system** - **MANDATORY**: Review `src/core/types.rs` before any implementation
4. **Run verification** - Ensure your environment works: `cargo fmt && cargo clippy && cargo test && cargo check --all-targets`
5. **Pick an issue** - Look for "good first issue" or "help wanted" labels
6. **Follow our workflow** - Type-driven development with tight verification loops

## Type-Driven Development

**This is the core philosophy of fsPrompt.** Type-driven development is non-negotiable and must be followed for all contributions.

### Before Any Implementation

1. **Read `src/core/types.rs`** - This is mandatory. The type system is the foundation of correctness.
2. **Use existing types** - Don't reinvent wheels. Use `CanonicalPath` instead of `PathBuf`, `TokenCount` instead of `usize`, etc.
3. **Extend types when needed** - If you need a new abstraction, add it to `types.rs` first.

### Type System Principles

1. **Use newtypes for domain concepts**
   ```rust
   // ❌ Don't do this
   fn count_tokens(content: &str) -> usize { /* ... */ }
   
   // ✅ Do this
   fn count_tokens(content: &str) -> TokenCount { /* ... */ }
   ```

2. **Make illegal states unrepresentable**
   ```rust
   // ❌ Don't rely on runtime validation
   struct WindowConfig {
       ratio: f32, // Could be invalid
   }
   
   // ✅ Use validated types
   struct WindowConfig {
       ratio: WindowRatio, // Guaranteed valid
   }
   ```

3. **Leverage the compiler**
   - Type errors are your friend - they prevent bugs
   - Use `#[must_use]` for important return values
   - Implement `Debug` for all public types

### Development Workflow

1. **Start with types** - Define your data structures first
2. **Let the compiler guide you** - Fix type errors before runtime errors
3. **Verify continuously** - Run `cargo check` after every small change
4. **Test the types** - Write unit tests for your type constructors and methods

## Code Style Guidelines

### Linting Configuration

We use extremely strict linting to maintain code quality:

```rust
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs
)]
```

Additional limits (see `clippy.toml`):
- Cognitive complexity: ≤30
- Function lines: ≤100
- Function arguments: ≤7
- All public items must have documentation

### Formatting

- Use `cargo fmt` - no exceptions
- Line length: 100 characters (rustfmt default)
- Use trailing commas in multi-line constructs

### Documentation

1. **Module documentation**: Every module needs `//!` doc comments
2. **Public items**: All public functions, structs, enums need `///` doc comments
3. **Examples**: Include examples in doc comments where helpful
4. **Safety**: Document any unsafe code thoroughly

### Error Handling

- Use `Result<T, E>` for fallible operations
- Create custom error types when appropriate
- Provide meaningful error messages with context

## Pull Request Process

### Before Opening a PR

1. **Ensure your branch is up to date** with the main branch
2. **Run the full verification suite**:
   ```bash
   cargo fmt && cargo clippy && cargo test && cargo check --all-targets
   ```
3. **Test cross-platform compatibility** if touching filesystem or UI code
4. **Update documentation** if adding new public APIs

### PR Requirements

- **Clear title** - Describe what the PR does, not how
- **Linked issue** - Reference the issue number if applicable
- **Description** - Explain the motivation and approach
- **Test coverage** - Include tests for new functionality
- **Breaking changes** - Document any breaking changes clearly

### PR Template

```markdown
## Summary
Brief description of what this PR does and why.

## Changes
- List the main changes
- Include any breaking changes
- Note any new dependencies

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing performed
- [ ] Cross-platform testing (if applicable)

## Performance Impact
Describe any performance implications (positive or negative).

## Documentation
- [ ] Public APIs documented
- [ ] Examples updated if needed
- [ ] Architecture docs updated if needed
```

## Testing Requirements

### Test Categories

1. **Unit Tests** - Test individual components in isolation
   - Place in `#[cfg(test)]` modules
   - Test both success and error cases
   - Use descriptive test names

2. **Integration Tests** - Test component interactions
   - Use the `tests/` directory
   - Test complete workflows
   - Use realistic test data

3. **Performance Tests** - Benchmark critical paths
   - Use `criterion` for benchmarks
   - Test with large datasets (10,000+ files)
   - Measure memory usage and timing

### Test Guidelines

- **Test names** should clearly describe what is being tested
- **Use temporary directories** for filesystem tests
- **Mock external dependencies** when possible
- **Test error conditions** not just happy paths
- **Include performance assertions** for critical operations

See [testing-guide.md](./testing-guide.md) for detailed testing instructions.

## Performance Considerations

fsPrompt is designed to handle large codebases (10,000+ files) efficiently. Keep these principles in mind:

### UI Responsiveness
- **Never block the UI thread** - Use worker threads for heavy operations
- **Use channels** for worker communication (`crossbeam` channels)
- **Share state carefully** - Use `Arc<Mutex<T>>` for shared state
- **Request repaints** - Clone `egui::Context` to call `request_repaint()` from workers

### File Operations
- **Use memory mapping** for files >256KB (`memmap2` crate)
- **Validate UTF-8** after memory mapping with `str::from_utf8`
- **Parallel directory traversal** with `ignore::WalkBuilder::build_parallel()`
- **Batch operations** - Process files in chunks, not individually

### Memory Management
- **Use `Arc` for shared ownership** across threads
- **Prefer borrowing** over cloning when possible
- **Cache expensive computations** (regex compilation, etc.)
- **Implement virtualization** for large lists in UI

### Benchmarking
- **Profile before optimizing** - Use actual data to guide decisions
- **Use criterion** for consistent benchmarks
- **Test with realistic datasets** - Large repositories with varied file sizes
- **Monitor memory usage** - Use tools like `heaptrack` or `valgrind`

## Cross-Platform Compatibility

fsPrompt runs on Windows, macOS, and Linux. Test your changes across platforms when possible.

### Platform-Specific Considerations

1. **File Paths**
   - Always use `CanonicalPath` for filesystem operations
   - Test with paths containing spaces and unicode characters
   - Be aware of case sensitivity differences

2. **Clipboard Operations**
   - Test with large content (≥8MB) on all platforms
   - Handle clipboard errors gracefully
   - macOS has specific threading requirements (handled by `rfd`)

3. **File Dialogs**
   - Use `rfd::AsyncFileDialog` for non-blocking dialogs
   - Test dialog cancellation behavior
   - Ensure proper parent window association

4. **Performance Characteristics**
   - File I/O performance varies significantly between platforms
   - Memory mapping behavior differs on Windows vs Unix
   - Thread scheduling varies between platforms

## Code Review Process

### For Contributors

1. **Self-review first** - Review your own code before requesting review
2. **Respond promptly** to reviewer feedback
3. **Ask questions** if feedback is unclear
4. **Test suggestions** from reviewers thoroughly

### For Reviewers

1. **Focus on correctness** first, then style and performance
2. **Check type safety** - Ensure proper use of the type system
3. **Verify test coverage** - New code should have appropriate tests
4. **Consider performance** - Flag potential performance issues
5. **Be constructive** - Provide suggestions, not just criticism

### Review Checklist

- [ ] Type safety: Proper use of newtypes and domain types
- [ ] Error handling: Appropriate use of `Result` types
- [ ] Performance: No obvious bottlenecks or inefficiencies
- [ ] Documentation: Public APIs are documented
- [ ] Tests: Adequate test coverage for new functionality
- [ ] Cross-platform: Considers platform differences if applicable
- [ ] Breaking changes: Properly documented and justified

## Getting Help

- **Documentation**: Check the `docs/` directory for detailed guides
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Use GitHub Discussions for design questions
- **Type System**: Review `src/core/types.rs` and related documentation

## Commit Guidelines

- Use clear, descriptive commit messages
- Keep commits focused - one logical change per commit
- Reference issue numbers when applicable
- Follow conventional commit format when possible

```
feat: add token count estimation for markdown output
fix: resolve memory leak in file watcher
docs: update contributing guidelines for type system
test: add integration tests for directory selection
perf: optimize parallel file reading for large repositories
```

Thank you for contributing to fsPrompt! Your efforts help make this tool better for the entire LLM development community.