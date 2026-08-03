---
id: MAC-REG
type: module-spec
parent: MAC-MASTER
title: Component Registry & Install State — Module Spec
status: approved
version: 0.0.3
updated: 2026-08-03
change_requests: []
depends_on: [MAC-MASTER, MAC-CTX, MAC-AG, MAC-SK, MAC-CMD]
---

# Component Registry & Install State — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 0.0.3 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | OAC v0.7.1 `registry.json`, `packages/cli` (manifest/installer/registry libs), `scripts/registry/`, `install.sh` (profile-based install) |
| **Note** | Added 2026-08-03 from the OAC feature inventory (gaps G1/G2/G3). The registry is a **JSON manifest over the markdown tree**; validating it is C16-compatible because every rule it checks (paths exist, dependencies resolve, profiles reference existing components) is declared by the registry schema itself. 0.0.3: command renamed `init` → **`install`** (user decision, Brick 1) |

---

## 1. Purpose

OAC's "component management" spine is a **registry**: a JSON catalog of every
agent, subagent, command, skill, context file, tool, plugin, and config in the
tree, plus named **profiles** that select component sets. Alongside it lives an
**install state** (`.oac/manifest.json`) that tracks what is installed, its
SHA256 hash, and whether the user modified it.

Our Rust tool implements the *file-based, agnostic* side of this: validate the
registry, resolve component dependencies, install by profile or by id, and
track install state so `status`/`update` are trustworthy. It does **not**
implement IDE-specific adapters (`apply` → `.cursorrules`/`CLAUDE.md`/
`.windsurfrules`) or the Claude Code integration (out of scope, master §3).

## 2. Registry Schema (`registry.json`, vendored)

The registry is vendored under `content/registry.json` (source of truth, C6)
and copied by `install` to `.opencode/../registry.json` or a project-level location
managed by the tool (see cli-spec §3).

```json
{
  "version": "2.0.0",
  "schema_version": "2.0.0",
  "categories": { "essential": "", "standard": "", "extended": "", "specialized": "", "meta": "" },
  "components": {
    "agents":     [ { "id": "system-builder", "name": "OpenSystemBuilder", "type": "agent",
                       "path": ".opencode/agent/meta/system-builder.md",
                       "description": "...", "tags": ["..."],
                       "dependencies": ["subagent:domain-analyzer", "..."],
                       "category": "meta" } ],
    "subagents":  [ ],
    "commands":   [ ],
    "skills":     [ ],
    "contexts":   [ ],
    "tools":      [ ],
    "plugins":    [ ],
    "config":     [ ]
  },
  "profiles": {
    "essential": { "name": "Essential", "description": "...", "components": ["agent:openagent", "..."] }
  },
  "metadata": { "lastUpdated": "2026-03-21", "schemaVersion": "1.0.0" }
}
```

Reference counts (OAC v0.7.1): 8 agents, 19 subagents, 17 commands, 4 skills,
194 contexts, 2 tools, 1 plugin, 3 configs, 5 profiles.

## 3. Registry Validation Rules (declared by the registry schema)

1. **Schema**: `version`, `schema_version` present; `categories`/`profiles`/
   `components` are the only top-level keys (metadata tolerated).
2. **Path existence**: every component `path` (relative to the tree root) must
   exist. Forward check: validates what the registry *declares*, never requires
   every file to be listed (C16; the removed CTX-4 would be the opposite).
3. **Dependencies**: every `dependencies` entry must resolve in its namespace:
   `subagent:<id>` → in `components.subagents`; `context:<path>` → exists in the
   context tree; `skill:<id>` → in `components.skills`; `tool:<id>` → in
   `components.tools`.
4. **Category validity**: `category` ∈ {essential, standard, extended,
   specialized, meta}.
5. **Profile coverage**: every `profiles.*.components` entry must reference an
   existing component (`agent:<id>`, `context:<id>`, `skill:<id>`, ...).
6. **Uniqueness**: component `id`s are unique per type.

## 4. Install State (`.oac/manifest.json`, tool DX)

The tool maintains a manifest in the target project to make installs
trustworthy and idempotent:

```json
{
  "oac_version": "0.0.2",
  "files": {
    ".opencode/agent/core/opencoder.md": {
      "type": "agent",
      "installed_at": "2026-08-03T...",
      "sha256": "abc..."
    }
  }
}
```

- `type` ∈ {agent, context, skill, config, other}.
- **User-modified detection**: on `status`/`update`, compare disk hash against
  the manifest hash; a mismatch means the user edited the file.
- **Backup before overwrite**: `update` never destroys a user-modified file; it
  backs it up (e.g. `*.bak`) and reports the collision instead of silently
  overwriting, unless `--force`.
- The manifest is tool DX (C16): a developer-experience record, not a format
  rule.

## 5. Profiles & Install Semantics

- **Install by profile**: `myagentcontrol install --profile developer` installs the
  component set named by the profile (see cli-spec §3).
- **Add a component**: `myagentcontrol add <type>:<id>` installs that component
  plus its transitive dependencies, non-destructively.
- **Collision detection**: if a target path already exists with different
  content, report the collision and the chosen strategy (keep existing | backup
  + overwrite), never silent overwrite (NFR2/NFR6).
- **Uninstall**: `myagentcontrol remove <type>:<id>` removes the component and
  its manifest entry; leaves user files alone.

## 6. Functional Requirements

Markers (C16): `[OAC format]` validates a rule the registry schema declares;
`[tool DX]` is a user-approved developer-experience feature.

- **REG-1** `[OAC format]` Parse and validate `registry.json` (schema fields, enums, category, uniqueness).
- **REG-2** `[tool DX]` Validate that every component `path` exists (forward-only).
- **REG-3** `[tool DX]` Validate dependency resolution across namespaces (subagent/context/skill/tool).
- **REG-4** `[tool DX]` Validate profile coverage (profiles reference existing components).
- **REG-5** `[tool DX]` Auto-detect: scan the managed tree for unregistered components and report them (never silently mutate).
- **REG-6** `[tool DX]` `add <type>:<id>` installs a component with transitive dependency resolution, non-destructive.
- **REG-7** `[tool DX]` Maintain `.oac/manifest.json`: SHA256 per installed file, type classification, installed-at.
- **REG-8** `[tool DX]` `status` compares manifest vs disk: report modified, added, and removed files with a diff summary.
- **REG-9** `[tool DX]` `update` applies bundle changes, preserves user-modified files (backup + report), supports `--check`/dry-run.
- **REG-10** `[tool DX]` `install --profile <name>` installs a named profile's component set; collision detection on every copy.

## 7. Examples & Scenarios

### 7.1 Valid registry entry (accepted)

```json
{
  "id": "context-organizer",
  "name": "ContextOrganizer",
  "type": "subagent",
  "path": ".opencode/agent/subagents/system-builder/context-organizer.md",
  "description": "Organizes context files",
  "tags": ["context", "system-builder"],
  "dependencies": [],
  "category": "meta"
}
```

### 7.2 Invalid cases (each must be rejected)

- `category: bogus` → error REG-201 (not in the enum)
- `path: .opencode/agent/missing.md` → error REG-202 (path does not exist)
- `dependencies: ["subagent:ghost"]` → error REG-203 (unknown subagent)
- `profiles.essential.components: ["agent:unknown"]` → error REG-204 (profile ref unresolved)
- Duplicate component id in a type → error REG-205

## 8. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-R1** Given the vendored `registry.json`, **when** `validate --registry` runs, **then** it passes; **when** any §7.2 defect is injected, **then** it fails with the matching error code and the component id.
- **AC-R2** Given a fresh project, **when** `install && status` runs, **then** status reports no modifications; **when** the user edits one managed file, **then** status flags exactly that file as modified.
- **AC-R3** Given a profile, **when** `install --profile <name>` runs, **then** only the profile's component set is installed (plus dependencies) and every copied file is recorded in the manifest.
- **AC-R4** Given an installed tree with a user-modified file, **when** `update` runs, **then** the modified file is preserved (backed up + reported), never silently overwritten.

## 9. Cross-References

- Registry entries reference all modules → [`context-spec.md`](./context-spec.md), [`agents-spec.md`](./agents-spec.md), [`skills-spec.md`](./skills-spec.md), [`commands-spec.md`](./commands-spec.md)
- CLI surface: `install --profile`, `add`, `remove`, `status`, `update`, `validate --registry` → [`cli-spec.md`](./cli-spec.md)
- Scope of vendoring (registry.json in `content/`) → [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) §6.5
