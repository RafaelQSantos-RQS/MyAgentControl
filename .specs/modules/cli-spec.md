---
id: MAC-CLI
type: module-spec
parent: MAC-MASTER
title: CLI Binary — Module Spec
status: approved
version: 0.2.0
updated: 2026-08-02
change_requests: [CR-001]
depends_on: [MAC-MASTER, MAC-CTX, MAC-AG, MAC-SK, MAC-CMD, MAC-EV]
---

# CLI Binary — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 0.2.0 |
| **Change Request** | [CR-001](../changes/CR-001-cr.md) — `init` copies vendored `content/` (Model C) |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | `Cargo.toml` (edition 2024), original `bin/oac.js`, `scripts/` |

---

## 1. Purpose

`myagentcontrol` is a single Rust binary (crate `myagentcontrol`, edition 2024) that **manages** the `.opencode/`-compatible configuration: it scaffolds, validates, lists, and interactively generates agents, skills, commands, context, and evals. It does **not** execute agents or call the OpenCode CLI at runtime (except the optional `evals run` subcommand — see [`evals-spec.md`](./evals-spec.md)).

## 2. Binary Name & Distribution

- Crate/binary name: `myagentcontrol` (user decision, R4Q2).
- Install via `cargo install --path .` (v1). No npm/homebrew packaging in v1.

## 3. Command Surface

```
myagentcontrol init [--dir .opencode] [--force?]
myagentcontrol validate [--agents|--skills|--commands|--context|--evals|--registry|--all]
myagentcontrol list <agents|skills|commands|context|evals> [--format table|json]
myagentcontrol wizard agent|skill|command new          # interactive generators
myagentcontrol wizard add-context [--update]           # 6-question Project Intelligence wizard
myagentcontrol evals validate|dashboard        # (evals run is deferred to post-v1 — see evals-spec D1)
myagentcontrol import <path-to-oac> [--dry-run]        # import an existing .opencode/ tree
myagentcontrol export <path>                           # export managed tree to a target dir
myagentcontrol doctor                                 # check environment (paths, opencode present, structure)
```

## 4. Behavioral Requirements

- **CLI-1** `init` **copies** the vendored `content/` tree (repo top-level, source of truth per CR-001) to `.opencode/` — **non-destructively and idempotently**: existing files are never overwritten unless `--force`; re-running produces identical results (NFR2 determinism). No template generation.
- **CLI-2** `validate` runs module validators (per the module specs) and reports grouped, actionable errors with exit code 1 on any failure; `--all` is the default. Exit 0 = pristine.
- **CLI-3** `list` renders a table (default) or JSON (`--format json`) for machine consumption.
- **CLI-4** Wizards are interactive (prompted TTY flow) and produce spec-compliant files that immediately pass `validate`.
- **CLI-5** `import` reads an existing OAC tree (e.g. a checkout of the reference repo at tag `v0.7.1`) into managed state with `--dry-run` preview.
- **CLI-6** `doctor` checks: context root resolution (local/global/custom_dir), required dirs exist, opencode availability (only informational for `evals run`), and reports a summary.
- **CLI-7** Output is deterministic and stable (stable column ordering, no random colors that break parsing; `--no-color` flag).
- **CLI-8** `validate` covers the ancillary paths in §9 under `--all` (or `--registry` for registry.json alone), with per-path error grouping.

## 5. Dependencies (decision D11)

- **CLI parsing:** `clap` v4 (derive API — `Parser`, `Subcommand`, `Args`), verified current via context7 `find-docs` (Aug 2026).
- **YAML parsing (frontmatter/context):** **`serde-saphyr`** — NOT `serde_yaml` (archived/deprecated) nor `serde_yml` (transition shim); see master decision D11. Typed-only: permission maps deserialize as `HashMap<String, HashMap<String, Permission>>`; no `Value` DOM. Escape hatch: `noyalib` (feature `compat-serde-yaml`) if `Value` support is ever needed. Uses `serde_saphyr::from_str` / `to_string` (panic-free, budget limits, zero-copy).
- **Serialization:** `serde` + `serde_json` (registry, results, category JSONs).
- **Errors:** `thiserror` (typed errors, E100–E500).
- **Determinism:** no timestamps embedded by default (NFR2).

## 6. Crate-Level Architecture (proposed layout)

```
src/
├── main.rs                  # arg parsing + dispatch
├── cli/                     # clap definitions, output formatting (table/json)
├── core/                    # shared types, errors, paths
├── context/                 # CTX module (validate/scaffold/resolve/wizard)
├── agents/                  # AG module (schema, inventory, validate, wizard)
├── skills/                  # SK module
├── commands/                # CMD module
├── evals/                   # EV module (cases, results, dashboard html)
└── golden/                  # reference-tree golden tests (tests/ dir)
```

> **CR-001 (Model C):** parity content is **not** embedded templates. It lives in the repo-top-level **`content/`** dir (vendored OAC v0.7.1 tree, renamed from `.opencode/`), plus `NOTICE.md` + `LICENSE` for attribution (D7). `init` copies from `content/` to `.opencode/`.

## 7. Error Handling & UX

- Typed errors (`thiserror`) with error codes: `E100` parse, `E200` schema, `E300` reference/dangling, `E400` io, `E500` internal. These are the **category envelopes**.
- **Two-tier error scheme:** each module spec defines its own rule IDs (`XX-2xx`, e.g. `CTX-201`, `AG-202`, `SK-204`, `CMD-201`) which map into the E-envelope by kind — schema defects → `E200`, dangling references → `E300`. The envelope is the reported prefix, the rule ID names the specific violation (see §10.3 example: `E200 [agents] … rule: AG-202`).
- Every validation error includes: file path, line/col where available, rule ID (e.g. `CTX-201`), and a suggestion.
- Non-interactive mode: when stdin is not a TTY, wizards error out with guidance (or accept `--yes` defaults).

## 8. Testing & Validation Strategy

- **Unit tests**: pure functions (frontmatter parsing, MVI line checks, context resolution matrix, permission-map validation, dashboard HTML generation).
- **Integration tests**: run the binary on a temp scaffolded project; assert exit codes and output.
- **Golden tests (D8)**: diff the vendored `content/` tree against the **external pinned checkout** of [`darrenhinde/OpenAgentsControl`](https://github.com/darrenhinde/OpenAgentsControl) at tag `v0.7.1` (with `Updated` dates and result artifacts normalized) — **non-self-referential** (CR-001 point 3). Adopted-PR deltas become approved exceptions via managed divergence (CR-001 point 5); re-baseline per D8. Golden fixtures are committed under `tests/golden/`; the Phase 0 fixture slice is kept as a fast unit fixture.
- **Lints**: `cargo clippy -- -D warnings` and `cargo fmt --check` in CI.
- **CI (future)**: GitHub Actions matrix (linux/macos/windows), `cargo test` + clippy.

## 9. Ancillary Structure Coverage (profiles, prompts, tool, plugin, scripts, registry)

These appear in the reference `.opencode/` tree but have no dedicated module spec. Scope (default per master OQ1: validate + scaffold):

| Path | Managed scope |
|---|---|
| `profiles/` (advanced, business, developer, essential, full) | Scaffold folder structure; validate that each profile references existing agents/commands/context |
| `prompts/` (core, development, data, content) | Scaffold; validate frontmatter/links if present |
| `tool/` (index.ts, package.json, tsconfig) | Scaffold as-is (managed source artifacts — not executed by Rust, per NFR5) |
| `plugin/` (agent-validator.ts, notify.ts, package.json) | Scaffold as-is (managed source artifacts) |
| `scripts/` (validation, registry, versioning, bridge, external-context, maintenance, tests) | Scaffold as-is; validate referenced script paths exist |
| `registry.json` | Validate schema: referenced models/agents/commands/skills resolve to real files; mirrors original `scripts/registry/validate-registry.ts` behavior |

## 10. Examples & Scenarios

### 10.1 Happy path

```
$ myagentcontrol init
✔ Scaffolded .opencode/ (451 files, 13 errors fixed by validate)
$ myagentcontrol validate --all
✔ All modules pass
$ myagentcontrol list agents --format json
[{"name":"OpenAgent","mode":"primary","category":"core"}, ...]
```

### 10.2 Idempotency

`init` → user edits `opencoder.md` → `init` again → user edit **survives**; no overwrite.

### 10.3 Error output shape

```
E200 [agents] .opencode/agent/core/opencoder.md:12
  rule: AG-202 — permission verb "allow all" not in {allow, ask, deny}
  hint: use one of: allow, ask, deny
```

## 11. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-L1** Given a fresh project, **when** `myagentcontrol init && myagentcontrol validate --all` runs, **then** it exits 0 (self-consistent).
- **AC-L2** Given a scaffolded tree, **when** `init` runs a second time after a user edit, **then** the tree is identical except the user edit is preserved (no overwrite).
- **AC-L3** Given the reference repo, **when** golden diff tests run for all modules, **then** they pass.
- **AC-L4** Given a valid run, **when** `list --format json` emits, **then** the output validates against a committed JSON schema.
- **AC-L5** Given the codebase, **when** `cargo test`, `cargo clippy -D warnings`, and `cargo fmt --check` run, **then** all pass with zero warnings.
- **AC-L6** Given the built binary, **when** it runs on a machine without node/bun, **then** it works (NFR5).
- **AC-L7** Given a `registry.json` with an entry pointing to a nonexistent agent/model/command/skill, **when** `validate --registry` runs, **then** it fails, naming the broken entry.
- **AC-L8** Given a scaffolded tree, **when** golden tests run on the `profiles/`, `prompts/`, `tool/`, `plugin/`, `scripts/` trees, **then** the diff is clean vs the reference (source artifacts verbatim).

## 12. Cross-References

- Module validators implemented per: [`context-spec.md`](./context-spec.md), [`agents-spec.md`](./agents-spec.md), [`skills-spec.md`](./skills-spec.md), [`commands-spec.md`](./commands-spec.md), [`evals-spec.md`](./evals-spec.md)
- Master decisions D1–D11 (incl. D11: `serde-saphyr` over deprecated `serde_yaml`) → [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md)
