# fsPrompt Advanced Patterns & Complex Workflows

This guide covers sophisticated techniques for power users working with large codebases, complex projects, and specialized use cases.

## Advanced Selection Strategies

### Pattern 1: Layer-by-Layer Architecture Analysis

**Use Case**: Understanding complex software architecture  
**Complexity**: High  
**Time**: 15-30 minutes  

**Strategy**: Break the codebase into logical layers and analyze each separately.

1. **Infrastructure Layer Prompt**
   ```
   Select: config/, middleware/, database/, auth/
   Include: package.json, docker-compose.yml, .env.example
   Ignore: node_modules,dist,coverage,*.log
   Token Target: ~10k (Medium)
   ```

2. **Core Business Logic Prompt**
   ```
   Select: src/models/, src/services/, src/controllers/
   Include: Key interfaces and type definitions
   Ignore: tests,node_modules,dist
   Token Target: ~15k (Medium)
   ```

3. **UI Layer Prompt**
   ```
   Select: src/components/, src/pages/, src/hooks/
   Include: src/styles/, package.json (for UI deps)
   Ignore: node_modules,dist,build,*.test.js
   Token Target: ~12k (Medium)
   ```

**LLM Interaction Pattern**:
```
1. "Here's the infrastructure layer of my application..."
2. "Now here's the business logic layer. How does it connect to the infrastructure?"
3. "Finally, here's the UI layer. Analyze the complete data flow."
```

### Pattern 2: Feature Development Workflow

**Use Case**: Implementing a new feature with LLM assistance  
**Complexity**: Medium  
**Time**: 10-20 minutes per iteration  

**Phase 1: Discovery**
```bash
# Find all files related to similar features
Search: "auth" (if building new auth feature)
Select: All auth-related files
Include: Core types and interfaces
Purpose: Understand existing patterns
```

**Phase 2: Planning**
```bash
# Show architectural foundation
Select: src/types/, src/interfaces/, core configuration files
Include: Directory tree for context
Purpose: Plan feature architecture
```

**Phase 3: Implementation**
```bash
# Show work-in-progress with context
Select: New feature files + related existing files
Include: Test files and documentation
Purpose: Get implementation help
```

**Phase 4: Integration**
```bash
# Show feature + integration points
Select: Feature files + entry points + routing + tests
Include: Configuration files
Purpose: Ensure proper integration
```

### Pattern 3: Debugging Complex Issues

**Use Case**: Tracking down bugs across multiple files  
**Complexity**: High  
**Time**: Variable  

**Step 1: Error Context Collection**
```bash
# Gather all files mentioned in stack trace
Select: Files from stack trace (top to bottom)
Include: Configuration files that might affect behavior
Add: Related test files
Token Target: Keep under 8k for focused analysis
```

**Step 2: Data Flow Analysis**
```bash
# Trace data flow through the system
Select: Input → Processing → Output files
Include: Type definitions and interfaces
Add: Validation and transformation logic
Purpose: Understand data mutations
```

**Step 3: State Management Investigation**
```bash
# Focus on state-related files
Select: State management, stores, reducers
Include: Actions and middleware
Add: Components that read/write state
Purpose: Identify state inconsistencies
```

## Large Repository Strategies

### Strategy 1: Microservice Analysis

**Challenge**: Understanding a large microservice architecture  
**Solution**: Service-by-service analysis

```bash
# Service Discovery
1. Generate directory tree only (no files)
   - Include tree: ✓
   - Select: Nothing (just see structure)
   - Purpose: Map service boundaries

# Individual Service Analysis
2. For each service:
   - Select: service-specific src/ directory
   - Include: service config files
   - Add: shared types/interfaces
   - Ignore: node_modules,dist,coverage
```

**Token Budget Management**:
- Gateway service: ~8k tokens
- Auth service: ~6k tokens  
- Core business services: ~10k tokens each
- Shared utilities: ~4k tokens

### Strategy 2: Monorepo Navigation

**Challenge**: Working with large monorepos (e.g., Nx, Lerna)  
**Solution**: Package-focused selection

```bash
# Package Discovery
1. Include directory tree with ignore patterns:
   Ignore: node_modules,dist,coverage,*.log,packages/*/node_modules

# Individual Package Analysis  
2. Per-package prompts:
   Select: packages/[package-name]/src/
   Include: packages/[package-name]/package.json
   Add: shared/common dependencies
   Include: Workspace root configuration
```

**Cross-Package Dependency Analysis**:
```bash
# Dependency Mapping
Select: All package.json files in monorepo
Include: Workspace configuration files
Add: TypeScript config files
Purpose: Understand inter-package dependencies
```

### Strategy 3: Legacy Code Migration

**Use Case**: Understanding and modernizing legacy codebases  
**Approach**: Incremental analysis with historical context

**Phase 1: Archaeological Survey**
```bash
# Get the lay of the land
Include: Directory tree only
Ignore: node_modules,vendor,build,dist
Purpose: Understand overall structure and age indicators
```

**Phase 2: Core Logic Extraction**
```bash
# Find the business logic
Select: Core business files (avoid UI/framework code)
Include: Configuration files
Add: Database schema files
Purpose: Understand what the system actually does
```

**Phase 3: Dependency Analysis**
```bash
# Understand external dependencies
Select: package.json, Gemfile, requirements.txt, etc.
Include: Lock files
Add: Configuration files
Purpose: Assess modernization effort
```

**Phase 4: Targeted Modernization**
```bash
# Focus on specific subsystem
Select: One logical module at a time
Include: Related tests (if they exist)
Add: Modern equivalent examples
Purpose: Plan incremental updates
```

## Specialized Use Cases

### Scientific/Research Code Analysis

**Challenge**: Understanding research code with minimal documentation  
**Technique**: Data flow and algorithm focus

```bash
# Data Processing Pipeline
Select: Input processing → Analysis → Output generation
Include: Configuration files with parameters
Add: Any README or documentation files
Ignore: Raw data files, generated outputs
Token Focus: Algorithm implementation details
```

**Academic Code Review**:
```bash
# Reproducibility Check
Select: Main analysis scripts
Include: Environment setup (requirements.txt, environment.yml)
Add: Configuration files
Include: Any validation or test scripts
Purpose: Verify computational reproducibility
```

### Open Source Contribution Workflow

**Challenge**: Understanding unfamiliar open source projects  
**Approach**: Contribution-focused exploration

**Step 1: Project Understanding**
```bash
# Get oriented
Select: README.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md
Include: Directory tree
Add: Core configuration files
Purpose: Understand project goals and structure
```

**Step 2: Issue-Specific Analysis**
```bash
# Focus on specific issue/PR
Select: Files mentioned in issue
Include: Related test files
Add: Similar functionality (for patterns)
Purpose: Understand requirements and context
```

**Step 3: Testing Strategy**
```bash
# Understand testing patterns
Select: Test files related to your changes
Include: Test configuration
Add: CI/CD configuration files
Purpose: Write appropriate tests
```

### Performance Optimization Workflows

**Challenge**: Identifying and fixing performance bottlenecks  
**Method**: Measurement-driven analysis

**Phase 1: Hot Path Identification**
```bash
# Focus on critical paths
Select: Performance-critical files (from profiling)
Include: Configuration affecting performance
Add: Benchmark/test files
Ignore: Non-critical features
Purpose: Understand current performance characteristics
```

**Phase 2: Optimization Research**
```bash
# Study optimization opportunities
Select: Current implementation + similar efficient code
Include: Performance-related dependencies
Add: Relevant benchmarks
Purpose: Plan optimization approach
```

## Advanced Configuration Techniques

### Dynamic Ignore Patterns

**Project-Type Specific Patterns**:

```bash
# Node.js Projects
node_modules,dist,build,coverage,*.log,.nyc_output,npm-debug.log*

# Rust Projects  
target,Cargo.lock,*.pdb,*.exe,*.so,*.dylib

# Python Projects
__pycache__,*.pyc,*.pyo,venv,env,.pytest_cache,dist,build

# Java Projects
target,*.class,*.jar,*.war,*.ear,*.iml

# Unity/Game Development
Library,Temp,Logs,UserSettings,*.tmp,*.meta

# Web Development
node_modules,dist,build,.next,.nuxt,coverage,*.log

# Data Science
*.csv,*.parquet,*.h5,__pycache__,data/raw,models/*.pkl
```

### Context-Aware Selection

**Backend API Focus**:
```bash
Select: controllers,models,services,middleware
Include: API documentation (OpenAPI/Swagger)
Add: Database schema and migrations
Ignore: Frontend assets,public,static
```

**Frontend Component Focus**:
```bash
Select: components,pages,hooks,context
Include: Package.json (for dependencies)
Add: Style files,configuration
Ignore: node_modules,dist,backend
```

### Multi-Session Workflows

**Session 1: Architecture Review**
```bash
Purpose: Understand overall structure
Files: Core architecture + configuration
Save As: "architecture-review-YYYY-MM-DD.md"
```

**Session 2: Feature Implementation**
```bash
Purpose: Implement specific feature
Files: Feature area + related utilities + tests
Save As: "feature-auth-implementation-YYYY-MM-DD.md"
```

**Session 3: Integration & Testing**
```bash
Purpose: Ensure feature works end-to-end
Files: Feature + integration points + test suite
Save As: "integration-testing-YYYY-MM-DD.md"
```

## Performance Optimization for Large Codebases

### Memory Management

**Large Repository Strategies**:
1. **Use XML format** (lower memory footprint)
2. **Process in chunks** - Don't select entire large repos at once
3. **Restart fsPrompt** after processing very large selections
4. **Use aggressive ignore patterns** to exclude unnecessary files

**Token Budget Planning**:
```bash
# Conservative approach for large repos
Session 1: Core (≤8k tokens)
Session 2: Feature Area 1 (≤8k tokens)  
Session 3: Feature Area 2 (≤8k tokens)
Session 4: Integration (≤8k tokens)

# Rather than trying to fit everything in one 32k+ prompt
```

### Efficient Navigation

**Keyboard-First Workflow**:
- **Ctrl+K**: Jump to files quickly
- **Ctrl+Z/Shift+Z**: Experiment with selections
- **Space**: Toggle current item
- **Ctrl+G**: Generate immediately
- **Ctrl+C**: Copy and continue

**Search-Driven Selection**:
```bash
# Instead of manual tree navigation
1. Search for file patterns: "service" 
2. Select relevant results
3. Search for "test" to add test files
4. Search for "config" to add configuration
```

## Integration with Development Workflow

### Git Integration Patterns

**Branch-Based Analysis**:
```bash
# Analyze changes in feature branch
$ git diff main..feature-branch --name-only
# Use output to guide file selection in fsPrompt
```

**Commit-Based Review**:
```bash
# Review specific commit
$ git show --name-only [commit-hash]
# Select these files + context in fsPrompt
```

### IDE Integration Workflow

**VS Code Integration**:
1. Use VS Code's file explorer to identify relevant files
2. Copy file paths from VS Code
3. Use fsPrompt search to find and select these files
4. Generate prompt for LLM assistance
5. Apply LLM suggestions back in VS Code

**JetBrains Integration**:
1. Use "Find in Files" to identify related code
2. Note file paths from search results
3. Select corresponding files in fsPrompt
4. Use generated prompts for refactoring assistance

## Anti-Patterns to Avoid

### Selection Anti-Patterns

❌ **Selecting Everything**
- Creates massive, unfocused prompts
- Hits token limits
- Confuses LLMs with too much context

❌ **Ignoring Token Counts**
- Red token counts (32k+) often perform poorly
- LLMs can't effectively process massive context

❌ **No Clear Purpose**
- Random file selection without goal
- Include irrelevant files "just in case"

### Workflow Anti-Patterns

❌ **Single-Session Everything**
- Trying to understand entire codebase in one prompt
- Better to break into logical sessions

❌ **No Context Preservation**
- Not saving prompts for future reference
- Losing track of what was already analyzed

❌ **Ignoring Ignore Patterns**
- Including build artifacts and dependencies
- Wasting tokens on generated code

### Performance Anti-Patterns

❌ **No Memory Management**
- Not restarting after processing large repos
- Running multiple large analysis sessions consecutively

❌ **Inefficient Navigation**
- Manual scrolling instead of search
- Not using keyboard shortcuts

## Advanced Tips & Tricks

### Pro Tips

1. **Save Templates**: Create saved ignore pattern sets for different project types
2. **Token Budgeting**: Allocate token budgets before selection (e.g., 60% code, 20% tests, 20% config)
3. **Iterative Refinement**: Start with broad selection, then narrow based on LLM feedback
4. **Context Switching**: Use different formats (XML vs Markdown) for different purposes
5. **Session Logging**: Keep notes on what worked well for different types of analysis

### Expert Workflows

**The Spiral Approach**: Start with high-level architecture, spiral down to specific implementation details across multiple sessions.

**The Cross-Section Method**: Select files from different layers of the stack in one prompt to understand vertical data flow.

**The Diff-Driven Analysis**: Use git diffs to identify changed files, then include minimal necessary context for understanding changes.

---

*Next: [Integration Guide](integration.md) for using fsPrompt with LLMs, CI/CD, and development tools*