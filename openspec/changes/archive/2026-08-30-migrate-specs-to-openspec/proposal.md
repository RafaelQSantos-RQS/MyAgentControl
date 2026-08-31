## Why

The project has two parallel spec systems: `.specs/` (custom SDD with YAML frontmatter, manual lifecycle, governance docs) and `openspec/` (standardized tool with schema, artifacts, and workflow). The `.specs/` system is legacy and creates confusion about the source of truth. Consolidating into `openspec/` simplifies the workflow, leverages tooling, and eliminates the dual-system overhead.

## What Changes

- Migrate 9 spec files from `.specs/` to `openspec/specs/` as capabilities
- Convert `.specs/` frontmatter (YAML with `id`, `type`, `status`, `version`) to OpenSpec-compatible structure
- Preserve all requirements, scenarios, and acceptance criteria verbatim
- Move the constitution to `openspec/specs/constitution/spec.md` as a capability
- Move the master spec to `openspec/specs/master/spec.md` as a capability
- Move 7 module specs to their respective capability directories
- Update `openspec/config.yaml` with project context from the constitution
- **BREAKING**: Remove `.specs/` directory after migration is verified

## Capabilities

### New Capabilities

- `constitution`: Immutable project rules (C1–C16); the non-negotiable baseline
- `master`: Master spec (MAC-MASTER): vision, goals, architecture, decisions, roadmap
- `agents`: Agents module (MAC-AG): schema, permissions, delegation graph
- `cli`: CLI binary module (MAC-CLI): commands, validation, walk tests, wizards
- `commands`: Commands module (MAC-CMD): slash commands
- `context`: Context system module (MAC-CTX): MVI, local-first resolution, wizard
- `evals`: Evals module (MAC-EV): YAML cases, results JSON, dashboard
- `registry`: Component registry & install state (MAC-REG): manifest, profiles, add/update
- `skills`: Skills module (MAC-SK): SKILL.md validation, router.sh

### Modified Capabilities

- `context-system`: Already exists from prior change; no requirements change (the migration creates a separate `context` capability for the MAC-CTX spec; `context-system` stays as the implementation spec)

## Impact

- `.specs/` directory removed (BREAKING for anyone referencing those paths)
- `openspec/specs/` populated with 9 capability directories
- `openspec/config.yaml` updated with project context
- No code changes — this is a spec-only migration
- `.specs/README.md` governance doc is replaced by OpenSpec workflow
