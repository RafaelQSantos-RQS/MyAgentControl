//! `myagentcontrol` library crate.
//!
//! Exposes the same modules the binary uses so integration tests in
//! `tests/` can import them (walk tests, cli-spec §8).
//! `src/main.rs` is a thin wrapper over [`cli::run`].

pub mod cli;
pub mod context;
pub mod core;
pub mod skills;
