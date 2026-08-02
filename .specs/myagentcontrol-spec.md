---
id: MAC-MASTER
type: master-spec
title: MyAgentControl — Master Spec
status: draft
version: 0.1.0
updated: 2026-08-02
owner: Rafael (user)
depends_on: []
language: en
license: MIT + attribution to OpenAgentsControl
---

# MyAgentControl — Master Spec

| | |
|---|---|
| **Status** | Draft |
| **Version** | 0.1.0 |
| **Updated** | 2026-08-02 |
| **Owner** | Rafael (user) |
| **Language** | English (per decision) |
| **License** | MIT + attribution to OpenAgentsControl |

---

## 1. Background & Motivation

**OpenAgentsControl (OAC)** (<https://github.com/darrenhinde/OpenAgentsControl>) is a model-agnostic AI agent framework built on top of the OpenCode CLI. It ships as a set of markdown files (agents, subagents, skills, commands, context, eval cases) that teach AI agents a project's coding patterns, enforce plan-first workflows with approval gates, and reduce token usage via a Minimal Viable Information (MVI) context system. The maintainer has slowed down; many high-quality community PRs remain unmerged.

The user — a Rust enthusiast who uses many different AI models — wants a **faithful Rust rewrite** of the framework: same concepts, same features (feature parity), same file formats, but implemented as a Rust tool. The plan is to first build the spec (this folder, SDD), then implement, then mine the open PRs as future features.

## 2. Goals

1. **Feature parity** with OAC v0.7.1: all core agents, subagents, skills, commands, context system, and eval framework.
2. **Model-agnostic**: never tie the framework to one vendor. Execution happens via the OpenCode CLI (user's chosen backend), which is itself model-agnostic.
3. **Configuration manager, not a runtime**: the Rust binary generates/validates/maintains the `.opencode/`-compatible structure. It does **not** invoke OpenCode or any model API itself (per user decision).
4. **Golden-test verifiable**: the generated/validated structure must match the reference repo ([`darrenhinde/OpenAgentsControl`](https://github.com/darrenhinde/OpenAgentsControl), pinned `v0.7.1`) so parity is machine-checked.
5. **Editable & transparent**: agents/skills/commands remain human-editable markdown with YAML frontmatter, exactly like the original.
6. **Token-efficient MVI context system**: lazy loading, files < 200 lines, local-first resolution.
7. **Developer-friendly CLI**: `myagentcontrol` binary with init/validate/list/wizard commands.

## 3. Non-Goals (explicit out-of-scope for v1)

- ❌ Running agents, calling model APIs, or wrapping the OpenCode CLI as a subprocess. (Note: a `myagentcontrol evals run` subcommand that shells out to OpenCode was considered but is **deferred to post-v1** — see D1.)
- ❌ A TUI or chat interface.
- ❌ The Claude Code plugin distribution (`.claude-plugin/`, `plugins/claude-code/`) — noted as reference only.
- ❌ Open PRs as features — they are intentionally **not** listed in this spec; tracked separately and spec'd individually when picked up (see §10 Post-v1).
- ❌ Cloud migration, agent marketplace/registry hosting, IDE integrations.

## 4. Core Concepts & Terminology

| Term | Meaning |
|---|---|
| **Agent** | A markdown file with YAML frontmatter describing an AI persona (`mode: primary` or `subagent`), temperature, and permission rules. |
| **Subagent** | A specialized agent invoked via the `task` tool by primary agents (ContextScout, TaskManager, etc.). |
| **Skill** | A self-contained capability folder: `SKILL.md` + `router.sh` (bash entrypoint) + optional scripts/workflows. |
| **Command** | A slash command (`/commit`, `/test`, …) — a markdown file loaded into the agent context when invoked. |
| **Context** | Project coding standards/patterns as markdown, loaded before execution. MVI: < 200 lines, scannable in < 30s. |
| **MVI** | Minimal Viable Information — only load what's needed, when it's needed. |
| **Approval gate** | Rule that agents must request approval before write/edit/bash. |
| **Frontmatter** | YAML metadata block at the top of markdown files (`---` delimited) or HTML comment metadata for context files. |
| **paths.json** | Optional config that sets `custom_dir` context root and `global` fallback path. |

## 5. High-Level Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                    myagentcontrol (Rust binary)              │
│  init │ validate │ list │ wizard │ evals │ import │ export   │
└───────────────┬──────────────────────────────────────────────┘
                │ generates / validates / maintains
                ▼
┌──────────────────────────────────────────────────────────────┐
│            .opencode/ (markdown + config, OAC-compatible)    │
│  agent/  subagents/  skill(s)/  command/  context/  profiles/│
│  prompts/  tool/  plugin/  config.json  opencode.json        │
│  evals/  registry.json  scripts/                            │
└──────────────────────────────────────────────────────────────┘
                │ read by (not invoked by us)
                ▼
┌──────────────────────────────────────────────────────────────┐
│                  OpenCode CLI (user's tool)                  │
│   model-agnostic: Claude │ GPT │ Gemini │ MiniMax │ local    │
└──────────────────────────────────────────────────────────────┘
```

**Key architectural decision (D1):** The Rust binary is a *manager* of the `.opencode/` structure. It does not execute agents. The OpenCode CLI (already installed and used by the user) is the execution backend. This keeps the rewrite scope tractable, keeps the user's existing workflows intact, and preserves model-agnosticism (OpenCode handles providers).

## 6. Feature Inventory (from reference repo v0.7.1)

### 6.1 Agents (`.opencode/agent/`)
- **Core (primary):** `openagent.md`, `opencoder.md`, `eval-runner.md`
- **Meta:** `system-builder.md`, `repo-manager.md`
- **Subagents:**
  - *core:* contextscout, externalscout, task-manager, batch-executor, context-manager, context-retriever, documentation, stage-orchestrator
  - *code:* coder-agent, test-engineer, reviewer, build-agent
  - *planning:* architecture-analyzer, adr-manager, contract-manager, prioritization-engine, story-mapper
  - *development:* frontend-specialist, devops-specialist
  - *system-builder:* agent-generator, command-creator, context-organizer, domain-analyzer, workflow-designer
  - *content:* copywriter, technical-writer
  - *data:* data-analyst
  - *utils:* image-specialist
  - *test:* simple-responder
  - *category metadata:* `0-category.json` files

### 6.2 Skills
- `skill/project-orchestration/` — multi-agent workflows (context-handoff, session-context, 8-stage-delivery)
- `skills/task-management/` — task CLI (`task-cli.ts`), JSON-driven task breakdown
- `skills/smart-router-skill/` — personality routing (yoda/stark/sherlock workflows)
- `skills/context7/` — external docs via Context7
- `skills/context-manager/` — context management
- `skill/task-management/` — with TypeScript tests

### 6.3 Commands (`.opencode/command/`)
`add-context`, `commit`, `commit-openagents`, `test`, `optimize`, `context`, `clean`, `analyze-patterns`, `validate-repo`, `worktrees`, `build-context-system`, `test-new-command`, `prompt-enhancer` (in `command/prompt-engineering/`), `openagents/` subfolder.

### 6.4 Context (`.opencode/context/`)
`core/`, `ui/`, `development/`, `project-intelligence/`, `product/`, `data/`, `learning/`, `content-creation/`, `system-builder-templates/`, `openagents-repo/`, `project/` + `navigation.md`, `index.md`, `CODEBASE_STANDARDS.md`.

### 6.5 Other structure
`profiles/` (advanced, business, developer, essential, full), `prompts/` (core, development, data, content), `tool/` (TypeScript tool barrel — empty in latest), `plugin/` (agent-validator, notify), `config.json`, `opencode.json`, `registry.json`, `scripts/` (registry validation, markdown link validation, versioning, bridge, external-context, testing, maintenance, tests), `evals/`, `bin/oac.js`, `install.sh`, `update.sh`.

## 7. Cross-References

| Topic | Module spec |
|---|---|
| Context system, MVI, navigation, local-first resolution | [`modules/context-spec.md`](./modules/context-spec.md) |
| Agent/subagent schema, permissions, delegation graph | [`modules/agents-spec.md`](./modules/agents-spec.md) |
| Skills structure, router.sh, workflows | [`modules/skills-spec.md`](./modules/skills-spec.md) |
| Slash commands | [`modules/commands-spec.md`](./modules/commands-spec.md) |
| Eval framework, YAML cases, dashboard | [`modules/evals-spec.md`](./modules/evals-spec.md) |
| The Rust CLI binary | [`modules/cli-spec.md`](./modules/cli-spec.md) |

## 8. Technical Decisions (ADR-style, MADR format)

> Format per [MADR](https://adr.github.io/madr/): **Context → Decision → Consequences**. Changes to accepted decisions require a Change Request (see README §Change Requests).

| ID | Status | Context / Decision | Consequences |
|---|---|---|---|
| D1 | accepted | Rust binary = config manager; does not call OpenCode. A future `evals run` subcommand that shells out to the OpenCode CLI is explicitly **deferred to post-v1** (user chose "Gerenciador de config" over "Gerenciador + evals em Rust" in the interview) | Easier: tractable scope, no LLM-provider coupling in Rust. Harder: we cannot run evals ourselves until post-v1; depends on the user's OpenCode install for execution |
| D2 | accepted | Keep agent/skill/command format = markdown + YAML frontmatter | Easier: parity, human-editable, golden-testable. Harder: YAML edge cases must be parsed robustly; schema validation is essential |
| D3 | accepted | Must read/validate `.opencode/` structure (OAC-compatible) | Easier: drop-in migration for existing OAC users. Harder: must preserve exact byte-level compat (frontmatter, HTML comments, line counts) |
| D4 | accepted | Rust edition 2024; single binary crate | Easier: modern language features, simple distribution. Harder: no workspace-level reuse if a lib is needed later (revisit via CR) |
| D5 | accepted | Backend target: OpenCode only | Easier: one backend to reason about. Harder: adding Claude Code or direct APIs later needs a new decision |
| D6 | accepted | Specs written in English | Easier: open-source friendly, machine-parseable. Harder: owner reviews in a second language |
| D7 | accepted | License: MIT + attribution to OAC | Easier: legally safe reuse. Harder: must keep attribution headers correct across scaffolds |
| D8 | accepted | Golden tests against a checkout of [`darrenhinde/OpenAgentsControl`](https://github.com/darrenhinde/OpenAgentsControl) at tag `v0.7.1` | Easier: machine-checked parity. Harder: upgrading the reference requires re-baselining |
| D9 | accepted | Interactive wizard generators for agents/skills/commands | Easier: SystemBuilder-style DX, spec-compliant output. Harder: TTY interaction must degrade gracefully in CI/non-interactive mode |
| D10 | accepted | Evals: harness + results JSON + HTML dashboard | Easier: useful feedback loop without running agents. Harder: dashboard HTML generation must be deterministic and dependency-free |
| D11 | accepted | YAML parsing: use **`serde-saphyr`** — NOT `serde_yaml` (archived/deprecated). Discovered via the context7 `find-docs` skill (Aug 2026): `serde_yaml` has no Context7 entry; the top matches were `serde_yml` (a transition shim whose own docs recommend migrating off it), `noyalib` (drop-in, keeps `Value`), and `serde-saphyr` (typed-only, no `Value`). **Chosen: `serde-saphyr`** (benchmark 87.23 vs 76.74; panic-free, budget limits, zero-copy) because our frontmatter schemas are fully typed and we don't need a dynamic `Value` DOM. Escape hatch: `noyalib` is a drop-in if `Value` support is ever needed | Easier: robust, panic-free, budget-guarded parsing (anti-DoS on 450+ file trees); zero-copy perf; typed schemas fit the validation philosophy. Harder: no `Value`/`Mapping` dynamic DOM (permission maps must be typed as `HashMap<String, HashMap<String, Permission>>`); serde-saphyr is a newer project |

## 9. Non-Functional Requirements

- **NFR1 — Parity:** Every file in the reference `.opencode/` tree has a managed counterpart; golden diff tests pass.
- **NFR2 — Determinism:** Running `myagentcontrol init` on the same input produces byte-identical output (no timestamps embedded by default).
- **NFR3 — Performance:** `validate` on a full project runs in < 2s; `list` < 500ms.
- **NFR4 — Rust quality:** `cargo test` green, `cargo clippy -- -D warnings` clean, `cargo fmt --check` clean, no unsafe code.
- **NFR5 — Zero runtime deps beyond Rust:** the binary must not require node/bun at runtime (TS scripts are *source artifacts* managed by us, not executed by us).
- **NFR6 — Backward compat:** never corrupt or overwrite user edits; `validate` reports, `init` scaffolds non-destructively (idempotent).

## 10. Roadmap

### Phase 0 — Foundation (specs approved → skeleton)
- Cargo project layout, module structure, CLI arg parsing, error types, test harness setup.

### Phase 1 — Context system (MVI)
- Parse/validate context files incl. HTML-comment frontmatter, navigation.md, paths.json resolution rules, local-first/global-fallback logic (unit tested).

### Phase 2 — Agents & subagents
- Frontmatter schema validation (YAML), permission maps, category JSONs, delegation graph validation, OpenCoder/OpenAgent/eval-runner/managed files.

### Phase 3 — Skills & commands
- SKILL.md + router.sh presence/structure validation, workflow files, command files with `dependencies` frontmatter.

### Phase 4 — Eval framework
- YAML case schema validation, results JSON, HTML dashboard generator, `evals/framework` tree parity. (`evals run` is post-v1 — see D1.)

### Phase 5 — Golden tests & polish
- Full-tree golden diff vs reference; `profiles/`/`prompts/`/`tool/`/`plugin/`/`scripts/`/`registry.json` parity (see [`modules/cli-spec.md`](./modules/cli-spec.md) §9); docs.

### Post-v1 (candidates, tracked elsewhere)
- Per user decision (R5Q4), open PRs from the original repo are intentionally **not** listed in this spec. They are tracked separately as future feature candidates and will be spec'd individually when picked up.

## 11. Appendix — Acceptance Criteria (master-level)

Given/When/Then form per constitution C10.

- **AC1** Given a fresh empty project, **when** `myagentcontrol init` runs, **then** it scaffolds a complete `.opencode/` tree that golden-matches the reference repo (dates normalized).
- **AC2** Given the pristine reference tree, **when** `myagentcontrol validate` runs, **then** it exits 0; **when** a tree contains a broken markdown link, invalid YAML frontmatter, a missing navigation entry, an invalid registry.json, or a permission-map error, **then** it exits 1 and reports each specific issue.
- **AC3** Given a wizard invocation, **when** it completes, **then** the generated agent/skill/command files pass `myagentcontrol validate` immediately.
- **AC4** Given a valid `results/latest.json`, **when** `myagentcontrol evals dashboard` runs, **then** it renders a self-contained HTML dashboard with pass/fail counts.
- **AC5** Given the codebase, **when** `cargo test` (incl. golden tests), `cargo clippy -- -D warnings`, and `cargo fmt --check` run, **then** all pass with zero warnings.

## 12. Open Questions

- OQ1: Should the binary also manage `profiles/`, `prompts/`, `tool/`, `plugin/` content, or only validate their presence? (Default: validate + scaffold — see [`modules/cli-spec.md`](./modules/cli-spec.md) §9.)
- OQ2: How strictly to preserve the TypeScript artifacts (`task-cli.ts`, `stage-cli.ts`, `context-index.ts`)? They are source artifacts; we manage them but don't execute them. (Default: keep as-is in scaffold.)
- OQ3: Is `install.sh`/`update.sh` parity required, or a Rust-native `install` subcommand instead? (Default: scaffold the scripts as-is.)
- OQ4: Should we eventually add a Rust-native execution adapter (direct provider APIs) behind a trait, without committing to it now? (Default: no, per D1.)
