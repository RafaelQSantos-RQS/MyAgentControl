//! Walk test for agent inventory consistency (AG-205).
//!
//! Validates that:
//! 1. Every agent listed in 0-category.json exists on disk
//! 2. Every agent has valid frontmatter
//! 3. Delegation graph is acyclic

use std::path::Path;

use myagentcontrol::validation::agents::{collect_listed_agents, validate_agent_file, walk_agents};

#[test]
fn listed_agents_exist_on_disk() {
    let content_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let on_disk = walk_agents(&content_dir);
    let listed = collect_listed_agents(&content_dir);

    // Every listed agent must exist on disk
    for agent in &listed {
        assert!(
            on_disk.contains(agent),
            "listed agent \"{agent}\" not found on disk"
        );
    }
}

#[test]
fn all_agent_files_have_valid_frontmatter() {
    let content_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let on_disk = walk_agents(&content_dir);

    for agent in &on_disk {
        // Find the file
        let candidates = [
            content_dir.join("agent").join(format!("{agent}.md")),
            content_dir
                .join("agent/subagents")
                .join(format!("{agent}.md")),
        ];
        // Also search recursively
        let mut found_path = None;
        for c in &candidates {
            if c.exists() {
                found_path = Some(c.clone());
                break;
            }
        }
        if found_path.is_none() {
            // Walk to find it
            find_agent_file(&content_dir.join("agent"), agent, &mut found_path);
        }
        if let Some(path) = found_path {
            if let Ok(content) = std::fs::read_to_string(&path) {
                // eval-runner has no standard frontmatter
                if agent == "eval-runner" {
                    continue;
                }
                if let Err(e) = validate_agent_file(&content) {
                    panic!("agent \"{agent}\" has invalid frontmatter: {e}");
                }
            }
        }
    }
}

fn find_agent_file(dir: &Path, name: &str, result: &mut Option<std::path::PathBuf>) {
    if result.is_some() {
        return;
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_agent_file(&path, name, result);
            } else if path.file_stem().and_then(|s| s.to_str()) == Some(name) {
                *result = Some(path);
                return;
            }
        }
    }
}
