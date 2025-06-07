# Security Reference

This document outlines the security measures and access controls implemented in fsPrompt to prevent path traversal attacks and ensure safe filesystem operations.

## Path Traversal Prevention

### CanonicalPath Type System

The core security model relies on the `CanonicalPath` newtype defined in `src/core/types.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalPath(PathBuf);
```

Key security features:

- **Automatic canonicalization**: All paths are resolved through `canonicalize()` which:
  - Resolves symlinks to prevent symlink-based traversal attacks
  - Normalizes path separators and removes `.` and `..` components
  - Returns an absolute path

- **Root containment validation**: The `is_contained_within()` method ensures paths remain within the expected directory tree:
  ```rust
  pub fn is_contained_within(&self, root: &CanonicalPath) -> bool {
      self.0.starts_with(&root.0)
  }
  ```

- **Secure path construction**: The `new_within_root()` method prevents path traversal during construction:
  ```rust
  pub fn new_within_root(path: impl AsRef<Path>, root: &CanonicalPath) -> std::io::Result<Self> {
      let canonical = Self::new(path)?;
      if !canonical.is_contained_within(root) {
          return Err(std::io::Error::new(
              std::io::ErrorKind::PermissionDenied,
              "Path traversal detected: path escapes root directory",
          ));
      }
      Ok(canonical)
  }
  ```

### Filesystem Operations Security

#### Directory Scanning (`src/utils/parallel_fs.rs`)

1. **Root path validation**: All scanning operations start with canonical root creation:
   ```rust
   let canonical_root = match CanonicalPath::new(root) {
       Ok(cr) => cr,
       Err(_) => return Vec::new(), // Fail-safe: return empty
   };
   ```

2. **Per-entry validation**: Each discovered path is validated against the root:
   ```rust
   if let Ok(canonical_path) = 
       CanonicalPath::new_within_root(&path_buf, &canonical_root) {
       // Process only validated paths
   }
   ```

3. **Symlink prevention**: The walker configuration explicitly disables symlink following:
   ```rust
   builder.follow_links(false); // Don't follow symlinks
   ```

#### File Reading Operations

1. **Secure parallel reading**: The `read_files_parallel_secure()` function validates all paths before reading:
   ```rust
   // Validate path is within root
   if !path.is_contained_within(root) {
       return (
           path.clone(),
           Err("Security error: path traversal detected".to_string()),
       );
   }
   ```

2. **Memory-mapped file safety**: Large file reading with memory mapping includes validation:
   - File existence verification before mapping
   - UTF-8 validation for text content
   - Proper error handling for mapping failures

## Access Control Boundaries

### Application-Level Controls

1. **Directory selection**: Only user-selected directories through native file dialogs are allowed as roots
2. **Pattern-based filtering**: Ignore patterns prevent access to sensitive directories by default:
   ```rust
   ignore_patterns: vec![
       ".*".to_string(),           // Hidden files
       "node_modules".to_string(),  // Package directories  
       "__pycache__".to_string(),   // Python cache
       "target".to_string(),        // Rust build artifacts
       "build".to_string(),         // Build directories
       "dist".to_string(),          // Distribution directories
   ]
   ```

3. **Configuration file security**: Config files use `SerializableCanonicalPath` for safe serialization without exposing the security model

### Platform-Specific Considerations

#### File Dialog Security
- Uses `rfd` crate for native file dialogs which provide OS-level security
- No manual path parsing from user input
- Relies on OS sandbox and permission models

#### Clipboard Access
- Uses `arboard` crate for cross-platform clipboard access
- Handles clipboard access failures gracefully
- No sensitive path information exposed in clipboard content

## Error Handling and Logging

### Security-Aware Error Messages

1. **Path validation errors**: Generic error messages prevent information disclosure:
   ```rust
   Err("Security error: path traversal detected".to_string())
   ```

2. **File access errors**: Detailed errors only for legitimate access failures:
   ```rust
   Err(format!("Failed to get metadata for {}: file may not exist or be inaccessible", 
              path.as_path().display()))
   ```

### Security Event Logging

The application logs security-relevant events through the toast notification system:
- Directory access attempts
- File reading failures  
- Path validation failures
- Configuration loading/saving errors

## Security Best Practices

### Development Guidelines

1. **Always use CanonicalPath**: Never use raw `PathBuf` or `&Path` for user-provided paths
2. **Validate before processing**: All filesystem operations must validate paths against the root
3. **Fail-safe defaults**: Security violations should result in empty results, not errors that might reveal information
4. **Minimize attack surface**: Use type system to make security violations impossible at compile time

### Deployment Considerations

1. **Sandboxing**: Run fsPrompt in a sandboxed environment when possible
2. **File permissions**: Ensure the application runs with minimal necessary file permissions
3. **Network isolation**: fsPrompt operates entirely locally and requires no network access
4. **Audit trails**: Monitor file access patterns for suspicious activity in enterprise environments

## Security Testing

### Automated Tests

The security model is validated through:

1. **Path traversal tests**: Ensure `..` sequences are properly handled
2. **Symlink tests**: Verify symlinks cannot escape the root directory  
3. **Edge case tests**: Test with unusual paths, Unicode characters, and platform-specific paths

### Manual Security Review Checklist

- [ ] All user-provided paths use `CanonicalPath`
- [ ] Root containment is verified before filesystem operations
- [ ] Error messages don't leak sensitive path information
- [ ] File dialogs are used instead of text input for path selection
- [ ] Pattern matching uses compiled patterns to prevent regex injection
- [ ] Memory mapping includes proper bounds checking

## Known Limitations

1. **Platform path limits**: Very long paths may cause issues on some platforms
2. **Permission handling**: Application relies on OS permissions and may not detect all access restrictions
3. **Race conditions**: File access permissions could change between validation and reading
4. **Symbolic link resolution**: Complex symbolic link chains might cause performance issues

## Threat Model

### Mitigated Threats

- **Path traversal attacks**: Prevented by canonical path validation
- **Symlink attacks**: Blocked by disabling symlink following
- **Directory escape**: Impossible due to root containment validation
- **Malicious patterns**: Compiled pattern validation prevents regex injection

### Residual Risks

- **Social engineering**: User could deliberately select malicious directories
- **Time-of-check-time-of-use**: Files could be modified between scanning and reading
- **Resource exhaustion**: Very large directory trees could consume excessive memory
- **Platform vulnerabilities**: Underlying OS or Rust stdlib vulnerabilities