//! CLI dispatch — cli-spec §6 layout, `src/cli/`.
//!
//! Phase 0 skeleton: every subcommand parses and dispatches to a stub
//! handler that prints a placeholder and returns `Ok`. Real behavior lands
//! in later phases (context, agents, skills, commands, evals).

pub mod args;

use crate::core::errors::Result;
use args::{
    Cli, Command, EvalsAction, ListTarget, OutputFormat, WizardKind, WizardNew, WizardNewAction,
};

/// Entry point after `Cli::parse` (see `src/main.rs`).
pub fn run(cli: Cli) -> Result<()> {
    let format = cli.format;
    let _no_color = cli.no_color; // consumed by output rendering in later phases
    match cli.command {
        Command::Init(a) => cmd_init(a.dir.display().to_string(), a.force),
        Command::Validate(v) => cmd_validate(
            v.agents, v.skills, v.commands, v.context, v.evals, v.registry, v.all,
        ),
        Command::List(l) => cmd_list(l.target, format),
        Command::Wizard(w) => cmd_wizard(w.kind),
        Command::Evals(e) => cmd_evals(e.action),
        Command::Import(i) => cmd_import(i.path.display().to_string(), i.dry_run),
        Command::Export(e) => cmd_export(e.path.display().to_string()),
        Command::Doctor => cmd_doctor(),
    }
}

/// `init [--dir .opencode] [--force]` — CLI-1.
fn cmd_init(dir: String, force: bool) -> Result<()> {
    let flag = if force { " --force" } else { "" };
    println!("✔ [stub] init{flag} → would scaffold managed tree into {dir}");
    Ok(())
}

/// `validate [--agents|--skills|--commands|--context|--evals|--registry|--all]` — CLI-2/CLI-8.
fn cmd_validate(
    agents: bool,
    skills: bool,
    commands: bool,
    context: bool,
    evals: bool,
    registry: bool,
    all: bool,
) -> Result<()> {
    let scope = if all || !(agents || skills || commands || context || evals || registry) {
        "all modules".to_string()
    } else {
        [
            (agents, "agents"),
            (skills, "skills"),
            (commands, "commands"),
            (context, "context"),
            (evals, "evals"),
            (registry, "registry"),
        ]
        .iter()
        .filter(|(on, _)| *on)
        .map(|(_, name)| *name)
        .collect::<Vec<_>>()
        .join(", ")
    };
    println!("✔ [stub] validate → would check {scope}");
    Ok(())
}

/// `list <agents|skills|commands|context|evals> [--format table|json]` — CLI-3.
fn cmd_list(target: ListTarget, format: OutputFormat) -> Result<()> {
    let what = match target {
        ListTarget::Agents => "agents",
        ListTarget::Skills => "skills",
        ListTarget::Commands => "commands",
        ListTarget::Context => "context",
        ListTarget::Evals => "evals",
    };
    let fmt = match format {
        OutputFormat::Table => "table",
        OutputFormat::Json => "json",
    };
    println!("✔ [stub] list {what} --format {fmt}");
    Ok(())
}

/// `wizard agent|skill|command new` and `wizard add-context [--update]` — CLI-4.
fn cmd_wizard(kind: WizardKind) -> Result<()> {
    let what = match kind {
        WizardKind::Agent(WizardNew {
            action: WizardNewAction::New,
        }) => "agent",
        WizardKind::Skill(WizardNew {
            action: WizardNewAction::New,
        }) => "skill",
        WizardKind::Command(WizardNew {
            action: WizardNewAction::New,
        }) => "command",
        WizardKind::AddContext(a) => {
            let flag = if a.update { " --update" } else { "" };
            return println_ok(&format!("[stub] wizard add-context{flag}"));
        }
    };
    println_ok(&format!("[stub] wizard {what} new"))
}

/// `evals validate|dashboard` — deferred `evals run` per evals-spec D1.
fn cmd_evals(action: EvalsAction) -> Result<()> {
    let what = match action {
        EvalsAction::Validate => "validate",
        EvalsAction::Dashboard => "dashboard",
    };
    println_ok(&format!("[stub] evals {what}"))
}

/// `import <path> [--dry-run]` — CLI-5.
fn cmd_import(path: String, dry_run: bool) -> Result<()> {
    let flag = if dry_run { " --dry-run" } else { "" };
    println_ok(&format!(
        "[stub] import{flag} → would import OAC tree from {path}"
    ))
}

/// `export <path>`.
fn cmd_export(path: String) -> Result<()> {
    println_ok(&format!(
        "[stub] export → would write managed tree to {path}"
    ))
}

/// `doctor` — CLI-6.
fn cmd_doctor() -> Result<()> {
    println_ok("[stub] doctor → would check paths, opencode presence, structure")
}

/// Print a status line and return `Ok`.
fn println_ok(msg: &str) -> Result<()> {
    println!("✔ {msg}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn init_stub_prints_and_returns_ok() {
        let cli = Cli::try_parse_from(["myagentcontrol", "init"]).expect("parse");
        let out = run(cli);
        assert!(out.is_ok());
    }

    #[test]
    fn validate_all_ok() {
        let cli = Cli::try_parse_from(["myagentcontrol", "validate", "--all"]).expect("parse");
        assert!(run(cli).is_ok());
    }

    #[test]
    fn wizard_agent_new_ok() {
        let cli = Cli::try_parse_from(["myagentcontrol", "wizard", "agent", "new"]).expect("parse");
        assert!(run(cli).is_ok());
    }

    #[test]
    fn doctor_with_flags_ok() {
        let cli =
            Cli::try_parse_from(["myagentcontrol", "--no-color", "--format", "json", "doctor"])
                .expect("parse");
        assert!(run(cli).is_ok());
    }
}
