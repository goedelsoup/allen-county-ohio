//! The public feed, and the rules a claim must pass to leave this repository.
//!
//! `web/README.md` named two things to settle before a page existed, and this crate is both
//! answers made executable.
//!
//! **The data source is a bundled export feed with a contract version, not the corpus read
//! live.** A page that parses node files couples every rendering change to the ontology. So
//! nothing under `web/` reads `.yidam/corpus/`: it reads the JSON this crate writes, which
//! carries [`feed::FEED_VERSION`].
//!
//! **Audience decides the rest.** This repository publishes, so the publication rules in
//! `agent-conduct.md` apply, and they are checked here rather than remembered: a claim's tier
//! is computed from the corpus on every build, `[open]` never leaves, and a derived assertion
//! carries the verbatim span it rests on and any refusal the cited node makes.
//!
//! # The ceiling, and why it is not `verified`
//!
//! The rule says `[verified]` may reach public material and `[inference]` reaches *attributed*
//! memos and backgrounders. A site rendering an inference as a bare fact is the first thing;
//! a site rendering it beside its tier, its span and its source is the second. This feed
//! publishes at [`Tier::Inference`] and the site is built to earn it — every claim carries its
//! tag, and every derived assertion carries its citations. `[open]` is excluded absolutely and
//! there is no flag to include it.
//!
//! # The gate
//!
//! `publish-feeds --check` regenerates the feeds and compares them to what is committed. The
//! feeds are a pure function of the corpus, so a difference means the corpus moved and the
//! feeds did not. That is the derivation being the gate, which is what the directory
//! conventions ask of anything outside `.yidam/corpus/` that derives from it.

pub mod claim;
pub mod derived;
pub mod feed;
pub mod load;
pub mod tier;

pub use tier::Tier;

use feed::FEED_VERSION;

/// The tier this site publishes at. See the module docs for why it is not `Verified`.
pub const CEILING: Tier = Tier::Inference;

const RATIONALE: &str = "Verified claims may reach public material; inference reaches \
                         attributed material. Every claim here carries its tag, its span and \
                         its source, which is what makes it attributed. Open claims do not \
                         leave the repository.";

/// A feed file: the name it is written under, and its content.
pub struct File {
    pub name: &'static str,
    pub json: String,
}

/// Every block holding a publishable claim beside an `[open]` one, as `node — opening words`.
///
/// Reported by `publish-feeds` and gated by nothing; [`claim::withheld`] says why it cannot be
/// a gate. The point is that the author is shown the blocks in this state rather than having
/// to remember that the state exists.
pub fn withheld(nodes: &[load::Node]) -> Vec<String> {
    let mut out = Vec::new();
    for node in nodes {
        for block in &node.blocks {
            let Some(strongest) = claim::withheld(block) else {
                continue;
            };
            let text = claim::normalize(&block.text);
            let opening: String = text.chars().take(72).collect();
            out.push(format!("{} [{strongest}] {opening}…", node.id));
        }
    }
    out
}

/// Build every feed from a loaded corpus.
///
/// Returns the files and every defect found. A caller writing files while defects exist is
/// publishing unchecked claims, so `main` refuses to.
///
/// `classes` is the corpus's own ontology, and it is passed in rather than inferred from the
/// nodes for the reason the whole crate exists: what a class is, is declared, and a feed that
/// guessed it from the instances would be stating it a second time.
pub fn build(
    nodes: &[load::Node],
    classes: &[load::Class],
) -> Result<(Vec<File>, Vec<derived::Defect>), serde_json::Error> {
    let (graph, mut counts) = feed::graph(nodes, CEILING);
    let series = feed::series(nodes, CEILING);
    let points = feed::map(nodes, CEILING);
    let (assertions, defects) = derived::resolve(derived::ASSERTIONS, nodes, CEILING);
    counts.assertions = assertions.len();

    let manifest = feed::Manifest {
        feed_version: FEED_VERSION,
        policy: feed::Policy {
            ceiling: CEILING,
            rationale: RATIONALE,
        },
        corpus: counts,
        classes: feed::schema(classes),
    };

    Ok((
        vec![
            file("manifest.json", &manifest)?,
            file("graph.json", &graph)?,
            file(
                "series.json",
                &feed::SeriesFeed {
                    feed_version: FEED_VERSION,
                    series,
                    assertions,
                },
            )?,
            file(
                "map.json",
                &feed::MapFeed {
                    feed_version: FEED_VERSION,
                    points,
                },
            )?,
        ],
        defects,
    ))
}

/// Serialize one feed.
///
/// Pretty-printed with a trailing newline: these are committed files, and a diff a reviewer
/// can read is worth more than the bytes it costs.
fn file(name: &'static str, value: &impl serde::Serialize) -> Result<File, serde_json::Error> {
    Ok(File {
        name,
        json: format!("{}\n", serde_json::to_string_pretty(value)?),
    })
}
