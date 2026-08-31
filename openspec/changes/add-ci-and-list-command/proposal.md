## Why

1. **No CI**: The README promises quality gates (`cargo test`, `clippy`, `fmt`) but nothing enforces them. Every push relies on manual checks. A GitHub Actions workflow catches regressions before merge.

2. **No `list` subcommand**: The `list_components` function exists inside the interactive TUI (`ui.rs`) but is not exposed as a standalone CLI command. Users who want to see available components must launch the full interactive installer.

## What Changes

- Add `.github/workflows/ci.yml` with three jobs: test, clippy, fmt
- Add `list` subcommand to the CLI that prints available components grouped by category
- Reuse existing `list_components` logic from `ui.rs` (extract to a non-TUI function)

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `cli`: Add `list` subcommand

## Impact

- `.github/workflows/ci.yml` — new file
- `src/main.rs` — add `List` variant to `Command` enum
- `src/install/ui.rs` — extract list logic to be callable without TUI
