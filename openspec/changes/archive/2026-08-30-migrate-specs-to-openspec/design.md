## Context

The project has two parallel spec systems:
- `.specs/` — custom SDD with YAML frontmatter (`id`, `type`, `status`, `version`), manual lifecycle, governance docs (constitution, master spec, 7 module specs)
- `openspec/` — standardized tool with `spec-driven` schema, artifact-based workflow (proposal → specs → design → tasks)

Both contain equivalent content. The `.specs/` system is legacy and creates confusion about the source of truth.

## Goals / Non-Goals

**Goals:**
- Consolidate all specs into `openspec/specs/` as capabilities
- Preserve all requirements and scenarios verbatim (this is a migration, not a rewrite)
- Update `openspec/config.yaml` with project context from the constitution
- Remove `.specs/` after verification

**Non-Goals:**
- Rewriting or improving spec content (preserve as-is)
- Changing any code or behavior
- Modifying the OpenSpec tooling itself
- Changing the constitution's lifecycle semantics (it stays "outside the lifecycle")

## Decisions

### Decision 1: Constitution as a capability

**Choice**: Move `constitution.md` to `openspec/specs/constitution/spec.md` as a regular capability.

**Alternatives considered**:
- Keep in `.specs/` separately — rejected: defeats consolidation purpose
- Put in `openspec/config.yaml` as project context — rejected: loses the structured requirement/scenario format
- Create a custom "constitution" artifact type — rejected: over-engineering, OpenSpec doesn't support it

**Rationale**: Treating the constitution as a capability is the simplest path. Its "outside the lifecycle" semantics are preserved by convention (it's not modified by normal change flows).

### Decision 2: One capability per module spec

**Choice**: Create 7 separate capabilities (agents, cli, commands, context, evals, registry, skills) matching the 7 module specs.

**Alternatives considered**:
- Single "modules" capability — rejected: loses modularity, violates C9
- Keep module specs as subdirectories of a single capability — rejected: OpenSpec doesn't support nested capabilities

**Rationale**: Direct 1:1 mapping preserves the existing modular structure and makes future changes easier to reason about.

### Decision 3: Delta specs are copies (not modifications)

**Choice**: Since this is a migration, all delta specs use `## ADDED Requirements` with the full content from the originals.

**Alternatives considered**:
- Create empty deltas and rely on archive to populate — rejected: too fragile, content could be lost
- Modify originals in-place first — rejected: breaks the `.specs/` system before migration is verified

**Rationale**: ADDED deltas are the safest migration path. The archive step will create the main specs with the full content.

### Decision 4: `context-system` stays as-is

**Choice**: The existing `openspec/specs/context-system/` (from the prior change) is not modified. The new `context` capability is a separate entry for the MAC-CTX module spec.

**Alternatives considered**:
- Merge into `context-system` — rejected: different scopes (implementation spec vs module spec)
- Delete `context-system` — rejected: it's the implementation spec from a completed change

**Rationale**: `context-system` is the implementation spec for the context system change. `context` is the module spec for the MAC-CTX capability. They serve different purposes.

## Risks / Trade-offs

- **Risk**: Losing governance nuance from `.specs/README.md` (CR process, versioning rules) → **Mitigation**: These rules are captured in the constitution (C11) and can be added to `openspec/config.yaml` project context
- **Risk**: Users referencing `.specs/` paths in documentation → **Mitigation**: Commit message and PR description clearly state the migration
- **Risk**: OpenSpec validate may reject some delta formats → **Mitigation**: Test with `openspec validate` before committing

## Migration Plan

1. Create all 9 delta specs (done)
2. Create design.md and tasks.md
3. Run `openspec apply change migrate-specs-to-openspec`
4. Verify `openspec/specs/` contains all 9 capabilities
5. Run `openspec archive change migrate-specs-to-openspec`
6. Remove `.specs/` directory
7. Commit with conventional commit message

## Open Questions

None — all decisions are resolved.
