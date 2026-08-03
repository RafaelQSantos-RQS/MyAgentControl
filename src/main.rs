use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use myagentcontrol::install::{self, Options};

/// The `myagentcontrol` CLI manages an `.opencode/`-compatible tree.
#[derive(Debug, Parser)]
#[command(name = "myagentcontrol", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Interactive installer (OAC install.sh flow, Brick 1: TUI only).
    Install(InstallArgs),
}

#[derive(Debug, Args)]
struct InstallArgs {
    /// Target directory for the managed tree.
    #[arg(long, default_value = ".opencode")]
    dir: String,
    /// Path to the component registry (defaults to the vendored copy).
    #[arg(long, default_value = "content/registry.json")]
    registry: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Install(args) => install::run(&Options {
            dir: args.dir,
            registry_path: args.registry,
        }),
    };
    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
