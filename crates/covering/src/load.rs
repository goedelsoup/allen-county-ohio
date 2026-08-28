//! Reading `.yidam/corpus/` into a [`Graph`].
//!
//! All of the I/O and none of the deciding, kept apart for the same reason `succession` keeps
//! them apart: the interesting logic should be testable without a corpus on disk.

use crate::{Graph, Link, Node};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RawLink {
    target: String,
    relationship: String,
}

#[derive(Debug, Deserialize)]
struct RawNode {
    class: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    properties: BTreeMap<String, serde_yaml::Value>,
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

/// Render a YAML scalar as the string the corpus wrote.
///
/// Years in this corpus are quoted, but a bare `1820` is legal YAML and arrives as a number.
/// Both are accepted here; the `property-type` lint is what argues about which the file should
/// contain, and a calculator that refused to read one would be enforcing a second opinion.
fn scalar(v: &serde_yaml::Value) -> Option<String> {
    match v {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        serde_yaml::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// Resolve a link target against the class directory of the node that wrote it.
///
/// Corpus links are relative paths: `../jurisdiction/city-of-lima.yml` from a `place` node, and
/// a bare `ohio-house-district-4-2020.yml` for a sibling in the same class directory.
fn resolve(from_class_dir: &str, target: &str) -> String {
    let t = target.trim_start_matches("./");
    match t.strip_prefix("../") {
        Some(rest) => rest.to_string(),
        None if t.contains('/') => t.to_string(),
        None => format!("{from_class_dir}/{t}"),
    }
}

/// Every node under `corpus_dir`, in every class directory.
///
/// Class contracts (`*.ont.yml`) are skipped: they describe the classes rather than instancing
/// them, and an `instance-of` edge pointing at one is not a coverage edge.
pub fn graph(corpus_dir: &Path) -> Result<Graph, LoadError> {
    let mut g = Graph::default();
    if !corpus_dir.is_dir() {
        return Ok(g);
    }
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(corpus_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();

    for dir in dirs {
        let class_dir = dir
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
            let id = format!(
                "{class_dir}/{}",
                path.file_name().unwrap_or_default().to_string_lossy()
            );
            g.insert(Node {
                id,
                class: raw.class,
                label: raw.label,
                properties: raw
                    .properties
                    .iter()
                    .filter_map(|(k, v)| scalar(v).map(|s| (k.clone(), s)))
                    .collect(),
                links: raw
                    .links
                    .iter()
                    .map(|l| Link {
                        target: resolve(&class_dir, &l.target),
                        relationship: l.relationship.clone(),
                    })
                    .collect(),
            });
        }
    }
    Ok(g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_targets_resolve_to_graph_ids() {
        assert_eq!(
            resolve("place", "../jurisdiction/city-of-lima.yml"),
            "jurisdiction/city-of-lima.yml"
        );
        assert_eq!(
            resolve("division", "ohio-house-district-4-2020.yml"),
            "division/ohio-house-district-4-2020.yml"
        );
        assert_eq!(
            resolve("place", "./allen-county.yml"),
            "place/allen-county.yml"
        );
    }

    #[test]
    fn a_year_reads_whether_or_not_the_corpus_quoted_it() {
        assert_eq!(
            scalar(&serde_yaml::Value::from(1820)).as_deref(),
            Some("1820")
        );
        assert_eq!(
            scalar(&serde_yaml::Value::from("1820")).as_deref(),
            Some("1820")
        );
    }

    #[test]
    fn a_missing_corpus_is_an_empty_graph_and_not_an_error() {
        let g = graph(Path::new("/nonexistent/corpus")).unwrap();
        assert!(g.is_empty());
    }
}
