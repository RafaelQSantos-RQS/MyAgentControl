use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const MANIFEST_FILE: &str = "manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CachedDoc {
    pub id: String,
    pub sha256: String,
    pub version: u32,
    pub source_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheManifest {
    pub docs: HashMap<String, CachedDoc>,
}

impl CacheManifest {
    pub fn empty() -> Self {
        Self {
            docs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheError {
    Io(String),
    ManifestParse(String),
    DocNotFound(String),
}

impl From<std::io::Error> for CacheError {
    fn from(e: std::io::Error) -> Self {
        CacheError::Io(e.to_string())
    }
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::Io(e) => write!(f, "cache I/O error: {e}"),
            CacheError::ManifestParse(e) => write!(f, "cache manifest parse error: {e}"),
            CacheError::DocNotFound(id) => write!(f, "cached doc not found: {id}"),
        }
    }
}

impl std::error::Error for CacheError {}

fn manifest_path(cache_dir: &Path) -> PathBuf {
    cache_dir.join(MANIFEST_FILE)
}

fn doc_path(cache_dir: &Path, doc_id: &str) -> PathBuf {
    cache_dir.join(format!("{doc_id}.md"))
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{b:02x}")).collect()
}

/// Load the manifest from disk, or return an empty one.
pub fn load_manifest(cache_dir: &Path) -> Result<CacheManifest, CacheError> {
    let path = manifest_path(cache_dir);
    if !path.exists() {
        return Ok(CacheManifest::empty());
    }
    let data = fs::read_to_string(&path).map_err(|e| CacheError::Io(e.to_string()))?;
    serde_json::from_str(&data).map_err(|e| CacheError::ManifestParse(e.to_string()))
}

/// Save the manifest to disk.
pub fn save_manifest(cache_dir: &Path, manifest: &CacheManifest) -> Result<(), CacheError> {
    fs::create_dir_all(cache_dir).map_err(|e| CacheError::Io(e.to_string()))?;
    let data = serde_json::to_string_pretty(manifest)
        .map_err(|e| CacheError::ManifestParse(e.to_string()))?;
    fs::write(manifest_path(cache_dir), data).map_err(|e| CacheError::Io(e.to_string()))
}

/// Add a document to the cache.
pub fn add(
    cache_dir: &Path,
    doc_id: &str,
    content: &[u8],
    source_url: Option<String>,
) -> Result<CachedDoc, CacheError> {
    let mut manifest = load_manifest(cache_dir)?;
    let hash = sha256_hex(content);
    let version = manifest
        .docs
        .get(doc_id)
        .map(|d| d.version + 1)
        .unwrap_or(1);

    // Write file
    fs::create_dir_all(cache_dir).map_err(|e| CacheError::Io(e.to_string()))?;
    fs::write(doc_path(cache_dir, doc_id), content).map_err(|e| CacheError::Io(e.to_string()))?;

    let doc = CachedDoc {
        id: doc_id.to_string(),
        sha256: hash,
        version,
        source_url,
    };
    manifest.docs.insert(doc_id.to_string(), doc.clone());
    save_manifest(cache_dir, &manifest)?;
    Ok(doc)
}

/// Update a cached document (alias for add with version bump).
pub fn update(
    cache_dir: &Path,
    doc_id: &str,
    content: &[u8],
    source_url: Option<String>,
) -> Result<CachedDoc, CacheError> {
    add(cache_dir, doc_id, content, source_url)
}

/// List all cached documents.
pub fn list(cache_dir: &Path) -> Result<Vec<CachedDoc>, CacheError> {
    let manifest = load_manifest(cache_dir)?;
    Ok(manifest.docs.values().cloned().collect())
}

/// Remove a cached document.
pub fn remove(cache_dir: &Path, doc_id: &str) -> Result<(), CacheError> {
    let mut manifest = load_manifest(cache_dir)?;
    manifest.docs.remove(doc_id);
    let path = doc_path(cache_dir, doc_id);
    if path.exists() {
        fs::remove_file(path).map_err(|e| CacheError::Io(e.to_string()))?;
    }
    save_manifest(cache_dir, &manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_cache() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "mac_test_cache_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn add_and_list() {
        let cache = tmp_cache();
        let doc = add(&cache, "test-doc", b"hello world", None).unwrap();
        assert_eq!(doc.id, "test-doc");
        assert_eq!(doc.version, 1);
        assert!(!doc.sha256.is_empty());

        let docs = list(&cache).unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].id, "test-doc");

        let _ = fs::remove_dir_all(&cache);
    }

    #[test]
    fn update_bumps_version() {
        let cache = tmp_cache();
        add(&cache, "doc1", b"v1", None).unwrap();
        let updated = update(&cache, "doc1", b"v2", None).unwrap();
        assert_eq!(updated.version, 2);

        let _ = fs::remove_dir_all(&cache);
    }

    #[test]
    fn remove_doc() {
        let cache = tmp_cache();
        add(&cache, "doc1", b"content", None).unwrap();
        remove(&cache, "doc1").unwrap();
        let docs = list(&cache).unwrap();
        assert!(docs.is_empty());

        let _ = fs::remove_dir_all(&cache);
    }

    #[test]
    fn load_empty_manifest() {
        let cache = tmp_cache();
        let manifest = load_manifest(&cache).unwrap();
        assert!(manifest.docs.is_empty());

        let _ = fs::remove_dir_all(&cache);
    }
}
