//! Embedded `content/` tree (C6 distribution). The vendored registry and
//! every managed file are compiled into the binary at build time, so the
//! installer is self-contained: it never needs the `content/` directory on
//! the target machine.

use std::path::Path;

use include_dir::{Dir, include_dir};

/// The full vendored `content/` tree, embedded at compile time.
pub static CONTENT: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/content");

/// The vendored `registry.json` as a string.
pub fn registry_json() -> &'static str {
    // Compile-time: the registry is embedded with the rest of the tree.
    CONTENT
        .get_file("registry.json")
        .and_then(|f| f.contents_utf8())
        .expect("registry.json is embedded and valid UTF-8")
}

/// Read a managed file by its path relative to the content root
/// (e.g. `agent/core/openagent.md`), or `None` if not embedded.
pub fn read(rel: &Path) -> Option<&'static [u8]> {
    CONTENT.get_file(rel).map(|f| f.contents())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_registry_parses_and_has_profiles() {
        let json = registry_json();
        let reg: serde_json::Value = serde_json::from_str(json).expect("valid JSON");
        assert!(reg["profiles"].is_object());
        assert!(!reg["profiles"].as_object().unwrap().is_empty());
    }

    #[test]
    fn read_returns_bytes_for_managed_file() {
        let bytes = read(Path::new("registry.json")).expect("registry embedded");
        assert!(!bytes.is_empty());
    }

    #[test]
    fn read_none_for_missing_file() {
        assert!(read(Path::new("does/not/exist.md")).is_none());
    }
}
