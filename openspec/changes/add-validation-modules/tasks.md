## 1. Setup

- [x] 1.1 Add `serde_yaml` dependency to Cargo.toml
- [x] 1.2 Create `src/validation/mod.rs` with shared YAML `---` frontmatter parser
- [x] 1.3 Add `pub mod validation;` to `src/lib.rs`

## 2. Agents Module

- [x] 2.1 Create `src/validation/agents.rs` — agent schema validator (name, description, mode, permissions)
- [x] 2.2 Implement category JSON validator (0-category.json parsing)
- [x] 2.3 Implement delegation graph cycle detection (DFS)
- [x] 2.4 Add unit tests for agent validation
- [x] 2.5 Create `tests/agent_walk.rs` — inventory walk test

## 3. Skills Module

- [x] 3.1 Create `src/validation/skills.rs` — SKILL.md schema validator
- [x] 3.2 Implement router.sh validator (shebang + executable check)
- [x] 3.3 Implement referenced file validation
- [x] 3.4 Add unit tests for skill validation
- [x] 3.5 Create `tests/skill_walk.rs` — skill structure walk test

## 4. Commands Module

- [x] 4.1 Create `src/validation/commands.rs` — command frontmatter validator
- [x] 4.2 Implement dependency reference validation
- [x] 4.3 Add unit tests for command validation
- [x] 4.4 Create `tests/command_walk.rs` — command inventory walk test

## 5. Evals Module

- [x] 5.1 Create `src/evals/mod.rs` — eval case schema types
- [x] 5.2 Implement YAML case parser + validator
- [x] 5.3 Implement results JSON types
- [x] 5.4 Implement HTML dashboard generator
- [x] 5.5 Add unit tests for evals

## 6. CLI Integration

- [x] 6.1 Extend `validate` subcommand with `--agents`, `--skills`, `--commands`, `--evals` flags
- [x] 6.2 Route each flag to the corresponding validator
- [x] 6.3 Test `myagentcontrol validate` with each flag

## 7. Verification

- [x] 7.1 Run `cargo test` — all tests pass
- [x] 7.2 Run `cargo clippy -- -D warnings`
- [x] 7.3 Run `cargo fmt --check`
- [x] 7.4 Commit and push
