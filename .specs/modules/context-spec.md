---
id: MAC-CTX
type: module-spec
parent: MAC-MASTER
title: Context System — Module Spec
status: approved
version: 0.1.0
updated: 2026-08-02
depends_on: [MAC-MASTER]
---

# Context System — Module Spec

| | |
|---|---|
| **Status** | Approved |
| **Version** | 0.1.0 |
| **Parent** | [`../myagentcontrol-spec.md`](../myagentcontrol-spec.md) |
| **Reference** | OAC repo at tag `v0.7.1`: [`/opencode/context/`](https://github.com/darrenhinde/OpenAgentsControl/tree/v0.7.1/.opencode/context/) + [`CONTEXT_SYSTEM_GUIDE.md`](https://github.com/darrenhinde/OpenAgentsControl/blob/v0.7.1/CONTEXT_SYSTEM_GUIDE.md) |

---

## 1. Purpose

The context system is OAC's "secret weapon": project coding standards and patterns stored as markdown, loaded by agents *before* code generation, using the **MVI (Minimal Viable Information)** principle to keep token usage ~80% lower than loading a whole codebase.

Our Rust tool must be able to **generate, validate, and maintain** this tree with full parity, including its metadata conventions (HTML-comment frontmatter, priority levels, versions) and its resolution rules (local-first, global fallback for `core/` only).

## 2. Directory Structure (target parity)

```
context/
├── navigation.md                  # entry point: quick routes + deep dives tables
├── index.md
├── CODEBASE_STANDARDS.md
├── core/                          # universal standards (standards/, workflows/, task-management/, context-system/)
│   ├── standards/{code-quality,security-patterns,test-coverage,documentation,project-intelligence}.md
│   ├── workflows/{design-iteration,task-delegation,external-libraries,code-review}.md
│   ├── task-management/standards/task-schema.md
│   └── context-system/standards/{mvi,frontmatter}.md
├── ui/web/{ui-styling-standards,animation-patterns,react-patterns,design-systems}.md
├── development/{backend-navigation,ui-navigation,[language-patterns]}.md
├── project-intelligence/{technical-domain,business-domain,navigation}.md
├── product/  data/  learning/  content-creation/  system-builder-templates/  openagents-repo/  project/
```

## 3. MVI Rules (must be validated)

1. Files < **200 lines** (scannable < 30s).
2. MVI formula per file: 1–3 sentence concept, 3–5 key points, 5–10 line example, reference link.
3. All files start with HTML-comment frontmatter:

```html
<!-- Context: {category}/{function} | Priority: {level} | Version: X.Y | Updated: YYYY-MM-DD -->
```

4. Priority assignment: **critical** (80% usage) > **high** (15%) > **medium** (4%) > **low** (1%).
5. Version tracking: new file → 1.0; content update → minor; structure change → major.
6. Files MUST include a "📂 Codebase References" section linking context → actual code.
7. `navigation.md` MUST be updated when files are created/modified (Quick Routes or Deep Dives table).

## 4. Context Resolution Rules (to implement in Rust)

From `CONTEXT_SYSTEM_GUIDE.md` + `contextscout.md`:

1. **One-time startup check** (max 2 glob checks, never per-file):
   - `glob("{local}/core/navigation.md")` → found → local is the single source. Done.
   - Not found → read `paths.json` `global` value. If `false`/missing → local only, no fallback.
   - Else `glob("{global}/core/navigation.md")` → found → use global **only for `core/`** files.
   - Set `{core_root}` = whichever path has core. All other categories (project-intelligence, ui, …) stay local.
2. **Local always wins** — if a local install exists, global is never consulted.
3. **Global fallback is ONLY for `core/`** — project-intelligence is never loaded from global.
4. **custom_dir** — if `paths.json` sets `custom_dir` (e.g. `.context`, `.ai/context`), that replaces the default `.opencode/context/` root.

## 5. paths.json

Optional. Shape:
```json
{
  "custom_dir": ".opencode/context",   // or false
  "global": "~/.config/opencode"        // or false
}
```

## 6. Functional Requirements

- **CTX-1** Parse and validate HTML-comment frontmatter metadata (category, priority, version, updated).
- **CTX-2** Validate MVI constraints: line count < 200, required sections, navigation registration.
- **CTX-3** Implement local-first / global-fallback resolution as a pure function (testable without a filesystem by injecting a glob impl).
- **CTX-4** Validate `navigation.md` cross-references (every listed file exists; every context file is listed or is an index/discovery file).
- **CTX-5** Scaffold the full context tree (all categories above) with correct metadata.
- **CTX-6** Provide `add-context` wizard equivalent (6-question Project Intelligence wizard → `project-intelligence/technical-domain.md`), following the original `/add-context` command rules (project_intelligence, frontmatter_required, mvi_compliance, codebase_refs, navigation_update, priority_assignment, version_tracking).
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

### 7.3 Frontmatter examples (invalid — each must be rejected)

- `Priority: urgent` (not in {critical, high, medium, low})
- `Version: x.y` (not semver)
- No frontmatter comment at all
- `Updated: 08/02/2026` (not YYYY-MM-DD)

## 8. Acceptance Criteria

Given/When/Then form per constitution C10.

- **AC-C1** Given the pristine reference tree, **when** `myagentcontrol validate --context` runs, **then** it passes with exit 0.
- **AC-C2** Given a context file with a broken priority value, missing frontmatter, >200 lines, or a dangling navigation link, **when** validation runs, **then** each defect is detected with a specific error code (e.g. `CTX-201`).
- **AC-C3** Given the resolution scenarios in §7.1, **when** the resolver runs with injected glob results, **then** it returns exactly the expected `{core_root}` for all five rows.
- **AC-C4** Given a scaffolded context tree, **when** compared to the reference, **then** the diff is clean with `Updated` dates normalized.

## 9. Cross-References

- Agent that consumes this: `ContextScout` → [`agents-spec.md`](./agents-spec.md)
- Command that writes this: `/add-context` → [`commands-spec.md`](./commands-spec.md)
- CLI surface: `validate`, `wizard add-context` → [`cli-spec.md`](./cli-spec.md)
