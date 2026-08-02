//! Integration test: `serde-saphyr` typed YAML parsing (cli-spec §5 / decision D11).
//!
//! Verifies the YAML stack chosen in Fase 0:
//! - typed-only deserialization (no `Value` DOM)
//! - permission maps as `HashMap<String, HashMap<String, Permission>>`
//! - panic-free error reporting on malformed input

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Permission verbs used by the reference OAC `.opencode/agent/` frontmatter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Permission {
    Allow,
    Ask,
    Deny,
}

/// Minimal frontmatter-shaped struct exercising typed YAML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AgentFrontmatter {
    name: String,
    mode: String,
    category: String,
    permissions: HashMap<String, HashMap<String, Permission>>,
}

const SAMPLE_YAML: &str = r#"
name: OpenAgent
mode: primary
category: core
permissions:
  tools:
    read: allow
    write: ask
    delete: deny
  context:
    load: allow
    save: allow
"#;

#[test]
fn parses_typed_yaml_permission_map() {
    let parsed: AgentFrontmatter =
        serde_saphyr::from_str(SAMPLE_YAML).expect("sample YAML must parse");

    assert_eq!(parsed.name, "OpenAgent");
    assert_eq!(parsed.mode, "primary");
    assert_eq!(parsed.category, "core");

    let tools = parsed.permissions.get("tools").expect("tools map present");
    assert_eq!(tools.get("read"), Some(&Permission::Allow));
    assert_eq!(tools.get("write"), Some(&Permission::Ask));
    assert_eq!(tools.get("delete"), Some(&Permission::Deny));

    let context = parsed
        .permissions
        .get("context")
        .expect("context map present");
    assert_eq!(context.get("load"), Some(&Permission::Allow));
    assert_eq!(context.get("save"), Some(&Permission::Allow));
}

#[test]
fn roundtrip_serialize_deserialize() {
    let parsed: AgentFrontmatter = serde_saphyr::from_str(SAMPLE_YAML).expect("parse");
    let serialized = serde_saphyr::to_string(&parsed).expect("serialize");
    let reparsed: AgentFrontmatter = serde_saphyr::from_str(&serialized).expect("re-parse");
    assert_eq!(parsed, reparsed);
}

#[test]
fn malformed_yaml_errors_without_panicking() {
    let bad = "permissions: [unclosed";
    let result: Result<AgentFrontmatter, _> = serde_saphyr::from_str(bad);
    assert!(
        result.is_err(),
        "malformed YAML must return an error, not panic"
    );
}

#[test]
fn unknown_permission_verb_is_rejected() {
    let bad = r#"
name: OpenAgent
mode: primary
category: core
permissions:
  tools:
    read: allow_all
"#;
    let result: Result<AgentFrontmatter, _> = serde_saphyr::from_str(bad);
    assert!(
        result.is_err(),
        "unknown permission verb must be rejected (typed enum)"
    );
}
