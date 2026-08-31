//! Reading `.yidam/corpus/` into [`Node`]s.
//!
//! All of the I/O and none of the deciding, kept apart for the same reason the other crates
//! here keep them apart: the publication rules should be testable without a corpus on disk.

use crate::claim::{blocks, Block};
use crate::tier::Tier;
use serde::Deserialize;
use std::collections::BTreeMap;
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
struct RawFoundational {
    ontology: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize)]
struct RawProperty {
    name: String,
    #[serde(default)]
    required: bool,
}

#[derive(Debug, Deserialize)]
struct RawOntology {
    class: String,
    #[serde(default)]
    label: String,
    foundational_type: RawFoundational,
    #[serde(default)]
    edge_policy: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    properties: Vec<RawProperty>,
}

#[derive(Debug, Deserialize)]
struct RawNode {
    class: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    properties: BTreeMap<String, serde_yaml::Value>,
    #[serde(default)]
    links: Vec<RawLink>,
}

/// An edge, with the tag it carries.
#[derive(Debug, Clone)]
pub struct Link {
    /// The link as written — a relative path from the writing node's class directory.
    pub target: String,
    /// `class/name.yml`, resolved against the writing node.
    pub resolved: String,
    pub relationship: String,
    pub claim_tag: Option<Tier>,
    /// The catalog entry supporting this relationship, as `catalog/<name>.md`.
    pub source: Option<String>,
}

/// A corpus node, with its prose already cut into claims.
#[derive(Debug, Clone)]
pub struct Node {
    /// `class/name.yml` — the node's identity throughout the feeds.
    pub id: String,
    pub class: String,
    pub name: String,
    pub label: String,
    pub properties: BTreeMap<String, String>,
    pub blocks: Vec<Block>,
    pub links: Vec<Link>,
}

impl Node {
    /// The value of a property, as the corpus wrote it.
    pub fn property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(String::as_str)
    }
}

/// What a class declares about itself.
///
/// Every field is read from `<class>.ont.yml` and none is computed. The corpus states its
/// own ontology there, and this is the only place that reads it — a second statement of
/// what a tenure is would be a second thing to keep true.
#[derive(Debug, Clone)]
pub struct Class {
    /// The class name, which is also its corpus directory: `natural-feature`.
    pub class: String,
    /// The class's display name: `Natural Feature`.
    pub label: String,
    /// The ontology the foundational type is drawn from — `ufo` throughout this corpus.
    pub ontology: String,
    /// `kind`, `role`, `relator`, `event`, `quality` or `situation`.
    pub foundational_type: String,
    pub edge_policy: String,
    /// The class's own account of why it exists, as one paragraph.
    pub description: String,
    /// Properties the class declares `required: true`, in the order it declares them.
    pub required: Vec<String>,
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
/// The same accommodation the other crates make: a year is usually quoted here, but a bare
/// `1820` is legal YAML and arrives as a number. Nested values — a `properties` entry that is
/// itself a map — are not scalars and are dropped, because a feed is not the place to invent
/// a flattening the corpus did not write.
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
/// Corpus links are relative: `../jurisdiction/city-of-lima.yml` from a `place` node, and a
/// bare `sibling.yml` within one class directory.
fn resolve(class_dir: &str, target: &str) -> String {
    match target.strip_prefix("../") {
        Some(rest) => rest.to_string(),
        None => format!("{class_dir}/{target}"),
    }
}

/// Load every class declaration in `corpus_dir`.
///
/// These sit beside the class directories as `<class>.ont.yml`, and they are the corpus's
/// statement of its own ontology: what kind of thing each class is, and which of its
/// properties a node of that class may not omit. The site renders it rather than restating
/// it, which is the same rule that keeps `web/` off the node files.
pub fn classes(corpus_dir: &Path) -> Result<Vec<Class>, LoadError> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(corpus_dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(".ont.yml"))
        .collect();
    paths.sort();

    let mut classes = Vec::new();
    for path in paths {
        let text = std::fs::read_to_string(&path)?;
        let raw: RawOntology = serde_yaml::from_str(&text).map_err(|source| LoadError::Yaml {
            path: path.clone(),
            source,
        })?;

        classes.push(Class {
            class: raw.class,
            label: raw.label,
            ontology: raw.foundational_type.ontology,
            foundational_type: raw.foundational_type.kind,
            edge_policy: raw.edge_policy,
            // The `description` is a folded block wrapped at the corpus's line width. It is
            // one paragraph in every class file, so collapsing the wrapping is the whole of
            // the transformation — the same thing `claim::normalize` does to node prose.
            description: crate::claim::normalize(&raw.description),
            required: raw
                .properties
                .into_iter()
                .filter(|p| p.required)
                .map(|p| p.name)
                .collect(),
        });
    }

    Ok(classes)
}

/// Load every instance node under `corpus_dir`.
///
/// Ontology files (`*.ont.yml`) and the prose that sits beside nodes (`README.md`,
/// `ACTIONS.md`) are not instances and are skipped.
pub fn corpus(corpus_dir: &Path) -> Result<Vec<Node>, LoadError> {
    let mut nodes = Vec::new();

    let mut class_dirs: Vec<PathBuf> = std::fs::read_dir(corpus_dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    class_dirs.sort();

    for dir in class_dirs {
        let class_dir = dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| {
                p.extension().is_some_and(|e| e == "yml")
                    && !p.to_string_lossy().ends_with(".ont.yml")
            })
            .collect();
        files.sort();

        for path in files {
            let text = std::fs::read_to_string(&path)?;
            let raw: RawNode = serde_yaml::from_str(&text).map_err(|source| LoadError::Yaml {
                path: path.clone(),
                source,
            })?;

            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();

            nodes.push(Node {
                id: format!("{class_dir}/{name}.yml"),
                class: raw.class,
                label: raw.label,
                properties: raw
                    .properties
                    .iter()
                    .filter_map(|(k, v)| scalar(v).map(|s| (k.clone(), s)))
                    .collect(),
                blocks: blocks(&raw.description),
                links: raw
                    .links
                    .into_iter()
                    .map(|l| Link {
                        resolved: resolve(&class_dir, &l.target),
                        claim_tag: l.claim_tag.as_deref().and_then(Tier::parse),
                        source: l.source.as_deref().map(|s| {
                            s.trim_start_matches("../")
                                .trim_start_matches("../")
                                .to_string()
                        }),
                        target: l.target,
                        relationship: l.relationship,
                    })
                    .collect(),
                name,
            });
        }
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_parent_relative_link_resolves_to_class_and_file() {
        assert_eq!(
            resolve("measure", "../place/allen-county.yml"),
            "place/allen-county.yml"
        );
    }

    #[test]
    fn a_sibling_link_resolves_within_the_writing_class() {
        assert_eq!(
            resolve("measure", "allen-county-population-2010.yml"),
            "measure/allen-county-population-2010.yml"
        );
    }

    #[test]
    fn a_quoted_year_and_a_bare_one_read_the_same() {
        assert_eq!(
            scalar(&serde_yaml::Value::String("1820".into())).as_deref(),
            Some("1820")
        );
        assert_eq!(
            scalar(&serde_yaml::from_str::<serde_yaml::Value>("1820").unwrap()).as_deref(),
            Some("1820")
        );
    }
}
