---
id: MAC-SK
type: module-spec
parent: MAC-MASTER
title: Skills — Module Spec
status: draft
version: 0.1.0
updated: 2026-08-02
depends_on: [MAC-MASTER, MAC-AG]
---

# Skills — Module Spec

| | |
|---|---|
| **Status** | Draft |
| **Version** | 0.1.0 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | OAC repo at tag `v0.7.1`: [`/opencode/skill/`](https://github.com/darrenhinde/OpenAgentsControl/tree/v0.7.1/.opencode/skill/) + [`/opencode/skills/`](https://github.com/darrenhinde/OpenAgentsControl/tree/v0.7.1/.opencode/skills/) |

---

## 1. Purpose

Skills are self-contained capabilities: a `SKILL.md` (markdown + YAML frontmatter), a `router.sh` bash entrypoint, and optional scripts/workflows. They extend agents with reusable workflows (multi-agent orchestration, task management, external docs, context management, personality routing).

## 2. Skill Structure (target parity)

```
skill/
├── project-orchestration/          # multi-agent workflow orchestration
│   ├── SKILL.md                    # frontmatter: name, description, version, author, type, category, tags
│   ├── router.sh                   # bash dispatcher (create, get-context, add-output, show, stage-*)
│   ├── scripts/{session-context-manager.ts, context-index.ts, stage-cli.ts}
│   └── workflows/{context-handoff.md, 8-stage-delivery.md, planning-agents.md}
└── task-management/
    ├── SKILL.md
    ├── router.sh
    └── tests/{enhanced-schema.test.ts, line-number-validation.test.ts}

skills/
├── task-management/                # JSON-driven task breakdown
│   ├── SKILL.md, router.sh
│   └── scripts/task-cli.ts
├── smart-router-skill/
│   ├── SKILL.md, router.sh
│   ├── config/personality-config.json
│   └── scripts/{yoda-workflow.sh, stark-workflow.sh, sherlock-workflow.sh}
├── context7/
│   ├── SKILL.md, README.md, navigation.md, library-registry.md
└── context-manager/
    ├── SKILL.md, router.sh
```

## 3. SKILL.md Frontmatter Schema

```yaml
---
name: project-orchestration
description: Orchestrate multi-agent workflows for feature development using planning agents, context handoff, and stage management
version: 1.0.0
author: opencode
type: skill
category: orchestration
tags:
  - orchestration
  - multi-agent
  - planning
  - workflow
  - stages
---
```

**Validation rules:**
- `name` matches folder name.
- `type: skill` required.
- `version` is semver.
- `category` and `tags` present.
- `router.sh` exists, is executable (mode ≥ 0755), and is referenced by the SKILL.md body.
- All files referenced in SKILL.md body exist (scripts/, workflows/).

## 4. Project Orchestration Workflows (parity content)

Three workflows:
1. **Context Handoff (lightweight)** — commands: `create`, `get-context`, `add-output`, `show`. Passes minimal context between planning agents (recommended for automation).
2. **Session Context (interactive)** — commands: `session-create`, `session-load`, `session-summary`. Human-readable planning narrative.
3. **Multi-Stage Delivery (8 stages)** — commands: `stage-init`, `stage-status`, `stage-complete`, `stage-rollback`. Full feature delivery workflow.

## 5. Functional Requirements

- **SK-1** Validate skill folder structure: SKILL.md present, frontmatter valid, router.sh present + executable, referenced files exist.
- **SK-2** Validate router.sh is a bash script with a `#!/usr/bin/env bash` shebang and that it dispatches the documented subcommands.
- **SK-3** Scaffold all 5 skills above with parity content.
- **SK-4** `wizard skill new` — interactive generator: name, description, version, category, tags, subcommands → generates SKILL.md + router.sh skeleton + optional workflow stub.
- **SK-5** `list skills` — table of name, version, category, tags, folder path.
- **SK-6** Cross-check: skills referenced in agent `skill:` permission allowlists exist.

## 6. Examples & Scenarios

### 6.1 Valid SKILL.md frontmatter

```yaml
---
name: project-orchestration
description: Orchestrate multi-agent workflows
type: skill
version: 1.0.0
category: orchestration
tags: [orchestration, multi-agent]
---
```

### 6.2 Invalid cases (each must be rejected)

- `name` ≠ folder name → error SK-201
- `type` missing or ≠ `skill` → error SK-202
- `version: 1.0` (not semver) → error SK-203
- `router.sh` missing or not executable → error SK-204
- SKILL.md references `workflows/x.md` that does not exist → error SK-205

### 6.3 Router dispatch example

`router.sh create auth-system` → dispatches `create`; unknown subcommand → non-zero exit + usage.

## 7. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-S1** Given the pristine reference `skill/`+`skills/` trees, **when** `validate --skills` runs, **then** it passes; **when** any §6.2 defect is injected, **then** it fails with the matching error code.
- **AC-S2** Given a scaffolded skills tree, **when** compared to the reference, **then** the diff is clean.
- **AC-S3** Given a wizard-generated skill, **when** `validate --skills` runs, **then** it passes immediately.
- **AC-S4** Given the task-management skill's TS tests, **when** the Rust binary runs, **then** they remain untouched managed artifacts (never executed).

## 8. Cross-References

- Agents that consume skills (`task-manager` → task-management) → [`agents-spec.md`](./agents-spec.md)
- Workflow files are context-like → [`context-spec.md`](./context-spec.md)
- CLI surface → [`cli-spec.md`](./cli-spec.md)
