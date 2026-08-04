//! Interactive installer: TUI flow mirroring OAC `install.sh`.
//! Entry point: [`run`]. Loads the registry (embedded by default) then
//! hands over to the TUI ([`ui`]).

pub mod content;
pub mod installer;
pub mod model;
pub mod registry;
pub mod ui;

use std::path::PathBuf;

/// Options for the installer (filled by the CLI in `src/main.rs`).
#[derive(Debug, Clone)]
pub struct Options {
    /// Target directory for the managed tree (default `.opencode`).
    pub dir: String,
    /// Optional override for the registry file. `None` uses the registry
    /// embedded into the binary at build time (`content/registry.json`).
    pub registry_path: Option<PathBuf>,
    /// Overwrite existing files instead of skipping them.
    pub force: bool,
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
    /// Copy/manifest failures (E600 install-state envelope, cli-spec §7).
    #[error("E600: {0}")]
    Install(String),
}

/// Convenience alias used across the installer.
pub type Result<T> = std::result::Result<T, InstallError>;

/// Entry point: load the registry (embedded unless overridden), then run
/// the interactive TUI.
pub fn run(options: &Options) -> Result<()> {
    let registry = match &options.registry_path {
        Some(path) => registry::load(path)?,
        None => {
            let path = "content/registry.json (embedded)".to_string();
            serde_json::from_str(content::registry_json())
                .map_err(|source| registry::LoadError::Parse { path, source })?
        }
    };
    ui::run(&registry, options)
}
