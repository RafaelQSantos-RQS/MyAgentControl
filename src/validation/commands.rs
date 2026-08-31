//! Command file validation: frontmatter schema, dependency references.

use std::path::Path;

use super::{Frontmatter, FrontmatterError, optional_str_list, parse_frontmatter, required_str};

/// Valid component types for dependency references.
const VALID_DEP_TYPES: &[&str] = &[
    "subagent", "context", "skill", "command", "agent", "tool", "plugin",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    Frontmatter(FrontmatterError),
    MissingField(String),
    InvalidDependencyRef(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Frontmatter(e) => write!(f, "CMD-401: {e}"),
            CommandError::MissingField(k) => write!(f, "CMD-401: missing required field: {k}"),
            CommandError::InvalidDependencyRef(r) => {
                write!(
                    f,
                    "CMD-402: invalid dependency reference \"{r}\"; expected type:name format"
                )
            }
        }
    }
}

impl std::error::Error for CommandError {}

/// Validate command YAML frontmatter (CMD-401).
pub fn validate_command_file(content: &str) -> Result<Frontmatter, CommandError> {
    let fm = parse_frontmatter(content).map_err(CommandError::Frontmatter)?;
    let _desc = required_str(&fm, "description").map_err(CommandError::MissingField)?;
    Ok(fm)
}

/// Validate dependency references (CMD-402).
pub fn validate_dependencies(fm: &Frontmatter) -> Vec<CommandError> {
    let mut errors = Vec::new();
    let deps = optional_str_list(fm, "dependencies");
    for dep in &deps {
        if let Some((type_name, _name)) = dep.split_once(':') {
            if !VALID_DEP_TYPES.contains(&type_name) {
                errors.push(CommandError::InvalidDependencyRef(dep.clone()));
            }
        } else {
            errors.push(CommandError::InvalidDependencyRef(dep.clone()));
        }
    }
    errors
}

/// Walk commands directory and return all command file names (without .md).
pub fn walk_commands(content_dir: &Path) -> Vec<String> {
    let cmd_dir = content_dir.join("command");
    let mut commands = Vec::new();
    if !cmd_dir.exists() {
        return commands;
    }
    walk_commands_recursive(&cmd_dir, &mut commands);
    commands
}

fn walk_commands_recursive(dir: &std::path::Path, commands: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_commands_recursive(&path, commands);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md")
                && let Some(name) = path.file_stem().and_then(|s| s.to_str())
            {
                commands.push(name.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_command_file() {
        let content = "---\ndescription: Create commits\n---";
        let fm = validate_command_file(content).unwrap();
        assert_eq!(
            fm.get("description").unwrap().as_str().unwrap(),
            "Create commits"
        );
    }

    #[test]
    fn missing_description() {
        let content = "---\ntags: [test]\n---";
        assert!(matches!(
            validate_command_file(content),
            Err(CommandError::MissingField(_))
        ));
    }

    #[test]
    fn valid_dependency() {
        let mut fm = Frontmatter::new();
        let deps = vec![serde_yaml::Value::String("subagent:task-manager".into())];
        fm.insert(
            "dependencies".to_string(),
            serde_yaml::Value::Sequence(deps),
        );
        assert!(validate_dependencies(&fm).is_empty());
    }

    #[test]
    fn invalid_dependency_ref() {
        let mut fm = Frontmatter::new();
        let deps = vec![serde_yaml::Value::String("task-manager".into())];
        fm.insert(
            "dependencies".to_string(),
            serde_yaml::Value::Sequence(deps),
        );
        let errors = validate_dependencies(&fm);
        assert_eq!(errors.len(), 1);
        assert!(matches!(&errors[0], CommandError::InvalidDependencyRef(_)));
    }
}
