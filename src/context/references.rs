#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefError {
    /// Dynamic reference like `@${var}` or `@$path`
    DynamicReference(String),
    /// Non-standard `@` reference not in the allowlist
    NonStandardReference(String),
}

impl std::fmt::Display for RefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefError::DynamicReference(r) => {
                write!(f, "CTX-209: dynamic reference \"{r}\" is not allowed")
            }
            RefError::NonStandardReference(r) => {
                write!(
                    f,
                    "CTX-210: non-standard reference \"{r}\" is not in the allowlist"
                )
            }
        }
    }
}

impl std::error::Error for RefError {}

/// Check if an `@` reference token is allowed.
///
/// Allowlist:
/// - `@.opencode/context/...`
/// - `@AGENTS.md`
/// - `@.cursorrules`
/// - `@$N` (positional arguments, digit after $)
/// - email: contains `@` before the token start or `mailto:` prefix
fn is_allowed_ref(token: &str) -> bool {
    // .opencode/context/ prefix
    if token.starts_with("@.opencode/context/") {
        return true;
    }
    // Exact allowed files
    if token == "@AGENTS.md" || token == "@.cursorrules" {
        return true;
    }
    // Positional args: @$N where N is a digit
    if token.starts_with("@$") && token.len() == 3 && token.as_bytes()[2].is_ascii_digit() {
        return true;
    }
    false
}

/// Validate `@`-reference syntax in a text block.
///
/// Returns a list of errors for any invalid references found.
pub fn validate_references(text: &str) -> Vec<RefError> {
    let mut errors = Vec::new();

    for line in text.lines() {
        // Find all @ tokens: scan for @ not inside code blocks or HTML comments
        for (i, ch) in line.char_indices() {
            if ch != '@' {
                continue;
            }

            // Skip if inside backtick code span
            // Simple heuristic: count backticks before this position
            let before = &line[..i];
            let backtick_count = before.chars().filter(|&c| c == '`').count();
            if backtick_count % 2 == 1 {
                continue; // inside a code span
            }

            // Extract the token: @ followed by word chars, path chars, $, or email prefix
            let rest = &line[i..];
            let token_len = rest[1..]
                .chars()
                .take_while(|c| {
                    c.is_alphanumeric() || *c == '_' || *c == '/' || *c == '.' || *c == '$'
                })
                .count();
            let token = &rest[..=token_len]; // include the @

            // Check for email pattern: something@domain
            if i > 0 {
                let prev_char = line[..i].chars().last();
                if let Some(prev) = prev_char
                    && (prev.is_alphanumeric() || prev == '.' || prev == '-' || prev == '_')
                {
                    continue; // part of an email address
                }
            }

            // Check for mailto: prefix
            if line[..i].ends_with("mailto:") {
                continue;
            }

            // Check allowlist
            if !is_allowed_ref(token) {
                // Dynamic reference: @$ followed by non-digit (e.g. @${var}, @$path, bare @$)
                if token.starts_with("@$") {
                    let third = token.as_bytes().get(2).copied();
                    match third {
                        Some(c) if c.is_ascii_digit() => {
                            // @$N — should have been allowed; shouldn't reach here
                            errors.push(RefError::NonStandardReference(token.to_string()));
                        }
                        _ => {
                            errors.push(RefError::DynamicReference(token.to_string()));
                        }
                    }
                } else {
                    errors.push(RefError::NonStandardReference(token.to_string()));
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_opencode_ref() {
        let text = "See @.opencode/context/core/standards/code-quality.md";
        assert!(validate_references(text).is_empty());
    }

    #[test]
    fn valid_agents_md() {
        let text = "Check @AGENTS.md for details";
        assert!(validate_references(text).is_empty());
    }

    #[test]
    fn valid_cursorrules() {
        let text = "Follow @.cursorrules";
        assert!(validate_references(text).is_empty());
    }

    #[test]
    fn valid_positional_arg() {
        let text = "Use @$1 and @$2";
        assert!(validate_references(text).is_empty());
    }

    #[test]
    fn valid_email() {
        let text = "Email team@example.com for info";
        assert!(validate_references(text).is_empty());
    }

    #[test]
    fn valid_mailto() {
        let text = "Send to mailto:team@example.com";
        assert!(validate_references(text).is_empty());
    }

    #[test]
    fn invalid_dynamic_reference() {
        let text = "Use @${var} here";
        let errors = validate_references(text);
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], RefError::DynamicReference(_)));
    }

    #[test]
    fn invalid_non_standard() {
        let text = "See @some-other-place";
        let errors = validate_references(text);
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], RefError::NonStandardReference(_)));
    }

    #[test]
    fn mixed_valid_and_invalid() {
        let text = "Check @AGENTS.md and @bad-place";
        let errors = validate_references(text);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn inside_code_span_ignored() {
        let text = "Use `@some-bad-ref` in code";
        assert!(validate_references(text).is_empty());
    }
}
