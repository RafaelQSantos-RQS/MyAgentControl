---
id: MAC-CMD
type: module-spec
parent: MAC-MASTER
title: Commands — Module Spec
status: approved
version: 0.0.1
updated: 2026-08-03
change_requests: []
depends_on: [MAC-MASTER, MAC-AG, MAC-CTX]
---

# Commands — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 0.0.1 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | Vendored `content/command/` (OAC v0.7.1 as starting point) |
| **Note** | Rewritten 2026-08-03 under the format-fidelity principle (C16). The command frontmatter schema (`description`, `tags`, `dependencies`) is OAC-declared (`[OAC format]`); dependency-resolution checks and wizards are tool-added developer-experience features (`[tool DX]`). `navigation_update` in `/add-context` is a behavior the wizard performs, not a machine-validated rule |

---

## 1. Purpose

Slash commands are markdown files loaded into the agent's working memory when invoked (e.g. `/commit`, `/test`). They act as smart routers/injectors: frontmatter declares dependencies, and the body instructs the agent with the exact workflow. The Rust tool validates and scaffolds them.

## 2. Command File Schema

```yaml
---
description: One-line purpose
tags: [context, onboarding, wizard]        # optional
dependencies:                              # optional
  - subagent:context-organizer
  - context:core/context-system/standards/mvi.md
  - context:core/standards/project-intelligence.md
---
```

**Validation rules:**
- `description` required.
- `dependencies` entries resolve: `subagent:<name>` → must exist in agent tree; `context:<path>` → must exist in context tree (relative to context root); `skill:<name>` → must exist in skills tree.

## 3. Required Command Inventory (vendored, `content/command/`)

| File | Purpose |
|---|---|
| `add-context.md` | 6-question Project Intelligence onboarding wizard (`/add-context`, `--update`) |
| `commit.md` | Conventional commits with emoji; pre-commit lint/build validation |
| `commit-openagents.md` | Commit variant for the OpenAgents repo itself |
| `test.md` | Testing pipeline (typecheck → lint → test → report) |
| `optimize.md` | Code optimization workflow |
| `context.md` | Context management |
| `clean.md` | Cleanup workflow |
| `analyze-patterns.md` | Pattern analysis |
| `validate-repo.md` | Repo validation |
| `worktrees.md` | Git worktree management |
| `build-context-system.md` | Context system build |
| `test-new-command.md` | Test scaffold for new commands |
| `openagents/` (subfolder) | OpenAgents-specific commands |
| `prompt-engineering/` (subfolder) | Prompt enhancement commands |

## 4. Key Command Behaviors (parity content preserved verbatim)

- **`/add-context`**: interactive wizard; 6 questions (tech stack, API example, component example, naming conventions, code standards, security requirements) → `project-intelligence/technical-domain.md`. Critical rules: project_intelligence, frontmatter_required (HTML comment), mvi_compliance (<200 lines), codebase_refs, navigation_update, priority_assignment, version_tracking.
- **`/commit`**: pre-commit validation → analyze git status/diff → conventional message with emoji → confirm → commit.
- **`/test`**: run the project's full testing pipeline and fix failures iteratively.

## 5. Functional Requirements

Markers (C16): `[OAC format]` validates a rule the command/OpenCode format declares; `[tool DX]` is a user-approved developer-experience feature.

- **CMD-1** `[OAC format]` Parse/validate command frontmatter (description, tags, dependencies).
- **CMD-2** `[tool DX]` Validate dependency resolution across all three reference spaces (subagent/context/skill); report dangling references with the specific command + dep. The vendored command tree satisfies this check today (any future defect is fixed in the tree per C16 policy).
- **CMD-3** `[OAC format]` `install` copies the vendored `content/command/` tree (C6); validate operates on the real tree.
- **CMD-4** `[tool DX]` `wizard command new`: interactive generator for description, tags, dependencies, workflow steps → command markdown.
- **CMD-5** `[tool DX]` `list commands`: table of name, description, tags, dependency count.

## 6. Examples & Scenarios

### 6.1 Valid command frontmatter

```yaml
---
description: Create well-formatted commits
tags: [git, commit]
dependencies:
  - subagent:context-organizer
  - context:core/standards/project-intelligence.md
---
```

### 6.2 Invalid cases (each must be rejected)

- `dependencies: [subagent:ghost]` → error CMD-201 (unknown subagent)
- `dependencies: [context:core/missing.md]` → error CMD-202 (unknown context file)
- `dependencies: [skill:unknown-skill]` → error CMD-203 (unknown skill)
- Missing `description` → error CMD-204

## 7. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-M1** Given the vendored `content/command/` tree, **when** `validate --commands` runs, **then** it passes; **when** any §6.2 defect is injected, **then** it fails naming the command and the broken dependency.
- **AC-M2** Given the `content/command/` tree, **when** the commands walk test runs, **then** every file is validated against the schema (always-on, no external checkout).
- **AC-M3** Given a wizard-generated command, **when** `validate --commands` runs, **then** it passes immediately.

## 8. Cross-References

- `dependencies` reference agents and context → [`agents-spec.md`](./agents-spec.md), [`context-spec.md`](./context-spec.md)
- CLI surface → [`cli-spec.md`](./cli-spec.md)
