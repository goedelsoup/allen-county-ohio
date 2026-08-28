//! Reading tenure and office nodes off disk.
//!
//! Kept apart from the audit so the calculator stays pure. Everything here is I/O and
//! shape-wrangling; nothing here decides anything.

use crate::Term;
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
    /// A year field that is not a year. Reported rather than skipped: a tenure whose dates
    /// cannot be read is exactly the node an audit would otherwise silently omit, and an
    /// omitted term manufactures a gap that is not in the corpus.
    BadYear {
        path: PathBuf,
        field: String,
        value: String,
    },
    MissingBegan {
        path: PathBuf,
    },
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "{e}"),
            LoadError::Yaml { path, source } => write!(f, "{}: {source}", path.display()),
            LoadError::BadYear { path, field, value } => {
                write!(f, "{}: `{field}` is not a year: {value:?}", path.display())
            }
            LoadError::MissingBegan { path } => {
                write!(f, "{}: tenure has no `began`", path.display())
            }
        }
    }
}
impl std::error::Error for LoadError {}
impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::Io(e)
    }
}

/// Render a YAML scalar as the string the corpus wrote. Years are quoted in this corpus, but
/// a bare `1831` is legal YAML and parses as an integer, so both are accepted here — the
/// `property-type` lint is what argues about which the file should contain.
fn scalar(v: &serde_yaml::Value) -> Option<String> {
    match v {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn year(path: &Path, field: &str, v: &serde_yaml::Value) -> Result<i32, LoadError> {
    let raw = scalar(v).unwrap_or_default();
    // A date may be recorded at year, month or day precision. Only the year is used: the
    // sources this reads give years, and truncating is honest where padding would not be.
    let head = raw.split('-').next().unwrap_or("").trim().to_string();
    head.parse::<i32>().map_err(|_| LoadError::BadYear {
        path: path.to_path_buf(),
        field: field.to_string(),
        value: raw,
    })
}

fn read_nodes(dir: &Path) -> Result<Vec<(PathBuf, RawNode)>, LoadError> {
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "yml"))
        .collect();
    entries.sort();
    for path in entries {
        let text = std::fs::read_to_string(&path)?;
        let node: RawNode = serde_yaml::from_str(&text).map_err(|source| LoadError::Yaml {
            path: path.clone(),
            source,
        })?;
        out.push((path, node));
    }
    Ok(out)
}

fn rel(target: &str) -> String {
    // `../office/allen-county-sheriff.yml` -> `office/allen-county-sheriff.yml`
    target.trim_start_matches("../").to_string()
}

/// Every tenure under `corpus_dir`, paired with the office node it points at.
pub fn tenures(corpus_dir: &Path) -> Result<Vec<(String, Term)>, LoadError> {
    let mut out = Vec::new();
    for (path, node) in read_nodes(&corpus_dir.join("tenure"))? {
        if node.class != "tenure" {
            continue;
        }
        let began = match node.properties.get("began") {
            Some(v) => year(&path, "began", v)?,
            None => return Err(LoadError::MissingBegan { path }),
        };
        let ended = match node.properties.get("ended") {
            Some(v) => Some(year(&path, "ended", v)?),
            None => None,
        };
        let term_number = node
            .properties
            .get("term_number")
            .and_then(scalar)
            .and_then(|s| s.parse::<u32>().ok());

        let office = node
            .links
            .iter()
            .find(|l| l.relationship == "of-office")
            .map(|l| rel(&l.target))
            .unwrap_or_default();
        let holder = node
            .links
            .iter()
            .find(|l| l.relationship == "held-by")
            .map(|l| rel(&l.target))
            .unwrap_or_default();

        let node_path = format!(
            "tenure/{}",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        out.push((
            office,
            Term {
                node: node_path,
                holder,
                began,
                ended,
                term_number,
            },
        ));
        let _ = &node.label;
    }
    Ok(out)
}

/// Seat counts per office node, defaulting to 1 where the class does not say.
///
/// The default is 1 because that is what an office is unless it says otherwise, and because
/// the failure it produces is the safe one: a multi-seat board read as single-seat reports
/// overlaps that a reader will immediately recognise, where the reverse silently accepts a
/// real double-holding.
pub fn office_seats(corpus_dir: &Path) -> Result<BTreeMap<String, u32>, LoadError> {
    let mut m = BTreeMap::new();
    for (path, node) in read_nodes(&corpus_dir.join("office"))? {
        if node.class != "office" {
            continue;
        }
        let seats = node
            .properties
            .get("seats")
            .and_then(scalar)
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(1);
        let key = format!(
            "office/{}",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        m.insert(key, seats);
    }
    Ok(m)
}
