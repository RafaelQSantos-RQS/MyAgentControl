const REFERENCE_HEADINGS: &[&str] = &[
    "References",
    "Reference",
    "Quick Reference",
    "Related Context",
    "Related Files",
    "Related",
    "Codebase References",
];

const DISCOVERY_FILES: &[&str] = &[
    "navigation.md",
    "index.md",
    "README.md",
    "CODEBASE_STANDARDS.md",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MviError {
    /// File exceeds 200 lines and is not a reference doc
    TooLong(usize),
    /// Concept card missing a reference section
    MissingReference,
}

impl std::fmt::Display for MviError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MviError::TooLong(n) => {
                write!(f, "CTX-205: file has {n} lines (max 200 for concept cards)")
            }
            MviError::MissingReference => {
                write!(f, "CTX-208: concept card missing a reference section")
            }
        }
    }
}

impl std::error::Error for MviError {}

/// Validate MVI constraints on a context file.
///
/// - Files ≥ 200 lines are reference docs (exempt from reference-section rule).
/// - Discovery files are exempt from the reference-section rule.
/// - Concept cards (< 200 lines, non-discovery) must include a reference heading.
pub fn validate_mvi(content: &str, file_name: &str) -> Result<(), MviError> {
    let lines: Vec<&str> = content.lines().collect();
    let line_count = lines.len();

    let is_discovery = DISCOVERY_FILES.contains(&file_name);
    let is_reference_doc = line_count >= 200;

    if !is_reference_doc && !is_discovery {
        let has_reference = lines.iter().any(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("## ")
                && REFERENCE_HEADINGS
                    .iter()
                    .any(|h| trimmed.eq_ignore_ascii_case(&format!("## {h}")))
        });

        if !has_reference {
            return Err(MviError::MissingReference);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn concept_card() -> String {
        "# Code Quality\n\n## Overview\nSome content.\n\n## References\n- link\n".to_string()
    }

    #[test]
    fn valid_concept_card() {
        assert!(validate_mvi(&concept_card(), "code-quality.md").is_ok());
    }

    #[test]
    fn missing_reference_heading() {
        let content = "# Code Quality\n\n## Overview\nSome content.\n";
        assert_eq!(
            validate_mvi(content, "code-quality.md"),
            Err(MviError::MissingReference)
        );
    }

    #[test]
    fn reference_heading_case_insensitive() {
        let content = "# Code Quality\n\n## references\n- link\n";
        assert!(validate_mvi(content, "code-quality.md").is_ok());
    }

    #[test]
    fn discovery_file_excluded() {
        let content = "# Navigation\n\nSome nav content.\n".repeat(5);
        assert!(validate_mvi(&content, "navigation.md").is_ok());
    }

    #[test]
    fn large_file_excluded() {
        let content = "# Large\n\n".repeat(200);
        assert!(validate_mvi(&content, "large.md").is_ok());
    }

    #[test]
    fn concept_card_with_any_reference_heading() {
        let content = "# Doc\n\n## Quick Reference\n- stuff\n";
        assert!(validate_mvi(content, "doc.md").is_ok());
    }
}
