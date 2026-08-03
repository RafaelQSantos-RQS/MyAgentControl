//! Interactive installer: TUI flow mirroring OAC `install.sh`.
//! Entry point: [`run`]. Loads the registry then hands over to the TUI
//! ([`ui`]); real copy/collision/manifest logic lands in a later stage.

pub mod model;
pub mod registry;
pub mod ui;

use std::path::PathBuf;

/// Options for the installer (filled by the CLI in `src/main.rs`).
#[derive(Debug, Clone)]
pub struct Options {
    /// Target directory for the managed tree (default `.opencode`).
    pub dir: String,
    /// Path to the component registry (default `content/registry.json`).
    pub registry_path: PathBuf,
}

/// Top-level installer error (cli-spec §7 envelopes, minimal on purpose).
#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    /// Registry load/parse failures (E400 io / E100 parse).
    #[error(transparent)]
    Registry(#[from] registry::LoadError),
    /// Non-TTY invocation — guidance error, no E-envelope (cli-spec §7/§10.4).
    #[error("interactive installer requires a terminal; run from a TTY")]
    NotInteractive,
    /// Prompt/interaction failures — guidance error, no E-envelope (§7/§10.4).
    #[error("{0}")]
    Prompt(String),
}

/// Convenience alias used across the installer.
pub type Result<T> = std::result::Result<T, InstallError>;

/// Entry point: load the registry, then run the interactive TUI.
pub fn run(options: &Options) -> Result<()> {
    let registry = registry::load(&options.registry_path)?;
    ui::run(&registry, options)
}
