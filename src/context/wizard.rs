use std::fs;
use std::io::{self, IsTerminal};
use std::path::{Path, PathBuf};

use crate::context::frontmatter;

const QUESTIONS: &[(&str, &str)] = &[
    ("project_name", "What is the project name?"),
    (
        "language",
        "What primary language/framework does the project use?",
    ),
    (
        "domain",
        "What is the business domain? (e.g. fintech, e-commerce, health)",
    ),
    (
        "architecture",
        "What architecture pattern is used? (e.g. monolith, microservices, serverless)",
    ),
    (
        "conventions",
        "Any key coding conventions? (e.g. naming, error handling, testing)",
    ),
    (
        "tools",
        "What key tools/services does the project use? (e.g. PostgreSQL, Redis, Docker)",
    ),
];

#[derive(Debug, Clone)]
pub struct WizardAnswers {
    pub project_name: String,
    pub language: String,
    pub domain: String,
    pub architecture: String,
    pub conventions: String,
    pub tools: String,
}

#[derive(Debug, Clone)]
pub enum WizardError {
    NotInteractive,
    Io(String),
}

impl From<io::Error> for WizardError {
    fn from(e: io::Error) -> Self {
        WizardError::Io(e.to_string())
    }
}

impl std::fmt::Display for WizardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WizardError::NotInteractive => {
                write!(f, "interactive wizard requires a terminal; run from a TTY")
            }
            WizardError::Io(e) => write!(f, "wizard I/O error: {e}"),
        }
    }
}

impl std::error::Error for WizardError {}

/// Run the interactive wizard, collecting answers via TTY prompts.
pub fn run_wizard() -> Result<WizardAnswers, WizardError> {
    if !io::stdin().is_terminal() {
        return Err(WizardError::NotInteractive);
    }

    let mut answers = Vec::new();
    for (key, question) in QUESTIONS {
        print!("{question} ");
        io::Write::flush(&mut io::stdout())?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let value = input.trim().to_string();
        if value.is_empty() {
            return Err(WizardError::NotInteractive);
        }
        answers.push((*key, value));
    }

    Ok(WizardAnswers {
        project_name: answers[0].1.clone(),
        language: answers[1].1.clone(),
        domain: answers[2].1.clone(),
        architecture: answers[3].1.clone(),
        conventions: answers[4].1.clone(),
        tools: answers[5].1.clone(),
    })
}

/// Generate the context file content from wizard answers.
pub fn generate_file(answers: &WizardAnswers, version: &str) -> String {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    format!(
        "<!-- Context: project-intelligence/technical-domain | Priority: critical | Version: {version} | Updated: {today} -->\n\
         \n\
         # Technical Domain — {project}\n\
         \n\
         ## Overview\n\
         {project} is a {domain} project built with {language}.\n\
         Architecture: {architecture}.\n\
         \n\
         ## Key Technologies\n\
         - Language/Framework: {language}\n\
         - Architecture: {architecture}\n\
         - Tools: {tools}\n\
         \n\
         ## Conventions\n\
         {conventions}\n\
         \n\
         ## References\n\
         - See also: `navigation.md`\n",
        project = answers.project_name,
        domain = answers.domain,
        language = answers.language,
        architecture = answers.architecture,
        tools = answers.tools,
        conventions = answers.conventions,
        version = version,
        today = today,
    )
}

/// Write the generated file to disk and update navigation.md.
pub fn write_context_file(
    tree_root: &Path,
    answers: &WizardAnswers,
    version: &str,
) -> Result<PathBuf, WizardError> {
    let content = generate_file(answers, version);
    let target_dir = tree_root.join("project-intelligence");
    fs::create_dir_all(&target_dir)?;

    let file_path = target_dir.join("technical-domain.md");
    fs::write(&file_path, &content)?;

    // Update navigation.md
    let nav_path = tree_root.join("navigation.md");
    if nav_path.exists() {
        let mut nav = fs::read_to_string(&nav_path)?;
        let entry = format!(
            "| {} | Technical Domain | `project-intelligence/technical-domain.md` |\n",
            answers.project_name
        );
        if !nav.contains("project-intelligence/technical-domain.md") {
            // Append to Deep Dives section if it exists, otherwise to end
            if let Some(pos) = nav.find("## Deep Dives") {
                let after_header = nav[pos..].find('\n').map(|p| pos + p + 1).unwrap_or(pos);
                nav.insert_str(after_header, &entry);
            } else {
                nav.push_str("\n## Deep Dives\n\n");
                nav.push_str("| Project | Topic | File |\n");
                nav.push_str("|---------|-------|------|\n");
                nav.push_str(&entry);
            }
            fs::write(&nav_path, &nav)?;
        }
    }

    Ok(file_path)
}

/// Update an existing context file with new version and date.
pub fn update_context_file(tree_root: &Path, bump_major: bool) -> Result<PathBuf, WizardError> {
    let file_path = tree_root.join("project-intelligence/technical-domain.md");
    if !file_path.exists() {
        return Err(WizardError::Io(
            "technical-domain.md not found; run wizard first".to_string(),
        ));
    }

    let content = fs::read_to_string(&file_path)?;
    let fm =
        frontmatter::parse_frontmatter(&content).map_err(|e| WizardError::Io(e.to_string()))?;

    let old_version = &fm.version;
    let new_version = if bump_major {
        let major: u32 = old_version
            .split('.')
            .next()
            .unwrap_or("1")
            .parse()
            .unwrap_or(1);
        format!("{}.0", major + 1)
    } else {
        let parts: Vec<&str> = old_version.split('.').collect();
        let major = parts
            .first()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        let minor = parts
            .get(1)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        format!("{}.{}", major, minor + 1)
    };

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let new_line = format!(
        "<!-- Context: {} | Priority: {} | Version: {} | Updated: {} -->",
        fm.category,
        fm.priority.as_str(),
        new_version,
        today,
    );

    let mut lines: Vec<&str> = content.lines().collect();
    if let Some(first) = lines.first_mut() {
        *first = &new_line;
    }
    let updated = lines.join("\n");
    fs::write(&file_path, updated)?;

    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_file_content() {
        let answers = WizardAnswers {
            project_name: "MyApp".to_string(),
            language: "Rust".to_string(),
            domain: "fintech".to_string(),
            architecture: "monolith".to_string(),
            conventions: "use thiserror".to_string(),
            tools: "PostgreSQL, Docker".to_string(),
        };
        let content = generate_file(&answers, "1.0");
        assert!(content.contains("Context: project-intelligence/technical-domain"));
        assert!(content.contains("Priority: critical"));
        assert!(content.contains("Version: 1.0"));
        assert!(content.contains("MyApp"));
        assert!(content.contains("Rust"));
        assert!(content.contains("fintech"));
    }
}
