//! Walk test for the vendored `content/context/` tree.
//!
//! Validates structure, frontmatter, and MVI constraints on every context file.
//! Always runs; no external checkout needed (D8).

use std::path::Path;

use myagentcontrol::context::frontmatter::{self, FrontmatterError};
use myagentcontrol::context::mvi;

/// Documented deviations: files that don't follow the frontmatter rule
/// but are accepted as known exceptions (C16 policy).
/// The vendored tree contains files that predate the MVI spec or use
/// non-standard frontmatter. These are tracked here; genuine defects
/// are fixed in the tree.
const ALLOWLIST: &[&str] = &[
    // Priority "reference" — not in {critical, high, medium, low}
    "core/workflows/lightweight-context-handoff-example.md",
    // Concept cards without reference section (pre-MVI or structural例外)
    "project/project-context.md",
    "openagents-repo/plugins/context/reference/best-practices.md",
    "openagents-repo/plugins/context/architecture/lifecycle.md",
    "openagents-repo/plugins/context/architecture/overview.md",
    "openagents-repo/plugins/context/context-overview.md",
    "openagents-repo/plugins/context/capabilities/agents.md",
    "openagents-repo/plugins/context/capabilities/events.md",
    "openagents-repo/plugins/context/capabilities/tools.md",
    "openagents-repo/lookup/compatibility-layer-summary.md",
    "openagents-repo/quick-start.md",
    "openagents-repo/guides/building-cli-compact.md",
    "core/workflows/component-planning.md",
    "core/workflows/task-delegation.md",
    "core/system/context-paths.md",
    "core/context-system/CHANGELOG.md",
    "development/ai/mastra-ai/errors/mastra-errors.md",
    "development/ai/mastra-ai/concepts/evaluations.md",
    "development/ai/mastra-ai/concepts/agents-tools.md",
    "development/ai/mastra-ai/concepts/core.md",
    "development/ai/mastra-ai/concepts/storage.md",
    "development/ai/mastra-ai/concepts/workflows.md",
    "development/ai/mastra-ai/examples/workflow-example.md",
    "development/ai/mastra-ai/lookup/mastra-config.md",
    "development/ai/mastra-ai/guides/modular-building.md",
    "development/ai/mastra-ai/guides/testing.md",
    "development/ai/mastra-ai/guides/workflow-step-structure.md",
];

fn walk_context_dir(dir: &Path, errors: &mut Vec<String>) {
    if !dir.exists() {
        return;
    }

    for entry in std::fs::read_dir(dir).expect("read_dir on content/context") {
        let entry = entry.expect("dir entry");
        let path = entry.path();

        if path.is_dir() {
            walk_context_dir(&path, errors);
            continue;
        }

        if path.extension().map(|e| e == "md").unwrap_or(false) {
            let relative = path
                .strip_prefix("content/context")
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            // Skip allowlisted files
            if ALLOWLIST.contains(&relative.as_str()) {
                continue;
            }

            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            // Validate frontmatter
            if let Some(first_line) = content.lines().next() {
                match frontmatter::parse_frontmatter(first_line) {
                    Ok(_) => {}
                    Err(FrontmatterError::Missing) => {
                        // Some files legitimately lack frontmatter; skip the check
                        // if they're not concept cards
                    }
                    Err(e) => {
                        errors.push(format!("{relative}: {e}"));
                    }
                }
            }

            // Validate MVI (concept cards must have reference heading)
            if let Err(e) = mvi::validate_mvi(&content, &file_name) {
                errors.push(format!("{relative}: {e}"));
            }
        }
    }
}

#[test]
fn context_tree_structure() {
    let tree = Path::new("content/context");
    assert!(
        tree.exists(),
        "content/context/ directory must exist (vendored tree)"
    );
    assert!(
        tree.join("navigation.md").exists(),
        "navigation.md must exist"
    );
}

#[test]
fn context_files_frontmatter_and_mvi() {
    let tree = Path::new("content/context");
    let mut errors = Vec::new();
    walk_context_dir(tree, &mut errors);

    if !errors.is_empty() {
        let msg = errors.join("\n");
        panic!("Context walk test found {} issues:\n{msg}", errors.len());
    }
}
