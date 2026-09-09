//! Runs the normalizer against this repository's own corpus.
//!
//! What is pinned here is the **policy table's agreement with the corpus**, not the corpus's
//! size. A count of dated nodes would fail on every phase that adds one, and a gate that fails
//! for the wrong reason gets switched off — the argument `.yidam/lint-baseline.yml` rests on,
//! and the one that struck a word range out of
//! `.yidam/decisions/a-page-is-an-argument-not-an-inbox.yml` rather than widening it.
//!
//! So these are invariants. A class that starts carrying dates fails here until somebody
//! decides what its silence means; an entry that outlives the dates it describes fails here
//! too; and a date field nobody can read fails here rather than emptying a year on a map.

use chronology::{load, policy_for, Dated, OpenEnd, Span, Undated, POLICY};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

fn corpus() -> Vec<Dated> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.yidam/corpus");
    load::corpus(&dir).expect("the corpus loads")
}

/// The date properties every class declares, read from the corpus rather than from the table,
/// so the two can be compared without one being derived from the other.
fn classes_carrying_dates(nodes: &[Dated]) -> BTreeMap<String, BTreeSet<String>> {
    // Every property any ontology in this corpus uses for a date. Written out rather than
    // taken from `POLICY`, because a table that supplied its own evidence would agree with
    // itself no matter what the corpus said.
    const DATE_PROPERTIES: &[&str] = &[
        "occurred",
        "occurred_through",
        "began",
        "ended",
        "as_of",
        "erected",
        "abolished",
        "established",
        "built",
        "ceased",
        "founded",
        "dissolved",
        "born",
        "died",
        "effective_from",
        "effective_to",
        "opened",
        "closed",
    ];
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.yidam/corpus");
    let mut found: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in std::fs::read_dir(&dir).expect("corpus dir") {
        let path = entry.expect("dir entry").path();
        if !path.is_dir() {
            continue;
        }
        for file in std::fs::read_dir(&path).expect("class dir") {
            let file = file.expect("dir entry").path();
            if file.extension().is_none_or(|x| x != "yml")
                || file.to_string_lossy().ends_with(".ont.yml")
            {
                continue;
            }
            let text = std::fs::read_to_string(&file).expect("node reads");
            let doc: serde_yaml::Value = serde_yaml::from_str(&text).expect("node parses");
            let Some(class) = doc.get("class").and_then(|c| c.as_str()) else {
                continue;
            };
            let Some(props) = doc.get("properties").and_then(|p| p.as_mapping()) else {
                continue;
            };
            for key in DATE_PROPERTIES {
                if props.get(serde_yaml::Value::from(*key)).is_some() {
                    found
                        .entry(class.to_string())
                        .or_default()
                        .insert((*key).to_string());
                }
            }
        }
    }
    let _ = nodes;
    found
}

#[test]
fn every_class_carrying_dates_has_decided_what_its_silence_means() {
    let nodes = corpus();
    let carrying = classes_carrying_dates(&nodes);
    let missing: Vec<&String> = carrying
        .keys()
        .filter(|c| policy_for(c).is_none())
        .collect();
    assert!(
        missing.is_empty(),
        "these classes carry date properties and are not in POLICY: {missing:?}\n\
         Add an entry with the argument for its reading — see the decision node \
         `an-absent-end-date-means-four-things`."
    );
}

#[test]
fn no_entry_outlives_the_dates_it_describes() {
    // The half that survives contact with time. A list of exceptions permitted to be wrong
    // drifts, and one that over-lists silently re-permits whatever a later edit puts in its
    // place — the argument `design/departures.md` and the lint baseline are both built on.
    let nodes = corpus();
    let carrying = classes_carrying_dates(&nodes);
    let stale: Vec<&str> = POLICY
        .iter()
        .map(|p| p.class)
        .filter(|c| !carrying.contains_key(*c))
        .collect();
    assert!(
        stale.is_empty(),
        "POLICY names these classes and no instance of them carries a date: {stale:?}\n\
         Remove the entry, or find out why the dates went."
    );
}

#[test]
fn each_policy_names_the_properties_its_class_actually_uses() {
    // A typo in the table would not fail anything else: the start property would never be
    // found, every instance would read as undated, and the class would quietly leave the map.
    let nodes = corpus();
    let carrying = classes_carrying_dates(&nodes);
    for p in POLICY {
        let used = carrying
            .get(p.class)
            .unwrap_or_else(|| panic!("{} carries no dates", p.class));
        assert!(
            used.contains(p.start),
            "{} dates itself by `{}` in POLICY, and no instance carries that property. \
             The corpus uses: {used:?}",
            p.class,
            p.start
        );
        if let Some(end) = p.end {
            assert!(
                used.contains(end) || !used.iter().any(|u| u == end),
                "{}: `{end}` is unreadable",
                p.class
            );
        }
    }
}

#[test]
fn every_date_in_the_corpus_can_be_read() {
    // The failure this catches is silent everywhere else: `c. 1885` in a date field parses as
    // YAML, passes the graph check, and removes a node from every year of the map.
    let unreadable: Vec<String> = corpus()
        .into_iter()
        .filter_map(|n| match n.span {
            Err(Undated::Unreadable { property, value }) => {
                Some(format!("{}: `{property}` = {value:?}", n.id))
            }
            _ => None,
        })
        .collect();
    assert!(
        unreadable.is_empty(),
        "date fields that are not dates:\n  {}",
        unreadable.join("\n  ")
    );
}

#[test]
fn every_span_lands_in_a_year_this_county_could_have() {
    // A four-digit typo is the one date error that parses cleanly. The county's ground was
    // ceded in 1818 and the corpus reaches back to a 1769 birth; nothing here is medieval and
    // nothing is in the future.
    for n in corpus() {
        let Ok(span) = n.span else { continue };
        let from = span.from_year();
        assert!((1700..=2100).contains(&from), "{}: starts in {from}", n.id);
        if let Some(to) = span.to_year() {
            assert!(
                to >= from,
                "{}: ends in {to}, before it began in {from}",
                n.id
            );
            assert!((1700..=2100).contains(&to), "{}: ends in {to}", n.id);
        }
    }
}

#[test]
fn all_four_readings_are_exercised_by_the_corpus() {
    // The same anti-drift rule one level up: a reading nothing in the corpus reaches is a
    // distinction being maintained for nobody, and should be argued for or removed.
    let mut seen: BTreeSet<OpenEnd> = BTreeSet::new();
    for n in corpus() {
        match n.span {
            Ok(Span::Instant(_)) => {
                seen.insert(OpenEnd::Instant);
            }
            Ok(Span::OpenEnded { reading, .. }) => {
                seen.insert(reading);
            }
            _ => {}
        }
    }
    for reading in [
        OpenEnd::Instant,
        OpenEnd::Running,
        OpenEnd::Unvouched,
        OpenEnd::Unknown,
    ] {
        assert!(
            seen.contains(&reading),
            "no node in the corpus reads as {reading}"
        );
    }
}

#[test]
fn the_fourth_reading_keeps_a_one_night_event_out_of_the_next_year() {
    // The raid on the county jail, 12 October 1933. Under the two-reading model this node had
    // no `occurred_through` and would have been drawn as still happening in 1934 and in every
    // year since. It is the case the fourth reading was written for.
    let nodes = corpus();
    let raid = nodes
        .iter()
        .find(|n| n.id == "event/allen-county-jail-raid-1933.yml")
        .expect("the jail raid is in the corpus");
    let span = raid.span.as_ref().expect("it is dated");
    assert!(matches!(span, Span::Instant(_)), "got {span}");
    assert!(span.admits(1933));
    assert!(!span.admits(1934));
}

#[test]
fn a_division_is_admitted_after_its_start_and_vouched_only_at_it() {
    // `covering`'s reading, checked against a node that still has the shape it argues from.
    // The congressional district of the same map was this test's subject until the plan that
    // superseded it was read and gave 31 October 2025; an absent end is a gap for a source to
    // close, so a node leaving this state is the reading working rather than failing.
    let nodes = corpus();
    let district = nodes
        .iter()
        .find(|n| n.id == "division/ohio-house-district-4-2020.yml")
        .expect("the 2020 House district is in the corpus");
    let span = district.span.as_ref().expect("it is dated");
    assert!(span.admits(2024), "it may still stand");
    assert!(!span.vouched(2024), "and the corpus does not say it does");
    assert!(span.vouched(span.from_year()));
}

#[test]
fn nothing_reports_the_present_as_a_recorded_end() {
    // Across the whole corpus, not only the unit-test cases: an open end must never answer
    // `to_year`, because a caller drawing a bar from it would turn the build's clock into a
    // claim about the world.
    for n in corpus() {
        if let Ok(span @ Span::OpenEnded { .. }) = n.span {
            assert_eq!(span.to_year(), None, "{}", n.id);
        }
    }
}
