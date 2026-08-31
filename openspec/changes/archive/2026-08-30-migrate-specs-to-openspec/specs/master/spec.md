## Purpose

The master spec defines the project's vision, goals, architecture, technical decisions (ADR format), non-functional requirements, and roadmap. It is the steering document that all module specs derive from.

## ADDED Requirements

### Requirement: Project vision

The project SHALL reimplement OpenAgentsControl as a Rust configuration manager, keeping the same concepts and file formats but evolving as its own vision (not a parity yardstick).

#### Scenario: Vision alignment
- **WHEN** design decisions are made
- **THEN** they align with the reimplementation vision

### Requirement: Model-agnostic execution

Execution SHALL be delegated to the OpenCode CLI, which is model-agnostic. The binary SHALL NOT invoke any model API directly (D1, D5).

#### Scenario: Backend delegation
- **WHEN** agents run
- **THEN** they execute via OpenCode, not the binary

### Requirement: Content inventory

The managed tree SHALL include agents, subagents, skills, commands, context, profiles, prompts, tool, plugin, plugins, docs, config, scripts, and registry.json (§6).

#### Scenario: Tree completeness
- **WHEN** `install` runs
- **THEN** all managed paths are scaffolded

### Requirement: Non-functional requirements

The project SHALL meet NFR1–NFR6: content integrity (walk tests), determinism, performance (<2s validate), Rust quality (clippy/fmt/test), zero runtime node/bun, and backward compatibility (§9).

#### Scenario: NFR compliance
- **WHEN** the binary ships
- **THEN** all NFRs are satisfied

### Requirement: Roadmap phases

Implementation SHALL follow phases: Foundation, Context, Agents, Skills, Evals, Registry, Walk Tests, Polish (§10).

#### Scenario: Phase ordering
- **WHEN** development proceeds
- **THEN** phases are completed in order
