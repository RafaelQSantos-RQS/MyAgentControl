//! Skill file validation: SKILL.md schema, router.sh, referenced files.

use std::path::Path;

use super::{Frontmatter, FrontmatterError, parse_frontmatter, required_str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    Frontmatter(FrontmatterError),
    MissingField(String),
    RouterMissing,
    RouterNoShebang,
    ReferencedFileMissing(String),
    InvalidStructure(String),
}

impl std::fmt::Display for SkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillError::Frontmatter(e) => write!(f, "SK-701: {e}"),
            SkillError::MissingField(k) => write!(f, "SK-701: missing required field: {k}"),
            SkillError::RouterMissing => write!(f, "SK-702: router.sh not found"),
            SkillError::RouterNoShebang => write!(f, "SK-702: router.sh missing shebang line"),
            SkillError::ReferencedFileMissing(path) => {
                write!(f, "SK-703: referenced file not found: {path}")
            }
            SkillError::InvalidStructure(msg) => write!(f, "SK-706: {msg}"),
        }
    }
}

impl std::error::Error for SkillError {}

/// Validate SKILL.md frontmatter (SK-701).
pub fn validate_skill_file(content: &str) -> Result<Frontmatter, SkillError> {
    let fm = parse_frontmatter(content).map_err(SkillError::Frontmatter)?;
    let _name = required_str(&fm, "name").map_err(SkillError::MissingField)?;
    Ok(fm)
}

/// Validate router.sh has shebang line (SK-702).
pub fn validate_router(skill_dir: &Path) -> Result<(), SkillError> {
    let router = skill_dir.join("router.sh");
    if !router.exists() {
        return Err(SkillError::RouterMissing);
    }
    let content = std::fs::read_to_string(&router).map_err(|_e| SkillError::RouterNoShebang)?;
    if !content.starts_with("#!") {
        return Err(SkillError::RouterNoShebang);
    }
    Ok(())
}

/// Validate that referenced files exist (SK-703).
///
/// Scans the skill content for file references like `scripts/foo.ts` and
/// checks they exist in the skill directory.
pub fn validate_referenced_files(content: &str, skill_dir: &Path) -> Vec<SkillError> {
    let mut errors = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        // Look for paths that look like file references
        if trimmed.contains("scripts/") || trimmed.contains("workflows/") {
            // Extract potential paths
            for word in trimmed.split_whitespace() {
                let word = word.trim_matches(|c: char| c == '`' || c == '"' || c == '\'');
                if (word.starts_with("scripts/") || word.starts_with("workflows/"))
                    && !word.contains('*')
                    && !word.contains('$')
                {
                    let path = skill_dir.join(word);
                    if !path.exists() {
                        errors.push(SkillError::ReferencedFileMissing(word.to_string()));
                    }
                }
            }
        }
    }
    errors
}

/// Validate skill directory structure (SK-706).
pub fn validate_structure(skill_dir: &Path) -> Result<(), SkillError> {
    let skill_md = skill_dir.join("SKILL.md");
    if !skill_md.exists() {
        return Err(SkillError::InvalidStructure(
            "SKILL.md not found in skill directory".into(),
        ));
    }
    // router.sh is optional — some skills don't need routing
    Ok(())
}

/// Walk skills directory and return all skill names.
pub fn walk_skills(content_dir: &Path) -> Vec<String> {
    let skills_dir = content_dir.join("skills");
    let mut skills = Vec::new();
    if !skills_dir.exists() {
        return skills;
    }
    if let Ok(entries) = std::fs::read_dir(&skills_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir()
                && let Some(name) = entry.file_name().to_str()
            {
                skills.push(name.to_string());
            }
        }
    }
    skills
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_skill_file() {
        let content = "---\nname: my-skill\ndescription: A skill\n---";
        let fm = validate_skill_file(content).unwrap();
        assert_eq!(fm.get("name").unwrap().as_str().unwrap(), "my-skill");
    }

    #[test]
    fn missing_name() {
        let content = "---\ndescription: A skill\n---";
        assert!(matches!(
            validate_skill_file(content),
            Err(SkillError::MissingField(_))
        ));
    }

    #[test]
    fn router_no_shebang() {
        let dir = std::env::temp_dir().join("mac_test_skill_noshebang");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("router.sh"), "echo hello").unwrap();
        assert!(matches!(
            validate_router(&dir),
            Err(SkillError::RouterNoShebang)
        ));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn router_with_shebang() {
        let dir = std::env::temp_dir().join("mac_test_skill_shebang");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("router.sh"), "#!/bin/bash\necho hello").unwrap();
        assert!(validate_router(&dir).is_ok());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
