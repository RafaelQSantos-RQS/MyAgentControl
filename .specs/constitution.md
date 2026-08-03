---
id: MAC-CONST
type: constitution
title: MyAgentControl — Constitution
status: ratified
version: 0.0.1
updated: 2026-08-03
depends_on: []
---

# MyAgentControl — Constitution

> **Immutable rules.** This file is the global, non-negotiable baseline for *every* spec and *every* line of code in this project. It is deliberately short. Changing the constitution requires a full project review and explicit user approval, not a normal spec edit.

> **Lifecycle note:** the constitution is a special-case artifact, **outside** the feature spec lifecycle (draft → review → approved → …). Its only statuses are `ratified` (in force) and `amended` (after a version bump).

| | |
|---|---|
| **Status** | Ratified |
| **Version** | 0.0.1 |
| **Updated** | 2026-08-03 |

> **Amendment (2026-08-02, user decision):** the `.specs/` folder was
> **rebuilt from scratch**. The project is now defined as *its own vision* of
> OpenAgentsControl: OAC v0.7.1 is the historical **starting point**, not a
> moving upstream dependency and not a parity yardstick. C6 was rewritten
> accordingly (no external golden diff against an OAC checkout).

> **Amendment (2026-08-02, user decision):** pre-release, spec changes
> are edited **directly** (with a `version` bump); the Change Request ceremony
> applies only to **released** specs (see `README.md` §Change Requests). C11
> updated to match; the CR trail only starts at v1.

> **Amendment (2026-08-03, user decision):** **C16 added: format
> fidelity.** The tool validates **only** rules that the managed formats
> declare (OAC/OpenCode). It never invents integrity rules beyond those
> formats. The former CTX-4 navigation rule ("every context file must be
> listed") was removed as such an invention. Walk-test deviation policy
> codified: genuine defects are fixed in the tree, intentional deviations are
> documented per-file in the walk tests. The full spec suite was rewritten in
> this amendment to carry the principle through (module specs now mark each
> functional requirement as `[OAC format]` or `[tool DX]`).

---

## 1. Identity

1. **C1.** The project is `myagentcontrol`, a **Rust reimplementation** in the spirit of [OpenAgentsControl](https://github.com/darrenhinde/OpenAgentsControl) v0.7.1, developed under Spec-Driven Development (SDD). OAC v0.7.1 is the **starting point**; the project evolves as its own version of the framework.
2. **C2.** License: **MIT**, with attribution to the original OpenAgentsControl project (see `NOTICE.md`).

## 2. Architecture Non-Negotiables

3. **C3.** The Rust binary is a **configuration manager**: it generates, validates, and maintains the `.opencode/`-compatible structure. It **never** executes agents, calls model APIs, or invokes the OpenCode CLI at runtime. (The `evals run` subcommand is deferred to post-v1.)
4. **C4.** The managed format is **markdown + YAML frontmatter**, byte-compatible with what OpenCode loads. No proprietary Rust-only config format may be introduced without a constitution amendment.
5. **C5.** **Model-agnosticism is sacred.** No code, spec, or dependency may assume a single AI vendor. Execution is delegated to the user's chosen CLI (OpenCode), which handles providers.
6. **C6.** **`content/` is the source of truth.** The full managed tree is vendored in this repository under `content/` and maintained **in-repo**: it is allowed to diverge intentionally from OAC v0.7.1 (adopted PRs, project-specific changes) and is **never re-fetched from upstream**. Structural integrity is machine-checked by **always-on walk tests against the real `content/` tree**, never against an external OAC checkout (D8).
7. **C16.** **Format fidelity.** The tool validates **only** rules that the managed formats declare: frontmatter fields, permission verbs, MVI thresholds, context resolution rules, dependency references, the SKILL.md contract. It **never** invents integrity rules beyond those formats (for example, no "every context file must be listed in a navigation file"). Developer-experience features (wizards, `list` output, evals dashboard) are explicit user-approved additions, not format rules. When the vendored tree deviates from a declared rule, walk tests document each deviation per-file: genuine defects are **fixed in the tree** (preferred), intentional deviations are allowlisted with a reason.

## 3. SDD Process Rules

8. **C7.** No implementation happens without an **Approved** spec. A spec is a contract; code is its derivative.
9. **C8.** Specs are written in **English**.
10. **C9.** Specs are **modular**: one master spec + one spec per module. Monolithic single-file specs are forbidden.
11. **C10.** Acceptance criteria are written in **Given/When/Then** form and must be objectively verifiable (no "fast", "nice", "robust" without a number).
12. **C11.** **Released** specs are edited only via a **Change Request** (see `README.md` §Change Requests), never silently in place. **Pre-release (pre-v1)**, specs are still being shaped and are edited directly with a `version` bump (2026-08-02 amendment).

## 4. Quality Gates

13. **C12.** Rust code: `cargo test` green, `cargo clippy -- -D warnings` clean, `cargo fmt --check` clean, zero `unsafe` (NFR4).
14. **C13.** The binary must not require node/bun at runtime (NFR5).
15. **C14.** `install` is idempotent and non-destructive (NFR2, NFR6).
16. **C15.** Specs and code must never drift: when behavior changes, the spec changes first (SDD Spec-Anchored level).

## 5. Changing This Constitution

Any amendment requires:
1. A proposal describing the rule to change and its motivation.
2. Explicit user (owner) approval.
3. A version bump (e.g. `0.0.2`, `0.1.0`) and an entry in the Change Log.

## 6. Change Log

| Version | Date | Change |
|---|---|---|
| 0.0.1 | 2026-08-03 | C16 added (format fidelity): validate only OAC-declared rules; walk-test deviation policy codified (fix defects, allowlist intentional deviations); full spec suite rewritten with `[OAC format]`/`[tool DX]` markers; versions synchronized to the project version v0.0.1 |
| 0.0.0 | 2026-08-02 | Constitution ratified; initial C1–C15 (consolidates the 2026-08-02 `.specs/` rebuild and the C11 pre-release amendment) |
