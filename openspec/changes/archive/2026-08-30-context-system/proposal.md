## Why

The context system is the foundation of the agent tree: it stores project coding standards and patterns as markdown files, loaded by agents before code generation using the MVI (Minimal Viable Information) principle. Without the context module, the installed tree has no validation, no resolution logic, and no interactive wizard for adding context files. The master spec (MAC-MASTER) roadmap lists Phase 1 as the context system, and the existing `.specs/modules/context-spec.md` (MAC-CTX, v0.0.2) already defines all functional requirements. The install module is done; the context module is the next logical step.

## What Changes

- Implement HTML-comment frontmatter parsing and validation (CTX-1)
- Validate MVI constraints: file size, required reference sections on concept cards (CTX-2)
- Implement local-first / global-fallback context resolution as a pure function (CTX-3)
- Integrate `install` to copy `content/context/` tree correctly (CTX-4)
- Implement `/add-context` wizard: 6-question Project Intelligence flow → `project-intelligence/technical-domain.md` (CTX-5)
- Implement `--update` mode: increment version, refresh date (CTX-6)
- Validate `@`-reference syntax in agent/command files (CTX-7)
- Implement external context cache under `.tmp/external-context/` (CTX-8)
- Add `validate --context` subcommand to the CLI
- Add walk tests for the `content/context/` tree

## Capabilities

### New Capabilities

- `context-system`: Frontmatter parsing, MVI validation, context resolution, `/add-context` wizard, `--update` mode, `@`-reference validation, external context cache.

### Modified Capabilities

- (none — this is a new module; no existing spec requirements change)

## Impact

- New `src/context/` module (frontmatter parser, MVI validator, resolver, wizard, cache)
- CLI additions: `validate --context`, `wizard add-context`, `--update` flag
- New dependencies: none beyond what's already in `Cargo.toml` (no YAML parser needed; context uses HTML-comment frontmatter, not YAML)
- Walk tests in `tests/context_walk.rs` validate the real `content/context/` tree
