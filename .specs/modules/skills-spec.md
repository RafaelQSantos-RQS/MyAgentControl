---
id: MAC-SK
type: module-spec
parent: MAC-MASTER
title: Skills — Module Spec
status: approved
version: 1.0.0
updated: 2026-08-02
change_requests: []
depends_on: [MAC-MASTER, MAC-AG]
---

# Skills — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 1.0.0 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | Vendored `content/skills/` (plural only; OAC v0.7.1 as starting point). The singular `skill/` tree from OAC was **removed** by user decision (orphaned — the OpenCode runtime only loads `skills/<name>/SKILL.md`; see `NOTICE.md`) |

---

## 1. Purpose

Skills are self-contained capabilities: a `SKILL.md` (markdown + YAML frontmatter), a `router.sh` bash entrypoint, and optional scripts/workflows. They extend agents with reusable workflows (task management, external docs, context management, personality routing).

## 2. Skill Structure (vendored, `content/skills/` — 4 skills)

```
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

## 3. SKILL.md Frontmatter Validation (two tiers)

**OpenCode-official contract (what the runtime enforces — `src/skills/frontmatter.rs`):**
- SKILL.md starts with YAML frontmatter (`---` delimited); only `name` (required),
  `description` (required), `license`, `compatibility`, `metadata` are recognized;
  **unknown fields are ignored** (OAC's `version`/`author`/`type`/`category`/`tags`
  are tolerated).
- `name` matches folder name, is 1–64 chars, and matches
  `^[a-z0-9]+(-[a-z0-9]+)*$`; `description` is 1–1024 chars.
- Error codes (implemented): `SK-201` name≠folder, `SK-206` missing block, `SK-207` bad YAML,
  `SK-208` missing `name`, `SK-209` invalid `name`, `SK-210` missing
  `description`, `SK-211` description length.

*Authoring conventions (stricter, for wizard-generated skills only, SK-4):*
- `type: skill` required, `version` semver, `category` and `tags` present.
- `router.sh` exists, is executable (mode ≥ 0755), and is referenced by the SKILL.md body.
- All files referenced in SKILL.md body exist (scripts/, workflows/).
- Authoring-tier error codes (wizard-only, not enforced on vendored content):
  `SK-202` type missing, `SK-203` version not semver, `SK-204` router.sh
  missing/not executable, `SK-205` referenced file missing.

> The vendored `content/skills/` (e.g. `context7`) omits `type`/`category`/`tags` —
> enforcing the authoring tier on vendored content would break AC-S1 ("pristine
> passes `validate --skills`"). The OpenCode-official tier is the loadability
> gate; authoring rules apply to `wizard skill new` output only.

## 4. Functional Requirements

- **SK-1** Validate skill folder structure: SKILL.md present, frontmatter valid (OpenCode tier), router.sh present + executable, referenced files exist.
- **SK-2** Validate router.sh is a bash script with a `#!/usr/bin/env bash` shebang and that it dispatches the documented subcommands.
- **SK-3** `init` copies the vendored `content/skills/` tree (C6); validate operates on the real tree.
- **SK-4** `wizard skill new` — interactive generator: name, description, version, category, tags, subcommands → generates SKILL.md + router.sh skeleton + optional workflow stub (authoring tier enforced on output).
- **SK-5** `list skills` — table of name, version, category, tags, folder path.
- **SK-6** Cross-check: skills referenced in agent `skill:` permission allowlists exist.

## 5. Examples & Scenarios

### 5.1 Valid SKILL.md frontmatter (OpenCode tier)

```yaml
---
name: task-management
description: JSON-driven task breakdown CLI
---
```

### 5.2 Invalid cases (each must be rejected, OpenCode tier)

- `name` ≠ folder name → error SK-201
- Missing `---` frontmatter block → error SK-206
- Malformed YAML → error SK-207
- Missing `name` → error SK-208
- Invalid `name` (bad regex / wrong length) → error SK-209
- Missing `description` → error SK-210
- Description too long → error SK-211

### 5.3 Authoring-tier defects (wizard output only)

- `type` missing or ≠ `skill` → SK-202
- `version: 1.0` (not semver) → SK-203
- `router.sh` missing or not executable → SK-204
- SKILL.md references a file that does not exist → SK-205

### 5.4 Router dispatch example

`router.sh create auth-system` → dispatches `create`; unknown subcommand → non-zero exit + usage.

## 6. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-S1** Given the vendored `content/skills/` tree, **when** `validate --skills` runs, **then** it passes; **when** any §5.2 defect is injected, **then** it fails with the matching error code.
- **AC-S2** Given the `content/skills/` tree, **when** the skills walk test runs, **then** every SKILL.md passes the OpenCode tier (always-on, no external checkout).
- **AC-S3** Given a wizard-generated skill, **when** `validate --skills` runs, **then** it passes immediately (authoring tier).
- **AC-S4** Given the task-management skill's TS tests, **when** the Rust binary runs, **then** they remain untouched managed artifacts (never executed).

## 7. Cross-References

- Agents that consume skills (`task-manager` → task-management) → [`agents-spec.md`](./agents-spec.md)
- Workflow files are context-like → [`context-spec.md`](./context-spec.md)
- CLI surface → [`cli-spec.md`](./cli-spec.md)
