//! Reading `.yidam/corpus/` into dated nodes.
//!
//! All of the I/O and none of the deciding, kept apart for the reason `succession` and
//! `covering` both keep them apart: the interesting logic should be testable without a corpus
//! on disk.

use crate::{normalize, Span, Undated};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RawNode {
    class: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    properties: BTreeMap<String, serde_yaml::Value>,
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

/// One node, reduced to what a chronology needs.
///
/// `span` carries the failure as well as the success. A node that could not be dated is kept
/// rather than dropped, because the shape of what this corpus cannot date is the subject of
/// `/history` and is not a thing to discover by noticing a count is short.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dated {
    /// `place/lima.yml` — class directory and filename, as links resolve to.
    pub id: String,
    pub class: String,
    pub label: String,
    pub span: Result<Span, Undated>,
}

/// Every node under `corpus_dir`, with its span or the reason it has none.
///
/// Class contracts (`*.ont.yml`) are skipped: they describe the classes rather than instancing
/// them, and a contract has no dates of its own.
pub fn corpus(corpus_dir: &Path) -> Result<Vec<Dated>, LoadError> {
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
            let properties: BTreeMap<String, String> = raw
                .properties
                .iter()
                .filter_map(|(k, v)| scalar(v).map(|s| (k.clone(), s)))
                .collect();
            out.push(Dated {
                id: format!(
                    "{class_dir}/{}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ),
                span: normalize(&raw.class, &properties),
                class: raw.class,
                label: raw.label,
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_corpus_is_an_empty_one_rather_than_an_error() {
        // Same posture as `covering::load`: a corpus that is not there is a state, not a
        // build failure, and the caller decides what an empty answer means.
        assert_eq!(corpus(Path::new("does/not/exist")).unwrap(), Vec::new());
    }

    #[test]
    fn reads_a_year_written_as_a_bare_number() {
        assert_eq!(
            scalar(&serde_yaml::Value::Number(1820.into())),
            Some("1820".to_string())
        );
    }
}
