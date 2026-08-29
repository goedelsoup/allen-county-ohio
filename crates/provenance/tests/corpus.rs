//! The gate. `mise run ci` runs `cargo test`, so these are what keep the corpus's edges tagged.
//!
//! This is the first check in `crates/` that fails on a corpus defect rather than reporting a
//! fact about the county. The rule it enforces does not exist in the vendored `graph-lint`, and
//! `.yidam/.vendor/` is read-only, so the domain computer carries it until upstream does.

use provenance::{audit, load, DefectKind};
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run() -> (provenance::Audit, usize) {
    let edges = load::edges(&root().join(".yidam/corpus")).expect("corpus links load");
    let catalog = load::catalog(&root().join(".yidam/catalog")).expect("catalog loads");
    let n = edges.len();
    (audit(&edges, &catalog), n)
}

#[test]
fn every_edge_that_asserts_something_says_what_kind_of_claim_it_is() {
    let (a, _) = run();
    let lines: Vec<String> = a
        .defects
        .iter()
        .map(|d| {
            format!(
                "{} --{}-> {} : {}",
                d.edge.node, d.edge.relationship, d.edge.target, d.kind
            )
        })
        .collect();
    assert!(
        a.is_clean(),
        "{} untagged or malformed edge(s):\n  {}",
        a.defects.len(),
        lines.join("\n  ")
    );
}

#[test]
fn the_corpus_has_more_edges_than_nodes_and_most_of_them_assert_something() {
    let (a, total) = run();
    assert!(total > 300, "expected the whole link set, got {total}");
    assert!(
        a.empirical > 200,
        "{} empirical edges — the graph is mostly claims, not bookkeeping",
        a.empirical
    );
    assert_eq!(a.empirical + a.structural, total);
}

/// Pins the shape of what the corpus knows versus what it recalls.
///
/// Not a target. This is the number that says how much of the graph rests on a source, and it
/// should move when the corpus retrieves rather than when someone edits a tag — a fall here is
/// worth looking at, and so is a rise nobody can name a retrieval for.
#[test]
fn most_edges_are_sourced_and_the_rest_are_labelled_inference() {
    let (a, _) = run();
    let verified = a.by_tag.get("verified").copied().unwrap_or(0);
    let inference = a.by_tag.get("inference").copied().unwrap_or(0);
    let open = a.by_tag.get("open").copied().unwrap_or(0);
    assert_eq!(verified + inference + open, a.empirical);
    assert!(
        verified >= 120,
        "sourced edges fell to {verified}; the corpus stood at 121"
    );
    assert!(
        verified as f64 / a.empirical as f64 > 0.5,
        "under half the graph is sourced: {verified}/{}",
        a.empirical
    );
}

#[test]
fn no_verified_edge_cites_a_catalog_entry_that_is_not_there() {
    let (a, _) = run();
    assert!(!a
        .defects
        .iter()
        .any(|d| d.kind == DefectKind::SourceNotInCatalog));
}
