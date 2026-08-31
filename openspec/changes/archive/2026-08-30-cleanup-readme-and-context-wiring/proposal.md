## Why

The README's status section is stale: it claims `validate` and `wizard` are "not yet implemented" when they are fully functional. Additionally, `context/cache.rs` (209 lines, 4 tests) and `context/resolver.rs` (156 lines, 5 tests) are fully implemented but never called from any command — dead code that adds maintenance burden without value.

## What Changes

- Update README.md status section to reflect that `validate` and `wizard` are implemented
- Update README.md commands table to include `validate` and `wizard`
- Update README.md repository layout to reflect the new `openspec/specs/` structure (`.specs/` was removed)
- Wire `context::resolver::resolve` into `run_validate` to use local-first resolution instead of hardcoded path logic
- Wire `context::cache` into a new `cache` subcommand or remove it entirely (TBD in design)

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `cli`: README accuracy and CLI command completeness

## Impact

- `README.md` — documentation update (no code change)
- `src/main.rs` — potential integration of resolver into validate, possible new cache subcommand
- `src/context/cache.rs` — either wired into a command or deleted
- `src/context/resolver.rs` — integrated into validate's path resolution
