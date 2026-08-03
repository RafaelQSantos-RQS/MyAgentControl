//! Interactive installer TUI (Brick 1). Mirrors the OAC `install.sh` flow:
//! banner → location → mode → (profile | custom) → preview → confirm.
//!
//! **No real install logic yet**: after confirmation we show a placeholder
//! (copy/collision/manifest land in Brick 2+). Menus are fed by the real
//! registry ([`model::Registry`]) so the interface is faithful from day one.
//!
//! All menu logic is small, testable functions; the dialoguer calls are
//! isolated in [`select`]/[`prompt_err`] so the rest is unit-testable.

use console::{Term, style};
use dialoguer::{Confirm, Input, MultiSelect, Select};

use crate::install::model::{Category, Component, Registry};
use crate::install::{InstallError, Options, Result};

/// True when stdin/stdout is an interactive terminal (cli-spec D9 note).
pub fn is_interactive() -> bool {
    Term::stderr().is_term() && console::user_attended()
}

/// The installation location selected by the user.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Location {
    Local,
    Global,
    Custom(String),
    Exit,
}

/// The installation mode selected by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Profile,
    Custom,
    List,
    Exit,
}

/// A component picked in custom mode: its category plus the index within
/// that category's bucket (needed to resolve the real `id` afterward).
#[derive(Debug, Clone, Copy)]
struct Pick {
    category: Category,
    index: usize,
}

/// Run the interactive installer flow.
pub fn run(registry: &Registry, options: &Options) -> Result<()> {
    if !is_interactive() {
        return Err(InstallError::NotInteractive);
    }

    banner();
    let location = choose_location()?;
    if location == Location::Exit {
        return Ok(());
    }
    let install_dir = resolve_install_dir(&location, options);

    loop {
        match choose_mode()? {
            Mode::Profile => {
                let profile_key = choose_profile(registry)?;
                let selection = registry
                    .profiles
                    .get(&profile_key)
                    .expect("choose_profile returns a real key")
                    .components
                    .clone();
                if preview_and_confirm(&install_dir, &profile_key, &selection)? {
                    placeholder_install(&install_dir);
                }
                return Ok(());
            }
            Mode::Custom => {
                let selection = choose_custom(registry)?;
                if preview_and_confirm(&install_dir, "Custom", &selection)? {
                    placeholder_install(&install_dir);
                }
                return Ok(());
            }
            Mode::List => {
                list_components(registry)?;
                // back to mode menu
            }
            Mode::Exit => return Ok(()),
        }
    }
}

/// ASCII banner, matching the OAC installer header (Brick 1: compact).
fn banner() {
    let term = Term::stderr();
    let _ = term.write_line(
        &style("╔══════════════════════════════════════════════════════════╗")
            .cyan()
            .bold()
            .to_string(),
    );
    let _ = term.write_line(
        &style("║        MyAgentControl Installer v0.0.1                    ║")
            .cyan()
            .bold()
            .to_string(),
    );
    let _ = term.write_line(
        &style("╚══════════════════════════════════════════════════════════╝")
            .cyan()
            .bold()
            .to_string(),
    );
}

/// Step 1 — installation location (OAC `show_install_location_menu`).
fn choose_location() -> Result<Location> {
    let items = [
        "Local   — install to .opencode/ in the current directory".to_string(),
        "Global  — install to ~/.config/opencode".to_string(),
        "Custom  — enter an exact path".to_string(),
        "Exit".to_string(),
    ];
    match select("Choose installation location:", &items, 0)? {
        0 => Ok(Location::Local),
        1 => Ok(Location::Global),
        2 => {
            let path = Input::<String>::new()
                .with_prompt("Enter installation path")
                .allow_empty(false)
                .interact_text()
                .map_err(prompt_err)?;
            Ok(Location::Custom(path))
        }
        _ => Ok(Location::Exit),
    }
}

/// Step 2 — installation mode (OAC `show_main_menu`).
fn choose_mode() -> Result<Mode> {
    let items = [
        "Quick Install — choose a profile".to_string(),
        "Custom Install — pick individual components".to_string(),
        "List available components".to_string(),
        "Exit".to_string(),
    ];
    match select("Choose installation mode:", &items, 0)? {
        0 => Ok(Mode::Profile),
        1 => Ok(Mode::Custom),
        2 => Ok(Mode::List),
        _ => Ok(Mode::Exit),
    }
}

/// Step 3a — choose a profile (OAC `show_profile_menu`).
fn choose_profile(registry: &Registry) -> Result<String> {
    let keys: Vec<&String> = registry.profiles.keys().collect();
    let items: Vec<String> = keys
        .iter()
        .map(|key| {
            let p = &registry.profiles[*key];
            let count = p.components.len();
            match &p.description {
                Some(desc) => format!("{} — {} ({} components)", p.name, desc, count),
                None => format!("{} ({} components)", p.name, count),
            }
        })
        .collect();
    let idx = select("Available installation profiles:", &items, 0)?;
    Ok(keys[idx].clone())
}

/// Step 3b — custom component selection (OAC `show_custom_menu` + component
/// selection). Returns selected `type:id` strings.
fn choose_custom(registry: &Registry) -> Result<Vec<String>> {
    // Category multi-select.
    let cat_items: Vec<String> = Category::ALL
        .iter()
        .map(|c| format!("{} ({})", c.label(), c.components(registry).len()))
        .collect();
    let cat_indices = MultiSelect::new()
        .with_prompt("Select component categories (space to toggle, enter to continue)")
        .items(&cat_items)
        .max_length(8)
        .interact()
        .map_err(prompt_err)?;

    // Component multi-select across the chosen categories. Each entry maps
    // back to (category, index-in-category) so we can resolve the real id.
    let mut picks: Vec<(Pick, String)> = Vec::new();
    for ci in cat_indices {
        let cat = Category::ALL[ci];
        for (index, comp) in cat.components(registry).iter().enumerate() {
            picks.push((
                Pick {
                    category: cat,
                    index,
                },
                component_label(comp),
            ));
        }
    }
    if picks.is_empty() {
        return Ok(Vec::new());
    }

    let comp_items: Vec<&String> = picks.iter().map(|(_, l)| l).collect();
    let comp_indices = MultiSelect::new()
        .with_prompt("Select components (space to toggle, enter to continue)")
        .items(&comp_items)
        .max_length(10)
        .interact()
        .map_err(prompt_err)?;

    Ok(comp_indices
        .into_iter()
        .map(|i| {
            let (pick, _) = &picks[i];
            let comp = &pick.category.components(registry)[pick.index];
            format!("{}:{}", pick.category.type_key(), comp.id)
        })
        .collect())
}

/// Display label for one component in the custom menu.
fn component_label(comp: &Component) -> String {
    match &comp.description {
        Some(d) => format!("{} — {} (id: {})", comp.name, d, comp.id),
        None => format!("{} (id: {})", comp.name, comp.id),
    }
}

/// Step 4 — grouped preview + confirm (OAC `show_installation_preview`).
fn preview_and_confirm(dir: &str, title: &str, selection: &[String]) -> Result<bool> {
    let term = Term::stderr();
    let _ = term.write_line("");
    let _ = term.write_line(&style("Installation Preview").bold().to_string());
    let _ = term.write_line(&format!("Mode: {title}"));
    let _ = term.write_line(&format!("Installation directory: {dir}"));
    let _ = term.write_line(&format!(
        "\nComponents to install ({} total):",
        selection.len()
    ));
    for line in preview_lines(selection) {
        let _ = term.write_line(&line);
    }
    Confirm::new()
        .with_prompt("Proceed with installation?")
        .default(true)
        .interact()
        .map_err(prompt_err)
}

/// Group the selection (`type:id`) into readable preview lines.
fn preview_lines(selection: &[String]) -> Vec<String> {
    let mut groups: std::collections::BTreeMap<&str, Vec<&str>> = std::collections::BTreeMap::new();
    for s in selection {
        let (typ, id) = s.split_once(':').unwrap_or((s.as_str(), s.as_str()));
        groups.entry(typ).or_default().push(id);
    }
    groups
        .iter()
        .map(|(typ, ids)| format!("  {typ} ({}): {}", ids.len(), ids.join(", ")))
        .collect()
}

/// Step 5 placeholder — real install lands in Brick 2+.
fn placeholder_install(dir: &str) {
    let term = Term::stderr();
    let _ = term.write_line("");
    let _ = term.write_line(
        &style("✔ Ready to install — copy step lands in the next brick.")
            .green()
            .to_string(),
    );
    let _ = term.write_line(&format!("  Target: {dir}"));
    let _ = term.write_line(
        &style("  (Nothing was written; the interactive flow is the point of this brick.)")
            .dim()
            .to_string(),
    );
}

/// `list` mode: dump available components per category (OAC `list_components`).
fn list_components(registry: &Registry) -> Result<()> {
    let term = Term::stderr();
    let _ = term.write_line(&style("\nAvailable Components").bold().to_string());
    for cat in Category::ALL {
        let comps = cat.components(registry);
        if comps.is_empty() {
            continue;
        }
        let _ = term.write_line(
            &style(format!("{} ({}):", cat.label(), comps.len()))
                .cyan()
                .to_string(),
        );
        for comp in comps {
            let _ = term.write_line(&format!("  {} — {}", comp.name, comp.id));
        }
    }
    let _ = Term::stdout()
        .read_line()
        .map_err(|e| InstallError::Prompt(e.to_string()))?;
    Ok(())
}

fn resolve_install_dir(location: &Location, options: &Options) -> String {
    match location {
        Location::Local => options.dir.clone(),
        Location::Global => {
            let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
            format!("{home}/.config/opencode")
        }
        Location::Custom(path) => path.clone(),
        Location::Exit => options.dir.clone(),
    }
}

fn select(prompt: &str, items: &[String], default: usize) -> Result<usize> {
    Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(default)
        .interact()
        .map_err(prompt_err)
}

fn prompt_err(e: dialoguer::Error) -> InstallError {
    InstallError::Prompt(e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_registry() -> Registry {
        let json = r#"{
          "profiles": {
            "essential": {
              "name": "Essential (Minimal)",
              "description": "Minimal starter kit",
              "components": ["agent:openagent", "subagent:task-manager"]
            }
          },
          "components": {
            "agents": [{"id": "openagent", "name": "OpenAgent", "path": "p", "description": "Universal agent"}],
            "subagents": [{"id": "task-manager", "name": "TaskManager", "path": "p"}],
            "contexts": [],
            "skills": [],
            "commands": [],
            "tools": [],
            "plugins": [],
            "config": []
          }
        }"#;
        serde_json::from_str(json).expect("fixture parses")
    }

    #[test]
    fn preview_lines_groups_by_type() {
        let sel = vec![
            "agent:openagent".to_string(),
            "agent:opencoder".to_string(),
            "context:essential-patterns".to_string(),
        ];
        let lines = preview_lines(&sel);
        assert_eq!(lines.len(), 2);
        assert!(
            lines
                .iter()
                .any(|l| l == "  agent (2): openagent, opencoder")
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "  context (1): essential-patterns")
        );
    }

    #[test]
    fn preview_lines_empty_selection() {
        assert!(preview_lines(&[]).is_empty());
    }

    #[test]
    fn preview_lines_handles_entries_without_colon() {
        // Malformed selection entries must not panic.
        let lines = preview_lines(&["orphan-id".to_string()]);
        assert_eq!(lines, vec!["  orphan-id (1): orphan-id"]);
    }

    #[test]
    fn category_helpers_match_counts() {
        let reg = fixture_registry();
        assert_eq!(Category::Agents.components(&reg).len(), 1);
        assert_eq!(Category::Subagents.components(&reg).len(), 1);
        assert_eq!(Category::Contexts.components(&reg).len(), 0);
        assert_eq!(Category::Agents.components(&reg)[0].id, "openagent");
    }

    #[test]
    fn resolve_install_dir_local_and_custom() {
        let opts = Options {
            dir: ".opencode".to_string(),
            registry_path: std::path::PathBuf::from("content/registry.json"),
        };
        assert_eq!(resolve_install_dir(&Location::Local, &opts), ".opencode");
        assert_eq!(
            resolve_install_dir(&Location::Custom("/tmp/x".to_string()), &opts),
            "/tmp/x"
        );
    }
}
