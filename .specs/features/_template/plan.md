# Plan: SPEC-XXX — <Feature Name>

> Companion to [`spec.md`](./spec.md) and [`tasks.md`](./tasks.md). Source of truth: `.specs/`.

## Objective

[One-sentence: what SPEC-XXX delivers and how it fits the master roadmap.]

## Approach

1. [Step 1 — e.g. extend the module validator]
2. [Step 2 — e.g. add walk-test coverage against the real `content/` tree]
3. [Step 3 — e.g. wire the CLI subcommand]

## Files touched (expected)

- `src/<module>/…`
- `tests/<name>_walk.rs` (if content changes)
- `content/…` (if vendored tree changes — as intentional divergence, C6)

## Testing

`cargo test` (unit + integration + walk), `cargo clippy -- -D warnings`, `cargo fmt --check` (constitution C12 / NFR4).

## Scale/Scope

[Estimate: number of files, LOC, affected modules.]

## Checklist (constitution mapping)

- [ ] **C7** — spec approved before implementation
- [ ] **C10** — acceptance criteria are Given/When/Then, objectively verifiable
- [ ] **C15** — spec updated first if behavior drifts
