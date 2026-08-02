---
id: MAC-EV
type: module-spec
parent: MAC-MASTER
title: Eval Framework — Module Spec
status: approved
version: 1.0.0
updated: 2026-08-02
change_requests: []
depends_on: [MAC-MASTER, MAC-AG]
---

# Eval Framework — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 1.0.0 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | OAC v0.7.1 eval concept (YAML cases → results JSON → HTML dashboard). NOTE: the OAC `evals/` tree lives at the OAC **repo root**, not in `.opencode/`; it is **not vendored** into `content/` (master §6.5) |

---

## 1. Purpose

OAC ships an evaluation harness that runs agents against YAML test cases and produces JSON results plus an HTML dashboard. Per user decisions (R3Q4: harness + dashboard), we port the framework's *management* side to Rust: define/validate cases, produce results JSON, and render the dashboard.

**Scope notes:**
- **No vendored `evals/` in `content/`** (master §6.5): the OAC `evals/` tree is a repo-root artifact. The Rust tool therefore operates on a **user-provided** `evals/` directory in the target project (validated, not scaffolded from `content/`).
- **No `registry.json` validation** (EV-6 removed): `registry.json` is an OAC repo-root artifact, not present in the managed `.opencode/` tree.
- **D1:** v1 evals are a **config-manager** feature: validate cases, validate result schemas, generate the dashboard from an existing results JSON. Running cases requires executing agents, which is out of scope for the config-manager binary; an `evals run` subcommand that shells out to the OpenCode CLI is **deferred to post-v1**.

## 2. Target Structure (user-provided `evals/` in the project)

```
evals/                       # user-provided (e.g. copied from an OAC checkout by `import`)
├── agents/                  # agent-specific eval configs (YAML cases)
├── results/
│   ├── latest.json
│   ├── history/*.json
│   └── serve.sh             # dashboard server (port 8000)
└── test_tmp/
```

## 3. Eval Case Schema (YAML)

Original cases live under `evals/agents/...` with patterns such as `developer/*.yaml`, `context-loading/*.yaml`, `business/*.yaml`, `smoke-test.yaml`. Validation must enforce:
- `name` present and unique.
- `agent` references an existing agent (openagent, opencoder, …).
- `model` field is provider-qualified (`opencode/grok-code-fast`, `anthropic/claude-...`, `openai/gpt-4-turbo`) or absent (default).
- Case body defines steps and (optionally) evaluators.

## 4. Model-Agnostic Test Matrix (from package.json)

- Defaults: `opencode/grok-code-fast`; alternatives: `anthropic/claude-3-5-sonnet-20241022`, `openai/gpt-4-turbo`.
- Scripts pattern: `test:<agent>:<model>` e.g. `test:openagent:grok`, `test:opencoder:claude`.
- CI patterns: `test:ci` → smoke-test + simple-bash-test.

## 5. Functional Requirements

- **EV-1** Validate YAML eval cases against the schema above (name uniqueness, agent existence, model format).
- **EV-2** `evals validate`: validate all cases + result schema in the target `evals/` dir.
- **EV-3** (post-v1, deferred) `evals run [--agent] [--model] [--pattern]`: future subcommand that would shell out to the OpenCode CLI (`opencode` must be on PATH) and write `results/latest.json` + timestamped `results/history/<agent>-<ts>.json`. Documented here so the results JSON schema is designed now; the runner itself is not built in v1.
- **EV-4** `evals dashboard`: generate a self-contained HTML dashboard from `results/latest.json` (pass/fail counts, per-agent summary, history links). No external assets; opens on port 8000 like the original `serve.sh`.
- **EV-5** `import <path-to-oac>` may bring an existing `evals/` tree into a project (managed artifacts, never executed by Rust).
- **EV-6** Validate `registry.json` consistency: **removed** (not part of the managed tree; see §1).

## 6. Examples & Scenarios

### 6.1 Valid eval case (excerpt)

```yaml
name: smoke-test
agent: openagent
model: opencode/grok-code-fast
steps:
  - task: "echo hello"
```

### 6.2 Invalid cases (each must be rejected)

- duplicate `name` → error EV-201
- `agent: ghost` → error EV-202 (unknown agent)
- `model: gpt4` (unqualified) → error EV-203
- missing `steps` → error EV-204

### 6.3 Dashboard fixture

`latest.json` with `{passed: 2, failed: 1}` → HTML shows totals and per-agent rows.

## 7. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-E1** Given a user-provided `evals/` dir with valid cases, **when** `evals validate` runs, **then** it passes; **when** any §6.2 defect is injected, **then** it fails with the matching error code.
- **AC-E2** Given a sample `latest.json` fixture, **when** `evals dashboard` runs, **then** it renders correct totals and per-agent rows (unit-tested HTML).
- **AC-E3** (post-v1) Given OpenCode on PATH, **when** `evals run` runs, **then** it produces a schema-valid results JSON; deferred, not required for v1.
- **AC-E4** Given a project with a `results/` dir, **when** the tool runs, **then** generated results are never committed into `content/` (kept out of the managed tree).

## 8. Cross-References

- Agents under test → [`agents-spec.md`](./agents-spec.md)
- CLI surface → [`cli-spec.md`](./cli-spec.md)
