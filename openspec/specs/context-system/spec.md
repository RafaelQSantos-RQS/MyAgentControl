## Purpose

Validate and maintain the context system tree: parse HTML-comment frontmatter, enforce MVI constraints on concept cards, resolve context paths (local-first, global fallback), run the `/add-context` wizard, validate `@`-reference syntax, and manage external context cache.

## ADDED Requirements

### Requirement: Frontmatter parsing and validation

The system SHALL parse HTML-comment frontmatter from context files and validate its fields: category (string), priority (one of `critical`, `high`, `medium`, `low`), version (semver `X.Y`), and updated (`YYYY-MM-DD`). Invalid values SHALL produce a validation error with a specific rule ID.

#### Scenario: Valid frontmatter
- **WHEN** a context file contains `<!-- Context: core/standards/code-quality | Priority: critical | Version: 1.2 | Updated: 2026-08-02 -->`
- **THEN** the parser extracts category=`core/standards/code-quality`, priority=`critical`, version=`1.2`, updated=`2026-08-02`

#### Scenario: Invalid priority
- **WHEN** a context file contains `Priority: urgent`
- **THEN** the validator emits error `CTX-201` with message indicating the invalid priority value

#### Scenario: Missing frontmatter
- **WHEN** a context file has no HTML-comment frontmatter line
- **THEN** the validator emits error `CTX-202` indicating missing frontmatter

#### Scenario: Invalid version format
- **WHEN** a context file contains `Version: x.y`
- **THEN** the validator emits error `CTX-203` indicating invalid semver format

#### Scenario: Invalid date format
- **WHEN** a context file contains `Updated: 08/02/2026`
- **THEN** the validator emits error `CTX-204` indicating the date must be `YYYY-MM-DD`

### Requirement: MVI concept card validation

The system SHALL validate that concept cards (files < 200 lines, non-discovery files) include a reference section. Discovery files (`navigation.md`, `index.md`, `README.md`, `CODEBASE_STANDARDS.md`) and reference docs (≥ 200 lines) are exempt.

#### Scenario: Concept card without reference section
- **WHEN** a context file is < 200 lines, is not a discovery file, and has no reference section heading
- **THEN** the validator emits error `CTX-208`

#### Scenario: Concept card with reference section
- **WHEN** a context file is < 200 lines and contains a `References` heading
- **THEN** validation passes for the reference-section rule

#### Scenario: Reference doc exempt
- **WHEN** a context file is ≥ 200 lines
- **THEN** the reference-section rule is not enforced

### Requirement: Local-first context resolution

The system SHALL resolve context paths using local-first, global-fallback logic with at most 2 glob checks. If a local `core/navigation.md` exists, it is the single source. If not, the `paths.json` `global` value is checked; if `core/` exists there, it is used only for `core/` files. `custom_dir` in `paths.json` replaces the default root.

#### Scenario: Local install exists
- **WHEN** `{local}/core/navigation.md` exists
- **THEN** `{core_root}` is set to `{local}` and no global check is performed

#### Scenario: Global fallback for core only
- **WHEN** `{local}/core/navigation.md` does not exist and `{global}/core/navigation.md` exists
- **THEN** `{core_root}` for `core/` is `{global}/core/`; other categories remain local-only

#### Scenario: No fallback
- **WHEN** neither local nor global `core/navigation.md` exists
- **THEN** `{core_root}` is local only, no fallback

#### Scenario: Custom directory
- **WHEN** `paths.json` sets `custom_dir` to `.context`
- **THEN** the context root is `.context/` instead of `.opencode/context/`

### Requirement: Context tree installation

The system SHALL copy the vendored `content/context/` tree into the target directory during `install`. Existing context files SHALL NOT be overwritten unless `--force` is given.

#### Scenario: Fresh install
- **WHEN** `myagentcontrol install` runs on a project without `.opencode/context/`
- **THEN** the full `content/context/` tree is copied to `.opencode/context/`

#### Scenario: Idempotent re-install
- **WHEN** `myagentcontrol install` runs and `.opencode/context/` already exists
- **THEN** existing context files are preserved; only missing files are added

### Requirement: /add-context wizard

The system SHALL provide an interactive wizard that guides the user through 6 questions to create a Project Intelligence context file at `project-intelligence/technical-domain.md`. The wizard SHALL update `navigation.md` so the new file is reachable.

#### Scenario: Wizard completes successfully
- **WHEN** the user runs the wizard and answers all 6 questions
- **THEN** a valid context file is created at `project-intelligence/technical-domain.md` and `navigation.md` is updated

#### Scenario: Wizard in non-interactive mode
- **WHEN** the wizard runs and stdin is not a TTY
- **THEN** it errors with a guidance message and exit code 1

### Requirement: @-reference syntax validation

The system SHALL validate `@`-reference syntax in agent and command files. Dynamic references (`@${var}`), non-standard `@` references, and missing allowlist entries SHALL be rejected. Allowlisted patterns: `@.opencode/context/...`, `@AGENTS.md`, `@.cursorrules`, `@$N` positional args, email/mailto.

#### Scenario: Valid reference
- **WHEN** an agent file contains `@.opencode/context/core/standards/code-quality.md`
- **THEN** validation passes

#### Scenario: Dynamic reference rejected
- **WHEN** an agent file contains `@${var}`
- **THEN** the validator emits error `CTX-209`

#### Scenario: Non-standard reference rejected
- **WHEN** an agent file contains `@some-other-place`
- **THEN** the validator emits error `CTX-210`

### Requirement: External context cache

The system SHALL manage cached external documentation under `.tmp/external-context/` with a JSON manifest supporting add, update, list, and remove operations.

#### Scenario: Add to cache
- **WHEN** a new external doc is cached
- **THEN** it is stored under `.tmp/external-context/` and recorded in the manifest with SHA256

#### Scenario: List cache
- **WHEN** the user requests a cache listing
- **THEN** the system returns all cached items with their metadata

### Requirement: --update mode

The system SHALL support `--update` mode on the wizard: increment the context file version (minor on content change, major on structural change) and refresh the `Updated` date.

#### Scenario: Content update
- **WHEN** `--update` is used and the file content changes but not its structure
- **THEN** the version minor is incremented and `Updated` is set to today's date

#### Scenario: Structural change
- **WHEN** `--update` is used and the file structure changes
- **THEN** the version major is incremented and `Updated` is set to today's date
