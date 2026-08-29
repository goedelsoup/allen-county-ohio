//! Reading links, with their tags, off disk.

use crate::Edge;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RawLink {
    target: String,
    relationship: String,
    #[serde(default)]
    claim_tag: Option<String>,
    #[serde(default)]
    source: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawNode {
    class: String,
    #[serde(default)]
    links: Vec<RawLink>,
}

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Yaml {
        path: PathBuf,
        source: serde_yaml::Error,
    },
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "{e}"),
            LoadError::Yaml { path, source } => write!(f, "{}: {source}", path.display()),
        }
    }
}
impl std::error::Error for LoadError {}
impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::Io(e)
    }
}

/// Every link in the corpus, structural ones included — deciding which are structural is
/// [`crate::audit`]'s job, not this module's.
pub fn edges(corpus_dir: &Path) -> Result<Vec<Edge>, LoadError> {
    let mut out = Vec::new();
    if !corpus_dir.is_dir() {
        return Ok(out);
    }
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(corpus_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();

    for dir in dirs {
        let class = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "yml"))
            .filter(|p| !p.to_string_lossy().ends_with(".ont.yml"))
            .collect();
        files.sort();

        for path in files {
            let text = std::fs::read_to_string(&path)?;
            let raw: RawNode = serde_yaml::from_str(&text).map_err(|source| LoadError::Yaml {
                path: path.clone(),
                source,
            })?;
            let node = format!(
                "{class}/{}",
                path.file_name().unwrap_or_default().to_string_lossy()
            );
            for l in raw.links {
                out.push(Edge {
                    node: node.clone(),
                    class: raw.class.clone(),
                    relationship: l.relationship,
                    target: l.target,
                    raw_tag: l.claim_tag,
                    source: l.source,
                });
            }
        }
    }
    Ok(out)
}

/// The filenames in `.yidam/catalog/`, for checking that a `source` names a real entry.
pub fn catalog(catalog_dir: &Path) -> Result<Vec<String>, LoadError> {
    if !catalog_dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut out: Vec<String> = std::fs::read_dir(catalog_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .filter(|n| n != "README.md")
        .collect();
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_corpus_yields_nothing_and_no_error() {
        assert!(edges(Path::new("/nonexistent")).unwrap().is_empty());
        assert!(catalog(Path::new("/nonexistent")).unwrap().is_empty());
    }
}
