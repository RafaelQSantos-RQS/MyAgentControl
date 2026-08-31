## Purpose

The registry module defines the component registry (`registry.json`), install state (`manifest.json`), and the install/update/status commands.

## ADDED Requirements

### Requirement: Registry schema

`registry.json` SHALL be a JSON catalog listing all agents, subagents, commands, skills, context, profiles, prompts, tools, plugins, docs, configs, and scripts. Each entry SHALL have `type`, `path`, and optional `dependencies` and `profiles` fields (REG-601).

#### Scenario: Valid registry
- **WHEN** `registry.json` is validated
- **THEN** all entries have required fields

### Requirement: Manifest schema

`manifest.json` SHALL track installed components with `name`, `type`, `version`, `sha256`, and `modified` fields (REG-602).

#### Scenario: Valid manifest
- **WHEN** `manifest.json` is validated
- **THEN** all entries have required fields

### Requirement: Profile resolution

Named profiles in `registry.json` SHALL select subsets of components. Profiles SHALL reference valid component IDs (REG-603).

#### Scenario: Profile selection
- **WHEN** a profile is used with `install`
- **THEN** only the profile's components are installed

### Requirement: Install command

`install` SHALL read `registry.json`, resolve the requested profile, scaffold files, and write `manifest.json` (REG-604).

#### Scenario: Fresh install
- **WHEN** `install` runs on a clean project
- **THEN** all files are scaffolded and manifest created

### Requirement: Status command

`status` SHALL read `manifest.json` and display installed components with modification status (REG-605).

#### Scenario: Status display
- **WHEN** `status` is run
- **THEN** installed components are listed

### Requirement: Update command

`update` SHALL compare installed versions against `registry.json` and offer updates (REG-606).

#### Scenario: Update available
- **WHEN** `update` is run and newer versions exist
- **THEN** the user is prompted to update
