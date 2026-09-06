//! Runs the resolver against this repository's own corpus.
//!
//! Same posture as `chronology/tests/corpus.rs`: what is pinned is the **route table's
//! agreement with the corpus**, plus the refusals that would be invisible if they broke. Node
//! counts are not pinned — a test reading `assert_eq!(marks, 129)` fails on the next phase that
//! adds a site, and a gate that fails for the wrong reason gets switched off.
//!
//! The one exception is the shape of the distribution, which is checked as an ordering rather
//! than as numbers: counts outnumber marks, and marks outnumber the unplaced. That is the
//! finding the whole design rests on, and it is stable under ordinary corpus growth.

use placement::{load, place, place_all, route_for, Graph, Placed, Routing, Treatment, ROUTES};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

fn graph() -> Graph {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.yidam/corpus");
    load::graph(&dir).expect("the corpus loads")
}

fn placed() -> Vec<Placed> {
    place_all(&graph())
}

fn by_treatment(all: &[Placed]) -> BTreeMap<Treatment, usize> {
    let mut m = BTreeMap::new();
    for p in all {
        *m.entry(p.treatment).or_insert(0) += 1;
    }
    m
}

/// Every class-and-relationship pair the corpus actually uses.
fn pairs_in_corpus(g: &Graph) -> BTreeSet<(String, String)> {
    let mut out = BTreeSet::new();
    for n in g.nodes() {
        for l in &n.links {
            out.insert((n.class.clone(), l.relationship.clone()));
        }
    }
    out
}

#[test]
fn every_edge_the_corpus_uses_has_been_classified() {
    let g = graph();
    let missing: Vec<String> = pairs_in_corpus(&g)
        .into_iter()
        .filter(|(c, r)| route_for(c, r).is_none())
        .map(|(c, r)| format!("{c} --{r}->"))
        .collect();
    assert!(
        missing.is_empty(),
        "these edges are in the corpus and not in ROUTES:\n  {}\n\
         Decide whether each places or is refused, with the argument beside it.",
        missing.join("\n  ")
    );
}

#[test]
fn no_route_outlives_the_edge_it_describes() {
    // The anti-drift half. An entry for a pair the corpus stopped using is a rule nobody can
    // check, and a table permitted to over-list re-permits whatever a later edit puts there.
    let g = graph();
    let used = pairs_in_corpus(&g);
    let stale: Vec<String> = ROUTES
        .iter()
        .filter(|r| !used.contains(&(r.class.to_string(), r.relationship.to_string())))
        .map(|r| format!("{} --{}->", r.class, r.relationship))
        .collect();
    assert!(
        stale.is_empty(),
        "ROUTES names these and the corpus no longer uses them:\n  {}",
        stale.join("\n  ")
    );
}

#[test]
fn the_countys_founding_is_not_placed_inside_the_county() {
    // The case `AGENTS.md` names, checked against the two nodes it names. Both reach ground
    // only through `affected`, and refusing that edge is what keeps them off the map.
    let g = graph();
    for id in [
        "event/treaty-of-st-marys.yml",
        "event/erection-of-allen-county.yml",
    ] {
        let node = g.get(id).unwrap_or_else(|| panic!("{id} is in the corpus"));
        assert!(
            node.links.iter().any(|l| l.relationship == "affected"),
            "{id} should reach ground through `affected`"
        );
        assert!(
            place(&g, id).is_none(),
            "{id} was placed; `affected` is what it changed, not where it was"
        );
    }
}

#[test]
fn a_work_is_not_placed_where_it_used_to_stand() {
    // Fort Amanda has been outside this county since 1848 and carries the corpus's only
    // `formerly-in` edge. The edge exists because `located-in` carries no date; routing
    // through it would make exactly the error it was declared to avoid.
    let g = graph();
    let fort = g
        .nodes()
        .find(|n| n.links.iter().any(|l| l.relationship == "formerly-in"))
        .expect("the corpus has a formerly-in edge");
    let p = place(&g, &fort.id);
    if let Some(p) = p {
        for r in &p.reached {
            for step in &r.via {
                assert_ne!(
                    step.relationship, "formerly-in",
                    "{} was placed through formerly-in",
                    fort.id
                );
            }
        }
    }
}

#[test]
fn no_placement_runs_deeper_than_the_warrant_can_be_read() {
    for p in placed() {
        if let Some(pl) = &p.placement {
            assert!(
                pl.hops <= placement::MAX_HOPS,
                "{}: {} hops",
                p.node,
                pl.hops
            );
            for r in &pl.reached {
                assert_eq!(
                    r.via.len(),
                    pl.hops,
                    "{}: chain length disagrees with hops",
                    p.node
                );
            }
        }
    }
}

#[test]
fn every_step_of_every_route_is_a_licensed_edge() {
    // The resolver could not follow a refused edge without this failing, which makes the
    // refusals checkable rather than merely written down.
    let g = graph();
    for p in place_all(&g) {
        let Some(pl) = &p.placement else { continue };
        let mut class = p.class.clone();
        for r in &pl.reached {
            class.clone_from(&p.class);
            for step in &r.via {
                let route = route_for(&class, &step.relationship).unwrap_or_else(|| {
                    panic!("{}: --{}-> is not in ROUTES", p.node, step.relationship)
                });
                assert_eq!(
                    route.routing,
                    Routing::Places,
                    "{} was placed through a refused edge: --{}->",
                    p.node,
                    step.relationship
                );
                class = g.get(&step.to).map(|n| n.class.clone()).unwrap_or_default();
            }
        }
    }
}

#[test]
fn a_mark_never_sits_at_the_centre_of_the_whole_frame() {
    // The second test of the mark rule, checked end to end. `map.ts` already refuses this
    // placement for labels; a mark at the county centroid is a pin in a field near Lima.
    for p in placed() {
        if p.treatment == Treatment::Mark {
            let pl = p.placement.as_ref().expect("a mark is placed");
            assert!(
                pl.discriminates(),
                "{} is a mark on the whole frame",
                p.node
            );
        }
        if p.treatment == Treatment::Register {
            let pl = p.placement.as_ref().expect("a register entry is placed");
            assert!(
                !pl.discriminates(),
                "{} discriminates and is not a mark",
                p.node
            );
        }
    }
}

#[test]
fn every_stated_track_parses_and_places_the_node_that_states_it() {
    // The silent failure this guards is a Unicode minus. `track` is lifted out of prose that
    // writes U+2212, `parse_track` refuses it by rule, and a node whose track will not parse
    // does not fail anything — it quietly routes out through `occurred-in` and lands back on
    // the county, which is the exact state this change was made to end. Nothing else in the
    // gate can see the difference, so it is checked here.
    let g = graph();
    let stating: Vec<&str> = g
        .nodes()
        .filter(|n| n.properties.contains_key("track"))
        .map(|n| n.id.as_str())
        .collect();
    assert!(
        !stating.is_empty(),
        "no node states a track; the treatment has no members and this check is dead"
    );
    for id in stating {
        let raw = g.get(id).unwrap().properties.get("track").unwrap();
        let anchor = placement::parse_track(raw)
            .unwrap_or_else(|| panic!("{id}: track does not parse — {raw:?}"));
        let points = match anchor {
            placement::Anchor::Track { points } => points,
            other => panic!("{id}: parsed as {other:?}"),
        };
        assert!(points.len() >= 2, "{id}: a track needs two positions");

        let p = place(&g, id).unwrap_or_else(|| panic!("{id} is unplaced"));
        assert_eq!(p.hops, 0, "{id}: a stated track places at zero hops");
        assert!(p.is_track(), "{id}: placed at zero hops and not as a track");
        assert_eq!(
            p.reached[0].node, id,
            "{id}: a stated track anchors on the node that states it"
        );
    }
}

#[test]
fn a_track_is_never_filed_as_placed_no_finer_than_the_county() {
    // The defect the change was opened on, checked from the other end: both tornadoes were
    // registers while the corpus held four surveyed coordinates for them.
    for p in placed() {
        let states_a_track = p
            .placement
            .as_ref()
            .is_some_and(placement::Placement::is_track);
        if states_a_track {
            assert_eq!(p.treatment, Treatment::Track, "{}", p.node);
        }
        if p.treatment == Treatment::Track {
            assert!(states_a_track, "{}: a track that states none", p.node);
        }
    }
}

#[test]
fn a_count_is_drawn_on_its_subject_at_whatever_grain_that_subject_is() {
    // The discrimination test must not reach counts. Most of this corpus's measures are about
    // the county as a whole, and filing them under "could not be placed" would send its most
    // ordinary claims to the register.
    let all = placed();
    let county_counts = all
        .iter()
        .filter(|p| p.treatment == Treatment::Count)
        .filter(|p| p.placement.as_ref().is_some_and(|pl| !pl.discriminates()))
        .count();
    assert!(
        county_counts > 0,
        "no count resolves to the county, which cannot be right for this corpus"
    );
}

#[test]
fn a_period_is_never_placed() {
    for p in placed() {
        if p.class == "period" {
            assert_eq!(p.treatment, Treatment::NotSpatial, "{}", p.node);
            assert!(p.placement.is_none(), "{}", p.node);
        }
    }
}

#[test]
fn the_shape_of_the_distribution_holds() {
    // Not the numbers — the ordering, which is the finding the design rests on and is stable
    // under ordinary growth. Counts outnumber marks because most of this corpus is figures;
    // marks outnumber the unplaced because most things reach ground.
    //
    // A seventh treatment was added and neither assertion moved, which is the property the
    // decision claimed for this gate when it declined to pin the sizes. The partition sum below
    // is what actually catches a new member going unaccounted for.
    let all = placed();
    let t = by_treatment(&all);
    let get = |k: Treatment| t.get(&k).copied().unwrap_or(0);
    assert!(
        get(Treatment::Count) > get(Treatment::Mark),
        "counts {} vs marks {}",
        get(Treatment::Count),
        get(Treatment::Mark)
    );
    assert!(
        get(Treatment::Mark) > get(Treatment::Unplaced),
        "marks {} vs unplaced {}",
        get(Treatment::Mark),
        get(Treatment::Unplaced)
    );
    assert_eq!(
        all.len(),
        t.values().sum::<usize>(),
        "every node gets exactly one treatment"
    );
}

#[test]
fn most_of_the_corpus_reaches_ground_in_one_edge_or_none() {
    // The claim the plan rests on: the corpus is already placeable, and shallowly. If this
    // ever inverts, the derived placements have become chains nobody can check.
    let all = placed();
    let shallow = all
        .iter()
        .filter(|p| p.placement.as_ref().is_some_and(|pl| pl.hops <= 1))
        .count();
    assert!(
        shallow * 2 > all.len(),
        "only {shallow} of {} nodes are within one edge of ground",
        all.len()
    );
}

#[test]
fn every_anchor_a_placement_names_actually_exists_and_carries_a_position() {
    let g = graph();
    for p in place_all(&g) {
        let Some(pl) = &p.placement else { continue };
        assert!(!pl.reached.is_empty(), "{}: placed with no anchor", p.node);
        for r in &pl.reached {
            let target = g
                .get(&r.node)
                .unwrap_or_else(|| panic!("{}: anchor {} is not a node", p.node, r.node));
            assert!(
                placement::own_anchor(target).is_some(),
                "{}: anchor {} carries no position",
                p.node,
                r.node
            );
        }
    }
}
