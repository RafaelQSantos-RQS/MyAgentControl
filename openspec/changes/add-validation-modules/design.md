## Context

- Context frontmatter uses HTML comments (`<!-- ... -->`), agents/skills/commands use YAML `---` fences — different parsers
- 8 agents, 19 subagents in `content/agent/` with `0-category.json` files per category
- 4 skills in `content/skills/` with `SKILL.md` + `router.sh` structure
- 14 commands in `content/command/` with YAML frontmatter
- No evals directory exists yet — must be created
- `serde` and `serde_json` already in dependencies; `serde_yaml` needs adding

## Goals / Non-Goals

**Goals:**
- Shared YAML frontmatter parser for agents/skills/commands
- Module-specific validators matching each spec's requirements
- CLI integration via `validate <module>` subcommand
- Walk tests for agents, skills, commands
- Evals case validation + dashboard generation

**Non-Goals:**
- `evals run` (deferred per D1)
- Eval import from OAC checkout (low priority, can add later)
- Modifying the content tree itself

## Decisions

### Decision 1: Shared validation core

**Choice**: Create `src/validation/mod.rs` with a generic YAML `---` frontmatter parser that returns `HashMap<String, Value>`. Each module (agents, skills, commands) defines its own schema validation on top.

**Rationale**: All three use the same `---` fence format. One parser, three validators.

### Decision 2: Dependency for YAML parsing

**Choice**: Add `serde_yaml` crate for YAML parsing. The existing `serde` + `serde_json` handle the rest.

**Alternatives considered**:
- Manual YAML parsing — rejected: fragile, reinventing the wheel
- Use `serde_json` with YAML-to-JSON conversion — rejected: indirect, lossy

**Rationale**: `serde_yaml` is the standard Rust YAML parser, small, well-maintained.

### Decision 3: Validate subcommand structure

**Choice**: Extend existing `validate` command with `--agents`, `--skills`, `--commands`, `--evals` flags. Without flags, validates everything.

**Rationale**: Keeps the CLI simple, follows existing pattern.

### Decision 4: Evals scope

**Choice**: Implement case schema validation + dashboard HTML generation. Skip `import` (low value) and `run` (deferred per spec).

**Rationale**: Validates the spec requirements EV-501 through EV-506 without over-building.

## Risks / Trade-offs

- **Risk**: New `serde_yaml` dependency → **Mitigation**: small crate, widely used, no transitive issues
- **Risk**: Large change (4 modules at once) → **Mitigation**: modular structure, each module is independent, walk tests verify each
- **Risk**: Delegation graph cycle detection complexity → **Mitigation**: simple DFS, well-understood algorithm

## Migration Plan

1. Add `serde_yaml` dependency
2. Create `src/validation/` with shared parser
3. Implement agents validator
4. Implement skills validator
5. Implement commands validator
6. Create `src/evals/` module
7. Extend CLI validate subcommand
8. Add walk tests
9. Test everything
10. Commit

## Open Questions

None — all specs are clear.
