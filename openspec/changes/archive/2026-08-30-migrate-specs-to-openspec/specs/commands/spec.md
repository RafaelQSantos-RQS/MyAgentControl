## Purpose

The commands module defines the schema, validation, and scaffolding for slash command markdown files.

## ADDED Requirements

### Requirement: Command file schema

Command files SHALL have YAML frontmatter with `description` (required), optional `tags` (list of strings), and optional `dependencies` (list of dependency refs) (CMD-401).

#### Scenario: Valid command frontmatter
- **WHEN** a command file has valid frontmatter with description
- **THEN** it passes schema validation

### Requirement: Dependency validation

Command dependencies SHALL reference valid component types and names. Invalid references SHALL produce errors (CMD-402, [tool DX]).

#### Scenario: Valid dependency
- **WHEN** a command references `subagent:context-organizer`
- **THEN** it resolves successfully

#### Scenario: Invalid dependency
- **WHEN** a command references `subagent:nonexistent`
- **THEN** validation emits an error

### Requirement: Inventory walk test

A walk test SHALL validate that all commands listed in the tree are accounted for (CMD-403, [tool DX]).

#### Scenario: Command inventory
- **WHEN** the walk test runs
- **THEN** all command files are listed
