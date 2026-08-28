//! Runs the query against this repository's own corpus.
//!
//! What is pinned here is not "the answer is currently X" but the discipline the calculator
//! exists to enforce: dated and undated coverage never merge, a split county line reaches
//! every member downstream of it, and a 2020 district is not quietly treated as current.

use covering::{covering, load, Extent, Reach, Warrant};
use std::path::PathBuf;

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.yidam/corpus")
}

fn graph() -> covering::Graph {
    load::graph(&corpus()).expect("corpus loads")
}

#[test]
fn asking_what_governed_lima_in_1900_separates_what_the_corpus_dates_from_what_it_asserts() {
    let c = covering(&graph(), "place/lima.yml", Some(1900)).unwrap();

    let dated: Vec<&str> = c.dated.iter().map(|m| m.node.as_str()).collect();
    assert_eq!(
        dated,
        vec!["jurisdiction/allen-county-government.yml"],
        "the county government is the only covering body this corpus has dated"
    );

    let undated: Vec<&str> = c.undated.iter().map(|m| m.node.as_str()).collect();
    assert_eq!(
        undated,
        vec![
            "jurisdiction/city-of-lima.yml",
            "jurisdiction/lima-city-school-district.yml"
        ],
        "the corpus does not record when either was created, and the answer must say so \
         rather than implying they were there in 1900"
    );

    // Every 2020 district is set aside rather than dropped: the query considered it.
    assert_eq!(c.excluded.len(), 3);
    assert!(c.excluded.iter().all(|m| m.class == "division"));
}

#[test]
fn the_2020_districts_are_open_ended_and_are_not_read_as_current() {
    // `effective_to` is absent on all three because the corpus does not know when Ohio's
    // post-2020 maps superseded them — see the open question on the congressional district.
    // Warrant::Open says exactly that; Warrant::Bounded would claim an end the corpus lacks,
    // and treating the absence as "still in force" would claim a currency it disclaims.
    let g = graph();
    let c = covering(&g, "place/lima.yml", None).unwrap();
    for id in [
        "division/ohio-congressional-district-4-2020.yml",
        "division/ohio-house-district-4-2020.yml",
        "division/ohio-senate-district-12-2020.yml",
    ] {
        let m = c.member(id).unwrap_or_else(|| panic!("{id} covers Lima"));
        assert_eq!(m.warrant, Warrant::Open { from: 2020 }, "{id}");
    }
}

#[test]
fn a_place_that_crosses_the_county_line_is_covered_only_partially() {
    // Delphos crosses into Van Wert and Bluffton into Hancock; both nodes say so with a
    // verified Census citation. Nothing at county scale covers either of them whole — not the
    // county government, and not the districts drawn over the county.
    let g = graph();
    for place in ["place/delphos.yml", "place/bluffton.yml"] {
        let c = covering(&g, place, None).unwrap();
        let county = c
            .member("jurisdiction/allen-county-government.yml")
            .unwrap_or_else(|| panic!("{place} is in Allen County"));
        assert!(
            matches!(county.extent, Extent::Partial { .. }),
            "{place}: the county covers it partially and the answer must not round that up"
        );
        for m in c.all().filter(|m| m.class == "division") {
            assert!(
                matches!(m.extent, Extent::Partial { .. }),
                "{place}: {} is drawn over the county, so it inherits the split",
                m.node
            );
        }
    }
}

#[test]
fn a_place_wholly_inside_the_county_is_covered_wholly() {
    // The contrast the previous test needs: partiality comes from an edge that says so, not
    // from being anywhere near a boundary.
    let c = covering(&graph(), "place/lima.yml", None).unwrap();
    assert!(c.all().all(|m| m.extent == Extent::Whole));
}

#[test]
fn township_authority_over_a_community_inside_it_is_reported_as_an_inference() {
    // Fort Shawnee is a CDP inside Shawnee Township with no government of its own. The
    // township's authority over it is real and is *derived* — from `within`, not from any
    // edge the corpus wrote between the two — so it must not be reported as asserted.
    let c = covering(&graph(), "place/fort-shawnee.yml", None).unwrap();
    let twp = c
        .member("jurisdiction/shawnee-township.yml")
        .expect("found");
    assert_eq!(twp.reach, Reach::Inherited);
    assert_eq!(twp.via, vec!["place/fort-shawnee.yml"]);
}

#[test]
fn a_site_answers_with_its_places_covering_set_and_names_the_hop() {
    let c = covering(&graph(), "site/allen-county-courthouse.yml", None).unwrap();
    assert_eq!(c.place, "place/lima.yml");
    assert_eq!(
        c.resolved_from.as_deref(),
        Some("site/allen-county-courthouse.yml"),
        "the answer is coarser than the site and the result has to admit it"
    );
    assert!(c.member("jurisdiction/city-of-lima.yml").is_some());
}

#[test]
fn every_place_and_site_in_the_corpus_answers_and_lands_under_the_county() {
    let g = graph();
    let ids: Vec<String> = g
        .ids_of_class("place")
        .chain(g.ids_of_class("site"))
        .map(str::to_string)
        .collect();
    assert!(ids.len() >= 13, "expected the corpus's places and sites");

    for id in ids {
        let c = covering(&g, &id, None).unwrap_or_else(|e| panic!("{id}: {e}"));
        if id == "place/allen-county.yml" {
            continue;
        }
        assert!(
            c.member("jurisdiction/allen-county-government.yml")
                .is_some(),
            "{id}: every ground in this corpus is under the county government"
        );
    }
}
