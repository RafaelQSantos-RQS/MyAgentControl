## Purpose

The skills module defines the SKILL.md validation rules, router.sh validation, and skill structure for the `content/skills/` tree.

## ADDED Requirements

### Requirement: SKILL.md schema

`SKILL.md` files SHALL have YAML frontmatter with `name` (required), optional `description`, and optional `triggers` (list of strings) (SK-701).

#### Scenario: Valid SKILL.md
- **WHEN** a SKILL.md has valid frontmatter with name
- **THEN** it passes schema validation

### Requirement: Router validation

`router.sh` SHALL be a valid bash script with a shebang line. It SHALL be executable (SK-702).

#### Scenario: Valid router
- **WHEN** a router.sh is validated
- **THEN** it has a shebang and is executable

### Requirement: Referenced file validation

Files referenced in SKILL.md (scripts, workflows) SHALL exist on disk (SK-703, [tool DX]).

#### Scenario: Referenced file exists
- **WHEN** SKILL.md references `scripts/task-cli.ts`
- **THEN** the file exists in the skill directory

### Requirement: Cross-file consistency

Skills with `triggers` SHALL have corresponding entries in the agent's trigger list (SK-704, [tool DX]).

#### Scenario: Trigger consistency
- **WHEN** a skill declares triggers
- **THEN** agents referencing the skill have matching trigger entries

### Requirement: Inventory walk test

A walk test SHALL validate that all skills in `content/skills/` are accounted for (SK-705, [tool DX]).

#### Scenario: Skill inventory
- **WHEN** the walk test runs
- **THEN** all skill directories are listed

### Requirement: Skill structure

Each skill directory SHALL contain `SKILL.md`, `router.sh`, and optional `scripts/` and `workflows/` subdirectories (SK-706).

#### Scenario: Directory layout
- **WHEN** a skill directory is inspected
- **THEN** it has the expected structure
