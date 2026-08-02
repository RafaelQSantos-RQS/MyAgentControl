//! HTML-comment frontmatter parser for context files (context-spec §3.3, CTX-1).
//!
//! Format (single line at the top of every context file):
//!
//! ```html
//! <!-- Context: {category}/{function} | Priority: {level} | Version: X.Y | Updated: YYYY-MM-DD -->
//! ```
//!
//! - `category` — a path like `core/navigation` or `content-creation/hooks`.
//! - `priority` — one of `{critical, high, medium, low}` (§3.4).
//! - `version` — `MAJOR.MINOR` (§3.5); extra numeric components tolerated.
//! - `updated` — a real calendar date `YYYY-MM-DD`.
//!
//! Errors use the two-tier scheme (cli-spec §7): `E200` schema envelope with
//! module rule IDs `CTX-201..207` and the §10.3 output shape.

use crate::core::errors::{Module, ValidationError};

/// Allowed priority levels (context-spec §3.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl Priority {
    /// Canonical lowercase label.
    pub fn as_str(self) -> &'static str {
        match self {
            Priority::Critical => "critical",
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "critical" => Ok(Priority::Critical),
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(()),
        }
    }
}

/// `MAJOR.MINOR` version (§3.5). Extra numeric components (e.g. `1.0.0`)
/// found in the reference tree are tolerated and dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Version {
    /// Parse `X.Y` (or `X.Y.Z` — patch tolerated). Rejects letters (`x.y`).
    fn parse_xy(s: &str) -> Option<Version> {
        let mut parts = s.split('.');
        let major = parts.next()?.parse::<u32>().ok()?;
        let minor = parts.next()?.parse::<u32>().ok()?;
        Some(Version { major, minor })
    }
}

/// A real calendar date `YYYY-MM-DD` (leap years handled).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    /// Parse `YYYY-MM-DD` and validate it is a real date (month 1–12,
    /// day within the month's range, leap-year-aware Feb).
    fn parse_ymd(s: &str) -> Option<Date> {
        if s.len() != 10 {
            return None;
        }
        let mut parts = s.split('-');
        let year = parts.next()?.parse::<u32>().ok()?;
        let month = parts.next()?.parse::<u32>().ok()?;
        let day = parts.next()?.parse::<u32>().ok()?;
        if parts.next().is_some() || !(1..=12).contains(&month) || day < 1 {
            return None;
        }
        let max_day = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                if leap { 29 } else { 28 }
            }
            _ => return None,
        };
        if day > max_day {
            return None;
        }
        Some(Date { year, month, day })
    }
}

/// Typed result of parsing a context file's HTML-comment frontmatter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextFrontmatter {
    pub category: String,
    pub priority: Priority,
    pub version: Version,
    pub updated: Date,
}

/// Parse the HTML-comment frontmatter from a context file's content.
///
/// Returns the typed fields on success, or **all** schema defects found
/// (CTX-2xx) so callers can aggregate and report every issue at once.
pub fn parse(path: &str, content: &str) -> Result<ContextFrontmatter, Vec<ValidationError>> {
    let mut errors = Vec::new();

    let first = content.lines().map(str::trim).find(|l| !l.is_empty());
    let Some(line) = first else {
        return Err(vec![frontmatter_missing(path)]);
    };
    if !line.starts_with("<!--") || !line.ends_with("-->") {
        return Err(vec![frontmatter_missing(path)]);
    }

    let inner = line
        .trim_start_matches("<!--")
        .trim_end_matches("-->")
        .trim();

    let mut category: Option<String> = None;
    let mut priority: Option<Priority> = None;
    let mut version: Option<Version> = None;
    let mut updated: Option<Date> = None;

    for segment in inner.split('|') {
        let segment = segment.trim();
        let Some((key, value)) = segment.split_once(':') else {
            errors.push(field_error(
                path,
                "CTX-202",
                format!("malformed frontmatter segment {segment:?}"),
                Some("each `|` field must be `Key: value`".to_string()),
            ));
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        match key.as_str() {
            "context" if is_valid_category(value) => category = Some(value.to_string()),
            "context" => errors.push(field_error(
                path,
                "CTX-203",
                format!("invalid category {value:?}"),
                Some("category must be a path like `core/navigation`".to_string()),
            )),
            "priority" => match value.parse::<Priority>() {
                Ok(p) => priority = Some(p),
                Err(()) => errors.push(field_error(
                    path,
                    "CTX-204",
                    format!("invalid priority {value:?}"),
                    Some("use one of: critical, high, medium, low".to_string()),
                )),
            },
            "version" => match Version::parse_xy(value) {
                Some(v) => version = Some(v),
                None => errors.push(field_error(
                    path,
                    "CTX-205",
                    format!("invalid version {value:?}"),
                    Some("use MAJOR.MINOR (e.g. 1.0)".to_string()),
                )),
            },
            "updated" => match Date::parse_ymd(value) {
                Some(d) => updated = Some(d),
                None => errors.push(field_error(
                    path,
                    "CTX-206",
                    format!("invalid date {value:?}"),
                    Some("use YYYY-MM-DD (e.g. 2026-02-15)".to_string()),
                )),
            },
            _ => { /* unknown keys are ignored (forward-compat) */ }
        }
    }

    if category.is_none() {
        errors.push(field_error(
            path,
            "CTX-207",
            "missing required field `Context`",
            None,
        ));
    }
    if priority.is_none() {
        errors.push(field_error(
            path,
            "CTX-207",
            "missing required field `Priority`",
            None,
        ));
    }
    if version.is_none() {
        errors.push(field_error(
            path,
            "CTX-207",
            "missing required field `Version`",
            None,
        ));
    }
    if updated.is_none() {
        errors.push(field_error(
            path,
            "CTX-207",
            "missing required field `Updated`",
            None,
        ));
    }

    // A malformed segment (no `:`) pushes CTX-202 with all fields still `Some`,
    // so this guard must run before the destructure below.
    if !errors.is_empty() {
        return Err(errors);
    }

    match (category, priority, version, updated) {
        (Some(category), Some(priority), Some(version), Some(updated)) => Ok(ContextFrontmatter {
            category,
            priority,
            version,
            updated,
        }),
        // Every missing required field pushed a CTX-207 above, so this arm is
        // unreachable after the errors guard.
        _ => Err(errors),
    }
}

/// Category is a non-empty path of alnum segments joined by `/`.
fn is_valid_category(s: &str) -> bool {
    !s.is_empty()
        && s.split('/').all(|seg| {
            !seg.is_empty()
                && seg
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        })
}

fn frontmatter_missing(path: &str) -> ValidationError {
    ValidationError::schema(
        Module::Context,
        path,
        Some(1),
        "CTX-201",
        "missing HTML-comment frontmatter",
        Some(
            "add as the first line: `<!-- Context: {category}/{function} | Priority: {level} | Version: X.Y | Updated: YYYY-MM-DD -->`"
                .to_string(),
        ),
    )
}

fn field_error(
    path: &str,
    rule: &str,
    message: impl Into<String>,
    hint: Option<String>,
) -> ValidationError {
    ValidationError::schema(Module::Context, path, Some(1), rule, message, hint)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fm(path: &str, content: &str) -> Result<ContextFrontmatter, Vec<ValidationError>> {
        parse(path, content)
    }

    #[test]
    fn valid_frontmatter_parses_to_typed_fields() {
        let f = fm(
            "ctx.md",
            "<!-- Context: core/navigation | Priority: critical | Version: 1.0 | Updated: 2026-02-15 -->\n\n# Nav\n",
        )
        .unwrap();
        assert_eq!(f.category, "core/navigation");
        assert_eq!(f.priority, Priority::Critical);
        assert_eq!(f.version, Version { major: 1, minor: 0 });
        assert_eq!(
            f.updated,
            Date {
                year: 2026,
                month: 2,
                day: 15
            }
        );
    }

    #[test]
    fn valid_with_leading_whitespace_and_lowercase_values() {
        let f = fm(
            "ctx.md",
            "  <!-- Context: learning/README | priority: high | Version: 2.0 | Updated: 2025-01-21 -->\n",
        )
        .unwrap();
        assert_eq!(f.priority, Priority::High);
        assert_eq!(f.version, Version { major: 2, minor: 0 });
    }

    #[test]
    fn three_part_version_is_tolerated() {
        // Reference tree contains some `1.0.0` values; tolerate patch part.
        let f = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: medium | Version: 1.0.0 | Updated: 2026-02-15 -->",
        )
        .unwrap();
        assert_eq!(f.version, Version { major: 1, minor: 0 });
    }

    #[test]
    fn priority_urgent_rejected() {
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: urgent | Version: 1.0 | Updated: 2026-02-15 -->",
        )
        .unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-204")));
    }

    #[test]
    fn version_letters_rejected() {
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: high | Version: x.y | Updated: 2026-02-15 -->",
        )
        .unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-205")));
    }

    #[test]
    fn wrong_date_format_rejected() {
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: high | Version: 1.0 | Updated: 08/02/2026 -->",
        )
        .unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-206")));
    }

    #[test]
    fn impossible_calendar_date_rejected() {
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: high | Version: 1.0 | Updated: 2026-02-30 -->",
        )
        .unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-206")));
    }

    #[test]
    fn leap_year_feb_29_accepted() {
        let f = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: low | Version: 1.0 | Updated: 2024-02-29 -->",
        )
        .unwrap();
        assert_eq!(
            f.updated,
            Date {
                year: 2024,
                month: 2,
                day: 29
            }
        );
    }

    #[test]
    fn missing_frontmatter_rejected() {
        let errs = fm("ctx.md", "# No frontmatter here\n").unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-201")));
    }

    #[test]
    fn missing_required_field_rejected() {
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: high | Updated: 2026-02-15 -->",
        )
        .unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-207")));
        assert!(errs.iter().any(|e| e.to_string().contains("Version")));
    }

    #[test]
    fn multiple_errors_aggregated() {
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: urgent | Version: x.y | Updated: 2026-02-15 -->",
        )
        .unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-204")));
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-205")));
    }

    #[test]
    fn malformed_segment_with_all_fields_present_is_rejected() {
        // Regression: a segment without `:` pushes CTX-202 while all four
        // required fields remain Some — the Ok destructure alone would drop
        // the error. The errors guard must run first.
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: high | Version: 1.0 | Updated: 2026-01-01 | badsegment -->",
        )
        .unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-202")));
    }

    #[test]
    fn unknown_keys_are_ignored() {
        let f = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: high | Version: 1.0 | Updated: 2026-02-15 | Foo: bar -->",
        )
        .unwrap();
        assert_eq!(f.category, "core/x");
        assert_eq!(f.priority, Priority::High);
    }

    #[test]
    fn non_padded_date_rejected() {
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: high | Version: 1.0 | Updated: 2026-2-5 -->",
        )
        .unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-206")));
    }

    #[test]
    fn empty_content_rejected() {
        let errs = fm("ctx.md", "").unwrap_err();
        assert!(errs.iter().any(|e| e.to_string().contains("CTX-201")));
    }

    #[test]
    fn error_has_10_3_shape() {
        let errs = fm(
            "ctx.md",
            "<!-- Context: core/x | Priority: urgent | Version: 1.0 | Updated: 2026-02-15 -->",
        )
        .unwrap_err();
        let rendered = errs[0].to_string();
        assert!(rendered.starts_with("E200 [context] ctx.md:1"));
        assert!(rendered.contains("rule: CTX-204"));
        assert!(rendered.contains("hint:"));
    }
}
