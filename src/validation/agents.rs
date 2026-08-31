//! Agent file validation: schema, category JSON, delegation graph.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use super::{Frontmatter, FrontmatterError, optional_str, parse_frontmatter, required_str};

/// Valid permission verbs for agent files.
const VALID_PERMISSION_VERBS: &[&str] = &["allow", "ask", "deny"];

/// Valid agent modes.
const VALID_MODES: &[&str] = &["primary", "subagent"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentError {
    Frontmatter(FrontmatterError),
    MissingField(String),
    InvalidMode(String),
    InvalidPermissionVerb(String),
    InvalidCategoryJson(String),
    DelegationCycle(Vec<String>),
    MissingAgentFile(String),
    UnlistedAgent(String),
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentError::Frontmatter(e) => write!(f, "AG-201: {e}"),
            AgentError::MissingField(k) => write!(f, "AG-201: missing required field: {k}"),
            AgentError::InvalidMode(m) => {
                write!(
                    f,
                    "AG-201: invalid mode \"{m}\"; must be primary or subagent"
                )
            }
            AgentError::InvalidPermissionVerb(v) => {
                write!(
                    f,
                    "AG-202: invalid permission verb \"{v}\"; must be allow, ask, or deny"
                )
            }
            AgentError::InvalidCategoryJson(e) => write!(f, "AG-203: {e}"),
            AgentError::DelegationCycle(path) => {
                write!(f, "AG-204: delegation cycle detected: {}", path.join(" → "))
            }
            AgentError::MissingAgentFile(name) => {
                write!(f, "AG-203: agent file not found for \"{name}\"")
            }
            AgentError::UnlistedAgent(name) => {
                write!(
                    f,
                    "AG-205: agent \"{name}\" not listed in any 0-category.json"
                )
            }
        }
    }
}

impl std::error::Error for AgentError {}

/// Validate agent YAML frontmatter schema (AG-201, AG-202).
pub fn validate_agent_file(content: &str) -> Result<Frontmatter, AgentError> {
    let fm = parse_frontmatter(content).map_err(AgentError::Frontmatter)?;

    // Required fields
    let _name = required_str(&fm, "name").map_err(AgentError::MissingField)?;
    let _desc = required_str(&fm, "description").map_err(AgentError::MissingField)?;

    // Mode validation
    let mode = optional_str(&fm, "mode").unwrap_or("primary");
    if !VALID_MODES.contains(&mode) {
        return Err(AgentError::InvalidMode(mode.to_string()));
    }

    // Permission verb validation
    if let Some(serde_yaml::Value::Mapping(permissions)) = fm.get("permission") {
        for (_key, val) in permissions {
            if let serde_yaml::Value::String(verb) = val
                && !VALID_PERMISSION_VERBS.contains(&verb.as_str())
            {
                return Err(AgentError::InvalidPermissionVerb(verb.clone()));
            }
        }
    }

    Ok(fm)
}

/// Parse a category JSON file (AG-203).
#[derive(Debug, Clone)]
pub struct CategoryInfo {
    pub name: String,
    pub agents: Vec<String>,
}

pub fn parse_category_json(content: &str, path: &Path) -> Result<CategoryInfo, AgentError> {
    let json: serde_json::Value = serde_json::from_str(content).map_err(|e| {
        AgentError::InvalidCategoryJson(format!("failed to parse {}: {e}", path.display()))
    })?;

    let name = json
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let mut agents = Vec::new();
    if let Some(obj) = json.get("agents").and_then(|v| v.as_object()) {
        for key in obj.keys() {
            agents.push(key.clone());
        }
    }

    Ok(CategoryInfo { name, agents })
}

/// Validate delegation graph is acyclic (AG-204).
///
/// `edges`: map of agent name → list of subagent names it invokes.
pub fn validate_delegation_graph(edges: &HashMap<String, Vec<String>>) -> Result<(), AgentError> {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();

    for agent in edges.keys() {
        if !visited.contains(agent) {
            dfs_cycle_check(agent, edges, &mut visited, &mut stack, &mut Vec::new())?;
        }
    }
    Ok(())
}

fn dfs_cycle_check(
    node: &str,
    edges: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    stack: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> Result<(), AgentError> {
    if stack.contains(node) {
        path.push(node.to_string());
        return Err(AgentError::DelegationCycle(path.clone()));
    }
    if visited.contains(node) {
        return Ok(());
    }

    visited.insert(node.to_string());
    stack.insert(node.to_string());
    path.push(node.to_string());

    if let Some(subs) = edges.get(node) {
        for sub in subs {
            dfs_cycle_check(sub, edges, visited, stack, path)?;
        }
    }

    path.pop();
    stack.remove(node);
    Ok(())
}

/// Walk an agent tree and return all agent names found.
pub fn walk_agents(content_dir: &Path) -> Vec<String> {
    let mut agents = Vec::new();
    let agent_dir = content_dir.join("agent");
    if !agent_dir.exists() {
        return agents;
    }
    walk_agents_recursive(&agent_dir, &mut agents);
    agents
}

fn walk_agents_recursive(dir: &std::path::Path, agents: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_agents_recursive(&path, agents);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md")
                && let Some(name) = path.file_stem().and_then(|s| s.to_str())
                && name != "0-category"
            {
                agents.push(name.to_string());
            }
        }
    }
}

/// Collect all agents listed in 0-category.json files.
pub fn collect_listed_agents(content_dir: &Path) -> Vec<String> {
    let mut listed = Vec::new();
    let agent_dir = content_dir.join("agent");
    if !agent_dir.exists() {
        return listed;
    }
    collect_listed_recursive(&agent_dir, &mut listed);
    listed
}

fn collect_listed_recursive(dir: &std::path::Path, listed: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_listed_recursive(&path, listed);
            } else if path.file_name().and_then(|s| s.to_str()) == Some("0-category.json")
                && let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(info) = parse_category_json(&content, &path)
            {
                listed.extend(info.agents);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_agent_file() {
        let content = "---\nname: OpenAgent\ndescription: \"Test\"\nmode: primary\n---";
        let fm = validate_agent_file(content).unwrap();
        assert_eq!(fm.get("name").unwrap().as_str().unwrap(), "OpenAgent");
    }

    #[test]
    fn invalid_mode() {
        let content = "---\nname: Test\ndescription: x\nmode: invalid\n---";
        assert!(matches!(
            validate_agent_file(content),
            Err(AgentError::InvalidMode(_))
        ));
    }

    #[test]
    fn invalid_permission_verb() {
        let content =
            "---\nname: Test\ndescription: x\nmode: primary\npermission:\n  bash: allow all\n---";
        assert!(matches!(
            validate_agent_file(content),
            Err(AgentError::InvalidPermissionVerb(_))
        ));
    }

    #[test]
    fn no_cycle_in_dag() {
        let mut edges = HashMap::new();
        edges.insert("a".into(), vec!["b".into(), "c".into()]);
        edges.insert("b".into(), vec!["c".into()]);
        edges.insert("c".into(), vec![]);
        assert!(validate_delegation_graph(&edges).is_ok());
    }

    #[test]
    fn cycle_detected() {
        let mut edges = HashMap::new();
        edges.insert("a".into(), vec!["b".into()]);
        edges.insert("b".into(), vec!["a".into()]);
        assert!(matches!(
            validate_delegation_graph(&edges),
            Err(AgentError::DelegationCycle(_))
        ));
    }
}
