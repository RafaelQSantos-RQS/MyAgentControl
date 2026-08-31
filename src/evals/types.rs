//! Eval types: case schema and result types.

use serde::{Deserialize, Serialize};

/// An eval case definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvalCase {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub plan: Option<String>,
    pub expected: Option<String>,
    #[serde(default)]
    pub eval: String,
    #[serde(default)]
    pub context: String,
}

/// An eval result from a completed run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvalResult {
    pub case_name: String,
    pub passed: bool,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    Io(String, String),
    Parse(String, String),
}

impl EvalError {
    pub fn io(path: &str, e: std::io::Error) -> Self {
        EvalError::Io(path.to_string(), e.to_string())
    }
    pub fn parse(path: &str, e: serde_yaml::Error) -> Self {
        EvalError::Parse(path.to_string(), e.to_string())
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::Io(path, e) => write!(f, "EV-506: IO error reading {path}: {e}"),
            EvalError::Parse(path, e) => write!(f, "EV-501: parse error in {path}: {e}"),
        }
    }
}

impl std::error::Error for EvalError {}
