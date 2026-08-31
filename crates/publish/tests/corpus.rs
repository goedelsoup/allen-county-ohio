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
    //
    // A fourth event, and this one is an answer. `state-a-refusal-at-the-grain-it-protects` said
    // that if a fourth narrowing arrived it was time to ask whether the caveat still had a
    // subject. County Business Patterns supplied thirty-six years of Allen County manufacturing
    // employment, all of it inside the period: 15,762 in 1986 down to 7,127 in 2010. So
    //
    //   "*manufacturing employment* inside this period"    ANSWERED by County Business Patterns
    //
    // and the question got asked. The caveat does still have a subject, and a better one. What
    // the chart invites is a causal reading — the people left because the factories did — and
    // the corpus has never licensed it. Before, it could not license it because the cause was
    // unmeasured. Now it declines to because the two series diverge: 2010 to 2022, private
    // employment fell 3,709 while manufacturing rose 1,446. The pin moves from an absence of
    // evidence to a conflict in it, which is the stronger place for it to sit, and it is still
    // something retrieval cannot answer because causation is not a thing you can download.
    // See .yidam/decisions/the-fourth-narrowing-was-an-answer.yml.
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
            .any(|r| r.contains("does not establish that")),
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
            .any(|c| c.contains("does not establish that")),
        "the refusal did not reach the reader"
    );
}

#[test]
fn the_corpuss_own_counts_are_what_the_feed_reports() {
    // A cheap tripwire against the loader silently skipping a class directory — the shape of
    // failure that leaves every gate passing and a third of the county missing from the map.
    let nodes = nodes();
    assert_eq!(nodes.len(), 388, "corpus node count moved; update this pin");
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
fn every_elected_county_office_ohio_creates_has_a_node() {
    // The closed-set check argued for in `a-deep-instance-can-hide-an-empty-class`. Ohio gives
    // every county these nine elected offices, so nine is a denominator the corpus does not get
    // to choose. For nine phases it held one of them while the class looked exercised, because
    // the sheriff alone carried 39 tenures and a crate. A count is the only thing that catches
    // that; reading harder does not.
    let ids: Vec<String> = nodes()
        .into_iter()
        .filter(|n| n.class == "office")
        .map(|n| n.id)
        .collect();
    for office in [
        "office/allen-county-auditor.yml",
        "office/allen-county-board-of-commissioners.yml",
        "office/allen-county-clerk-of-courts.yml",
        "office/allen-county-coroner.yml",
        "office/allen-county-engineer.yml",
        "office/allen-county-prosecuting-attorney.yml",
        "office/allen-county-recorder.yml",
        "office/allen-county-sheriff.yml",
        "office/allen-county-treasurer.yml",
    ] {
        assert!(
            ids.iter().any(|id| id == office),
            "no node for {office} — Ohio creates this office in every county"
        );
    }
}

#[test]
fn the_county_still_has_five_hospitals() {
    // A count, for the reason `a-deep-instance-can-hide-an-empty-class` gives: reading harder
    // does not catch a set that has changed size, and this set is small enough to be checked.
    //
    // Five is a choice and the corpus argues for it. CMS's Care Compare roster says four — it has
    // no long-term care category, so Kindred Hospital Lima is absent from all 193 of its Ohio
    // rows — and the cost reports say five. Neither file is wrong about what it counts. The corpus
    // takes the wider set because a hospital that files a cost report and employs sixty-four
    // people is a hospital in this county whether or not it is in a quality programme.
    //
    // If this ever fails, the question is not "update the pin". It is which file changed and
    // whether a hospital opened, closed, or merely moved between federal categories.
    let hospitals: Vec<String> = nodes()
        .into_iter()
        .filter(|n| n.class == "organization")
        .filter(|n| n.property("industry").is_some_and(|i| i == "hospitals"))
        .map(|n| n.id)
        .collect();
    assert_eq!(hospitals.len(), 5, "hospital count moved: {hospitals:?}");
}

#[test]
fn every_refusal_survives_the_way_its_node_is_wrapped() {
    // The defect this guards shipped and was invisible: `refusal()` searched the raw block, node
    // prose is hard-wrapped at about 95 columns, and three of this corpus's refusals had their
    // trigger phrase split across a wrap. A refusal nobody detects looks exactly like a node with
    // none, so an assertion citing one would have published with no caveat and no complaint.
    //
    // Checked here rather than only in the unit tests because the property is about the corpus:
    // no node may state a refusal that the detector cannot see. Rewrapping a paragraph must never
    // change whether its caveat reaches a reader.
    const REFUSALS: [&str; 12] = [
        "does not establish",
        "does not assert",
        "does not infer",
        "do not infer",
        "does not license",
        "does not follow",
        "does not know",
        "cannot say",
        "cannot show",
        "cannot answer",
        "is not containment",
        "not a proof of",
    ];
    let mut missed = Vec::new();
    for node in nodes() {
        for block in &node.blocks {
            let flat = publish::claim::normalize(&block.text).to_lowercase();
            if REFUSALS.iter().any(|p| flat.contains(p)) && block.refusal.is_none() {
                missed.push(format!("{}: {}", node.id, &flat[..flat.len().min(90)]));
            }
        }
    }
    assert!(
        missed.is_empty(),
        "refusals the detector cannot see: {missed:#?}"
    );
}

#[test]
fn every_cited_block_names_the_source_it_rests_on() {
    // Three phases running, a citation reached the public page with a provenance badge and no
    // provenance in it. The cause is always the same and always reads fine in the file: a block
    // ends "[verified] — same source", where the same source is a paragraph above. The reader of
    // the node follows it; the reader of the site gets a claim attributed to nothing.
    //
    // `graph-lint` has a node-level version of this rule and it cannot see the case, because the
    // node does draw on a source — just not in the block that went out. Publication is where the
    // block becomes the unit, so this is where the block-level rule belongs.
    // The rule is about `[verified]` and not about every block. A verified claim asserts that a
    // source supports it, so the reader is owed the source. An `[inference]` block is the corpus
    // reasoning over material it already holds — "computed here", "from the three series above" —
    // and its provenance is the node it sits in, which the site shows beside it.
    let (resolved, _) = resolve(ASSERTIONS, &nodes(), CEILING);
    let mut bare = Vec::new();
    for a in &resolved {
        for c in &a.citations {
            if c.tier == Tier::Verified && c.sources.is_empty() {
                bare.push(format!(
                    "{}: {} — {}",
                    a.id,
                    c.node,
                    &c.span[..c.span.len().min(70)]
                ));
            }
        }
    }
    assert!(
        bare.is_empty(),
        "verified citations resting on no source: {bare:#?}"
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

#[test]
fn the_block_shape_that_hid_four_township_officers_is_reported() {
    // Written for a paragraph this corpus shipped and then split: Bath Township's four
    // officers, named and verified, followed by the admission that the township's erection
    // year is still unknown. The block travels on its weakest tag, so naming the government
    // of a township published nothing at all — and nothing was malformed, so nothing failed.
    let mixed = blocks(
        "**Who governs it now.** Its fiscal officer is Berlin Ralph Carroll II. [verified] — \
         the roster. The corpus still cannot name the year it was erected. [open]\n",
    );
    assert_eq!(
        publish::claim::withheld(&mixed[0]),
        Some(Tier::Verified),
        "a verified fact sitting behind an open one went unreported"
    );

    // The fix is a blank line, and the report has to go quiet when the author applies it.
    let split = blocks(
        "Its fiscal officer is Berlin Ralph Carroll II. [verified] — the roster.\n\n\
         [open] The year this township was erected.\n",
    );
    assert!(
        split.iter().all(|b| publish::claim::withheld(b).is_none()),
        "splitting the block did not clear the report"
    );

    // It is a report and not a gate: this corpus has thirty-odd of these on purpose, and the
    // feeds build clean with every one of them in place.
    let nodes = nodes();
    assert!(
        !publish::withheld(&nodes).is_empty(),
        "the corpus stopped exercising the report entirely; is it still wired up?"
    );
    let (_, defects) = build(&nodes).expect("feeds serialize");
    assert!(defects.is_empty(), "the report became a gate");
}
