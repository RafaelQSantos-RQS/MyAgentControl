use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use myagentcontrol::install::{self, Options, add, remove, status};

/// Manage an OpenCode-compatible agent tree: install components from an
/// embedded registry, track them in a manifest, and keep the tree in sync.
#[derive(Debug, Parser)]
#[command(
    name = "myagentcontrol",
    version,
    about = "Manage an OpenCode-compatible agent tree",
    long_about = "Install, add, remove, and verify components of an OpenCode-compatible \
                  agent tree.\n\nComponents are packaged in an embedded registry and tracked \
                  in a manifest (`.mac/manifest.json`) inside the target directory, so every \
                  command knows exactly what was installed and where.",
    arg_required_else_help = true,
    after_help = "Examples:\n  myagentcontrol install\n  myagentcontrol add agent:openagent\n  \
                  myagentcontrol status\n  myagentcontrol remove context:quick-start\n\nEvery \
                  command accepts --dir to target a custom tree root (default: .opencode)."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Interactive installer (TUI only).
    #[command(
        long_about = "Walk through an interactive menu: pick a location, choose a profile or \
                      individual components, preview the selection, and install into the target \
                      tree.\n\nRequires an interactive terminal (see the NotInteractive error \
                      otherwise)."
    )]
    Install(InstallArgs),
    /// Install a component plus its dependencies into an existing tree.
    #[command(
        long_about = "Install one component (`<type>:<id>`, e.g. `context:quick-start`) plus its \
                      transitive dependencies into an existing tree, merging the new files into \
                      the manifest.\n\nExisting files are preserved unless --force is given."
    )]
    Add(AddArgs),
    /// Uninstall a component's tracked files from an existing tree.
    #[command(
        long_about = "Remove the files of one component (`<type>:<id>`) that the manifest \
                      records, prune empty directories, and update the manifest.\n\nDependencies \
                      are left in place unless removed explicitly. Files the user modified are \
                      preserved unless --force is given."
    )]
    Remove(RemoveArgs),
    /// Compare the manifest against the install directory (read-only).
    #[command(
        long_about = "Read the manifest and compare SHA256 hashes against the files on disk, \
                      reporting modified, removed, and added files.\n\nExits 0 when the tree is \
                      pristine, 1 when it diverges."
    )]
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
struct AddArgs {
    /// Component to install, in `<type>:<id>` form (e.g. `context:quick-start`).
    component: String,
    /// Target directory for the managed tree.
    #[arg(long, default_value = ".opencode")]
    dir: String,
    /// Overwrite existing files instead of skipping them.
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Args)]
struct RemoveArgs {
    /// Component to remove, in `<type>:<id>` form (e.g. `context:quick-start`).
    component: String,
    /// Target directory for the managed tree.
    #[arg(long, default_value = ".opencode")]
    dir: String,
    /// Remove files even if the user modified them.
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
        Command::Add(args) => match add::run(&args.component, &args.dir, args.force) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("Error: {err}");
                1
            }
        },
        Command::Remove(args) => match remove::run(&args.component, &args.dir, args.force) {
            Ok(code) => code,
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
