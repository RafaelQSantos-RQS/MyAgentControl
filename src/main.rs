use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand};

use myagentcontrol::context;
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
    /// Validate context files (MVI, frontmatter, @-references).
    #[command(
        long_about = "Validate the context tree: check frontmatter on all files, enforce MVI \
                      rules on concept cards, and check @-reference syntax in agent/command files.\n\n\
                      Exits 0 when all checks pass, 1 on any defect."
    )]
    Validate(ValidateArgs),
    /// Interactive wizards for creating agents, skills, commands, or context.
    #[command(
        long_about = "Run an interactive wizard to generate a new agent, skill, command, \
                      or context file.\n\nRequires an interactive terminal."
    )]
    Wizard(WizardArgs),
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

#[derive(Debug, Args)]
struct ValidateArgs {
    /// Target directory for the managed tree.
    #[arg(long, default_value = ".opencode")]
    dir: String,
    /// Validate only context files.
    #[arg(long)]
    context: bool,
}

#[derive(Debug, Subcommand)]
enum WizardCommand {
    /// Create a new context file via the 6-question Project Intelligence wizard.
    AddContext {
        /// Target directory for the managed tree.
        #[arg(long, default_value = ".opencode")]
        dir: String,
        /// Update an existing context file (bump version + date).
        #[arg(long)]
        update: bool,
    },
}

#[derive(Debug, Args)]
struct WizardArgs {
    #[command(subcommand)]
    command: WizardCommand,
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
        Command::Validate(args) => {
            let root = Path::new(&args.dir);
            match run_validate(root, args.context) {
                Ok(()) => 0,
                Err(err) => {
                    eprintln!("Error: {err}");
                    1
                }
            }
        }
        Command::Wizard(args) => match args.command {
            WizardCommand::AddContext { dir, update } => {
                let root = Path::new(&dir);
                match run_add_context(root, update) {
                    Ok(()) => 0,
                    Err(err) => {
                        eprintln!("Error: {err}");
                        1
                    }
                }
            }
        },
    };
    std::process::exit(exit_code);
}

fn run_validate(root: &Path, context_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    use myagentcontrol::context::frontmatter;
    use myagentcontrol::context::mvi;
    use myagentcontrol::context::resolver::{self, FsGlob};

    // Resolve context root using local-first, global-fallback logic
    let home = std::env::var("HOME").ok().map(std::path::PathBuf::from);
    let global = home.map(|h| h.join(".config/opencode"));
    let resolved = resolver::resolve(root, global.as_deref(), None, &FsGlob);

    let context_dir = match &resolved {
        resolver::CoreRoot::Local(path) => path.join("context"),
        resolver::CoreRoot::GlobalForCore { local, global_core } => {
            // Prefer local context, fall back to global core
            let local_ctx = local.join("context");
            if local_ctx.exists() {
                local_ctx
            } else {
                global_core.join("context")
            }
        }
        resolver::CoreRoot::LocalOnly(path) => path.join("context"),
    };

    if !context_dir.exists() {
        return Err("context directory not found".into());
    }

    let mut errors = Vec::new();

    // Walk context files
    for entry in std::fs::read_dir(&context_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "md").unwrap_or(false) {
            let content = std::fs::read_to_string(&path)?;
            let file_name = path.file_name().unwrap().to_string_lossy();

            // Check frontmatter
            if let Some(first_line) = content.lines().next()
                && let Err(e) = frontmatter::parse_frontmatter(first_line)
            {
                errors.push(format!("{}: {}", path.display(), e));
            }

            // Check MVI
            if let Err(e) = mvi::validate_mvi(&content, &file_name) {
                errors.push(format!("{}: {}", path.display(), e));
            }
        }
    }

    // Walk agent/command files for @-references (unless context_only)
    if !context_only {
        use myagentcontrol::context::references;
        for dir_name in &["agent", "command"] {
            let dir = root.join(dir_name);
            if !dir.exists() {
                continue;
            }
            for entry in std::fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    let content = std::fs::read_to_string(&path)?;
                    let errs = references::validate_references(&content);
                    for e in errs {
                        errors.push(format!("{}: {}", path.display(), e));
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        println!("All checks passed");
        Ok(())
    } else {
        for e in &errors {
            eprintln!("{e}");
        }
        Err(format!("{} validation errors", errors.len()).into())
    }
}

fn run_add_context(root: &Path, update: bool) -> Result<(), Box<dyn std::error::Error>> {
    if update {
        let path = context::wizard::update_context_file(root, false)?;
        println!("Updated: {}", path.display());
    } else {
        let answers = context::wizard::run_wizard()?;
        let path = context::wizard::write_context_file(root, &answers, "1.0")?;
        println!("Created: {}", path.display());
    }
    Ok(())
}
