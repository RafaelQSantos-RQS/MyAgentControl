---
id: MAC-EV
type: module-spec
parent: MAC-MASTER
title: Eval Framework — Module Spec
status: draft
version: 0.1.0
updated: 2026-08-02
depends_on: [MAC-MASTER, MAC-AG]
---

# Eval Framework — Module Spec

| | |
|---|---|
| **Status** | Draft |
| **Version** | 0.1.0 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | OAC repo at tag `v0.7.1`: [`/evals/`](https://github.com/darrenhinde/OpenAgentsControl/tree/v0.7.1/evals/) + `package.json` scripts |

---

## 1. Purpose

OAC ships an evaluation harness that runs agents against YAML test cases and produces JSON results plus an HTML dashboard. Per user decisions (R3Q4: harness + dashboard), we port the framework's *management* side to Rust: define/validate cases, produce results JSON, and render the dashboard.

**Scope note (D1):** v1 evals are a **config-manager** feature — validate cases, validate result schemas, generate the dashboard from an existing results JSON. Running cases requires executing agents, which is out of scope for the config-manager binary; an `evals run` subcommand that shells out to the OpenCode CLI is **deferred to post-v1** (the user explicitly chose "Gerenciador de config" over "Gerenciador + evals em Rust" in the interview, R5Q1).

## 2. Target Structure (parity)

```
evals/
├── framework/
│   ├── src/                          # original TS sources (managed artifact)
│   ├── scripts/
│   └── package.json                  # eval:sdk scripts (managed artifact)
├── agents/                           # agent-specific eval configs
│   ├── core/  meta/  shared/  openagent/  development/  subagents/  content/
├── results/
│   ├── latest.json
│   ├── history/*openagent*.json, *opencoder*.json
│   └── serve.sh                      # dashboard server (port 8000)
└── test_tmp/
```

## 3. Eval Case Schema (YAML)

Original cases live under `evals/agents/...` with patterns such as `developer/*.yaml`, `context-loading/*.yaml`, `business/*.yaml`, `smoke-test.yaml`, `developer/bash-*.yaml`, `developer/simple-bash-test.yaml`. Validation must enforce:
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
- **EV-2** `evals validate` — validate all cases + registry + result schema.
- **EV-3** (post-v1, deferred) `evals run [--agent] [--model] [--pattern]` — future subcommand that would shell out to the OpenCode CLI (`opencode` must be on PATH) and write `results/latest.json` + timestamped `results/history/<agent>-<ts>.json`. Documented here so the results JSON schema is designed now; the runner itself is not built in v1.
- **EV-4** `evals dashboard` — generate a self-contained HTML dashboard from `results/latest.json` (pass/fail counts, per-agent summary, history links). No external assets; opens on port 8000 like the original `serve.sh`.
- **EV-5** Scaffold the `evals/` tree with parity cases and framework sources (as managed artifacts — not executed by the Rust binary; TypeScript sources preserved verbatim).
- **EV-6** Validate `registry.json` consistency (referenced models/agents exist).

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

- **AC-E1** Given the pristine reference cases, **when** `evals validate` runs, **then** it passes; **when** any §6.2 defect is injected, **then** it fails with the matching error code.
- **AC-E2** Given a sample `latest.json` fixture, **when** `evals dashboard` runs, **then** it renders correct totals and per-agent rows (unit-tested HTML).
- **AC-E3** (post-v1) Given OpenCode on PATH, **when** `evals run` runs, **then** it produces a schema-valid results JSON — deferred, not required for v1.
- **AC-E4** Given a scaffolded `evals/` tree, **when** compared to the reference (excluding generated results), **then** the diff is clean.

## 8. Cross-References

- Agents under test → [`agents-spec.md`](./agents-spec.md)
- Registry validation also covers context/commands → [`context-spec.md`](./context-spec.md), [`commands-spec.md`](./commands-spec.md)
- CLI surface → [`cli-spec.md`](./cli-spec.md)
