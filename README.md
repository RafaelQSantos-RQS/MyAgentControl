# MyAgentControl

A Rust configuration manager for OpenCode-compatible agent trees: install,
track, and verify agents, skills, commands, and context components from an
embedded registry.

`myagentcontrol` is a **Rust reimplementation** of
[OpenAgentsControl (OAC)](https://github.com/darrenhinde/OpenAgentsControl)
v0.7.1. OAC is a model-agnostic AI agent framework (agents, subagents,
skills, commands, a context system, and an eval harness) shipped as markdown
and config files that run on top of the OpenCode CLI. This project keeps the
same concepts, file formats, and workflow, but implements the tooling in
Rust and evolves the framework as its own vision: the managed tree is
maintained **in this repository** and is never re-fetched from upstream. See
[`NOTICE.md`](./NOTICE.md) for the attribution and divergence policy.

The binary is a **configuration manager, not a runtime**: it copies,
validates, and maintains the `.opencode/`-compatible structure. It does not
invoke the OpenCode CLI or any model API itself; execution stays with the
OpenCode CLI the user already relies on.

## Status

Pre-v1. The `install`, `add`, `remove`, `status`, `validate`, and `wizard`
commands are implemented against the real embedded registry (442 files,
OAC v0.7.1 as starting point). `list` and the eval framework are specified
but not yet implemented.

## Commands

| Command | Description |
|---|---|
| `install` | Interactive installer (TUI): pick a location, choose a profile or individual components, preview, and install. |
| `add <type>:<id>` | Install a component plus its transitive dependencies into an existing tree. |
| `remove <type>:<id>` | Remove a component's tracked files and update the manifest. |
| `status` | Compare the manifest against the install directory; reports modified, removed, and added files. |
| `validate` | Validate context files (MVI, frontmatter, @-references). |
| `list` | List available components from the registry. |
| `wizard add-context` | Interactive wizard for creating or updating a context file. |

Every command accepts `--dir` to target a custom tree root (default:
`.opencode`). Installed files are tracked by SHA256 in a manifest
(`.mac/manifest.json`) inside the target directory; existing files are
preserved unless `--force` is given.

## Build

Requires Rust (edition 2024).

```sh
cargo build --release
```

## Usage

```sh
# Interactive installer
myagentcontrol install

# Install a single component plus its dependencies
myagentcontrol add agent:openagent

# Check installed tree health against the manifest
myagentcontrol status

# Uninstall a component's tracked files
myagentcontrol remove context:quick-start
```

## Architecture

```
myagentcontrol (Rust binary)
  install │ add │ remove │ status
        │ copies from content/ · tracks in .mac/manifest.json
        ▼
.opencode/ (markdown + config, OAC-compatible)
  agent/  subagents/  skills/  command/  context/  profiles/  ...
        │ read by (not invoked by us)
        ▼
OpenCode CLI (model-agnostic)
```

The vendored [`content/`](./content/) tree is the in-repo source of truth:
`install` copies it into the user project as `.opencode/`. This keeps the
result drop-in compatible with OpenCode while staying model-agnostic.

## Repository layout

| Path | Purpose |
|---|---|
| `src/` | Rust crate: `main.rs` (CLI) + `install/` module (installer, registry, manifest, add/remove/status) |
| `content/` | Vendored OAC-compatible tree (agents, subagents, skills, commands, context, registry, config) |
| `openspec/specs/` | Spec-Driven Development docs: constitution, master spec, module specs |
| `NOTICE.md` | OAC v0.7.1 attribution and divergence policy |
| `LICENSE` | MIT |

## Development

- Specs are the source of truth: code is written only after a spec is
  approved (see `openspec/` for the lifecycle and change-request process).
- Quality gates: `cargo test`, `cargo clippy -- -D warnings`,
  `cargo fmt --check`, no unsafe code.
- The `content/` tree is maintained in-repo and validated by always-on walk
  tests; there is no external checkout dependency.

## License

MIT. The vendored `content/` tree originates from OpenAgentsControl v0.7.1
(c) 2025 Darren Hinde, MIT; see [`NOTICE.md`](./NOTICE.md) and
[`LICENSE`](./LICENSE).
