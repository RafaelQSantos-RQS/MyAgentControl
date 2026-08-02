# tests/golden — Golden fixtures (task 05, D8)

Fixture layout for golden tests (cli-spec §8 / master decision D8): diff a
generated/scaffolded tree against a reference, after normalizing volatile
content (`Updated:` dates, CRLF, trailing whitespace).

## Layout

```
tests/golden/
├── README.md
└── reference/                 # committed reference subtree (source of truth)
    ├── context-guide.md
    ├── context-paths.md
    └── navigation.md
```

## Provenance

The `reference/` files are a **small slice** of the OAC reference repo
([`darrenhinde/OpenAgentsControl`](https://github.com/darrenhinde/OpenAgentsControl))
at tag **v0.7.1**, copied from:

```
.opencode/context/core/system/
```

They are committed verbatim (MIT-licensed, © 2025 Darren Hinde) so golden
tests run without requiring the full checkout. To refresh from the pinned
tag, run:

```bash
git clone --branch v0.7.1 https://github.com/darrenhinde/OpenAgentsControl .tmp/reference/OpenAgentsControl
cp .tmp/reference/OpenAgentsControl/.opencode/context/core/system/*.md tests/golden/reference/
```

## Normalization (D8)

`myagentcontrol::core::golden::normalize` strips:

- date tokens `YYYY-MM-DD` → `<date>` (e.g. `| Updated: 2026-02-15 |`)
- CRLF → LF
- trailing whitespace per line; single trailing newline

The smoke test (`tests/golden_smoke.rs`) copies this subtree, verifies a
clean diff, and proves date drift still diffs clean while real content
changes are flagged.
