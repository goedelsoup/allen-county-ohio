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
        vec![
            "jurisdiction/city-of-lima.yml",
            "jurisdiction/allen-county-government.yml"
        ],
        "two covering bodies are dated for 1900: the county government, erected 1820, and the \
         city of Lima, whose corporation was organized on 29 March 1842. Lima moved from the \
         undated list to this one when its founding date reached the corporation's own field \
         from an event node that had held it since the settlements phase"
    );

    let undated: Vec<&str> = c.undated.iter().map(|m| m.node.as_str()).collect();
    assert_eq!(
        undated,
        vec![
            "jurisdiction/bath-local-school-district.yml",
            "jurisdiction/elida-local-school-district.yml",
            "jurisdiction/lima-city-school-district.yml",
            "jurisdiction/perry-local-school-district.yml",
            "jurisdiction/shawnee-local-school-district.yml",
            "jurisdiction/allen-county-court-of-common-pleas.yml",
            "jurisdiction/lima-municipal-court.yml",
            "jurisdiction/third-district-court-of-appeals.yml"
        ],
        "the corpus dates none of these, and the answer must say so rather than implying eight \
         bodies were governing Lima in 1900 — five school districts whose boundaries are known \
         only in the 2020 geography, and three courts whose establishment the corpus has not \
         dated at all. The courts arrived correctly: they cover the county, Lima is within it, \
         and a court's authority over Lima in 1900 is exactly the kind of claim this split is \
         here to refuse"
    );

    // Every division is set aside rather than dropped: the query considered it. Six now — the
    // three districts of the 2020 map, the two of the 2023 map, and the precinct that covers
    // part of the city. The 2023 pair are excluded on their own `effective_from`, which is the
    // check working: a district adopted in September 2023 governed nothing in 1900.
    assert_eq!(c.excluded.len(), 6);
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
fn partiality_comes_from_an_edge_that_says_so_and_never_from_proximity() {
    // The contrast the previous test needs. Lima does not cross the county line, so the county
    // government covers it whole — being cut into five school districts does not make that
    // partial, and neither does being wrapped in townships. The only partial members are the
    // ones whose own edge says `partially-covers`.
    let c = covering(&graph(), "place/lima.yml", None).unwrap();
    for id in [
        "jurisdiction/allen-county-government.yml",
        "jurisdiction/city-of-lima.yml",
    ] {
        assert_eq!(
            c.member(id)
                .unwrap_or_else(|| panic!("{id} covers Lima"))
                .extent,
            Extent::Whole,
            "{id}"
        );
    }
    // The three 2020 districts reach Lima through the county, which contains it wholly, so the
    // inheritance must not manufacture a split either.
    for id in [
        "division/ohio-congressional-district-4-2020.yml",
        "division/ohio-house-district-4-2020.yml",
        "division/ohio-senate-district-12-2020.yml",
    ] {
        assert_eq!(
            c.member(id)
                .unwrap_or_else(|| panic!("{id} covers Lima"))
                .extent,
            Extent::Whole,
            "{id}"
        );
    }
    // A precinct inside the city is the one division that says `partially-covers`, and it must
    // still not drag the county or the districts down with it.
    for id in [
        "division/voting-district-lima-1a-2020.yml",
        "jurisdiction/lima-city-school-district.yml",
        "jurisdiction/elida-local-school-district.yml",
        "jurisdiction/bath-local-school-district.yml",
        "jurisdiction/shawnee-local-school-district.yml",
        "jurisdiction/perry-local-school-district.yml",
    ] {
        let m = c
            .member(id)
            .unwrap_or_else(|| panic!("{id} covers part of Lima"));
        assert!(
            matches!(m.extent, Extent::Partial { .. }),
            "{id}: the edge says partially-covers and the answer must not round it up"
        );
    }
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
        // The one exception, and it is the finding rather than a defect. Fort Amanda is the
        // origin of this county's settlement and the namesake of one of its townships, and it
        // stands 990 feet outside the line: it carries `formerly-in` to the county and
        // `located-in` to nothing, because it has been in Auglaize County since 1848. The query
        // is right to refuse — the corpus does not hold the ground it stands on — and this
        // assertion is here so that a later `located-in` edge, which would be well formed and
        // false, fails the build. See `.yidam/decisions/of-here-is-not-located-here.yml`.
        if id == "site/fort-amanda.yml" {
            assert!(
                covering(&g, &id, None).is_err(),
                "fort-amanda is not on ground this corpus covers and must not answer as if it were"
            );
            continue;
        }
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

#[test]
fn the_census_tract_covers_two_townships_and_neither_of_them_wholly() {
    // 90 of the tract's 107 blocks are in Sugar Creek and 17 in American. It is 98.5% of the
    // first and 21.5% of the second, so the corpus writes `partially-covers` for both and the
    // query must carry that through rather than rounding either to whole.
    let g = graph();
    for place in [
        "place/sugar-creek-township.yml",
        "place/american-township.yml",
    ] {
        let c = covering(&g, place, None).unwrap();
        let tract = c
            .member("division/census-tract-39003010300.yml")
            .unwrap_or_else(|| panic!("{place} is partly in the tract"));
        assert!(
            matches!(tract.extent, Extent::Partial { .. }),
            "{place}: the tract covers it partly and the answer must not round that up"
        );
        assert_eq!(tract.reach, Reach::Asserted, "{place}");
    }
}

#[test]
fn the_tank_plant_answers_with_shawnee_township_not_lima() {
    // TIGERweb puts the plant in Shawnee Township and in no incorporated or designated place.
    // The corpus followed, so the site's covering set is the township's — and must no longer
    // contain the City of Lima.
    let c = covering(&graph(), "site/lima-army-tank-plant.yml", None).unwrap();
    assert_eq!(c.place, "place/shawnee-township.yml");
    assert!(c.member("jurisdiction/shawnee-township.yml").is_some());
    assert!(
        c.member("jurisdiction/city-of-lima.yml").is_none(),
        "the plant is not in Lima and its covering set must not say it is"
    );
}
