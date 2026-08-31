/// HTML-comment frontmatter for context files.
///
/// Format: `<!-- Context: {category} | Priority: {level} | Version: X.Y | Updated: YYYY-MM-DD -->`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frontmatter {
    pub category: String,
    pub priority: Priority,
    pub version: String,
    pub updated: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Critical => "critical",
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "critical" => Some(Priority::Critical),
            "high" => Some(Priority::High),
            "medium" => Some(Priority::Medium),
            "low" => Some(Priority::Low),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontmatterError {
    Missing,
    InvalidPriority(String),
    InvalidVersion(String),
    InvalidDate(String),
}

impl std::fmt::Display for FrontmatterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontmatterError::Missing => write!(f, "CTX-202: missing HTML-comment frontmatter"),
            FrontmatterError::InvalidPriority(v) => {
                write!(
                    f,
                    "CTX-201: invalid priority \"{v}\"; must be critical, high, medium, or low"
                )
            }
            FrontmatterError::InvalidVersion(v) => {
                write!(f, "CTX-203: invalid version \"{v}\"; must be semver X.Y")
            }
            FrontmatterError::InvalidDate(v) => {
                write!(f, "CTX-204: invalid date \"{v}\"; must be YYYY-MM-DD")
            }
        }
    }
}

impl std::error::Error for FrontmatterError {}

/// Parse frontmatter from the first line of a context file.
///
/// Returns `Err` if the line is not valid frontmatter or contains invalid field values.
pub fn parse_frontmatter(line: &str) -> Result<Frontmatter, FrontmatterError> {
    let line = line.trim();

    // Must start with `<!-- Context:` and end with `-->`
    let Some(inner) = line
        .strip_prefix("<!-- ")
        .and_then(|s| s.strip_suffix(" -->"))
    else {
        return Err(FrontmatterError::Missing);
    };

    // Must start with `Context:`
    let Some(rest) = inner.strip_prefix("Context: ") else {
        return Err(FrontmatterError::Missing);
    };

    let fields: Vec<&str> = rest.split(" | ").collect();
    if fields.len() < 4 {
        return Err(FrontmatterError::Missing);
    }

    let category = fields[0].trim().to_string();

    let priority_str = fields[1].strip_prefix("Priority: ").unwrap_or("").trim();
    let priority = Priority::parse(priority_str)
        .ok_or_else(|| FrontmatterError::InvalidPriority(priority_str.to_string()))?;

    let version = fields[2]
        .strip_prefix("Version: ")
        .unwrap_or("")
        .trim()
        .to_string();
    if !is_valid_version(&version) {
        return Err(FrontmatterError::InvalidVersion(version));
    }

    let updated = fields[3]
        .strip_prefix("Updated: ")
        .unwrap_or("")
        .trim()
        .to_string();
    if !is_valid_date(&updated) {
        return Err(FrontmatterError::InvalidDate(updated));
    }

    Ok(Frontmatter {
        category,
        priority,
        version,
        updated,
    })
}

fn is_valid_version(v: &str) -> bool {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 2 {
        return false;
    }
    parts[0].parse::<u32>().is_ok() && parts[1].parse::<u32>().is_ok()
}

fn is_valid_date(d: &str) -> bool {
    let parts: Vec<&str> = d.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    parts[0].parse::<u32>().is_ok()
        && parts[0].len() == 4
        && parts[1].parse::<u32>().is_ok()
        && parts[1].len() == 2
        && parts[2].parse::<u32>().is_ok()
        && parts[2].len() == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_frontmatter() {
        let line = "<!-- Context: core/standards/code-quality | Priority: critical | Version: 1.2 | Updated: 2026-08-02 -->";
        let fm = parse_frontmatter(line).unwrap();
        assert_eq!(fm.category, "core/standards/code-quality");
        assert_eq!(fm.priority, Priority::Critical);
        assert_eq!(fm.version, "1.2");
        assert_eq!(fm.updated, "2026-08-02");
    }

    #[test]
    fn missing_frontmatter() {
        let line = "# Just a heading";
        assert_eq!(parse_frontmatter(line), Err(FrontmatterError::Missing));
    }

    #[test]
    fn invalid_priority() {
        let line =
            "<!-- Context: core/test | Priority: urgent | Version: 1.0 | Updated: 2026-01-01 -->";
        assert!(matches!(
            parse_frontmatter(line),
            Err(FrontmatterError::InvalidPriority(_))
        ));
    }

    #[test]
    fn invalid_version() {
        let line =
            "<!-- Context: core/test | Priority: high | Version: x.y | Updated: 2026-01-01 -->";
        assert!(matches!(
            parse_frontmatter(line),
            Err(FrontmatterError::InvalidVersion(_))
        ));
    }

    #[test]
    fn invalid_date() {
        let line =
            "<!-- Context: core/test | Priority: high | Version: 1.0 | Updated: 08/02/2026 -->";
        assert!(matches!(
            parse_frontmatter(line),
            Err(FrontmatterError::InvalidDate(_))
        ));
    }
}
