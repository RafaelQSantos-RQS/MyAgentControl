//! `myagentcontrol` library crate.
//!
//! Exposes the modules the binary uses so integration tests in `tests/`
//! can import them. `src/main.rs` is a thin wrapper over [`install::run`].
//!
//! Interactive installer TUI (mirrors OAC `install.sh`): the interactive
//! interface over the real `content/registry.json` data. No real install
//! logic yet.

pub mod install;
