//! The feeds themselves: what the site is allowed to read.
//!
//! `web/` never parses a corpus node. It reads these files, and they carry a
//! [`FEED_VERSION`] so a rendering change and an ontology change stay separable — the
//! coupling the repository's `web/README.md` asked to be settled before a page existed.
//!
//! Everything here is a pure function of the corpus. No timestamp, no commit hash, nothing
//! that changes when the corpus does not: that is what lets `publish-feeds --check` compare
//! the committed feeds byte for byte and fail on a stale one.

use crate::derived::Resolved;
use crate::load::Node;
use crate::tier::Tier;
use serde::Serialize;
use std::collections::BTreeMap;

/// The feed contract version.
///
/// Bump when a consumer would break: a field removed, a field's meaning changed, a shape
/// altered. Adding a field is not a break.
pub const FEED_VERSION: u32 = 1;

/// Relationships that describe the corpus rather than the world.
///
/// The same three `provenance` names. They carry no claim tag by design, so they cannot be
/// filtered by tier and are simply not published: an edge saying which class a node
/// instantiates is of no use to a reader who is not holding the ontology.
const STRUCTURAL: [&str; 3] = ["instance-of", "concerns", "subject-of"];

#[derive(Debug, Serialize)]
pub struct Policy {
    /// The weakest tier this feed carries.
    pub ceiling: Tier,
    /// Why that ceiling, in the site's own words.
    pub rationale: &'static str,
}

#[derive(Debug, Serialize)]
pub struct Manifest {
    pub feed_version: u32,
    pub policy: Policy,
    pub corpus: Counts,
}

#[derive(Debug, Default, Serialize)]
pub struct Counts {
    pub nodes: usize,
    pub nodes_published: usize,
    pub blocks: usize,
    pub blocks_published: usize,
    /// Blocks withheld because they carry no tag — corpus commentary, not county claims.
    pub blocks_untagged: usize,
    /// Blocks withheld because they are weaker than the ceiling.
    pub blocks_withheld: usize,
    /// Property values withheld because they carry a tag weaker than the ceiling.
    pub properties_withheld: usize,
    pub edges: usize,
    pub edges_published: usize,
    pub assertions: usize,
    pub by_class: BTreeMap<String, usize>,
    /// Edge counts per `class --relationship->` per tag, the shape `edge-audit` prints.
    pub edge_tags: BTreeMap<String, BTreeMap<String, usize>>,
}

#[derive(Debug, Serialize)]
pub struct FeedBlock {
    pub text: String,
    pub tier: Tier,
}

#[derive(Debug, Serialize)]
pub struct FeedNode {
    pub id: String,
    pub class: String,
    pub label: String,
    /// The node's tier: the weakest claim it publishes.
    pub tier: Tier,
    pub properties: BTreeMap<String, String>,
    pub blocks: Vec<FeedBlock>,
    /// Refusal sentences anywhere in this node, carried verbatim.
    pub refusals: Vec<String>,
    /// How many of this node's blocks did not publish, and are therefore not below.
    pub withheld: usize,
}

#[derive(Debug, Serialize)]
pub struct FeedEdge {
    pub from: String,
    pub to: String,
    pub relationship: String,
    pub tier: Tier,
    pub source: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Graph {
    pub feed_version: u32,
    pub nodes: Vec<FeedNode>,
    pub edges: Vec<FeedEdge>,
}

/// A point in a series, with the edge that says what it is about.
#[derive(Debug, Serialize)]
pub struct Point {
    pub node: String,
    pub label: String,
    pub as_of: String,
    /// The figure as a number, for plotting.
    pub value: f64,
    /// The figure as the corpus published it, unrounded and underived.
    pub published: String,
    pub tier: Tier,
    pub source: Option<String>,
    /// The `method` property: how it was arrived at, and any break in series.
    pub method: Option<String>,
}

/// Every measure describing one subject with one parameter, ordered in time.
#[derive(Debug, Serialize)]
pub struct Series {
    pub id: String,
    pub subject: String,
    pub subject_label: String,
    pub parameter: String,
    pub unit: Option<String>,
    pub tier: Tier,
    pub points: Vec<Point>,
}

#[derive(Debug, Serialize)]
pub struct SeriesFeed {
    pub feed_version: u32,
    pub series: Vec<Series>,
    pub assertions: Vec<Resolved>,
}

/// A corpus node that can be put on a map.
#[derive(Debug, Serialize)]
pub struct MapPoint {
    pub id: String,
    pub class: String,
    pub label: String,
    pub tier: Tier,
    pub lat: f64,
    pub lon: f64,
    /// The Census GEOID, where the node carries one — the join key to the vendored geometry.
    pub geoid: Option<String>,
    pub kind: Option<String>,
    pub area_sq_mi: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MapFeed {
    pub feed_version: u32,
    pub points: Vec<MapPoint>,
}

/// Parse a decimal-degree pair as the corpus writes it: `"40.771627, -84.106103"`.
fn coordinates(raw: &str) -> Option<(f64, f64)> {
    let (lat, lon) = raw.split_once(',')?;
    Some((lat.trim().parse().ok()?, lon.trim().parse().ok()?))
}

/// Parse a published figure. Thousands separators are presentation, not value.
fn figure(raw: &str) -> Option<f64> {
    raw.replace(',', "").trim().parse().ok()
}

fn published_blocks(node: &Node, ceiling: Tier) -> Vec<FeedBlock> {
    node.blocks
        .iter()
        .filter(|b| b.publishable(ceiling))
        .map(|b| FeedBlock {
            text: b.text.clone(),
            tier: b.tier.expect("publishable implies tagged"),
        })
        .collect()
}

/// Build the public graph feed, and count what it left behind.
pub fn graph(nodes: &[Node], ceiling: Tier) -> (Graph, Counts) {
    let mut counts = Counts {
        nodes: nodes.len(),
        ..Default::default()
    };
    let mut feed_nodes = Vec::new();

    for node in nodes {
        *counts.by_class.entry(node.class.clone()).or_default() += 1;
        counts.blocks += node.blocks.len();
        counts.blocks_untagged += node.blocks.iter().filter(|b| b.tier.is_none()).count();
        counts.blocks_withheld += node
            .blocks
            .iter()
            .filter(|b| b.tier.is_some_and(|t| !t.reaches(ceiling)))
            .count();

        let blocks = published_blocks(node, ceiling);
        counts.blocks_published += blocks.len();

        let properties: BTreeMap<String, String> = node
            .properties
            .iter()
            .filter(|(_, v)| crate::claim::publishable_property(v, ceiling))
            .map(|(k, v)| (k.clone(), crate::claim::normalize(v)))
            .collect();
        counts.properties_withheld += node.properties.len() - properties.len();

        // A node with nothing publishable is not published. It keeps its edges out of the
        // feed too — an edge to a node the reader cannot open is a dangling reference.
        let Some(tier) = Tier::weakest(blocks.iter().map(|b| b.tier)) else {
            continue;
        };
        counts.nodes_published += 1;

        feed_nodes.push(FeedNode {
            id: node.id.clone(),
            class: node.class.clone(),
            label: node.label.clone(),
            tier,
            properties,
            withheld: node.blocks.len() - blocks.len(),
            blocks,
            refusals: node
                .blocks
                .iter()
                .filter_map(|b| b.refusal.clone())
                .collect(),
        });
    }

    let published: std::collections::BTreeSet<&str> =
        feed_nodes.iter().map(|n| n.id.as_str()).collect();

    let mut edges = Vec::new();
    for node in nodes {
        for link in &node.links {
            if STRUCTURAL.contains(&link.relationship.as_str()) {
                continue;
            }
            counts.edges += 1;

            let shape = format!("{} --{}->", node.class, link.relationship);
            let tag = link.claim_tag.map_or("untagged".into(), |t| t.to_string());
            *counts
                .edge_tags
                .entry(shape)
                .or_default()
                .entry(tag)
                .or_default() += 1;

            // An untagged edge is a defect `edge-audit` gates on. It is not this crate's
            // job to report it, but it is this crate's job never to publish it: an edge
            // that does not say what kind of claim it is has no tier to travel on.
            let Some(tier) = link.claim_tag else { continue };
            if !tier.reaches(ceiling) {
                continue;
            }
            if !published.contains(node.id.as_str()) || !published.contains(link.resolved.as_str())
            {
                continue;
            }

            counts.edges_published += 1;
            edges.push(FeedEdge {
                from: node.id.clone(),
                to: link.resolved.clone(),
                relationship: link.relationship.clone(),
                tier,
                source: link.source.clone(),
            });
        }
    }

    (
        Graph {
            feed_version: FEED_VERSION,
            nodes: feed_nodes,
            edges,
        },
        counts,
    )
}

/// Group every measure into a series by what it describes and what it measures.
///
/// Nothing is curated here. A series is every `measure` node carrying the same `parameter`
/// and pointing at the same subject, ordered by `as_of` — which is exactly the query
/// `measure/ACTIONS.md` names as the reason the class exists. A subject measured once comes
/// back as a series of one rather than being dropped, because a single figure with its date
/// and its provenance is still the answer to a question.
pub fn series(nodes: &[Node], ceiling: Tier) -> Vec<Series> {
    let labels: BTreeMap<&str, &str> = nodes
        .iter()
        .map(|n| (n.id.as_str(), n.label.as_str()))
        .collect();

    let mut grouped: BTreeMap<(String, String), Series> = BTreeMap::new();

    for node in nodes.iter().filter(|n| n.class == "measure") {
        let (Some(parameter), Some(raw)) = (node.property("parameter"), node.property("value"))
        else {
            continue;
        };
        let (Some(as_of), Some(value)) = (node.property("as_of"), figure(raw)) else {
            continue;
        };

        // The `describes` edge is what says whose figure this is, and it is the tagged,
        // sourced claim about the figure — so it is the figure's tier.
        let Some(link) = node.links.iter().find(|l| l.relationship == "describes") else {
            continue;
        };
        let Some(tier) = link.claim_tag else { continue };
        if !tier.reaches(ceiling) {
            continue;
        }

        let subject = link.resolved.clone();
        let entry = grouped
            .entry((subject.clone(), parameter.to_string()))
            .or_insert_with(|| Series {
                id: format!(
                    "{}::{}",
                    subject.trim_end_matches(".yml").replace('/', "-"),
                    parameter.replace(' ', "-")
                ),
                subject_label: labels.get(subject.as_str()).unwrap_or(&"").to_string(),
                subject,
                parameter: parameter.to_string(),
                unit: node.property("unit").map(str::to_string),
                tier,
                points: Vec::new(),
            });

        entry.tier = entry.tier.max(tier);
        entry.points.push(Point {
            node: node.id.clone(),
            label: node.label.clone(),
            as_of: as_of.to_string(),
            value,
            published: raw.to_string(),
            tier,
            source: link.source.clone(),
            method: node.property("method").map(crate::claim::normalize),
        });
    }

    let mut out: Vec<Series> = grouped.into_values().collect();
    for s in &mut out {
        // By date, then by node id: two figures for one date is not a tie to be broken
        // arbitrarily. This corpus has one — an enumeration and an estimates base, eleven
        // people apart — and a stable order is what keeps the feed reproducible.
        s.points
            .sort_by(|a, b| a.as_of.cmp(&b.as_of).then_with(|| a.node.cmp(&b.node)));
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

/// Every node the corpus places on the ground.
pub fn map(nodes: &[Node], ceiling: Tier) -> Vec<MapPoint> {
    let mut points: Vec<MapPoint> = nodes
        .iter()
        .filter_map(|node| {
            // `place` writes `centroid`; `site` and `natural-feature` write `coordinates`.
            // Both go through the same property rule as the graph feed: a located node whose
            // location is tagged below the ceiling is not placed on a public map.
            let raw = node
                .property("centroid")
                .or_else(|| node.property("coordinates"))
                .filter(|v| crate::claim::publishable_property(v, ceiling))?;
            let (lat, lon) = coordinates(raw)?;
            let tier = Tier::weakest(
                node.blocks
                    .iter()
                    .filter(|b| b.publishable(ceiling))
                    .filter_map(|b| b.tier),
            )?;

            Some(MapPoint {
                id: node.id.clone(),
                class: node.class.clone(),
                label: node.label.clone(),
                tier,
                lat,
                lon,
                geoid: node.property("geoid").map(str::to_string),
                kind: node
                    .property("place_type")
                    .or_else(|| node.property("site_type"))
                    .or_else(|| node.property("feature_type"))
                    .map(str::to_string),
                area_sq_mi: node
                    .property("area_sq_mi")
                    .filter(|v| crate::claim::publishable_property(v, ceiling))
                    .map(str::to_string),
            })
        })
        .collect();
    points.sort_by(|a, b| a.id.cmp(&b.id));
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claim::blocks;
    use crate::load::Link;

    fn measure(name: &str, parameter: &str, value: &str, as_of: &str, tag: Tier) -> Node {
        let mut properties = BTreeMap::new();
        properties.insert("parameter".into(), parameter.into());
        properties.insert("value".into(), value.into());
        properties.insert("as_of".into(), as_of.into());
        Node {
            id: format!("measure/{name}.yml"),
            class: "measure".into(),
            name: name.into(),
            label: name.into(),
            properties,
            blocks: blocks("A figure. [verified]"),
            links: vec![Link {
                target: "../place/allen-county.yml".into(),
                resolved: "place/allen-county.yml".into(),
                relationship: "describes".into(),
                claim_tag: Some(tag),
                source: Some("catalog/x.md".into()),
            }],
        }
    }

    #[test]
    fn measures_of_one_parameter_about_one_subject_become_one_series_in_date_order() {
        let nodes = vec![
            measure(
                "p2020",
                "total resident population",
                "102,217",
                "2020-04-01",
                Tier::Verified,
            ),
            measure(
                "p1970",
                "total resident population",
                "111,144",
                "1970",
                Tier::Verified,
            ),
        ];
        let s = series(&nodes, Tier::Inference);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].points.len(), 2);
        assert_eq!(s[0].points[0].as_of, "1970");
        assert_eq!(s[0].points[0].value, 111_144.0);
    }

    #[test]
    fn a_thousands_separator_is_presentation_and_the_published_string_survives_it() {
        let nodes = vec![measure("p", "pop", "102,217", "2020", Tier::Verified)];
        let s = series(&nodes, Tier::Inference);
        assert_eq!(s[0].points[0].value, 102_217.0);
        assert_eq!(s[0].points[0].published, "102,217");
    }

    #[test]
    fn two_figures_for_one_date_both_survive_in_a_stable_order() {
        // The enumeration and the estimates base, eleven people apart. Dropping either
        // would be the corpus's own caution about comparability, discarded silently.
        let nodes = vec![
            measure("b-est", "pop", "102217", "2020-04-01", Tier::Verified),
            measure("a-census", "pop", "102206", "2020-04-01", Tier::Verified),
        ];
        let s = series(&nodes, Tier::Inference);
        assert_eq!(s[0].points.len(), 2);
        assert_eq!(s[0].points[0].node, "measure/a-census.yml");
    }

    #[test]
    fn a_measure_whose_describes_edge_is_too_weak_does_not_publish() {
        let nodes = vec![measure("p", "pop", "1", "2020", Tier::Open)];
        assert!(series(&nodes, Tier::Inference).is_empty());
    }

    #[test]
    fn a_subject_measured_once_is_a_series_of_one() {
        let nodes = vec![measure("p", "land area", "402.545", "2020", Tier::Verified)];
        assert_eq!(series(&nodes, Tier::Inference)[0].points.len(), 1);
    }

    #[test]
    fn a_node_with_no_publishable_prose_is_not_in_the_graph() {
        let node = Node {
            id: "place/x.yml".into(),
            class: "place".into(),
            name: "x".into(),
            label: "X".into(),
            properties: BTreeMap::new(),
            blocks: blocks("Only a guess. [open]"),
            links: Vec::new(),
        };
        let (g, counts) = graph(&[node], Tier::Inference);
        assert!(g.nodes.is_empty());
        assert_eq!(counts.nodes, 1);
        assert_eq!(counts.nodes_published, 0);
        assert_eq!(counts.blocks_withheld, 1);
    }

    #[test]
    fn an_edge_into_an_unpublished_node_does_not_dangle_in_the_feed() {
        let withheld = Node {
            id: "place/y.yml".into(),
            class: "place".into(),
            name: "y".into(),
            label: "Y".into(),
            properties: BTreeMap::new(),
            blocks: blocks("Only a guess. [open]"),
            links: Vec::new(),
        };
        let citing = Node {
            id: "place/x.yml".into(),
            class: "place".into(),
            name: "x".into(),
            label: "X".into(),
            properties: BTreeMap::new(),
            blocks: blocks("Solid ground. [verified]"),
            links: vec![Link {
                target: "y.yml".into(),
                resolved: "place/y.yml".into(),
                relationship: "within".into(),
                claim_tag: Some(Tier::Verified),
                source: None,
            }],
        };
        let (g, _) = graph(&[citing, withheld], Tier::Inference);
        assert_eq!(g.nodes.len(), 1);
        assert!(g.edges.is_empty());
    }

    #[test]
    fn a_structural_edge_is_not_published_and_is_not_counted_as_a_claim() {
        let node = Node {
            id: "place/x.yml".into(),
            class: "place".into(),
            name: "x".into(),
            label: "X".into(),
            properties: BTreeMap::new(),
            blocks: blocks("Solid ground. [verified]"),
            links: vec![Link {
                target: "../place.ont.yml".into(),
                resolved: "place.ont.yml".into(),
                relationship: "instance-of".into(),
                claim_tag: None,
                source: None,
            }],
        };
        let (g, counts) = graph(&[node], Tier::Inference);
        assert!(g.edges.is_empty());
        assert_eq!(counts.edges, 0);
    }

    #[test]
    fn a_property_tagged_open_never_reaches_the_feed() {
        let mut properties = BTreeMap::new();
        properties.insert("geoid".into(), "39003".into());
        properties.insert(
            "boundary_basis".into(),
            "Both bounds are unsourced approximations. [open]".into(),
        );
        let node = Node {
            id: "period/x.yml".into(),
            class: "period".into(),
            name: "x".into(),
            label: "X".into(),
            properties,
            blocks: blocks("Solid ground. [verified]"),
            links: Vec::new(),
        };
        let (g, counts) = graph(&[node], Tier::Inference);
        assert_eq!(counts.properties_withheld, 1);
        assert!(g.nodes[0].properties.contains_key("geoid"));
        assert!(!g.nodes[0].properties.contains_key("boundary_basis"));
    }

    #[test]
    fn a_coordinate_pair_parses_as_the_corpus_writes_it() {
        assert_eq!(
            coordinates("40.771627, -84.106103"),
            Some((40.771627, -84.106103))
        );
        assert_eq!(coordinates("not a pair"), None);
    }
}
