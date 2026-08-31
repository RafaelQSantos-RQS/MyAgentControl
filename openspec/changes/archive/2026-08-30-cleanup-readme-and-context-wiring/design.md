## Context

- `README.md` lines 24-27 say `validate`, `list`, wizards, and eval framework are "not yet implemented" — but `validate` and `wizard` are fully implemented and tested
- `README.md` line 91 still references `.specs/` which was removed in the prior change
- `src/context/resolver.rs` implements local-first resolution (check `./opencode/` then `~/.config/opencode/`) but `run_validate` in `main.rs` uses hardcoded `root.join("context")` instead
- `src/context/cache.rs` is a full LRU cache with manifest, add/update/remove/list — all tested, zero call sites

## Goals / Non-Goals

**Goals:**
- Fix README to accurately reflect implemented features
- Integrate `resolver::resolve` into `run_validate` for correct path resolution
- Decide and execute on `cache.rs`: wire it in or delete it

**Non-Goals:**
- Adding new CLI subcommands (cache integration is separate from this cleanup)
- Rewriting validate logic beyond path resolution
- Touching any spec files

## Decisions

### Decision 1: Resolver integration

**Choice**: Replace hardcoded path logic in `run_validate` with `resolver::resolve`. The resolver already handles local-first, global-fallback, and custom dir — exactly what validate needs.

**Rationale**: The resolver was built for this. Using it eliminates duplicated logic and makes validate respect the same resolution rules as the rest of the system.

### Decision 2: Cache handling

**Choice**: Delete `cache.rs` entirely. It's a general-purpose document cache with no clear integration point. The context system uses file-based resolution, not a cache layer. If caching is needed later, it can be re-implemented with a simpler design.

**Alternatives considered**:
- Wire into `validate` — rejected: validate reads files directly, caching adds complexity without measurable benefit for a CLI tool
- Keep as dead code — rejected: maintenance burden, confusing for contributors

**Rationale**: YAGNI. The cache was designed for a future that hasn't arrived. Delete it, keep the tests as documentation of intent.

## Risks / Trade-offs

- **Risk**: Deleting cache.rs removes tested code that might be useful later → **Mitigation**: Git history preserves it; re-implementation is straightforward if needed
- **Risk**: Resolver integration changes validate behavior subtly → **Mitigation**: Resolver is well-tested with mock glob; behavior is identical for the common case (local install exists)

## Migration Plan

1. Update README.md (status, commands table, repository layout)
2. Integrate resolver into run_validate
3. Delete cache.rs and remove from mod.rs
4. Run tests to verify nothing breaks
5. Commit

## Open Questions

None.
