//! Shared validation utilities for YAML `---` frontmatter across agents, skills, and commands.

pub mod agents;
pub mod commands;
pub mod skills;

use std::collections::HashMap;

/// Parsed YAML frontmatter as a flat key-value map.
pub type Frontmatter = HashMap<String, serde_yaml::Value>;

/// Errors from frontmatter parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontmatterError {
    /// No `---` fences found.
    Missing,
    /// YAML parse failure.
    Parse(String),
}

impl std::fmt::Display for FrontmatterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontmatterError::Missing => write!(f, "no YAML frontmatter (--- fences) found"),
            FrontmatterError::Parse(e) => write!(f, "frontmatter parse error: {e}"),
        }
    }
}

impl std::error::Error for FrontmatterError {}

/// Extract YAML frontmatter from a markdown file's content.
///
/// Expects content starting with `---\n...\n---`. Returns the parsed
/// key-value map on success.
pub fn parse_frontmatter(content: &str) -> Result<Frontmatter, FrontmatterError> {
    let content = content.trim_start();
    let Some(rest) = content.strip_prefix("---") else {
        return Err(FrontmatterError::Missing);
    };
    let rest = rest.trim_start_matches('\n');

    // Find closing ---
    let Some(end_idx) = rest.find("\n---") else {
        return Err(FrontmatterError::Missing);
    };

    let yaml_str = &rest[..end_idx];
    let map: Frontmatter =
        serde_yaml::from_str(yaml_str).map_err(|e| FrontmatterError::Parse(e.to_string()))?;
    Ok(map)
}

/// Get a required string field from frontmatter.
pub fn required_str<'a>(fm: &'a Frontmatter, key: &str) -> Result<&'a str, String> {
    fm.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("missing required field: {key}"))
}

/// Get an optional string field from frontmatter.
pub fn optional_str<'a>(fm: &'a Frontmatter, key: &str) -> Option<&'a str> {
    fm.get(key).and_then(|v| v.as_str())
}

/// Get an optional list of strings from frontmatter.
pub fn optional_str_list(fm: &Frontmatter, key: &str) -> Vec<String> {
    fm.get(key)
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_frontmatter() {
        let content = "---\nname: test\ndescription: A test\n---\n# Body";
        let fm = parse_frontmatter(content).unwrap();
        assert_eq!(fm.get("name").unwrap().as_str().unwrap(), "test");
        assert_eq!(fm.get("description").unwrap().as_str().unwrap(), "A test");
    }

    #[test]
    fn parse_missing_fences() {
        let content = "# Just a heading";
        assert_eq!(parse_frontmatter(content), Err(FrontmatterError::Missing));
    }

    #[test]
    fn parse_no_closing_fence() {
        let content = "---\nname: test";
        assert_eq!(parse_frontmatter(content), Err(FrontmatterError::Missing));
    }

    #[test]
    fn parse_invalid_yaml() {
        let content = "---\n:\n  bad yaml\n---";
        assert!(matches!(
            parse_frontmatter(content),
            Err(FrontmatterError::Parse(_))
        ));
    }

    #[test]
    fn required_str_ok() {
        let mut fm = Frontmatter::new();
        fm.insert("name".to_string(), serde_yaml::Value::String("x".into()));
        assert_eq!(required_str(&fm, "name").unwrap(), "x");
    }

    #[test]
    fn required_str_missing() {
        let fm = Frontmatter::new();
        assert!(required_str(&fm, "name").is_err());
    }

    #[test]
    fn optional_str_list_present() {
        let mut fm = Frontmatter::new();
        let tags = vec![
            serde_yaml::Value::String("a".into()),
            serde_yaml::Value::String("b".into()),
        ];
        fm.insert("tags".to_string(), serde_yaml::Value::Sequence(tags));
        assert_eq!(optional_str_list(&fm, "tags"), vec!["a", "b"]);
    }

    #[test]
    fn optional_str_list_missing() {
        let fm = Frontmatter::new();
        assert!(optional_str_list(&fm, "tags").is_empty());
    }
}
