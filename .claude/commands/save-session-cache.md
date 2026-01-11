# Cache Project Context

I'll save the current session context for the ollama-rs project by reading critical project files and creating a cache for future session continuity.

## What Gets Cached

The cache script will automatically gather information from:

### Critical Project Files
- **spec/definition.md** - Complete project definition and implementation strategy
- **spec/api-analysis.md** - Detailed analysis of all 12 API endpoints
- **spec/primitives/*.yaml** - Individual API endpoint specifications (12 files)
- **DEV_NOTES.md** - Development notes and architectural decisions
- **CHANGELOG.md** - Version history and changes
- **README.md** - Project overview and quick start
- **CONTRIBUTING.md** - Development guidelines and standards
- **Cargo.toml** - Project configuration and workspace structure

### Extracted Information
From these files, the script extracts:
- Project name, version, repository, license (from Cargo.toml)
- Workspace crates and structure (from Cargo.toml)
- Build system and tooling information
- List of all critical documentation files
- List of all API specification files
- Build status (basic validation)
- Session tracking (count, timestamps)

### NOT Duplicated
The following information is **NOT** cached but read from files when needed:
- Implementation strategy and phases (read from definition.md)
- Current phase status and progress (read from definition.md)
- API endpoint details (read from api-analysis.md)
- Development notes and decisions (read from DEV_NOTES.md)
- Task lists and TODOs (read from relevant files)

This ensures the cache stays lean and source files remain the single source of truth.

## Usage

<function_calls>
<invoke name="Bash">
<parameter name="command">rust-script .claude/scripts/save_cache.rs</parameter>
<parameter name="description">Save session context to cache</parameter>
</invoke>