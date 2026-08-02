---
id: MAC-AG
type: module-spec
parent: MAC-MASTER
title: Agents — Module Spec
status: approved
version: 1.0.0
updated: 2026-08-02
change_requests: []
depends_on: [MAC-MASTER, MAC-CTX]
---

# Agents — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 1.0.0 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | Vendored `content/agent/` (OAC v0.7.1 as starting point) |

---

## 1. Purpose

Agents are markdown files with YAML frontmatter describing AI personas: their role, temperature, permissions, and delegation rules. Primary agents (`OpenAgent`, `OpenCoder`, `EvalRunner`, `SystemBuilder`, `RepoManager`) orchestrate; subagents are invoked via the `task` tool. The Rust tool must validate this schema and scaffold the vendored tree.

## 2. Agent File Schema (YAML frontmatter)

```yaml
---
name: AgentName
description: "One-line role description"
mode: primary | subagent          # required
temperature: 0.1                  # optional
permission:
  question: "allow"               # allow | ask | deny
  read:   { "*": "allow" }        # subagent-only read permissions
  grep:   { "*": "allow" }
  glob:   { "*": "allow" }
  bash:
    "*": "ask"
    "sudo *": "deny"
    "rm -rf /*": "deny"
  edit:
    "**/*.env*": "deny"
    "**/*.key": "deny"
    "**/*.secret": "deny"
    "node_modules/**": "deny"
    ".git/**": "deny"
  write: { "*": "deny" }
  task:
    "contextscout": "allow"       # delegation allowlist
    "*": "deny"
  skill: { "*": "deny", "task-management": "allow" }
---
```

**Validation rules:**
- `mode` ∈ {primary, subagent}.
- Permission verbs ∈ {allow, ask, deny}.
- Core agents use allowlist-with-deny patterns (`rm -rf *` ask, `sudo *` deny, secrets deny, node_modules/.git deny).
- Subagents generally deny bash/edit/write; read/grep/glob allow; task/skill allowlists for delegation.

## 3. Required Agent Inventory (vendored, `content/agent/`)

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

## 4. Behavioral Rules Enforced via Agent Content (documented, not executed by Rust)

These come from the markdown bodies and are preserved verbatim in scaffolds:
- **approval_gate**: request approval before any bash/write/edit/task; read/list/glob/grep exempt.
- **critical_context_requirement**: load required context files before code tasks.
- **stop_on_failure** / **report_first**: report → propose → request approval → fix (never auto-fix).
- **incremental_execution**: one step at a time with validation.
- **contextscout_exempt**: ContextScout is exempt from approval gates; always use it for discovery first.
- **local-first context resolution** (ContextScout rule): ≤ 2 glob checks, global fallback only for core/.

## 5. Functional Requirements

- **AG-1** Parse and validate agent frontmatter (required fields, enums, permission map structure).
- **AG-2** Validate delegation graph: `task:` allowlists in an agent must reference existing subagent names; subagents referenced in any `task` permission must exist.
- **AG-3** Validate category metadata (`0-category.json`) consistency with directory contents.
- **AG-4** `init` copies the vendored `content/agent/` tree (C6); validate operates on the real tree.
- **AG-5** `wizard agent new`: interactive generator producing a spec-compliant agent file (SystemBuilder-style); name, mode, temperature, permissions, delegation allowlist.
- **AG-6** `list agents`: table of name, mode, category, temperature, description.

## 6. Examples & Scenarios

### 6.1 Valid permission map (accepted)

```yaml
mode: subagent
permission:
  read: { "*": "allow" }
  bash: { "*": "deny" }
  task: { "contextscout": "allow", "*": "deny" }
```

### 6.2 Invalid cases (each must be rejected)

- `mode: bogus` → error AG-201 (mode not in {primary, subagent})
- `permission.bash."*"."allow all"` → error AG-202 (verb not in {allow, ask, deny})
- `task: { "ghost-agent": "allow" }` → error AG-203 (delegated subagent does not exist)
- `0-category.json` listing a file that is absent → error AG-204
- Missing `name` or `description` → error AG-205

## 7. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-A1** Given the vendored `content/agent/` tree, **when** `validate --agents` runs, **then** it passes; **when** any file from §6.2 is injected, **then** it fails with the matching error code and the file path.
- **AC-A2** Given the `content/agent/` tree, **when** the agents walk test runs, **then** every file is validated against the schema (always-on, no external checkout).
- **AC-A3** Given a wizard-generated agent, **when** `validate --agents` runs, **then** it passes immediately.
- **AC-A4** Given `mode: bogus` or `task: {"ghost-agent": "allow"}`, **when** validation runs, **then** the error names the exact field and offers a suggestion.

## 8. Cross-References

- Context loading rules used by agents → [`context-spec.md`](./context-spec.md)
- Skills referenced in skill permissions → [`skills-spec.md`](./skills-spec.md)
- Commands referenced from agent workflows → [`commands-spec.md`](./commands-spec.md)
- CLI surface: `list`, `validate`, `wizard` → [`cli-spec.md`](./cli-spec.md)
