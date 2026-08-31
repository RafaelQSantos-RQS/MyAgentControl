## 1. CI/CD

- [x] 1.1 Create `.github/workflows/ci.yml` with test, clippy, fmt jobs
- [ ] 1.2 Verify CI runs on push (test with first commit)

## 2. List Subcommand

- [x] 2.1 Extract `list_components_plain` from `list_components` in `ui.rs`
- [x] 2.2 Add `List` variant to `Command` enum in `main.rs`
- [x] 2.3 Implement `run_list` function that calls `list_components_plain`
- [x] 2.4 Test `myagentcontrol list` locally

## 3. Commit

- [x] 3.1 Run `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- [ ] 3.2 Commit and push (waiting for approval)
