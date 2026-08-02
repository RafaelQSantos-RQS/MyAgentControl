//! Empirical walk: every `SKILL.md` under `content/skills/` must validate
//! against the official OpenCode skill contract (skills-spec v0.1.1 note,
//! <https://opencode.ai/docs/skills/>): YAML frontmatter with `name` +
//! `description`, `name` matching the folder and the
//! `^[a-z0-9]+(-[a-z0-9]+)*$` rule.
//!
//! There is **no allowlist here**: the vendored tree is the ground truth and
//! every skill must pass. If a skill fails (or a new one is added that fails),
//! this test fails — a stricter gate than the context walk, because OpenCode
//! will not even load an invalid SKILL.md (skills-spec AC-S1).
//!
//! This mirrors the parity harness: content is pinned to OAC v0.7.1 and any
//! adopted-PR skill must satisfy the same rules (CR-001 point 5).

use std::fs;
use std::path::PathBuf;

use myagentcontrol::core::golden::collect_relative_paths;
use myagentcontrol::skills::frontmatter::parse;

fn skills_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content/skills")
}

#[test]
fn every_skill_md_validates_against_opencode_rules() {
    let dir = skills_dir();
    assert!(
        dir.is_dir(),
        "content/skills/ must exist (task phase1-context-05), got {}",
        dir.display()
    );

    let mut seen = 0usize;
    let mut failures: Vec<(String, Vec<String>)> = Vec::new();

    for rel in collect_relative_paths(&dir) {
        if rel.file_name().is_none_or(|n| n != "SKILL.md") {
            continue;
        }
        seen += 1;
        let path = dir.join(&rel);
        let path_str = path.display().to_string();
        let content = fs::read_to_string(&path).unwrap_or_default();
        match parse(&path_str, &content) {
            Ok(_) => {}
            Err(errs) => {
                failures.push((path_str, errs.iter().map(|e| e.to_string()).collect()));
            }
        }
    }

    // The OAC tree currently ships exactly 4 skills (context7, context-manager,
    // smart-router-skill, task-management). A change in count is worth
    // surfacing explicitly rather than silently.
    assert_eq!(
        seen, 4,
        "expected exactly 4 SKILL.md files under content/skills/, found {seen}"
    );

    assert!(
        failures.is_empty(),
        "{} SKILL.md file(s) fail the OpenCode contract:\n{:#?}",
        failures.len(),
        failures
    );
}
