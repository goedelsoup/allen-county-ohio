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
use crate::load::{Class, Link, Node};
use crate::tier::Tier;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

/// The feed contract version.
///
/// Bump when a consumer would break: a field removed, a field's meaning changed, a shape
/// altered. Adding a field is not a break.
///
/// `manifest.classes` was added under that rule and did not bump it. Every consumer of the
/// four feeds reads exactly what it read before; the manifest simply carries one more key.
/// Bumping for an addition would spend the signal that means *stop and read the diff*, and
/// then it would mean nothing when a field's sense actually changes.
///
/// **`atlas.json` did not bump it either, and that is the harder case.** A whole new feed
/// file looks like it should — it is the largest thing added to this contract since the
/// contract existed. It is still strictly additive: no field moved, no meaning changed, and
/// every page that read the four feeds reads exactly what it read before. The plan that
/// produced it assumed a bump; the rule written here is older than the plan and says
/// otherwise, and a version that moves for additions is a version nobody reads.
pub const FEED_VERSION: u32 = 1;

/// Relationships that describe the corpus rather than the world.
///
/// The same three `provenance` names. They carry no claim tag by design, so they cannot be
/// filtered by tier and are simply not published: an edge saying which class a node
/// instantiates is of no use to a reader who is not holding the ontology.
const STRUCTURAL: [&str; 3] = ["instance-of", "concerns", "subject-of"];

/// Relationships that judge one figure against another.
///
/// The same two names `provenance` gates, where they carry the rule that each must say why.
/// They are ordinary published edges as well as the source of the comparability tables — the
/// graph feed carries them like any other claim, and [`comparability`] reads them again into
/// the shape an entry renders.
const COMPARABILITY: [&str; 2] = ["comparable-to", "not-comparable-to"];

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
    /// What each class declares itself to be, keyed by class name.
    pub classes: BTreeMap<String, ClassSchema>,
}

/// A class's ontology, as the class states it.
///
/// This is not derived from the instances and does not move when they do: it is
/// `<class>.ont.yml` carried across the feed boundary so a page can say what licenses the
/// structure it is rendering — that a tenure is a relator is *why* the dates sit on the
/// holding rather than on the person.
#[derive(Debug, Serialize)]
pub struct ClassSchema {
    pub label: String,
    pub ontology: String,
    pub foundational_type: String,
    pub edge_policy: String,
    /// Properties a node of this class may not omit, in declaration order.
    pub required: Vec<String>,
    /// The class's own account of why it exists.
    pub description: String,
}

/// The declared ontology, keyed by class.
///
/// Every declared class is carried, including one the corpus has no instance of yet: the
/// ontology is what the corpus says it is, and a class with nothing in it is a fact about
/// the corpus rather than a reason to hide the declaration.
pub fn schema(classes: &[Class]) -> BTreeMap<String, ClassSchema> {
    classes
        .iter()
        .map(|c| {
            (
                c.class.clone(),
                ClassSchema {
                    label: c.label.clone(),
                    ontology: c.ontology.clone(),
                    foundational_type: c.foundational_type.clone(),
                    edge_policy: c.edge_policy.clone(),
                    required: c.required.clone(),
                    description: c.description.clone(),
                },
            )
        })
        .collect()
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

/// One row of a measure's comparability table.
///
/// The board this renders puts the year, the figure and a sentence saying whether the figure
/// may be set beside the one the entry is about. All three are carried; none is computed.
#[derive(Debug, Serialize)]
pub struct ComparabilityRow {
    pub node: String,
    pub label: String,
    pub as_of: String,
    /// The figure as the corpus published it, unrounded and underived.
    pub published: String,
    pub unit: Option<String>,
    /// True on the row for the entry's own figure — the board's *This figure.*
    pub this_figure: bool,
    /// The judgement. `null` on the entry's own row, which is not judged against itself.
    pub comparable: Option<bool>,
    /// Why. Never empty where `comparable` is set: `edge-audit` fails on a judgement that
    /// does not say, so a table can never print *Not comparable* and stop there.
    pub because: Option<String>,
    /// The **judgement's** tag, not the figure's. A verified figure may be set beside another
    /// verified figure on nothing better than an inference, and that is the tier a reader of
    /// the row is being asked to trust.
    pub tier: Option<Tier>,
    pub source: Option<String>,
}

/// A measure's comparability table, ready to render.
#[derive(Debug, Serialize)]
pub struct Comparability {
    /// The measure this table belongs to.
    pub node: String,
    /// Every judged figure and this one, in date order.
    pub rows: Vec<ComparabilityRow>,
}

#[derive(Debug, Serialize)]
pub struct SeriesFeed {
    pub feed_version: u32,
    pub series: Vec<Series>,
    /// One entry per measure the corpus has judged against another. A measure with no
    /// judgement is absent rather than present with a single row: an entry that says only
    /// *this figure* reads as a series of one, which is a claim nobody made.
    pub comparability: Vec<Comparability>,
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

/// Every measure's comparability table, from the judgements the corpus recorded.
///
/// # Why this does not read [`series`]
///
/// It would be one line to build these tables out of a series — the neighbours of a figure are
/// sitting right there, grouped by subject and parameter. That grouping is a mechanical join on
/// two strings, and a comparability table built on it would assert, in the one place designed to
/// warn against exactly this, that two figures sharing a `parameter` describe the same thing.
/// They frequently do not: the corpus's own minor-civil-division series puts 56,580 in 1910
/// beside 27,132 in 1930, and the whole of that fall is Lima leaving the table.
///
/// So the neighbourhood here is the edges and nothing else. A measure appears beside another
/// measure because somebody wrote down that it may, and said why.
///
/// The corpus writes the judgement once, on the later figure, which is the transition rule
/// `measure/ACTIONS.md` already stated. Both ends get the row: a reader arriving at the earlier
/// figure needs the warning at least as much, because that node is the one that cannot know its
/// successor moved the definition.
pub fn comparability(nodes: &[Node], ceiling: Tier) -> Vec<Comparability> {
    let published: BTreeMap<&str, &Node> = nodes
        .iter()
        .filter(|n| {
            // The same rule `graph` applies: a row linking to a node the reader cannot open
            // is a dangling reference, and here it would be one inside a warning.
            !published_blocks(n, ceiling).is_empty()
        })
        .map(|n| (n.id.as_str(), n))
        .collect();

    let row = |n: &Node, judged: Option<(&Link, bool)>| ComparabilityRow {
        node: n.id.clone(),
        label: n.label.clone(),
        as_of: n.property("as_of").unwrap_or_default().to_string(),
        published: n.property("value").unwrap_or_default().to_string(),
        unit: n.property("unit").map(str::to_string),
        this_figure: judged.is_none(),
        comparable: judged.map(|(_, c)| c),
        because: judged.and_then(|(l, _)| l.because.clone()),
        tier: judged.and_then(|(l, _)| l.claim_tag),
        source: judged.and_then(|(l, _)| l.source.clone()),
    };

    let mut tables: BTreeMap<&str, Vec<ComparabilityRow>> = BTreeMap::new();
    for node in nodes.iter().filter(|n| n.class == "measure") {
        for link in &node.links {
            if !COMPARABILITY.contains(&link.relationship.as_str()) {
                continue;
            }
            // A judgement with no tag, no reason, or a tag weaker than the ceiling does not
            // travel. The first two are defects `edge-audit` gates on; the third is the
            // publication rule, and an unpublishable judgement is silence rather than a row
            // with a hole in it.
            let Some(tier) = link.claim_tag else { continue };
            if !tier.reaches(ceiling) || link.because.is_none() {
                continue;
            }
            let (Some(here), Some(there)) = (
                published.get(node.id.as_str()),
                published.get(link.resolved.as_str()),
            ) else {
                continue;
            };
            let comparable = link.relationship == "comparable-to";

            tables
                .entry(&node.id)
                .or_insert_with(|| vec![row(here, None)])
                .push(row(there, Some((link, comparable))));
            tables
                .entry(&there.id)
                .or_insert_with(|| vec![row(there, None)])
                .push(row(here, Some((link, comparable))));
        }
    }

    let mut out: Vec<Comparability> = tables
        .into_iter()
        .map(|(node, mut rows)| {
            // By date, then by node id — the same tie-break `series` uses, and for the same
            // reason: two figures at one date is a thing this corpus has, and a stable order
            // is what lets `publish-feeds --check` compare bytes.
            rows.sort_by(|a, b| a.as_of.cmp(&b.as_of).then_with(|| a.node.cmp(&b.node)));
            Comparability {
                node: node.to_string(),
                rows,
            }
        })
        .collect();
    out.sort_by(|a, b| a.node.cmp(&b.node));
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

// ── the atlas ────────────────────────────────────────────────────────────────

/// One position, and the route that reached it.
#[derive(Debug, Serialize)]
pub struct AtlasAnchor {
    /// The node that states this position.
    pub node: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    /// A Census key, where the anchor is a shape rather than a point. The site joins it to
    /// `public/geo/`; nothing on this side of the boundary knows where a polygon is.
    pub geoid: Option<String>,
    /// The summary level `geoid` belongs to, without which the key does not identify anything:
    /// `3904752` is Beaverdam village *and* the Upper Scioto Valley Local School District, and
    /// a consumer indexing shapes by bare GEOID draws one as the other. `county`, `place`,
    /// `county-subdivision` or `school-district`.
    pub level: Option<&'static str>,
    /// The edges followed to reach it, in order. Empty where the node is its own anchor.
    pub via: Vec<AtlasStep>,
}

#[derive(Debug, Serialize)]
pub struct AtlasStep {
    pub relationship: String,
    pub to: String,
    /// The tag the edge carries, or `None` for a structural edge that carries none by rule.
    pub tier: Option<Tier>,
}

/// A node reduced to when it was and where it lands.
///
/// The two halves come from `chronology` and `placement`, joined here because this is the
/// only crate that knows the publication ceiling. Everything a caller needs to draw the node
/// is present: the shape, the year bounds, and the warrant for both.
#[derive(Debug, Serialize)]
pub struct AtlasRecord {
    pub node: String,
    pub class: String,
    pub label: String,
    pub tier: Tier,

    // ── when ──
    /// First year the corpus places it in.
    pub from: Option<i32>,
    /// Last year, where there is one. **Absent on every open end**, including the ones read
    /// as running to the present: "now" is not a date any source recorded.
    pub to: Option<i32>,
    /// `year`, `month` or `day` — how much of the date the source gave.
    pub precision: Option<&'static str>,
    /// What an absent end means for this class: `instantaneous`, `running`, `unvouched` or
    /// `unknown`. Absent where both ends are recorded, or where the node is undated.
    pub open_end: Option<&'static str>,
    /// Why the node carries no span, where it carries none.
    pub undated: Option<String>,

    // ── where ──
    /// `mark`, `polygon`, `count`, `register`, `unplaced` or `not-spatial`.
    pub treatment: &'static str,
    /// Edges followed to reach ground. `0` is a stated position.
    pub hops: Option<usize>,
    /// The weakest tag on the placement route.
    pub route_tier: Option<Tier>,
    /// Every anchor reached at `hops`. More than one is not a defect — a district serving
    /// five townships is at all five, and averaging them would invent a centroid.
    pub anchors: Vec<AtlasAnchor>,
}

#[derive(Debug, Serialize)]
pub struct AtlasFeed {
    pub feed_version: u32,
    pub records: Vec<AtlasRecord>,
}

fn precision_name(p: chronology::Precision) -> &'static str {
    match p {
        chronology::Precision::Year => "year",
        chronology::Precision::Month => "month",
        chronology::Precision::Day => "day",
    }
}

fn open_end_name(o: chronology::OpenEnd) -> &'static str {
    match o {
        chronology::OpenEnd::Instant => "instantaneous",
        chronology::OpenEnd::Running => "running",
        chronology::OpenEnd::Unvouched => "unvouched",
        chronology::OpenEnd::Unknown => "unknown",
    }
}

fn treatment_name(t: placement::Treatment) -> &'static str {
    match t {
        placement::Treatment::Mark => "mark",
        placement::Treatment::Polygon => "polygon",
        placement::Treatment::Count => "count",
        placement::Treatment::Register => "register",
        placement::Treatment::Unplaced => "unplaced",
        placement::Treatment::NotSpatial => "not-spatial",
    }
}

/// The properties a node states a position with.
///
/// A node whose position is tagged below the ceiling must not be placed, which is the rule
/// [`map`] already applies to its own points. Applying it by *removing the property* rather
/// than by filtering afterwards means the resolver never sees it — so nothing else can reach
/// that node's ground through it either.
const LOCATION_PROPERTIES: [&str; 4] = ["centroid", "coordinates", "geoid", "fips_code"];

/// The corpus as `placement` needs it, with everything the ceiling withholds already gone.
fn placement_graph(nodes: &[Node], ceiling: Tier) -> placement::Graph {
    let mut g = placement::Graph::default();
    for node in nodes {
        let properties = node
            .properties
            .iter()
            .filter(|(k, v)| {
                !LOCATION_PROPERTIES.contains(&k.as_str())
                    || crate::claim::publishable_property(v, ceiling)
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        g.insert(placement::Node {
            id: node.id.clone(),
            class: node.class.clone(),
            label: node.label.clone(),
            properties,
            links: node
                .links
                .iter()
                .filter(|l| l.relationship != "instance-of")
                .map(|l| placement::Link {
                    target: l.resolved.clone(),
                    relationship: l.relationship.clone(),
                    claim_tag: l.claim_tag.map(|t| t.to_string()),
                })
                .collect(),
        });
    }
    g
}

/// The Census summary level a key belongs to — and without which the key means nothing.
///
/// **A GEOID is unique only inside its summary level**, and this county holds a live collision
/// to prove it: `3904752` is Beaverdam village *and* the Upper Scioto Valley Local School
/// District. Both are seven digits, both are in `public/geo/`, and a site indexing shapes by
/// bare GEOID draws a village of 319 people as a school district spanning two counties.
///
/// Length settles every other case, because the Census builds the key out of the levels above
/// it: five digits is state and county, seven is state and place, ten is state, county and
/// county subdivision. Only the seven-digit case is ambiguous, and the corpus already carries
/// what resolves it — a school district node says so in `jurisdiction_type`.
///
/// `None` for a key of a length this corpus has not met. A caller that cannot name the level
/// should refuse the join rather than guess it.
fn geoid_level(key: &str, stated: Option<&Node>) -> Option<&'static str> {
    if stated.and_then(|n| n.property("jurisdiction_type")) == Some("school district") {
        return Some("school-district");
    }
    match key.len() {
        5 => Some("county"),
        7 => Some("place"),
        10 => Some("county-subdivision"),
        _ => None,
    }
}

fn tier_word(raw: Option<&String>) -> Option<Tier> {
    match raw.map(String::as_str) {
        Some("verified") => Some(Tier::Verified),
        Some("inference") => Some(Tier::Inference),
        Some("open") => Some(Tier::Open),
        _ => None,
    }
}

/// When every node was, and where it lands.
///
/// The join `chronology` and `placement` were written for. Every node appears, including the
/// undated and the unplaced: the shape of what this corpus cannot date or cannot place is the
/// subject of a page, and a feed that dropped those rows would make it undiscoverable.
///
/// A node with no publishable prose is excluded, on the same rule [`map`] applies — a node
/// that cannot say anything cannot be drawn saying it.
///
/// **One route step may cite an edge `graph.json` does not carry.** `concerns` is structural
/// by `provenance`'s reckoning and is not published as an edge, but it is a subject claim and
/// `placement` routes through it — it is what places all sixteen of the corpus's questions.
/// The endpoints are both published nodes, so the route is followable; the edge itself is
/// readable only in the corpus. Whether the graph feed should carry `concerns` is a question
/// for whoever next touches it.
pub fn atlas(nodes: &[Node], ceiling: Tier) -> Vec<AtlasRecord> {
    let graph = placement_graph(nodes, ceiling);
    let by_id: HashMap<&str, &Node> = nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let mut records: Vec<AtlasRecord> = nodes
        .iter()
        .filter_map(|node| {
            let tier = Tier::weakest(
                node.blocks
                    .iter()
                    .filter(|b| b.publishable(ceiling))
                    .filter_map(|b| b.tier),
            )?;

            let span = chronology::normalize(&node.class, &node.properties);
            let placed = placement::place(&graph, &node.id);
            let treatment = placement::treatment(&node.class, placed.as_ref());

            let (from, to, precision, open_end, undated) = match &span {
                Ok(s) => {
                    let open = match s {
                        chronology::Span::OpenEnded { reading, .. } => {
                            Some(open_end_name(*reading))
                        }
                        chronology::Span::Instant(_) => {
                            Some(open_end_name(chronology::OpenEnd::Instant))
                        }
                        chronology::Span::Bounded { .. } => None,
                    };
                    (
                        Some(s.from_year()),
                        s.to_year(),
                        Some(precision_name(s.precision())),
                        open,
                        None,
                    )
                }
                Err(why) => (None, None, None, None, Some(why.to_string())),
            };

            let anchors = placed
                .as_ref()
                .map(|p| {
                    p.reached
                        .iter()
                        .map(|r| {
                            let (lat, lon, geoid, level) = match &r.anchor {
                                placement::Anchor::Point { lat, lon } => {
                                    (Some(*lat), Some(*lon), None, None)
                                }
                                placement::Anchor::Census { key } => {
                                    // The level is a fact about the *stating* node, which on a
                                    // routed anchor is not the node being placed.
                                    let stated = by_id.get(r.node.as_str()).copied();
                                    (None, None, Some(key.clone()), geoid_level(key, stated))
                                }
                            };
                            AtlasAnchor {
                                node: r.node.clone(),
                                lat,
                                lon,
                                geoid,
                                level,
                                via: r
                                    .via
                                    .iter()
                                    .map(|s| AtlasStep {
                                        relationship: s.relationship.clone(),
                                        to: s.to.clone(),
                                        tier: tier_word(s.tag.as_ref()),
                                    })
                                    .collect(),
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            Some(AtlasRecord {
                node: node.id.clone(),
                class: node.class.clone(),
                label: node.label.clone(),
                tier,
                from,
                to,
                precision,
                open_end,
                undated,
                treatment: treatment_name(treatment),
                hops: placed.as_ref().map(|p| p.hops),
                route_tier: placed.as_ref().and_then(|p| tier_word(p.tag.as_ref())),
                anchors,
            })
        })
        .collect();
    records.sort_by(|a, b| a.node.cmp(&b.node));
    records
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
                because: None,
            }],
        }
    }

    /// Record a comparability judgement on `node`, pointing at a sibling measure.
    fn judge(node: &mut Node, relationship: &str, target: &str, because: Option<&str>, tag: Tier) {
        node.links.push(Link {
            target: format!("{target}.yml"),
            resolved: format!("measure/{target}.yml"),
            relationship: relationship.into(),
            claim_tag: Some(tag),
            source: None,
            because: because.map(str::to_string),
        });
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

    /// The corpus's own minor-civil-division pair, which is why this feed exists: one
    /// `parameter` string, two different things counted, and a fall that is entirely Lima
    /// leaving the table.
    fn divisions() -> Vec<Node> {
        let p = "total resident population by minor civil division, decennial";
        let mut later = measure("t1930", p, "27132", "1930", Tier::Verified);
        judge(
            &mut later,
            "not-comparable-to",
            "t1910",
            Some("The 1910 figure counts Ottawa, which was Lima; the 1930 one excludes the city."),
            Tier::Verified,
        );
        vec![later, measure("t1910", p, "56580", "1910", Tier::Verified)]
    }

    #[test]
    fn a_judgement_written_once_makes_a_table_at_both_ends() {
        let t = comparability(&divisions(), Tier::Inference);
        assert_eq!(t.len(), 2);

        // Written on the later figure. The earlier one gets the warning too — it is the node
        // that cannot know its successor changed what was being counted.
        let earlier = t.iter().find(|c| c.node == "measure/t1910.yml").unwrap();
        assert_eq!(earlier.rows.len(), 2);
        assert!(earlier.rows[0].this_figure);
        assert_eq!(earlier.rows[0].as_of, "1910");
        assert_eq!(earlier.rows[0].comparable, None);
        assert_eq!(earlier.rows[1].node, "measure/t1930.yml");
        assert_eq!(earlier.rows[1].comparable, Some(false));
        assert!(earlier.rows[1].because.as_deref().unwrap().contains("Lima"));

        let later = t.iter().find(|c| c.node == "measure/t1930.yml").unwrap();
        assert!(later.rows[1].this_figure);
        assert_eq!(later.rows[0].node, "measure/t1910.yml");
        assert_eq!(later.rows[0].comparable, Some(false));
    }

    #[test]
    fn the_table_carries_the_judgements_tier_and_not_the_figures() {
        // Both figures are verified and the judgement between them is not. That gap is the
        // whole thing a reader of the row is being asked to weigh, so it travels on the row.
        let mut later = measure("b", "farms", "897", "2022", Tier::Verified);
        judge(
            &mut later,
            "not-comparable-to",
            "a",
            Some("The two censuses set different thresholds and the thresholds cross."),
            Tier::Inference,
        );
        let nodes = vec![later, measure("a", "farms", "2939", "1910", Tier::Verified)];
        let t = comparability(&nodes, Tier::Inference);
        let judged = t[0].rows.iter().find(|r| !r.this_figure).unwrap();
        assert_eq!(judged.tier, Some(Tier::Inference));
    }

    #[test]
    fn a_measure_nobody_judged_gets_no_table_at_all() {
        // Not a table of one row. A lone *This figure.* reads as a series of one, which is a
        // claim about continuity that nobody made.
        let nodes = vec![measure("p", "pop", "1", "2020", Tier::Verified)];
        assert!(comparability(&nodes, Tier::Inference).is_empty());
    }

    #[test]
    fn a_judgement_that_does_not_say_why_does_not_travel() {
        let mut later = measure("b", "pop", "2", "2020", Tier::Verified);
        judge(&mut later, "comparable-to", "a", None, Tier::Verified);
        let nodes = vec![later, measure("a", "pop", "1", "2010", Tier::Verified)];
        assert!(comparability(&nodes, Tier::Inference).is_empty());
    }

    #[test]
    fn a_judgement_weaker_than_the_ceiling_is_silence_rather_than_a_row_with_a_hole_in_it() {
        let mut later = measure("b", "pop", "2", "2020", Tier::Verified);
        judge(
            &mut later,
            "comparable-to",
            "a",
            Some("a guess"),
            Tier::Open,
        );
        let nodes = vec![later, measure("a", "pop", "1", "2010", Tier::Verified)];
        assert!(comparability(&nodes, Tier::Inference).is_empty());
    }

    #[test]
    fn a_judgement_against_an_unpublished_figure_does_not_dangle() {
        let mut later = measure("b", "pop", "2", "2020", Tier::Verified);
        judge(
            &mut later,
            "comparable-to",
            "a",
            Some("same file"),
            Tier::Verified,
        );
        let mut hidden = measure("a", "pop", "1", "2010", Tier::Verified);
        hidden.blocks = blocks("Only a guess. [open]");
        assert!(comparability(&[later, hidden], Tier::Inference).is_empty());
    }

    #[test]
    fn the_table_is_not_the_series() {
        // Two figures share a parameter and a subject, so `series` groups them; nobody judged
        // them against each other, so `comparability` says nothing. That difference is the
        // point — a table built from the grouping would assert the continuity it exists to
        // warn about.
        let nodes = vec![
            measure("a", "pop", "1", "2010", Tier::Verified),
            measure("b", "pop", "2", "2020", Tier::Verified),
        ];
        assert_eq!(series(&nodes, Tier::Inference)[0].points.len(), 2);
        assert!(comparability(&nodes, Tier::Inference).is_empty());
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
                because: None,
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
                because: None,
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
    fn a_location_tagged_below_the_ceiling_places_nothing() {
        // The one way this feed could leak a withheld claim. `map` already refuses to publish
        // a point whose coordinate is tagged too weakly; the atlas has to refuse harder,
        // because a coordinate reached through an edge would place *another* node too.
        let mut lima = BTreeMap::new();
        lima.insert("centroid".into(), "40.740679, -84.112091 [open]".into());
        let place = Node {
            id: "place/lima.yml".into(),
            class: "place".into(),
            name: "lima".into(),
            label: "Lima".into(),
            properties: lima,
            blocks: blocks("The county seat. [verified]"),
            links: Vec::new(),
        };
        let measure = Node {
            id: "measure/x.yml".into(),
            class: "measure".into(),
            name: "x".into(),
            label: "X".into(),
            properties: BTreeMap::from([("as_of".to_string(), "2020".to_string())]),
            blocks: blocks("A figure. [verified]"),
            links: vec![crate::load::Link {
                target: "../place/lima.yml".into(),
                resolved: "place/lima.yml".into(),
                relationship: "describes".into(),
                claim_tag: Some(Tier::Verified),
                source: None,
                because: None,
            }],
        };

        let records = atlas(&[place, measure], Tier::Inference);
        let by = |id: &str| {
            records
                .iter()
                .find(|r| r.node == id)
                .unwrap_or_else(|| panic!("{id} missing"))
        };
        assert_eq!(
            by("place/lima.yml").treatment,
            "unplaced",
            "a withheld coordinate is not a position"
        );
        assert_eq!(
            by("measure/x.yml").treatment,
            "unplaced",
            "and nothing may reach ground through it either"
        );
        assert!(records.iter().all(|r| r.anchors.is_empty()));
    }

    #[test]
    fn an_open_end_never_serializes_a_closing_year() {
        let node = Node {
            id: "division/d.yml".into(),
            class: "division".into(),
            name: "d".into(),
            label: "D".into(),
            properties: BTreeMap::from([("effective_from".to_string(), "2020".to_string())]),
            blocks: blocks("A district. [verified]"),
            links: Vec::new(),
        };
        let records = atlas(&[node], Tier::Inference);
        assert_eq!(records[0].from, Some(2020));
        assert_eq!(records[0].to, None, "an open end has no recorded close");
        assert_eq!(records[0].open_end, Some("unvouched"));
    }

    #[test]
    fn a_node_that_cannot_be_dated_or_placed_still_gets_a_row() {
        // The shape of what this corpus cannot date or cannot place is a subject in its own
        // right. A feed that dropped those rows would make it undiscoverable.
        let node = Node {
            id: "natural-feature/creek.yml".into(),
            class: "natural-feature".into(),
            name: "creek".into(),
            label: "A creek".into(),
            properties: BTreeMap::new(),
            blocks: blocks("Water. [verified]"),
            links: Vec::new(),
        };
        let records = atlas(&[node], Tier::Inference);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].from, None);
        assert!(records[0].undated.is_some());
        assert_eq!(records[0].treatment, "unplaced");
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
