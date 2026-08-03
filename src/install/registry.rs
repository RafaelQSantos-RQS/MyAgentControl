//! Registry loading (registry-spec §4): read and parse the vendored
//! `content/registry.json` into the typed [`model::Registry`].

use std::fs;
use std::path::Path;

use crate::install::model::Registry;

/// Registry load/parse failures (E400-family, registry-spec §7 envelope).
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("E400: cannot read registry {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    /// Parse failure of the registry JSON (E100; "E200" reserved for schema
    /// validation in a later stage).
    #[error("E100: invalid registry JSON in {path}: {source}")]
    Parse {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

/// Read and parse the registry at `path`.
pub fn load(path: &Path) -> Result<Registry, LoadError> {
    let path_str = path.display().to_string();
    let text = fs::read_to_string(path).map_err(|source| LoadError::Io {
        path: path_str.clone(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| LoadError::Parse {
        path: path_str,
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Write `content` to a unique temp file and return its path. Each test
    /// uses its own directory (tests run in parallel; a shared dir races).
    fn temp_registry(name: &str, content: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "myagentcontrol-registry-{}-{}",
            std::process::id(),
            name
        ));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let path = dir.join("registry.json");
        let mut f = std::fs::File::create(&path).expect("create temp registry");
        f.write_all(content.as_bytes())
            .expect("write temp registry");
        path
    }

    #[test]
    fn loads_registry_version() {
        let path = temp_registry(
            "version",
            r#"{"version":"2.0.0","profiles":{},"components":{}}"#,
        );
        let reg = load(&path).expect("loads");
        assert_eq!(reg.version.as_deref(), Some("2.0.0"));
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn loads_empty_profiles() {
        let path = temp_registry(
            "profiles",
            r#"{"version":"2.0.0","profiles":{},"components":{}}"#,
        );
        let reg = load(&path).expect("loads");
        assert!(reg.profiles.is_empty());
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn missing_file_is_io_error() {
        let err = load(Path::new("/nonexistent/registry.json")).unwrap_err();
        assert!(err.to_string().starts_with("E400"), "got: {err}");
    }

    #[test]
    fn malformed_json_is_parse_error() {
        let path = temp_registry("malformed", "{ not json");
        let err = load(&path).unwrap_err();
        assert!(err.to_string().starts_with("E100"), "got: {err}");
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }
}
