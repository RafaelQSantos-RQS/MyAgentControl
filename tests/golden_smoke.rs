//! Golden smoke test (task 05, cli-spec §8 / D8).
//!
//! Copies the committed fixture subtree (`tests/golden/reference/`, a small
//! slice of the OAC reference repo at tag v0.7.1) into a temp dir and asserts
//! the normalized diff is clean. Also proves D8 normalization: `Updated:`
//! date drift between the sides must still diff clean.

use std::fs;
use std::path::{Path, PathBuf};

use myagentcontrol::core::golden::{copy_tree, diff_trees};

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden/reference")
}

fn temp_dst(tag: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "myagentcontrol-golden-smoke-{tag}-{}",
        std::process::id()
    ))
}

fn clean_temp(path: &Path) {
    let _ = fs::remove_dir_all(path);
}

#[test]
fn smoke_golden_diff_is_clean_for_committed_fixture() {
    let src = fixture_dir();
    assert!(src.join("navigation.md").exists(), "fixture must exist");
    let dst = temp_dst("clean");
    clean_temp(&dst);
    copy_tree(&src, &dst).expect("copy fixture subtree");

    let diffs = diff_trees(&src, &dst);
    assert!(diffs.is_empty(), "expected clean diff, got: {diffs:#?}");

    clean_temp(&dst);
}

#[test]
fn smoke_golden_normalizes_updated_date_drift() {
    let src = fixture_dir();
    let dst = temp_dst("drift");
    clean_temp(&dst);
    copy_tree(&src, &dst).expect("copy fixture subtree");

    // Simulate drift: rewrite one file's Updated date in the copy.
    let nav = dst.join("navigation.md");
    let text = fs::read_to_string(&nav).expect("read copied file");
    let drifted = text.replace("2026-02-15", "2030-11-30");
    assert_ne!(text, drifted, "drift must actually change content");
    fs::write(&nav, drifted).expect("write drifted file");

    let diffs = diff_trees(&src, &dst);
    assert!(
        diffs.is_empty(),
        "D8 normalization must ignore Updated-date drift, got: {diffs:#?}"
    );

    clean_temp(&dst);
}

#[test]
fn smoke_golden_flags_real_content_change() {
    let src = fixture_dir();
    let dst = temp_dst("realchange");
    clean_temp(&dst);
    copy_tree(&src, &dst).expect("copy fixture subtree");

    // A real (non-date) change must be flagged by the diff.
    let guide = dst.join("context-guide.md");
    let text = fs::read_to_string(&guide).expect("read copied file");
    fs::write(&guide, format!("{text}\n\n# Injected change\n")).expect("write");

    let diffs = diff_trees(&src, &dst);
    assert!(
        diffs.iter().any(|d| d.contains("content differs")),
        "real content change must be flagged, got: {diffs:#?}"
    );

    clean_temp(&dst);
}
