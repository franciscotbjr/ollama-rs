#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! chrono = "0.4"
//! toml = "0.8"
//! ```

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
struct SessionContext {
    task: String,
    summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectContext {
    // Project identification (from Cargo.toml)
    project_name: String,
    version: String,
    repository: String,
    license: String,

    // Build system
    build_system: String,
    language: String,
    edition: String,

    // Workspace structure (from Cargo.toml)
    workspace_crates: Vec<String>,
    total_crates: u32,

    // Critical files inventory
    critical_files: Vec<String>,
    spec_files: Vec<String>,
    impl_files: Vec<String>,

    // Session tracking
    session_count: u32,
    total_sessions: u32,
    created_at: String,
    last_session: String,
    project_path: String,

    // Build status (check if compilable)
    build_status: String,

    // Metadata
    cache_version: String,
    project_hash: String,

    // Session context (what was done)
    #[serde(default)]
    session_context: SessionContext,
}

#[derive(Deserialize)]
struct CargoToml {
    package: Option<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: Option<String>,
    version: Option<String>,
    repository: Option<String>,
    license: Option<String>,
    edition: Option<String>,
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

fn load_existing_cache(cache_file: &PathBuf) -> Option<ProjectContext> {
    if cache_file.exists() {
        if let Ok(content) = fs::read_to_string(cache_file) {
            if let Ok(context) = serde_json::from_str::<ProjectContext>(&content) {
                println!("Found existing cache (Session #{})", context.total_sessions + 1);
                return Some(context);
            }
        }
    }
    println!("Creating new cache file");
    None
}

fn read_cargo_toml() -> Result<CargoToml, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("Cargo.toml")?;
    Ok(toml::from_str(&content)?)
}

fn find_critical_files() -> Vec<String> {
    let mut files = Vec::new();
    let critical = [
        "spec/definition.md",
        "spec/api-analysis.md",
        "DEV_NOTES.md",
        "CHANGELOG.md",
        "README.md",
        "CONTRIBUTING.md",
        "ARCHITECTURE.md",
        "DECISIONS.md",
        "BLOCKERS.md",
        "Cargo.toml",
    ];

    for file in critical {
        if PathBuf::from(file).exists() {
            files.push(file.to_string());
        }
    }
    files
}

fn find_spec_files() -> Vec<String> {
    let mut files = Vec::new();
    let spec_dir = PathBuf::from("spec/primitives");

    if spec_dir.exists() {
        if let Ok(entries) = fs::read_dir(&spec_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".yaml") {
                        files.push(format!("spec/primitives/{}", name));
                    }
                }
            }
        }
    }
    files.sort();
    files
}

fn find_impl_files() -> Vec<String> {
    let mut files = Vec::new();
    let impl_dir = PathBuf::from("impl");

    if impl_dir.exists() {
        if let Ok(entries) = fs::read_dir(&impl_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".md") {
                        files.push(format!("impl/{}", name));
                    }
                }
            }
        }
    }
    files.sort();
    files
}

fn check_build_status() -> String {
    // Check if we can at least parse Cargo.toml
    if PathBuf::from("Cargo.toml").exists() {
        match read_cargo_toml() {
            Ok(_) => "Cargo.toml valid".to_string(),
            Err(_) => "Cargo.toml has errors".to_string(),
        }
    } else {
        "No Cargo.toml found".to_string()
    }
}

fn parse_cli_args() -> SessionContext {
    let args: Vec<String> = env::args().collect();
    let mut task = String::new();
    let mut summary = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--task" => {
                if i + 1 < args.len() {
                    task = args[i + 1].clone();
                    i += 1;
                }
            }
            "--summary" => {
                if i + 1 < args.len() {
                    summary = args[i + 1].clone();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    SessionContext { task, summary }
}

fn main() {
    let cache_dir = get_cache_dir();
    let project_hash = get_project_hash();
    let cache_file = cache_dir.join(format!("project_{}.cache", project_hash));

    // Create cache directory if needed
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
        println!("Created cache directory: {}", cache_dir.display());
    }

    // Load existing cache or start fresh
    let existing_sessions = load_existing_cache(&cache_file)
        .map(|ctx| ctx.total_sessions + 1)
        .unwrap_or(1);

    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let current_dir = env::current_dir()
        .expect("Could not get current directory")
        .to_string_lossy()
        .to_string();

    // Read project info from Cargo.toml
    let cargo_toml = read_cargo_toml().expect("Failed to read Cargo.toml");

    let project_name = cargo_toml.package.as_ref()
        .and_then(|p| p.name.clone())
        .unwrap_or_else(|| "ollama-oxide".to_string());

    let version = cargo_toml.package.as_ref()
        .and_then(|p| p.version.clone())
        .unwrap_or_else(|| "0.1.0".to_string());

    let repository = cargo_toml.package.as_ref()
        .and_then(|p| p.repository.clone())
        .unwrap_or_else(|| "https://github.com/franciscotbjr/ollama-oxide".to_string());

    let license = cargo_toml.package.as_ref()
        .and_then(|p| p.license.clone())
        .unwrap_or_else(|| "MIT".to_string());

    let edition = cargo_toml.package.as_ref()
        .and_then(|p| p.edition.clone())
        .unwrap_or_else(|| "2024".to_string());

    // Single crate - no workspace
    let workspace_crates = vec![project_name.clone()];
    let total_crates = 1;

    // Find critical, spec, and impl files
    let critical_files = find_critical_files();
    let spec_files = find_spec_files();
    let impl_files = find_impl_files();

    // Parse CLI args for session context
    let session_context = parse_cli_args();

    // Build context object
    let context = ProjectContext {
        // Project identification
        project_name: project_name.clone(),
        version: version.clone(),
        repository,
        license,

        // Build system
        build_system: "Cargo (workspace)".to_string(),
        language: "Rust".to_string(),
        edition,

        // Workspace structure
        workspace_crates: workspace_crates.clone(),
        total_crates,

        // Critical files inventory
        critical_files,
        spec_files: spec_files.clone(),
        impl_files: impl_files.clone(),

        // Session tracking
        session_count: existing_sessions,
        total_sessions: existing_sessions,
        created_at: current_time.clone(),
        last_session: current_time,
        project_path: current_dir,

        // Build status
        build_status: check_build_status(),

        // Metadata
        cache_version: "1.1".to_string(),
        project_hash: project_hash.clone(),

        // Session context
        session_context,
    };

    // Save to cache file
    let json = serde_json::to_string_pretty(&context).expect("Failed to serialize context");
    fs::write(&cache_file, json).expect("Failed to write cache file");

    println!("\n✅ Context saved successfully!\n");
    println!("📊 Cache Summary:");
    println!("  Location: {}", cache_file.display());
    println!("  Project: {} v{}", context.project_name, context.version);
    println!("  Session: #{}", context.session_count);
    println!("  Architecture: Single crate with {} modules", context.total_crates);
    println!("  API Specs: {} endpoints", spec_files.len());
    println!("  Impl Plans: {} files", impl_files.len());
    println!("  Build: {}", context.build_status);
    println!("\n📁 Critical Files Tracked:");
    for file in &context.critical_files {
        println!("  ✓ {}", file);
    }

    // Display session context if provided
    if !context.session_context.task.is_empty() || !context.session_context.summary.is_empty() {
        println!("\n📝 Session Context Saved:");
        if !context.session_context.task.is_empty() {
            println!("  Task: {}", context.session_context.task);
        }
        if !context.session_context.summary.is_empty() {
            println!("  Summary: {}", context.session_context.summary);
        }
    }

    println!("\nReady to continue in next session with /continue-session");
}
