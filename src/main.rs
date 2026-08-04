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
    /// Interactive installer (TUI only).
    Install(InstallArgs),
}

#[derive(Debug, Args)]
struct InstallArgs {
    /// Target directory for the managed tree.
    #[arg(long, default_value = ".opencode")]
    dir: String,
    /// Override the embedded registry with a file on disk (advanced).
    #[arg(long)]
    registry: Option<PathBuf>,
    /// Overwrite existing files instead of skipping them.
    #[arg(long)]
    force: bool,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Install(args) => install::run(&Options {
            dir: args.dir,
            registry_path: args.registry,
            force: args.force,
        }),
    };
    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
