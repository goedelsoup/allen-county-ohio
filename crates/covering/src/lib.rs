//! What covers a piece of ground, and what the corpus is entitled to say about when.
//!
//! The question is [`covering`]: given a place and a year, which jurisdictions and divisions
//! covered it? The answer is a **set**, not a hierarchy. A village, the township around it,
//! the county, a school district and three redistricting artifacts can all lie over one acre
//! with five different boundaries, and none of them contains the others.
//!
//! # The part that is actually hard
//!
//! Almost every edge in this corpus is undated. `place governed-by jurisdiction` records that
//! the City of Lima governs Lima; it does not record since when. Walking those edges and
//! handing back the result as the answer for 1850 would invent a fact — the same failure
//! `succession` was built to avoid, wearing different clothes.
//!
//! So every member of the covering set carries a [`Warrant`]: what the corpus records about
//! this member's own dates, and whether the queried year falls inside. Members whose dates
//! admit the year and members the corpus never dated are returned in **separate lists** and
//! are never merged. The undated ones are a real answer to "what covers this ground" and no
//! answer at all to "what covered it in 1850", and the shape of the result says so.
//!
//! # Where this deliberately disagrees with `succession`
//!
//! In `succession` a tenure with no `ended` is read as running to the present, because the
//! last entry in a sheriff's roster is the sitting sheriff. The same shape means the opposite
//! here. The corpus node `division/ohio-congressional-district-4-2020.yml` carries
//! `effective_from: 2020` and no `effective_to`, and its own text says why: `effective_to` is
//! absent because the corpus does not know the date the map was superseded, not because the
//! district still stands as drawn. Ohio redistricted after 2020.
//!
//! A missing end is therefore [`Warrant::Open`] — it admits years at or after the start and
//! vouches for nothing beyond the last date the corpus can actually support. Two calculators,
//! two readings of the same absent field, because the sources differ in what silence means.

pub mod load;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

// ── the graph ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub target: String,
    pub relationship: String,
}

/// One corpus node, reduced to what a coverage query needs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Node {
    /// `place/lima.yml` — class directory and filename, as links resolve to.
    pub id: String,
    pub class: String,
    pub label: String,
    pub properties: BTreeMap<String, String>,
    pub links: Vec<Link>,
}

/// The corpus as an addressable graph. Built by [`load`]; every function here is pure.
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

    pub fn ids_of_class<'a>(&'a self, class: &'a str) -> impl Iterator<Item = &'a str> {
        self.nodes
            .values()
            .filter(move |n| n.class == class)
            .map(|n| n.id.as_str())
    }

    /// Targets of `id`'s outgoing `relationship` edges that resolve to a node in the graph.
    ///
    /// An edge to a node that is not here is dropped rather than reported: the corpus gate
    /// already fails on a dangling link, so anything this drops is either a class this loader
    /// did not read or a defect a different tool owns.
    pub fn out(&self, id: &str, relationship: &str) -> Vec<&Node> {
        let Some(n) = self.nodes.get(id) else {
            return Vec::new();
        };
        n.links
            .iter()
            .filter(|l| l.relationship == relationship)
            .filter_map(|l| self.nodes.get(&l.target))
            .collect()
    }

    /// Nodes with a `relationship` edge pointing **at** `id`.
    ///
    /// Needed because the corpus writes some coverage edges from the covering side: a school
    /// district `serves` a place, a division `covers` one. Those are the same fact as
    /// `governed-by` read from the other end, and a query that only walked outward from the
    /// place would silently miss every school district in the corpus.
    pub fn inbound(&self, id: &str, relationship: &str) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|n| {
                n.links
                    .iter()
                    .any(|l| l.relationship == relationship && l.target == id)
            })
            .collect()
    }
}

// ── what the corpus says about when ──────────────────────────────────────────

/// What the corpus records about a member's own dates, and how the queried year fares.
///
/// Intervals are **closed at both ends**, matching `succession::holders_in` and for the same
/// reason: these dates are year-precision, so a jurisdiction erected in 1820 and one abolished
/// in 1820 both have a claim on that year, and picking a side would be a precision the sources
/// do not have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Warrant {
    /// Both ends recorded, and the year — if one was asked for — falls inside.
    Bounded { from: i32, to: i32 },
    /// A start with no recorded end.
    ///
    /// This admits the year and asserts nothing past it. A missing end in this corpus means
    /// "not recorded", which is not the same as "still in force" — see the module docs.
    Open { from: i32 },
    /// An end with no recorded start. Admits years at or before it.
    Until { to: i32 },
    /// No dates at all. The membership is asserted; when it began and whether it still holds
    /// are both unrecorded.
    Undated,
    /// The year lies outside what is recorded. `recorded` renders the dates that ruled it out.
    Outside { recorded: String },
}

impl Warrant {
    /// Whether a member with this warrant belongs in the covering set at all.
    pub fn admits(&self) -> bool {
        !matches!(self, Warrant::Outside { .. })
    }

    /// Whether the corpus dated this membership, as opposed to merely asserting it.
    pub fn is_dated(&self) -> bool {
        !matches!(self, Warrant::Undated | Warrant::Outside { .. })
    }
}

impl std::fmt::Display for Warrant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Warrant::Bounded { from, to } => write!(f, "recorded {from}–{to}"),
            Warrant::Open { from } => write!(f, "recorded from {from}, no end recorded"),
            Warrant::Until { to } => write!(f, "recorded to {to}, no start recorded"),
            Warrant::Undated => write!(f, "no dates recorded"),
            Warrant::Outside { recorded } => write!(f, "{recorded}"),
        }
    }
}

/// Decide a warrant from a recorded date pair and an optional query year.
///
/// With `year: None` nothing can be excluded, so this reports what the corpus records and
/// stops there. That is a different question from a dated one, not a lazier version of it.
pub fn warrant(from: Option<i32>, to: Option<i32>, year: Option<i32>) -> Warrant {
    let recorded = match (from, to) {
        (None, None) => Warrant::Undated,
        (Some(f), None) => Warrant::Open { from: f },
        (None, Some(t)) => Warrant::Until { to: t },
        (Some(f), Some(t)) => Warrant::Bounded { from: f, to: t },
    };
    let Some(y) = year else {
        return recorded;
    };
    let inside = match (from, to) {
        (None, None) => true,
        (Some(f), None) => y >= f,
        (None, Some(t)) => y <= t,
        (Some(f), Some(t)) => y >= f && y <= t,
    };
    if inside {
        recorded
    } else {
        Warrant::Outside {
            recorded: recorded.to_string(),
        }
    }
}

// ── members of a covering set ────────────────────────────────────────────────

/// How a member was arrived at, in descending order of what the corpus actually asserts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reach {
    /// An edge between this member and the queried place itself.
    Asserted,
    /// Reached from an asserted member by following authority edges — `territory-within`,
    /// `nested-in`. Still a chain of things the corpus states.
    ///
    /// A path is only as strong as its weakest step, so nesting out of an *inherited* member
    /// stays inherited rather than being promoted to this.
    Nested,
    /// Reached from a place the queried place lies `within`.
    ///
    /// **This is an inference, not an assertion.** Ground inside a larger named place is
    /// normally under that place's authorities, and normally is not always: an incorporated
    /// village inside a township is not governed by the township for most purposes, and Lima
    /// is separated from the townships around it entirely. The corpus records those
    /// separations in prose, so this reach is the weakest one here and is labelled as such
    /// rather than being suppressed.
    Inherited,
}

impl std::fmt::Display for Reach {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reach::Asserted => write!(f, "asserted"),
            Reach::Nested => write!(f, "nested"),
            Reach::Inherited => write!(f, "inherited"),
        }
    }
}

/// Whether the member covers the queried ground entirely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extent {
    Whole,
    /// Some path to this member crosses an edge that says the ground is split.
    ///
    /// `via` names the edge that said so, because the claim belongs to that edge and a reader
    /// should be able to go and read it.
    Partial {
        via: String,
    },
}

impl Extent {
    fn is_partial(&self) -> bool {
        matches!(self, Extent::Partial { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub node: String,
    pub label: String,
    /// `jurisdiction` or `division`.
    pub class: String,
    /// `jurisdiction_type` or `division_type` — county, school district, census tract.
    pub kind: String,
    pub reach: Reach,
    /// The nodes stepped through to get here, in order, excluding the start and the member.
    pub via: Vec<String>,
    pub extent: Extent,
    pub warrant: Warrant,
}

/// A place the query declined to inherit through, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pruned {
    pub node: String,
    pub label: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Covering {
    /// The place the answer is about. For a site query this is the place the site is in, not
    /// the site.
    pub place: String,
    /// Set when the query started at a site. A site's covering set is its **place's** covering
    /// set, which is coarser than the site's own ground: it cannot say which ward an address
    /// falls in, only which authorities lie over the place containing it.
    pub resolved_from: Option<String>,
    pub year: Option<i32>,
    /// Members the corpus dates, whose dates admit the year.
    pub dated: Vec<Member>,
    /// Members the corpus asserts but never dated. Real coverage; no evidence about the year.
    pub undated: Vec<Member>,
    /// Members whose recorded dates put the year outside them.
    pub excluded: Vec<Member>,
    /// Ancestor places not walked through, because their own dates exclude the year.
    pub pruned: Vec<Pruned>,
}

impl Covering {
    /// Every member that covers the ground, dated or not.
    pub fn all(&self) -> impl Iterator<Item = &Member> {
        self.dated.iter().chain(self.undated.iter())
    }

    pub fn member(&self, node: &str) -> Option<&Member> {
        self.all().find(|m| m.node == node)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverError {
    UnknownNode(String),
    /// A class this query has no way to start from.
    NotGround {
        node: String,
        class: String,
    },
    /// A site with no `located-in` or `situated-in` edge to a place in the graph.
    SiteWithoutPlace(String),
}

impl std::fmt::Display for CoverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoverError::UnknownNode(id) => write!(f, "no such node: {id}"),
            CoverError::NotGround { node, class } => write!(
                f,
                "{node} is a {class}; coverage is asked of a place or a site"
            ),
            CoverError::SiteWithoutPlace(id) => {
                write!(f, "{id} is not located in any place this graph holds")
            }
        }
    }
}
impl std::error::Error for CoverError {}

// ── the query ────────────────────────────────────────────────────────────────

/// Edges that carry authority from a jurisdiction or division to a larger one.
///
/// `partially-within` is here and is not the same edge as `territory-within`: it is the
/// corpus's way of saying the ground is split, and collapsing the two would erase the only
/// place that distinction is written down. The `bool` is whether the edge says so.
const NESTING: [(&str, bool); 3] = [
    ("territory-within", false),
    ("nested-in", false),
    ("partially-within", true),
];

/// Edges that attach an authority directly to a place: the relationship, whether the corpus
/// writes it from the covering side, and whether it says the coverage is partial.
///
/// `covers` and `partially-covers` are both here because this corpus has used `covers` to mean
/// whole coverage in every instance of it — three districts over a county, each verified as
/// containing all 3,552 of its blocks. A division that holds four fifths of a township is a
/// different claim and gets a different edge.
const SEEDS: [(&str, bool, bool); 4] = [
    ("governed-by", false, false),
    ("serves", true, false),
    ("covers", true, false),
    ("partially-covers", true, true),
];

/// Edges that put one named place inside another.
///
/// `partially-within` appears here too, and it has to: Delphos and Bluffton both cross a
/// county line, and the containment edge is where a place — as opposed to a jurisdiction —
/// gets to say so.
const CONTAINMENT: [(&str, bool); 2] = [("within", false), ("partially-within", true)];

/// Which jurisdictions and divisions cover `start`, and what the corpus dates.
///
/// `start` is a `place` node, or a `site` node — a site resolves to the place it is located in
/// and the answer is that place's, which [`Covering::resolved_from`] records.
///
/// `year` filters. `None` asks the undated question: what covers this ground at all, with each
/// member's own dates reported and nothing excluded.
pub fn covering(g: &Graph, start: &str, year: Option<i32>) -> Result<Covering, CoverError> {
    let node = g
        .get(start)
        .ok_or_else(|| CoverError::UnknownNode(start.to_string()))?;

    let (place_id, resolved_from) = match node.class.as_str() {
        "place" => (node.id.clone(), None),
        "site" => {
            let place = ["located-in", "situated-in"]
                .iter()
                .flat_map(|r| g.out(&node.id, r))
                .find(|n| n.class == "place")
                .ok_or_else(|| CoverError::SiteWithoutPlace(node.id.clone()))?;
            (place.id.clone(), Some(node.id.clone()))
        }
        other => {
            return Err(CoverError::NotGround {
                node: node.id.clone(),
                class: other.to_string(),
            })
        }
    };

    let mut out = Covering {
        place: place_id.clone(),
        resolved_from,
        year,
        ..Default::default()
    };

    // Pass A — the containment chain of places, breadth first so `via` is the shortest path.
    // A place whose own dates exclude the year is not walked through: the ground was there,
    // but the named place the authorities attach to was not.
    let mut ground: Vec<(String, Vec<String>, Extent)> = Vec::new();
    let mut seen_places: BTreeSet<String> = BTreeSet::new();
    let mut queue: VecDeque<(String, Vec<String>, Extent)> =
        VecDeque::from([(place_id.clone(), vec![], Extent::Whole)]);
    while let Some((id, via, extent)) = queue.pop_front() {
        if !seen_places.insert(id.clone()) {
            continue;
        }
        // The queried place itself is never pruned. Asking what covered Lima in 1700 is a
        // question about Lima, and refusing to answer because Lima was not there yet would
        // discard the query rather than answer it.
        if !via.is_empty() {
            let w = node_warrant(g, &id, year);
            if !w.admits() {
                let n = g.get(&id);
                out.pruned.push(Pruned {
                    node: id.clone(),
                    label: n.map(|n| n.label.clone()).unwrap_or_default(),
                    reason: w.to_string(),
                });
                continue;
            }
        }
        for (rel, splits) in CONTAINMENT {
            for n in g.out(&id, rel) {
                if n.class != "place" {
                    continue;
                }
                let ext = if splits {
                    Extent::Partial {
                        via: format!("{id} {rel}"),
                    }
                } else {
                    extent.clone()
                };
                let mut v = via.clone();
                v.push(id.clone());
                queue.push_back((n.id.clone(), v, ext));
            }
        }
        ground.push((id, via, extent));
    }

    // Pass B — the authorities attached directly to each of those places.
    let mut found: BTreeMap<String, Member> = BTreeMap::new();
    let mut weak_partial: BTreeSet<String> = BTreeSet::new();
    let mut frontier: VecDeque<(String, Vec<String>, Extent, Reach, bool)> = VecDeque::new();
    for (place, via, extent) in &ground {
        let reach = if via.is_empty() {
            Reach::Asserted
        } else {
            Reach::Inherited
        };
        for (rel, from_covering_side, splits) in SEEDS {
            let seeds = if from_covering_side {
                g.inbound(place, rel)
            } else {
                g.out(place, rel)
            };
            for n in seeds {
                if n.class != "jurisdiction" && n.class != "division" {
                    continue;
                }
                let ext = if splits {
                    Extent::Partial {
                        via: format!("{} {rel}", n.id),
                    }
                } else {
                    extent.clone()
                };
                // A seed that covers only part of the place opens a **weak** path: whatever
                // lies further out contains a member that holds part of this ground, and
                // nothing follows from that about how much of the ground the outer body holds.
                // "This tract covers part of this township" says nothing about the county the
                // tract sits in — the county's coverage has to be judged on its own paths.
                let weak = splits && !extent.is_partial();
                if let Some(mut m) = record(
                    g,
                    &mut found,
                    &mut weak_partial,
                    n,
                    reach,
                    via.clone(),
                    ext,
                    weak,
                    year,
                ) {
                    if !weak {
                        m.2 = extent.clone();
                    }
                    frontier.push_back(m);
                }
            }
        }
    }

    // Pass C — closure over authority nesting, breadth first.
    while let Some((id, via, extent, from, weak)) = frontier.pop_front() {
        // A path is only as strong as its weakest step. Following `territory-within` out of a
        // member that was itself inherited from a containing place does not upgrade the
        // answer to an assertion — Delphos reaches the county government through a
        // redistricting artifact that way, and calling that chain "nested" would dress an
        // inference about containment as a statement the corpus made.
        let reach = from.max(Reach::Nested);
        for (rel, splits) in NESTING {
            for n in g.out(&id, rel) {
                if n.class != "jurisdiction" && n.class != "division" {
                    continue;
                }
                // A split declared on a weak path is a fact about the member's own
                // territory, not about the queried ground: this district reaching outside the
                // county limits nothing when the district only ever held part of the place.
                let ext = if splits && !weak {
                    Extent::Partial {
                        via: format!("{id} {rel}"),
                    }
                } else {
                    extent.clone()
                };
                let mut v = via.clone();
                v.push(id.clone());
                if let Some(m) = record(
                    g,
                    &mut found,
                    &mut weak_partial,
                    n,
                    reach,
                    v,
                    ext,
                    weak,
                    year,
                ) {
                    frontier.push_back(m);
                }
            }
        }
    }

    for m in found.into_values() {
        if !m.warrant.admits() {
            out.excluded.push(m);
        } else if m.warrant.is_dated() {
            out.dated.push(m);
        } else {
            out.undated.push(m);
        }
    }
    for list in [&mut out.dated, &mut out.undated, &mut out.excluded] {
        list.sort_by(|a, b| (a.reach, &a.node).cmp(&(b.reach, &b.node)));
    }
    Ok(out)
}

/// Add or improve one member, returning the frontier entry when the closure should continue
/// through it.
///
/// Two merge rules, both load-bearing:
///
/// - **The strongest reach wins.** A node found both by nesting and by inheriting from a
///   containing place is reported as nested, because that path is a chain of edges the corpus
///   states rather than an inference about containment.
/// - **Partial wins over whole, on a path that could have shown wholeness.** A
///   `partially-within` edge is an explicit claim that part of the territory lies outside; a
///   whole path is a chain of edges each of which is *silent* about splitting rather than
///   asserting completeness. Reading silence as a refutation of a stated claim has it
///   backwards. Bluffton is the worked case: the village jurisdiction says `partially-within`
///   the county because it crosses into Hancock, while the school district serving the same
///   village says a flat `territory-within`. The county covers Bluffton partially.
/// - **A weak path can never refute a whole one.** A path that reaches a member through a body
///   holding only *part* of the queried ground carries no information about how much of that
///   ground anything further out holds. Lima is the worked case: four school districts each
///   `partially-covers` the city, and one of them is `partially-within` the county because it
///   reaches into Auglaize. That split is a fact about the district's territory. Lima is
///   wholly inside Allen County and the answer must keep saying so.
///
///   The distinction was invisible while edges were untagged, when reading any non-partial
///   relationship as an assertion of completeness would have been generous. It is not generous
///   now: `within` and `partially-within` are a contrasting pair this corpus uses deliberately,
///   and [`provenance`](../../provenance/) requires every one of them to carry a claim tag and,
///   where verified, a source.
#[allow(clippy::too_many_arguments)]
fn record(
    g: &Graph,
    found: &mut BTreeMap<String, Member>,
    weak_partial: &mut BTreeSet<String>,
    n: &Node,
    reach: Reach,
    via: Vec<String>,
    extent: Extent,
    weak: bool,
    year: Option<i32>,
) -> Option<(String, Vec<String>, Extent, Reach, bool)> {
    // A member whose own dates exclude the year is recorded as excluded and not continued
    // through: authority cannot be inherited through a body that did not exist.
    let w = node_warrant(g, &n.id, year);
    let admits = w.admits();

    let candidate = Member {
        node: n.id.clone(),
        label: n.label.clone(),
        class: n.class.clone(),
        kind: n
            .properties
            .get("jurisdiction_type")
            .or_else(|| n.properties.get("division_type"))
            .cloned()
            .unwrap_or_default(),
        reach,
        via: via.clone(),
        extent: extent.clone(),
        warrant: w,
    };

    match found.get_mut(&n.id) {
        None => {
            if weak && extent.is_partial() {
                weak_partial.insert(n.id.clone());
            }
            found.insert(n.id.clone(), candidate);
        }
        Some(existing) => {
            let stronger = reach < existing.reach
                || (reach == existing.reach && via.len() < existing.via.len());
            let newly_partial = extent.is_partial() && !existing.extent.is_partial() && !weak;
            // A strong whole path replaces a split that only a weak path had claimed. Nothing
            // else can restore wholeness: a split established on a path that could have shown
            // wholeness stands.
            let newly_whole = !extent.is_partial() && !weak && weak_partial.contains(&n.id);
            if newly_partial || newly_whole {
                existing.extent = extent.clone();
                weak_partial.remove(&n.id);
            }
            if stronger {
                existing.reach = reach;
                existing.via = via.clone();
            }
            // Re-walk only when this member's extent has just changed, so the change reaches
            // everything downstream of it. A member can be split at most once and restored at
            // most once, so the closure still terminates. Any other repeat visit would find
            // exactly what the first one found.
            let admitted = existing.warrant.admits();
            return ((newly_partial || newly_whole) && admitted)
                .then(|| (n.id.clone(), via, extent, reach, weak));
        }
    }

    admits.then(|| (n.id.clone(), via, extent, reach, weak))
}

/// The date pair a node carries, read per class, and the warrant it yields for `year`.
fn node_warrant(g: &Graph, id: &str, year: Option<i32>) -> Warrant {
    let Some(n) = g.get(id) else {
        return Warrant::Undated;
    };
    let (a, b) = match n.class.as_str() {
        "jurisdiction" => ("erected", "abolished"),
        "division" => ("effective_from", "effective_to"),
        "place" => ("established", ""),
        _ => ("", ""),
    };
    let y = |k: &str| -> Option<i32> {
        if k.is_empty() {
            return None;
        }
        n.properties.get(k).and_then(|v| year_of(v))
    };
    warrant(y(a), y(b), year)
}

/// The year in a date field, at whatever precision it was written.
///
/// `1820`, `1820-05`, `1820-05-01` all read as 1820. Truncating is honest here where padding
/// would not be: the sources give years, and a corpus that wrote a month did not thereby learn
/// one.
pub fn year_of(raw: &str) -> Option<i32> {
    raw.trim().split('-').next()?.trim().parse::<i32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, class: &str, props: &[(&str, &str)], links: &[(&str, &str)]) -> Node {
        Node {
            id: id.to_string(),
            class: class.to_string(),
            label: id.to_string(),
            properties: props
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            links: links
                .iter()
                .map(|(r, t)| Link {
                    relationship: r.to_string(),
                    target: t.to_string(),
                })
                .collect(),
        }
    }

    /// A miniature of the real shape: a city inside a county, a municipal corporation over the
    /// city, a school district reaching the city from the covering side, a county government
    /// both are within, and a district drawn in 2020 over the county.
    fn fixture() -> Graph {
        let mut g = Graph::default();
        g.insert(node(
            "place/city.yml",
            "place",
            &[],
            &[
                ("within", "place/county.yml"),
                ("governed-by", "jurisdiction/city-corp.yml"),
            ],
        ));
        g.insert(node(
            "place/county.yml",
            "place",
            &[("established", "1820")],
            &[("governed-by", "jurisdiction/county-gov.yml")],
        ));
        g.insert(node(
            "jurisdiction/city-corp.yml",
            "jurisdiction",
            &[("jurisdiction_type", "municipal corporation")],
            &[("territory-within", "jurisdiction/county-gov.yml")],
        ));
        g.insert(node(
            "jurisdiction/county-gov.yml",
            "jurisdiction",
            &[("jurisdiction_type", "county"), ("erected", "1820")],
            &[],
        ));
        g.insert(node(
            "jurisdiction/school.yml",
            "jurisdiction",
            &[("jurisdiction_type", "school district")],
            &[
                ("serves", "place/city.yml"),
                ("territory-within", "jurisdiction/county-gov.yml"),
            ],
        ));
        g.insert(node(
            "division/district-2020.yml",
            "division",
            &[
                ("division_type", "congressional district"),
                ("effective_from", "2020"),
            ],
            &[("covers", "place/county.yml")],
        ));
        g
    }

    #[test]
    fn the_undated_query_returns_the_whole_covering_set() {
        let c = covering(&fixture(), "place/city.yml", None).unwrap();
        let mut ids: Vec<&str> = c.all().map(|m| m.node.as_str()).collect();
        ids.sort();
        assert_eq!(
            ids,
            vec![
                "division/district-2020.yml",
                "jurisdiction/city-corp.yml",
                "jurisdiction/county-gov.yml",
                "jurisdiction/school.yml",
            ]
        );
        assert!(
            c.excluded.is_empty(),
            "no year was asked, so nothing can be excluded"
        );
    }

    #[test]
    fn a_covering_edge_written_from_the_covering_side_is_still_found() {
        // The school district points at the place; the place points at nothing. A query that
        // only walked outward from the place would miss every school district in the corpus.
        let c = covering(&fixture(), "place/city.yml", None).unwrap();
        let m = c.member("jurisdiction/school.yml").expect("school found");
        assert_eq!(m.reach, Reach::Asserted);
    }

    #[test]
    fn undated_members_never_join_the_dated_ones() {
        let c = covering(&fixture(), "place/city.yml", Some(1900)).unwrap();
        let dated: Vec<&str> = c.dated.iter().map(|m| m.node.as_str()).collect();
        let undated: Vec<&str> = c.undated.iter().map(|m| m.node.as_str()).collect();
        assert_eq!(dated, vec!["jurisdiction/county-gov.yml"]);
        assert_eq!(
            undated,
            vec!["jurisdiction/city-corp.yml", "jurisdiction/school.yml"],
            "the corpus does not date these and the answer must not pretend it does"
        );
    }

    #[test]
    fn a_start_with_no_end_is_not_read_as_still_in_force() {
        // The 2020 district shape. It admits 2020 and later, and it is Open rather than
        // Bounded, which is how a reader learns the corpus has no end date rather than that
        // the district still stands.
        let c = covering(&fixture(), "place/city.yml", Some(2021)).unwrap();
        let m = c.member("division/district-2020.yml").unwrap();
        assert_eq!(m.warrant, Warrant::Open { from: 2020 });
        // And it is genuinely excluded before its start.
        let c = covering(&fixture(), "place/city.yml", Some(1900)).unwrap();
        assert!(c.member("division/district-2020.yml").is_none());
        assert!(c
            .excluded
            .iter()
            .any(|m| m.node == "division/district-2020.yml"));
    }

    #[test]
    fn ground_is_not_walked_through_a_place_that_did_not_exist_yet() {
        let c = covering(&fixture(), "place/city.yml", Some(1810)).unwrap();
        assert_eq!(c.pruned.len(), 1);
        assert_eq!(c.pruned[0].node, "place/county.yml");
        // Nothing reached only through the county place survives.
        assert!(c.member("division/district-2020.yml").is_none());
        // The city's own authorities are undated and still stand: the corpus does not say
        // when the corporation began, and this query must not invent it either way.
        assert!(c.member("jurisdiction/city-corp.yml").is_some());
    }

    #[test]
    fn the_queried_place_is_never_pruned_by_its_own_dates() {
        let mut g = fixture();
        g.insert(node(
            "place/city.yml",
            "place",
            &[("established", "1831")],
            &[],
        ));
        let c = covering(&g, "place/city.yml", Some(1700)).unwrap();
        assert_eq!(
            c.place, "place/city.yml",
            "the question is still about this place"
        );
        assert!(c.pruned.is_empty());
    }

    #[test]
    fn a_split_edge_makes_the_member_partial_even_where_another_path_says_whole() {
        // Bluffton in miniature: the village crosses a county line and says so, while the
        // school district serving it asserts a flat containment.
        let mut g = fixture();
        g.insert(node(
            "jurisdiction/city-corp.yml",
            "jurisdiction",
            &[("jurisdiction_type", "municipal corporation")],
            &[("partially-within", "jurisdiction/county-gov.yml")],
        ));
        let c = covering(&g, "place/city.yml", None).unwrap();
        let m = c.member("jurisdiction/county-gov.yml").unwrap();
        assert!(
            m.extent.is_partial(),
            "an explicit split must not be overridden by a path that is merely silent"
        );
        match &m.extent {
            Extent::Partial { via } => assert!(via.contains("partially-within")),
            _ => unreachable!(),
        }
    }

    #[test]
    fn nesting_outranks_inheriting_from_a_containing_place() {
        let c = covering(&fixture(), "place/city.yml", None).unwrap();
        let m = c.member("jurisdiction/county-gov.yml").unwrap();
        assert_eq!(
            m.reach,
            Reach::Nested,
            "a chain of asserted authority edges beats a containment inference"
        );
    }

    #[test]
    fn a_division_reached_through_a_containing_place_is_marked_as_inferred() {
        let c = covering(&fixture(), "place/city.yml", None).unwrap();
        let m = c.member("division/district-2020.yml").unwrap();
        assert_eq!(m.reach, Reach::Inherited);
        assert_eq!(m.via, vec!["place/city.yml"]);
    }

    #[test]
    fn a_site_resolves_to_its_place_and_says_so() {
        let mut g = fixture();
        g.insert(node(
            "site/courthouse.yml",
            "site",
            &[],
            &[("located-in", "place/city.yml")],
        ));
        let c = covering(&g, "site/courthouse.yml", None).unwrap();
        assert_eq!(c.place, "place/city.yml");
        assert_eq!(c.resolved_from.as_deref(), Some("site/courthouse.yml"));
    }

    #[test]
    fn asking_coverage_of_a_person_is_an_error_rather_than_an_empty_set() {
        let mut g = fixture();
        g.insert(node("person/someone.yml", "person", &[], &[]));
        let e = covering(&g, "person/someone.yml", None).unwrap_err();
        assert!(matches!(e, CoverError::NotGround { .. }));
        assert!(matches!(
            covering(&g, "place/nowhere.yml", None).unwrap_err(),
            CoverError::UnknownNode(_)
        ));
    }

    #[test]
    fn a_containment_cycle_terminates() {
        let mut g = Graph::default();
        g.insert(node(
            "place/a.yml",
            "place",
            &[],
            &[("within", "place/b.yml")],
        ));
        g.insert(node(
            "place/b.yml",
            "place",
            &[],
            &[("within", "place/a.yml")],
        ));
        assert!(covering(&g, "place/a.yml", None).is_ok());
    }

    #[test]
    fn closed_intervals_give_a_boundary_year_to_both_sides() {
        // Same reason as succession::holders_in: a body erected in 1820 and one abolished in
        // 1820 both have a claim on it, and year precision cannot say which came first.
        assert!(warrant(Some(1820), Some(1850), Some(1820)).admits());
        assert!(warrant(Some(1820), Some(1850), Some(1850)).admits());
        assert!(!warrant(Some(1820), Some(1850), Some(1819)).admits());
        assert!(!warrant(Some(1820), Some(1850), Some(1851)).admits());
    }

    #[test]
    fn an_undated_node_admits_every_year_and_is_never_called_dated() {
        let w = warrant(None, None, Some(1700));
        assert!(w.admits());
        assert!(!w.is_dated());
    }

    #[test]
    fn a_date_reads_at_whatever_precision_it_was_written() {
        assert_eq!(year_of("1820"), Some(1820));
        assert_eq!(year_of("1820-05"), Some(1820));
        assert_eq!(year_of("1820-05-01"), Some(1820));
        assert_eq!(year_of("sometime"), None);
    }

    #[test]
    fn a_division_that_only_partly_covers_a_place_says_so() {
        // Census tract 39003010300 holds 98.5% of Sugar Creek Township and a fifth of
        // American. Neither is `covers`, which this corpus has used to mean whole coverage
        // everywhere else, and a query that reported it as whole would be the tidier lie.
        let mut g = fixture();
        g.insert(node(
            "division/tract.yml",
            "division",
            &[("division_type", "census tract")],
            &[("partially-covers", "place/city.yml")],
        ));
        let c = covering(&g, "place/city.yml", None).unwrap();
        let m = c
            .member("division/tract.yml")
            .expect("tract covers the city");
        assert_eq!(m.reach, Reach::Asserted);
        match &m.extent {
            Extent::Partial { via } => assert!(via.contains("partially-covers"), "{via}"),
            other => panic!("expected partial coverage, got {other:?}"),
        }
        // The whole-coverage seeds are unaffected.
        assert_eq!(
            c.member("jurisdiction/city-corp.yml").unwrap().extent,
            Extent::Whole
        );
    }

    #[test]
    fn partial_coverage_of_a_place_does_not_make_its_county_partial() {
        // Sugar Creek Township, exactly. A tract holding four fifths of the township is
        // `nested-in` the county government, and the county also reaches the township by the
        // ordinary containment path. Part of the township being inside the tract puts that part
        // inside the county too; it says nothing that limits the county's own coverage.
        let mut g = fixture();
        g.insert(node(
            "division/tract.yml",
            "division",
            &[("division_type", "census tract")],
            &[
                ("partially-covers", "place/city.yml"),
                ("nested-in", "jurisdiction/county-gov.yml"),
            ],
        ));
        let c = covering(&g, "place/city.yml", None).unwrap();
        assert!(c.member("division/tract.yml").unwrap().extent.is_partial());
        assert_eq!(
            c.member("jurisdiction/county-gov.yml").unwrap().extent,
            Extent::Whole,
            "the county covers this ground whole; the tract's partial reach is not its own"
        );
    }

    #[test]
    fn a_partial_coverer_reaching_outside_the_county_does_not_split_the_county() {
        // Lima in miniature, and the case that changed this rule. Shawnee Local School District
        // covers part of the city and is `partially-within` the county because four of its
        // blocks are in Auglaize. Both edges are right. Neither says anything about Lima, which
        // is wholly inside Allen County — and an answer that reported the county as covering
        // the city partially would be reading a fact about the district's territory as a fact
        // about the city's.
        let mut g = fixture();
        g.insert(node(
            "jurisdiction/school-district.yml",
            "jurisdiction",
            &[("jurisdiction_type", "school district")],
            &[
                ("partially-covers", "place/city.yml"),
                ("partially-within", "jurisdiction/county-gov.yml"),
            ],
        ));
        let c = covering(&g, "place/city.yml", None).unwrap();
        assert!(c
            .member("jurisdiction/school-district.yml")
            .unwrap()
            .extent
            .is_partial());
        assert_eq!(
            c.member("jurisdiction/county-gov.yml").unwrap().extent,
            Extent::Whole,
            "the district's reach outside the county limits the district, not the city"
        );
    }

    /// Delphos in miniature. The only route from the place to the county government runs
    /// through a district that covers the *county* — so the first step is already an
    /// inference, and following an authority edge out of it must not launder that into an
    /// assertion.
    #[test]
    fn nesting_out_of_an_inherited_member_stays_inherited() {
        let mut g = Graph::default();
        g.insert(node(
            "place/town.yml",
            "place",
            &[],
            &[("within", "place/county.yml")],
        ));
        g.insert(node("place/county.yml", "place", &[], &[]));
        g.insert(node(
            "division/district.yml",
            "division",
            &[("division_type", "state legislative district")],
            &[
                ("covers", "place/county.yml"),
                ("nested-in", "jurisdiction/county-gov.yml"),
            ],
        ));
        g.insert(node(
            "jurisdiction/county-gov.yml",
            "jurisdiction",
            &[("jurisdiction_type", "county")],
            &[],
        ));

        let c = covering(&g, "place/town.yml", None).unwrap();
        let m = c.member("jurisdiction/county-gov.yml").unwrap();
        assert_eq!(
            m.reach,
            Reach::Inherited,
            "a chain is only as strong as its weakest step"
        );
    }
}
