# Type System Architecture

## Philosophy: Make Illegal States Unrepresentable

The fsPrompt type system is designed around the principle of **type-driven development**. Every domain concept gets its own type, making the code self-documenting and preventing entire classes of bugs at compile time.

## Core Type System Principles

### 1. Newtypes Over Primitives
Instead of using raw primitives, we wrap them in meaningful types:

```rust
// ❌ Primitive hell - easy to mix up
fn process_file(path: String, size: u64, tokens: usize) -> Result<String, String>

// ✅ Type-safe domain modeling
fn process_file(path: CanonicalPath, size: FileSize, tokens: TokenCount) -> Result<OutputContent, ProcessingError>
```

### 2. Validation at Construction
Types validate their constraints when created, ensuring invariants throughout the lifetime:

```rust
impl FontSize {
    pub fn new(size: f32) -> Result<Self, ValidationError> {
        if size >= Self::MIN && size <= Self::MAX {
            Ok(Self(size))
        } else {
            Err(ValidationError::FontSizeOutOfRange)
        }
    }
}
```

### 3. Immutable Value Types
All core types are immutable values with explicit transformation methods:

```rust
impl TokenCount {
    pub const fn new(count: usize) -> Self { Self(count) }
    pub const fn get(&self) -> usize { self.0 }
    pub const fn level(&self) -> TokenLevel { /* classification logic */ }
}
```

## Type Categories

### Path Types - Security & Validation

```rust
/// Canonical, validated filesystem path
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalPath(PathBuf);

impl CanonicalPath {
    /// Creates canonical path, resolving symlinks
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self>
    
    /// Security: ensure path stays within root
    pub fn is_contained_within(&self, root: &CanonicalPath) -> bool
    
    /// Secure path creation within root bounds
    pub fn new_within_root(path: impl AsRef<Path>, root: &CanonicalPath) -> std::io::Result<Self>
}
```

**Design Rationale**: 
- Prevents path traversal attacks
- Eliminates symlink confusion
- Makes path validation explicit and reusable

### Measurement Types - Semantic Clarity

```rust
/// File size with optimization hints
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileSize(u64);

impl FileSize {
    pub const fn read_strategy(&self) -> FileReadStrategy {
        if self.0 < 256 * 1024 { // 256KB threshold
            FileReadStrategy::Direct
        } else {
            FileReadStrategy::MemoryMapped
        }
    }
}

/// Token count with level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenCount(usize);

impl TokenCount {
    pub const fn level(&self) -> TokenLevel {
        match self.0 {
            0..=999 => TokenLevel::Low,
            1000..=9999 => TokenLevel::Medium,
            _ => TokenLevel::High,
        }
    }
}
```

**Design Rationale**:
- Encodes business logic in types
- Enables size-based optimizations
- Provides semantic meaning to numbers

### Progress Types - Structured Reporting

```rust
/// Progress tracking with percentage calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgressCount {
    current: usize,
    total: usize,
}

impl ProgressCount {
    pub fn percentage(&self) -> f32 {
        if self.total == 0 { 100.0 } 
        else { (self.current as f32 / self.total as f32) * 100.0 }
    }
    
    pub const fn is_complete(&self) -> bool {
        self.current >= self.total
    }
}
```

**Design Rationale**:
- Makes progress calculations explicit
- Prevents division by zero
- Standardizes progress reporting

### Configuration Types - Builder Pattern

```rust
/// Application configuration with builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub ui: UiConfig,
    pub ignore_patterns: Vec<String>,
    pub performance: PerformanceConfig,
}

/// Builder for type-safe construction
#[derive(Debug, Default)]
pub struct AppConfigBuilder {
    window: Option<WindowConfig>,
    ui: Option<UiConfig>,
    // ...
}

impl AppConfigBuilder {
    pub fn window(mut self, window: WindowConfig) -> Self {
        self.window = Some(window);
        self
    }
    
    pub fn build(self) -> AppConfig {
        AppConfig {
            window: self.window.unwrap_or_default(),
            // ... with sensible defaults
        }
    }
}
```

**Design Rationale**:
- Makes configuration construction explicit
- Provides sensible defaults
- Validates relationships between settings

## Validation Strategy

### 1. Constructor Validation

```rust
/// Non-empty string with validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(s: String) -> Result<Self, ValidationError> {
        if s.trim().is_empty() {
            Err(ValidationError::EmptyString)
        } else {
            Ok(Self(s))
        }
    }
}
```

### 2. Range Validation

```rust
/// Window ratio with bounds checking
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowRatio(f32);

impl WindowRatio {
    pub fn new(ratio: f32) -> Result<Self, ValidationError> {
        if (0.0..=1.0).contains(&ratio) {
            Ok(Self(ratio))
        } else {
            Err(ValidationError::RatioOutOfRange)
        }
    }
}
```

### 3. Business Rule Validation

```rust
impl CanonicalPath {
    /// Prevents directory traversal attacks
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
}
```

## Error Types - Structured Failure

```rust
/// Validation errors with context
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    FontSizeOutOfRange,
    RatioOutOfRange,
    EmptyString,
}

/// System-level errors
#[derive(Debug, Clone, PartialEq)]
pub enum SystemError {
    TimeError(String),
    MutexPoisoned(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FontSizeOutOfRange => write!(f, "Font size must be between 8.0 and 24.0"),
            Self::RatioOutOfRange => write!(f, "Ratio must be between 0.0 and 1.0"),
            Self::EmptyString => write!(f, "String cannot be empty"),
        }
    }
}
```

## Serialization Bridge Types

For types that need to cross serialization boundaries, we provide bridge types:

```rust
/// Serializable wrapper for CanonicalPath
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SerializableCanonicalPath(PathBuf);

impl SerializableCanonicalPath {
    pub fn from_canonical(path: &CanonicalPath) -> Self {
        Self(path.as_path().to_path_buf())
    }
    
    pub fn to_canonical(&self) -> Result<CanonicalPath, std::io::Error> {
        CanonicalPath::new(&self.0)
    }
}

// Convenient conversions
impl From<&CanonicalPath> for SerializableCanonicalPath { /* ... */ }
impl TryFrom<SerializableCanonicalPath> for CanonicalPath { /* ... */ }
```

## Pattern Matching with Types

Types enable exhaustive pattern matching for business logic:

```rust
impl FileSize {
    pub const fn read_strategy(&self) -> FileReadStrategy {
        const MEMORY_MAP_THRESHOLD: u64 = 256 * 1024;
        
        if self.0 < MEMORY_MAP_THRESHOLD {
            FileReadStrategy::Direct
        } else {
            FileReadStrategy::MemoryMapped
        }
    }
}

match entry.file_size() {
    Some(size) => match size.read_strategy() {
        FileReadStrategy::Direct => read_directly(path),
        FileReadStrategy::MemoryMapped => memory_map_file(path),
    },
    None => handle_directory(entry),
}
```

## Benefits Realized

### 1. Compile-Time Correctness
- Can't pass wrong types to functions
- Can't forget to validate inputs
- Can't mix up similar values

### 2. Self-Documenting Code
- Function signatures tell the story
- Business rules encoded in types
- Clear intentions and constraints

### 3. Fearless Refactoring
- Type errors guide refactoring
- Impossible to break contracts accidentally
- Compiler catches breaking changes

### 4. Performance Optimizations
- Zero-cost abstractions
- Optimization hints embedded in types
- Efficient memory layout

## Type-Driven Development Workflow

1. **Start with types.rs** - Understand available domain types
2. **Use newtypes everywhere** - Never use raw primitives for domain concepts
3. **Validate at boundaries** - Construct types safely at system edges
4. **Let compiler guide** - Type errors reveal design issues
5. **Extend as needed** - Add new types when patterns emerge

This type system makes fsPrompt both safe and performant while maintaining excellent developer experience.