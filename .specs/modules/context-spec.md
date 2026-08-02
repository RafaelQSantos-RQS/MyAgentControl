---
id: MAC-CTX
type: module-spec
parent: MAC-MASTER
title: Context System — Module Spec
status: approved
version: 1.1.0
updated: 2026-08-02
change_requests: []
depends_on: [MAC-MASTER]
---

# Context System — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 1.1.0 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | Vendored `content/context/` (OAC v0.7.1 as starting point) + `CONTEXT_SYSTEM_GUIDE.md` from the original repo |

---

## 1. Purpose

The context system is OAC's "secret weapon": project coding standards and patterns stored as markdown, loaded by agents *before* code generation, using the **MVI (Minimal Viable Information)** principle to keep token usage ~80% lower than loading a whole codebase.

Our Rust tool must be able to **validate and maintain** this tree, including its metadata conventions (HTML-comment frontmatter, priority levels, versions) and its resolution rules (local-first, global fallback for `core/` only). The tree itself is vendored under `content/context/` (source of truth, constitution C6).

## 2. Directory Structure (vendored, `content/context/`)

```
context/
├── navigation.md                  # entry point: quick routes + deep dives tables
├── index.md
├── CODEBASE_STANDARDS.md
├── core/                          # universal standards (standards/, workflows/, task-management/, context-system/)
├── ui/                            # web styling/animation/react patterns, design systems
├── development/                   # backend-navigation, ui-navigation, language patterns
├── project-intelligence/          # technical-domain, business-domain, navigation
├── product/  data/  learning/  content-creation/  system-builder-templates/
├── openagents-repo/  project/
```

## 3. MVI Rules (must be validated)

1. Files < **200 lines** (scannable < 30s). Files ≥ 200 lines are classified as
   **reference docs** and are **exempt** from the MVI formula (user decision).
2. MVI formula per file: 1–3 sentence concept, 3–5 key points, 5–10 line example, reference link.
3. All files start with HTML-comment frontmatter:

```html
<!-- Context: {category}/{function} | Priority: {level} | Version: X.Y | Updated: YYYY-MM-DD -->
```

4. Priority assignment: **critical** (80% usage) > **high** (15%) > **medium** (4%) > **low** (1%).
5. Version tracking: new file → 1.0; content update → minor; structure change → major.
6. **Concept cards** (< 200 lines, non-discovery) MUST include a reference section:
   any of `Codebase References`, `Related Context`, `Related Files`, `Related`,
   `References`, `Reference`, `Quick Reference`, linking context to related
   context/code ("any reference section" replaces the literal heading, user decision).
   Discovery files (`navigation.md`, `index.md`, `README.md`, `CODEBASE_STANDARDS.md`)
   and reference docs are exempt.
7. `navigation.md` MUST be updated when files are created/modified (Quick Routes or Deep Dives table).

> **Documented deviations:** the vendored tree contains a few files that do not
> follow the §3.3 rule (compatibility-shim comments, YAML `---` frontmatter in
> some files, concept docs without frontmatter, one `Priority: reference`). The
> parser stays **strict** per this spec; those files are tracked as a documented
> allowlist in `tests/context_walk.rs`.

## 4. Context Resolution Rules (to implement in Rust)

From `CONTEXT_SYSTEM_GUIDE.md` + `contextscout.md`:

1. **One-time startup check** (max 2 glob checks, never per-file):
   - `glob("{local}/core/navigation.md")` → found → local is the single source. Done.
   - Not found → read `paths.json` `global` value. If `false`/missing → local only, no fallback.
   - Else `glob("{global}/core/navigation.md")` → found → use global **only for `core/`** files.
   - Set `{core_root}` = whichever path has core. All other categories (project-intelligence, ui, …) stay local.
2. **Local always wins**: if a local install exists, global is never consulted.
3. **Global fallback is ONLY for `core/`**: project-intelligence is never loaded from global.
4. **custom_dir**: if `paths.json` sets `custom_dir` (e.g. `.context`, `.ai/context`), that replaces the default `.opencode/context/` root.

## 5. paths.json

Optional. Shape:
```json
{
  "custom_dir": ".opencode/context",
  "global": "~/.config/opencode"
}
```

## 6. Functional Requirements

- **CTX-1** Parse and validate HTML-comment frontmatter metadata (category, priority, version, updated).
- **CTX-2** Validate MVI constraints on **concept cards** (< 200 lines, non-discovery):
  required reference section → `CTX-208`. Files ≥ 200 lines are **reference docs**
  (exempt); discovery files (`navigation.md`, `index.md`, `README.md`,
  `CODEBASE_STANDARDS.md`) are exempt from the reference-section rule.
  (Navigation registration is validated under CTX-4.)
- **CTX-3** Implement local-first / global-fallback resolution as a pure function (testable without a filesystem by injecting a glob impl).
- **CTX-4** Validate `navigation.md` cross-references (every listed file exists; every context file is listed or is an index/discovery file).
- **CTX-5** `init` copies the vendored `content/context/` tree (C6); validate operates on the real tree.
- **CTX-6** Provide `add-context` wizard equivalent (6-question Project Intelligence wizard → `project-intelligence/technical-domain.md`), following the original `/add-context` command rules.
- **CTX-7** `--update` mode increments version and refreshes `Updated` date per versioning rules.

## 7. Examples & Scenarios

### 7.1 Context resolution matrix (unit-test cases)

| Scenario | Local core? | Global path? | custom_dir? | Expected `{core_root}` |
|---|---|---|---|---|
| Local install | ✓ | — | — | `{local}` for everything |
| Global-only | ✗ | `~/.config/opencode` with core | — | `{global}/core/` for core only; rest local |
| Neither | ✗ | `false`/missing | — | local only, no fallback |
| Custom dir | — | — | `.context` | `.context/` replaces default root |
| Global without core | ✗ | path exists but no core | — | no fallback (max 2 glob checks) |

### 7.2 Frontmatter example (valid)

```html
<!-- Context: core/standards/code-quality | Priority: critical | Version: 1.2 | Updated: 2026-08-02 -->
```

### 7.3 Frontmatter examples (invalid; each must be rejected)

- `Priority: urgent` (not in {critical, high, medium, low})
- `Version: x.y` (not semver)
- No frontmatter comment at all
- `Updated: 08/02/2026` (not YYYY-MM-DD)

## 8. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-C1** Given the vendored `content/context/` tree, **when** `myagentcontrol validate --context` runs, **then** it passes with exit 0 (allowlisted deviations excluded).
- **AC-C2** Given a context file with a broken priority value, missing frontmatter, a concept card missing a reference section, or a dangling navigation link, **when** validation runs, **then** each defect is detected with a specific error code (e.g. `CTX-201`, `CTX-208`).
- **AC-C3** Given the resolution scenarios in §7.1, **when** the resolver runs with injected glob results, **then** it returns exactly the expected `{core_root}` for all five rows.
- **AC-C4** Given the `content/context/` tree, **when** the context walk test runs, **then** it validates structure + frontmatter on every file (always-on, no external checkout).

## 9. Cross-References

- Agent that consumes this: `ContextScout` → [`agents-spec.md`](./agents-spec.md)
- Command that writes this: `/add-context` → [`commands-spec.md`](./commands-spec.md)
- CLI surface: `validate`, `wizard add-context` → [`cli-spec.md`](./cli-spec.md)
