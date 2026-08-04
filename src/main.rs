use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use myagentcontrol::install::{self, Options, status};

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
    /// Compare the manifest against the install directory (read-only).
    Status(StatusArgs),
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

#[derive(Debug, Args)]
struct StatusArgs {
    /// Target directory for the managed tree.
    #[arg(long, default_value = ".opencode")]
    dir: String,
}

fn main() {
    let cli = Cli::parse();
    let exit_code = match cli.command {
        Command::Install(args) => match install::run(&Options {
            dir: args.dir,
            registry_path: args.registry,
            force: args.force,
        }) {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("Error: {err}");
                1
            }
        },
        Command::Status(args) => match status::run(&args.dir) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("Error: {err}");
                1
            }
        },
    };
    std::process::exit(exit_code);
}
