# MyAgentControl — Specs (SDD)

> **Spec-Driven Development.** This folder is the single source of truth for *what* we are building. Code is written only after a spec exists and is approved. The immutable project rules live in [`constitution.md`](./constitution.md).

## What is this project?

A rewrite of [OpenAgentsControl (OAC)](https://github.com/darrenhinde/OpenAgentsControl) in **Rust** (`myagentcontrol`).

The original OAC is a model-agnostic AI agent framework (agents, skills, commands, context system, eval harness) distributed as markdown/config files that run on top of the OpenCode CLI. Our Rust version is a **configuration manager**: it copies the vendored `content/` tree (source of truth, per [CR-001](./changes/CR-001-cr.md)), validates, and maintains the exact same `.opencode/`-style structure (agents, subagents, skills, commands, context, evals) so users can keep using the OpenCode CLI they already rely on — with feature parity against the original.

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

**Change Request trail:** specs updated via the CR process also carry `change_requests: [<ID>, …]` in their frontmatter (e.g. `[CR-001]`) and a body-table row pointing at the CR file, so the audit trail is machine-readable and human-visible.

## Change Requests (editing an approved spec)

Approved specs are never edited silently in place (constitution C11). Changes go through a lightweight Change Request:

1. Open a CR (file: `.specs/changes/<id>-cr.md`) describing the change, motivation, and affected spec `id`s.
2. A mini-review cycle (user reviews the CR).
3. On approval, update the parent spec(s): bump `version`, flip `status` back to `review`, and note the CR reference.
4. Re-approval → implementation continues.

## SDD workflow

1. **Spec** → write/update specs in this folder (`.specs/`)
2. **Review** → user reviews and approves (status `approved`)
3. **Develop** → implement from the spec, one module at a time
4. **Verify** → validate against acceptance criteria + golden tests vs the reference repo
5. **Update spec** → keep specs in sync with reality (Spec-Anchored level; spec changes first, C15)

## Spec index

| File | Scope | Status |
|------|-------|--------|
| [`constitution.md`](./constitution.md) | Immutable project rules (C1–C15) — changes require amendment | Ratified |
| [`myagentcontrol-spec.md`](./myagentcontrol-spec.md) | Master spec (MAC-MASTER): vision, goals, architecture, decisions, roadmap | Approved |
| [`modules/context-spec.md`](./modules/context-spec.md) | Context system (MAC-CTX): MVI, local-first resolution, navigation | Approved |
| [`modules/agents-spec.md`](./modules/agents-spec.md) | Agents (MAC-AG): core agents, subagents, frontmatter schema, permissions, delegation | Approved |
| [`modules/skills-spec.md`](./modules/skills-spec.md) | Skills (MAC-SK): project-orchestration, task-management, smart-router, context7, context-manager | Approved |
| [`modules/commands-spec.md`](./modules/commands-spec.md) | Commands (MAC-CMD): `/add-context`, `/commit`, `/test`, … | Approved |
| [`modules/evals-spec.md`](./modules/evals-spec.md) | Evals (MAC-EV): YAML cases, results JSON, dashboard | Approved |
| [`modules/cli-spec.md`](./modules/cli-spec.md) | CLI (MAC-CLI): the Rust binary, commands, validation, golden tests, wizards | Approved |

## Spec identification (ID registry)

Every spec carries a machine-readable `id` in its YAML frontmatter. IDs follow a **semantic scheme**, not sequential numbers — this is the SDD convention for stable *architecture/module specs* (vs. transient *feature specs*, see below).

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

- **Architecture/module specs** use semantic IDs (`MAC-<MODULE>`) — stable, descriptive, and immune to renumbering churn. This is the *correct* scheme for specs that describe system parts (verified against Spec Kit / Kiro / Tessl practices).
- **Future feature specs** (e.g. OAC PRs mined as features) will use **sequential numbers** — `SPEC-001`, `SPEC-002`, … — following Spec Kit's `specs/<n>-<name>/` convention, because features are transient work items (propose → implement → archive). **Location:** `.specs/features/SPEC-001-<name>/` (each feature gets a folder with `spec.md`, plus `plan.md`/`tasks.md` once implementation starts). They deliberately do **not** live at the root or in `modules/` — root is reserved for governance, `modules/` for stable architecture specs. When the first feature spec is created, add it to the Spec index and this registry.
- **Ready-to-copy template:** `.specs/features/_template/` (Spec Kit–style `spec.md` + `plan.md` + `tasks.md`, adapted to the constitution). Copy the folder, rename `SPEC-XXX` → next number + feature slug, fill the placeholders.
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

Why three files at the root and the rest in `modules/`?

```
.specs/
├── README.md               # index, lifecycle, CR process (governance doc)
├── constitution.md         # MAC-CONST — immutable rules, global governance
├── myagentcontrol-spec.md  # MAC-MASTER — master/steering spec
└── modules/                # stable architecture specs (one per module)
    ├── context-spec.md     # MAC-CTX
    ├── agents-spec.md      # MAC-AG
    ├── skills-spec.md      # MAC-SK
    ├── commands-spec.md    # MAC-CMD
    ├── evals-spec.md       # MAC-EV
    └── cli-spec.md         # MAC-CLI
```

- **Root = governance & steering:** README (index), constitution (immutable rules, outside the feature lifecycle), and the master spec (goals/architecture/roadmap). This mirrors Spec Kit (`.specify/memory/constitution.md` at root) and Kiro (steering docs) — global rules load first, independent of any single feature.
- **`modules/` = architecture specs:** each stable system part gets one file (no monolithic spec, small context windows). Genuine *feature* specs (transient work items) will live in `.specs/features/SPEC-001-<name>/` — see the numbering policy above.
- **Not flattened deliberately:** putting all 9 files in one flat folder would erase the governance-vs-feature separation and force a restructure once `SPEC-001`+ feature specs arrive. For a small project the split reads as "messy" but it is the standard layout — document over restructure.

## SDD best-practices compliance matrix

How this folder maps to recognized SDD practices (Spec Kit, SPECLAN, MADR, Amazon Kiro, ISO/IEC/IEEE 29148):

| Best practice | Where implemented |
|---|---|
| Spec as source of truth; code is derivative | Constitution C7; README lifecycle |
| Separation of *what* (specs) from *how* (implementation plan) | Master spec goals; module specs keep requirements separate from crate layout (cli-spec §5) |
| Structured lifecycle with gating | README lifecycle table + frontmatter `status` |
| Change Request workflow for approved specs | README §Change Requests |
| Immutable constitution | `constitution.md` (C1–C15) |
| Machine-readable frontmatter (`id`, `type`, `status`, `depends_on`) | YAML frontmatter on every spec |
| Modular, focused spec files | Master + per-module structure (user decision R3Q1) |
| Testable acceptance criteria | Given/When/Then in every spec (constitution C10) |
| Concrete examples & edge cases | "Examples & Scenarios" sections in each module spec |
| ADRs with context/decision/consequences | Master spec §8 (MADR format) |
| NFRs with measurable numbers | Master spec §9 (NFR1–NFR6) |
| In/out scope boundaries | Master spec §3 Non-Goals |
| Task breakdown per phase | Master spec §10 Roadmap (per-phase tasks) |
| Versioned specs, audit history | Frontmatter `version`/`updated`; git history |

## Reference material

- **Original repo (source of truth):** [`darrenhinde/OpenAgentsControl`](https://github.com/darrenhinde/OpenAgentsControl) — pinned to tag `v0.7.1`. Golden tests and parity checks (D8) diff against a **local checkout** of this tag, e.g.:
  ```bash
  git clone --branch v0.7.1 https://github.com/darrenhinde/OpenAgentsControl .tmp/reference/OpenAgentsControl
  ```
  (`.tmp/` is gitignored — the checkout is a dev-only artifact, never committed.)

## Status legend (document body tables)

- **Draft** — being written, not yet approved
- **Approved** — user reviewed, implementation may begin
- **Implemented** — code exists and passes acceptance criteria
- **Revised** — spec updated after implementation feedback
- **Ratified** — constitution only: in force (outside the feature lifecycle)
- **Amended** — constitution only: changed via version bump
