//! Assertions this site makes that no single corpus node makes, and the gate over them.
//!
//! A chart is an assertion. "The county has lost population for five decades" is not written
//! in any node in those words; it is derived, and the three rules in
//! `.yidam/.vendor/prelude/guidelines/agent-conduct.md` — *When claims leave the repository* —
//! are what let it leave:
//!
//! 1. **A derived assertion travels only as far as the weakest claim beneath it.** Its tier
//!    is computed here from the blocks it cites, never declared, so a downgrade upstream
//!    propagates on the next build.
//! 2. **Cite a span, not a node.** Every entry below names verbatim text, and [`resolve`]
//!    asserts that text still appears in that node. A node reference alone rots invisibly —
//!    the node keeps its name while its content is rewritten and the citation still resolves.
//! 3. **A refusal in the cited block fails the build.** An assertion citing a node that
//!    refuses an inference must name that refusal in `answers`, and the refusal is exported
//!    alongside the assertion so it reaches the reader.
//!
//! # Two places this is stricter than the rule
//!
//! **Refusals are checked per node, not per block.** The rule says *a refusal in the cited
//! block*. This corpus's clearest case defeats that reading: `period/deindustrialization`
//! demonstrates a five-decade decline in one paragraph and refuses the reading that 1970 was
//! its peak in the next. A block-level check passes a population chart that cites the first
//! and silently drops the second, which is the exact failure the rule exists to prevent.
//!
//! **An `answers` entry matching no refusal is a defect too.** The same reasoning as the lint
//! baseline: a list of answers permitted to be wrong drifts, and one that over-lists silently
//! re-permits whatever it over-lists. If the corpus withdraws a refusal, the answer here has
//! to go with it.

use crate::load::Node;
use crate::tier::Tier;
use serde::Serialize;
use std::collections::BTreeMap;

/// A number an assertion plots, and the text it is read from.
///
/// A chart is where a corpus's care goes to die: the prose says 66.5 per cent, somebody
/// types 66.9 into an array, and every gate still passes because no rule reaches into the
/// data a chart is drawn from. `literal` is checked against the cited spans and `value` is
/// checked against `literal`, so a plotted number is only ever the number the corpus wrote.
pub struct Figure {
    /// What this number is, for the axis.
    pub label: &'static str,
    /// The number as it will be plotted.
    pub value: f64,
    /// The number exactly as the cited span writes it, separators and all.
    pub literal: &'static str,
}

/// A verbatim span of one corpus node, supporting an assertion.
pub struct Support {
    pub node: &'static str,
    pub span: &'static str,
}

/// An assertion the site makes, with everything beneath it named.
pub struct Assertion {
    pub id: &'static str,
    /// What the site says in its own words.
    pub statement: &'static str,
    /// Which view rests on this — the site renders an assertion beside what it justifies.
    pub topic: &'static str,
    pub supports: &'static [Support],
    /// Refusals in the cited nodes that this assertion carries rather than routes around.
    /// Each must be a fragment of an actual refusal sentence, and every refusal in every
    /// cited node must be matched by one of these.
    pub answers: &'static [&'static str],
    /// Numbers the site plots for this assertion, each quoted from a cited span.
    pub figures: &'static [Figure],
}

macro_rules! support {
    ($node:literal, $span:literal) => {
        Support {
            node: $node,
            span: $span,
        }
    };
}

/// Every assertion this site makes on its own authority.
///
/// Kept as one table rather than scattered through the pages, because the gate has to be
/// able to enumerate them. An assertion added to a page and not to this list is an
/// unsupported claim on a public site, which is the failure mode all of this is against.
pub const ASSERTIONS: &[Assertion] = &[
    Assertion {
        id: "county-population-decline",
        statement: "Allen County's population has fallen at every measured point since 1970, \
                    from 111,144 to 100,866 — a loss of 10,278 people, or 9.2 per cent.",
        topic: "population",
        supports: &[support!(
            "period/deindustrialization.yml",
            "Allen County counted 111,144 at the 1970 census and 100,866 at the 2024 estimate — a loss of 10,278 people, or 9.2%, across five decades with no reversal at any measured point: 111,144 (1970), 108,464 (2000), 106,331 (2010), 102,217 (2020), 100,866 (2024)"
        )],
        answers: &[
            "It does not establish that 1970 is the start",
            "It does not establish that either mechanism reaches back before 2020",
            "It does not establish that the decline has ended",
        ],
        // The chart on the population page is this enumeration and nothing else. The corpus
        // wrote the series out longhand, so the site plots the corpus's own five numbers
        // rather than re-selecting among the six measures — which would be an editorial
        // choice about comparability made silently, in a chart.
        figures: &[
            Figure { label: "1970", value: 111_144.0, literal: "111,144" },
            Figure { label: "2000", value: 108_464.0, literal: "108,464" },
            Figure { label: "2010", value: 106_331.0, literal: "106,331" },
            Figure { label: "2020", value: 102_217.0, literal: "102,217" },
            Figure { label: "2024", value: 100_866.0, literal: "100,866" },
        ],
    },
    Assertion {
        id: "decline-concentrated-in-lima",
        statement: "The loss is concentrated in Lima: between 2000 and 2010 the city fell 3.8 \
                    per cent while the rest of the county fell 1.5 per cent.",
        topic: "population",
        supports: &[support!(
            "period/deindustrialization.yml",
            "between 2000 and 2010 Lima fell 3.8% while the balance of Allen County — everything outside every incorporated place — fell 1.5%, from 50,809 to 50,048"
        )],
        answers: &[
            "It does not establish that 1970 is the start",
            "It does not establish that either mechanism reaches back before 2020",
            "It does not establish that the decline has ended",
        ],
        figures: &[
            Figure { label: "Lima", value: 3.8, literal: "3.8%" },
            Figure { label: "The rest of the county", value: 1.5, literal: "1.5%" },
        ],
    },
    Assertion {
        id: "decline-is-not-suburbanization",
        statement: "The suburban ring is shrinking too. Eleven of the county's thirteen civil \
                    subdivisions lost population between 2020 and 2024.",
        topic: "population",
        supports: &[support!(
            "period/deindustrialization.yml",
            "eleven of the county's thirteen civil subdivisions lost population"
        )],
        answers: &[
            "It does not establish that 1970 is the start",
            "It does not establish that either mechanism reaches back before 2020",
            "It does not establish that the decline has ended",
        ],
        figures: &[],
    },
    Assertion {
        id: "lima-was-this-size-in-1920",
        statement: "Lima was already about its present size a century ago: 41,306 people in \
                    1920, against 40,307 in 2000.",
        topic: "population",
        // Cited to the measure rather than to the period node, because the point of this
        // assertion is the figures and where they came from — three county histories, not the
        // Census Bureau — and the measure is the node that says so.
        supports: &[
            support!(
                "measure/lima-population-1880-1920.yml",
                "the city held **7,567 people in 1880, 21,723 in 1900 and 41,306 in 1920**"
            ),
            support!(
                "measure/lima-population-1880-1920.yml",
                "the city was **already at about its 2000 size in 1920**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1880", value: 7_567.0, literal: "7,567" },
            Figure { label: "1900", value: 21_723.0, literal: "21,723" },
            Figure { label: "1920", value: 41_306.0, literal: "41,306" },
        ],
    },
    Assertion {
        id: "decline-is-migration-and-deaths",
        statement: "Over the four full years to 2024 the county lost 1,271 people: 506 to \
                    natural decrease and 793 to net migration.",
        topic: "population",
        // Cited to the two measures rather than to the period node, so the figures come from
        // the nodes that carry the arithmetic and its method.
        supports: &[
            support!(
                "measure/allen-county-natural-change-2021-2024.yml",
                "Across the four full years 2021 to 2024 the county recorded **4,763 births and 5,269 deaths** — a natural change of **−506**"
            ),
            support!(
                "measure/allen-county-net-migration-2021-2024.yml",
                "Across the four full years 2021 to 2024 the county's net migration was **−793**: a domestic net loss of **1,217** against an international net gain of **424**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Natural decrease", value: -506.0, literal: "−506" },
            Figure { label: "Net migration", value: -793.0, literal: "−793" },
        ],
    },
    Assertion {
        id: "three-places",
        statement: "The county is three places: Lima, a suburban ring larger than the city, and \
                    nine rural townships.",
        topic: "geography",
        supports: &[support!(
            "place/allen-county.yml",
            "Lima holds 35,531 people; the three townships wrapped around it — [American](american-township.yml) 14,571, [Shawnee](shawnee-township.yml) 12,509 and [Bath](bath-township.yml) 9,399 — hold 36,479 between them, more than the city"
        )],
        answers: &[],
        figures: &[
            Figure { label: "Lima", value: 35_531.0, literal: "35,531" },
            Figure { label: "American Township", value: 14_571.0, literal: "14,571" },
            Figure { label: "Shawnee Township", value: 12_509.0, literal: "12,509" },
            Figure { label: "Bath Township", value: 9_399.0, literal: "9,399" },
        ],
    },
    Assertion {
        id: "republican-share-rising",
        statement: "Republican share of the top-of-ticket vote rose across three consecutive \
                    elections: 66.5 per cent in 2016, 67.3 in 2018, 69.0 in 2020.",
        topic: "elections",
        supports: &[support!(
            "measure/allen-county-presidential-vote-2020.yml",
            "Republican share of the top-of-ticket vote ran 66.5 per cent in [2016](allen-county-presidential-vote-2016.yml), 67.3 in [2018](allen-county-governor-vote-2018.yml) and 69.0 here — a move of 2.5 points across two presidential elections, against a median precinct move of 1.5"
        )],
        answers: &[],
        figures: &[
            Figure { label: "2016", value: 66.5, literal: "66.5" },
            Figure { label: "2018", value: 67.3, literal: "67.3" },
            Figure { label: "2020", value: 69.0, literal: "69.0" },
        ],
    },
    Assertion {
        id: "democratic-ground-moved-fastest",
        statement: "The county's most Democratic precincts moved right the fastest — a \
                    correlation of −0.39 between 2016 Republican share and the 2016–2020 swing.",
        topic: "elections",
        supports: &[support!(
            "measure/allen-county-presidential-vote-2020.yml",
            "Across the 88 the correlation between 2016 Republican share and the 2016–2020 swing is **−0.39**: the more Democratic a precinct was, the harder it moved"
        )],
        answers: &[],
        figures: &[Figure { label: "Correlation", value: -0.39, literal: "−0.39" }],
    },
    Assertion {
        id: "eleven-precincts",
        statement: "Eleven of 88 precincts went to Biden in 2020, and ten of them are in Lima.",
        topic: "elections",
        supports: &[support!(
            "measure/allen-county-presidential-vote-2020.yml",
            "**Eleven of the county's 88 precincts went to Biden, and ten of them are in Lima.**"
        )],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "an-address-is-not-a-municipality",
        statement: "A Lima postal address is not the city of Lima. The refinery has one and \
                    stands in Shawnee Township.",
        topic: "geography",
        supports: &[support!(
            "site/lima-refinery.yml",
            "**It has a Lima address and is not in Lima.** The county's address file records it at 1150 S Metcalf Street with `USPS_CITY` \"LIMA\" and `MUNI` **blank**, at 40.7221100, -84.1134691."
        )],
        answers: &[],
        figures: &[],
    },
];

/// One span of one node, as it survived the gate.
#[derive(Debug, Clone, Serialize)]
pub struct Citation {
    pub node: String,
    pub node_label: String,
    pub span: String,
    pub tier: Tier,
    /// Catalog entries the cited block names, as `catalog/<name>.md`.
    pub sources: Vec<String>,
}

/// A number that survived the gate, ready to plot.
#[derive(Debug, Clone, Serialize)]
pub struct PlottedFigure {
    pub label: String,
    pub value: f64,
    pub literal: String,
}

/// An assertion with its tier computed and its refusals attached.
#[derive(Debug, Clone, Serialize)]
pub struct Resolved {
    pub id: String,
    pub statement: String,
    pub topic: String,
    pub tier: Tier,
    pub citations: Vec<Citation>,
    /// Refusal sentences from the cited nodes, verbatim, for the site to render.
    pub caveats: Vec<String>,
    /// Numbers this assertion licenses the site to plot.
    pub figures: Vec<PlottedFigure>,
}

// `Eq` is not derived: `FigureMisread` carries the `f64` a chart would have plotted, and
// quoting it back is the whole point of the message.
#[derive(Debug, Clone, PartialEq)]
pub enum Defect {
    /// An assertion citing a node that is not in the corpus.
    NodeNotFound { assertion: String, node: String },
    /// A span that no longer appears in the node it names.
    SpanNotFound {
        assertion: String,
        node: String,
        span: String,
    },
    /// A span found in prose that carries no claim tag at all.
    UntaggedSpan {
        assertion: String,
        node: String,
        span: String,
    },
    /// A refusal in a cited node that the assertion does not carry.
    UnansweredRefusal {
        assertion: String,
        node: String,
        refusal: String,
    },
    /// An `answers` entry matching no refusal in any cited node.
    StaleAnswer { assertion: String, answer: String },
    /// A number the site plots that appears in none of the assertion's cited spans.
    FigureNotInSpan { assertion: String, literal: String },
    /// A number whose plotted value is not what its own quoted text says.
    FigureMisread {
        assertion: String,
        literal: String,
        value: f64,
    },
    /// An assertion too weak for the policy it is published under.
    BeyondPolicy {
        assertion: String,
        tier: Tier,
        ceiling: Tier,
    },
}

impl std::fmt::Display for Defect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Defect::NodeNotFound { assertion, node } => {
                write!(f, "{assertion}: cites {node}, which is not in the corpus")
            }
            Defect::SpanNotFound {
                assertion,
                node,
                span,
            } => write!(
                f,
                "{assertion}: {node} no longer contains the cited span.\n    «{}»\n    The far \
                 side changed its mind. The question is whether the assertion survives it — do \
                 not re-quote.",
                elide(span)
            ),
            Defect::UntaggedSpan {
                assertion,
                node,
                span,
            } => write!(
                f,
                "{assertion}: the span cited in {node} sits in prose carrying no claim tag.\n    \
                 «{}»",
                elide(span)
            ),
            Defect::UnansweredRefusal {
                assertion,
                node,
                refusal,
            } => write!(
                f,
                "{assertion}: {node} refuses an inference this assertion does not answer.\n    \
                 «{refusal}»\n    Answer it in `answers`, or withdraw the assertion. Routing \
                 around it is what this check exists to stop."
            ),
            Defect::StaleAnswer { assertion, answer } => write!(
                f,
                "{assertion}: answers a refusal no cited node makes any more.\n    «{answer}»\n \
                 The corpus withdrew it; withdraw the answer with it."
            ),
            Defect::FigureNotInSpan { assertion, literal } => write!(
                f,
                "{assertion}: plots «{literal}», which appears in none of the spans it cites.\n    \
                 A number on a chart is an assertion; quote it from the corpus or drop it."
            ),
            Defect::FigureMisread {
                assertion,
                literal,
                value,
            } => write!(f, "{assertion}: plots {value} but quotes «{literal}»"),
            Defect::BeyondPolicy {
                assertion,
                tier,
                ceiling,
            } => write!(
                f,
                "{assertion}: rests on {tier} ground and this feed publishes at {ceiling}"
            ),
        }
    }
}

fn elide(s: &str) -> String {
    if s.chars().count() <= 90 {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(89).collect::<String>())
    }
}

/// Read a quoted figure as a number.
///
/// Thousands separators and a trailing per-cent sign are presentation. Everything else is
/// refused rather than guessed at: a literal this cannot read is a literal nobody should be
/// plotting.
fn read_figure(literal: &str) -> Option<f64> {
    literal
        .trim()
        .trim_matches('*')
        .trim_end_matches('%')
        .replace(',', "")
        // The corpus writes a typographic minus, which `str::parse` does not accept.
        .replace('\u{2212}', "-")
        .parse()
        .ok()
}

/// Catalog entries a block's markdown links point at, as `catalog/<name>.md`.
fn sources(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find("catalog/") {
        let tail = &rest[at..];
        let end = tail.find(".md").map(|i| i + 3).unwrap_or(tail.len());
        let entry = &tail[..end];
        if entry.ends_with(".md") && !found.contains(&entry.to_string()) {
            found.push(entry.to_string());
        }
        rest = &tail[end..];
    }
    found
}

/// Check every assertion against the corpus and return the ones that survive.
///
/// Returns the resolved assertions and every defect found. Both, always: a caller that only
/// wanted the defects would still have to compute the assertions to find them, and a caller
/// that only wanted the assertions would be publishing unchecked ones.
pub fn resolve(
    assertions: &[Assertion],
    nodes: &[Node],
    ceiling: Tier,
) -> (Vec<Resolved>, Vec<Defect>) {
    let by_id: BTreeMap<&str, &Node> = nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let mut resolved = Vec::new();
    let mut defects = Vec::new();

    for a in assertions {
        let mut citations = Vec::new();
        let mut caveats: Vec<String> = Vec::new();
        let mut unanswered = Vec::new();
        let mut matched_answers: Vec<&str> = Vec::new();
        let mut failed = false;

        for s in a.supports {
            let Some(node) = by_id.get(s.node) else {
                defects.push(Defect::NodeNotFound {
                    assertion: a.id.into(),
                    node: s.node.into(),
                });
                failed = true;
                continue;
            };

            let wanted = crate::claim::normalize(s.span);
            let Some(block) = node.blocks.iter().find(|b| b.text.contains(&wanted)) else {
                defects.push(Defect::SpanNotFound {
                    assertion: a.id.into(),
                    node: s.node.into(),
                    span: wanted,
                });
                failed = true;
                continue;
            };

            let Some(tier) = block.tier else {
                defects.push(Defect::UntaggedSpan {
                    assertion: a.id.into(),
                    node: s.node.into(),
                    span: wanted,
                });
                failed = true;
                continue;
            };

            citations.push(Citation {
                node: s.node.into(),
                node_label: node.label.clone(),
                span: wanted,
                tier,
                sources: sources(&block.text),
            });

            // Refusals are gathered from the whole node, not only the cited block. See the
            // module docs for why this is stricter than the rule as written.
            for refusal in node.blocks.iter().filter_map(|b| b.refusal.as_deref()) {
                match a.answers.iter().find(|ans| refusal.contains(**ans)) {
                    Some(ans) => {
                        if !matched_answers.contains(ans) {
                            matched_answers.push(ans);
                        }
                        if !caveats.iter().any(|c| c == refusal) {
                            caveats.push(refusal.to_string());
                        }
                    }
                    None => {
                        if !unanswered.iter().any(|(_, r)| r == refusal) {
                            unanswered.push((s.node, refusal.to_string()));
                        }
                    }
                }
            }
        }

        for (node, refusal) in unanswered {
            defects.push(Defect::UnansweredRefusal {
                assertion: a.id.into(),
                node: node.into(),
                refusal,
            });
            failed = true;
        }

        for answer in a.answers {
            if !matched_answers.contains(answer) {
                defects.push(Defect::StaleAnswer {
                    assertion: a.id.into(),
                    answer: (*answer).into(),
                });
                failed = true;
            }
        }

        // A plotted number has to be quoted from a span this assertion already cites, and
        // has to be the number that quote says.
        let mut figures = Vec::new();
        for fig in a.figures {
            if !citations.iter().any(|c| c.span.contains(fig.literal)) {
                defects.push(Defect::FigureNotInSpan {
                    assertion: a.id.into(),
                    literal: fig.literal.into(),
                });
                failed = true;
                continue;
            }
            match read_figure(fig.literal) {
                Some(read) if (read - fig.value).abs() < f64::EPSILON => {}
                _ => {
                    defects.push(Defect::FigureMisread {
                        assertion: a.id.into(),
                        literal: fig.literal.into(),
                        value: fig.value,
                    });
                    failed = true;
                    continue;
                }
            }
            figures.push(PlottedFigure {
                label: fig.label.into(),
                value: fig.value,
                literal: fig.literal.into(),
            });
        }

        let Some(tier) = Tier::weakest(citations.iter().map(|c| c.tier)) else {
            continue; // Every support failed; the defects above say why.
        };

        if !tier.reaches(ceiling) {
            defects.push(Defect::BeyondPolicy {
                assertion: a.id.into(),
                tier,
                ceiling,
            });
            failed = true;
        }

        if !failed {
            resolved.push(Resolved {
                id: a.id.into(),
                statement: a.statement.into(),
                topic: a.topic.into(),
                tier,
                citations,
                caveats,
                figures,
            });
        }
    }

    (resolved, defects)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claim::blocks;
    use crate::load::Node;
    use std::collections::BTreeMap;

    fn node(id: &str, description: &str) -> Node {
        Node {
            id: id.into(),
            class: id.split('/').next().unwrap().into(),
            name: "n".into(),
            label: "A node".into(),
            properties: BTreeMap::new(),
            blocks: blocks(description),
            links: Vec::new(),
        }
    }

    const NO_ANSWERS: &[&str] = &[];

    fn assertion(supports: &'static [Support], answers: &'static [&'static str]) -> Assertion {
        Assertion {
            id: "a",
            statement: "s",
            topic: "t",
            supports,
            answers,
            figures: &[],
        }
    }

    fn plotting(supports: &'static [Support], figures: &'static [Figure]) -> Assertion {
        Assertion {
            id: "a",
            statement: "s",
            topic: "t",
            supports,
            answers: NO_ANSWERS,
            figures,
        }
    }

    #[test]
    fn an_assertion_inherits_the_weakest_block_it_cites() {
        let nodes = vec![
            node("place/x.yml", "Solid ground here. [verified]"),
            node("place/y.yml", "Softer ground here. [inference]"),
        ];
        let a = assertion(
            &[
                support!("place/x.yml", "Solid ground here."),
                support!("place/y.yml", "Softer ground here."),
            ],
            NO_ANSWERS,
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(defects.is_empty(), "{defects:?}");
        assert_eq!(ok[0].tier, Tier::Inference);
    }

    #[test]
    fn a_span_that_no_longer_appears_fails_the_build() {
        let nodes = vec![node("place/x.yml", "The figure is now 41. [verified]")];
        let a = assertion(
            &[support!("place/x.yml", "The figure is now 40.")],
            NO_ANSWERS,
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(ok.is_empty());
        assert!(
            matches!(defects[0], Defect::SpanNotFound { .. }),
            "{defects:?}"
        );
    }

    #[test]
    fn a_span_matches_across_the_hard_wrapping_of_the_file() {
        // The corpus wraps prose at about 95 columns; a cited sentence straddles that.
        let nodes = vec![node(
            "place/x.yml",
            "The county holds\n  402.545 square miles\n  of land. [verified]",
        )];
        let a = assertion(
            &[support!(
                "place/x.yml",
                "The county holds 402.545 square miles of land."
            )],
            NO_ANSWERS,
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(defects.is_empty(), "{defects:?}");
        assert_eq!(ok.len(), 1);
    }

    #[test]
    fn an_unanswered_refusal_in_another_block_of_a_cited_node_fails_the_build() {
        // The case the node-level rule exists for: the demonstration and the refusal of it
        // are neighbouring paragraphs of one node.
        let nodes = vec![node(
            "period/x.yml",
            "The county lost 10,278 people. [verified]\n\nIt does not establish that 1970 is \
             the start. [open]",
        )];
        let a = assertion(
            &[support!("period/x.yml", "The county lost 10,278 people.")],
            NO_ANSWERS,
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(ok.is_empty());
        assert!(
            matches!(defects[0], Defect::UnansweredRefusal { .. }),
            "{defects:?}"
        );
    }

    #[test]
    fn answering_a_refusal_carries_it_to_the_reader() {
        let nodes = vec![node(
            "period/x.yml",
            "The county lost 10,278 people. [verified]\n\nIt does not establish that 1970 is \
             the start. [open]",
        )];
        let a = assertion(
            &[support!("period/x.yml", "The county lost 10,278 people.")],
            &["does not establish that 1970 is the start"],
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(defects.is_empty(), "{defects:?}");
        assert_eq!(ok[0].caveats.len(), 1);
        assert!(ok[0].caveats[0].contains("1970 is the start"));
        // And the refusal reaches the reader even though it sits in an `[open]` block:
        // a refusal narrows what is asserted, so carrying it can never widen it.
    }

    #[test]
    fn an_answer_matching_no_refusal_fails_the_build() {
        let nodes = vec![node("place/x.yml", "Solid ground. [verified]")];
        let a = assertion(
            &[support!("place/x.yml", "Solid ground.")],
            &["does not establish that 1970 is the start"],
        );
        let (_, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(
            matches!(defects[0], Defect::StaleAnswer { .. }),
            "{defects:?}"
        );
    }

    #[test]
    fn an_assertion_resting_on_open_ground_does_not_publish() {
        let nodes = vec![node("place/x.yml", "A guess about the county. [open]")];
        let a = assertion(
            &[support!("place/x.yml", "A guess about the county.")],
            NO_ANSWERS,
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(ok.is_empty());
        assert!(
            matches!(defects[0], Defect::BeyondPolicy { .. }),
            "{defects:?}"
        );
    }

    #[test]
    fn a_span_in_untagged_prose_does_not_publish() {
        let nodes = vec![node(
            "place/x.yml",
            "This node exists to argue about the class.",
        )];
        let a = assertion(
            &[support!(
                "place/x.yml",
                "This node exists to argue about the class."
            )],
            NO_ANSWERS,
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(ok.is_empty());
        assert!(
            matches!(defects[0], Defect::UntaggedSpan { .. }),
            "{defects:?}"
        );
    }

    #[test]
    fn a_plotted_number_must_be_quoted_from_a_cited_span() {
        let nodes = vec![node(
            "measure/x.yml",
            "The share ran 66.5 per cent. [verified]",
        )];
        let a = plotting(
            &[support!("measure/x.yml", "The share ran 66.5 per cent.")],
            &[Figure {
                label: "2016",
                value: 66.5,
                literal: "66.5",
            }],
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(defects.is_empty(), "{defects:?}");
        assert_eq!(ok[0].figures[0].value, 66.5);
    }

    #[test]
    fn a_number_the_corpus_never_wrote_does_not_reach_a_chart() {
        let nodes = vec![node(
            "measure/x.yml",
            "The share ran 66.5 per cent. [verified]",
        )];
        let a = plotting(
            &[support!("measure/x.yml", "The share ran 66.5 per cent.")],
            &[Figure {
                label: "2016",
                value: 66.9,
                literal: "66.9",
            }],
        );
        let (ok, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(ok.is_empty());
        assert!(
            matches!(defects[0], Defect::FigureNotInSpan { .. }),
            "{defects:?}"
        );
    }

    #[test]
    fn a_figure_plotted_at_odds_with_its_own_quote_fails_the_build() {
        // The typo that no other check can see: the quote is real, the array is not.
        let nodes = vec![node(
            "measure/x.yml",
            "The share ran 66.5 per cent. [verified]",
        )];
        let a = plotting(
            &[support!("measure/x.yml", "The share ran 66.5 per cent.")],
            &[Figure {
                label: "2016",
                value: 65.5,
                literal: "66.5",
            }],
        );
        let (_, defects) = resolve(std::slice::from_ref(&a), &nodes, Tier::Inference);
        assert!(
            matches!(defects[0], Defect::FigureMisread { .. }),
            "{defects:?}"
        );
    }

    #[test]
    fn a_thousands_separator_and_a_typographic_minus_both_read() {
        assert_eq!(read_figure("35,531"), Some(35_531.0));
        assert_eq!(read_figure("\u{2212}0.39"), Some(-0.39));
        assert_eq!(read_figure("3.8%"), Some(3.8));
        assert_eq!(read_figure("eleven"), None);
    }

    #[test]
    fn catalog_links_in_the_cited_block_become_the_citation_sources() {
        let text = "A figure. [verified] — [Gazetteer](../../catalog/census-gazetteer-2020.md).";
        assert_eq!(sources(text), vec!["catalog/census-gazetteer-2020.md"]);
    }

    #[test]
    fn a_block_naming_one_source_twice_names_it_once() {
        let text = "[a](../../catalog/x.md) and [b](../../catalog/x.md)";
        assert_eq!(sources(text), vec!["catalog/x.md"]);
    }
}
