//! SKILL.md YAML-frontmatter validator (skills-spec §3, OpenCode official).
//!
//! OpenCode discovers skills only at `.opencode/skills/<name>/SKILL.md`
//! (plural) and each `SKILL.md` must start with YAML frontmatter. Per the
//! official docs (<https://opencode.ai/docs/skills/>) only these fields are
//! recognized: `name` (required), `description` (required), `license`
//! (optional), `compatibility` (optional), `metadata` (optional).
//! **Unknown frontmatter fields are ignored** — so the OAC tree's extra
//! `version`/`author`/`type`/`category`/`tags` keys are tolerated.
//!
//! `name` must be 1–64 chars, lowercase alphanumeric with single hyphen
//! separators (`^[a-z0-9]+(-[a-z0-9]+)*$`), and match the folder name.
//! `description` must be 1–1024 chars.
//!
//! Error codes: `SK-206` missing frontmatter block, `SK-207` YAML parse
//! error, `SK-208` missing `name`, `SK-209` invalid `name`, `SK-201` name ≠
//! folder, `SK-210` missing `description`, `SK-211` description length — all
//! wrapped in the `E200 [skills]` envelope (cli-spec §7, §10.3).
//!
//! > Spec note (v0.1.1): MAC-SK §3 lists stricter authoring rules
//! > (`type: skill`, semver, category/tags). Those are **authoring
//! > conventions** for wizard-generated skills (SK-4), not loadability rules:
//! > the pristine OAC tree (e.g. `context7`) omits them and must still pass
//! > (AC-S1). This validator enforces the OpenCode-official contract.

use crate::core::errors::{Module, ValidationError};
use serde::Deserialize;

/// Recognized-by-OpenCode frontmatter fields (unknown keys are ignored by
/// serde by default, matching the documented behavior).
#[derive(Debug, Deserialize)]
struct RawFrontmatter {
    name: Option<String>,
    description: Option<String>,
}

/// Typed result of parsing a `SKILL.md` frontmatter block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
}

/// Parse and validate the YAML frontmatter of a `SKILL.md` file.
///
/// - `path` — the SKILL.md path (its parent dir name is the skill folder).
/// - `content` — the full SKILL.md text.
///
/// Returns the typed fields on success, or **all** defects found (SK-2xx) so
/// callers can aggregate and report every issue at once.
pub fn parse(path: &str, content: &str) -> Result<SkillFrontmatter, Vec<ValidationError>> {
    let mut errors = Vec::new();

    let Some(block) = extract_frontmatter(content) else {
        return Err(vec![frontmatter_missing(path)]);
    };

    let raw: RawFrontmatter = match serde_saphyr::from_str(&block) {
        Ok(raw) => raw,
        Err(e) => {
            return Err(vec![yaml_parse_error(path, &e.to_string())]);
        }
    };

    let name = raw.name.clone();
    let description = raw.description.clone();

    if let Some(name) = &name {
        let len = name.chars().count();
        if !is_valid_name(name) || !(1..=64).contains(&len) {
            errors.push(invalid_name(path, name));
        }
        if let Some(folder) = folder_from_path(path)
            && name != &folder
        {
            errors.push(name_folder_mismatch(path, name, &folder));
        }
    } else {
        errors.push(missing_field(path, "SK-208", "name"));
    }

    if let Some(description) = &description {
        let len = description.chars().count();
        if !(1..=1024).contains(&len) {
            errors.push(description_length(path, len));
        }
    } else {
        errors.push(missing_field(path, "SK-210", "description"));
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(SkillFrontmatter {
        name: name.unwrap(),
        description: description.unwrap(),
    })
}

/// Extract the YAML block between the leading `---` delimiters of a SKILL.md.
fn extract_frontmatter(content: &str) -> Option<String> {
    let mut lines = content.lines();
    let first = lines.next()?.trim();
    if first != "---" {
        return None;
    }
    let mut block = String::new();
    for line in lines {
        if line.trim() == "---" {
            return Some(block);
        }
        block.push_str(line);
        block.push('\n');
    }
    None
}

/// The skill folder name = the parent directory of the SKILL.md path.
fn folder_from_path(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .parent()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
}

/// OpenCode `name` rule: `^[a-z0-9]+(-[a-z0-9]+)*$` — lowercase alnum runs
/// separated by single hyphens, no leading/trailing hyphen, no `--`.
fn is_valid_name(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let bytes = s.as_bytes();
    let mut first = true;
    let mut prev_hyphen = false;
    for &b in bytes {
        let alnum = b.is_ascii_lowercase() || b.is_ascii_digit();
        let hyphen = b == b'-';
        if !alnum && !hyphen {
            return false;
        }
        if first && hyphen {
            return false;
        }
        if hyphen && prev_hyphen {
            return false;
        }
        first = false;
        prev_hyphen = hyphen;
    }
    !prev_hyphen
}

fn frontmatter_missing(path: &str) -> ValidationError {
    ValidationError::schema(
        Module::Skills,
        path,
        Some(1),
        "SK-206",
        "missing YAML frontmatter block",
        Some("SKILL.md must start with `---\\nname: ...\\ndescription: ...\\n---`".to_string()),
    )
}

fn yaml_parse_error(path: &str, detail: &str) -> ValidationError {
    ValidationError::schema(
        Module::Skills,
        path,
        Some(1),
        "SK-207",
        format!("frontmatter is not valid YAML: {detail}"),
        Some("check the frontmatter between the `---` delimiters".to_string()),
    )
}

fn invalid_name(path: &str, name: &str) -> ValidationError {
    ValidationError::schema(
        Module::Skills,
        path,
        Some(1),
        "SK-209",
        format!("invalid skill `name` {name:?}"),
        Some(
            "use 1-64 lowercase letters/digits with single hyphens: ^[a-z0-9]+(-[a-z0-9]+)*$"
                .to_string(),
        ),
    )
}

fn name_folder_mismatch(path: &str, name: &str, folder: &str) -> ValidationError {
    ValidationError::schema(
        Module::Skills,
        path,
        Some(1),
        "SK-201",
        format!("`name` {name:?} does not match folder name {folder:?}"),
        Some("the SKILL.md `name` must equal its parent folder name".to_string()),
    )
}

fn missing_field(path: &str, rule: &str, field: &str) -> ValidationError {
    ValidationError::schema(
        Module::Skills,
        path,
        Some(1),
        rule,
        format!("missing required field `{field}`"),
        None,
    )
}

fn description_length(path: &str, len: usize) -> ValidationError {
    ValidationError::schema(
        Module::Skills,
        path,
        Some(1),
        "SK-211",
        format!("`description` must be 1-1024 characters, got {len}"),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(path: &str, content: &str) -> SkillFrontmatter {
        parse(path, content).expect("expected valid frontmatter")
    }

    fn errs(path: &str, content: &str) -> Vec<ValidationError> {
        parse(path, content).expect_err("expected validation errors")
    }

    #[test]
    fn valid_minimal_skill_parses() {
        let f = ok(
            "content/skills/context7/SKILL.md",
            "---\nname: context7\ndescription: Docs lookup via Context7 API\n---\n\n# Context7\n",
        );
        assert_eq!(f.name, "context7");
        assert_eq!(f.description, "Docs lookup via Context7 API");
    }

    #[test]
    fn valid_with_unknown_fields_ignored() {
        // OAC tree carries version/author/type/category/tags — ignored by OpenCode.
        let f = ok(
            "content/skills/task-management/SKILL.md",
            "---\nname: task-management\ndescription: Task CLI\ntype: skill\nversion: 1.0.0\ncategory: development\ntags:\n  - cli\n---\n",
        );
        assert_eq!(f.name, "task-management");
    }

    #[test]
    fn name_matching_folder_hyphenated_passes() {
        let f = ok(
            "content/skills/smart-router-skill/SKILL.md",
            "---\nname: smart-router-skill\ndescription: Character router\n---\n",
        );
        assert_eq!(f.name, "smart-router-skill");
    }

    #[test]
    fn name_not_matching_folder_rejected() {
        let errs = errs(
            "content/skills/context7/SKILL.md",
            "---\nname: wrong-name\ndescription: x\n---\n",
        );
        assert!(errs.iter().any(|e| e.to_string().contains("SK-201")));
    }

    #[test]
    fn missing_frontmatter_block_rejected() {
        let errs = errs("content/skills/foo/SKILL.md", "# No frontmatter\n");
        assert!(errs.iter().any(|e| e.to_string().contains("SK-206")));
    }

    #[test]
    fn unterminated_frontmatter_rejected() {
        let errs = errs("content/skills/foo/SKILL.md", "---\nname: foo\n");
        assert!(errs.iter().any(|e| e.to_string().contains("SK-206")));
    }

    #[test]
    fn malformed_yaml_rejected() {
        let errs = errs(
            "content/skills/foo/SKILL.md",
            "---\nname: [unclosed\ndescription: x\n---\n",
        );
        assert!(errs.iter().any(|e| e.to_string().contains("SK-207")));
    }

    #[test]
    fn missing_name_rejected() {
        let errs = errs("content/skills/foo/SKILL.md", "---\ndescription: x\n---\n");
        assert!(errs.iter().any(|e| e.to_string().contains("SK-208")));
    }

    #[test]
    fn missing_description_rejected() {
        let errs = errs("content/skills/foo/SKILL.md", "---\nname: foo\n---\n");
        assert!(errs.iter().any(|e| e.to_string().contains("SK-210")));
    }

    #[test]
    fn uppercase_name_rejected() {
        let errs = errs(
            "content/skills/foo/SKILL.md",
            "---\nname: Foo\ndescription: x\n---\n",
        );
        assert!(errs.iter().any(|e| e.to_string().contains("SK-209")));
    }

    #[test]
    fn underscore_and_leading_hyphen_rejected() {
        for bad in ["foo_bar", "-foo", "foo-", "foo--bar"] {
            let errs = errs(
                "content/skills/foo/SKILL.md",
                &format!("---\nname: {bad}\ndescription: x\n---\n"),
            );
            assert!(
                errs.iter().any(|e| e.to_string().contains("SK-209")),
                "name {bad:?} should be rejected"
            );
        }
    }

    #[test]
    fn name_over_64_chars_rejected() {
        let long = "a".repeat(65);
        let errs = errs(
            "content/skills/foo/SKILL.md",
            &format!("---\nname: {long}\ndescription: x\n---\n"),
        );
        assert!(errs.iter().any(|e| e.to_string().contains("SK-209")));
    }

    #[test]
    fn description_over_1024_chars_rejected() {
        let long = "d".repeat(1025);
        let errs = errs(
            "content/skills/foo/SKILL.md",
            &format!("---\nname: foo\ndescription: {long}\n---\n"),
        );
        assert!(errs.iter().any(|e| e.to_string().contains("SK-211")));
    }

    #[test]
    fn multiple_errors_aggregated() {
        // `description: ` (empty YAML value) deserializes to `None` in
        // serde-saphyr, so an empty description yields SK-210 (missing) — use
        // an over-length description to trigger SK-211 deterministically.
        let long = "d".repeat(1025);
        let errs = errs(
            "content/skills/foo/SKILL.md",
            &format!("---\nname: BAD-NAME\ndescription: {long}\n---\n"),
        );
        assert!(errs.iter().any(|e| e.to_string().contains("SK-209")));
        assert!(errs.iter().any(|e| e.to_string().contains("SK-211")));
    }

    #[test]
    fn error_has_10_3_shape() {
        let errs = errs(
            "content/skills/context7/SKILL.md",
            "---\nname: wrong\ndescription: x\n---\n",
        );
        let rendered = errs[0].to_string();
        assert!(rendered.starts_with("E200 [skills] content/skills/context7/SKILL.md:1"));
        assert!(rendered.contains("rule: SK-201"));
        assert!(rendered.contains("hint:"));
    }
}
