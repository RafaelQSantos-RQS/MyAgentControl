# MyAgentControl — Specs (SDD)

> **Spec-Driven Development.** This folder is the single source of truth for *what* we are building. Code is written only after a spec exists and is approved. The immutable project rules live in [`constitution.md`](./constitution.md).

> **Rebuilt 2026-08-02 (user decision).** The previous spec set was replaced by
> this one in a single commit. This folder is the current, authoritative spec
> set.

## What is this project?

A **Rust reimplementation** of [OpenAgentsControl (OAC)](https://github.com/darrenhinde/OpenAgentsControl): `myagentcontrol`.

OAC is a model-agnostic AI agent framework (agents, skills, commands, context system, eval harness) distributed as markdown/config files that run on top of the OpenCode CLI. `myagentcontrol` is a **configuration manager** written in Rust: it **copies the vendored `content/` tree** (the in-repo source of truth, constitution C6), validates, and maintains the `.opencode/`-style structure so users keep using the OpenCode CLI they already rely on.

**OAC v0.7.1 is the historical starting point, not a moving upstream and not a parity yardstick.** The project evolves as its own vision of the framework: `content/` is maintained in this repository, diverges intentionally (adopted community PRs, project-specific changes), and is **never re-fetched** from upstream.

## Spec lifecycle & gating (SDD)

Every spec file follows a strict lifecycle (with one exception: the [`constitution.md`](./constitution.md) lives **outside** this lifecycle, using only `ratified`/`amended`). **No implementation may begin until its spec is `approved`** (constitution C7).

```
draft → review → approved → in-development → released → deprecated
```

| Status | Meaning | Gate |
|---|---|---|
| `draft` | Being written; open for comment | — |
| `review` | Submitted for user review | Nothing may be implemented |
| `approved` | User accepted; implementation may begin | Implementation allowed |
| `in-development` | Implementation in progress | Spec locked; changes via Change Request only |
| `released` | Implementation complete; acceptance criteria met | — |
| `deprecated` | Superseded; kept for history | No new work |

**Frontmatter gating:** each spec's YAML frontmatter carries `status` and `id` (e.g. `MAC-CTX`). Tools/agents MUST refuse to implement a spec whose `status` is not `approved`.

**Change Request trail:** specs updated via the CR process also carry `change_requests: [<ID>, …]` in their frontmatter and a body-table row pointing at the CR file.

## Change Requests (editing an approved spec)

> **Pre-release note:** while there is no functional release yet (pre-v1),
> specs are still being shaped; edit them **directly** (bump `version`) without
> opening a CR. The CR workflow below applies only to **released** specs.

Approved specs are never edited silently in place (constitution C11). Changes to a *released* spec go through a lightweight Change Request:

1. Open a CR (file: `.specs/changes/<id>-cr.md`) describing the change, motivation, and affected spec `id`s.
2. A mini-review cycle (user reviews the CR).
3. On approval, update the parent spec(s): bump `version`, flip `status` back to `review`, and note the CR reference.
4. Re-approval → implementation continues.

## SDD workflow

1. **Spec** → write/update specs in this folder (`.specs/`)
2. **Review** → user reviews and approves (status `approved`)
3. **Develop** → implement from the spec, one module at a time
4. **Verify** → validate against acceptance criteria + walk tests against the real `content/` tree
5. **Update spec** → keep specs in sync with reality (Spec-Anchored level; spec changes first, C15)

## Spec index

| File | Scope | Status |
|------|-------|--------|
| [`constitution.md`](./constitution.md) | Immutable project rules (C1–C15); changes require amendment | Ratified (v2.0.0) |
| [`myagentcontrol-spec.md`](./myagentcontrol-spec.md) | Master spec (MAC-MASTER): vision, goals, architecture, decisions, roadmap | Approved |
| [`modules/context-spec.md`](./modules/context-spec.md) | Context system (MAC-CTX): MVI, local-first resolution, navigation | Approved |
| [`modules/agents-spec.md`](./modules/agents-spec.md) | Agents (MAC-AG): core agents, subagents, frontmatter schema, permissions, delegation | Approved |
| [`modules/skills-spec.md`](./modules/skills-spec.md) | Skills (MAC-SK): the four vendored skills, SKILL.md validation, router.sh | Approved |
| [`modules/commands-spec.md`](./modules/commands-spec.md) | Commands (MAC-CMD): `/add-context`, `/commit`, `/test`, … | Approved |
| [`modules/evals-spec.md`](./modules/evals-spec.md) | Evals (MAC-EV): YAML cases, results JSON, dashboard | Approved |
| [`modules/cli-spec.md`](./modules/cli-spec.md) | CLI (MAC-CLI): the Rust binary, commands, validation, walk tests, wizards | Approved |

## Spec identification (ID registry)

| ID | Artifact | Purpose |
|----|----------|---------|
| `MAC-CONST` | `constitution.md` | Immutable project rules (C1–C15); outside the feature lifecycle |
| `MAC-MASTER` | `myagentcontrol-spec.md` | Master spec: goals, architecture, ADR table (D1–D11), NFRs, roadmap |
| `MAC-CTX` | `modules/context-spec.md` | Context system module |
| `MAC-AG` | `modules/agents-spec.md` | Agents module |
| `MAC-SK` | `modules/skills-spec.md` | Skills module |
| `MAC-CMD` | `modules/commands-spec.md` | Commands module |
| `MAC-EV` | `modules/evals-spec.md` | Evals module |
| `MAC-CLI` | `modules/cli-spec.md` | CLI binary module |

**Numbering policy:**

- **Architecture/module specs** use semantic IDs (`MAC-<MODULE>`): stable, descriptive, immune to renumbering churn.
- **Future feature specs** (e.g. OAC PRs mined as features) use **sequential numbers** (`SPEC-001`, `SPEC-002`, …) in `.specs/features/SPEC-001-<name>/` (each feature gets a folder with `spec.md`, plus `plan.md`/`tasks.md` once implementation starts). They deliberately do **not** live at the root or in `modules/`. When the first feature spec is created, add it to the Spec index and this registry.
- **Ready-to-copy template:** `.specs/features/_template/`; copy the folder, rename `SPEC-XXX` → next number + feature slug, fill the placeholders.
- Decisions inside specs are numbered separately (`D1`–`D11` in the master ADR table).

**Prefix map** (functional requirements, acceptance criteria, and error rule IDs per module):

| Module spec | FR IDs | AC IDs | Error rule IDs |
|---|---|---|---|
| Context | `CTX-1..7` | `AC-C1..4` | `CTX-2xx` |
| Agents | `AG-1..6` | `AC-A1..4` | `AG-2xx` |
| Skills | `SK-1..6` | `AC-S1..4` | `SK-2xx` |
| Commands | `CMD-1..5` | `AC-M1..3` (M avoids C clash with Context) | `CMD-2xx` |
| Evals | `EV-1..6` | `AC-E1..4` | `EV-2xx` |
| CLI | `CLI-1..8` | `AC-L1..8` (L for CLI) | `E100–E500` envelopes + module rule IDs |

**Error codes are two-tier:** the CLI reports a category envelope (`E200` schema, `E300` dangling reference, …) plus the specific module rule ID (e.g. `E200 [agents] … rule: AG-202`). See [`modules/cli-spec.md`](./modules/cli-spec.md) §7.

## File organization rationale

```
.specs/
├── README.md               # index, lifecycle, CR process (governance doc)
├── constitution.md         # MAC-CONST: immutable rules, global governance
├── myagentcontrol-spec.md  # MAC-MASTER: master/steering spec
├── modules/                # stable architecture specs (one per module)
│   ├── context-spec.md     # MAC-CTX
│   ├── agents-spec.md      # MAC-AG
│   ├── skills-spec.md      # MAC-SK
│   ├── commands-spec.md    # MAC-CMD
│   ├── evals-spec.md       # MAC-EV
│   └── cli-spec.md         # MAC-CLI
└── features/_template/     # feature-spec template (SPEC-001, …)
```

- **Root = governance & steering:** README (index), constitution (immutable rules), and the master spec (goals/architecture/roadmap).
- **`modules/` = architecture specs:** each stable system part gets one file (no monolithic spec, small context windows).
- **`features/` = transient work items:** mined PRs and one-off features, sequentially numbered.

## SDD best-practices compliance matrix

| Best practice | Where implemented |
|---|---|
| Spec as source of truth; code is derivative | Constitution C7; README lifecycle |
| Separation of *what* (specs) from *how* (implementation plan) | Master spec goals; module specs keep requirements separate from crate layout |
| Structured lifecycle with gating | README lifecycle table + frontmatter `status` |
| Change Request workflow for approved specs | README §Change Requests |
| Immutable constitution | `constitution.md` (C1–C15) |
| Machine-readable frontmatter (`id`, `type`, `status`, `depends_on`) | YAML frontmatter on every spec |
| Modular, focused spec files | Master + per-module structure |
| Testable acceptance criteria | Given/When/Then in every spec (constitution C10) |
| Concrete examples & edge cases | "Examples & Scenarios" sections in each module spec |
| ADRs with context/decision/consequences | Master spec §8 (MADR format) |
| NFRs with measurable numbers | Master spec §9 (NFR1–NFR6) |
| In/out scope boundaries | Master spec §3 Non-Goals |
| Task breakdown per phase | Master spec §10 Roadmap |
| Versioned specs, audit history | Frontmatter `version`/`updated`; git history |

> **Prose style (stop-slop skill, 2026-08):** prose avoids em-dashes (use
> colons/semicolons/commas). Em-dashes remain only in document titles
> (frontmatter `title:` + `# H1`/`H2`), table placeholder cells (`| — |` =
> n/a), and the error-output format: cli-spec §10.3, the `Display` impl in
> `src/core/errors.rs` (`rule: {rule} — {message}`), and test expectations
> that mirror it.

## Status legend (document body tables)

- **Draft**: being written, not yet approved
- **Approved**: user reviewed, implementation may begin
- **Implemented**: code exists and passes acceptance criteria
- **Revised**: spec updated after implementation feedback
- **Ratified**: constitution only; in force (outside the feature lifecycle)
- **Amended**: constitution only; changed via version bump
