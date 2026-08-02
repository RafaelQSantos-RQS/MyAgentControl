# NOTICE

## Starting point: OpenAgentsControl v0.7.1

The [`content/`](./content/) directory in this repository began as a
**verbatim copy** of the `.opencode/` tree from
[OpenAgentsControl](https://github.com/darrenhinde/OpenAgentsControl),
pinned at tag **`v0.7.1`** (agents, subagents, skills, commands, context,
profiles, prompts, tool, plugin, scripts; 440 files).

`myagentcontrol` is a **Rust rewrite** of OpenAgentsControl: OAC v0.7.1 is the
**starting point**, not a moving upstream dependency. From here, the project
evolves as its own vision of the framework: `content/` is maintained **in this
repository**, diverging from the original intentionally (adopted community PRs
and project-specific changes), and is **never re-fetched from upstream**.

Copyright (c) 2025 Darren Hinde. Licensed under the **MIT License**
(see [`LICENSE`](./LICENSE) and the
[original license](https://github.com/darrenhinde/OpenAgentsControl/blob/v0.7.1/LICENSE)).

## Divergence policy

- `content/` is the **source of truth** for the managed tree (constitution C6).
  `myagentcontrol init` copies it into the user project as `.opencode/`.
- Divergence from the OAC v0.7.1 baseline is **intentional and tracked**:
  adopted community PRs are recorded in a Change Request
  (`.specs/changes/<id>-cr.md`) and reflected in the specs; never silent
  drift. Validation is by the always-on walk tests against the real
  `content/` tree (master decision D8).
- **Orphaned `skill/` (singular) tree removed (2026-08).** OAC v0.7.1 also
  ships an `.opencode/skill/` (singular) subtree (`project-orchestration/` +
  `task-management/tests/`, 10 files) that the OpenCode runtime never
  discovers: the official docs only load `.opencode/skills/<name>/SKILL.md`
  (plural); neither the OAC `registry.json` nor `install.sh`
  references it. Removed from `content/` by user decision; documented in the
  skills spec and `NOTICE.md`.

## Project license

`myagentcontrol` (this repository, excluding the vendored tree above) is
Copyright (c) 2026 Rafael Queiroz Santos, licensed under the **MIT License**; see
[`LICENSE`](./LICENSE).
