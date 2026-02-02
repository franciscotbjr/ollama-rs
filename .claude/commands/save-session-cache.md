# Cache Project Context

I'll save the current session context for the ollama-oxide project by reading critical project files and creating a cache for future session continuity.

## Session Context (CRITICAL)

After running the cache script, you MUST update the `session_context` in the cache file with:

### `task` attribute
- Must contain the current/last task being developed in the project
- Example: "Rename feature 'create' to 'model' - refactoring complete, pending git commit"

### `summary` attribute
- **MUST have content equivalent to the result of `/compact` command execution**
- Must follow the exact structure with these 9 sections:
  1. **Primary Request and Intent** - What the user requested and invoked
  2. **Key Technical Concepts** - Technical patterns and concepts involved
  3. **Files and Code Sections** - All files modified/created with descriptions
  4. **Errors and fixes** - Any errors encountered and how they were resolved
  5. **Problem Solving** - How the task was approached and completed
  6. **All user messages** - List of user commands/messages in the session
  7. **Pending Tasks** - What remains to be done (if any)
  8. **Current Work** - Current state of work and verification results
  9. **Optional Next Step** - Logical next action to take

**IMPORTANT**: If `session_context.task` or `session_context.summary` are empty after running the script, you MUST manually update them in the cache file before completing this command.

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