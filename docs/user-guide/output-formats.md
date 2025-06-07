# Output Formats Guide

fsPrompt supports two output formats optimized for different use cases: XML and Markdown. This guide explains the structure, advantages, and use cases for each format.

## Format Overview

| Format | Best For | File Extension | Structure |
|--------|----------|----------------|-----------|
| **XML** | Structured data, API integration, formal analysis | `.xml` | Hierarchical, machine-readable |
| **Markdown** | Documentation, human reading, GitHub/GitLab | `.md` | Readable, platform-friendly |

## XML Format

XML format provides a structured, machine-readable representation of your codebase that's ideal for programmatic processing and formal analysis.

### Structure

```xml
<?xml version="1.0" encoding="UTF-8"?>
<codebase>
  <directory_tree>
    <![CDATA[
    📁 project-root/
    ├── 📁 src/
    │   ├── 📄 main.rs
    │   └── 📁 modules/
    │       └── 📄 parser.rs
    └── 📄 README.md
    ]]>
  </directory_tree>
  
  <files>
    <file path="src/main.rs">
      <![CDATA[
fn main() {
    println!("Hello, world!");
}
      ]]>
    </file>
    
    <file path="src/modules/parser.rs">
      <![CDATA[
pub struct Parser {
    content: String,
}
      ]]>
    </file>
    
    <file path="README.md">
      <![CDATA[
# My Project
This is a sample project.
      ]]>
    </file>
  </files>
</codebase>
```

### Key Features

**CDATA Sections**: All content is wrapped in CDATA sections to preserve:
- Special characters (`<`, `>`, `&`)
- Code formatting and indentation
- Raw file content without XML escaping

**Relative Paths**: File paths are relative to the selected root directory, making the output portable and context-aware.

**Hierarchical Structure**: Clear separation between directory structure and file contents.

### XML Advantages

- **Machine Readable**: Easy to parse with XML libraries
- **Structured Data**: Well-defined schema for automated processing
- **Namespace Support**: Can be extended with custom namespaces
- **Validation**: Can be validated against XML schemas
- **Tool Integration**: Works with XML-aware editors and tools

### XML Use Cases

1. **API Integration**: Feed codebase data into analysis APIs
2. **Documentation Generation**: Transform into other formats via XSLT
3. **Database Import**: Import structured data into databases
4. **Formal Analysis**: Use with code analysis tools that expect XML
5. **CI/CD Pipelines**: Integrate with build systems that process XML

## Markdown Format

Markdown format provides a human-readable representation that's perfect for documentation, sharing, and platforms that support Markdown rendering.

### Structure

```markdown
# Codebase Export

Generated on 2024-01-15 at 14:30:25

## Directory Structure

```
📁 project-root/
├── 📁 src/
│   ├── 📄 main.rs
│   └── 📁 modules/
│       └── 📄 parser.rs
└── 📄 README.md
```

## Files

### src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

### src/modules/parser.rs

```rust
pub struct Parser {
    content: String,
}
```

### README.md

```markdown
# My Project
This is a sample project.
```
```

### Key Features

**Syntax Highlighting**: File contents include appropriate language identifiers for syntax highlighting in viewers that support it.

**Clean Hierarchy**: Uses standard Markdown headers to organize content logically.

**Platform Compatibility**: Renders properly on GitHub, GitLab, documentation sites, and Markdown editors.

**Unicode Icons**: Uses 📁 and 📄 emojis for visual distinction between folders and files.

### Language Detection

fsPrompt automatically detects file languages based on extensions:

| Extension | Language | Syntax Highlighting |
|-----------|----------|-------------------|
| `.rs` | Rust | `rust` |
| `.js` | JavaScript | `javascript` |
| `.ts` | TypeScript | `typescript` |
| `.py` | Python | `python` |
| `.java` | Java | `java` |
| `.c`, `.h` | C | `c` |
| `.cpp`, `.hpp` | C++ | `cpp` |
| `.cs` | C# | `csharp` |
| `.go` | Go | `go` |
| `.rb` | Ruby | `ruby` |
| `.php` | PHP | `php` |
| `.swift` | Swift | `swift` |
| `.kt` | Kotlin | `kotlin` |
| `.scala` | Scala | `scala` |
| `.r` | R | `r` |
| `.sh`, `.bash` | Shell | `bash` |
| `.sql` | SQL | `sql` |
| `.html`, `.htm` | HTML | `html` |
| `.css` | CSS | `css` |
| `.xml` | XML | `xml` |
| `.json` | JSON | `json` |
| `.yaml`, `.yml` | YAML | `yaml` |
| `.toml` | TOML | `toml` |
| `.md` | Markdown | `markdown` |

### Markdown Advantages

- **Human Readable**: Easy to read and understand without tools
- **Platform Support**: Works on GitHub, GitLab, docs sites, and editors
- **Lightweight**: Simple syntax, small file sizes
- **Version Control**: Diffs well in Git and other VCS
- **Search Friendly**: Text search works naturally
- **Printable**: Renders cleanly when printed or exported to PDF

### Markdown Use Cases

1. **Code Reviews**: Share with team members for review
2. **Documentation**: Include in project documentation
3. **GitHub Issues**: Paste directly into GitHub issues or PRs
4. **Wikis**: Add to project wikis or knowledge bases
5. **AI Assistants**: Optimal format for most LLM interactions
6. **Learning**: Study codebases in a readable format

## Directory Tree Inclusion

Both formats support optional directory tree inclusion, which provides structural context.

### Tree Structure Features

**Visual Hierarchy**: Uses Unicode box-drawing characters:
- `├──` for intermediate items
- `└──` for last items
- `│` for vertical connections
- `📁` for directories
- `📄` for files

**Depth Limiting**: Prevents infinite recursion with configurable maximum depth (default: 10 levels).

**Pattern Filtering**: Respects ignore patterns in tree generation.

**Sorting**: Directories appear first, then files, both alphabetically sorted.

### Example Tree Output

```
📁 my-project/
├── 📄 .gitignore
├── 📄 Cargo.toml
├── 📄 README.md
├── 📁 src/
│   ├── 📄 main.rs
│   ├── 📁 handlers/
│   │   ├── 📄 auth.rs
│   │   └── 📄 user.rs
│   └── 📁 utils/
│       ├── 📄 db.rs
│       └── 📄 validation.rs
└── 📁 tests/
    ├── 📄 integration.rs
    └── 📄 unit.rs
```

## Format Comparison

### File Size

**XML Format:**
- Larger due to markup overhead
- CDATA sections add verbosity
- Structured metadata increases size

**Markdown Format:**
- Smaller, more compact
- Minimal markup overhead
- Direct content representation

### Processing

**XML Format:**
```xml
<file path="src/main.rs">
  <![CDATA[
fn main() {
    println!("Hello!");
}
  ]]>
</file>
```

**Markdown Format:**
```markdown
### src/main.rs

```rust
fn main() {
    println!("Hello!");
}
```
```

### Readability

**XML**: Requires XML viewer or editor for optimal reading experience.

**Markdown**: Human-readable in any text editor, enhanced in Markdown viewers.

## Choosing the Right Format

### Use XML When:

- **API Integration**: Feeding data into analysis tools or services
- **Structured Processing**: Need to parse and process the output programmatically
- **Formal Documentation**: Creating formal technical documentation
- **Database Import**: Importing data into databases or content management systems
- **Tool Integration**: Working with XML-aware development tools

### Use Markdown When:

- **Human Review**: Sharing with colleagues for code review
- **Documentation**: Adding to project documentation or wikis
- **Version Control**: Committing readable documentation to repositories
- **Platform Sharing**: Posting on GitHub, GitLab, or documentation platforms
- **AI Interaction**: Working with LLMs that prefer readable text
- **Learning**: Studying codebases in a readable format

## Advanced Configuration

### XML Customization

```json
{
  "xml_format": {
    "include_metadata": true,
    "validate_structure": true,
    "pretty_print": true,
    "encoding": "UTF-8"
  }
}
```

### Markdown Customization

```json
{
  "markdown_format": {
    "include_toc": false,
    "code_fence_style": "```",
    "heading_style": "atx",
    "line_breaks": "lf"
  }
}
```

## Export Examples

### Full Project Export (XML)
Perfect for comprehensive analysis or archival:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<codebase>
  <directory_tree><!-- Full tree structure --></directory_tree>
  <files><!-- All selected files --></files>
</codebase>
```

### Feature-Specific Export (Markdown)
Ideal for focused code review:
```markdown
# Authentication Module Review

## Directory Structure
```
📁 src/auth/
├── 📄 mod.rs
├── 📄 login.rs
└── 📄 session.rs
```

## Files
<!-- Only auth-related files -->
```

### Documentation Export (Markdown)
Great for project documentation:
```markdown
# Project Structure Overview

## Directory Structure
<!-- Complete tree for navigation -->

## Core Files
<!-- Key implementation files only -->
```

---

For configuration details, see the [Configuration Guide](configuration.md). For feature-specific information, see the [Features Guide](features.md).