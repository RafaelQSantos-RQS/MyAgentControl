## Why

Cargo.toml is missing standard metadata fields (`description`, `license`, `repository`, `authors`, `readme`, `keywords`, `categories`). These fields are required for crates.io publishing and improve discoverability. Even if we don't publish soon, having them correct is low-effort hygiene.

## What Changes

- Add `description`, `license`, `repository`, `authors`, `readme`, `keywords`, `categories` to `[package]`
- No code changes, no behavior changes

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

(none)

## Impact

- `Cargo.toml` only — metadata fields added
