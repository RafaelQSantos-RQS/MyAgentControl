## Context

Cargo.toml currently has only `name`, `version`, and `edition`. Standard Rust metadata fields are missing. The LICENSE file is MIT. The repo is at `github.com/RafaelQSantos-RQS/MyAgentControl`. NOTICE.md attributes OpenAgentsControl v0.7.1.

## Goals / Non-Goals

**Goals:**
- Add all standard `[package]` metadata fields
- Make the crate ready for eventual crates.io publishing

**Non-Goals:**
- Publishing to crates.io (that's a separate change)
- Changing version (stays 0.0.1 for pre-v1)
- Modifying dependencies or features

## Decisions

### Decision 1: Metadata values

**Choice**: Use these values based on existing project files:
- `description`: from README line 1
- `license`: "MIT"
- `repository`: from git remote
- `authors`: project owner
- `readme`: "README.md"
- `keywords`: ["opencode", "agents", "configuration", "sdd"]
- `categories`: ["command-line-utilities", "config"]

## Risks / Trade-offs

None — metadata-only, zero runtime impact.

## Migration Plan

1. Edit Cargo.toml
2. Run `cargo check` to verify no breakage
3. Commit

## Open Questions

None.
