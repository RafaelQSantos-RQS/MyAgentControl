//! `status` command (registry-spec REG-8, cli-spec CLI-10): read the
//! `<install_dir>/.mac/manifest.json` and diff SHA256 hashes against disk.
//! Read-only — reports three file states without touching the filesystem
//! beyond reading.

use std::fs;
use std::path::Path;

use crate::install::installer::{Manifest, sha256_hex};
use crate::install::{InstallError, Result};

/// Path of the manifest inside the install directory.
pub const MANIFEST_REL_PATH: &str = ".mac/manifest.json";

/// Run `status` for the given install directory: read the manifest, diff
/// against disk, print the grouped report. Returns 0 when pristine, 1 when
/// any divergence exists (CLI-7 deterministic output).
pub fn run(install_dir: &str) -> Result<i32> {
    let manifest = read_manifest(Path::new(install_dir))?;
    let diffs = diff(&manifest, Path::new(install_dir))?;
    match render_status(&diffs) {
        Some(report) => {
            print!("{report}");
            Ok(1)
        }
        None => {
            println!("No modifications");
            Ok(0)
        }
    }
}

/// A managed file whose disk state differs from the manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffFile {
    /// Path relative to the install directory.
    pub rel: String,
    /// How the file diverges from the manifest.
    pub state: FileState,
}

/// The divergence between the manifest record and the on-disk file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileState {
    /// Disk hash differs from the manifest hash (user edited the file).
    Modified,
    /// The file was expected but is missing on disk.
    Removed,
    /// The file exists on disk but is not recorded in the manifest.
    Added,
}

/// Parse the manifest at `<install_dir>/.mac/manifest.json`.
pub fn read_manifest(install_dir: &Path) -> Result<Manifest> {
    let path = install_dir.join(MANIFEST_REL_PATH);
    let text = fs::read_to_string(&path).map_err(|e| {
        InstallError::Install(format!("cannot read manifest {}: {e}", path.display()))
    })?;
    serde_json::from_str(&text)
        .map_err(|e| InstallError::Install(format!("invalid manifest {}: {e}", path.display())))
}

/// Diff the manifest against the install directory. Every recorded file is
/// classified `Modified` (hash differs) or `Removed` (missing); every file
/// under `install_dir` that is not recorded (and not part of `.mac/`) is
/// classified `Added`.
pub fn diff(manifest: &Manifest, install_dir: &Path) -> Result<Vec<DiffFile>> {
    let mut diffs = Vec::new();
    for (rel, entry) in &manifest.files {
        let path = install_dir.join(rel);
        if !path.exists() {
            diffs.push(DiffFile {
                rel: rel.clone(),
                state: FileState::Removed,
            });
            continue;
        }
        let actual = sha256_hex(&path)
            .map_err(|e| InstallError::Install(format!("{}: {e}", path.display())))?;
        if actual != entry.sha256 {
            diffs.push(DiffFile {
                rel: rel.clone(),
                state: FileState::Modified,
            });
        }
    }
    for disk_file in files_on_disk(install_dir)? {
        if !manifest.files.contains_key(&disk_file) {
            diffs.push(DiffFile {
                rel: disk_file,
                state: FileState::Added,
            });
        }
    }
    // Deterministic ordering (CLI-7): stable by path, then state.
    diffs.sort_by(|a, b| {
        a.rel
            .cmp(&b.rel)
            .then_with(|| (a.state as u8).cmp(&(b.state as u8)))
    });
    Ok(diffs)
}

/// Every regular file under `install_dir`, relative to it, excluding the
/// `.mac/` metadata directory.
fn files_on_disk(install_dir: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    walk(install_dir, install_dir, &mut files)?;
    Ok(files)
}

/// Recursively collect regular files under `dir` (paths relative to
/// `root`), skipping `.mac/`.
fn walk(root: &Path, dir: &Path, files: &mut Vec<String>) -> Result<()> {
    let entries = fs::read_dir(dir)
        .map_err(|e| InstallError::Install(format!("cannot read {}: {e}", dir.display())))?;
    for entry in entries {
        let entry = entry
            .map_err(|e| InstallError::Install(format!("cannot read {}: {e}", dir.display())))?;
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();
        if rel == ".mac" {
            continue;
        }
        if entry
            .file_type()
            .map_err(|e| InstallError::Install(format!("cannot stat {}: {e}", path.display())))?
            .is_dir()
        {
            walk(root, &path, files)?;
        } else {
            files.push(rel);
        }
    }
    Ok(())
}

/// Render the diff as deterministic, grouped status lines (CLI-7). Returns
/// `None` when the tree is pristine.
pub fn render_status(diffs: &[DiffFile]) -> Option<String> {
    if diffs.is_empty() {
        return None;
    }
    let mut out = String::new();
    for group in ["modified", "removed", "added"] {
        let matching: Vec<&DiffFile> = diffs.iter().filter(|d| label(d.state) == group).collect();
        if matching.is_empty() {
            continue;
        }
        out.push_str(&format!("{} ({})\n", group, matching.len()));
        for file in matching {
            out.push_str(&format!("  {}\n", file.rel));
        }
    }
    Some(out)
}

/// Human-readable label for a file state.
fn label(state: FileState) -> &'static str {
    match state {
        FileState::Modified => "modified",
        FileState::Removed => "removed",
        FileState::Added => "added",
    }
}

/// Convenience for tests: build a `ManifestEntry` quickly.
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::io::Write;

    use crate::install::installer::ManifestEntry;

    fn entry(sha: &str) -> ManifestEntry {
        ManifestEntry {
            r#type: "agent".to_string(),
            installed_at: "2026-08-03T00:00:00Z".to_string(),
            sha256: sha.to_string(),
        }
    }

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "myagentcontrol-status-{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn sha_of(text: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        hex::encode(Sha256::digest(text))
    }

    fn write_tree(dir: &std::path::Path, rel: &str, text: &[u8]) {
        let path = dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).expect("mkdir");
        let mut f = fs::File::create(&path).expect("create");
        f.write_all(text).expect("write");
    }

    #[test]
    fn diff_pristine_is_empty() {
        let dir = temp_dir("pristine");
        write_tree(&dir, "agent/x.md", b"data");
        let manifest = Manifest {
            mac_version: "0.0.2".to_string(),
            files: BTreeMap::from([("agent/x.md".to_string(), entry(&sha_of(b"data")))]),
        };
        let diffs = diff(&manifest, &dir).expect("diff");
        assert!(diffs.is_empty());
    }

    #[test]
    fn diff_flags_modified_file() {
        let dir = temp_dir("modified");
        write_tree(&dir, "agent/x.md", b"changed");
        let manifest = Manifest {
            mac_version: "0.0.2".to_string(),
            files: BTreeMap::from([("agent/x.md".to_string(), entry(&sha_of(b"original")))]),
        };
        let diffs = diff(&manifest, &dir).expect("diff");
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].rel, "agent/x.md");
        assert_eq!(diffs[0].state, FileState::Modified);
    }

    #[test]
    fn diff_flags_removed_file() {
        let dir = temp_dir("removed");
        let manifest = Manifest {
            mac_version: "0.0.2".to_string(),
            files: BTreeMap::from([("agent/x.md".to_string(), entry(&sha_of(b"data")))]),
        };
        let diffs = diff(&manifest, &dir).expect("diff");
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].state, FileState::Removed);
    }

    #[test]
    fn diff_flags_added_file() {
        let dir = temp_dir("added");
        write_tree(&dir, "agent/x.md", b"data");
        write_tree(&dir, "user/manual.md", b"mine");
        let manifest = Manifest {
            mac_version: "0.0.2".to_string(),
            files: BTreeMap::from([("agent/x.md".to_string(), entry(&sha_of(b"data")))]),
        };
        let diffs = diff(&manifest, &dir).expect("diff");
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].rel, "user/manual.md");
        assert_eq!(diffs[0].state, FileState::Added);
    }

    #[test]
    fn diff_skips_mac_directory() {
        let dir = temp_dir("skip-mac");
        write_tree(&dir, "agent/x.md", b"data");
        write_tree(&dir, ".mac/manifest.json", b"{}");
        let manifest = Manifest {
            mac_version: "0.0.2".to_string(),
            files: BTreeMap::from([("agent/x.md".to_string(), entry(&sha_of(b"data")))]),
        };
        let diffs = diff(&manifest, &dir).expect("diff");
        assert!(diffs.is_empty());
    }

    #[test]
    fn read_manifest_missing_is_error() {
        let dir = temp_dir("no-manifest");
        let err = read_manifest(&dir).expect_err("missing manifest should error");
        assert!(err.to_string().contains("cannot read manifest"));
    }

    #[test]
    fn render_status_groups_and_is_deterministic() {
        let diffs = vec![
            DiffFile {
                rel: "b.md".to_string(),
                state: FileState::Added,
            },
            DiffFile {
                rel: "a.md".to_string(),
                state: FileState::Modified,
            },
            DiffFile {
                rel: "c.md".to_string(),
                state: FileState::Removed,
            },
        ];
        let text = render_status(&diffs).expect("non-empty");
        let expected = "modified (1)\n  a.md\nremoved (1)\n  c.md\nadded (1)\n  b.md\n";
        assert_eq!(text, expected);
    }

    #[test]
    fn render_status_pristine_is_none() {
        assert!(render_status(&[]).is_none());
    }
}
