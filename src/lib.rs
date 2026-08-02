//! `myagentcontrol` library crate.
//!
//! Exposes the same modules the binary uses so integration tests in
//! `tests/` can import them (task 05 test harness, cli-spec §8).
//! `src/main.rs` is a thin wrapper over [`cli::run`].

pub mod cli;
pub mod core;
