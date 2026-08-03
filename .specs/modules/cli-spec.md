---
id: MAC-CLI
type: module-spec
parent: MAC-MASTER
title: CLI Binary — Module Spec
status: approved
version: 0.0.3
updated: 2026-08-03
change_requests: []
depends_on: [MAC-MASTER, MAC-CTX, MAC-AG, MAC-SK, MAC-CMD, MAC-EV, MAC-REG]
---

# CLI Binary — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 0.0.3 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | `Cargo.toml` (edition 2024), vendored `content/` tree, original `bin/oac.js` (repo-root, reference only) |
| **Note** | Rewritten 2026-08-03 under the format-fidelity principle (C16). `validate` gates on format-declared rules; walk tests validate structure + frontmatter (no navigation cross-reference validation, see context-spec). 0.0.3: command renamed `init` → **`install`** (user decision, Brick 1) — the interactive installer mirroring OAC `install.sh`; `--registry <path>` flag added |

---

## 1. Purpose

`myagentcontrol` is a single Rust binary (crate `myagentcontrol`, edition 2024) that **manages** the `.opencode/`-compatible configuration: it scaffolds (copies from vendored `content/`), validates, lists, and interactively generates agents, skills, commands, context, and evals. It does **not** execute agents or call the OpenCode CLI at runtime (except the optional `evals run` subcommand; see [`evals-spec.md`](./evals-spec.md)).

## 2. Binary Name & Distribution

- Crate/binary name: `myagentcontrol`.
- Install via `cargo install --path .` (v1). No npm/homebrew packaging in v1.

## 3. Command Surface

```
myagentcontrol install [--dir .opencode] [--registry <path>] [--force] [--profile <name>]
myagentcontrol validate [--agents|--skills|--commands|--context|--evals|--registry|--all]
myagentcontrol list <agents|skills|commands|context|evals|registry> [--format table|json]
myagentcontrol status                                   # manifest vs disk diff (modified/added/removed)
myagentcontrol add <type>:<id>                          # install a component + dependencies (registry)
myagentcontrol remove <type>:<id>                       # remove a component + manifest entry
myagentcontrol update [--check] [--force]               # apply bundle changes, preserve user-modified
myagentcontrol version bump <alpha|beta|rc|patch|minor|major>   # project-level version bump
myagentcontrol cleanup sessions [--hours 24]            # remove stale .tmp/sessions
myagentcontrol wizard agent|skill|command new           # interactive generators
myagentcontrol wizard add-context [--update]            # 6-question Project Intelligence wizard
myagentcontrol evals validate|dashboard                 # (evals run is deferred to post-v1; see evals-spec)
myagentcontrol import <path-to-oac> [--dry-run]         # import an existing .opencode/ tree
myagentcontrol export <path>                            # export managed tree to a target dir
myagentcontrol doctor                                   # check environment (paths, opencode present, structure, manifest)
```

> **Not in v1 (explicitly excluded, master §3):** IDE-specific config generation
> (`apply` → `.cursorrules`/`CLAUDE.md`/`.windsurfrules`) and Claude Code
> integration. The tool is model/IDE-agnostic by constitution C5.

## 4. Behavioral Requirements

Markers (C16): `[OAC format]` validates a rule the managed format declares; `[tool DX]` is a user-approved developer-experience feature.

- **CLI-1** `[OAC format]` `install` is the **interactive installer** (mirroring OAC `install.sh`): banner → location → mode → profile/custom → preview → confirm. Confirmed installs **copy** the selected components from the vendored `content/` tree (source of truth per C6) to `.opencode/`, **non-destructively and idempotently**: existing files are never overwritten unless `--force`; re-running produces identical results (NFR2 determinism). No template generation. `--profile <name>` installs the profile's component set per registry-spec REG-10. **Brick 1 status:** the interactive flow (banner/location/mode/profile/custom/preview/confirm) is implemented over the real registry; the actual copy/collision/manifest step is a placeholder until Brick 2.
- **CLI-2** `[tool DX]` `validate` runs module validators (per the module specs) and reports grouped, actionable errors with exit code 1 on any failure; `--all` is the default. Exit 0 = pristine.
- **CLI-3** `[tool DX]` `list` renders a table (default) or JSON (`--format json`) for machine consumption.
- **CLI-4** `[tool DX]` Wizards are interactive (prompted TTY flow) and produce spec-compliant files that immediately pass `validate`.
- **CLI-5** `[tool DX]` `import` reads an existing OAC tree (e.g. a checkout of the reference repo at tag `v0.7.1`, or any `.opencode/`-style tree) into managed state with `--dry-run` preview.
- **CLI-6** `[tool DX]` `doctor` checks: context root resolution (local/global/custom_dir), required dirs exist, registry/manifest validity, opencode availability (only informational for `evals run`), and reports a summary.
- **CLI-7** `[tool DX]` Output is deterministic and stable (stable column ordering, no random colors that break parsing; `--no-color` flag).
- **CLI-8** `[tool DX]` `validate` covers the ancillary paths in §9 under `--all`, with per-path error grouping.
- **CLI-9** `[tool DX]` `add <type>:<id>` / `remove <type>:<id>` delegate to registry-spec REG-6/REG-10; non-destructive with collision reporting.
- **CLI-10** `[tool DX]` `status` reads the manifest and diffs hashes against disk (registry-spec REG-8).
- **CLI-11** `[tool DX]` `version bump <stage>` bumps the project version file (alpha/beta/rc/patch/minor/major), mirroring the original `bump-version.sh`; requires a `VERSION` file or equivalent in the project.
- **CLI-12** `[tool DX]` `cleanup sessions [--hours N]` removes stale session directories under `.tmp/sessions/` older than N hours (default 24), mirroring `cleanup-stale-sessions.sh`.
- **CLI-13** `[tool DX]` `update` delegates to registry-spec REG-9: apply bundle changes, preserve user-modified files (backup + report), `--check` = dry-run.

## 5. Dependencies (decision D11)

- **CLI parsing:** `clap` v4 (derive API: `Parser`, `Subcommand`, `Args`), verified current via context7 `find-docs` (Aug 2026).
- **YAML parsing (frontmatter/context):** **`serde-saphyr`**, not `serde_yaml` (archived/deprecated) nor `serde_yml` (transition shim); see master decision D11. Typed-only: permission maps deserialize as `HashMap<String, HashMap<String, Permission>>`; no `Value` DOM. Escape hatch: `noyalib` (feature `compat-serde-yaml`) if `Value` support is ever needed.
- **Serialization:** `serde` + `serde_json` (registry, results, category JSONs).
- **Errors:** `thiserror` (typed errors, E100–E600).
- **Hashing:** `sha256` computed via a small pure function (no heavy dependency; `sha2` crate or std-only implementation, decision left to implementation).
- **Determinism:** no timestamps embedded by default (NFR2).

## 6. Crate-Level Architecture (current layout)

```
src/
├── main.rs                  # arg parsing + dispatch
├── install/                 # interactive installer (Brick 1: TUI; copy lands Brick 2+)
├── cli/                     # clap definitions, output formatting (table/json)
├── core/                    # shared types, errors (E100–E600), tree helpers (golden.rs)
├── context/                 # CTX module (validate/scaffold/resolve/wizard)
├── agents/                  # AG module (schema, inventory, validate, wizard)
├── skills/                  # SK module (frontmatter validation)
├── commands/                # CMD module
├── registry/                # REG module (registry.json, manifest, add/status/update)
└── evals/                   # EV module (cases, results, dashboard html)
```

> **C6 (distribution):** the managed tree is the vendored **`content/`** dir (440 files, OAC v0.7.1 as starting point) + `NOTICE.md` + `LICENSE`. `install` copies `content/` → `.opencode/`. `src/core/golden.rs` provides tree helpers (path collection, copying) used by the always-on walk tests in `tests/`.

## 7. Error Handling & UX

- **E100** parse, **E200** schema, **E300** reference/dangling, **E400** io, **E500** internal, **E600** registry/install state. These are the **category envelopes**.
- **Two-tier error scheme:** each module spec defines its own rule IDs (`XX-2xx`, e.g. `CTX-201`, `AG-202`, `SK-204`, `CMD-201`) which map into the E-envelope by kind: schema defects → `E200`, dangling references → `E300`. The envelope is the reported prefix, the rule ID names the specific violation (see §10.3 example: `E200 [agents] … rule: AG-202`).
- Every validation error includes: file path, line/col where available, rule ID (e.g. `CTX-201`), and a suggestion.
- Non-interactive mode: when stdin is not a TTY, wizards error out with guidance (or accept `--yes` defaults). For the interactive installer, a non-TTY invocation and prompt/interaction failures are **guidance errors** (no E-envelope): they report a plain message and exit 1, per cli-spec §10.4.

## 8. Testing & Validation Strategy

- **Unit tests**: pure functions (frontmatter parsing, MVI line checks, context resolution matrix, permission-map validation, dashboard HTML generation).
- **Integration tests**: run the binary on a temp scaffolded project; assert exit codes and output.
- **Walk tests (D8, NFR1)**: `tests/*_walk.rs` validate the **real `content/` tree** (structure, frontmatter, inventories) and **always run**: no external checkout, no gitignored dev artifact, no skip logic. Tree helpers come from `src/core/golden.rs` (path collection, copying).
- **Lints**: `cargo clippy -- -D warnings` and `cargo fmt --check` in CI.
- **CI (future)**: GitHub Actions matrix (linux/macos/windows), `cargo test` + clippy.

## 9. Ancillary Structure Coverage (profiles, prompts, tool, plugin, plugins, docs, scripts, config)

These appear in the vendored `content/` tree but have no dedicated module spec. Scope (default per master OQ1: validate + scaffold):

| Path | Managed scope |
|---|---|
| `profiles/` (advanced, business, developer, essential, full) | Scaffold folder structure; validate that each profile references existing agents/commands/context |
| `prompts/` (core, content, data, development) | Scaffold; validate frontmatter/links if present |
| `tool/` (index.ts, package.json, tsconfig, env/, template/, gemini/) | Scaffold as-is (managed source artifacts, not executed by Rust, per NFR5) |
| `plugin/` (agent-validator.ts, notify.ts, tests/, docs/, package.json) | Scaffold as-is (managed source artifacts) |
| `plugins/` (coder-verification/) | Scaffold as-is |
| `docs/` (agents/, guides/, workflows/) | Scaffold as-is |
| `scripts/` (task-cli.ts) | Scaffold as-is; validate referenced script paths exist |
| `config.json`, `opencode.json`, `config/agent-metadata.json` | Scaffold as-is; validate well-formed JSON |

> **Managed scope:** reference checks in this table (profiles → agents/commands/context, scripts → referenced paths) are `[tool DX]` consistency checks under C16; the vendored tree satisfies them.
>
> **Registry:** `registry.json` is **managed** since 2026-08-03 (vendored in `content/`, validated per [`registry-spec.md`](./registry-spec.md)).
>
> **Not managed:** OAC repo-root `evals/`, `bin/oac.js`, `install.sh`, `update.sh` are **out of scope** (master §6.5). `import` may bring them into a user project, but `validate` does not require them.

## 10. Examples & Scenarios

### 10.1 Happy path

```
$ myagentcontrol install
✔ Scaffolded .opencode/ (440 files)
$ myagentcontrol validate --all
✔ All modules pass
$ myagentcontrol list agents --format json
[{"name":"OpenAgent","mode":"primary","category":"core"}, ...]
```

### 10.2 Idempotency

`install` → user edits `opencoder.md` → `install` again → user edit **survives**; no overwrite.

### 10.3 Error output shape

```
E200 [agents] .opencode/agent/core/opencoder.md:12
  rule: AG-202 — permission verb "allow all" not in {allow, ask, deny}
  hint: use one of: allow, ask, deny
```

### 10.4 Non-interactive / prompt guidance (installer)

```
$ myagentcontrol install < /dev/null
Error: interactive installer requires a terminal; run from a TTY
$ echo y | myagentcontrol install
Error: interactive installer requires a terminal; run from a TTY
```

These are guidance errors (plain message, exit 1) — **no E-envelope** (see §7).

## 11. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-L1** Given a fresh project, **when** `myagentcontrol install && myagentcontrol validate --all` runs, **then** it exits 0 (self-consistent).
- **AC-L2** Given a scaffolded tree, **when** `install` runs a second time after a user edit, **then** the tree is identical except the user edit is preserved (no overwrite).
- **AC-L3** Given the vendored `content/` tree, **when** the walk tests run, **then** they pass for all managed modules (always-on, no external checkout).
- **AC-L4** Given a valid run, **when** `list --format json` emits, **then** the output validates against a committed JSON schema.
- **AC-L5** Given the codebase, **when** `cargo test`, `cargo clippy -D warnings`, and `cargo fmt --check` run, **then** all pass with zero warnings.
- **AC-L6** Given the built binary, **when** it runs on a machine without node/bun, **then** it works (NFR5).
- **AC-L7** Given a `config.json`/`opencode.json` with invalid JSON or a profile referencing a nonexistent agent, **when** `validate --all` runs, **then** it fails, naming the broken entry.
- **AC-L8** Given the scaffolded tree, **when** the ancillary walk test runs on `profiles/`, `prompts/`, `tool/`, `plugin/`, `plugins/`, `docs/`, `scripts/` trees, **then** the structure is validated (source artifacts verbatim).
- **AC-L9** Given an installed tree with a user-modified file, **when** `status` and `update --check` run, **then** the modified file is reported (never silently overwritten); **when** `add context:<id>` runs for a valid registry component, **then** the component and its dependencies install and the manifest records them.

## 12. Cross-References

- Module validators implemented per: [`context-spec.md`](./context-spec.md), [`agents-spec.md`](./agents-spec.md), [`skills-spec.md`](./skills-spec.md), [`commands-spec.md`](./commands-spec.md), [`evals-spec.md`](./evals-spec.md), [`registry-spec.md`](./registry-spec.md)
- Master decisions D1–D11 (incl. D11: `serde-saphyr` over deprecated `serde_yaml`) → [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md)
