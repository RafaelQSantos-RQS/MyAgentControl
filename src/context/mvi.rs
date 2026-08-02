//! MVI validation for context files (context-spec §3, CTX-2).
//!
//! MVI (Minimal Viable Information) keeps context concept cards scannable
//! (< 200 lines). Per user decision:
//!
//! - Files ≥ 200 lines are **reference docs**: exempt from the MVI formula.
//! - Files < 200 lines are **concept cards**: must include a reference section
//!   (any of [`REFERENCE_SECTIONS`]).
//! - Discovery files (`navigation.md`, `index.md`, `README.md`,
//!   `CODEBASE_STANDARDS.md`) are exempt from the reference-section rule.
//!
//! Errors use the two-tier scheme (cli-spec §7): `E200` schema envelope with
//! rule ID `CTX-208` and the §10.3 output shape.

use crate::core::errors::{Module, ValidationError};

/// Line threshold separating reference docs from concept cards (user decision).
pub const REFERENCE_DOC_LINE_THRESHOLD: usize = 200;

/// Accepted reference-section headings (context-spec §3.6, user decision: "any
/// reference section" replaces the literal "📂 Codebase References").
pub const REFERENCE_SECTIONS: &[&str] = &[
    "Codebase References",
    "Related Context",
    "Related Files",
    "Related",
    "References",
    "Reference",
    "Quick Reference",
];

/// Discovery/index files exempt from the reference-section rule (user decision).
pub const DISCOVERY_BASENAMES: &[&str] = &[
    "navigation.md",
    "index.md",
    "README.md",
    "CODEBASE_STANDARDS.md",
];

/// MVI classification of a context file (§3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MviClass {
    /// < 200 lines; must satisfy the MVI formula.
    ConceptCard,
    /// ≥ 200 lines; reference doc, exempt from the MVI formula.
    ReferenceDoc,
}

/// Classify a file by its line count.
pub fn classify(line_count: usize) -> MviClass {
    if line_count >= REFERENCE_DOC_LINE_THRESHOLD {
        MviClass::ReferenceDoc
    } else {
        MviClass::ConceptCard
    }
}

/// True when the basename of `path` is a discovery file (exempt).
fn is_discovery_file(path: &str) -> bool {
    std::path::Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|base| DISCOVERY_BASENAMES.contains(&base))
}

/// Extract the heading text of a markdown heading line (`#`–`######`).
fn heading_text(line: &str) -> Option<&str> {
    let line = line.trim_start();
    let hashes = line.bytes().take_while(|b| *b == b'#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }
    let text = line[hashes..].trim();
    if text.is_empty() { None } else { Some(text) }
}

/// True when `content` contains any accepted reference-section heading.
///
/// Matches the exact heading name, or a heading starting with it (e.g.
/// `## Related Work` counts as `Related`). Mirrors the measurement used to
/// build the walk-test allowlist.
pub fn has_reference_section(content: &str) -> bool {
    content.lines().any(|line| {
        let Some(text) = heading_text(line) else {
            return false;
        };
        // With prefix matching, `Related` subsumes `Related Context`/`Related
        // Files` and `Reference` subsumes `References`-style prefixes; the
        // extra entries document the spec's explicit section list (user decision).
        REFERENCE_SECTIONS.iter().any(|name| {
            text == *name
                || text
                    .strip_prefix(name)
                    .is_some_and(|rest| rest.starts_with(' ') || rest.starts_with(':'))
        })
    })
}

/// Validate MVI constraints for a single context file.
///
/// Returns an empty vector for reference docs and discovery files (both
/// exempt, user decision), and `CTX-208` for concept cards missing a
/// reference section.
pub fn validate(path: &str, content: &str) -> Vec<ValidationError> {
    let line_count = content.lines().count();
    match classify(line_count) {
        MviClass::ReferenceDoc => Vec::new(),
        MviClass::ConceptCard => {
            if is_discovery_file(path) || has_reference_section(content) {
                Vec::new()
            } else {
                vec![ctx_208(path)]
            }
        }
    }
}

fn ctx_208(path: &str) -> ValidationError {
    ValidationError::schema(
        Module::Context,
        path,
        None,
        "CTX-208",
        "concept card missing a reference section",
        Some("add a heading like `## Related` or `## References`".to_string()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card_with(heading: &str) -> String {
        format!("# Concept\n\nIntro sentence.\n\n{heading}\n\n- related file a\n- related file b\n")
    }

    #[test]
    fn classify_threshold() {
        assert_eq!(classify(199), MviClass::ConceptCard);
        assert_eq!(classify(200), MviClass::ReferenceDoc);
        assert_eq!(classify(4616), MviClass::ReferenceDoc);
    }

    #[test]
    fn reference_doc_exempt_even_without_sections() {
        let content = "# Big\n\n".repeat(201); // 201 lines
        assert_eq!(classify(content.lines().count()), MviClass::ReferenceDoc);
        assert!(validate("core/standards/big.md", &content).is_empty());
    }

    #[test]
    fn each_reference_section_accepted() {
        for name in REFERENCE_SECTIONS {
            let content = card_with(&format!("## {name}"));
            assert!(
                has_reference_section(&content),
                "heading `## {name}` should count as a reference section"
            );
            assert!(validate("ctx.md", &content).is_empty());
        }
    }

    #[test]
    fn heading_with_extra_words_counts_as_prefix() {
        assert!(has_reference_section(&card_with("## Related Work")));
        assert!(has_reference_section(&card_with("## Reference: docs")));
    }

    #[test]
    fn concept_card_without_reference_section_flagged_ctx_208() {
        let content = "# Concept\n\nJust prose, no reference section.\n";
        let errs = validate("core/workflows/x.md", content);
        assert_eq!(errs.len(), 1);
        assert!(errs[0].to_string().contains("CTX-208"));
        assert!(errs[0].to_string().starts_with("E200 [context]"));
    }

    #[test]
    fn discovery_files_exempt() {
        for name in DISCOVERY_BASENAMES {
            let content = "# Nav\n\nNo reference section.\n";
            assert!(
                validate(&format!("core/{name}"), content).is_empty(),
                "{name} should be exempt"
            );
        }
    }

    #[test]
    fn non_heading_mentions_do_not_count() {
        // "Related" appears in prose but not as a heading.
        let content = "# Concept\n\nSee the Related section in the guide.\n";
        assert!(!has_reference_section(content));
        assert!(!validate("ctx.md", content).is_empty());
    }

    #[test]
    fn four_hashes_are_headings_too() {
        let content = card_with("#### Related Context");
        assert!(has_reference_section(&content));
    }

    #[test]
    fn error_has_10_3_shape() {
        let errs = validate("ctx.md", "# Concept\n\nNo ref.\n");
        let rendered = errs[0].to_string();
        assert!(rendered.starts_with("E200 [context] ctx.md"));
        assert!(rendered.contains("rule: CTX-208"));
        assert!(rendered.contains("hint:"));
    }
}
