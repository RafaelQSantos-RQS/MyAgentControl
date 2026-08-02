//! Typed error types for `myagentcontrol` (cli-spec §7).
//!
//!
//! Two-tier error scheme:
//! - **Envelope** (`ErrorCode`): `E100` parse, `E200` schema, `E300` dangling
//!   reference, `E400` io, `E500` internal — the reported prefix.
//! - **Rule ID** (`rule`): module-specific violation ID (e.g. `AG-202`,
//!   `CTX-201`, `SK-204`), naming the specific violation.
//!
//! Output shape (cli-spec §10.3):
//! ```text
//! E200 [agents] .opencode/agent/core/opencoder.md:12
//!   rule: AG-202 — permission verb "allow all" not in {allow, ask, deny}
//!   hint: use one of: allow, ask, deny
//! ```

use std::fmt;

use thiserror::Error;

/// Category envelope for the two-tier error scheme (cli-spec §7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// E100 — parse errors (frontmatter, MVI lines, JSON).
    Parse,
    /// E200 — schema defects (module rule IDs `XX-2xx`).
    Schema,
    /// E300 — dangling references.
    Reference,
    /// E400 — I/O errors.
    Io,
    /// E500 — internal errors.
    Internal,
}

impl ErrorCode {
    /// The `E###` prefix reported to the user.
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::Parse => "E100",
            ErrorCode::Schema => "E200",
            ErrorCode::Reference => "E300",
            ErrorCode::Io => "E400",
            ErrorCode::Internal => "E500",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Module label shown in the `[agents]` position of an error line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Module {
    Context,
    Agents,
    Skills,
    Commands,
    Evals,
    Registry,
    Cli,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Module::Context => "context",
            Module::Agents => "agents",
            Module::Skills => "skills",
            Module::Commands => "commands",
            Module::Evals => "evals",
            Module::Registry => "registry",
            Module::Cli => "cli",
        };
        f.write_str(label)
    }
}

/// A validation error with the §10.3 output shape.
///
/// Every validation error includes: file path, line/col where available,
/// a module rule ID (e.g. `CTX-201`), and an actionable hint.
///
/// `Display` is implemented manually (not via thiserror's `#[error(…)]`
/// format string) because the multi-line shape with conditional line/hint
/// is clearer hand-written; `Error` is blanket-impl'd so thiserror's
/// `#[from]` in [`AppError`] accepts it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    code: ErrorCode,
    module: Module,
    path: String,
    line: Option<u32>,
    rule: String,
    message: String,
    hint: Option<String>,
}

impl ValidationError {
    /// Create a new validation error.
    ///
    /// - `code`: category envelope (`E###`).
    /// - `module`: module label for the `[…]` position.
    /// - `path`: offending file path.
    /// - `line`: 1-based line where known.
    /// - `rule`: module rule ID (e.g. `"AG-202"`).
    /// - `message`: human-readable violation description.
    /// - `hint`: optional actionable suggestion.
    pub fn new(
        code: ErrorCode,
        module: Module,
        path: impl Into<String>,
        line: Option<u32>,
        rule: impl Into<String>,
        message: impl Into<String>,
        hint: Option<String>,
    ) -> Self {
        Self {
            code,
            module,
            path: path.into(),
            line,
            rule: rule.into(),
            message: message.into(),
            hint,
        }
    }

    /// Convenience constructor for schema defects (E200).
    pub fn schema(
        module: Module,
        path: impl Into<String>,
        line: Option<u32>,
        rule: impl Into<String>,
        message: impl Into<String>,
        hint: Option<String>,
    ) -> Self {
        Self::new(ErrorCode::Schema, module, path, line, rule, message, hint)
    }

    /// Convenience constructor for dangling references (E300).
    pub fn reference(
        module: Module,
        path: impl Into<String>,
        line: Option<u32>,
        rule: impl Into<String>,
        message: impl Into<String>,
        hint: Option<String>,
    ) -> Self {
        Self::new(
            ErrorCode::Reference,
            module,
            path,
            line,
            rule,
            message,
            hint,
        )
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}] {}", self.code, self.module, self.path)?;
        if let Some(line) = self.line {
            write!(f, ":{line}")?;
        }
        write!(f, "\n  rule: {} — {}", self.rule, self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, "\n  hint: {hint}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

/// Crate-level error: the `main`/dispatch error type.
#[derive(Debug, Error)]
pub enum AppError {
    /// A structured validation error (schema/reference/etc.).
    #[error(transparent)]
    Validation(#[from] ValidationError),
    /// I/O failure (E400 envelope).
    #[error("E400: {0}")]
    Io(#[from] std::io::Error),
    /// Internal invariant failure (E500 envelope).
    #[error("E500 internal error: {0}")]
    Internal(String),
}

/// Convenient alias used across the crate.
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_code_prefixes() {
        assert_eq!(ErrorCode::Parse.as_str(), "E100");
        assert_eq!(ErrorCode::Schema.as_str(), "E200");
        assert_eq!(ErrorCode::Reference.as_str(), "E300");
        assert_eq!(ErrorCode::Io.as_str(), "E400");
        assert_eq!(ErrorCode::Internal.as_str(), "E500");
        assert_eq!(ErrorCode::Schema.to_string(), "E200");
    }

    #[test]
    fn module_labels() {
        assert_eq!(Module::Agents.to_string(), "agents");
        assert_eq!(Module::Context.to_string(), "context");
        assert_eq!(Module::Skills.to_string(), "skills");
        assert_eq!(Module::Commands.to_string(), "commands");
        assert_eq!(Module::Evals.to_string(), "evals");
        assert_eq!(Module::Registry.to_string(), "registry");
        assert_eq!(Module::Cli.to_string(), "cli");
    }

    #[test]
    fn render_matches_cli_spec_10_3_shape() {
        let err = ValidationError::schema(
            Module::Agents,
            ".opencode/agent/core/opencoder.md",
            Some(12),
            "AG-202",
            "permission verb \"allow all\" not in {allow, ask, deny}",
            Some("use one of: allow, ask, deny".to_string()),
        );
        let rendered = err.to_string();
        let expected = "E200 [agents] .opencode/agent/core/opencoder.md:12\n  rule: AG-202 — permission verb \"allow all\" not in {allow, ask, deny}\n  hint: use one of: allow, ask, deny";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_omits_line_and_hint_when_absent() {
        let err = ValidationError::reference(
            Module::Skills,
            "skills/foo.md",
            None,
            "SK-204",
            "references missing agent",
            None,
        );
        let rendered = err.to_string();
        let expected = "E300 [skills] skills/foo.md\n  rule: SK-204 — references missing agent";
        assert_eq!(rendered, expected);
        assert!(!rendered.contains("hint:"));
    }

    #[test]
    fn envelopes_map_to_rule_ids() {
        // Schema defects map into E200; dangling references into E300.
        let schema =
            ValidationError::schema(Module::Context, "ctx.md", None, "CTX-201", "bad", None);
        let dangling = ValidationError::reference(
            Module::Commands,
            "cmd.md",
            None,
            "CMD-201",
            "dangling",
            None,
        );
        assert!(schema.to_string().starts_with("E200 [context]"));
        assert!(dangling.to_string().starts_with("E300 [commands]"));
    }

    #[test]
    fn app_error_wraps_validation() {
        let v = ValidationError::schema(Module::Cli, "x", None, "CLI-1", "boom", None);
        let e = AppError::from(v);
        assert!(e.to_string().starts_with("E200 [cli]"));
    }

    #[test]
    fn io_error_maps_to_e400() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "no such file");
        let e = AppError::Io(io);
        assert!(e.to_string().starts_with("E400"));
    }
}
