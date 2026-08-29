//! Reading coordinates out of `.yidam/corpus/`.
//!
//! Two classes carry a usable coordinate and the rest are excluded on purpose — see
//! [`points`].

use crate::{parse_area_sq_mi, parse_lat_lon, Anchor, Point};
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
    /// A coordinate property that is present and unreadable.
    ///
    /// Reported rather than skipped. A node silently dropped for a malformed coordinate is a
    /// node that quietly stops appearing in every proximity answer, which is worse than a
    /// failure because nothing says it happened.
    BadCoordinate {
        path: PathBuf,
        field: String,
        value: String,
    },
    /// A coordinate that parses, is a real place on Earth, and is not in Ohio.
    ///
    /// This exists for one failure the range check cannot see: writing the pair the wrong way
    /// round. `-84.112091, 40.740679` is Lima's coordinate transposed and is a valid point in
    /// Antarctica, so nothing about its *form* is wrong. Only the domain knows it is wrong,
    /// and this is the domain.
    OutsideOhio {
        path: PathBuf,
        field: String,
        lat: f64,
        lon: f64,
    },
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "{e}"),
            LoadError::Yaml { path, source } => write!(f, "{}: {source}", path.display()),
            LoadError::BadCoordinate { path, field, value } => write!(
                f,
                "{}: `{field}` is not a latitude, longitude pair: {value:?}",
                path.display()
            ),
            LoadError::OutsideOhio {
                path,
                field,
                lat,
                lon,
            } => write!(
                f,
                "{}: `{field}` is {lat}, {lon} — a valid coordinate outside Ohio. \
                 Transposed?",
                path.display()
            ),
        }
    }
}
impl std::error::Error for LoadError {}
impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::Io(e)
    }
}

fn scalar(v: &serde_yaml::Value) -> Option<String> {
    match v {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// Every corpus node carrying a coordinate this calculator will rank on.
///
/// **`place` and `site` only, and the exclusions are deliberate.**
///
/// `natural-feature` carries `mouth` and `source` coordinates and is left out. A stream's mouth
/// is where it *ends*, not where it is: the Ottawa River runs through the middle of Lima and its
/// mouth is in Putnam County, so ranking against it would report the county's principal
/// watercourse as far from the city it flows through. The corpus already refused the same
/// mistake for the Miami and Erie Canal, whose single GNIS point sits 34 miles north of the
/// reach this corpus cares about. One point does not locate a line.
///
/// `division` carries no coordinate property at all; the tract's internal point lives in prose.
///
/// Every coordinate is bounded against Ohio, generously. See [`LoadError::OutsideOhio`].
pub fn points(corpus_dir: &Path) -> Result<Vec<Point>, LoadError> {
    let mut out = Vec::new();
    for (class, field, anchor) in [
        ("place", "centroid", Anchor::InternalPoint),
        ("site", "coordinates", Anchor::Location),
    ] {
        let dir = corpus_dir.join(class);
        if !dir.is_dir() {
            continue;
        }
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
            if raw.class != class {
                continue;
            }
            let Some(v) = raw.properties.get(field).and_then(scalar) else {
                continue;
            };
            let Some((lat, lon)) = parse_lat_lon(&v) else {
                return Err(LoadError::BadCoordinate {
                    path,
                    field: field.to_string(),
                    value: v,
                });
            };
            if !in_ohio(lat, lon) {
                return Err(LoadError::OutsideOhio {
                    path,
                    field: field.to_string(),
                    lat,
                    lon,
                });
            }
            out.push(Point {
                node: format!(
                    "{class}/{}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ),
                label: raw.label,
                class: class.to_string(),
                lat,
                lon,
                anchor,
                area_sq_mi: raw
                    .properties
                    .get("area_sq_mi")
                    .and_then(scalar)
                    .and_then(|s| parse_area_sq_mi(&s)),
            });
        }
    }
    out.sort_by(|a, b| a.node.cmp(&b.node));
    Ok(out)
}

/// Ohio's extent, rounded outward by about a tenth of a degree.
///
/// Deliberately loose. This is a tripwire for a transposed or mistyped pair, not a claim about
/// where the state line runs — a corpus that has spent four phases learning that internal
/// points do not bound polygons is not about to bound one with a rectangle.
fn in_ohio(lat: f64, lon: f64) -> bool {
    (38.3..=42.4).contains(&lat) && (-85.0..=-80.4).contains(&lon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_corpus_yields_no_points_and_no_error() {
        assert!(points(Path::new("/nonexistent/corpus")).unwrap().is_empty());
    }

    #[test]
    fn the_domain_bound_catches_what_range_validation_cannot() {
        // Lima, and Lima with the pair the wrong way round.
        assert!(in_ohio(40.740679, -84.112091));
        assert!(!in_ohio(-84.112091, 40.740679));
        // The county's far corners, comfortably inside.
        assert!(in_ohio(40.647272, -83.879939));
        assert!(in_ohio(40.904493, -84.388008));
        // Fort Wayne, which GNIS files under a county called Allen.
        assert!(!in_ohio(41.0833816, -85.1321928));
    }
}
