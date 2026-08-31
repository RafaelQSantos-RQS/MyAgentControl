## 1. Preparation

- [ ] 1.1 Run `openspec validate` on current change to ensure specs are well-formed
- [ ] 1.2 Run `openspec apply change migrate-specs-to-openspec` to create main specs
- [ ] 1.3 Verify `openspec/specs/` contains all 9 capability directories

## 2. Verification

- [ ] 2.1 Verify `openspec/specs/constitution/spec.md` exists and has all C1–C16 requirements
- [ ] 2.2 Verify `openspec/specs/master/spec.md` exists with vision, goals, architecture
- [ ] 2.3 Verify `openspec/specs/agents/spec.md` exists with schema, permissions, delegation
- [ ] 2.4 Verify `openspec/specs/cli/spec.md` exists with commands, validation, wizards
- [ ] 2.5 Verify `openspec/specs/commands/spec.md` exists with schema, dependencies
- [ ] 2.6 Verify `openspec/specs/context/spec.md` exists with MVI, resolution, @-refs
- [ ] 2.7 Verify `openspec/specs/evals/spec.md` exists with cases, results, dashboard
- [ ] 2.8 Verify `openspec/specs/registry/spec.md` exists with registry, manifest, profiles
- [ ] 2.9 Verify `openspec/specs/skills/spec.md` exists with SKILL.md, router, inventory
- [ ] 2.10 Verify `openspec/specs/context-system/spec.md` still exists (unchanged from prior change)

## 3. Config Update

- [ ] 3.1 Update `openspec/config.yaml` with project context (constitution rules, SDD process, quality gates)

## 4. Cleanup

- [ ] 4.1 Archive the change: `openspec archive change migrate-specs-to-openspec`
- [ ] 4.2 Remove `.specs/` directory
- [ ] 4.3 Update `.gitignore` if it references `.specs/`

## 5. Commit

- [ ] 5.1 Stage all changes: `git add openspec/ .specs/`
- [ ] 5.2 Commit with message: `chore(openspec): migrate specs from .specs/ to openspec/specs/`
- [ ] 5.3 Push to remote
