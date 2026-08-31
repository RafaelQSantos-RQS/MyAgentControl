## Purpose

The evals module defines the evaluation framework: YAML test cases, results JSON, and HTML dashboard generation.

## ADDED Requirements

### Requirement: Eval case schema

Eval cases SHALL be YAML files with `agent`, `input`, `expected`, and optional `timeout` fields (EV-501).

#### Scenario: Valid eval case
- **WHEN** an eval case has required fields
- **THEN** it passes schema validation

### Requirement: Results JSON

Results SHALL be stored as JSON with `case_id`, `status`, `actual`, `timestamp`, and optional `error` fields (EV-502).

#### Scenario: Results format
- **WHEN** an eval completes
- **THEN** a results JSON entry is created

### Requirement: Dashboard generation

An HTML dashboard SHALL be generated from results JSON, showing pass/fail rates and timing (EV-503, [tool DX]).

#### Scenario: Dashboard render
- **WHEN** `dashboard` is invoked with results JSON
- **THEN** an HTML file is generated

### Requirement: Validate command

`validate` SHALL check eval cases against the schema and report errors (EV-504, [tool DX]).

#### Scenario: Validate cases
- **WHEN** `validate` is run on eval cases
- **THEN** invalid cases are reported

### Requirement: Import command

`import` SHALL copy eval cases from an OAC checkout to the local `evals/` directory (EV-505, [tool DX]).

#### Scenario: Import cases
- **WHEN** `import` is invoked with an OAC path
- **THEN** eval cases are copied locally

### Requirement: Run deferred

`evals run` SHALL be deferred to post-v1. The config-manager binary SHALL NOT execute agents (D1).

#### Scenario: Run not implemented
- **WHEN** `evals run` is invoked pre-v1
- **THEN** it returns a "not implemented" message
