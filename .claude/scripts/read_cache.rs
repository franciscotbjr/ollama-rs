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

    println!("🚀 Ready to continue where we left off!");
    println!("\n💡 Next Steps:");
    println!("  1. Review critical files (spec/definition.md, spec/api-analysis.md)");
    println!("  2. Check current implementation status in DEV_NOTES.md");
    println!("  3. Continue with Phase 1 implementation tasks");
}
