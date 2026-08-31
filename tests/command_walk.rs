//! Walk test for command inventory consistency (CMD-403).
//!
//! Validates that all command files have valid frontmatter.

use std::path::Path;

use myagentcontrol::validation::commands::{validate_command_file, walk_commands};

#[test]
fn command_inventory_consistency() {
    let content_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let commands = walk_commands(&content_dir);

    assert!(
        !commands.is_empty(),
        "no commands found in content/command/"
    );

    let mut errors = Vec::new();
    for cmd in &commands {
        let cmd_file = content_dir.join("command").join(format!("{cmd}.md"));
        if let Ok(content) = std::fs::read_to_string(&cmd_file) {
            if let Err(e) = validate_command_file(&content) {
                errors.push(format!("{cmd}: {e}"));
            }
        }
    }

    if !errors.is_empty() {
        eprintln!(
            "Command validation issues ({}):\n{}",
            errors.len(),
            errors.join("\n")
        );
    }
    // All commands must have valid frontmatter per CMD-401
    assert!(
        errors.is_empty(),
        "{} commands have invalid frontmatter",
        errors.len()
    );
}
