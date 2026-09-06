//! Reading `.yidam/corpus/` into a [`Graph`].
//!
//! All of the I/O and none of the deciding, kept apart for the reason `succession`,
//! `covering` and `chronology` all keep them apart: the interesting logic should be testable
//! without a corpus on disk.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One outgoing edge, with the kind of claim it makes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// Resolved to `class/name.yml`, as the graph addresses nodes.
    pub target: String,
    pub relationship: String,
    /// `verified` or `inference`. `None` on a structural edge, which carries none by rule —
    /// `crates/provenance` is what holds every other edge to having one.
    pub claim_tag: Option<String>,
}

/// One corpus node, reduced to what a placement query needs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Node {
    /// `place/lima.yml` — class directory and filename, as links resolve to.
    pub id: String,
    pub class: String,
    pub label: String,
    pub properties: BTreeMap<String, String>,
    pub links: Vec<Link>,
}

/// The corpus as an addressable graph.
#[derive(Debug, Clone, Default)]
pub struct Graph {
    nodes: BTreeMap<String, Node>,
}

impl Graph {
    pub fn insert(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn get(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }
}

#[derive(Debug, Deserialize)]
struct RawLink {
    target: String,
    relationship: String,
    #[serde(default)]
    claim_tag: Option<String>,
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
/// a bare `sibling.yml` for a node in the same class directory.
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
/// Class contracts (`*.ont.yml`) are skipped, and so are the `instance-of` edges pointing at
/// them: a class contract is not a place.
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
            g.insert(Node {
                id: format!(
                    "{class_dir}/{}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ),
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
                    .filter(|l| l.relationship != "instance-of")
                    .map(|l| Link {
                        target: resolve(&class_dir, &l.target),
                        relationship: l.relationship.clone(),
                        claim_tag: l.claim_tag.clone(),
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
    fn a_missing_corpus_is_an_empty_one_rather_than_an_error() {
        assert!(graph(Path::new("does/not/exist")).unwrap().is_empty());
    }

    #[test]
    fn resolves_both_link_shapes_the_corpus_writes() {
        assert_eq!(
            resolve("place", "../jurisdiction/x.yml"),
            "jurisdiction/x.yml"
        );
        assert_eq!(resolve("place", "sibling.yml"), "place/sibling.yml");
        assert_eq!(resolve("place", "./sibling.yml"), "place/sibling.yml");
    }
}
