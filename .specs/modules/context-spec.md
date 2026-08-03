---
id: MAC-CTX
type: module-spec
parent: MAC-MASTER
title: Context System — Module Spec
status: approved
version: 0.0.2
updated: 2026-08-03
change_requests: []
depends_on: [MAC-MASTER]
---

# Context System — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 0.0.2 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | Vendored `content/context/` (OAC v0.7.1 as starting point) + `CONTEXT_SYSTEM_GUIDE.md` from the original repo |
| **Note** | Rewritten 2026-08-03 under the format-fidelity principle (C16). The navigation cross-reference validator (old CTX-4) was **removed**: OAC does not enforce it and its own tree does not satisfy it, by design (`navigation.md` is a curated, token-efficient map, not a manifest) |

---

## 1. Purpose

The context system is OAC's "secret weapon": project coding standards and patterns stored as markdown, loaded by agents *before* code generation, using the **MVI (Minimal Viable Information)** principle to keep token usage ~80% lower than loading a whole codebase.

Our Rust tool must be able to **validate and maintain** this tree, including its metadata conventions (HTML-comment frontmatter, priority levels, versions) and its resolution rules (local-first, global fallback for `core/` only). The tree itself is vendored under `content/context/` (source of truth, constitution C6). Per C16, the tool validates **only** rules the context format declares; it does not invent new integrity rules (the removed CTX-4 is the canonical example).

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

## 3. MVI Rules (declared by the context format; must be validated)

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
   `References`, `Reference`, `Quick Reference` ("any reference section" replaces
   the literal heading, user decision). Discovery files (`navigation.md`,
   `index.md`, `README.md`, `CODEBASE_STANDARDS.md`) and reference docs are exempt.
7. `navigation.md` is updated by the `/add-context` wizard when files are
   created/modified (Quick Routes or Deep Dives table). Navigation cross-reference
   completeness is **not** machine-validated (C16; removed 2026-08-03).

> **Documented deviations:** the vendored tree contains a few files that do not
> follow the §3.3 rule (compatibility-shim comments, YAML `---` frontmatter in
> some files, concept docs without frontmatter, one `Priority: reference`). The
> parser stays **strict** per this spec; those files are tracked as a documented
> allowlist in `tests/context_walk.rs`. Genuine defects are fixed in the tree
> (C16 policy).

## 4. Context Resolution Rules (declared by ContextScout; to implement in Rust)

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

Markers (C16): `[OAC format]` validates a rule the context format/ContextScout
declares; `[tool DX]` is a user-approved developer-experience feature.

- **CTX-1** `[OAC format]` Parse and validate HTML-comment frontmatter metadata (category, priority, version, updated).
- **CTX-2** `[OAC format]` Validate MVI constraints on **concept cards** (< 200 lines, non-discovery): required reference section → `CTX-208`. Files ≥ 200 lines are **reference docs** (exempt); discovery files (`navigation.md`, `index.md`, `README.md`, `CODEBASE_STANDARDS.md`) are exempt from the reference-section rule.
- **CTX-3** `[OAC format]` Implement local-first / global-fallback resolution as a pure function (testable without a filesystem by injecting a glob impl; ≤ 2 checks).
- **CTX-4** `[OAC format]` `install` copies the vendored `content/context/` tree (C6); validate operates on the real tree.
- **CTX-5** `[tool DX]` Provide `/add-context` wizard equivalent (6-question Project Intelligence wizard → `project-intelligence/technical-domain.md`), following the original `/add-context` command rules. The wizard updates `navigation.md` so newly created files are reachable from the map.
- **CTX-6** `[tool DX]` `--update` mode increments version and refreshes the `Updated` date per the versioning rules (minor on content update, major on structure change; CTX-5 version rules).
- **CTX-7** `[OAC format]` Enforce the OAC `@`-reference syntax convention (from the original `validate-context-refs.sh`) in agent and command files: reject dynamic references (`@$var`), flag non-standard `@` references, and allowlist `@.opencode/context/...`, `@AGENTS.md`, `@.cursorrules`, `@$N` positional args, and email/mailto. This is a **forward syntax** rule (the file's own references must follow the convention); it does **not** check navigation completeness (removed CTX-4, C16).
- **CTX-8** `[tool DX]` External context cache: manage cached external documentation under `.tmp/external-context/` with a JSON manifest (add/update/list/remove), mirroring the original `manage-external-context.sh` and pairing with ExternalScout.

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

### 7.4 `@`-reference syntax (CTX-7)

Valid (allowlisted):
- `@.opencode/context/core/standards/code-quality.md`
- `@AGENTS.md`
- `@$1`, `@$2` (positional arguments)
- `team@example.com`, `mailto:team@example.com`

Invalid (each must be rejected):
- `@${var}` or `@$path` (dynamic reference) → error CTX-209
- `@some-other-place` (non-standard reference) → error CTX-210

## 8. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-C1** Given the vendored `content/context/` tree, **when** `myagentcontrol validate --context` runs, **then** it passes with exit 0 (allowlisted deviations excluded).
- **AC-C2** Given a context file with a broken priority value, missing frontmatter, or a concept card missing a reference section, **when** validation runs, **then** each defect is detected with a specific error code (e.g. `CTX-201`, `CTX-208`).
- **AC-C3** Given the resolution scenarios in §7.1, **when** the resolver runs with injected glob results, **then** it returns exactly the expected `{core_root}` for all five rows.
- **AC-C4** Given the `content/context/` tree, **when** the context walk test runs, **then** it validates structure + frontmatter on every file (always-on, no external checkout).
- **AC-C5** Given an agent or command file containing a dynamic `@` reference or a non-standard `@` reference, **when** `validate` runs, **then** the defect is reported with `CTX-209`/`CTX-210`; **when** only allowlisted references are present, **then** it passes.

## 9. Cross-References

- Agent that consumes this: `ContextScout` → [`agents-spec.md`](./agents-spec.md)
- Command that writes this: `/add-context` → [`commands-spec.md`](./commands-spec.md)
- CLI surface: `validate`, `wizard add-context`, `--update` → [`cli-spec.md`](./cli-spec.md)
