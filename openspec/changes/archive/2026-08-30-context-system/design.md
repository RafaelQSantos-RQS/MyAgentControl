## Context

The `src/install/` module is complete: the interactive installer (TUI), `add`, `remove`, and `status` commands work against the real embedded registry and `.mac/manifest.json`. The vendored `content/context/` tree (442 files total, context subset is a portion) is the in-repo source of truth (C6). The context-spec (MAC-CTX, v0.0.2) already defines all 8 functional requirements (CTX-1 through CTX-8) with acceptance criteria. No `src/context/` module exists yet; the `lib.rs` exposes only `pub mod install`.

## Goals / Non-Goals

**Goals:**
- Implement all 8 CTX requirements as a self-contained `src/context/` module
- Provide a pure-function resolver testable without filesystem access
- Walk-test the real `content/context/` tree (always-on, D8)
- Wire `validate --context` and `wizard add-context` into the CLI

**Non-Goals:**
- Modifying the install module (CTX-4 integration is minimal: ensure the copy step includes `context/`)
- Implementing `evals` or other non-context modules
- Changing the registry or manifest logic

## Decisions

### 1. HTML-comment parser: hand-rolled, not regex

Context frontmatter is `<!-- Context: ... | Priority: ... | Version: ... | Updated: ... -->`. A simple split-on-`|` parser with trim is sufficient and avoids pulling in a regex crate. The format is stable (OAC-declared) and narrow.

**Alternative considered:** `regex` crate. Rejected: overkill for a fixed four-field format, adds compile time.

### 2. Resolver: trait-based glob injection

The context resolution logic (local-first, global-fallback, ≤ 2 checks) needs to call `glob()` but should not hit the filesystem in unit tests. Define a `Glob` trait with a single method `fn exists(&self, path: &Path) -> bool`; the production impl calls `std::path::Path::exists`, the test impl uses a `HashSet<PathBuf>`.

**Alternative considered:** Temp directory in tests. Rejected: slower, does not test the "max 2 checks" constraint cleanly.

### 3. MVI validator: line-count + heading scan

Concept card validation checks two things: file line count (< 200) and presence of a reference heading. Scan lines once: count total lines, collect headings. Single pass, O(n).

### 4. @-reference validator: string scan, not AST

Agent and command files are markdown. The `@`-reference check scans for `@` characters, extracts the token, and matches against the allowlist. No full markdown parser needed.

### 5. Wizard: interactive TUI using `dialoguer`

The existing `src/install/ui.rs` already uses `dialoguer` for the installer TUI. The `/add-context` wizard reuses the same pattern: 6 prompted questions, file write, `navigation.md` update.

### 6. External context cache: JSON manifest + directory

`.tmp/external-context/` stores cached docs. A `manifest.json` maps doc IDs to SHA256 + metadata. The cache module provides add/update/list/remove operations that read/write this manifest.

## Risks / Trade-offs

- **[Frontmatter drift]** → The parser stays strict per spec. Documented deviations in the vendored tree are tracked in walk tests (allowlist). Genuine defects are fixed in the tree (C16 policy).
- **[Global fallback complexity]** → The ≤ 2 glob check constraint limits the fallback to a simple exists/not-exists decision. No recursion, no caching.
- **[Wizard TTY requirement]** → Non-interactive mode errors with guidance. Acceptable for a config manager; the `--yes` default flag is deferred to post-v1.
- **[Cache invalidation]** → The external context cache uses SHA256 to detect changes. Stale docs are the user's responsibility; the cache is a convenience, not a sync engine.
