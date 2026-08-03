//! Interactive installer (Brick 1): TUI flow mirroring OAC `install.sh`.
//!
//! Entry point: [`run`]. Loads the registry (real data), then hands over
//! to the TUI ([`ui`]). Real copy/collision/manifest logic is deliberately
//! deferred to Brick 2+ — this brick is the interactive interface.
//!
//! Error envelope follows cli-spec §7 (E-codes), kept minimal on purpose.

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

/// Top-level installer error (cli-spec §7 envelopes, minimal for Brick 1).
#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    /// Registry load/parse failures.
    #[error(transparent)]
    Registry(#[from] registry::LoadError),
    /// Interactive mode requires a terminal (cli-spec D9 note).
    #[error("E100: interactive installer requires a terminal; run from a TTY")]
    NotInteractive,
    /// Prompt/interaction failures (cancelled, I/O, etc.).
    #[error("E100: {0}")]
    Prompt(String),
}

/// Convenience alias used across the installer.
pub type Result<T> = std::result::Result<T, InstallError>;

/// Entry point: load the registry, then run the interactive TUI.
pub fn run(options: &Options) -> Result<()> {
    let registry = registry::load(&options.registry_path)?;
    ui::run(&registry, options)
}
