//! Interactive installer TUI: banner → location → mode →
//! (profile | custom) → preview → confirm. Menus are fed by the real
//! registry ([`model::Registry`]); confirmed selections run the real
//! install via [`installer::install`].

use std::path::Path;

use console::{Term, style};
use dialoguer::theme::Theme;
use dialoguer::{Confirm, Input, MultiSelect, Select};

use crate::install::model::{Category, Component, Registry};
use crate::install::{InstallError, Options, Result, installer};

/// Dracula accent colors, fixed truecolor so the TUI looks the same
/// regardless of the terminal theme (ANSI 16 slots vary per terminal).
mod palette {
    use console::Color;

    /// Active items and category headers.
    pub const PURPLE: Color = Color::TrueColor(0xBD, 0x93, 0xF9);
    /// Success messages.
    pub const GREEN: Color = Color::TrueColor(0x50, 0xFA, 0x7B);
    /// Muted/secondary text.
    pub const MUTED: Color = Color::TrueColor(0x62, 0x72, 0xA4);
}

/// True when stdin is an interactive terminal.
pub fn is_interactive() -> bool {
    Term::stdout().is_term() && console::user_attended()
}

/// The installation location selected by the user.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Location {
    Local,
    Global,
    Custom(String),
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
    let location = match choose_location()? {
        Some(loc) => loc,
        None => return Ok(()),
    };
    let install_dir = resolve_install_dir(&location, options);

    loop {
        match choose_mode()? {
            Mode::Profile => {
                let profile_key = choose_profile(registry)?;
                let profile = registry.profiles.get(&profile_key).ok_or_else(|| {
                    InstallError::Prompt(format!("unknown profile {profile_key:?}"))
                })?;
                if preview_and_confirm(&install_dir, &profile_key, &profile.components)? {
                    do_install(registry, options, &install_dir, &profile.components)?;
                }
                return Ok(());
            }
            Mode::Custom => {
                let selection = choose_custom(registry)?;
                if preview_and_confirm(&install_dir, "Custom", &selection)? {
                    do_install(registry, options, &install_dir, &selection)?;
                }
                return Ok(());
            }
            Mode::List => list_components(registry)?,
            Mode::Exit => return Ok(()),
        }
    }
}

/// ASCII banner, centered on the terminal width.
fn banner() {
    const ART: &str = "
███╗   ███╗██╗   ██╗     █████╗  ██████╗ ███████╗███╗   ██╗████████╗
████╗ ████║╚██╗ ██╔╝    ██╔══██╗██╔════╝ ██╔════╝████╗  ██║╚══██╔══╝
██╔████╔██║ ╚████╔╝     ███████║██║  ███╗█████╗  ██╔██╗ ██║   ██║
██║╚██╔╝██║  ╚██╔╝      ██╔══██║██║   ██║██╔══╝  ██║╚██╗██║   ██║
██║ ╚═╝ ██║   ██║       ██║  ██║╚██████╔╝███████╗██║ ╚████║   ██║
╚═╝     ╚═╝   ╚═╝       ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝   ╚═╝

 ██████╗ ██████╗ ███╗   ██╗████████╗██████╗  ██████╗ ██╗
██╔════╝██╔═══██╗████╗  ██║╚══██╔══╝██╔══██╗██╔═══██╗██║
██║     ██║   ██║██╔██╗ ██║   ██║   ██████╔╝██║   ██║██║
██║     ██║   ██║██║╚██╗██║   ██║   ██╔══██╗██║   ██║██║
╚██████╗╚██████╔╝██║ ╚████║   ██║   ██║  ██║╚██████╔╝███████╗
 ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝
";

    let term = Term::stderr();
    // `Term::size()` returns (rows, cols).
    let (_, cols) = term.size();
    // ART lines differ in width (fmt strips trailing padding); normalize to
    // the widest line so the whole block centers with one shared pad.
    let art_lines: Vec<&str> = ART.lines().filter(|l| !l.trim().is_empty()).collect();
    let art_width = art_lines
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);
    let pad = (cols as usize).saturating_sub(art_width) / 2;
    for line in art_lines {
        let padded = format!("{}{:<pad$}", " ".repeat(pad), line, pad = art_width);
        let _ = term.write_line(&style(padded).bold().for_stderr().to_string());
    }
    let _ = term.write_line("");
}

/// Step 1 — installation location (OAC `show_install_location_menu`).
/// Returns `None` when the user chooses "Exit".
fn choose_location() -> Result<Option<Location>> {
    let items = [
        "Local   — install to .opencode/ in the current directory".to_string(),
        "Global  — install to ~/.config/opencode".to_string(),
        "Custom  — enter an exact path".to_string(),
        "Exit".to_string(),
    ];
    match select("Choose installation location:", &items, 0)? {
        0 => Ok(Some(Location::Local)),
        1 => Ok(Some(Location::Global)),
        2 => {
            let path = Input::<String>::new()
                .with_prompt("Enter installation path")
                .allow_empty(false)
                .interact_text()
                .map_err(prompt_err)?;
            Ok(Some(Location::Custom(path)))
        }
        _ => Ok(None),
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
            let head = format!("{} — {} components", p.name, count);
            match &p.description {
                Some(desc) => format!("{head}\n{}", wrap(desc, PROFILE_DESC_WIDTH)),
                None => head,
            }
        })
        .collect();
    let idx = select("Available installation profiles:", &items, 0)?;
    Ok(keys[idx].clone())
}

/// Max column width for a profile description line.
const PROFILE_DESC_WIDTH: usize = 64;

/// Word-wrap `text` to at most `width` columns, preserving words.
fn wrap(text: &str, width: usize) -> String {
    let mut out = String::new();
    let mut line_len = 0usize;
    for word in text.split_whitespace() {
        let space = if line_len == 0 { 0 } else { 1 };
        if line_len + space + word.len() > width {
            out.push('\n');
            out.push_str(word);
            line_len = word.len();
        } else {
            if line_len > 0 {
                out.push(' ');
            }
            out.push_str(word);
            line_len += space + word.len();
        }
    }
    out
}

/// Step 3b — custom component selection (OAC `show_custom_menu` + component
/// selection). Returns selected `type:id` strings.
fn choose_custom(registry: &Registry) -> Result<Vec<String>> {
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

    // Each entry maps back to (category, index) so the real id can be
    // resolved after selection.
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

/// Run the real install for a confirmed selection and print the summary.
fn do_install(
    registry: &Registry,
    options: &Options,
    install_dir: &str,
    selection: &[String],
) -> Result<()> {
    let summary = installer::install(registry, selection, Path::new(install_dir), options.force)?;
    let term = Term::stderr();
    let _ = term.write_line("");
    let _ = term.write_line(
        &style(format!(
            "✔ Installed {} file(s) — {} skipped, {} collision(s)",
            summary.copied, summary.skipped, summary.collided
        ))
        .fg(palette::GREEN)
        .for_stderr()
        .to_string(),
    );
    let _ = term.write_line(&format!("  Manifest: {}/.mac/manifest.json", install_dir));
    Ok(())
}

/// Print available components per category to stdout (no TUI, no keypress).
pub fn list_components_plain(registry: &Registry) {
    println!("\nAvailable Components");
    for cat in Category::ALL {
        let comps = cat.components(registry);
        if comps.is_empty() {
            continue;
        }
        println!("{} ({}):", cat.label(), comps.len());
        for comp in comps {
            println!("  {} — {}", comp.name, comp.id);
        }
    }
}

/// `list` mode: dump available components per category (OAC `list_components`).
fn list_components(registry: &Registry) -> Result<()> {
    list_components_plain(registry);
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
    }
}

/// Dialoguer theme: bold prompts (no trailing colon — prompt strings already
/// include it), purple active items, multi-line items whose first line is
/// the option name and following lines its description.
#[derive(Debug, Clone, Copy)]
struct InstallerTheme;

impl Theme for InstallerTheme {
    fn format_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        write!(f, "{}", style(prompt).bold().for_stderr())
    }

    fn format_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        // Keep only the first line of a multi-line item in the collapsed line.
        let head = sel.split('\n').next().unwrap_or(sel);
        write!(f, "{} {}", style(prompt).bold().for_stderr(), head)
    }

    fn format_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        active: bool,
    ) -> std::fmt::Result {
        let mut lines = text.splitn(2, '\n');
        let head = lines.next().unwrap_or_default();
        let rest = lines.next();
        if active {
            write!(f, "{} ", style(">").fg(palette::PURPLE).bold().for_stderr())?;
            write!(f, "{}", style(head).fg(palette::PURPLE).bold().for_stderr())?;
        } else {
            write!(f, "  {}", style(head).for_stderr())?;
        }
        if let Some(desc) = rest {
            for line in desc.lines() {
                writeln!(f)?;
                write!(f, "    {}", style(line).fg(palette::MUTED).for_stderr())?;
            }
        }
        Ok(())
    }
}

fn select(prompt: &str, items: &[String], default: usize) -> Result<usize> {
    Select::with_theme(&InstallerTheme)
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

    #[test]
    fn preview_lines_groups_by_type() {
        let sel = vec![
            "agent:openagent".to_string(),
            "agent:opencoder".to_string(),
            "context:essential-patterns".to_string(),
        ];
        let lines = preview_lines(&sel);
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn preview_lines_joins_same_type_ids() {
        let sel = vec!["agent:openagent".to_string(), "agent:opencoder".to_string()];
        let lines = preview_lines(&sel);
        assert!(
            lines
                .iter()
                .any(|l| l == "  agent (2): openagent, opencoder")
        );
    }

    #[test]
    fn preview_lines_lists_singleton_type() {
        let sel = vec!["context:essential-patterns".to_string()];
        let lines = preview_lines(&sel);
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
    fn resolve_install_dir_local() {
        let opts = Options {
            dir: ".opencode".to_string(),
            registry_path: None,
            force: false,
        };
        assert_eq!(resolve_install_dir(&Location::Local, &opts), ".opencode");
    }

    #[test]
    fn resolve_install_dir_custom() {
        let opts = Options {
            dir: ".opencode".to_string(),
            registry_path: None,
            force: false,
        };
        assert_eq!(
            resolve_install_dir(&Location::Custom("/tmp/x".to_string()), &opts),
            "/tmp/x"
        );
    }

    #[test]
    fn wrap_short_text_unchanged() {
        assert_eq!(wrap("short text", 64), "short text");
    }

    #[test]
    fn wrap_breaks_long_lines_at_width() {
        let wrapped = wrap("one two three four", 8);
        assert_eq!(wrapped, "one two\nthree\nfour");
    }

    #[test]
    fn wrap_keeps_words_whole() {
        let wrapped = wrap("a b c d", 3);
        assert_eq!(wrapped, "a b\nc d");
    }

    #[test]
    fn wrap_empty_text() {
        assert_eq!(wrap("", 64), "");
    }
}
