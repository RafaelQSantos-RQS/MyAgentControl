## 1. Module Structure

- [x] 1.1 Create `src/context/mod.rs` with public API surface
- [x] 1.2 Create `src/context/frontmatter.rs` — HTML-comment frontmatter parser + validator
- [x] 1.3 Create `src/context/mvi.rs` — MVI concept card validator (line count, reference heading)
- [x] 1.4 Create `src/context/resolver.rs` — local-first / global-fallback resolver with `Glob` trait
- [x] 1.5 Create `src/context/references.rs` — `@`-reference syntax validator
- [x] 1.6 Create `src/context/cache.rs` — external context cache (manifest + add/update/list/remove)
- [x] 1.7 Create `src/context/wizard.rs` — `/add-context` interactive wizard (6 questions, file write, nav update)
- [x] 1.8 Register `pub mod context` in `src/lib.rs`

## 2. Frontmatter Parser (CTX-1)

- [x] 2.1 Implement `parse_frontmatter(line: &str) -> Option<Frontmatter>` with split-on-`|` logic
- [x] 2.2 Validate priority field against `{critical, high, medium, low}` set → `CTX-201`
- [x] 2.3 Validate version field as semver `X.Y` → `CTX-203`
- [x] 2.4 Validate date field as `YYYY-MM-DD` → `CTX-204`
- [x] 2.5 Detect missing frontmatter → `CTX-202`
- [x] 2.6 Unit tests for all valid and invalid frontmatter cases from context-spec §7.2–7.3

## 3. MVI Validator (CTX-2)

- [x] 3.1 Implement single-pass line counter + heading collector
- [x] 3.2 Enforce < 200 line limit on concept cards (reference docs ≥ 200 exempt)
- [x] 3.3 Check reference heading presence on non-discovery concept cards → `CTX-208`
- [x] 3.4 Exempt discovery files (`navigation.md`, `index.md`, `README.md`, `CODEBASE_STANDARDS.md`)
- [x] 3.5 Unit tests covering: valid card, missing reference, exempt discovery, exempt large file

## 4. Context Resolver (CTX-3)

- [x] 4.1 Define `Glob` trait: `fn exists(&self, path: &Path) -> bool`
- [x] 4.2 Implement `resolve(local, global, custom_dir, glob) -> CoreRoot` pure function (≤ 2 checks)
- [x] 4.3 Implement production `FsGlob` impl calling `Path::exists`
- [x] 4.4 Unit tests for all 5 rows of the resolution matrix (context-spec §7.1)
- [x] 4.5 Assert max 2 glob calls in each test scenario

## 5. @-Reference Validator (CTX-7)

- [x] 5.1 Implement scanner: extract `@` tokens from markdown text
- [x] 5.2 Allowlist: `@.opencode/context/...`, `@AGENTS.md`, `@.cursorrules`, `@$N`, email/mailto
- [x] 5.3 Reject dynamic references `@${var}` → `CTX-209`
- [x] 5.4 Reject non-standard references `@other` → `CTX-210`
- [x] 5.5 Unit tests for valid and invalid reference cases from context-spec §7.4

## 6. External Context Cache (CTX-8)

- [x] 6.1 Implement `manifest.json` read/write for `.tmp/external-context/`
- [x] 6.2 Implement `add(doc_id, content, metadata)` — store file + SHA256 + update manifest
- [x] 6.3 Implement `update(doc_id, content)` — overwrite if changed, bump version
- [x] 6.4 Implement `list() -> Vec<CachedDoc>` — return all entries
- [x] 6.5 Implement `remove(doc_id)` — delete file + manifest entry
- [x] 6.6 Unit tests for add/update/list/remove operations

## 7. Wizard (CTX-5, CTX-6)

- [x] 7.1 Implement 6-question interactive prompt using `dialoguer`
- [x] 7.2 Generate `project-intelligence/technical-domain.md` from answers
- [x] 7.3 Update `navigation.md` (Quick Routes or Deep Dives table) with new file
- [x] 7.4 Implement `--update` mode: minor/major version bump + date refresh
- [x] 7.5 Non-interactive mode: error with guidance message, exit 1
- [x] 7.6 Unit tests: generated file passes frontmatter + MVI validation

## 8. CLI Integration

- [x] 8.1 Add `validate --context` subcommand to `src/main.rs`
- [x] 8.2 Add `wizard add-context` subcommand to `src/main.rs`
- [x] 8.3 Wire `--update` flag into the wizard subcommand
- [x] 8.4 Ensure `install` step copies `content/context/` correctly (CTX-4)
- [x] 8.5 Integration test: run binary on temp scaffolded project, assert exit codes

## 9. Walk Tests

- [x] 9.1 Create `tests/context_walk.rs` (or extend existing walk test)
- [x] 9.2 Validate structure of `content/context/` tree (directories exist)
- [x] 9.3 Validate frontmatter on every context file (allowlist documented deviations)
- [x] 9.4 Validate MVI constraints on concept cards
- [x] 9.5 Walk test passes on the real `content/context/` tree (no external checkout)

## 10. Polish

- [x] 10.1 `cargo clippy -- -D warnings` clean
- [x] 10.2 `cargo fmt --check` clean
- [x] 10.3 `cargo test` green (all unit + integration + walk tests)
