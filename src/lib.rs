//! `myagentcontrol` library crate.
//!
//! Exposes the modules the binary uses so integration tests in `tests/`
//! can import them. `src/main.rs` is a thin wrapper over [`install::run`].
//!
//! Brick 1: interactive installer TUI (mirrors OAC `install.sh`). No real
//! install logic yet — only the interactive interface over the real
//! `content/registry.json` data.

pub mod install;
