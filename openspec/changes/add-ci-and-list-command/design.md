## Context

- No `.github/` directory exists — zero CI
- `list_components` in `ui.rs` (line 332) iterates `Category::ALL`, prints components, then waits for a keypress (TUI-specific)
- The `model::Registry` and `model::Category` types already provide the data needed for listing
- The CLI uses clap derive with `Command` enum in `main.rs`

## Goals / Non-Goals

**Goals:**
- GitHub Actions CI: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- Standalone `list` subcommand: print available components grouped by category

**Non-Goals:**
- Caching or matrix builds (keep CI simple)
- Adding `--format` or `--filter` flags to list (YAGNI)
- Modifying the interactive installer flow

## Decisions

### Decision 1: CI structure

**Choice**: Single workflow file with three parallel jobs (test, clippy, fmt). Each job uses `actions/checkout@v4` and `dtolnay/rust-toolchain@stable`.

**Alternatives considered**:
- Single job running all three sequentially — rejected: slower, no parallelism
- Separate workflow files — rejected: over-fragmented for a small project

**Rationale**: Three parallel jobs give faster feedback and clear failure signals.

### Decision 2: list extraction

**Choice**: Extract the print logic from `list_components` into a public `list_components_plain` function in `ui.rs` that writes to stdout (no TUI, no keypress wait). The existing `list_components` calls it then waits for input. The CLI `list` command calls `list_components_plain` directly.

**Alternatives considered**:
- Move to a new `list.rs` module — rejected: the function is 20 lines, not worth a new file
- Duplicate the logic — rejected: DRY violation

**Rationale**: Minimal change, reuses existing code, keeps the TUI flow intact.

## Risks / Trade-offs

- **Risk**: CI may fail on first run if hidden issues exist → **Mitigation**: run all checks locally before committing
- **Risk**: `list` output goes to stdout while TUI version uses stderr → **Mitigation**: correct behavior (CLI tools write data to stdout, UI to stderr)

## Migration Plan

1. Create `.github/workflows/ci.yml`
2. Extract `list_components_plain` from `list_components`
3. Add `List` variant to CLI
4. Test locally
5. Commit and push (CI runs on push)

## Open Questions

None.
