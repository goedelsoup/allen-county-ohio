//! Runs the calculator against this repository's own corpus.
//!
//! The expected result is not invented for the test: the succession was checked by hand when
//! the roster was extracted, and the commit that added it recorded no gaps and no true
//! overlaps across all 39 tenures. This test pins that, so a future edit to a tenure node
//! that breaks the line fails here rather than being noticed by nobody.

use std::path::PathBuf;
use succession::{audit, by_office, holders_in, load};

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.yidam/corpus")
}

#[test]
fn the_sheriffs_line_is_complete_and_clean() {
    let terms = load::tenures(&corpus()).expect("tenure nodes load");
    let grouped = by_office(terms);
    let sheriff = grouped
        .get("office/allen-county-sheriff.yml")
        .expect("sheriff office has tenures");

    assert_eq!(sheriff.len(), 39, "expected the full roster");

    let a = audit(1, sheriff);
    assert!(
        a.gaps.is_empty(),
        "the roster is continuous; gaps found: {:?}",
        a.gaps
    );
    assert!(
        a.overlaps.is_empty(),
        "year-precision boundaries must not read as overlaps; found: {:?}",
        a.overlaps
    );

    assert_eq!(
        a.line.first().unwrap().began,
        1831,
        "line starts at Lippencott"
    );
    assert!(
        a.line.last().unwrap().ended.is_none(),
        "the current holder's term is open"
    );
}

#[test]
fn eighteen_ninety_three_has_two_claimants() {
    let terms = load::tenures(&corpus()).expect("tenure nodes load");
    let grouped = by_office(terms);
    let sheriff = &grouped["office/allen-county-sheriff.yml"];
    let held = holders_in(sheriff, 1893);
    assert_eq!(
        held.len(),
        2,
        "1893 is a shared boundary year and both holders have a claim on it"
    );
}

#[test]
fn every_office_has_a_holder_and_an_empty_one_would_still_audit_clean() {
    // This pinned `office/mayor-of-lima.yml` as the corpus's one office with no tenures, which
    // it was from genesis until the elected officials roster was read across every precinct
    // and named the mayor. No office is empty now, so the corpus half of the assertion is
    // inverted rather than repointed at another node: the claim is that there is none.
    let seats = load::office_seats(&corpus()).expect("office nodes load");
    let grouped = by_office(load::tenures(&corpus()).unwrap());
    let empty: Vec<&String> = seats.keys().filter(|o| !grouped.contains_key(*o)).collect();
    assert!(empty.is_empty(), "offices with no holder: {empty:?}");

    // The property the pin was there for does not need such an office to exist: a seat with no
    // terms is a roster with no gaps, not a roster with one.
    assert!(audit(1, &[]).is_clean());
}

#[test]
fn every_office_declares_a_seat_count_the_loader_can_read() {
    let seats = load::office_seats(&corpus()).expect("office nodes load");
    assert!(!seats.is_empty());
    for (office, n) in &seats {
        assert!(*n >= 1, "{office} has {n} seats");
    }
}
