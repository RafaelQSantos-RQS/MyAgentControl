# agents Specification

## Purpose
The agents module defines the schema, validation rules, and scaffolding for agent and subagent markdown files with YAML frontmatter.

## Requirements

### Requirement: Agent file schema

Agent files SHALL have YAML frontmatter with `name`, `description`, `mode` (primary|subagent), optional `temperature`, and optional `permission` map. Permission verbs SHALL be one of `allow`, `ask`, `deny` (AG-201, AG-202).

#### Scenario: Valid agent frontmatter
- **WHEN** an agent file has valid frontmatter with required fields
- **THEN** it passes schema validation

#### Scenario: Invalid permission verb
- **WHEN** an agent file has permission verb "allow all"
- **THEN** validation emits error AG-202

### Requirement: Category JSON validation

Each agent category SHALL have a `0-category.json` with valid JSON listing agents in that category (AG-203).

#### Scenario: Valid category
- **WHEN** a category JSON is well-formed and lists existing agents
- **THEN** it passes validation

### Requirement: Delegation graph validation

The delegation graph (which agents invoke which subagents) SHALL be acyclic and reference only existing subagents (AG-204, [tool DX]).

#### Scenario: Acyclic delegation
- **WHEN** the delegation graph is validated
- **THEN** no cycles exist

### Requirement: Inventory walk test

An inventory walk test SHALL validate that every agent listed in `0-category.json` exists on disk, and every agent file is listed in a category (AG-205, [tool DX]).

#### Scenario: Inventory consistency
- **WHEN** the walk test runs
- **THEN** all agents are accounted for
