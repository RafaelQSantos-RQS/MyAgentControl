---
description: "Implementation plan for feature SPEC-XXX — [FEATURE NAME]"
---

# Implementation Plan: [FEATURE]

**Branch**: `SPEC-XXX-<feature-name>` | **Date**: [DATE] | **Spec**: [`spec.md`](./spec.md)
**Input**: Feature specification from `.specs/features/SPEC-XXX-<feature-name>/spec.md`
**Note**: This file is written after `spec.md` reaches `approved` status (constitution C7) — plan the *how*, never re-open the *what*.

## Summary

[Extract from the feature spec: primary requirement + technical approach. One paragraph.]

## Technical Context

<!--
ACTION REQUIRED: Replace with the concrete technical details for this feature.
Defaults below come from the project's master spec and cli-spec — adjust only with justification.
-->

**Language/Version**: Rust (edition 2024), single binary crate `myagentcontrol`
**Primary Dependencies**: `clap` v4 (derive API), `serde-saphyr`, `serde` + `serde_json`, `thiserror` (see [`../../modules/cli-spec.md`](../../modules/cli-spec.md) §5 / decision D11) + any feature-specific crates
**Storage**: `.opencode/`-compatible markdown + YAML frontmatter tree (managed artifacts; no DB)
**Testing**: `cargo test` (unit + integration + golden), `cargo clippy -- -D warnings`, `cargo fmt --check` (constitution C12 / NFR4)
**Target Platform**: Linux/macOS/Windows (CI matrix)
**Project Type**: CLI — configuration manager (constitution C3: never executes agents or calls model APIs)
**Performance Goals**: `validate` < 2s, `list` < 500ms (NFR3)
**Constraints**: deterministic output (NFR2), no node/bun at runtime (NFR5), non-destructive/idempotent (NFR6)
**Scale/Scope**: OAC reference repo (tag `v0.7.1`) tree (~450 files, golden-tested per D8)

## Constitution Check

*GATE: must pass before Phase 0 research. Re-check after Phase 1 design.*

- [ ] **C3** — feature stays within "configuration manager" scope (no agent execution, no model API calls)
- [ ] **C5** — model-agnostic: no single-vendor assumption
- [ ] **C6** — feature parity with reference repo, machine-checked via golden tests
- [ ] **C7** — spec is `approved` before any implementation begins
- [ ] **C10** — acceptance criteria remain Given/When/Then and objectively verifiable
- [ ] **C12** — `cargo test` green, clippy clean, fmt clean, zero `unsafe`
- [ ] **C15** — spec-first: any behavior change updates the spec before the code

## Project Structure

### Documentation (this feature)

```text
.specs/features/SPEC-XXX-<feature-name>/
├── spec.md        # this feature's spec (approved)
├── plan.md        # this file (the "how")
└── tasks.md       # task breakdown (created after plan approval)
```

### Source Code (repository root)

<!--
ACTION REQUIRED: Replace with the concrete layout for this feature, extending the
crate skeleton from cli-spec §6. Delete unused options before delivering.
-->

```text
# [REMOVE IF UNUSED] Option 1: Extend existing module (DEFAULT)
src/<module>/
├── <feature>.rs            # new logic
└── mod.rs                  # wiring
tests/
└── <feature>_test.rs

# [REMOVE IF UNUSED] Option 2: New module (only if justified in Complexity Tracking)
src/<new-module>/
├── mod.rs
├── <feature>.rs
└── ...
tests/
└── ...
```

**Structure Decision**: [Document the selected structure and reference the real directories above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., new crate / new module] | [current need] | [why the simpler option is insufficient] |
