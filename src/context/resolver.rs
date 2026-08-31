use std::path::{Path, PathBuf};

/// Trait for injecting glob behavior into the resolver.
pub trait Glob {
    /// Returns `true` if the path exists (file or directory).
    fn exists(&self, path: &Path) -> bool;
}

/// Production implementation using `std::path::Path::exists`.
pub struct FsGlob;

impl Glob for FsGlob {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

/// Where the `core/` context root resolved to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreRoot {
    /// Local install is the single source for everything.
    Local(PathBuf),
    /// Local has no core; global provides only `core/` files.
    GlobalForCore {
        local: PathBuf,
        global_core: PathBuf,
    },
    /// No fallback; local only.
    LocalOnly(PathBuf),
}

/// Resolve the context root using local-first, global-fallback logic.
///
/// Performs at most 2 glob checks:
/// 1. `{local}/core/navigation.md`
/// 2. If missing, `{global}/core/navigation.md`
pub fn resolve(
    local: &Path,
    global: Option<&Path>,
    custom_dir: Option<&Path>,
    glob: &dyn Glob,
) -> CoreRoot {
    let local_root = custom_dir.unwrap_or(local);

    // Check 1: local core
    if glob.exists(&local_root.join("core/navigation.md")) {
        return CoreRoot::Local(local_root.to_path_buf());
    }

    // Check 2: global core (only if global path provided)
    if let Some(global_path) = global
        && glob.exists(&global_path.join("core/navigation.md"))
    {
        return CoreRoot::GlobalForCore {
            local: local_root.to_path_buf(),
            global_core: global_path.join("core"),
        };
    }

    CoreRoot::LocalOnly(local_root.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    struct MockGlob {
        exists: HashSet<PathBuf>,
        calls: std::cell::Cell<usize>,
    }

    impl MockGlob {
        fn new(exists: Vec<PathBuf>) -> Self {
            Self {
                exists: exists.into_iter().collect(),
                calls: std::cell::Cell::new(0),
            }
        }
    }

    impl Glob for MockGlob {
        fn exists(&self, path: &Path) -> bool {
            self.calls.set(self.calls.get() + 1);
            self.exists.contains(path)
        }
    }

    #[test]
    fn local_install_exists() {
        let glob = MockGlob::new(vec![".opencode/core/navigation.md".into()]);
        let result = resolve(Path::new(".opencode"), None, None, &glob);
        assert_eq!(result, CoreRoot::Local(".opencode".into()));
        assert_eq!(glob.calls.get(), 1);
    }

    #[test]
    fn global_fallback_for_core() {
        let glob = MockGlob::new(vec![
            "/home/user/.config/opencode/core/navigation.md".into(),
        ]);
        let result = resolve(
            Path::new(".opencode"),
            Some(Path::new("/home/user/.config/opencode")),
            None,
            &glob,
        );
        assert_eq!(
            result,
            CoreRoot::GlobalForCore {
                local: ".opencode".into(),
                global_core: "/home/user/.config/opencode/core".into(),
            }
        );
        assert_eq!(glob.calls.get(), 2);
    }

    #[test]
    fn no_fallback() {
        let glob = MockGlob::new(vec![]);
        let result = resolve(
            Path::new(".opencode"),
            Some(Path::new("/home/user/.config/opencode")),
            None,
            &glob,
        );
        assert_eq!(result, CoreRoot::LocalOnly(".opencode".into()));
        assert_eq!(glob.calls.get(), 2);
    }

    #[test]
    fn custom_dir() {
        let glob = MockGlob::new(vec![".context/core/navigation.md".into()]);
        let result = resolve(
            Path::new(".opencode"),
            None,
            Some(Path::new(".context")),
            &glob,
        );
        assert_eq!(result, CoreRoot::Local(".context".into()));
        assert_eq!(glob.calls.get(), 1);
    }

    #[test]
    fn global_without_core() {
        let glob = MockGlob::new(vec![]);
        let result = resolve(
            Path::new(".opencode"),
            Some(Path::new("/home/user/.config/opencode")),
            None,
            &glob,
        );
        assert_eq!(result, CoreRoot::LocalOnly(".opencode".into()));
        assert_eq!(glob.calls.get(), 2);
    }
}
