//! Real install logic (registry-spec §5): resolve components, expand
//! dependencies, plan the copy, then write files non-destructively. Source
//! bytes come from the embedded [`content`](crate::install::content) tree
//! (C6 distribution); the `.mac/manifest.json` record (REG-7) is written
//! inside the install directory by [`write_manifest`].

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::install::content;
use crate::install::model::{Category, Component, Registry};
use crate::install::{InstallError, Result};

/// One planned file copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedFile {
    /// Path relative to the embedded `content/` root (e.g.
    /// `agent/core/openagent.md`).
    pub rel: PathBuf,
    /// Destination under the install directory.
    pub target: PathBuf,
    /// Component type (`agent`, `skill`, ...).
    pub kind: String,
}

/// Copy outcome counters for the summary line.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InstallSummary {
    pub copied: usize,
    pub skipped: usize,
    pub collided: usize,
}

/// Resolve `type:id` against the registry, returning the component.
pub fn resolve_component<'r>(
    registry: &'r Registry,
    kind: &str,
    id: &str,
) -> Option<&'r Component> {
    Category::ALL
        .iter()
        .find(|c| c.type_key() == kind)
        .and_then(|c| c.components(registry).iter().find(|comp| comp.id == id))
}

/// Expand a selection of `type:id` strings to its transitive dependency
/// closure (BFS order, deduplicated, cycle-safe). Unknown references are
/// ignored (validation is a separate concern).
pub fn expand_dependencies(registry: &Registry, selection: &[String]) -> Vec<String> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut order: Vec<String> = Vec::new();
    for entry in selection {
        expand_one(registry, entry, &mut seen, &mut order);
    }
    order
}

/// BFS from one `type:id`, appending new entries to `order`.
fn expand_one<'r>(
    registry: &'r Registry,
    entry: &'r str,
    seen: &mut BTreeSet<&'r str>,
    order: &mut Vec<String>,
) {
    if !seen.insert(entry) {
        return;
    }
    order.push(entry.to_string());
    let Some((kind, id)) = entry.split_once(':') else {
        return;
    };
    let Some(comp) = resolve_component(registry, kind, id) else {
        return;
    };
    for dep in &comp.dependencies {
        expand_one(registry, dep, seen, order);
    }
}

/// Plan every file to copy for `selection`: the component `path` plus each
/// `files` entry. Paths are root-relative (e.g. `agent/core/openagent.md`);
/// a legacy `.opencode/` prefix is tolerated and stripped.
pub fn plan_files(
    registry: &Registry,
    selection: &[String],
    install_dir: &Path,
) -> Vec<PlannedFile> {
    let mut planned = Vec::new();
    for entry in selection {
        let Some((kind, id)) = entry.split_once(':') else {
            continue;
        };
        let Some(comp) = resolve_component(registry, kind, id) else {
            continue;
        };
        for rel in std::iter::once(&comp.path).chain(comp.files.iter()) {
            let rel = rel.strip_prefix(".opencode/").unwrap_or(rel);
            planned.push(PlannedFile {
                rel: PathBuf::from(rel),
                target: install_dir.join(rel),
                kind: kind.to_string(),
            });
        }
    }
    planned
}

/// Write `files` (target path + bytes) into the install tree,
/// non-destructively. Existing files are skipped; a differing existing file
/// is counted as a collision and left untouched. With `force`, differing
/// files are overwritten. Parent directories are created as needed.
pub fn copy_bytes(files: &[(PathBuf, Vec<u8>)], force: bool) -> Result<InstallSummary> {
    let mut summary = InstallSummary::default();
    for (target, bytes) in files {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| InstallError::Install(e.to_string()))?;
        }
        if target.exists() {
            let same = fs::read(target)
                .map(|existing| existing == *bytes)
                .unwrap_or(false);
            if same {
                summary.skipped += 1;
            } else if force {
                fs::write(target, bytes)
                    .map_err(|e| InstallError::Install(format!("{}: {e}", target.display())))?;
                summary.copied += 1;
            } else {
                summary.collided += 1;
            }
            continue;
        }
        fs::write(target, bytes)
            .map_err(|e| InstallError::Install(format!("{}: {e}", target.display())))?;
        summary.copied += 1;
    }
    Ok(summary)
}

/// SHA-256 of a file as a lowercase hex string (streamed in 8 KiB chunks).
pub fn sha256_hex(path: &Path) -> Result<String> {
    let file = fs::File::open(path)
        .map_err(|e| InstallError::Install(format!("{}: {e}", path.display())))?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| InstallError::Install(format!("{}: {e}", path.display())))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

/// Resolve a `type:id` selection to a full install: expand dependencies,
/// plan files, write them from the embedded `content/` tree, then record
/// `<install_dir>/.mac/manifest.json` (REG-7). When `force` is set,
/// existing files are overwritten instead of skipped.
pub fn install(
    registry: &Registry,
    selection: &[String],
    install_dir: &Path,
    force: bool,
) -> Result<InstallSummary> {
    let expanded = expand_dependencies(registry, selection);
    let planned = plan_files(registry, &expanded, install_dir);

    let mut files: Vec<(PathBuf, Vec<u8>)> = Vec::new();
    for file in &planned {
        let bytes = content::read(&file.rel).ok_or_else(|| {
            InstallError::Install(format!(
                "missing embedded content file {}",
                file.rel.display()
            ))
        })?;
        files.push((file.target.clone(), bytes.to_vec()));
    }
    let summary = copy_bytes(&files, force)?;

    let mut entries = Vec::new();
    for file in &planned {
        if !file.target.exists() {
            continue;
        }
        let sha = sha256_hex(&file.target)?;
        let relative = file
            .target
            .strip_prefix(install_dir)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| file.target.to_string_lossy().into_owned());
        entries.push((
            relative,
            ManifestEntry {
                r#type: file.kind.clone(),
                installed_at: now_rfc3339(),
                sha256: sha,
            },
        ));
    }
    write_manifest(install_dir, &entries)?;
    Ok(summary)
}

/// RFC 3339 timestamp for `installed_at` (REG-7 §4).
pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// A `.mac/manifest.json` entry per REG-7 §4.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManifestEntry {
    pub r#type: String,
    pub installed_at: String,
    pub sha256: String,
}

/// The full manifest document written to `<install_dir>/.mac/manifest.json`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    pub mac_version: String,
    pub files: BTreeMap<String, ManifestEntry>,
}

/// Write `<install_dir>/.mac/manifest.json` for the installed files. Paths
/// in `installed` are relative to `install_dir` (the managed tree root).
pub fn write_manifest(install_dir: &Path, installed: &[(String, ManifestEntry)]) -> Result<()> {
    let manifest = Manifest {
        mac_version: "0.0.2".to_string(),
        files: installed.iter().cloned().collect(),
    };
    let dir = install_dir.join(".mac");
    fs::create_dir_all(&dir).map_err(|e| InstallError::Install(e.to_string()))?;
    let text = serde_json::to_string_pretty(&manifest)
        .map_err(|e| InstallError::Install(e.to_string()))?;
    fs::write(dir.join("manifest.json"), text).map_err(|e| InstallError::Install(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "myagentcontrol-install-{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn resolve_known_component() {
        let reg: Registry = serde_json::from_str(include_str!("../../content/registry.json"))
            .expect("registry parses");
        let comp = resolve_component(&reg, "agent", "openagent").expect("known");
        assert_eq!(comp.path, "agent/core/openagent.md");
    }

    #[test]
    fn resolve_unknown_returns_none() {
        let reg: Registry = serde_json::from_str(include_str!("../../content/registry.json"))
            .expect("registry parses");
        assert!(resolve_component(&reg, "agent", "ghost").is_none());
        assert!(resolve_component(&reg, "bogus", "openagent").is_none());
    }

    #[test]
    fn expand_dependencies_is_transitive_and_deduped() {
        let reg: Registry = serde_json::from_str(include_str!("../../content/registry.json"))
            .expect("registry parses");
        let sel = vec!["agent:openagent".to_string()];
        let expanded = expand_dependencies(&reg, &sel);
        assert!(expanded.contains(&"agent:openagent".to_string()));
        // openagent depends on task-manager (subagent) transitively used.
        assert!(expanded.contains(&"subagent:task-manager".to_string()));
        let deduped: BTreeSet<&str> = expanded.iter().map(String::as_str).collect();
        assert_eq!(expanded.len(), deduped.len());
    }

    #[test]
    fn expand_dependencies_cycle_safe() {
        let reg: Registry = serde_json::from_str(include_str!("../../content/registry.json"))
            .expect("registry parses");
        // Pick a component with a self-referential-looking dep path; the
        // wildcard reference is ignored, and a cycle terminates.
        let sel = vec!["agent:system-builder".to_string()];
        let expanded = expand_dependencies(&reg, &sel);
        assert!(expanded.contains(&"agent:system-builder".to_string()));
    }

    #[test]
    fn plan_files_tolerates_legacy_opencode_prefix() {
        // Pre-fix registries hardcoded the `.opencode/` root prefix in every
        // path; the planner must still map them to the content root.
        let json = r#"{
            "version": "2.0.0",
            "profiles": {},
            "components": {
                "agents": [{
                    "id": "legacy", "name": "Legacy", "type": "agent",
                    "path": ".opencode/agent/legacy.md",
                    "description": "", "tags": [], "dependencies": [],
                    "category": "standard"
                }]
            }
        }"#;
        let reg: Registry = serde_json::from_str(json).expect("registry parses");
        let planned = plan_files(&reg, &["agent:legacy".to_string()], Path::new("/tmp/x"));
        assert_eq!(planned.len(), 1);
        assert_eq!(planned[0].rel, Path::new("agent/legacy.md"));
        assert_eq!(planned[0].target, Path::new("/tmp/x/agent/legacy.md"));
    }

    #[test]
    fn plan_files_maps_paths() {
        let reg: Registry = serde_json::from_str(include_str!("../../content/registry.json"))
            .expect("registry parses");
        let sel = vec!["skill:task-management".to_string()];
        let planned = plan_files(&reg, &sel, Path::new("/tmp/install"));
        assert!(
            planned
                .iter()
                .any(|f| f.rel.ends_with("skills/task-management/SKILL.md"))
        );
        assert!(
            planned
                .iter()
                .any(|f| f.target.starts_with("/tmp/install/skills/task-management/"))
        );
        // Skill has path + 2 extra files in the real registry.
        assert!(planned.len() >= 3);
    }

    #[test]
    fn copy_files_skips_identical_and_collides_on_diff() {
        let dir = temp_dir("copy");
        let dst = dir.join("dst");
        fs::create_dir_all(&dst).expect("mkdir");
        fs::write(dst.join("same.md"), b"hello").expect("write");
        fs::write(dst.join("diff.md"), b"v1").expect("write");
        let files = vec![
            (dst.join("same.md"), b"hello".to_vec()),
            (dst.join("diff.md"), b"v2".to_vec()),
        ];
        let summary = copy_bytes(&files, false).expect("copy");
        assert_eq!(summary.copied, 0);
        assert_eq!(summary.skipped, 1);
        assert_eq!(summary.collided, 1);
        // diff.md untouched (v1 preserved)
        assert_eq!(fs::read_to_string(dst.join("diff.md")).unwrap(), "v1");
    }

    #[test]
    fn copy_files_copies_missing() {
        let dir = temp_dir("fresh");
        let dst = dir.join("dst");
        let files = vec![(dst.join("nested/new.md"), b"data".to_vec())];
        let summary = copy_bytes(&files, false).expect("copy");
        assert_eq!(summary.copied, 1);
        assert_eq!(summary.skipped, 0);
        assert_eq!(summary.collided, 0);
        assert_eq!(
            fs::read_to_string(dst.join("nested/new.md")).unwrap(),
            "data"
        );
    }

    #[test]
    fn sha256_matches_known_value() {
        let dir = temp_dir("hash");
        let f = dir.join("data.bin");
        let mut file = fs::File::create(&f).expect("create");
        file.write_all(b"hello").expect("write");
        // sha256 of "hello"
        let expected = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        assert_eq!(sha256_hex(&f).unwrap(), expected);
    }

    #[test]
    fn write_manifest_records_entries() {
        let dir = temp_dir("manifest");
        let entry = ManifestEntry {
            r#type: "agent".to_string(),
            installed_at: "2026-08-03T00:00:00Z".to_string(),
            sha256: "abc".to_string(),
        };
        write_manifest(&dir, &[("agent/core/x.md".to_string(), entry)]).expect("write");
        let text = fs::read_to_string(dir.join(".mac/manifest.json")).expect("read");
        let parsed: serde_json::Value = serde_json::from_str(&text).expect("json");
        assert_eq!(parsed["mac_version"], "0.0.2");
        assert_eq!(parsed["files"]["agent/core/x.md"]["type"], "agent");
        assert_eq!(parsed["files"]["agent/core/x.md"]["sha256"], "abc");
    }

    #[test]
    fn copy_files_force_overwrites_collision() {
        let dir = temp_dir("force");
        let dst = dir.join("dst");
        fs::create_dir_all(&dst).expect("mkdir");
        fs::write(dst.join("diff.md"), b"v1").expect("write");
        let files = vec![(dst.join("diff.md"), b"v2".to_vec())];
        let summary = copy_bytes(&files, true).expect("copy");
        assert_eq!(summary.copied, 1);
        assert_eq!(summary.skipped, 0);
        assert_eq!(summary.collided, 0);
        assert_eq!(fs::read_to_string(dst.join("diff.md")).unwrap(), "v2");
    }
}
