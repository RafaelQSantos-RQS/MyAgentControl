//! Golden-test helpers (cli-spec §8, master decision D8).
//!
//! Golden tests scaffold into a temp dir and diff against a reference
//! checkout of [`darrenhinde/OpenAgentsControl`](https://github.com/darrenhinde/OpenAgentsControl)
//! at tag `v0.7.1`, with volatile content normalized:
//! - `Updated: YYYY-MM-DD` dates (and any `YYYY-MM-DD` token) → `<date>`
//! - CRLF → LF
//! - trailing whitespace trimmed per line, single trailing newline
//!
//! Fixtures live under `tests/golden/` (task 05).

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Normalize volatile content for diffing (D8).
pub fn normalize(content: &str) -> String {
    // `lines()` already strips a trailing `\r`, so CRLF is handled implicitly.
    content
        .lines()
        .map(normalize_dates)
        .map(|line| line.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// Replace `YYYY-MM-DD` date tokens with `<date>` (handles the
/// `| Updated: 2026-02-15 |` pattern found in reference frontmatter).
fn normalize_dates(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::with_capacity(line.len());
    let mut i = 0;
    while i < chars.len() {
        let is_date = i + 10 <= chars.len()
            && chars[i..i + 4].iter().all(|c| c.is_ascii_digit())
            && chars[i + 4] == '-'
            && chars[i + 5..i + 7].iter().all(|c| c.is_ascii_digit())
            && chars[i + 7] == '-'
            && chars[i + 8..i + 10].iter().all(|c| c.is_ascii_digit());
        if is_date {
            out.push_str("<date>");
            i += 10;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

/// Recursively copy a directory tree (`src` dir or file → `dst`).
pub fn copy_tree(src: &Path, dst: &Path) -> io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            copy_tree(&entry.path(), &dst.join(entry.file_name()))?;
        }
    } else {
        fs::copy(src, dst)?;
    }
    Ok(())
}

/// All file paths under `root`, relative to it, sorted.
pub fn collect_relative_paths(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if root.is_dir()
        && let Ok(entries) = fs::read_dir(root)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.extend(collect_relative_paths(&path));
            } else if let Ok(rel) = path.strip_prefix(root) {
                out.push(rel.to_path_buf());
            }
        }
    }
    out.sort();
    out
}

/// Diff two trees after normalization.
///
/// Returns a list of human-readable differences; empty means clean.
/// Missing/extra files and content differences are reported per file.
pub fn diff_trees(expected: &Path, actual: &Path) -> Vec<String> {
    let exp_files = collect_relative_paths(expected);
    let act_files = collect_relative_paths(actual);

    let mut diffs = Vec::new();
    for rel in &exp_files {
        let exp_path = expected.join(rel);
        let act_path = actual.join(rel);
        if !act_path.exists() {
            diffs.push(format!("missing in actual: {}", rel.display()));
            continue;
        }
        if read_normalized(&exp_path) != read_normalized(&act_path) {
            diffs.push(format!("content differs: {}", rel.display()));
        }
    }
    for rel in &act_files {
        if !expected.join(rel).exists() {
            diffs.push(format!("extra in actual: {}", rel.display()));
        }
    }
    diffs
}

/// Read a file, normalizing text or hex-encoding raw bytes so binary and
/// non-UTF-8 files still diff deterministically (source-artifact parity,
/// AC-L8). Text files go through [`normalize`]; unreadable-as-UTF-8 files
/// compare byte-for-byte via a hex representation.
fn read_normalized(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(text) => normalize(&text),
        Err(_) => fs::read(path)
            .map(|bytes| hex_encode(&bytes))
            .unwrap_or_default(),
    }
}

/// `AB:CD:…` representation of raw bytes (no `0x` prefix).
fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 {
            out.push(':');
        }
        write!(&mut out, "{b:02X}").expect("write to String never fails");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_replaces_dates() {
        let input = "<!-- Context: core/navigation | Priority: critical | Version: 1.0 | Updated: 2026-02-15 -->";
        assert_eq!(
            normalize(input),
            "<!-- Context: core/navigation | Priority: critical | Version: 1.0 | Updated: <date> -->\n"
        );
    }

    #[test]
    fn normalize_handles_crlf_and_trailing_whitespace() {
        let input = "line one  \r\nline two\t\r\n";
        assert_eq!(normalize(input), "line one\nline two\n");
    }

    #[test]
    fn normalize_different_dates_compare_equal() {
        let a = "Updated: 2026-02-15";
        let b = "Updated: 2030-01-01";
        assert_ne!(a, b);
        assert_eq!(normalize(a), normalize(b));
    }

    #[test]
    fn copy_tree_copies_recursively() {
        let dir =
            std::env::temp_dir().join(format!("myagentcontrol-golden-unit-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let src = dir.join("src");
        fs::create_dir_all(src.join("nested")).unwrap();
        fs::write(src.join("a.md"), "a").unwrap();
        fs::write(src.join("nested/b.md"), "b").unwrap();
        let dst = dir.join("dst");
        copy_tree(&src, &dst).unwrap();
        assert!(dst.join("a.md").exists());
        assert!(dst.join("nested/b.md").exists());
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn diff_trees_clean_for_identical() {
        let dir =
            std::env::temp_dir().join(format!("myagentcontrol-golden-diff-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let a = dir.join("a");
        let b = dir.join("b");
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();
        fs::write(a.join("x.md"), "Hello").unwrap();
        fs::write(b.join("x.md"), "Hello").unwrap();
        assert!(diff_trees(&a, &b).is_empty());
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn diff_trees_flags_missing_and_extra() {
        let dir = std::env::temp_dir().join(format!(
            "myagentcontrol-golden-diff2-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        let a = dir.join("a");
        let b = dir.join("b");
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();
        fs::write(a.join("x.md"), "Hello").unwrap();
        fs::write(a.join("gone.md"), "gone").unwrap();
        fs::write(b.join("x.md"), "Hello").unwrap();
        fs::write(b.join("extra.md"), "extra").unwrap();
        let diffs = diff_trees(&a, &b);
        assert!(diffs.iter().any(|d| d.contains("missing")));
        assert!(diffs.iter().any(|d| d.contains("extra")));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn diff_trees_ignores_updated_date_drift() {
        let dir = std::env::temp_dir().join(format!(
            "myagentcontrol-golden-drift-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        let a = dir.join("a");
        let b = dir.join("b");
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();
        fs::write(a.join("nav.md"), "Updated: 2026-02-15").unwrap();
        fs::write(b.join("nav.md"), "Updated: 2030-01-01").unwrap();
        assert!(
            diff_trees(&a, &b).is_empty(),
            "date drift must normalize clean"
        );
        fs::remove_dir_all(&dir).ok();
    }
}
