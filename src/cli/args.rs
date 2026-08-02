//! CLI argument definitions (clap derive) — cli-spec §3 command surface.
//!
//! ```text
//! myagentcontrol init [--dir .opencode] [--force]
//! myagentcontrol validate [--agents|--skills|--commands|--context|--evals|--registry|--all]
//! myagentcontrol list <agents|skills|commands|context|evals> [--format table|json]
//! myagentcontrol wizard agent|skill|command new
//! myagentcontrol wizard add-context [--update]
//! myagentcontrol evals validate|dashboard
//! myagentcontrol import <path-to-oac> [--dry-run]
//! myagentcontrol export <path>
//! myagentcontrol doctor
//! ```
//!
//! Global flags (CLI-7): `--no-color`, `--format table|json`.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// The `myagentcontrol` CLI — manages an `.opencode/`-compatible tree.
#[derive(Debug, Parser)]
#[command(name = "myagentcontrol", version, about)]
pub struct Cli {
    /// Disable colored output (CLI-7 determinism).
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Output format (CLI-7): table (default) or json.
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Command,
}

/// Output rendering format (CLI-3, CLI-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

/// Top-level subcommands (cli-spec §3).
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Scaffold the full managed tree (non-destructive, idempotent).
    Init(InitArgs),
    /// Run module validators; exit 1 on any failure (--all is default).
    Validate(ValidateArgs),
    /// List managed items (agents, skills, commands, context, evals).
    List(ListArgs),
    /// Interactive generators (agent, skill, command, add-context).
    Wizard(WizardArgs),
    /// Eval framework: validate cases, render dashboard.
    Evals(EvalsArgs),
    /// Import an existing OAC tree into managed state.
    Import(ImportArgs),
    /// Export the managed tree to a target directory.
    Export(ExportArgs),
    /// Check the environment (paths, opencode presence, structure).
    Doctor,
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Target directory for the managed tree.
    #[arg(long, default_value = ".opencode")]
    pub dir: PathBuf,
    /// Overwrite existing files.
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct ValidateArgs {
    #[arg(long)]
    pub agents: bool,
    #[arg(long)]
    pub skills: bool,
    #[arg(long)]
    pub commands: bool,
    #[arg(long)]
    pub context: bool,
    #[arg(long)]
    pub evals: bool,
    #[arg(long)]
    pub registry: bool,
    /// Validate everything (default).
    #[arg(long)]
    pub all: bool,
}

/// Valid `list` targets (CLI-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ListTarget {
    Agents,
    Skills,
    Commands,
    Context,
    Evals,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// What to list.
    #[arg(value_enum)]
    pub target: ListTarget,
}

#[derive(Debug, Args)]
pub struct WizardArgs {
    #[command(subcommand)]
    pub kind: WizardKind,
}

#[derive(Debug, Subcommand)]
pub enum WizardKind {
    /// Generate a new agent: `wizard agent new`.
    Agent(WizardNew),
    /// Generate a new skill: `wizard skill new`.
    Skill(WizardNew),
    /// Generate a new command: `wizard command new`.
    Command(WizardNew),
    /// 6-question Project Intelligence wizard.
    AddContext(AddContextArgs),
}

/// Shared action args for the generator wizards.
#[derive(Debug, Args)]
pub struct WizardNew {
    #[command(subcommand)]
    pub action: WizardNewAction,
}

/// The single generator action (cli-spec §3: `new`).
#[derive(Debug, Subcommand)]
pub enum WizardNewAction {
    /// Generate a new item.
    New,
}

#[derive(Debug, Args)]
pub struct AddContextArgs {
    /// Update existing context instead of creating it.
    #[arg(long)]
    pub update: bool,
}

#[derive(Debug, Args)]
pub struct EvalsArgs {
    #[command(subcommand)]
    pub action: EvalsAction,
}

#[derive(Debug, Subcommand)]
pub enum EvalsAction {
    /// Validate eval case files.
    Validate,
    /// Render the results dashboard.
    Dashboard,
}

#[derive(Debug, Args)]
pub struct ImportArgs {
    /// Path to an existing OAC tree.
    pub path: PathBuf,
    /// Preview without writing.
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Args)]
pub struct ExportArgs {
    /// Target directory for the export.
    pub path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;

    #[test]
    fn global_no_color_and_format_flags() {
        let cli =
            Cli::try_parse_from(["myagentcontrol", "--no-color", "--format", "json", "doctor"])
                .expect("parse");
        assert!(cli.no_color);
        assert_eq!(cli.format, OutputFormat::Json);
        assert!(matches!(cli.command, Command::Doctor));
    }

    #[test]
    fn validate_all_parses() {
        let cli = Cli::try_parse_from(["myagentcontrol", "validate", "--all"]).expect("parse");
        let Command::Validate(v) = cli.command else {
            panic!("expected validate");
        };
        assert!(v.all);
    }

    #[test]
    fn list_agents_json_parses() {
        let cli = Cli::try_parse_from(["myagentcontrol", "list", "agents", "--format", "json"])
            .expect("parse");
        let Command::List(l) = cli.command else {
            panic!("expected list");
        };
        assert_eq!(l.target, ListTarget::Agents);
        assert_eq!(cli.format, OutputFormat::Json);
    }

    #[test]
    fn wizard_agent_new_parses() {
        let cli = Cli::try_parse_from(["myagentcontrol", "wizard", "agent", "new"]).expect("parse");
        let Command::Wizard(w) = cli.command else {
            panic!("expected wizard");
        };
        let WizardKind::Agent(WizardNew {
            action: WizardNewAction::New,
        }) = w.kind
        else {
            panic!("expected wizard agent new");
        };
    }

    #[test]
    fn wizard_add_context_update_parses() {
        let cli = Cli::try_parse_from(["myagentcontrol", "wizard", "add-context", "--update"])
            .expect("parse");
        let Command::Wizard(w) = cli.command else {
            panic!("expected wizard");
        };
        let WizardKind::AddContext(a) = w.kind else {
            panic!("expected add-context");
        };
        assert!(a.update);
    }

    #[test]
    fn import_dry_run_parses() {
        let cli = Cli::try_parse_from(["myagentcontrol", "import", "/tmp/oac", "--dry-run"])
            .expect("parse");
        let Command::Import(i) = cli.command else {
            panic!("expected import");
        };
        assert_eq!(i.path, PathBuf::from("/tmp/oac"));
        assert!(i.dry_run);
    }

    #[test]
    fn unknown_subcommand_errors() {
        let err = Cli::try_parse_from(["myagentcontrol", "frobnicate"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
    }

    #[test]
    fn init_defaults_dir_to_opencode() {
        let cli = Cli::try_parse_from(["myagentcontrol", "init"]).expect("parse");
        let Command::Init(i) = cli.command else {
            panic!("expected init");
        };
        assert_eq!(i.dir, PathBuf::from(".opencode"));
        assert!(!i.force);
    }
}
