//! Where a corpus node lands on the ground, by what route, and where it refuses to guess.
//!
//! 72 of this corpus's 667 nodes carry a position of their own. Almost every other node sits
//! within a few edges of one — a measure `describes` a place, an event `occurred-in` one, a
//! person `resided-in` one, a tenure reaches ground through its office — so a map can draw far
//! more than the 37 points it draws today. What that costs is stated here rather than
//! discovered later: **every derived placement is an inference the map is making, not a fact
//! the corpus recorded**, and it travels with the route that produced it.
//!
//! # The half worth writing down is the refusal
//!
//! `proximity` learned this the expensive way about distance and containment, and the same
//! shape recurs here: an edge that reaches ground is not the same thing as an edge that
//! *places*. [`ROUTES`] classifies all 50 class-and-relationship pairs the corpus uses, and
//! twenty-four of them are refused with the reason attached.
//!
//! The case that decides the rule is `event --affected-> place`, and this repository's own
//! `AGENTS.md` states it: *"`occurred-in` is where it happened; `affected` is what it changed.
//! The Treaty of St. Marys was signed in another county and made this one. Collapsing the two
//! would put the county's founding somewhere it did not happen."* Refusing it leaves nine
//! events unplaced, among them the Treaty and the Erection of Allen County — which is the
//! correct answer, because the county's founding did not happen in the county.
//!
//! # A mark is not the only thing a node can be
//!
//! Two tests decide what a placed node becomes, and both are needed.
//!
//! **Is it a thing that was somewhere, or a figure about somewhere?** An event `occurred-in` a
//! place and a site is `located-in` one: those are claims about position. A measure only
//! `describes` a place: that is a claim about a subject which happens to have a position. The
//! first earns a mark; the second is a count on the thing it is about — which is also the only
//! rule that survives the arithmetic, since 374 counts resolve onto a few dozen subjects and
//! drawn individually they are a smear that no zoom separates.
//!
//! **Does the placement discriminate?** `map.ts` already refuses a placement that does not,
//! for labels, in these words: *"its extent is the whole frame, so a label at its centroid
//! names Lima's neighbourhood rather than the county."* 56 of the corpus's 99 people are
//! recorded as having resided in Allen County and nowhere finer. A pin at the county centroid
//! is a spot in a field, so those become a register beside the map rather than marks on it.
//!
//! The second test applies to marks only. A county-level measure is a fact *about the county*
//! and belongs on the county, so a count is drawn on its subject whatever that subject is.

pub mod load;

pub use load::{Graph, Link, Node};

use std::collections::{BTreeMap, BTreeSet};

// ── anchors ──────────────────────────────────────────────────────────────────

/// A position the corpus states outright.
#[derive(Debug, Clone, PartialEq)]
pub enum Anchor {
    /// A literal coordinate — `place.centroid` or `site.coordinates`.
    Point { lat: f64, lon: f64 },
    /// A Census key — `place.geoid` or `jurisdiction.fips_code`.
    ///
    /// The corpus holds the key and never the shape. The geometry is vendored under
    /// `web/public/geo/` and joined there, which is the one derivation the site performs and
    /// the one thing `web/test/geography.test.ts` gates. Nothing in this crate pretends to
    /// know where a polygon is.
    Census { key: String },
}

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Anchor::Point { lat, lon } => write!(f, "{lat}, {lon}"),
            Anchor::Census { key } => write!(f, "Census {key}"),
        }
    }
}

/// Read `"40.740679, -84.112091"`. Anything else is `None` rather than a guess.
pub fn parse_point(raw: &str) -> Option<Anchor> {
    let (lat, lon) = raw.split_once(',')?;
    let lat: f64 = lat.trim().parse().ok()?;
    let lon: f64 = lon.trim().parse().ok()?;
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    Some(Anchor::Point { lat, lon })
}

/// The position a node states about itself, if it states one.
///
/// A point beats a Census key where a node has both: the key names a shape this crate cannot
/// see, and a coordinate is the more specific of the two claims.
pub fn own_anchor(node: &Node) -> Option<Anchor> {
    let point = node
        .properties
        .get("centroid")
        .or_else(|| node.properties.get("coordinates"))
        .and_then(|v| parse_point(v));
    point.or_else(|| {
        node.properties
            .get("geoid")
            .or_else(|| node.properties.get("fips_code"))
            .map(|k| Anchor::Census { key: k.clone() })
    })
}

// ── the route table ──────────────────────────────────────────────────────────

/// Whether an edge carries a placement claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Routing {
    /// Following this edge places the node at its target.
    Places,
    /// This edge reaches another node and says nothing about where the source is.
    Refused,
}

/// One class-and-relationship pair, and whether it places.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Route {
    pub class: &'static str,
    pub relationship: &'static str,
    pub routing: Routing,
    /// Why. Printed by `where-is --routes`, so a refusal can be argued with.
    pub because: &'static str,
}

/// Every class-and-relationship pair the corpus uses, classified.
///
/// Gated in both directions by `tests/corpus.rs`: a pair occurring in the corpus and missing
/// here fails the build, and an entry naming a pair the corpus no longer uses fails it too.
/// The second half is the one that survives time, for the reason `chronology::POLICY`,
/// `design/departures.md` and `.yidam/lint-baseline.yml` all give — a list of exceptions
/// permitted to be wrong drifts, and one that over-lists silently re-permits whatever a later
/// edit puts in its place.
///
/// `instance-of` is not here and never will be: it points at a class contract, which is not a
/// place and not an instance.
pub const ROUTES: &[Route] = &[
    // ── placing ──────────────────────────────────────────────────────────────
    Route {
        class: "division",
        relationship: "covers",
        routing: Routing::Places,
        because: "The ground a division is drawn over is where it is.",
    },
    Route {
        class: "division",
        relationship: "nested-in",
        routing: Routing::Places,
        because: "A precinct sits inside the unit that drew it.",
    },
    Route {
        class: "division",
        relationship: "partially-covers",
        routing: Routing::Places,
        because: "Partial ground is still ground. The partiality belongs on the drawing, not \
                  on whether the division is there at all.",
    },
    Route {
        class: "event",
        relationship: "occurred-at",
        routing: Routing::Places,
        because: "The built work it happened at. The strongest placement an event has.",
    },
    Route {
        class: "event",
        relationship: "occurred-in",
        routing: Routing::Places,
        because: "Where it happened, as against what it changed — the distinction the whole \
                  table turns on.",
    },
    Route {
        class: "jurisdiction",
        relationship: "covers",
        routing: Routing::Places,
        because: "A unit of government is over the ground it covers.",
    },
    Route {
        class: "jurisdiction",
        relationship: "partially-covers",
        routing: Routing::Places,
        because: "Same reading as `covers`; the partiality is a fact about the boundary.",
    },
    Route {
        class: "jurisdiction",
        relationship: "partially-within",
        routing: Routing::Places,
        because: "Territory inside another unit's territory is territory.",
    },
    Route {
        class: "jurisdiction",
        relationship: "serves",
        routing: Routing::Places,
        because: "A school district serving a place has territory there. It usually serves \
                  several, which is why a placement may reach more than one anchor.",
    },
    Route {
        class: "jurisdiction",
        relationship: "territory-within",
        routing: Routing::Places,
        because: "Nesting of ground, which the class's own description says is what this edge \
                  records.",
    },
    Route {
        class: "measure",
        relationship: "concerns",
        routing: Routing::Places,
        because: "A coinage outside the class's declared edges, used 35 times and always the \
                  same way: the sites a figure is about. Toxic releases concern the refinery \
                  and the engine plant, which is where the releases were. Same reading as \
                  `describes`, and it is what gives those figures a subject finer than the \
                  county.",
    },
    Route {
        class: "measure",
        relationship: "describes",
        routing: Routing::Places,
        because: "The subject a figure is about. Not a claim that the figure is anywhere — \
                  which is why a measure is drawn as a count on its subject and never as a mark.",
    },
    Route {
        class: "natural-feature",
        relationship: "partially-covers",
        routing: Routing::Places,
        because: "A basin lying over part of a place is over it.",
    },
    Route {
        class: "natural-feature",
        relationship: "traverses",
        routing: Routing::Places,
        because: "A stream passing through a place runs there. A representative point is a \
                  poor drawing of a line, and phase V draws the line.",
    },
    Route {
        class: "natural-feature",
        relationship: "within",
        routing: Routing::Places,
        because: "A feature inside a named place is there.",
    },
    Route {
        class: "office",
        relationship: "established-within",
        routing: Routing::Places,
        because: "The unit of government whose office it is. An office has no address of its \
                  own and this is the only ground it has.",
    },
    Route {
        class: "organization",
        relationship: "seated-in",
        routing: Routing::Places,
        because: "Where it is or was headquartered — the class's own word for its location.",
    },
    Route {
        class: "person",
        relationship: "resided-in",
        routing: Routing::Places,
        because: "A place this person lived. Weak but real: 56 of the 99 resolve no finer \
                  than the county, and those become register entries rather than marks.",
    },
    Route {
        class: "place",
        relationship: "governed-by",
        routing: Routing::Places,
        because: "Ground under an authority is ground. Reached only where the place itself \
                  carries no centroid, which is rare.",
    },
    Route {
        class: "place",
        relationship: "partially-within",
        routing: Routing::Places,
        because: "A place straddling a line is on both sides of it.",
    },
    Route {
        class: "place",
        relationship: "within",
        routing: Routing::Places,
        because: "Ground inside larger ground.",
    },
    Route {
        class: "question",
        relationship: "concerns",
        routing: Routing::Places,
        because: "What the question is about, used 54 times and consistently in that \
                  direction. An open question about Lima's lending is a question about Lima's \
                  ground, and a corpus that can show where its own questions are is showing \
                  something worth seeing.",
    },
    Route {
        class: "site",
        relationship: "located-in",
        routing: Routing::Places,
        because: "The named place the work stands in. `formerly-in` is the one that does not \
                  place, and it is refused below.",
    },
    Route {
        class: "tenure",
        relationship: "of-office",
        routing: Routing::Places,
        because: "A term reaches ground through the office it is a term of, and the office \
                  through the jurisdiction it was established within. Two hops and no guess at \
                  either.",
    },
    // ── refused ──────────────────────────────────────────────────────────────
    Route {
        class: "event",
        relationship: "affected",
        routing: Routing::Refused,
        because: "What it changed, not where it was. `AGENTS.md` states the case: the Treaty \
                  of St. Marys was signed in another county and made this one, and collapsing \
                  the two would put the county's founding somewhere it did not happen. This \
                  refusal leaves nine events unplaced, which is the right number.",
    },
    Route {
        class: "event",
        relationship: "involved",
        routing: Routing::Refused,
        because: "A participant is not a location. An event involving a man who lived in Lima \
                  did not necessarily happen in Lima.",
    },
    Route {
        class: "event",
        relationship: "relates-to",
        routing: Routing::Refused,
        because: "Associative, and here associative between two happenings. Two storms of one \
                  evening are joined by this edge and the whole question about them is whether \
                  they were in the same place; placing one from the other would answer it by \
                  assumption.",
    },
    Route {
        class: "event",
        relationship: "situated-in",
        routing: Routing::Refused,
        because: "Points at a period. Temporal, and `chronology` owns it.",
    },
    Route {
        class: "jurisdiction",
        relationship: "erected-by",
        routing: Routing::Refused,
        because: "Points at the act that created it. Temporal, and the act often happened \
                  elsewhere — see `event --affected->`.",
    },
    Route {
        class: "jurisdiction",
        relationship: "evidenced-by",
        routing: Routing::Refused,
        because: "Provenance. A figure supporting a claim is not where the claim's subject is.",
    },
    Route {
        class: "measure",
        relationship: "comparable-to",
        routing: Routing::Refused,
        because: "A judgement about two figures, not about either one's subject. Both ends \
                  usually describe the same ground anyway, so following it would place a \
                  figure where `describes` already placed it.",
    },
    Route {
        class: "measure",
        relationship: "not-comparable-to",
        routing: Routing::Refused,
        because: "A break in series, and the tempting one: a break is often caused by ground \
                  moving — a corporation line that grew, a city entering a township table. It \
                  still names two figures and not the annexation between them, so following it \
                  would place the 1930 township count on the county the 1910 one describes and \
                  call that a position.",
    },
    Route {
        class: "measure",
        relationship: "relates-to",
        routing: Routing::Refused,
        because: "The associative edge, not the subject edge. `describes` says what a figure \
                  is about; `relates-to` says what it is worth reading beside, and 364 of them \
                  would drag every figure onto every subject it was ever compared with.",
    },
    Route {
        class: "natural-feature",
        relationship: "evidenced-by",
        routing: Routing::Refused,
        because: "Provenance.",
    },
    Route {
        class: "natural-feature",
        relationship: "flows-into",
        routing: Routing::Refused,
        because: "Hydrology. A creek is not located at its mouth — that is one end of it, and \
                  drawing the creek there would put every tributary in the same river.",
    },
    Route {
        class: "organization",
        relationship: "leased-to",
        routing: Routing::Refused,
        because: "A lessor still owns the road and is not at the lessee's address. The whole \
                  point of the edge is that the two bodies stayed distinct.",
    },
    Route {
        class: "period",
        relationship: "concentrated-in",
        routing: Routing::Refused,
        because: "A period is not drawn on the ground at all. It drives the time control, \
                  which is what `chronology` and the era spine are for.",
    },
    Route {
        class: "period",
        relationship: "evidenced-by",
        routing: Routing::Refused,
        because: "Provenance, and a period is not placed regardless.",
    },
    Route {
        class: "period",
        relationship: "precedes",
        routing: Routing::Refused,
        because: "Temporal.",
    },
    Route {
        class: "person",
        relationship: "affiliated-with",
        routing: Routing::Refused,
        because: "A person is not at their employer. A man on a company's board did not live \
                  at its works.",
    },
    Route {
        class: "person",
        relationship: "relates-to",
        routing: Routing::Refused,
        because: "Associative.",
    },
    Route {
        class: "place",
        relationship: "evidenced-by",
        routing: Routing::Refused,
        because: "Provenance.",
    },
    Route {
        class: "place",
        relationship: "named-for",
        routing: Routing::Refused,
        because: "Amanda Township is named for a fort that has not been in this county since \
                  1848. Placing a thing at its namesake would move the township into the next \
                  county — see `.yidam/decisions/of-here-is-not-located-here.yml`.",
    },
    Route {
        class: "question",
        relationship: "relates-to",
        routing: Routing::Refused,
        because: "Associative, and the only outgoing edge any question in this corpus has. \
                  `concerns` is declared with eleven target classes and used zero times, which \
                  is why all 16 questions are unplaced.",
    },
    Route {
        class: "person",
        relationship: "subject-of",
        routing: Routing::Refused,
        because: "See `question --subject-of->`. A person is not located at a question in any \
                  case.",
    },
    Route {
        class: "measure",
        relationship: "subject-of",
        routing: Routing::Refused,
        because: "See `question --subject-of->`.",
    },
    Route {
        class: "question",
        relationship: "subject-of",
        routing: Routing::Refused,
        because: "Refused because its direction is not consistent. All three uses in the \
                  corpus read differently: `person subject-of question` says the person is the \
                  subject, while `question subject-of jurisdiction` and `measure subject-of \
                  person` say the target is. A placement route needs a direction that holds, \
                  and `concerns` already carries this claim unambiguously 89 times.",
    },
    Route {
        class: "site",
        relationship: "formerly-in",
        routing: Routing::Refused,
        because: "A place this work stood in and stands in no longer. The edge exists \
                  precisely because `located-in` carries no date and would assert present \
                  containment; routing through it would make the same error the edge was \
                  declared to avoid. Phase V draws it as a line that crosses the county \
                  boundary, which is what it is.",
    },
    Route {
        class: "site",
        relationship: "operated-by",
        routing: Routing::Refused,
        because: "A plant is not at its operator's address. The class exists because a built \
                  work outlasts the bodies that run it.",
    },
    Route {
        class: "site",
        relationship: "relates-to",
        routing: Routing::Refused,
        because: "Associative.",
    },
    Route {
        class: "tenure",
        relationship: "held-by",
        routing: Routing::Refused,
        because: "A term is not at its holder's residence. `of-office` is the route, and it \
                  reaches the ground the office actually governs.",
    },
];

/// How a class-and-relationship pair routes, or `None` if the table does not name it.
pub fn route_for(class: &str, relationship: &str) -> Option<&'static Route> {
    ROUTES
        .iter()
        .find(|r| r.class == class && r.relationship == relationship)
}

fn places(class: &str, relationship: &str) -> bool {
    route_for(class, relationship).is_some_and(|r| r.routing == Routing::Places)
}

// ── the frame ────────────────────────────────────────────────────────────────

/// The nodes whose extent is the whole map.
///
/// A placement that reaches only these says nothing a map of Allen County is not already
/// saying. `map.ts` excludes the county from its label layer for the same reason and in almost
/// the same words.
pub const FRAME: &[&str] = &[
    "place/allen-county.yml",
    "jurisdiction/allen-county-government.yml",
];

// ── what a placement is ──────────────────────────────────────────────────────

/// One edge followed, with the claim tag it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub relationship: String,
    pub to: String,
    /// `verified`, `inference`, or `None` for a structural edge that carries no tag.
    pub tag: Option<String>,
}

/// One anchored node a placement reached, and the path that reached it.
#[derive(Debug, Clone, PartialEq)]
pub struct Reached {
    pub node: String,
    pub anchor: Anchor,
    /// Empty when the node is its own anchor.
    pub via: Vec<Step>,
}

/// Where a node lands, and the warrant for landing it there.
#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    /// Edges followed. `0` means the node carries its own position.
    pub hops: usize,
    /// Every anchor reached at `hops`.
    ///
    /// More than one is not a defect and must not be collapsed to a centroid: a school
    /// district covers five townships and is at all five. A caller drawing a single mark
    /// should say so rather than average them.
    pub reached: Vec<Reached>,
    /// The weakest claim tag on any path — `inference` if any step is inferred, `verified`
    /// only if every tagged step is. `None` where no step carried a tag.
    pub tag: Option<String>,
}

impl Placement {
    /// Whether any anchor reached is smaller than the whole frame.
    pub fn discriminates(&self) -> bool {
        self.reached
            .iter()
            .any(|r| !FRAME.contains(&r.node.as_str()))
    }
}

/// What the map should do with a node.
///
/// Declaration order is how much of a position the corpus supplies, most first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Treatment {
    /// A thing that was somewhere, placed somewhere smaller than the county.
    Mark,
    /// Covers ground rather than sitting on it. Drawn from its Census key.
    Polygon,
    /// A figure about somewhere — drawn on its subject, at whatever grain that subject is.
    Count,
    /// A thing that was somewhere, and the corpus places it no finer than the county.
    /// Listed beside the map for the year, never pinned in a field.
    Register,
    /// No licensed route to ground.
    Unplaced,
    /// Never drawn on the ground. `period` drives the time control instead.
    NotSpatial,
}

impl std::fmt::Display for Treatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Treatment::Mark => "mark",
            Treatment::Polygon => "polygon",
            Treatment::Count => "count on its subject",
            Treatment::Register => "countywide register",
            Treatment::Unplaced => "unplaced",
            Treatment::NotSpatial => "not spatial",
        };
        write!(f, "{s}")
    }
}

/// Classes that are things standing somewhere, as against figures about somewhere.
const MARK_CLASSES: &[&str] = &[
    "place",
    "site",
    "natural-feature",
    "event",
    "person",
    "organization",
];
/// Classes that cover ground rather than sit on it.
const POLYGON_CLASSES: &[&str] = &["jurisdiction", "division"];

/// A node, placed.
#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    pub node: String,
    pub class: String,
    pub label: String,
    pub placement: Option<Placement>,
    pub treatment: Treatment,
}

/// The most conservative of two claim tags.
fn weaker(a: Option<String>, b: Option<&String>) -> Option<String> {
    let rank = |t: &str| match t {
        "verified" => 0,
        "inference" => 1,
        _ => 2,
    };
    match (a, b) {
        (None, x) => x.cloned(),
        (Some(x), None) => Some(x),
        (Some(x), Some(y)) => {
            if rank(&y[..]) > rank(&x[..]) {
                Some(y.clone())
            } else {
                Some(x)
            }
        }
    }
}

/// The maximum edges followed before giving up.
///
/// Three, and nothing in the corpus needs a fourth: the longest real route is a tenure to its
/// office to the jurisdiction to a Census key. A deeper walk buys nothing and would make the
/// warrant unreadable — a chain nobody can check is not a warrant.
pub const MAX_HOPS: usize = 3;

/// Resolve one node to ground.
///
/// Breadth-first over licensed edges only, stopping at the first hop that reaches any anchor
/// and returning **every** anchor found at that depth. Returns `None` where no licensed route
/// reaches one within [`MAX_HOPS`].
pub fn place(graph: &Graph, id: &str) -> Option<Placement> {
    let node = graph.get(id)?;
    if let Some(anchor) = own_anchor(node) {
        return Some(Placement {
            hops: 0,
            reached: vec![Reached {
                node: id.to_string(),
                anchor,
                via: Vec::new(),
            }],
            tag: None,
        });
    }

    let mut seen: BTreeSet<&str> = BTreeSet::from([id]);
    // Each frontier entry is a node and the path that got there.
    let mut frontier: Vec<(&str, Vec<Step>)> = vec![(id, Vec::new())];

    for hop in 1..=MAX_HOPS {
        let mut next: Vec<(&str, Vec<Step>)> = Vec::new();
        let mut hits: BTreeMap<&str, (Anchor, Vec<Step>)> = BTreeMap::new();

        for (cur, path) in &frontier {
            let Some(from) = graph.get(cur) else { continue };
            for link in &from.links {
                if !places(&from.class, &link.relationship) {
                    continue;
                }
                let Some(target) = graph.get(&link.target) else {
                    continue;
                };
                let mut path = path.clone();
                path.push(Step {
                    relationship: link.relationship.clone(),
                    to: link.target.clone(),
                    tag: link.claim_tag.clone(),
                });
                match own_anchor(target) {
                    Some(anchor) => {
                        hits.entry(target.id.as_str()).or_insert((anchor, path));
                    }
                    None => {
                        if seen.insert(target.id.as_str()) {
                            next.push((target.id.as_str(), path));
                        }
                    }
                }
            }
        }

        if !hits.is_empty() {
            let mut tag = None;
            let reached: Vec<Reached> = hits
                .into_iter()
                .map(|(node, (anchor, via))| {
                    for step in &via {
                        tag = weaker(tag.take(), step.tag.as_ref());
                    }
                    Reached {
                        node: node.to_string(),
                        anchor,
                        via,
                    }
                })
                .collect();
            return Some(Placement {
                hops: hop,
                reached,
                tag,
            });
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    None
}

/// What the map does with a node, given where it landed.
pub fn treatment(class: &str, placement: Option<&Placement>) -> Treatment {
    if class == "period" {
        return Treatment::NotSpatial;
    }
    let Some(p) = placement else {
        return Treatment::Unplaced;
    };
    if POLYGON_CLASSES.contains(&class) {
        return Treatment::Polygon;
    }
    if MARK_CLASSES.contains(&class) {
        // The second test, and it applies to marks only: a count is drawn on its subject at
        // whatever grain that subject is, and a county-level figure belongs on the county.
        return if p.discriminates() {
            Treatment::Mark
        } else {
            Treatment::Register
        };
    }
    Treatment::Count
}

/// Place every node in the graph.
pub fn place_all(graph: &Graph) -> Vec<Placed> {
    let mut out: Vec<Placed> = graph
        .nodes()
        .map(|n| {
            let placement = place(graph, &n.id);
            let treatment = treatment(&n.class, placement.as_ref());
            Placed {
                node: n.id.clone(),
                class: n.class.clone(),
                label: n.label.clone(),
                placement,
                treatment,
            }
        })
        .collect();
    out.sort_by(|a, b| a.node.cmp(&b.node));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, class: &str, props: &[(&str, &str)], links: &[(&str, &str)]) -> Node {
        Node {
            id: id.to_string(),
            class: class.to_string(),
            label: String::new(),
            properties: props
                .iter()
                .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
                .collect(),
            links: links
                .iter()
                .map(|(rel, target)| Link {
                    relationship: (*rel).to_string(),
                    target: (*target).to_string(),
                    claim_tag: Some("verified".to_string()),
                })
                .collect(),
        }
    }

    fn graph(nodes: Vec<Node>) -> Graph {
        let mut g = Graph::default();
        for n in nodes {
            g.insert(n);
        }
        g
    }

    #[test]
    fn a_node_with_a_centroid_is_its_own_anchor() {
        let g = graph(vec![node(
            "place/lima.yml",
            "place",
            &[("centroid", "40.740679, -84.112091")],
            &[],
        )]);
        let p = place(&g, "place/lima.yml").unwrap();
        assert_eq!(p.hops, 0);
        assert!(p.reached[0].via.is_empty());
    }

    #[test]
    fn a_point_beats_a_census_key_on_the_same_node() {
        let n = node(
            "place/lima.yml",
            "place",
            &[("centroid", "40.7, -84.1"), ("geoid", "3943554")],
            &[],
        );
        assert!(matches!(own_anchor(&n), Some(Anchor::Point { .. })));
    }

    #[test]
    fn a_measure_reaches_the_place_it_describes() {
        let g = graph(vec![
            node(
                "measure/lima-population-2020.yml",
                "measure",
                &[],
                &[("describes", "place/lima.yml")],
            ),
            node(
                "place/lima.yml",
                "place",
                &[("centroid", "40.7, -84.1")],
                &[],
            ),
        ]);
        let p = place(&g, "measure/lima-population-2020.yml").unwrap();
        assert_eq!(p.hops, 1);
        assert_eq!(p.reached[0].node, "place/lima.yml");
        assert_eq!(p.reached[0].via[0].relationship, "describes");
    }

    #[test]
    fn an_event_is_not_placed_where_it_had_effects() {
        // The Treaty of St. Marys shape: signed elsewhere, made this county. Following
        // `affected` would put the county's founding inside the county.
        let g = graph(vec![
            node(
                "event/treaty-of-st-marys.yml",
                "event",
                &[],
                &[("affected", "place/allen-county.yml")],
            ),
            node(
                "place/allen-county.yml",
                "place",
                &[("centroid", "40.77, -84.10")],
                &[],
            ),
        ]);
        assert_eq!(place(&g, "event/treaty-of-st-marys.yml"), None);
    }

    #[test]
    fn a_site_is_not_placed_where_it_used_to_be() {
        // `formerly-in` exists because `located-in` carries no date. Routing through it would
        // make exactly the error the edge was declared to avoid.
        let g = graph(vec![
            node(
                "site/fort-amanda.yml",
                "site",
                &[],
                &[("formerly-in", "place/amanda-township.yml")],
            ),
            node(
                "place/amanda-township.yml",
                "place",
                &[("centroid", "40.73, -84.27")],
                &[],
            ),
        ]);
        assert_eq!(place(&g, "site/fort-amanda.yml"), None);
    }

    #[test]
    fn a_tenure_reaches_ground_through_its_office_and_not_its_holder() {
        let g = graph(vec![
            node(
                "tenure/x.yml",
                "tenure",
                &[],
                &[
                    ("held-by", "person/somebody.yml"),
                    ("of-office", "office/sheriff.yml"),
                ],
            ),
            node(
                "person/somebody.yml",
                "person",
                &[("centroid", "1.0, 1.0")],
                &[],
            ),
            node(
                "office/sheriff.yml",
                "office",
                &[],
                &[("established-within", "jurisdiction/county.yml")],
            ),
            node(
                "jurisdiction/county.yml",
                "jurisdiction",
                &[("fips_code", "39003")],
                &[],
            ),
        ]);
        let p = place(&g, "tenure/x.yml").unwrap();
        assert_eq!(
            p.hops, 2,
            "held-by is refused, so the person is not reached"
        );
        assert_eq!(p.reached[0].node, "jurisdiction/county.yml");
    }

    #[test]
    fn every_anchor_at_the_shallowest_depth_is_returned() {
        // A school district serving three places is at all three, and averaging them would
        // invent a centroid the corpus never stated.
        let g = graph(vec![
            node(
                "jurisdiction/school.yml",
                "jurisdiction",
                &[],
                &[
                    ("serves", "place/a.yml"),
                    ("serves", "place/b.yml"),
                    ("serves", "place/c.yml"),
                ],
            ),
            node("place/a.yml", "place", &[("centroid", "1.0, 1.0")], &[]),
            node("place/b.yml", "place", &[("centroid", "2.0, 2.0")], &[]),
            node("place/c.yml", "place", &[("centroid", "3.0, 3.0")], &[]),
        ]);
        let p = place(&g, "jurisdiction/school.yml").unwrap();
        assert_eq!(p.reached.len(), 3);
    }

    #[test]
    fn a_placement_reaching_only_the_county_does_not_discriminate() {
        let g = graph(vec![
            node(
                "person/x.yml",
                "person",
                &[],
                &[("resided-in", "place/allen-county.yml")],
            ),
            node(
                "place/allen-county.yml",
                "place",
                &[("centroid", "40.77, -84.10")],
                &[],
            ),
        ]);
        let p = place(&g, "person/x.yml").unwrap();
        assert!(!p.discriminates());
        assert_eq!(treatment("person", Some(&p)), Treatment::Register);
    }

    #[test]
    fn a_count_is_drawn_on_its_subject_even_when_that_subject_is_the_county() {
        // The discrimination test is for marks. A county population figure is a fact about the
        // county and belongs on it; sending it to the register would file the corpus's most
        // ordinary claim under "could not be placed".
        let g = graph(vec![
            node(
                "measure/allen-county-population-2020.yml",
                "measure",
                &[],
                &[("describes", "place/allen-county.yml")],
            ),
            node(
                "place/allen-county.yml",
                "place",
                &[("centroid", "40.77, -84.10")],
                &[],
            ),
        ]);
        let p = place(&g, "measure/allen-county-population-2020.yml").unwrap();
        assert!(!p.discriminates());
        assert_eq!(treatment("measure", Some(&p)), Treatment::Count);
    }

    #[test]
    fn a_period_is_never_placed_even_when_an_edge_would_reach_ground() {
        assert_eq!(treatment("period", None), Treatment::NotSpatial);
        let g = graph(vec![
            node(
                "period/oil-boom.yml",
                "period",
                &[],
                &[("concentrated-in", "place/lima.yml")],
            ),
            node(
                "place/lima.yml",
                "place",
                &[("centroid", "40.7, -84.1")],
                &[],
            ),
        ]);
        assert_eq!(place(&g, "period/oil-boom.yml"), None);
    }

    #[test]
    fn the_weakest_tag_on_the_chain_is_the_one_carried() {
        let mut n = node(
            "measure/x.yml",
            "measure",
            &[],
            &[("describes", "place/lima.yml")],
        );
        n.links[0].claim_tag = Some("inference".to_string());
        let g = graph(vec![
            n,
            node("place/lima.yml", "place", &[("centroid", "1.0, 1.0")], &[]),
        ]);
        assert_eq!(
            place(&g, "measure/x.yml").unwrap().tag.as_deref(),
            Some("inference")
        );
    }

    #[test]
    fn a_coordinate_that_is_not_a_coordinate_is_not_a_guess() {
        for bad in [
            "",
            "somewhere in Bath Township",
            "40.7",
            "91.0, 0.0",
            "0.0, 181.0",
        ] {
            assert_eq!(parse_point(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn routes_are_coherent() {
        let mut seen = BTreeSet::new();
        for r in ROUTES {
            assert!(
                seen.insert((r.class, r.relationship)),
                "{} --{}-> is listed twice",
                r.class,
                r.relationship
            );
            assert!(
                !r.because.trim().is_empty(),
                "{} --{}-> states no argument",
                r.class,
                r.relationship
            );
            assert_ne!(
                r.relationship, "instance-of",
                "instance-of points at a class contract, not a place"
            );
        }
    }
}
