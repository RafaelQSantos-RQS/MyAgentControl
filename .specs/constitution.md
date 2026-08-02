---
id: MAC-CONST
type: constitution
title: MyAgentControl — Constitution
status: ratified
version: 1.0.0
updated: 2026-08-02
depends_on: []
---

# MyAgentControl — Constitution

> **Immutable rules.** This file is the global, non-negotiable baseline for *every* spec and *every* line of code in this project. It is deliberately short. Changing the constitution requires a full project review and explicit user approval — not a normal spec edit.

> **Lifecycle note:** the constitution is a special-case artifact, **outside** the feature spec lifecycle (draft → review → approved → …). Its only statuses are `ratified` (in force) and `amended` (after a version bump).

| | |
|---|---|
| **Status** | Ratified |
| **Version** | 1.0.0 |
| **Updated** | 2026-08-02 |

---

## 1. Identity

1. **C1.** The project is `myagentcontrol` — a Rust rewrite of [OpenAgentsControl](https://github.com/darrenhinde/OpenAgentsControl) v0.7.1, developed under Spec-Driven Development (SDD).
2. **C2.** License: **MIT**, with attribution to the original OpenAgentsControl project (user decision R4Q4).

## 2. Architecture Non-Negotiables

3. **C3.** The Rust binary is a **configuration manager**: it generates, validates, and maintains the `.opencode/`-compatible structure. It **never** executes agents, calls model APIs, or invokes the OpenCode CLI at runtime. (D1; the `evals run` subcommand is deferred to post-v1.)
4. **C4.** The managed format is **markdown + YAML frontmatter**, byte-compatible with OAC (D2, D3). No proprietary Rust-only config format may be introduced without a constitution amendment.
5. **C5.** **Model-agnosticism is sacred.** No code, spec, or dependency may assume a single AI vendor. Execution is delegated to the user's chosen CLI (OpenCode), which handles providers.
6. **C6.** **Feature parity** with the reference repo is the yardstick: golden tests against a checkout of [`darrenhinde/OpenAgentsControl`](https://github.com/darrenhinde/OpenAgentsControl) at tag `v0.7.1` must pass (D8).

## 3. SDD Process Rules

7. **C7.** No implementation happens without an **Approved** spec. A spec is a contract; code is its derivative.
8. **C8.** Specs are written in **English** (D6).
9. **C9.** Specs are **modular**: one master spec + one spec per module. Monolithic single-file specs are forbidden (user decision R3Q1).
10. **C10.** Acceptance criteria are written in **Given/When/Then** form and must be objectively verifiable (no "fast", "nice", "robust" without a number).
11. **C11.** Approved specs are edited only via a **Change Request** (see `README.md` §Change Requests), never silently in place.

## 4. Quality Gates

12. **C12.** Rust code: `cargo test` green, `cargo clippy -- -D warnings` clean, `cargo fmt --check` clean, zero `unsafe` (NFR4).
13. **C13.** The binary must not require node/bun at runtime (NFR5).
14. **C14.** `init` is idempotent and non-destructive (NFR2, NFR6).
15. **C15.** Specs and code must never drift: when behavior changes, the spec changes first (SDD Spec-Anchored level).

## 5. Changing This Constitution

Any amendment requires:
1. A proposal describing the rule to change and its motivation.
2. Explicit user (owner) approval.
3. A version bump (`2.0.0`, etc.) and an entry in the Change Log.
