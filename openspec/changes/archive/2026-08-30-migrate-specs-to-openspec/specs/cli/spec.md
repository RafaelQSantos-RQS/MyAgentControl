## Purpose

The CLI module defines the commands, validation logic, walk tests, and wizard flows for the `myagentcontrol` binary.

## ADDED Requirements

### Requirement: CLI commands

The binary SHALL expose subcommands: `install`, `add`, `remove`, `status`, `validate`, `wizard`, and `help` (CLI-301).

#### Scenario: Command availability
- **WHEN** the binary is invoked with a valid subcommand
- **THEN** the corresponding action executes

### Requirement: Install command

`install` SHALL scaffold the `.opencode/` tree from `content/`, be idempotent, and preserve user edits (CLI-302).

#### Scenario: Idempotent install
- **WHEN** `install` runs twice on the same project
- **THEN** user-modified files are preserved

### Requirement: Add command

`add <component-type> <name>` SHALL install a single component and update the manifest (CLI-303).

#### Scenario: Add agent
- **WHEN** `add agent agent-name` is run
- **THEN** the agent is installed and manifest updated

### Requirement: Remove command

`remove <component-type> <name>` SHALL uninstall a component and update the manifest (CLI-304).

#### Scenario: Remove agent
- **WHEN** `remove agent agent-name` is run
- **THEN** the agent is removed and manifest updated

### Requirement: Status command

`status` SHALL display installed components with versions and modification status (CLI-305).

#### Scenario: Status display
- **WHEN** `status` is run
- **THEN** installed components are listed

### Requirement: Validate command

`validate` SHALL check all managed files against their schema rules and report errors (CLI-306).

#### Scenario: Validate all
- **WHEN** `validate` is run
- **THEN** all managed files are checked

### Requirement: Wizard command

`wizard` SHALL guide users through context discovery and configuration via interactive prompts (CLI-307, [tool DX]).

#### Scenario: Wizard flow
- **WHEN** `wizard` is run
- **THEN** the user is guided through configuration steps

### Requirement: Walk tests

Walk tests SHALL validate the `content/` tree against a known allowlist and report deviations (CLI-308).

#### Scenario: Walk test pass
- **WHEN** walk tests run on a clean tree
- **THEN** all files match the allowlist
