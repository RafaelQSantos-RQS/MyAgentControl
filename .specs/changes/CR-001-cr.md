---
id: CR-001
type: change-request
status: approved
created: 2026-08-02
approved: 2026-08-02
owner: Rafael (user)
affected_specs: [MAC-MASTER, MAC-CLI]
---

# CR-001 — Content distribution: vendored tree (Model C) + neutral `content/` folder

## Motivation

The master spec's decision **D1** states the Rust binary is a *configuration
manager* that does **not execute** agents. During review of Phase 0, the user
flagged that their original intent was closer to what OpenAgentsControl (OAC)
actually is: **a repository of static files** (450 files, 4.5 MB under
`.opencode/`, MIT-licensed) that OpenCode reads at execution time. D1 governs
the *execution* axis only; it never resolved the *distribution* axis — where
the content tree lives and how users obtain it.

The user also asked for the model-**agnostic** side: a neutral folder name
(not `.opencode/`) reinforces the project's model-agnostic positioning
(goal 2 / D5) by not tying the source-of-truth tree to any one backend CLI.

The current spec reads as "generate from embedded templates" (Model A). The
user wants a vendored tree as source of truth, with a **neutral folder name**
in the myagentcontrol repo (`content/`, not `.opencode/`), while keeping
drop-in compatibility with OpenCode at the destination.

## Decision (user-approved)

Adopt **Model C — Hybrid**:

1. **Vendored tree as source of truth.** The full OAC-compatible tree
   (agents, subagents, skills, commands, context, evals, profiles, prompts,
   registry, scripts) is committed in the myagentcontrol repo under
   **`content/`** (renamed from the `.opencode/` convention **in this repo
   only**). Sourced from [`darrenhinde/OpenAgentsControl`](https://github.com/darrenhinde/OpenAgentsControl)
   at tag `v0.7.1` (MIT + attribution, per D7).
2. **`init` copies, does not template.** `myagentcontrol init` copies the
   `content/` tree to `.opencode/` in the user's project, non-destructively
   and idempotently (NFR2/NFR6 preserved — C14). The destination folder in
   user projects **remains `.opencode/`** so OpenCode drop-in compatibility
   (D3, NFR1) is unchanged.
3. **Golden tests stay non-self-referential.** Golden diffs (D8) compare the
   vendored `content/` tree against the **external pinned checkout**
   (`.tmp/reference/OpenAgentsControl@v0.7.1`) — not against itself.
4. **Community PRs map directly.** OAC PRs that edit markdown files apply
   to the vendored files 1:1 (facilitates the "mine the PRs" goal).
5. **Managed divergence.** The vendored `content/` tree is allowed to
   *intentionally* diverge from pristine v0.7.1 when a community PR is
   adopted. That divergence is **managed**: recorded in the adopting CR,
   reviewed, and reflected as a golden-test delta (or a re-baseline per
   D8's own "upgrading the reference requires re-baselining" clause).
   Golden tests against the pinned v0.7.1 checkout remain the parity
   baseline; adopted-PR deltas become approved exceptions tracked as
   such, never silent drift.
6. **`content/` lives at the repo root.** Placement resolved (OQ-CR1):
   top-level `content/`, mirroring how OAC keeps `.opencode/` at its own
   repo root. No nesting under `assets/` or similar.
7. **Attribution via NOTICE + LICENSE + README** (OQ-CR2 resolved). The
   vendored files stay **verbatim** (no per-file license headers):
   - `NOTICE.md` at repo root credits OAC (MIT © 2025 Darren Hinde) and
     describes the vendored provenance.
   - Top-level `LICENSE` (MIT) covers the project; OAC attribution per D7.
   - README credits the source repo and pinned tag `v0.7.1`.
   Rationale: per-file headers would break byte-parity with the reference
   (D8/NFR1), risk frontmatter parsing by OpenCode (header before the
   `---` block), and add tokens to every agent's context (against goal 6).

## Affected Specs & Sections

| Spec | Section | Change |
|---|---|---|
| MAC-MASTER | §8 ADR table — D1 | Add distribution clarification: D1 covers execution only; distribution = Model C (vendored tree) per CR-001 |
| MAC-MASTER | §8 ADR table — D2 | Widen: keep markdown+YAML format (unchanged) + tree is vendored under `content/` |
| MAC-MASTER | §2 Goals — goal 4 | Golden tests diff `content/` vs pinned external checkout, with adopted-PR deltas handled via managed divergence (point 5) |
| MAC-MASTER | §8 ADR — D8 | Golden baseline policy gains "approved deltas via adopting CR; re-baseline per D8" — governs the managed-divergence mechanism |
| MAC-MASTER | §10 Roadmap — Phase 0 | Note: Phase 0 unchanged; vendoring lands in Phase 1+ |
| MAC-MASTER | §5 Architecture | Add `content/` (repo root) + `NOTICE.md` to the managed tree box (source of truth) |
| MAC-CLI | CLI-1 | `init` **copies** from `content/` (not "generates from templates"); still non-destructive/idempotent |
| MAC-CLI | §6 Crate layout | Add top-level `content/` dir; replace `src/scaffold/` note |
| MAC-CLI | §8 Golden tests | Diff `content/` against external pinned checkout; adopted-PR deltas handled via managed divergence (point 5) |

## Consequences

- **Easier:** faithful to the user's vision (repo *is* the big static
  collection); PR mining is near copy-paste; updates flow via `git pull`
  (mirrors OAC's `update.sh`); content browsable/editable by humans.
- **Harder:** repo grows by ~4.5 MB / ~450 files; two artifacts (tree +
  Rust) can drift — mitigated by golden tests **plus** CR-tracked deltas
  (point 5); attribution must be preserved via `NOTICE.md` + `LICENSE` +
  README credit, with vendored files kept verbatim (point 7, per D7).

## Non-Goals of this CR

- No change to the execution model (D1 still: no agent execution, no
  OpenCode invocation, no model API calls).
- No change to the destination folder in user projects (still `.opencode/`).
- No change to Phase 0 deliverables (skeleton/CLI/errors/golden harness).

## Definition of Done

1. CR-001 approved by the user (status → `approved`).
2. Specs updated per the table above, versions bumped, `status` flipped to
   `review` then re-approved, each noting "CR-001".
3. `content/` populated from OAC v0.7.1, together with top-level
   `NOTICE.md` (OAC provenance + attribution), `LICENSE` (MIT), and the
   README source credit (separate task, Phase 1).
4. Golden smoke test extended to diff `content/` vs the pinned checkout —
   **once `content/` exists** (Phase 1). Until then, Phase 0's existing
   fixture-based `tests/golden_smoke.rs` stays as-is and keeps passing.
   The Phase 0 fixture slice (`tests/golden/reference/`) is **kept** as a
   fast, dependency-free unit fixture after the transition (no plan to
   retire it).

## Open Questions (resolved)

- ~~OQ-CR1: Keep `content/` at repo root vs nested?~~ → **Resolved:** top-level `content/` (decision point 6).
- ~~OQ-CR2: Attribution — per-file header, top-level NOTICE, or both?~~ → **Resolved:** `NOTICE.md` + `LICENSE` + README credit, vendored files verbatim (decision point 7).
