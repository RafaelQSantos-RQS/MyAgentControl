# context Specification

## Purpose
The context module defines the MVI (Minimum Viable Integrity) rules for context files, local-first resolution, the @-reference format, and the wizard for context discovery.

## Requirements

### Requirement: Frontmatter validation

Context files SHALL have valid YAML frontmatter. The parser SHALL support OpenCode field names: `name`, `description`, `license`, `metadata`, `dependencies`, `allowed-tools`, `disable-model-invocation` (CTX-001).

#### Scenario: Valid frontmatter
- **WHEN** a context file has valid YAML frontmatter
- **THEN** it passes MVI validation

### Requirement: MVI rules

The validator SHALL check file existence, frontmatter validity, dependency resolution, and cross-file consistency (CTX-002).

#### Scenario: File existence
- **WHEN** a context file references another file
- **THEN** the referenced file exists on disk

### Requirement: Local-first resolution

The resolver SHALL check `./opencode/` first, then `~/.config/opencode/`, never network. It SHALL be byte-identical (CTX-003, NFR1).

#### Scenario: Resolution order
- **WHEN** a context file is resolved
- **THEN** the local path is checked before the global path

### Requirement: @-reference format

@-references SHALL use the format `@path/to/file` and the resolver SHALL find them in the two check paths (CTX-004).

#### Scenario: Valid @-reference
- **WHEN** a context file contains `@skills/task-management/SKILL.md`
- **THEN** it resolves to the correct file path

### Requirement: Cache for performance

A thread-safe LRU cache SHALL memoize resolution results for ≤10ms per call at 1000+ files (CTX-005, NFR3).

#### Scenario: Cache performance
- **WHEN** the resolver is called 1000 times
- **THEN** each call completes in ≤10ms

### Requirement: Context wizard

A wizard SHALL guide users through context discovery with interactive prompts and produce a candidate list (CTX-006, [tool DX]).

#### Scenario: Wizard flow
- **WHEN** the wizard is run
- **THEN** it produces a candidate list of context files

### Requirement: Walk tests

Walk tests SHALL validate the context tree against a known allowlist (CTX-007).

#### Scenario: Walk test pass
- **WHEN** walk tests run on a clean tree
- **THEN** all context files match the allowlist
