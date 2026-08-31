## Why

The project has complete specs for agents, skills, and commands validation modules but zero Rust code. The evals module is entirely spec-only. This change implements all four modules in one pass because agents, skills, and commands share the same pattern (YAML `---` frontmatter validation), making a single change more efficient than four separate ones.

## What Changes

### New Rust modules:
- `src/validation/` — shared YAML `---` frontmatter parser (reusable across agents, skills, commands)
- `src/validation/agents.rs` — agent schema, category JSON, delegation graph validation
- `src/validation/skills.rs` — SKILL.md schema, router.sh, referenced files, structure validation
- `src/validation/commands.rs` — command schema, dependency validation
- `src/evals/` — eval case schema, results JSON, dashboard HTML generation

### New CLI subcommands:
- `myagentcontrol validate agents` — validate agent files
- `myagentcontrol validate skills` — validate skill files  
- `myagentcontrol validate commands` — validate command files
- `myagentcontrol validate evals` — validate eval cases

### New walk tests:
- `tests/agent_walk.rs` — inventory consistency (0-category.json ↔ files)
- `tests/skill_walk.rs` — skill structure + inventory
- `tests/command_walk.rs` — command inventory

### Deferred (per spec D1):
- `evals run` — returns "not implemented" message

## Capabilities

### New Capabilities

- `validation`: Shared validation core for YAML frontmatter across agents, skills, commands

### Modified Capabilities

- `cli`: Extended validate subcommand with module-specific validation
- `agents`: Implementation of AG-201 through AG-205
- `skills`: Implementation of SK-701 through SK-706
- `commands`: Implementation of CMD-401 through CMD-403
- `evals`: Implementation of EV-501 through EV-506

## Impact

- `src/main.rs` — validate subcommand routing to module-specific validators
- `src/lib.rs` — new `validation` and `evals` module declarations
- New files: ~8 Rust source files + 3 walk test files
- `content/` — read-only (validated, not modified)
