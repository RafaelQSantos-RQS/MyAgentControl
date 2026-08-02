---
id: MAC-CONST
type: constitution
title: MyAgentControl — Constitution
status: ratified
version: 2.1.0
updated: 2026-08-02
depends_on: []
---

# MyAgentControl — Constitution

> **Immutable rules.** This file is the global, non-negotiable baseline for *every* spec and *every* line of code in this project. It is deliberately short. Changing the constitution requires a full project review and explicit user approval — not a normal spec edit.

> **Lifecycle note:** the constitution is a special-case artifact, **outside** the feature spec lifecycle (draft → review → approved → …). Its only statuses are `ratified` (in force) and `amended` (after a version bump).

| | |
|---|---|
| **Status** | Ratified |
| **Version** | 2.1.0 |
| **Updated** | 2026-08-02 |

> **v2.0.0 amendment (2026-08-02, user decision):** the `.specs/` folder was
> **rebuilt from scratch** (previous specs archived in `.specs-old/`). The
> project is now defined as *its own vision* of OpenAgentsControl — OAC v0.7.1
> is the historical **starting point**, not a moving upstream dependency and
> not a parity yardstick. C6 was rewritten accordingly (no external golden
> diff against an OAC checkout).

> **v2.1.0 amendment (2026-08-02, user decision):** pre-release, spec changes
> are edited **directly** (with a `version` bump); the Change Request ceremony
> applies only to **released** specs (see `README.md` §Change Requests). C11
> updated to match; the CR trail only starts at v1.

---

## 1. Identity

1. **C1.** The project is `myagentcontrol` — a **Rust reimplementation** in the spirit of [OpenAgentsControl](https://github.com/darrenhinde/OpenAgentsControl) v0.7.1, developed under Spec-Driven Development (SDD). OAC v0.7.1 is the **starting point**; the project evolves as its own version of the framework.
2. **C2.** License: **MIT**, with attribution to the original OpenAgentsControl project (see `NOTICE.md`).

## 2. Architecture Non-Negotiables

3. **C3.** The Rust binary is a **configuration manager**: it generates, validates, and maintains the `.opencode/`-compatible structure. It **never** executes agents, calls model APIs, or invokes the OpenCode CLI at runtime. (The `evals run` subcommand is deferred to post-v1.)
4. **C4.** The managed format is **markdown + YAML frontmatter**, byte-compatible with what OpenCode loads. No proprietary Rust-only config format may be introduced without a constitution amendment.
5. **C5.** **Model-agnosticism is sacred.** No code, spec, or dependency may assume a single AI vendor. Execution is delegated to the user's chosen CLI (OpenCode), which handles providers.
6. **C6.** **`content/` is the source of truth.** The full managed tree is vendored in this repository under `content/` and maintained **in-repo**: it is allowed to diverge intentionally from OAC v0.7.1 (adopted PRs, project-specific changes) and is **never re-fetched from upstream**. Structural integrity is machine-checked by **always-on walk tests against the real `content/` tree** — never against an external OAC checkout (D8).

## 3. SDD Process Rules

7. **C7.** No implementation happens without an **Approved** spec. A spec is a contract; code is its derivative.
8. **C8.** Specs are written in **English**.
9. **C9.** Specs are **modular**: one master spec + one spec per module. Monolithic single-file specs are forbidden.
10. **C10.** Acceptance criteria are written in **Given/When/Then** form and must be objectively verifiable (no "fast", "nice", "robust" without a number).
11. **C11.** **Released** specs are edited only via a **Change Request** (see `README.md` §Change Requests), never silently in place. **Pre-release (pre-v1)**, specs are still being shaped and are edited directly with a `version` bump (v2.1.0 amendment).

## 4. Quality Gates

12. **C12.** Rust code: `cargo test` green, `cargo clippy -- -D warnings` clean, `cargo fmt --check` clean, zero `unsafe` (NFR4).
13. **C13.** The binary must not require node/bun at runtime (NFR5).
14. **C14.** `init` is idempotent and non-destructive (NFR2, NFR6).
15. **C15.** Specs and code must never drift: when behavior changes, the spec changes first (SDD Spec-Anchored level).

## 5. Changing This Constitution

Any amendment requires:
1. A proposal describing the rule to change and its motivation.
2. Explicit user (owner) approval.
3. A version bump (e.g. `2.1.0`, `3.0.0`) and an entry in the Change Log.

## 6. Change Log

| Version | Date | Change |
|---|---|---|
| 2.1.0 | 2026-08-02 | C11 scoped to **released** specs; pre-release edits are direct (version bump only); CR ceremony starts at v1 (user decision, no-CR pre-release) |
| 2.0.0 | 2026-08-02 | Full `.specs/` rebuild; C6 rewritten (content/ source of truth, self-referential walk tests, no external OAC parity) |
| 1.0.0 | 2026-08-02 | Original constitution (superseded by the 2.0.0 rebuild) |
