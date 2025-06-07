# fsPrompt Integration Guide

This guide covers integrating fsPrompt with LLMs, development tools, CI/CD pipelines, and team workflows.

## LLM Integration

### OpenAI ChatGPT Integration

**Basic Workflow**:
1. Generate prompt in fsPrompt
2. Copy with **Ctrl+C**
3. Paste into ChatGPT with context

**Optimized Prompting Pattern**:
```
I'm working on [specific task/problem]. Here's my codebase:

[fsPrompt XML/Markdown output]

Please [specific request with clear requirements].

Focus on:
- [Specific aspect 1]  
- [Specific aspect 2]
- [Specific aspect 3]

Constraints:
- [Any limitations or requirements]
```

**Token Management for ChatGPT**:
- **GPT-3.5-turbo**: 4k context → Use Green/Low token counts only
- **GPT-4**: 8k context → Green/Low to Yellow/Medium safe
- **GPT-4-turbo**: 128k context → Can handle Red/High token counts
- **GPT-4o**: 128k context → Excellent for large codebases

### Claude Integration

**Anthropic Claude Strengths**:
- Excellent at understanding large codebases (100k+ tokens)
- Strong at architectural analysis
- Good at following coding standards

**Optimized Claude Workflow**:
```
I need help with [specific development task]. Here's the relevant codebase structure and files:

[fsPrompt Markdown output - Claude prefers Markdown]

Task: [Be very specific about what you want]

Please:
1. [First specific request]
2. [Second specific request]  
3. [Third specific request]

Requirements:
- [Coding standards to follow]
- [Architecture constraints]
- [Performance considerations]
```

### GitHub Copilot Integration

**Using fsPrompt with Copilot**:
1. Generate focused prompts for specific features
2. Save output as `context.md` in project root
3. Open in VS Code alongside working files
4. Copilot will use visible context for better suggestions

**Context File Strategy**:
```bash
# Create context files for Copilot
1. Generate relevant code in fsPrompt
2. Save as: docs/copilot-context-[feature].md
3. Keep open in VS Code tabs during development
4. Update as feature evolves
```

### Local LLM Integration

**Ollama Integration**:
```bash
# Generate prompt with fsPrompt
# Save to file: context.xml

# Use with Ollama
ollama run codellama:13b < context.xml

# Or interactively
ollama run codellama:13b
> Here's my codebase for [task]: [paste fsPrompt output]
```

**LM Studio Integration**:
1. Generate prompt in fsPrompt
2. Save as .md file
3. Load in LM Studio chat interface
4. Use for local code analysis

## Development Tool Integration

### VS Code Integration

**Workflow 1: Context-Aware Development**
```bash
# Setup
1. Generate relevant codebase context with fsPrompt
2. Save as `project-context.md` in workspace
3. Pin tab in VS Code
4. Reference during development
```

**Workflow 2: Code Review Preparation**
```bash
# Before creating PR
1. Use git to identify changed files: git diff --name-only main
2. Select these files + related context in fsPrompt  
3. Generate Markdown output
4. Save as PR description template
5. Review with team
```

**Custom VS Code Task**:
```json
// .vscode/tasks.json
{
    "version": "2.0.0", 
    "tasks": [
        {
            "label": "Generate Context",
            "type": "shell",
            "command": "cargo",
            "args": ["run", "--release"],
            "options": {
                "cwd": "/path/to/fsprompt"
            },
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "new"
            }
        }
    ]
}
```

### JetBrains IDE Integration

**IntelliJ IDEA Workflow**:
1. Use "Find in Files" to identify related code
2. Copy file paths from search results
3. Select corresponding files in fsPrompt
4. Generate context for refactoring assistance

**External Tool Configuration**:
```
Name: fsPrompt
Program: /path/to/fsprompt/target/release/fsprompt
Arguments: (none)
Working Directory: $ProjectFileDir$
```

### Vim/Neovim Integration

**Command Line Workflow**:
```bash
# Terminal-based workflow
1. Run fsPrompt in separate terminal
2. Generate and save context to file
3. Use :read command to include in Vim
:read ~/context.md
```

**File-Based Integration**:
```vim
" Add to .vimrc
command! LoadContext read ~/fsprompt-context.md
command! EditContext !open -a fsPrompt
```

## CI/CD Integration

### GitHub Actions Integration

**Automated Documentation Generation**:
```yaml
# .github/workflows/docs.yml
name: Generate Documentation Context

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  generate-context:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Build fsPrompt
        run: |
          git clone https://github.com/patrikpersson/codext-rs.git
          cd codext-rs
          cargo build --release
          
      - name: Generate Project Context
        run: |
          # This would need fsPrompt CLI mode (future feature)
          echo "Context generation would happen here"
          
      - name: Upload Context
        uses: actions/upload-artifact@v4
        with:
          name: project-context
          path: project-context.md
```

**PR Context Generation**:
```yaml
# .github/workflows/pr-context.yml
name: PR Context Generator

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  context:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
          
      - name: Get Changed Files
        id: changes
        run: |
          echo "files=$(git diff --name-only origin/main...HEAD | tr '\n' ',')" >> $GITHUB_OUTPUT
          
      - name: Comment PR with Context Hint
        uses: actions/github-script@v7
        with:
          script: |
            const files = '${{ steps.changes.outputs.files }}'.split(',').filter(f => f);
            if (files.length > 0) {
              github.rest.issues.createComment({
                issue_number: context.issue.number,
                owner: context.repo.owner,
                repo: context.repo.repo,
                body: `## 🔍 Code Review Context\n\nChanged files:\n${files.map(f => `- \`${f}\``).join('\n')}\n\n💡 **Tip**: Use these files in fsPrompt to generate context for LLM-assisted review.`
              });
            }
```

### GitLab CI Integration

**Pipeline Stage for Context Generation**:
```yaml
# .gitlab-ci.yml
stages:
  - build
  - test
  - documentation

generate-context:
  stage: documentation
  image: rust:latest
  script:
    - git clone https://github.com/patrikpersson/codext-rs.git
    - cd codext-rs && cargo build --release
    - echo "Generate context for key files"
    # Future: CLI mode for automated context generation
  artifacts:
    paths:
      - documentation/
    expire_in: 1 week
  only:
    - main
    - merge_requests
```

### Jenkins Integration

**Jenkinsfile Example**:
```groovy
pipeline {
    agent any
    
    stages {
        stage('Generate Context') {
            steps {
                script {
                    // Build fsPrompt if not cached
                    sh '''
                        if [ ! -f "tools/fsprompt" ]; then
                            git clone https://github.com/patrikpersson/codext-rs.git tools/fsprompt-src
                            cd tools/fsprompt-src
                            cargo build --release
                            cp target/release/fsprompt ../fsprompt
                        fi
                    '''
                    
                    // Generate context for documentation
                    sh 'echo "Context generation step"'
                }
            }
        }
    }
    
    post {
        success {
            archiveArtifacts artifacts: 'docs/*.md', fingerprint: true
        }
    }
}
```

## Team Workflow Integration

### Code Review Workflows

**Pre-Review Context Generation**:
```bash
# Reviewer Workflow
1. Check out PR branch
2. Identify changed files: git diff --name-only main
3. Use fsPrompt to select changed files + context
4. Generate review context document
5. Share with team or use for LLM-assisted review
```

**Review Template Generation**:
```markdown
# Code Review: [PR Title]

## Context
Generated with fsPrompt on [date]

## Changed Files
[List from fsPrompt selection]

## Architecture Impact
[Based on directory tree analysis]

## Review Focus Areas
- [ ] Business logic correctness
- [ ] Error handling
- [ ] Performance implications
- [ ] Security considerations
- [ ] Test coverage

## Generated Context
[fsPrompt XML/Markdown output]
```

### Documentation Workflows

**Architecture Documentation**:
```bash
# Quarterly architecture review
1. Generate comprehensive codebase overview
2. Focus on core interfaces and contracts
3. Create versioned architecture snapshots
4. Use for onboarding and training
```

**Onboarding Documentation**:
```bash
# New team member resources
1. Create "getting started" context with key files
2. Generate feature-specific mini-tours
3. Include configuration and setup files
4. Provide LLM-ready learning materials
```

### Knowledge Sharing

**Team Knowledge Base**:
```
/docs/context-snapshots/
├── architecture-2025-01-07.md      # Full system overview
├── auth-system-2025-01-07.md       # Authentication deep-dive
├── api-layer-2025-01-07.md         # API documentation
└── frontend-components-2025-01-07.md # UI component reference
```

**Best Practices Sharing**:
```bash
# Weekly tech talks
1. Use fsPrompt to extract examples of good patterns
2. Generate context showing before/after refactoring
3. Create educational prompts for team learning
4. Document decision rationales with code context
```

## Development Environment Setup

### Project-Specific Configuration

**Per-Project Ignore Patterns**:
```bash
# Frontend projects
node_modules,dist,build,.next,.nuxt,coverage,*.log

# Backend projects  
target,node_modules,coverage,*.log,dist,build

# Full-stack projects
node_modules,target,dist,build,coverage,*.log,.next,.nuxt

# Data science projects
*.csv,*.parquet,*.h5,__pycache__,data/raw,models/*.pkl,*.ipynb_checkpoints
```

**Team Configuration Sharing**:
```json
// .fsprompt/team-config.json (future feature)
{
  "defaultIgnorePatterns": ["node_modules", "coverage", "dist"],
  "projectStructure": {
    "frontend": ["src/components", "src/pages", "src/hooks"],
    "backend": ["src/controllers", "src/models", "src/services"],
    "shared": ["src/types", "src/utils", "config"]
  },
  "tokenTargets": {
    "review": 8000,
    "architecture": 15000,
    "feature": 10000
  }
}
```

### Automation Scripts

**Bash Script for Common Workflows**:
```bash
#!/bin/bash
# generate-context.sh

case $1 in
  "review")
    echo "Generating code review context..."
    # Open fsPrompt with review-specific settings
    ;;
  "feature") 
    echo "Generating feature context for: $2"
    # Open fsPrompt focused on feature directory
    ;;
  "architecture")
    echo "Generating architecture overview..."
    # Open fsPrompt with broad selection
    ;;
  *)
    echo "Usage: generate-context.sh [review|feature|architecture] [feature-name]"
    ;;
esac
```

**PowerShell Script for Windows**:
```powershell
# Generate-Context.ps1
param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("review", "feature", "architecture")]
    [string]$Type,
    
    [string]$FeatureName
)

switch ($Type) {
    "review" {
        Write-Host "Opening fsPrompt for code review..."
        & "C:\tools\fsprompt\fsprompt.exe"
    }
    "feature" {
        Write-Host "Generating context for feature: $FeatureName"
        # Future: CLI parameters for automated selection
    }
    "architecture" {
        Write-Host "Generating architecture overview..."
        # Future: Preset configurations
    }
}
```

## Advanced Integration Patterns

### Multi-Repository Workflows

**Microservice Context Generation**:
```bash
# Generate context across multiple repositories
1. Clone all related services locally
2. Use fsPrompt on each service separately  
3. Combine outputs with custom tooling
4. Create unified architecture documentation
```

**Monorepo Package Analysis**:
```bash
# For large monorepos
1. Generate package-level contexts
2. Create cross-package dependency maps
3. Use for impact analysis of changes
4. Maintain package-specific documentation
```

### LLM-Assisted Development Workflows

**TDD with LLM Assistance**:
```bash
1. Write test descriptions
2. Use fsPrompt to generate context with existing test patterns
3. Ask LLM to generate test implementations
4. Use fsPrompt again with failing tests for implementation help
```

**Refactoring Workflows**:
```bash
1. Identify refactoring target with fsPrompt
2. Generate context showing current implementation
3. Use LLM to suggest refactoring approach
4. Generate context with related files for impact analysis
5. Implement with LLM assistance
```

**API Design Workflows**:
```bash
1. Generate context showing existing API patterns
2. Use LLM to design new API endpoints
3. Generate context with related models and validation
4. Implement with consistent patterns
```

## Security and Privacy Considerations

### Sensitive Data Handling

**Safe Ignore Patterns**:
```bash
# Always exclude sensitive files
*.env,*.key,*.pem,*.p12,secrets/*,credentials/*,config/prod/*
```

**Team Guidelines**:
1. Never include production configuration files
2. Exclude API keys and credentials from selections
3. Review generated output before sharing with external LLMs
4. Use local LLMs for sensitive codebases

### Data Governance

**Enterprise Considerations**:
- Establish policies for LLM use with proprietary code
- Create approved LLM services list
- Implement review processes for external LLM usage
- Maintain audit trails of context generation

---

*Next: [Customization Guide](customization.md) for themes, configuration, and advanced customization options*