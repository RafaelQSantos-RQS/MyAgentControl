//! Tree helpers (master decision D8, cli-spec §8).
//!
//! These helpers power the **always-on walk tests** (`tests/*_walk.rs`) that
//! validate the real `content/` tree — there is **no** external reference
//! checkout and no golden diff against upstream (constitution C6).
//!
//! - [`collect_relative_paths`] lists every file under a tree (walk tests).
//! - [`copy_tree`] recursively copies a tree (`init`: `content/` → `.opencode/`).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Recursively copy a directory tree (`src` dir or file → `dst`).
///
/// Used by `init` to copy the vendored `content/` tree to the target
/// `.opencode/` directory (CLI-1).
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
///
/// Paths are relative to `root` at **every** depth (prefixes accumulate during
/// recursion) — not to the subdirectory where each file was found.
pub fn collect_relative_paths(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect_relative_paths_rec(root, root, &mut out);
    out.sort();
    out
}

fn collect_relative_paths_rec(root: &Path, base: &Path, out: &mut Vec<PathBuf>) {
    if root.is_dir()
        && let Ok(entries) = fs::read_dir(root)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_relative_paths_rec(&path, base, out);
            } else if let Ok(rel) = path.strip_prefix(base) {
                out.push(rel.to_path_buf());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_relative_paths_accumulates_prefixes() {
        let dir = std::env::temp_dir().join(format!(
            "myagentcontrol-golden-relpaths-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("a/b/c")).unwrap();
        fs::write(dir.join("top.md"), "top").unwrap();
        fs::write(dir.join("a/b/c/deep.md"), "deep").unwrap();
        let rels = collect_relative_paths(&dir);
        assert_eq!(
            rels,
            vec![PathBuf::from("a/b/c/deep.md"), PathBuf::from("top.md")]
        );
        fs::remove_dir_all(&dir).ok();
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
}
