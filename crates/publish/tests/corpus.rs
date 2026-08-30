//! Runs the publication rules against this repository's own corpus.
//!
//! What is pinned here is not "the feed currently says X" but the discipline: nothing tagged
//! `[open]` leaves, every derived assertion still resolves against the prose beneath it, and
//! the corpus's refusals still reach a reader who will never open a node file.
//!
//! `mise run ci` runs `cargo test`, so this is the gate — the same arrangement `provenance`
//! uses, for the same reason.

use publish::claim::blocks;
use publish::derived::{resolve, ASSERTIONS};
use publish::{build, load, Tier, CEILING};
use std::path::PathBuf;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.yidam/corpus")
}

fn nodes() -> Vec<load::Node> {
    load::corpus(&corpus_dir()).expect("corpus loads")
}

#[test]
fn every_assertion_this_site_makes_still_rests_on_the_prose_it_cites() {
    let (resolved, defects) = resolve(ASSERTIONS, &nodes(), CEILING);
    assert!(
        defects.is_empty(),
        "the site asserts something the corpus no longer supports:\n{}",
        defects
            .iter()
            .map(|d| format!("  {d}"))
            .collect::<Vec<_>>()
            .join("\n\n")
    );
    assert_eq!(resolved.len(), ASSERTIONS.len());
}

#[test]
fn nothing_tagged_open_leaves_the_repository() {
    // The one rule with no exception and no flag. Checked over the serialized bytes rather
    // than over the structs, because the question is what the file contains.
    let (files, _) = build(&nodes()).expect("feeds serialize");
    for f in &files {
        assert!(
            !f.json.contains("[open]"),
            "{} carries an open claim marker",
            f.name
        );
    }
}

#[test]
fn every_published_claim_carries_a_tag_that_reaches_the_ceiling() {
    for node in nodes() {
        for block in node.blocks.iter().filter(|b| b.publishable(CEILING)) {
            let tier = block.tier.expect("publishable implies tagged");
            assert!(
                tier.reaches(CEILING),
                "{}: a block at {tier} passed the filter",
                node.id
            );
        }
    }
}

#[test]
fn the_feeds_are_a_pure_function_of_the_corpus() {
    // What `publish-feeds --check` depends on. A timestamp or a commit hash in a feed would
    // make the gate fail on every commit and be switched off within a week.
    let (first, _) = build(&nodes()).expect("feeds serialize");
    let (second, _) = build(&nodes()).expect("feeds serialize");
    for (a, b) in first.iter().zip(second.iter()) {
        assert_eq!(a.name, b.name);
        assert_eq!(a.json, b.json, "{} is not reproducible", a.name);
    }
}

#[test]
fn the_committed_feeds_are_what_the_corpus_says() {
    let feeds = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../web/src/feeds");
    let (files, _) = build(&nodes()).expect("feeds serialize");
    for f in &files {
        let path = feeds.join(f.name);
        let on_disk = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{}: {e} — run `mise run publish`", path.display()));
        assert_eq!(
            on_disk,
            f.json,
            "{} is stale — run `mise run publish` and commit the result",
            path.display()
        );
    }
}

#[test]
fn the_corpus_still_refuses_the_reading_a_population_chart_invites() {
    // This corpus's sharpest refusal, and the reason the check is node-level rather than
    // block-level: `deindustrialization` demonstrates a decline in one paragraph and refuses
    // the reading its chart invites in the next. If the refusal detector ever stops matching,
    // the site keeps the chart and loses the caveat.
    //
    // The pin has moved twice, and both retired sentences are recorded here because a test
    // whose reason is lost gets deleted by whoever trips over it next.
    //
    //   "It does not establish that 1970 is the start"      retired by the 1990 Census volume
    //   "Lima peaked in 1970 rather than earlier"           retired by the 1960 Census volume
    //
    // Both were about where the corpus's window began, and both were answered by going and
    // getting the earlier observations — the county peaked in 1980, and Lima peaked in 1970
    // after all, with twelve rising counts behind it. Answered by evidence, not dropped.
    //
    // The pin is now the refusal that retrieval cannot answer: the period is named for
    // manufacturing employment and the corpus has no measurement of it inside the period. See
    // .yidam/decisions/pin-the-tripwire-to-what-retrieval-cannot-answer.yml.
    //
    // A third event, and a different kind from the two above. The 1910 Ohio supplement gave the
    // corpus its first employment figures — Lima's wage earners in 1899, 1904 and 1909 — which
    // did not answer this refusal but did falsify its wording. It used to say the corpus "does
    // not measure manufacturing employment at all"; it now says "inside this period", which is
    // what the caveat was ever protecting. The pin moved with it, and it is the narrower phrase
    // on purpose. See .yidam/decisions/state-a-refusal-at-the-grain-it-protects.yml.
    let text = std::fs::read_to_string(corpus_dir().join("period/deindustrialization.yml"))
        .expect("the node is there");
    let description = text
        .split("description: |")
        .nth(1)
        .and_then(|s| s.split("\nproperties:").next())
        .expect("it has a description");

    let refusals: Vec<String> = blocks(description)
        .into_iter()
        .filter_map(|b| b.refusal)
        .collect();

    assert!(
        refusals
            .iter()
            .any(|r| r.contains("*manufacturing employment* inside this period")),
        "the refusal is no longer detected; found {refusals:?}"
    );

    let (resolved, _) = resolve(ASSERTIONS, &nodes(), CEILING);
    let decline = resolved
        .iter()
        .find(|a| a.id == "county-population-decline")
        .expect("the population assertion survives");
    assert!(
        decline
            .caveats
            .iter()
            .any(|c| c.contains("*manufacturing employment* inside this period")),
        "the refusal did not reach the reader"
    );
}

#[test]
fn the_corpuss_own_counts_are_what_the_feed_reports() {
    // A cheap tripwire against the loader silently skipping a class directory — the shape of
    // failure that leaves every gate passing and a third of the county missing from the map.
    let nodes = nodes();
    assert_eq!(nodes.len(), 215, "corpus node count moved; update this pin");
    assert_eq!(
        nodes.iter().filter(|n| n.class == "place").count(),
        25,
        "place count moved"
    );
    assert!(
        nodes
            .iter()
            .all(|n| !n.class.is_empty() && !n.id.is_empty()),
        "a node loaded without a class or an id"
    );
}

#[test]
fn every_located_place_carries_the_geoid_the_map_joins_on() {
    // The map draws corpus places onto vendored Census geometry, and GEOID is the join. A
    // place with a centroid and no GEOID renders as a dot with no boundary and no warning.
    let missing: Vec<String> = nodes()
        .into_iter()
        .filter(|n| n.class == "place" && n.property("centroid").is_some())
        .filter(|n| n.property("geoid").is_none())
        .map(|n| n.id)
        .collect();
    assert!(missing.is_empty(), "places with no GEOID: {missing:?}");
}

#[test]
fn the_publication_ceiling_is_not_open() {
    // Guards the constant itself. Every other test here would still pass if `CEILING` were
    // widened to `Open`, and the whole apparatus would be off.
    assert!(CEILING < Tier::Open);
}
