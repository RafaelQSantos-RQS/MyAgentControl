# constitution Specification

## Purpose
The constitution defines the immutable, non-negotiable baseline rules for every spec and every line of code in the project. It is deliberately short and requires explicit user approval to amend.

## Requirements

### Requirement: Project identity

The project SHALL be identified as `myagentcontrol`, a Rust reimplementation of OpenAgentsControl v0.7.1. OAC v0.7.1 is the historical starting point, not a moving upstream dependency (C1, C6).

#### Scenario: Project identity declaration
- **WHEN** any spec or code references the project identity
- **THEN** it uses `myagentcontrol` and acknowledges OAC heritage

### Requirement: License

The project SHALL be licensed under MIT with attribution to OpenAgentsControl in `NOTICE.md` (C2).

#### Scenario: License compliance
- **WHEN** the project is distributed
- **THEN** MIT license and OAC attribution are included

### Requirement: Configuration manager scope

The Rust binary SHALL be a configuration manager that generates, validates, and maintains the `.opencode/`-compatible structure. It SHALL NOT execute agents, call model APIs, or invoke the OpenCode CLI at runtime (C3).

#### Scenario: Binary scope
- **WHEN** the binary is invoked
- **THEN** it only manages configuration, never executes agents

### Requirement: Managed format

The managed format SHALL be markdown + YAML frontmatter, byte-compatible with what OpenCode loads (C4).

#### Scenario: Format compatibility
- **WHEN** files are generated or validated
- **THEN** they are valid markdown with YAML frontmatter

### Requirement: Model-agnosticism

No code, spec, or dependency SHALL assume a single AI vendor (C5).

#### Scenario: Vendor neutrality
- **WHEN** code or specs are written
- **THEN** they delegate execution to the user's chosen CLI

### Requirement: Source of truth

The `content/` directory SHALL be the source of truth for the managed tree (C6).

#### Scenario: Content integrity
- **WHEN** the managed tree is validated
- **THEN** it is checked against `content/`, not an external source

### Requirement: Format fidelity

The tool SHALL validate only rules that the managed formats declare. It SHALL NOT invent integrity rules beyond those formats (C16).

#### Scenario: Rule validation
- **WHEN** validation runs
- **THEN** it only checks OAC/OpenCode-declared rules

### Requirement: Spec-driven development

No implementation SHALL happen without an approved spec (C7).

#### Scenario: SDD compliance
- **WHEN** new code is written
- **THEN** an approved spec exists first

### Requirement: English specs

All specs SHALL be written in English (C8).

#### Scenario: Language
- **WHEN** specs are authored
- **THEN** they use English

### Requirement: Modular specs

Specs SHALL be modular: one master spec + one spec per module (C9).

#### Scenario: Modularity
- **WHEN** specs are organized
- **THEN** each module has its own spec file

### Requirement: Given/When/Then acceptance criteria

Acceptance criteria SHALL be in Given/When/Then form and objectively verifiable (C10).

#### Scenario: Testable criteria
- **WHEN** acceptance criteria are written
- **THEN** they use Given/When/Then format

### Requirement: Released spec editing

Released specs SHALL only be edited via Change Request. Pre-release specs may be edited directly with a version bump (C11).

#### Scenario: Change request
- **WHEN** a released spec needs modification
- **THEN** a CR is filed and approved first

### Requirement: Quality gates

Rust code SHALL pass `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and contain zero `unsafe` (C12).

#### Scenario: CI gates
- **WHEN** code is committed
- **THEN** all quality gates pass

### Requirement: No runtime node/bun

The binary SHALL NOT require node/bun at runtime (C13).

#### Scenario: Standalone binary
- **WHEN** the binary runs
- **THEN** it has no node/bun dependency

### Requirement: Idempotent install

`install` SHALL be idempotent and non-destructive (C14, NFR2, NFR6).

#### Scenario: Idempotent operation
- **WHEN** `install` runs twice
- **THEN** user edits are preserved

### Requirement: Spec-code sync

Specs and code SHALL never drift: behavior changes require spec changes first (C15).

#### Scenario: Spec-first
- **WHEN** behavior changes
- **THEN** the spec is updated before code
