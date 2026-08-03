//! Registry data model (registry-spec §3): serde types for the OAC
//! `registry.json` v2 format (vendored at `content/registry.json`).
//!
//! - [`Registry`]: top-level document (version, profiles, components).
//! - [`Profile`]: a named component set (`essential`, `developer`, ...).
//! - [`Component`]: a single installable unit (agent, skill, context, ...).
//! - [`Category`]: the eight component buckets, in OAC menu order.
//!
//! Unknown JSON keys are ignored by serde (forward-compatible), so adding
//! fields to the registry never breaks the parser. Profiles preserve their
//! file order via [`IndexMap`], which the TUI uses for menus.

use indexmap::IndexMap;
use serde::Deserialize;

/// Top-level `registry.json` document.
#[derive(Debug, Clone, Deserialize)]
pub struct Registry {
    pub version: Option<String>,
    /// Profiles keyed by id, in file order (menu display order).
    pub profiles: IndexMap<String, Profile>,
    /// Component buckets, keyed by category name.
    pub components: Components,
}

/// A named install profile (registry-spec §3.2).
#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Component ids in `type:id` form (e.g. `agent:openagent`).
    #[serde(default)]
    pub components: Vec<String>,
}

/// A single installable component.
#[derive(Debug, Clone, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Transitive dependency ids in `type:id` form.
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Additional files for multi-file components (skills).
    #[serde(default)]
    pub files: Vec<String>,
}

/// Component buckets keyed by category (registry-spec §3.1).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Components {
    #[serde(default)]
    pub agents: Vec<Component>,
    #[serde(default)]
    pub subagents: Vec<Component>,
    #[serde(default)]
    pub commands: Vec<Component>,
    #[serde(default)]
    pub skills: Vec<Component>,
    #[serde(default)]
    pub contexts: Vec<Component>,
    #[serde(default)]
    pub tools: Vec<Component>,
    #[serde(default)]
    pub plugins: Vec<Component>,
    #[serde(default)]
    pub config: Vec<Component>,
}

/// The eight component buckets, in OAC `install.sh` menu order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Agents,
    Subagents,
    Commands,
    Tools,
    Plugins,
    Skills,
    Contexts,
    Config,
}

impl Category {
    /// All buckets in display order (used by the TUI menus).
    pub const ALL: [Category; 8] = [
        Category::Agents,
        Category::Subagents,
        Category::Commands,
        Category::Tools,
        Category::Plugins,
        Category::Skills,
        Category::Contexts,
        Category::Config,
    ];

    /// (plural display label, singular `type:` prefix) per bucket — one
    /// source of truth instead of a repeated 8-arm match.
    const fn label_and_key(self) -> (&'static str, &'static str) {
        match self {
            Category::Agents => ("Agents", "agent"),
            Category::Subagents => ("Subagents", "subagent"),
            Category::Commands => ("Commands", "command"),
            Category::Tools => ("Tools", "tool"),
            Category::Plugins => ("Plugins", "plugin"),
            Category::Skills => ("Skills", "skill"),
            Category::Contexts => ("Contexts", "context"),
            Category::Config => ("Config", "config"),
        }
    }

    /// Plural display label (e.g. `Agents`).
    pub fn label(self) -> &'static str {
        self.label_and_key().0
    }

    /// Singular `type:` prefix used in `type:id` strings (e.g. `agent`).
    pub fn type_key(self) -> &'static str {
        self.label_and_key().1
    }

    /// The components in this bucket.
    pub fn components(self, registry: &Registry) -> &[Component] {
        let c = &registry.components;
        match self {
            Category::Agents => &c.agents,
            Category::Subagents => &c.subagents,
            Category::Commands => &c.commands,
            Category::Tools => &c.tools,
            Category::Plugins => &c.plugins,
            Category::Skills => &c.skills,
            Category::Contexts => &c.contexts,
            Category::Config => &c.config,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal registry exercising every field the TUI reads.
    const FIXTURE: &str = r#"{
      "version": "2.0.0",
      "schema_version": "2.0.0",
      "profiles": {
        "essential": {
          "name": "Essential (Minimal)",
          "description": "Minimal starter kit",
          "components": ["agent:openagent", "subagent:task-manager"]
        },
        "developer": {
          "name": "Developer",
          "components": []
        }
      },
      "components": {
        "agents": [{
          "id": "openagent",
          "name": "OpenAgent",
          "path": ".opencode/agent/core/openagent.md",
          "description": "Universal agent",
          "dependencies": [],
          "files": []
        }],
        "contexts": [],
        "skills": [{
          "id": "task-management",
          "name": "Task Management",
          "path": ".opencode/skills/task-management/SKILL.md",
          "files": [".opencode/skills/task-management/SKILL.md"]
        }]
      }
    }"#;

    #[test]
    fn parses_registry_version() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        assert_eq!(reg.version.as_deref(), Some("2.0.0"));
    }

    #[test]
    fn parses_profile_count() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        assert_eq!(reg.profiles.len(), 2);
    }

    #[test]
    fn parses_agents_bucket() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        assert_eq!(reg.components.agents.len(), 1);
    }

    #[test]
    fn profiles_preserve_file_order() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        let keys: Vec<&str> = reg.profiles.keys().map(String::as_str).collect();
        assert_eq!(keys, vec!["essential", "developer"]);
    }

    #[test]
    fn component_dependencies_default_empty() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        let agent = &reg.components.agents[0];
        assert!(agent.dependencies.is_empty());
    }

    #[test]
    fn component_files_default_empty() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        let agent = &reg.components.agents[0];
        assert!(agent.files.is_empty());
    }

    #[test]
    fn skill_component_files_present() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        let skill = &reg.components.skills[0];
        assert_eq!(skill.files.len(), 1);
    }

    #[test]
    fn category_components_returns_agents() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        assert_eq!(Category::Agents.components(&reg).len(), 1);
    }

    #[test]
    fn category_components_returns_empty_contexts() {
        let reg: Registry = serde_json::from_str(FIXTURE).expect("fixture parses");
        assert_eq!(Category::Contexts.components(&reg).len(), 0);
    }

    #[test]
    fn category_all_has_eight_buckets() {
        assert_eq!(Category::ALL.len(), 8);
    }

    #[test]
    fn category_type_key_is_singular() {
        assert_eq!(Category::Subagents.type_key(), "subagent");
    }

    #[test]
    fn category_label_is_plural() {
        assert_eq!(Category::Config.label(), "Config");
    }
}
