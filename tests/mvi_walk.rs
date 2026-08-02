//! Empirical walk: run the MVI validator (CTX-2) over every real
//! context file in the vendored `content/context/` tree (task
//! phase1-context-02).
//!
//! Semantics (context-spec §3, user decision):
//! - Files ≥ 200 lines are **reference docs**: exempt from the MVI formula.
//! - Discovery files (`navigation.md`, `index.md`, `README.md`,
//!   `CODEBASE_STANDARDS.md`) are exempt from the reference-section rule.
//! - Concept cards (< 200 lines, non-discovery) MUST include a reference
//!   section (any of: Codebase References, Related Context, Related Files,
//!   Related, References, Reference, Quick Reference).
//!
//! The OAC v0.7.1 reference tree contains a handful of concept cards that
//! legitimately lack such a section; they are tracked in [`KNOWN_EXCEPTIONS`]
//! below (measured with exact validator semantics on 2026-08-02). Any *other*
//! file the validator rejects is a defect (or new drift) and fails the test.
//!
//! Skipped when `content/` is absent (e.g. a partial checkout).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use myagentcontrol::context::mvi;
use myagentcontrol::core::golden::collect_relative_paths;

/// Documented deviations of the OAC v0.7.1 tree from the reference-section
/// rule. Paths are relative to `content/context/`. If a refresh fixes a
/// file, remove its entry (the test fails if a listed file stops failing *and*
/// if an unlisted file starts failing, so the list stays honest).
const KNOWN_EXCEPTIONS: &[(&str, &str)] = &[
    (
        "core/context-system/CHANGELOG.md",
        "changelog, no reference section",
    ),
    ("core/system/context-paths.md", "no reference section"),
    (
        "core/workflows/component-planning.md",
        "no reference section",
    ),
    ("core/workflows/task-delegation.md", "no reference section"),
    (
        "development/ai/mastra-ai/concepts/agents-tools.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/concepts/core.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/concepts/evaluations.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/concepts/storage.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/concepts/workflows.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/errors/mastra-errors.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/examples/workflow-example.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/guides/modular-building.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/guides/testing.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/guides/workflow-step-structure.md",
        "no reference section",
    ),
    (
        "development/ai/mastra-ai/lookup/mastra-config.md",
        "no reference section",
    ),
    (
        "openagents-repo/guides/building-cli-compact.md",
        "no reference section",
    ),
    (
        "openagents-repo/plugins/context/architecture/lifecycle.md",
        "no reference section",
    ),
    (
        "openagents-repo/plugins/context/architecture/overview.md",
        "no reference section",
    ),
    (
        "openagents-repo/plugins/context/capabilities/agents.md",
        "no reference section",
    ),
    (
        "openagents-repo/plugins/context/capabilities/events.md",
        "no reference section",
    ),
    (
        "openagents-repo/plugins/context/capabilities/tools.md",
        "no reference section",
    ),
    (
        "openagents-repo/plugins/context/context-overview.md",
        "no reference section",
    ),
    (
        "openagents-repo/plugins/context/reference/best-practices.md",
        "no reference section",
    ),
    (
        "openagents-repo/quick-start.md",
        "quick-start, no reference section",
    ),
    ("project/project-context.md", "no reference section"),
];

fn context_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content/context")
}

fn is_markdown(p: &Path) -> bool {
    p.extension().is_some_and(|e| e == "md")
}

#[test]
fn every_context_concept_card_has_a_reference_section_or_is_known() {
    let root = context_dir();
    if !root.is_dir() {
        eprintln!("SKIP: content/context/ not present ({})", root.display());
        return;
    }

    let exceptions: HashMap<&str, &str> = KNOWN_EXCEPTIONS.iter().copied().collect();

    let mut passed = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut known: Vec<String> = Vec::new();

    for rel in collect_relative_paths(&root) {
        if !is_markdown(&rel) {
            continue;
        }
        let rel_str = rel.to_string_lossy();
        let path = root.join(&rel);
        let content = std::fs::read_to_string(&path).expect("read context file");
        let errs = mvi::validate(&rel_str, &content);
        if errs.is_empty() {
            passed += 1;
            continue;
        }
        match exceptions.get(rel_str.as_ref()) {
            Some(reason) => known.push(format!("{rel_str} (known: {reason})")),
            None => {
                for e in &errs {
                    failures.push(format!("{rel_str} :: {e}"));
                }
            }
        }
    }

    // Every allowlist entry must actually exist and still fail; if upstream
    // fixed one, the list is stale and the test tells us to remove it.
    let mut stale: Vec<String> = Vec::new();
    for (rel, reason) in KNOWN_EXCEPTIONS {
        let p = root.join(rel);
        if !p.is_file() {
            stale.push(format!("{rel} (missing from tree, reason: {reason})"));
        } else if !known.iter().any(|k| k.starts_with(&format!("{rel} ("))) {
            stale.push(format!("{rel} (no longer failing, reason: {reason})"));
        }
    }

    if !failures.is_empty() || !stale.is_empty() {
        panic!(
            "unexpected MVI results under content/context/:\n  failures: {}\n  stale allowlist entries: {}",
            if failures.is_empty() {
                "(none)".to_string()
            } else {
                failures.join("\n    ")
            },
            if stale.is_empty() {
                "(none)".to_string()
            } else {
                stale.join("\n    ")
            }
        );
    }

    eprintln!(
        "OK: {passed} files pass MVI (incl. reference docs & discovery files); {} known concept-card deviations tolerated",
        known.len()
    );
}
