//! `add <type>:<id>` command (registry-spec REG-6, cli-spec CLI-9): install
//! one component plus its transitive dependencies into an existing tree,
//! non-destructively, merging the new entries into `.mac/manifest.json`.
//! Reuses the copy pipeline from [`installer`].

use std::collections::BTreeMap;
use std::path::Path;

use crate::install::content;
use crate::install::installer::{
    InstallSummary, Manifest, ManifestEntry, copy_bytes, expand_dependencies, now_rfc3339,
    plan_files, resolve_component, sha256_hex, write_manifest,
};
use crate::install::model::Registry;
use crate::install::status;
use crate::install::{InstallError, Result};

/// Run `add <type>:<id>`: validate, install with dependencies, merge the
/// manifest, print the summary. Returns 0 on success.
pub fn run(component: &str, install_dir: &str, force: bool) -> Result<i32> {
    let registry: Registry = serde_json::from_str(content::registry_json())
        .map_err(|e| InstallError::Install(format!("invalid embedded registry: {e}")))?;
    let summary = add_component(&registry, component, Path::new(install_dir), force)?;
    println!(
        "Added {component}: {} file(s) copied, {} skipped, {} collision(s)",
        summary.copied, summary.skipped, summary.collided
    );
    println!("  Manifest: {install_dir}/.mac/manifest.json");
    Ok(0)
}

/// Install `component` (`type:id`) plus its transitive dependencies into
/// `install_dir`, merging the new manifest entries with any existing ones.
pub fn add_component(
    registry: &Registry,
    component: &str,
    install_dir: &Path,
    force: bool,
) -> Result<InstallSummary> {
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

    let expanded = expand_dependencies(registry, &[component.to_string()]);
    let planned = plan_files(registry, &expanded, install_dir);
    if planned.is_empty() {
        return Err(InstallError::Prompt(format!(
            "component {component:?} has no installable files (check its registry path)"
        )));
    }

    let mut files: Vec<(std::path::PathBuf, Vec<u8>)> = Vec::new();
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

    let mut new_entries: BTreeMap<String, ManifestEntry> = BTreeMap::new();
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
        new_entries.insert(
            relative,
            ManifestEntry {
                r#type: file.kind.clone(),
                installed_at: now_rfc3339(),
                sha256: sha,
            },
        );
    }

    let mut merged = read_existing_manifest(install_dir)?.files;
    for (rel, entry) in new_entries {
        merged.insert(rel, entry);
    }
    write_manifest(install_dir, &merged.into_iter().collect::<Vec<_>>())?;
    Ok(summary)
}

/// Read the manifest, or return an empty one when no install exists yet
/// (a fresh tree before its first `add`).
fn read_existing_manifest(install_dir: &Path) -> Result<Manifest> {
    if !install_dir.join(status::MANIFEST_REL_PATH).exists() {
        return Ok(Manifest {
            mac_version: "0.0.2".to_string(),
            files: BTreeMap::new(),
        });
    }
    status::read_manifest(install_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use crate::install::model::Category;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("myagentcontrol-add-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn real_registry() -> Registry {
        serde_json::from_str(include_str!("../../content/registry.json")).expect("registry parses")
    }

    #[test]
    fn add_simple_component_writes_manifest() {
        let dir = temp_dir("simple");
        let summary = add_component(&real_registry(), "tool:env", &dir, false).expect("add");
        assert!(summary.copied >= 1);
        let manifest = status::read_manifest(&dir).expect("manifest");
        assert!(manifest.files.contains_key("tool/env/index.ts"));
    }

    #[test]
    fn add_expands_transitive_dependencies() {
        let dir = temp_dir("deps");
        // openagent depends on subagent:task-manager transitively.
        let summary = add_component(&real_registry(), "agent:openagent", &dir, false).expect("add");
        assert!(summary.copied >= 1);
        let manifest = status::read_manifest(&dir).expect("manifest");
        assert!(manifest.files.contains_key("agent/core/openagent.md"));
        assert!(
            manifest
                .files
                .contains_key("agent/subagents/core/task-manager.md")
        );
    }

    #[test]
    fn add_merges_with_existing_manifest() {
        let dir = temp_dir("merge");
        add_component(&real_registry(), "tool:env", &dir, false).expect("first add");
        add_component(&real_registry(), "command:clean", &dir, false).expect("second add");
        let manifest = status::read_manifest(&dir).expect("manifest");
        assert!(manifest.files.contains_key("tool/env/index.ts"));
        assert!(manifest.files.contains_key("command/clean.md"));
    }

    #[test]
    fn add_unknown_component_is_error() {
        let dir = temp_dir("unknown");
        let err = add_component(&real_registry(), "agent:ghost", &dir, false)
            .expect_err("unknown should error");
        assert!(err.to_string().contains("unknown component"));
    }

    #[test]
    fn add_malformed_component_is_error() {
        let dir = temp_dir("malformed");
        let err = add_component(&real_registry(), "no-colon-here", &dir, false)
            .expect_err("malformed should error");
        assert!(err.to_string().contains("expected <type>:<id>"));
    }

    #[test]
    fn every_registry_component_has_installable_files() {
        // Data-integrity guard: every component's `path` + `files` must map
        // to a real file in the embedded content tree (REG-2 validation).
        // Catches broken registry entries like the old env-example/readme
        // paths that lacked the .opencode/ prefix and a vendored file.
        let reg = real_registry();
        let mut checked = 0usize;
        for cat in Category::ALL {
            for comp in cat.components(&reg) {
                let selection = format!("{}:{}", cat.type_key(), comp.id);
                let planned = plan_files(
                    &reg,
                    std::slice::from_ref(&selection),
                    std::path::Path::new("/tmp/integrity"),
                );
                assert!(
                    !planned.is_empty(),
                    "component {selection} has no installable files"
                );
                for file in &planned {
                    assert!(
                        content::read(&file.rel).is_some(),
                        "missing embedded content for {selection}: {}",
                        file.rel.display()
                    );
                }
                checked += 1;
            }
        }
        assert!(checked >= 100, "checked only {checked} components");
    }

    #[test]
    fn add_is_non_destructive_without_force() {
        let dir = temp_dir("collide");
        fs::create_dir_all(dir.join("config")).expect("mkdir");
        fs::write(dir.join("config/agent-metadata.json"), b"user-edited").expect("write");
        let summary =
            add_component(&real_registry(), "config:agent-metadata", &dir, false).expect("add");
        assert_eq!(summary.copied, 0);
        assert_eq!(summary.collided, 1);
        // User content preserved.
        assert_eq!(
            fs::read_to_string(dir.join("config/agent-metadata.json")).unwrap(),
            "user-edited"
        );
    }

    #[test]
    fn add_is_idempotent_on_rerun() {
        let dir = temp_dir("idempotent");
        add_component(&real_registry(), "tool:env", &dir, false).expect("first");
        let summary = add_component(&real_registry(), "tool:env", &dir, false).expect("second");
        assert_eq!(summary.copied, 0);
        assert_eq!(summary.skipped, 1);
    }
}
