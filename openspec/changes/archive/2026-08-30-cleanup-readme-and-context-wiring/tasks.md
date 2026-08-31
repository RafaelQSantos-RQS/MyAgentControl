## 1. README Updates

- [ ] 1.1 Update status section: mark `validate` and `wizard` as implemented
- [ ] 1.2 Update commands table: add `validate` and `wizard` rows
- [ ] 1.3 Update repository layout: replace `.specs/` with `openspec/specs/`
- [ ] 1.4 Update development section: replace `.specs/README.md` reference with `openspec/`

## 2. Resolver Integration

- [ ] 2.1 Replace hardcoded path logic in `run_validate` with `resolver::resolve`
- [ ] 2.2 Verify validate command works with `--dir` flag

## 3. Cache Cleanup

- [ ] 3.1 Delete `src/context/cache.rs`
- [ ] 3.2 Remove `pub mod cache;` from `src/context/mod.rs`
- [ ] 3.3 Run `cargo test` to verify no breakage

## 4. Commit

- [ ] 4.1 Run `cargo clippy -- -D warnings` and `cargo fmt --check`
- [ ] 4.2 Commit with message: `chore: fix stale README, integrate resolver, remove dead cache`
