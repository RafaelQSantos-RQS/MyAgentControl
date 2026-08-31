//! Eval case schema, parser, results types, and dashboard generator.
//!
//! Handles `openspec/evals/` cases, `openspec/evals/results/` JSON,
//! and generates HTML dashboards.

pub mod types;

use std::path::Path;

pub use types::*;

/// Parse all eval cases from the evals directory.
pub fn parse_cases(evals_dir: &Path) -> Vec<Result<EvalCase, EvalError>> {
    let mut results = Vec::new();
    if !evals_dir.exists() {
        return results;
    }
    for entry in std::fs::read_dir(evals_dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_dir() || path.extension().and_then(|e| e.to_str()) != Some("yaml") {
            continue;
        }
        let case = parse_case(&path);
        results.push(case);
    }
    results
}

/// Parse a single eval case YAML file.
pub fn parse_case(path: &Path) -> Result<EvalCase, EvalError> {
    let content =
        std::fs::read_to_string(path).map_err(|e| EvalError::io(&path.display().to_string(), e))?;
    let case: EvalCase = serde_yaml::from_str(&content)
        .map_err(|e| EvalError::parse(&path.display().to_string(), e))?;
    Ok(case)
}

/// Load eval results from the results directory.
pub fn load_results(results_dir: &Path) -> Vec<EvalResult> {
    let mut results = Vec::new();
    if !results_dir.exists() {
        return results;
    }
    for entry in std::fs::read_dir(results_dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if let Ok(content) = std::fs::read_to_string(&path)
            && let Ok(result) = serde_json::from_str::<EvalResult>(&content)
        {
            results.push(result);
        }
    }
    results
}

/// Generate an HTML dashboard from eval cases and results.
pub fn generate_dashboard(cases: &[EvalCase], results: &[EvalResult]) -> String {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Eval Dashboard</title>
<style>
body { font-family: sans-serif; margin: 2rem; }
table { border-collapse: collapse; width: 100%; }
th, td { border: 1px solid #ccc; padding: 0.5rem 1rem; text-align: left; }
th { background: #f5f5f5; }
.pass { color: green; }
.fail { color: red; }
</style>
</head>
<body>
<h1>Evaluation Dashboard</h1>
<table>
<tr><th>Case</th><th>Description</th><th>Status</th><th>Duration</th></tr>
"#,
    );

    let results_map: std::collections::HashMap<String, &EvalResult> =
        results.iter().map(|r| (r.case_name.clone(), r)).collect();

    for case in cases {
        let status = if let Some(result) = results_map.get(&case.name) {
            match result.passed {
                true => r#"<span class="pass">PASS</span>"#,
                false => r#"<span class="fail">FAIL</span>"#,
            }
        } else {
            "<em>pending</em>"
        };
        let duration = results_map
            .get(&case.name)
            .and_then(|r| r.duration_ms.map(|ms| format!("{ms}ms")))
            .unwrap_or_default();

        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            html_escape(&case.name),
            html_escape(&case.description),
            status,
            html_escape(&duration),
        ));
    }

    html.push_str("</table>\n</body>\n</html>");
    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Walk evals directory and return case file paths.
pub fn walk_cases(evals_dir: &Path) -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    if !evals_dir.exists() {
        return paths;
    }
    for entry in std::fs::read_dir(evals_dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_dir() || path.extension().and_then(|e| e.to_str()) != Some("yaml") {
            continue;
        }
        paths.push(path);
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_case() {
        let dir = std::env::temp_dir().join("mac_test_eval_case");
        let _ = std::fs::create_dir_all(&dir);
        let yaml = "name: test-case\ndescription: A test\ntags: [basic]\neval: check it works\n";
        let path = dir.join("test-case.yaml");
        std::fs::write(&path, yaml).unwrap();
        let case = parse_case(&path).unwrap();
        assert_eq!(case.name, "test-case");
        assert_eq!(case.description, "A test");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn dashboard_html() {
        let cases = vec![EvalCase {
            name: "foo".into(),
            description: "bar".into(),
            tags: vec![],
            plan: None,
            expected: None,
            eval: String::new(),
            context: String::new(),
        }];
        let html = generate_dashboard(&cases, &[]);
        assert!(html.contains("foo"));
        assert!(html.contains("<table>"));
    }
}
