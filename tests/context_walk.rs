//! Empirical walk: run the CTX-1 frontmatter parser over every real context
//! file in the vendored `content/context/` tree (task phase1-context-01).
//!
//! The parser is **strict** per context-spec §3.3. The OAC v0.7.1 reference
//! tree contains a handful of files that legitimately deviate from that rule
//! (compatibility shims, YAML-frontmatter files, discovery docs) — they are
//! tracked in [`KNOWN_EXCEPTIONS`] below (user decision: strict parser +
//! documented allowlist). Any *other* file the parser rejects is a defect
//! (or new drift on a future refresh) and fails the test.
//!
//! Skipped when `content/` is absent (e.g. a partial checkout).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use myagentcontrol::context::frontmatter;
use myagentcontrol::core::golden::collect_relative_paths;

/// Documented deviations of the OAC v0.7.1 tree from context-spec §3.3.
/// Paths are relative to `content/context/`. If a refresh fixes a file,
/// remove its entry (the test fails if a listed file stops failing *and* if
/// an unlisted file starts failing, so the list stays honest).
const KNOWN_EXCEPTIONS: &[(&str, &str)] = &[
    (
        "core/context-system/standards/frontmatter.md",
        "prose doc, no frontmatter",
    ),
    (
        "core/context-system/standards/templates.md",
        "prose doc, no frontmatter",
    ),
    (
        "core/context-system/standards/typescript-coding.md",
        "prose doc, no frontmatter",
    ),
    (
        "core/standards/csharp.md",
        "YAML `---` frontmatter, not HTML comment",
    ),
    (
        "core/standards/csharp-project-structure.md",
        "YAML `---` frontmatter, not HTML comment",
    ),
    (
        "core/workflows/component-planning.md",
        "missing `Updated` field",
    ),
    (
        "core/workflows/lightweight-context-handoff-example.md",
        "`Priority: reference` outside spec set {critical, high, medium, low}",
    ),
    (
        "core/workflows/task-delegation.md",
        "compatibility-shim comment, no Context fields",
    ),
    ("index.md", "compatibility-shim comment, no Context fields"),
    (
        "openagents-repo/core-concepts/agents.md",
        "concept doc, no frontmatter",
    ),
    (
        "openagents-repo/core-concepts/categories.md",
        "concept doc, no frontmatter",
    ),
    (
        "openagents-repo/quality/registry-dependencies.md",
        "YAML `---` frontmatter, not HTML comment",
    ),
];

fn context_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content/context")
}

fn is_markdown(p: &Path) -> bool {
    p.extension().is_some_and(|e| e == "md")
}

#[test]
fn every_context_markdown_file_parses() {
    let root = context_dir();
    if !root.is_dir() {
        eprintln!("SKIP: content/context/ not present ({})", root.display());
        return;
    }

    let exceptions: HashMap<&str, &str> = KNOWN_EXCEPTIONS.iter().copied().collect();

    let mut parsed = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut known: Vec<String> = Vec::new();

    for rel in collect_relative_paths(&root) {
        if !is_markdown(&rel) {
            continue;
        }
        let rel_str = rel.to_string_lossy();
        let path = root.join(&rel);
        let content = std::fs::read_to_string(&path).expect("read context file");
        match frontmatter::parse(&path.display().to_string(), &content) {
            Ok(_) => parsed += 1,
            Err(errs) => match exceptions.get(rel_str.as_ref()) {
                Some(reason) => known.push(format!("{rel_str} (known: {reason})")),
                None => {
                    for e in &errs {
                        failures.push(format!("{rel_str} :: {e}"));
                    }
                }
            },
        }
    }

    // Every allowlist entry must actually exist and still fail — if upstream
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
            "unexpected frontmatter results under content/context/:\n  failures: {}\n  stale allowlist entries: {}",
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
        "OK: {parsed} markdown files parsed clean; {} known OAC deviations tolerated",
        known.len()
    );
}
