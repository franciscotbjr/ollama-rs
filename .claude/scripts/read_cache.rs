#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct Phase {
    version: String,
    focus: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectContext {
    project_name: String,
    version: String,
    repository: String,
    license: String,
    build_system: String,
    language: String,
    edition: String,
    workspace_crates: Vec<String>,
    total_crates: u32,
    critical_files: Vec<String>,
    spec_files: Vec<String>,
    #[serde(default)]
    impl_files: Vec<String>,
    session_count: u32,
    total_sessions: u32,
    created_at: String,
    last_session: String,
    project_path: String,
    build_status: String,
    cache_version: String,
    project_hash: String,
}

fn get_cache_dir() -> PathBuf {
    let home = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .expect("Could not find home directory");
    PathBuf::from(home).join(".claude").join("ollama-oxide")
}

fn get_project_hash() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let current_dir = env::current_dir()
        .expect("Could not get current directory")
        .to_string_lossy()
        .to_string();

    let mut hasher = DefaultHasher::new();
    current_dir.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:08x}", hash)
}

fn read_file_summary(file_path: &str) -> String {
    if let Ok(content) = fs::read_to_string(file_path) {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        // Extract first meaningful line (skip empty lines and comments)
        let first_line = lines.iter()
            .find(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .map(|s| s.trim())
            .unwrap_or("(empty file)");

        if total_lines > 50 {
            format!("{} lines - {}", total_lines, first_line)
        } else {
            format!("{} lines", total_lines)
        }
    } else {
        "(not readable)".to_string()
    }
}

/// Displays active blockers from BLOCKERS.md
fn display_blockers() {
    let blockers_path = "BLOCKERS.md";

    if let Ok(content) = fs::read_to_string(blockers_path) {
        let lines: Vec<&str> = content.lines().collect();

        // Find the "## Bloqueios Ativos" section
        let mut in_active_section = false;
        let mut active_blockers: Vec<&str> = Vec::new();

        for line in &lines {
            if line.contains("## Bloqueios Ativos") {
                in_active_section = true;
                continue;
            }
            if in_active_section && line.starts_with("## ") {
                // Reached next section, stop
                break;
            }
            if in_active_section && line.starts_with('|') && !line.contains("---") && !line.contains("Date") {
                active_blockers.push(line);
            }
        }

        if !active_blockers.is_empty() {
            println!("🚧 Active Blockers ({}):", active_blockers.len());

            for row in &active_blockers {
                // Parse table row: | Date | Type | Blocker | Impact | Reference |
                let cols: Vec<&str> = row.split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                if cols.len() >= 3 {
                    let blocker_type = cols.get(1).unwrap_or(&"");
                    let blocker_desc = cols.get(2).unwrap_or(&"");
                    println!("  ⚠️  [{}] {}", blocker_type, blocker_desc);
                }
            }
            println!();
        } else {
            println!("🚧 Active Blockers: None");
            println!();
        }
    }
    // If file doesn't exist, silently skip (blockers are optional)
}

/// Displays next steps (TODO items) from DEV_NOTES.md
fn display_next_steps() {
    let dev_notes_path = "DEV_NOTES.md";

    if let Ok(content) = fs::read_to_string(dev_notes_path) {
        let lines: Vec<&str> = content.lines().collect();

        // Find the "### TODO" section
        let mut in_todo_section = false;
        let mut todo_items: Vec<&str> = Vec::new();

        for line in &lines {
            if line.contains("### TODO") {
                in_todo_section = true;
                continue;
            }
            if in_todo_section && line.starts_with("##") {
                // Reached next section, stop
                break;
            }
            if in_todo_section && line.trim().starts_with("- [ ]") {
                // Extract the task description (remove "- [ ] " prefix)
                let task = line.trim().trim_start_matches("- [ ]").trim();
                todo_items.push(task);
            }
        }

        if !todo_items.is_empty() {
            let show_count = std::cmp::min(5, todo_items.len());
            println!("📌 Next Steps ({} pending, showing first {}):", todo_items.len(), show_count);

            for (i, task) in todo_items.iter().take(show_count).enumerate() {
                println!("  {}. {}", i + 1, task);
            }
            println!();
        }
    }
    // If file doesn't exist or no TODOs, silently skip
}

/// Displays recent decisions from DECISIONS.md
fn display_decisions() {
    let decisions_path = "DECISIONS.md";

    if let Ok(content) = fs::read_to_string(decisions_path) {
        let lines: Vec<&str> = content.lines().collect();

        // Find table rows (lines starting with |)
        let table_rows: Vec<&str> = lines.iter()
            .filter(|line| line.starts_with('|') && !line.contains("---"))
            .copied()
            .collect();

        if table_rows.len() > 1 {
            // Skip header row, get last 5 decisions
            let decisions: Vec<&str> = table_rows.iter()
                .skip(1) // Skip header
                .copied()
                .collect();

            let recent_count = std::cmp::min(5, decisions.len());
            let recent_decisions: Vec<&str> = decisions.iter()
                .rev()
                .take(recent_count)
                .rev()
                .copied()
                .collect();

            println!("📜 Recent Decisions ({} total, showing last {}):", decisions.len(), recent_count);

            for row in recent_decisions {
                // Parse table row: | Date | Decision | Rationale | Reference |
                let cols: Vec<&str> = row.split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                if cols.len() >= 2 {
                    let date = cols.first().unwrap_or(&"");
                    let decision = cols.get(1).unwrap_or(&"");
                    println!("  [{:}] {}", date, decision);
                }
            }
            println!();
        }
    } else {
        println!("📜 Decisions: No DECISIONS.md found (consider creating one)");
        println!();
    }
}

fn main() {
    let cache_dir = get_cache_dir();
    let project_hash = get_project_hash();
    let cache_file = cache_dir.join(format!("project_{}.cache", project_hash));

    println!("🔍 Loading previous conversation context...\n");

    if !cache_file.exists() {
        println!("❌ No previous conversation found");
        println!("   Cache file: {}", cache_file.display());
        println!("\n💡 Tip: Run /save-session-cache to create a cache for this project");
        std::process::exit(1);
    }

    // Read and parse cache
    let content = fs::read_to_string(&cache_file)
        .expect("Failed to read cache file");

    let context: ProjectContext = serde_json::from_str(&content)
        .expect("Failed to parse cache file");

    // Display cache summary
    println!("✅ Context loaded successfully!\n");
    println!("📊 Project Information:");
    println!("  Project: {} v{}", context.project_name, context.version);
    println!("  Language: {} (edition {})", context.language, context.edition);
    println!("  Repository: {}", context.repository);
    println!("  License: {}", context.license);
    println!();

    println!("🏗️  Architecture:");
    println!("  Type: Single crate");
    println!("  Build System: {}", context.build_system);
    println!("  Modules: primitives, http, conveniences");
    println!("  Features: default (http + primitives), conveniences (optional)");
    println!();

    println!("📁 Critical Files ({} tracked):", context.critical_files.len());
    for file in &context.critical_files {
        let summary = read_file_summary(file);
        println!("  ✓ {} ({})", file, summary);
    }
    println!();

    println!("📄 API Specifications ({} endpoints):", context.spec_files.len());
    // Group by type
    let mut simple = Vec::new();
    let mut medium = Vec::new();
    let mut complex = Vec::new();

    for spec in &context.spec_files {
        let filename = spec.split('/').last().unwrap_or(spec);
        if filename.contains("version") {
            simple.push(filename);
        } else if filename.contains("tags") || filename.contains("ps") ||
                  filename.contains("copy") || filename.contains("delete") {
            simple.push(filename);
        } else if filename.contains("show") || filename.contains("embed") {
            medium.push(filename);
        } else {
            complex.push(filename);
        }
    }

    if !simple.is_empty() {
        println!("  Simple ({}):", simple.len());
        for spec in simple {
            println!("    - {}", spec);
        }
    }
    if !medium.is_empty() {
        println!("  Medium ({}):", medium.len());
        for spec in medium {
            println!("    - {}", spec);
        }
    }
    if !complex.is_empty() {
        println!("  Complex ({}):", complex.len());
        for spec in complex {
            println!("    - {}", spec);
        }
    }
    println!();

    if !context.impl_files.is_empty() {
        println!("📝 Implementation Plans ({} files):", context.impl_files.len());
        for impl_file in &context.impl_files {
            let filename = impl_file.split('/').last().unwrap_or(impl_file);
            let summary = read_file_summary(impl_file);
            println!("  ✓ {} ({})", filename, summary);
        }
        println!();
    }

    println!("📈 Session Information:");
    println!("  Session: #{}", context.session_count);
    println!("  Total Sessions: {}", context.total_sessions);
    println!("  Created: {}", context.created_at);
    println!("  Last Session: {}", context.last_session);
    println!();

    println!("🔨 Build Status:");
    println!("  Status: {}", context.build_status);
    println!();

    println!("📍 Project Location:");
    println!("  Path: {}", context.project_path);
    println!("  Hash: {}", context.project_hash);
    println!();

    // Read current phase from definition.md
    if let Ok(def_content) = fs::read_to_string("spec/definition.md") {
        println!("📋 Current Implementation Phase:");

        // Extract current phase info
        if def_content.contains("Phase 1 (v0.1.0)") {
            println!("  Phase: 1 (v0.1.0) - Foundation + HTTP Core");
            println!("  Status: In Progress");

            // Extract what's in progress
            if def_content.contains("**In Progress:**") {
                println!("  Focus:");
                if def_content.contains("Simple endpoints (1): version") {
                    println!("    - Implementing GET /api/version endpoint");
                }
                if def_content.contains("Primitives crate structure") {
                    println!("    - Setting up primitives crate structure");
                }
                if def_content.contains("HTTP client implementation") {
                    println!("    - Building HTTP client in http-core");
                }
                if def_content.contains("Error type hierarchy") {
                    println!("    - Creating error handling system");
                }
            }
        }
        println!();
    }

    // Read and display decisions from DECISIONS.md
    display_decisions();

    // Read and display active blockers from BLOCKERS.md
    display_blockers();

    // Read and display next steps from DEV_NOTES.md TODO section
    display_next_steps();

    println!("🚀 Ready to continue where we left off!");
}
