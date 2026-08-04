//! `remove <type>:<id>` command: uninstall one component from an existing
//! tree. Deletes only the component's own tracked files (its `path` + `files`
//! entries); dependencies stay unless removed explicitly, so shared
//! dependencies are never broken. User-modified files are preserved unless
//! `--force`.

use std::path::Path;

use crate::install::content;
use crate::install::installer::{plan_files, resolve_component, sha256_hex, write_manifest};
use crate::install::model::Registry;
use crate::install::status;
use crate::install::{InstallError, Result};

/// Run `remove <type>:<id>`: read the manifest, delete the component's
/// tracked files, prune empty dirs, rewrite the manifest. Returns 0.
pub fn run(component: &str, install_dir: &str, force: bool) -> Result<i32> {
    let registry: Registry = serde_json::from_str(content::registry_json())
        .map_err(|e| InstallError::Install(format!("invalid embedded registry: {e}")))?;
    let (removed, kept, missing) =
        remove_component(&registry, component, Path::new(install_dir), force)?;
    println!(
        "Removed {component}: {removed} file(s) removed, {kept} kept (modified), {missing} missing"
    );
    println!("  Manifest: {install_dir}/.mac/manifest.json");
    Ok(0)
}

/// Remove the files of `component` (`type:id`) recorded in the manifest.
/// Returns `(removed, kept, missing)` counts.
pub fn remove_component(
    registry: &Registry,
    component: &str,
    install_dir: &Path,
    force: bool,
) -> Result<(usize, usize, usize)> {
    let (kind, id) = component.split_once(':').ok_or_else(|| {
        InstallError::Prompt(format!(
            "invalid component {component:?}: expected <type>:<id>"
        ))
    })?;
    if resolve_component(registry, kind, id).is_none() {
        return Err(InstallError::Prompt(format!(
            "unknown component {component:?}"
        )));
    }

    if !install_dir.join(status::MANIFEST_REL_PATH).exists() {
        return Ok((0, 0, 0)); // nothing installed yet
    }
    let mut manifest = status::read_manifest(install_dir)?;
    let planned = plan_files(registry, &[component.to_string()], install_dir);

    let mut removed = 0usize;
    let mut kept = 0usize;
    let mut missing = 0usize;
    let mut dirs_to_prune: Vec<std::path::PathBuf> = Vec::new();

    for file in &planned {
        let rel = file.rel.to_string_lossy().into_owned();
        let Some(entry) = manifest.files.get(&rel).cloned() else {
            continue; // not installed
        };
        if !file.target.exists() {
            missing += 1;
            manifest.files.remove(&rel);
            continue;
        }
        let sha = sha256_hex(&file.target)?;
        if !force && sha != entry.sha256 {
            kept += 1; // user-modified: keep the file and its manifest entry
            continue;
        }
        std::fs::remove_file(&file.target).map_err(|e| {
            InstallError::Install(format!("cannot remove {}: {e}", file.target.display()))
        })?;
        manifest.files.remove(&rel);
        removed += 1;
        if let Some(parent) = file.target.parent() {
            dirs_to_prune.push(parent.to_path_buf());
        }
    }

    for dir in dirs_to_prune {
        prune_empty_dirs(install_dir, &dir)?;
    }
    write_manifest(install_dir, &manifest.files.into_iter().collect::<Vec<_>>())?;
    Ok((removed, kept, missing))
}

/// Remove `dir` and its empty ancestors up to (but not including) `root`.
fn prune_empty_dirs(root: &Path, start: &Path) -> Result<()> {
    let mut dir = start.to_path_buf();
    loop {
        if dir == root || !dir.exists() {
            break;
        }
        let mut entries = std::fs::read_dir(&dir)
            .map_err(|e| InstallError::Install(format!("cannot read {}: {e}", dir.display())))?;
        if entries.next().is_some() {
            break; // not empty
        }
        std::fs::remove_dir(&dir)
            .map_err(|e| InstallError::Install(format!("cannot remove {}: {e}", dir.display())))?;
        let Some(parent) = dir.parent() else {
            break;
        };
        dir = parent.to_path_buf();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use crate::install::add::add_component;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "myagentcontrol-remove-{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn real_registry() -> Registry {
        serde_json::from_str(include_str!("../../content/registry.json")).expect("registry parses")
    }

    #[test]
    fn remove_deletes_tracked_files_and_manifest_entry() {
        let dir = temp_dir("basic");
        add_component(&real_registry(), "config:env-example", &dir, false).expect("add");
        let (removed, kept, missing) =
            remove_component(&real_registry(), "config:env-example", &dir, false).expect("remove");
        assert_eq!((removed, kept, missing), (1, 0, 0));
        assert!(!dir.join("config/env.example").exists());
        let manifest = status::read_manifest(&dir).expect("manifest");
        assert!(manifest.files.is_empty());
    }

    #[test]
    fn remove_keeps_modified_file_unless_force() {
        let dir = temp_dir("modified");
        add_component(&real_registry(), "config:env-example", &dir, false).expect("add");
        fs::write(dir.join("config/env.example"), b"user edited").expect("write");
        let (removed, kept, _) =
            remove_component(&real_registry(), "config:env-example", &dir, false).expect("remove");
        assert_eq!((removed, kept), (0, 1));
        assert!(dir.join("config/env.example").exists());
        // The manifest entry survives, so a forced retry can delete it.
        let (removed, kept, _) =
            remove_component(&real_registry(), "config:env-example", &dir, true).expect("remove");
        assert_eq!((removed, kept), (1, 0));
        assert!(!dir.join("config/env.example").exists());
    }

    #[test]
    fn remove_prunes_empty_dirs() {
        let dir = temp_dir("prune");
        add_component(&real_registry(), "config:agent-metadata", &dir, false).expect("add");
        assert!(dir.join("config").exists());
        remove_component(&real_registry(), "config:agent-metadata", &dir, false).expect("remove");
        assert!(!dir.join("config").exists());
    }

    #[test]
    fn remove_keeps_dependencies() {
        let dir = temp_dir("deps");
        add_component(&real_registry(), "agent:openagent", &dir, false).expect("add");
        remove_component(&real_registry(), "agent:openagent", &dir, false).expect("remove");
        let manifest = status::read_manifest(&dir).expect("manifest");
        assert!(!manifest.files.contains_key("agent/core/openagent.md"));
        assert!(
            manifest
                .files
                .contains_key("agent/subagents/core/task-manager.md")
        );
    }

    #[test]
    fn remove_not_installed_is_noop() {
        let dir = temp_dir("noop");
        let (removed, kept, missing) =
            remove_component(&real_registry(), "config:env-example", &dir, false).expect("remove");
        assert_eq!((removed, kept, missing), (0, 0, 0));
    }

    #[test]
    fn remove_unknown_component_is_error() {
        let dir = temp_dir("unknown");
        let err = remove_component(&real_registry(), "agent:ghost", &dir, false)
            .expect_err("unknown should error");
        assert!(err.to_string().contains("unknown component"));
    }

    #[test]
    fn remove_malformed_component_is_error() {
        let dir = temp_dir("malformed");
        let err = remove_component(&real_registry(), "no-colon-here", &dir, false)
            .expect_err("malformed should error");
        assert!(err.to_string().contains("expected <type>:<id>"));
    }
}
