//! Walk test for skill inventory consistency (SK-705).

use std::path::Path;

use myagentcontrol::validation::skills::{validate_structure, walk_skills};

#[test]
fn skill_inventory_and_structure() {
    let content_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let skills = walk_skills(&content_dir);

    assert!(!skills.is_empty(), "no skills found in content/skills/");

    for skill in &skills {
        let skill_dir = content_dir.join("skills").join(skill);

        // Validate structure (SK-706)
        validate_structure(&skill_dir).unwrap_or_else(|e| {
            panic!("skill \"{skill}\" structure invalid: {e}");
        });

        // Validate SKILL.md frontmatter (SK-701)
        let skill_md = skill_dir.join("SKILL.md");
        let content = std::fs::read_to_string(&skill_md).unwrap();
        myagentcontrol::validation::skills::validate_skill_file(&content)
            .unwrap_or_else(|e| panic!("skill \"{skill}\" SKILL.md invalid: {e}"));

        // Validate router.sh (SK-702) — only if present
        let router = skill_dir.join("router.sh");
        if router.exists() {
            myagentcontrol::validation::skills::validate_router(&skill_dir)
                .unwrap_or_else(|e| panic!("skill \"{skill}\" router.sh invalid: {e}"));
        }
    }
}
