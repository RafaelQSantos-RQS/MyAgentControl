---
id: MAC-MASTER
type: master-spec
title: MyAgentControl — Master Spec
status: approved
version: 0.0.3
updated: 2026-08-03
change_requests: []
owner: Rafael (user)
depends_on: []
language: en
license: MIT + attribution to OpenAgentsControl
---

# MyAgentControl — Master Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 0.0.3 |
| **Updated** | 2026-08-03 |
| **Owner** | Rafael (user) |
| **Language** | English (per decision) |
| **License** | MIT + attribution to OpenAgentsControl |
| **Note** | Rebuilt 2026-08-02 from scratch; OAC v0.7.1 is the *starting point*, not a parity yardstick (constitution C6). Rewritten 2026-08-03 under the format-fidelity principle (C16): the tool validates only OAC-declared rules. 2026-08-03: incorporated the OAC feature inventory (registry, manifest, ref-syntax, external-context, version/cleanup). 0.0.3: the installer command is **`install`** (user decision), replacing the provisional `init` |

---

## 1. Background & Motivation

**OpenAgentsControl (OAC)** (<https://github.com/darrenhinde/OpenAgentsControl>) is a model-agnostic AI agent framework built on top of the OpenCode CLI. It ships as markdown files (agents, subagents, skills, commands, context, eval cases) that teach AI agents a project's coding patterns, enforce plan-first workflows with approval gates, and reduce token usage via a Minimal Viable Information (MVI) context system. The maintainer has slowed down; many high-quality community PRs remain unmerged.

The user, a Rust enthusiast who uses many different AI models, wants a **Rust reimplementation** in the spirit of the framework: same concepts, same file formats, same workflow, but implemented as a Rust tool, and evolved as **their own vision** of OAC. OAC v0.7.1 is the **starting point**: its tree is vendored once into `content/` and then maintained in this repository (adopted PRs and project-specific improvements), **never re-fetched** from upstream (constitution C6).

## 2. Goals

1. **Full content coverage**: all core agents, subagents, skills, commands, and the context system are present and managed from the vendored `content/` tree (440 files, OAC v0.7.1 as starting point).
2. **Model-agnostic**: never tie the framework to one vendor. Execution happens via the OpenCode CLI (user's chosen backend), which is itself model-agnostic.
3. **Configuration manager, not a runtime**: the Rust binary copies the vendored `content/` tree (source of truth, C6), validates, and maintains the `.opencode/`-compatible structure. It does **not** invoke OpenCode or any model API itself.
4. **Self-consistent and machine-verifiable**: walk tests validate the **real `content/` tree** (structure, frontmatter) and always run; no external checkout required (D8).
5. **Editable & transparent**: agents/skills/commands remain human-editable markdown with YAML frontmatter, exactly like the original.
6. **Token-efficient MVI context system**: lazy loading, files < 200 lines, local-first resolution.
7. **Developer-friendly CLI**: `myagentcontrol` binary with install/validate/list/wizard commands.
8. **Format fidelity**: the tool validates only rules the managed formats declare (OAC/OpenCode); it never invents integrity rules beyond the formats (C16).

## 3. Non-Goals (explicit out-of-scope for v1)

- ❌ Running agents, calling model APIs, or wrapping the OpenCode CLI as a subprocess. (A future `myagentcontrol evals run` subcommand that shells out to OpenCode is **deferred to post-v1**; see D1.)
- ❌ A TUI or chat interface.
- ❌ The Claude Code plugin distribution (`.claude-plugin/`, `plugins/claude-code/`); noted as reference only.
- ❌ Open PRs as features: they are intentionally **not** listed in this spec; tracked separately and spec'd individually when picked up (see §10 Post-v1).
- ❌ Cloud migration, agent marketplace/registry hosting, IDE integrations.
- ❌ IDE-specific config generation (`apply` → `.cursorrules`, `CLAUDE.md`, `.windsurfrules`) and Claude Code integration: the tool is model- and IDE-agnostic by constitution C5 (added 2026-08-03 from the OAC feature inventory).
- ❌ Re-fetching or re-syncing `content/` from the OAC upstream (C6); the tree is maintained in-repo.
- ❌ Inventing integrity rules the managed formats do not declare (for example, "every context file must be listed in a navigation file"). The tool validates only declared rules (C16).
- ❌ `evals/`, `registry.json`, `bin/oac.js`, `install.sh`, `update.sh` from the OAC **repo root**; only the `.opencode/` tree is vendored (see §6.5).

## 4. Core Concepts & Terminology

| Term | Meaning |
|---|---|
| **Agent** | A markdown file with YAML frontmatter describing an AI persona (`mode: primary` or `subagent`), temperature, and permission rules. |
| **Subagent** | A specialized agent invoked via the `task` tool by primary agents (ContextScout, TaskManager, etc.). |
| **Skill** | A self-contained capability folder: `SKILL.md` + `router.sh` (bash entrypoint) + optional scripts/workflows. |
| **Command** | A slash command (`/commit`, `/test`, …): a markdown file loaded into the agent context when invoked. |
| **Context** | Project coding standards/patterns as markdown, loaded before execution. MVI: < 200 lines, scannable in < 30s. |
| **MVI** | Minimal Viable Information: only load what's needed, when it's needed. |
| **Approval gate** | Rule that agents must request approval before write/edit/bash. |
| **Frontmatter** | YAML metadata block at the top of markdown files (`---` delimited) or HTML comment metadata for context files. |
| **paths.json** | Optional config that sets `custom_dir` context root and `global` fallback path. |

## 5. High-Level Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                    myagentcontrol (Rust binary)              │
│  install │ validate │ list │ wizard │ evals │ import │ export │
└───────────────┬──────────────────────────────────────────────┘
                │ copies from content/ (C6) · validates · maintains
                ▼
┌──────────────────────────────────────────────────────────────┐
│            .opencode/ (markdown + config, OAC-compatible)    │
│  agent/  subagents/  skills/  command/  context/  profiles/  │
│  prompts/  tool/  plugin/  plugins/  docs/  config.json      │
│  scripts/  config/                                           │
└──────────────────────────────────────────────────────────────┘
                │ read by (not invoked by us)
                ▼
┌──────────────────────────────────────────────────────────────┐
│                  OpenCode CLI (user's tool)                  │
│   model-agnostic: Claude │ GPT │ Gemini │ MiniMax │ local    │
└──────────────────────────────────────────────────────────────┘
```

**Key architectural decision (D1):** The Rust binary is a *manager* of the `.opencode/` structure. It does not execute agents. The OpenCode CLI (already installed and used by the user) is the execution backend. This keeps the rewrite scope tractable, keeps the user's existing workflows intact, and preserves model-agnosticism (OpenCode handles providers).

**Distribution (C6 / D8):** the full OAC-compatible tree is vendored under the repo-top-level **`content/`** dir (neutral name, reinforcing model-agnosticism), with `NOTICE.md` + `LICENSE` for attribution. `install` copies `content/` → `.opencode/` in the user's project; the destination name is unchanged for OpenCode drop-in compatibility.

## 6. Content Inventory (from vendored `content/`, OAC v0.7.1 as starting point)

### 6.1 Agents (`.opencode/agent/`)

| Location | Files |
|---|---|
| `agent/` (root) | `eval-runner.md` |
| `agent/core/` | `openagent.md`, `opencoder.md`, `0-category.json` |
| `agent/meta/` | `repo-manager.md`, `system-builder.md`, `0-category.json` |
| `agent/content/` | `copywriter.md`, `technical-writer.md`, `0-category.json` |
| `agent/data/` | `data-analyst.md`, `0-category.json` |
| `agent/subagents/core/` | `contextscout.md`, `externalscout.md`, `task-manager.md`, `batch-executor.md`, `context-manager.md`, `context-retriever.md`, `documentation.md`, `stage-orchestrator.md` |
| `agent/subagents/code/` | `coder-agent.md`, `test-engineer.md`, `reviewer.md`, `build-agent.md` |
| `agent/subagents/planning/` | `architecture-analyzer.md`, `adr-manager.md`, `contract-manager.md`, `prioritization-engine.md`, `story-mapper.md` |
| `agent/subagents/development/` | `frontend-specialist.md`, `devops-specialist.md`, `0-category.json` |
| `agent/subagents/system-builder/` | `agent-generator.md`, `command-creator.md`, `context-organizer.md`, `domain-analyzer.md`, `workflow-designer.md` |
| `agent/subagents/test/` | `simple-responder.md` |
| `agent/subagents/utils/` | `image-specialist.md` |

### 6.2 Skills (`content/skills/`, 4 skills, plural only)

- `skills/task-management/`: task CLI (`task-cli.ts`), JSON-driven task breakdown
- `skills/smart-router-skill/`: personality routing (yoda/stark/sherlock workflows)
- `skills/context7/`: external docs via Context7
- `skills/context-manager/`: context management

> The singular `skill/` tree from OAC v0.7.1 was **removed** by user decision (orphaned; the OpenCode runtime only loads `skills/<name>/SKILL.md`, plural). See `NOTICE.md`.

### 6.3 Commands (`.opencode/command/`)

`add-context.md`, `commit.md`, `commit-openagents.md`, `test.md`, `optimize.md`, `context.md`, `clean.md`, `analyze-patterns.md`, `validate-repo.md`, `worktrees.md`, `build-context-system.md`, `test-new-command.md`, plus `openagents/` and `prompt-engineering/` subfolders.

### 6.4 Context (`.opencode/context/`)

`core/`, `ui/`, `development/`, `project-intelligence/`, `product/`, `data/`, `learning/`, `content-creation/`, `system-builder-templates/`, `openagents-repo/`, `project/`, plus `navigation.md`, `index.md`, `CODEBASE_STANDARDS.md`.

### 6.5 Other managed paths (from `content/`; note: these differ from the OAC *repo root*)

| Path | Managed scope |
|---|---|
| `profiles/` | `advanced`, `business`, `developer`, `essential`, `full` |
| `prompts/` | `core/` (openagent, opencoder + per-model variants + results), `content/`, `data/`, `development/` |
| `tool/` | TypeScript tool barrel (`index.ts`, `package.json`, `tsconfig.json`, `env/`, `template/`, `gemini/`) |
| `plugin/` | `agent-validator.ts`, `notify.ts`, `tests/`, `docs/`, `package.json` |
| `plugins/` | `coder-verification/` |
| `docs/` | `agents/`, `guides/`, `workflows/` |
| `config.json`, `config/agent-metadata.json` | root config files |
| `scripts/` | `task-cli.ts` |
| `registry.json` | component registry (agents/subagents/commands/skills/contexts/tools/plugins/configs + profiles); validated per [`modules/registry-spec.md`](./modules/registry-spec.md) |

> **Not vendored:** OAC repo-root artifacts `evals/`, `bin/oac.js`, `install.sh`, `update.sh` are **out of scope**; only the `.opencode/` tree plus `registry.json` are vendored. (`evals/` handling for user projects is covered by [`modules/evals-spec.md`](./modules/evals-spec.md); `registry.json` was added to the managed tree on 2026-08-03.)

## 7. Cross-References

| Topic | Module spec |
|---|---|
| Context system, MVI, local-first resolution, wizard | [`modules/context-spec.md`](./modules/context-spec.md) |
| Agent/subagent schema, permissions, delegation graph | [`modules/agents-spec.md`](./modules/agents-spec.md) |
| Skills structure, router.sh, workflows | [`modules/skills-spec.md`](./modules/skills-spec.md) |
| Slash commands | [`modules/commands-spec.md`](./modules/commands-spec.md) |
| Eval framework, YAML cases, dashboard | [`modules/evals-spec.md`](./modules/evals-spec.md) |
| Component registry, install state (manifest), profiles, add/update | [`modules/registry-spec.md`](./modules/registry-spec.md) |
| The Rust CLI binary | [`modules/cli-spec.md`](./modules/cli-spec.md) |

## 8. Technical Decisions (ADR-style, MADR format)

> Format per [MADR](https://adr.github.io/madr/): **Context → Decision → Consequences**. Changes to accepted decisions require a Change Request (see README §Change Requests).

| ID | Status | Context / Decision | Consequences |
|---|---|---|---|
| D1 | accepted | Rust binary = config manager; does not call OpenCode. A future `evals run` subcommand that shells out to the OpenCode CLI is explicitly **deferred to post-v1** | Easier: tractable scope, no LLM-provider coupling in Rust. Harder: we cannot run evals ourselves until post-v1; depends on the user's OpenCode install for execution |
| D2 | accepted | Keep agent/skill/command format = markdown + YAML frontmatter | Easier: human-editable, OpenCode-compatible. Harder: YAML edge cases must be parsed robustly; schema validation is essential |
| D3 | accepted | Must read/validate `.opencode/` structure (OAC-compatible) | Easier: drop-in migration for existing OAC users. Harder: must preserve exact byte-level compat (frontmatter, HTML comments, line counts) |
| D4 | accepted | Rust edition 2024; single binary crate | Easier: modern language features, simple distribution. Harder: no workspace-level reuse if a lib is needed later (revisit via CR) |
| D5 | accepted | Backend target: OpenCode only | Easier: one backend to reason about. Harder: adding Claude Code or direct APIs later needs a new decision |
| D6 | accepted | Specs written in English | Easier: open-source friendly, machine-parseable. Harder: owner reviews in a second language |
| D7 | accepted | License: MIT + attribution to OAC | Easier: legally safe reuse. Harder: must keep attribution (`NOTICE.md`) correct as content evolves |
| D8 | accepted | **`content/` is the in-repo source of truth; validation is self-referential.** Walk tests validate the **real `content/` tree** (structure, frontmatter, references) and always run. There is **no** external OAC checkout dependency and **no** golden diff against upstream (constitution C6; replaces the old external-parity golden test) | Easier: no gitignored dev artifact to maintain; tests always run (incl. CI); intentional divergence is first-class. Harder: drift vs the historical OAC baseline is no longer machine-checked (accepted trade-off: OAC is a frozen starting point) |
| D9 | accepted | Interactive wizard generators for agents/skills/commands | Easier: SystemBuilder-style DX, spec-compliant output. Harder: TTY interaction must degrade gracefully in CI/non-interactive mode |
| D10 | accepted | Evals: validate cases + results JSON + HTML dashboard | Easier: useful feedback loop without running agents. Harder: dashboard HTML generation must be deterministic and dependency-free |
| D11 | accepted | YAML parsing: use **`serde-saphyr`**, not `serde_yaml` (archived/deprecated). Discovered via the context7 `find-docs` skill (Aug 2026): `serde_yaml` has no Context7 entry; top matches were `serde_yml` (transition shim), `noyalib` (drop-in, keeps `Value`), and `serde-saphyr` (typed-only). **Chosen: `serde-saphyr`** (panic-free, budget limits, zero-copy) because our frontmatter schemas are fully typed and we don't need a dynamic `Value` DOM. Escape hatch: `noyalib` if `Value` support is ever needed | Easier: robust, panic-free, budget-guarded parsing (anti-DoS on 440-file tree); zero-copy perf; typed schemas fit the validation philosophy. Harder: no `Value`/`Mapping` dynamic DOM; serde-saphyr is a newer project |
| D12 | accepted | Adopt the OAC **component registry** (`registry.json`) and **install-state manifest** (`<install_dir>/.mac/manifest.json`) as managed, file-based artifacts (2026-08-03, from the OAC feature inventory; renamed from `.oac/` to `.mac/` by user decision — MAC identity). The registry is a JSON catalog over the markdown tree; validating it is C16-compatible because every check (paths exist, dependencies resolve, profiles cover) is declared by the registry schema. The manifest tracks SHA256 per installed file for `status`/`update` with user-modified detection | Easier: trustworthy install/update/add, component install by id/profile with dependency resolution, matches OAC's actual component-management spine. Harder: more surface to implement (new REG module); the `.mac/` manifest is tool DX (C16) and must stay transparent, never a format rule |

## 9. Non-Functional Requirements

- **NFR1 (Content integrity):** every managed path in §6 is covered by an always-on walk test against the real `content/` tree (no external checkout needed).
- **NFR2 (Determinism):** running `myagentcontrol install` on the same input produces byte-identical output (no timestamps embedded by default).
- **NFR3 (Performance):** `validate` on a full project runs in < 2s; `list` < 500ms.
- **NFR4 (Rust quality):** `cargo test` green, `cargo clippy -- -D warnings` clean, `cargo fmt --check` clean, no unsafe code.
- **NFR5 (Zero runtime deps beyond Rust):** the binary must not require node/bun at runtime (TS scripts are *source artifacts* managed by us, not executed by us).
- **NFR6 (Backward compat):** never corrupt or overwrite user edits; `validate` reports, `install` scaffolds non-destructively (idempotent).

## 10. Roadmap

### Phase 0: Foundation (done)
Cargo project layout, module structure, CLI arg parsing, error types (E100–E500), test harness helpers (`src/core/golden.rs` → renamed responsibility: tree helpers used by walk tests).

### Phase 1: Context system (MVI)
Parse/validate context files incl. HTML-comment frontmatter (CTX-1), MVI thresholds (CTX-2), `paths.json` resolution rules + local-first/global-fallback logic (CTX-3), `install` copy (CTX-4), `/add-context` wizard (CTX-5), `--update` version increment (CTX-6), `@`-reference syntax convention (CTX-7), external-context cache (CTX-8). `content/` vendored (done) + walk tests. `navigation.md` files are validated as ordinary context files; the wizard keeps them updated. No navigation cross-reference validator (removed 2026-08-03, C16).

### Phase 2: Agents & subagents
Frontmatter schema validation (YAML), permission maps, category JSONs, delegation graph validation, inventory walk test.

### Phase 3: Skills & commands
SKILL.md + router.sh presence/structure validation, workflow files, command files with `dependencies` frontmatter, walk tests.

### Phase 4: Eval framework
YAML case schema validation, evaluator-name + `expectedContextFiles` validation, results JSON, HTML dashboard generator (user-provided `evals/` dir). (`evals run` and the logging collector are post-v1; see D1.)

### Phase 5: Component registry & install state
`registry.json` vendored + validated (paths, dependencies, profiles), manifest `<install_dir>/.mac/manifest.json` with SHA256, `add`/`remove`/`status`/`update` commands, profile-based install (`install --profile`), collision detection. Per [`modules/registry-spec.md`](./modules/registry-spec.md).

### Phase 6: Full-tree walk tests & polish
Walk tests for `profiles/`, `prompts/`, `tool/`, `plugin/`, `plugins/`, `docs/`, `scripts/`, root config files (cli-spec §9); docs; release.

### Post-v1 (candidates, tracked elsewhere)
Open PRs from the original repo are intentionally **not** listed in this spec. They are tracked separately as future feature candidates and will be spec'd individually when picked up. Known post-v1 runtime items (D1): `evals run`, the eval logging collector, the multi-agent test runner, and the abilities system (`packages/plugin-abilities`, needs OpenCode plugin runtime).

## 11. Appendix: Acceptance Criteria (master-level)

Given/When/Then form per constitution C10.

- **AC1** Given a fresh empty project, **when** `myagentcontrol install` runs, **then** it scaffolds a complete `.opencode/` tree identical to `content/` and `myagentcontrol validate` exits 0.
- **AC2** Given the managed tree, **when** `myagentcontrol validate` runs on a tree containing invalid YAML frontmatter, an invalid permission verb, or a concept card missing its reference section, **then** it exits 1 and reports each specific issue.
- **AC3** Given a wizard invocation, **when** it completes, **then** the generated agent/skill/command files pass `myagentcontrol validate` immediately.
- **AC4** Given a valid `results/latest.json`, **when** `myagentcontrol evals dashboard` runs, **then** it renders a self-contained HTML dashboard with pass/fail counts.
- **AC5** Given the codebase, **when** `cargo test` (incl. walk tests), `cargo clippy -- -D warnings`, and `cargo fmt --check` run, **then** all pass with zero warnings.
- **AC6** Given a fresh project, **when** `install --profile developer`, `status`, `add context:<id>`, and `update --check` run in sequence, **then** they succeed idempotently, the manifest records installs, and no user-modified file is ever silently overwritten.

## 12. Open Questions

- OQ1: Should the binary also manage `profiles/`, `prompts/`, `tool/`, `plugin/`, `plugins/`, `docs/` content, or only validate their presence? (Default: validate + scaffold; see [`modules/cli-spec.md`](./modules/cli-spec.md) §9.)
- OQ2: How strictly to preserve the TypeScript artifacts (`task-cli.ts`, tool/plugin sources)? They are source artifacts; we manage them but don't execute them. (Default: keep as-is in scaffold.)
- OQ3: Should we eventually add a Rust-native execution adapter (direct provider APIs) behind a trait, without committing to it now? (Default: no, per D1.)
