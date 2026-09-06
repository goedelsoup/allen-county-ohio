//! Whether every edge in the corpus says what kind of claim it is.
//!
//! The corpus's conventions say "an edge is a claim". Its prose claims must carry `[verified]`,
//! `[inference]` or `[open]` and `verified-unsourced` is a lint **error**; its links carried
//! `target` and `relationship` and nothing else. That asymmetry is where six phases of location
//! errors lived, so the corpus now tags its edges — and this is what keeps them tagged.
//!
//! # Why this is a crate and not a lint rule
//!
//! `graph-lint` comes from `.yidam/.vendor/`, which is read-only and fixed by re-vendoring
//! rather than by editing. The rule this corpus needs does not exist upstream yet. So the
//! domain computer enforces it instead: [`audit`] returns defects, `tests/corpus.rs` fails on
//! any of them, and `mise run ci` runs `cargo test`. The gate is the same gate.
//!
//! It is the first thing in `crates/` that checks the corpus rather than querying it.
//!
//! # What counts as an edge that needs a tag
//!
//! Edges that assert something about the world. [`STRUCTURAL`] relationships — `instance-of`,
//! `concerns`, `subject-of` — are statements about the corpus itself: which class a node
//! instantiates, which nodes a question is about. Tagging those would be a category error, so
//! carrying a tag on one is itself reported.
//!
//! # The comparability edges carry one field more
//!
//! [`COMPARABILITY`] — `comparable-to` and `not-comparable-to` — say whether two figures may be
//! set beside each other, and they carry `because` on top of the tag. The tag says how well the
//! judgement is evidenced; `because` says what the judgement *is about*, which for these two is
//! the whole content: "not comparable" with no reason is the bare assertion the edge exists to
//! replace, and a bare "comparable" is worse, because it licenses a subtraction and shows
//! nobody's work. So an unexplained one is a defect here, the same way an untagged edge is.

pub mod load;

use std::collections::{BTreeMap, BTreeSet};

/// Relationships that describe the corpus rather than the world, and take no claim tag.
pub const STRUCTURAL: [&str; 3] = ["instance-of", "concerns", "subject-of"];

/// Relationships that judge two figures against each other, and must say why.
pub const COMPARABILITY: [&str; 2] = ["comparable-to", "not-comparable-to"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tag {
    Verified,
    Inference,
    Open,
}

impl Tag {
    pub fn parse(s: &str) -> Option<Tag> {
        match s.trim() {
            "verified" => Some(Tag::Verified),
            "inference" => Some(Tag::Inference),
            "open" => Some(Tag::Open),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Verified => write!(f, "verified"),
            Tag::Inference => write!(f, "inference"),
            Tag::Open => write!(f, "open"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// `place/lima.yml`.
    pub node: String,
    pub class: String,
    pub relationship: String,
    /// As written in the file, e.g. `../place/allen-county.yml`.
    pub target: String,
    /// The raw `claim_tag` string, if any — kept unparsed so a typo is reported rather than
    /// silently read as untagged.
    pub raw_tag: Option<String>,
    pub source: Option<String>,
    /// Why the two figures may or may not be set beside each other. Only the comparability
    /// relationships carry it, and both of them must.
    pub because: Option<String>,
}

impl Edge {
    pub fn is_structural(&self) -> bool {
        STRUCTURAL.contains(&self.relationship.as_str())
    }
    pub fn is_comparability(&self) -> bool {
        COMPARABILITY.contains(&self.relationship.as_str())
    }
    pub fn tag(&self) -> Option<Tag> {
        self.raw_tag.as_deref().and_then(Tag::parse)
    }

    /// `class/name.yml`, the same identity [`Edge::node`] carries.
    ///
    /// Corpus links are relative to the writing node's class directory — `../place/lima.yml`
    /// from a measure, a bare `sibling.yml` inside one class. Taken from `node` rather than
    /// from `class`, so a file whose `class:` field and directory ever disagree resolves
    /// against the directory the link was actually written in.
    pub fn resolved(&self) -> String {
        let dir = self.node.split('/').next().unwrap_or_default();
        match self.target.strip_prefix("../") {
            Some(rest) => rest.to_string(),
            None => format!("{dir}/{}", self.target),
        }
    }

    /// The two ends of this edge, ordered — so A→B and B→A land on the same key.
    fn pair(&self) -> (String, String) {
        let (a, b) = (self.node.clone(), self.resolved());
        if a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }
}

/// One thing wrong with one edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Defect {
    pub edge: Edge,
    pub kind: DefectKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefectKind {
    /// An edge asserting something about the world with no `claim_tag`.
    Untagged,
    /// A `claim_tag` that is not one of the three.
    UnknownTag,
    /// `verified` with no `source`. The prose lint of the same name is an error upstream; this
    /// is the same rule applied to the claims the upstream lint cannot see.
    VerifiedUnsourced,
    /// A `source` naming a catalog entry that is not in `.yidam/catalog/`.
    SourceNotInCatalog,
    /// A structural edge carrying a claim tag — `instance-of` is not a claim about the world.
    TaggedStructural,
    /// A comparability edge with no `because`. The reason is the claim, not an annotation on it.
    ComparabilityUnexplained,
    /// A comparability edge pointing outside `measure/`. Comparability is a relation between two
    /// figures; pointed at the place a figure is about, it says nothing.
    ComparabilityOffClass,
    /// One pair of measures judged both comparable and not comparable. Direction does not make
    /// them two claims — `a comparable-to b` and `b not-comparable-to a` are one contradiction.
    ComparabilityContradicted,
    /// `because` on a relationship that is not a comparability judgement. Left unreported it
    /// becomes a general-purpose note field, and then the two edges that require it cannot be
    /// told from the ones that happen to carry it.
    BecauseWithoutAJudgement,
}

impl std::fmt::Display for DefectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DefectKind::Untagged => "no claim_tag",
            DefectKind::UnknownTag => "claim_tag is not verified|inference|open",
            DefectKind::VerifiedUnsourced => "verified with no source",
            DefectKind::SourceNotInCatalog => "source is not a catalog entry",
            DefectKind::TaggedStructural => "structural edge carrying a claim_tag",
            DefectKind::ComparabilityUnexplained => "comparability judgement with no because",
            DefectKind::ComparabilityOffClass => {
                "comparability judgement pointing outside measure/"
            }
            DefectKind::ComparabilityContradicted => {
                "one pair judged both comparable and not comparable"
            }
            DefectKind::BecauseWithoutAJudgement => "because on a non-comparability edge",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Audit {
    pub defects: Vec<Defect>,
    /// Empirical edges by tag.
    pub by_tag: BTreeMap<String, usize>,
    /// Empirical edges by `class --relationship->`, then by tag. This is the readable output:
    /// it shows at a glance which parts of the graph are sourced and which are recalled.
    pub by_shape: BTreeMap<String, BTreeMap<String, usize>>,
    pub structural: usize,
    pub empirical: usize,
}

impl Audit {
    pub fn is_clean(&self) -> bool {
        self.defects.is_empty()
    }
}

/// Check every edge, given the set of catalog entry filenames that exist.
pub fn audit(edges: &[Edge], catalog: &[String]) -> Audit {
    let mut a = Audit::default();
    for e in edges {
        if e.is_structural() {
            a.structural += 1;
            if e.raw_tag.is_some() {
                a.defects.push(Defect {
                    edge: e.clone(),
                    kind: DefectKind::TaggedStructural,
                });
            }
            continue;
        }
        a.empirical += 1;

        let kind = match (&e.raw_tag, e.tag()) {
            (None, _) => Some(DefectKind::Untagged),
            (Some(_), None) => Some(DefectKind::UnknownTag),
            (Some(_), Some(Tag::Verified)) if e.source.is_none() => {
                Some(DefectKind::VerifiedUnsourced)
            }
            _ => None,
        };
        if let Some(kind) = kind {
            a.defects.push(Defect {
                edge: e.clone(),
                kind,
            });
        }

        if let Some(src) = &e.source {
            let file = src.rsplit('/').next().unwrap_or(src);
            if !catalog.iter().any(|c| c == file) {
                a.defects.push(Defect {
                    edge: e.clone(),
                    kind: DefectKind::SourceNotInCatalog,
                });
            }
        }

        if e.is_comparability() {
            if e.because.as_deref().map(str::trim).unwrap_or("").is_empty() {
                a.defects.push(Defect {
                    edge: e.clone(),
                    kind: DefectKind::ComparabilityUnexplained,
                });
            }
            if !e.resolved().starts_with("measure/") {
                a.defects.push(Defect {
                    edge: e.clone(),
                    kind: DefectKind::ComparabilityOffClass,
                });
            }
        } else if e.because.is_some() {
            a.defects.push(Defect {
                edge: e.clone(),
                kind: DefectKind::BecauseWithoutAJudgement,
            });
        }

        let tag = e.tag().map(|t| t.to_string()).unwrap_or("—".into());
        *a.by_tag.entry(tag.clone()).or_default() += 1;
        *a.by_shape
            .entry(format!("{} --{}->", e.class, e.relationship))
            .or_default()
            .entry(tag)
            .or_default() += 1;
    }

    // The one check that cannot be made edge by edge. Comparability is symmetric — the corpus
    // writes it on the later figure only, so nothing stops a second phase writing the opposite
    // judgement from the other end, and both files would read as correct on their own.
    let mut judged: BTreeMap<(String, String), BTreeSet<&str>> = BTreeMap::new();
    for e in edges.iter().filter(|e| e.is_comparability()) {
        judged
            .entry(e.pair())
            .or_default()
            .insert(e.relationship.as_str());
    }
    for e in edges.iter().filter(|e| e.is_comparability()) {
        if judged.get(&e.pair()).is_some_and(|r| r.len() > 1) {
            a.defects.push(Defect {
                edge: e.clone(),
                kind: DefectKind::ComparabilityContradicted,
            });
        }
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(class: &str, rel: &str, tag: Option<&str>, src: Option<&str>) -> Edge {
        Edge {
            node: format!("{class}/n.yml"),
            class: class.into(),
            relationship: rel.into(),
            target: "../place/x.yml".into(),
            raw_tag: tag.map(str::to_string),
            source: src.map(str::to_string),
            because: None,
        }
    }

    /// A comparability edge from `measure/from.yml` to a sibling measure.
    fn cmp(from: &str, rel: &str, to: &str, because: Option<&str>) -> Edge {
        Edge {
            node: format!("measure/{from}.yml"),
            class: "measure".into(),
            relationship: rel.into(),
            target: format!("{to}.yml"),
            raw_tag: Some("inference".into()),
            source: None,
            because: because.map(str::to_string),
        }
    }
    fn catalog() -> Vec<String> {
        vec!["a.md".to_string()]
    }

    #[test]
    fn an_untagged_empirical_edge_is_a_defect() {
        let a = audit(&[e("place", "within", None, None)], &catalog());
        assert_eq!(a.defects.len(), 1);
        assert_eq!(a.defects[0].kind, DefectKind::Untagged);
        assert!(!a.is_clean());
    }

    #[test]
    fn a_structural_edge_needs_no_tag_and_must_not_have_one() {
        let clean = audit(&[e("place", "instance-of", None, None)], &catalog());
        assert!(clean.is_clean());
        assert_eq!(clean.structural, 1);
        assert_eq!(clean.empirical, 0);

        // `instance-of` is a statement about the corpus. Tagging it as evidence about the world
        // is a category error, and a silent one.
        let tagged = audit(
            &[e("place", "instance-of", Some("verified"), None)],
            &catalog(),
        );
        assert_eq!(tagged.defects[0].kind, DefectKind::TaggedStructural);
    }

    #[test]
    fn verified_without_a_source_is_the_same_defect_the_prose_lint_catches() {
        let a = audit(&[e("place", "within", Some("verified"), None)], &catalog());
        assert_eq!(a.defects[0].kind, DefectKind::VerifiedUnsourced);
        // With one, it is clean.
        let ok = audit(
            &[e(
                "place",
                "within",
                Some("verified"),
                Some("../../catalog/a.md"),
            )],
            &catalog(),
        );
        assert!(ok.is_clean());
    }

    #[test]
    fn inference_and_open_need_no_source() {
        for t in ["inference", "open"] {
            assert!(audit(&[e("place", "within", Some(t), None)], &catalog()).is_clean());
        }
    }

    #[test]
    fn a_source_that_names_no_catalog_entry_is_a_defect() {
        let a = audit(
            &[e(
                "place",
                "within",
                Some("verified"),
                Some("../../catalog/nope.md"),
            )],
            &catalog(),
        );
        assert_eq!(a.defects[0].kind, DefectKind::SourceNotInCatalog);
    }

    #[test]
    fn a_misspelt_tag_is_reported_rather_than_read_as_untagged() {
        // The failure this guards: `claim_tag: verifed` silently becoming "no tag", which the
        // Untagged check would report as a missing tag and a careless fix would re-add.
        let a = audit(&[e("place", "within", Some("verifed"), None)], &catalog());
        assert_eq!(a.defects[0].kind, DefectKind::UnknownTag);
    }

    #[test]
    fn a_comparability_judgement_must_say_why() {
        let bare = audit(&[cmp("b", "not-comparable-to", "a", None)], &catalog());
        assert_eq!(bare.defects[0].kind, DefectKind::ComparabilityUnexplained);

        // Whitespace is not a reason either.
        let blank = audit(&[cmp("b", "comparable-to", "a", Some("  "))], &catalog());
        assert_eq!(blank.defects[0].kind, DefectKind::ComparabilityUnexplained);

        let ok = audit(
            &[cmp(
                "b",
                "comparable-to",
                "a",
                Some("one definition, one boundary"),
            )],
            &catalog(),
        );
        assert!(ok.is_clean());
    }

    #[test]
    fn a_comparability_judgement_points_at_another_figure_and_not_at_a_place() {
        let mut off = cmp("b", "comparable-to", "a", Some("why"));
        off.target = "../place/allen-county.yml".into();
        let a = audit(&[off], &catalog());
        assert_eq!(a.defects[0].kind, DefectKind::ComparabilityOffClass);
    }

    #[test]
    fn because_is_not_a_general_purpose_note_field() {
        let mut stray = e("place", "within", Some("inference"), None);
        stray.because = Some("it just is".into());
        let a = audit(&[stray], &catalog());
        assert_eq!(a.defects[0].kind, DefectKind::BecauseWithoutAJudgement);
    }

    #[test]
    fn one_pair_cannot_be_judged_both_ways_from_its_two_ends() {
        // Each file reads as correct alone. The corpus writes the judgement on the later figure
        // only, so nothing but this check stands between two phases and a graph that says both.
        let a = audit(
            &[
                cmp("b", "comparable-to", "a", Some("same definition")),
                cmp("a", "not-comparable-to", "b", Some("the threshold moved")),
            ],
            &catalog(),
        );
        assert_eq!(a.defects.len(), 2);
        assert!(a
            .defects
            .iter()
            .all(|d| d.kind == DefectKind::ComparabilityContradicted));

        // Said twice the same way it is redundant, not contradictory.
        let agreeing = audit(
            &[
                cmp("b", "comparable-to", "a", Some("same definition")),
                cmp("a", "comparable-to", "b", Some("same definition")),
            ],
            &catalog(),
        );
        assert!(agreeing.is_clean());
    }

    #[test]
    fn a_link_resolves_against_the_directory_it_was_written_in() {
        assert_eq!(
            cmp("b", "comparable-to", "a", Some("why")).resolved(),
            "measure/a.yml"
        );
        assert_eq!(
            e("measure", "describes", Some("inference"), None).resolved(),
            "place/x.yml"
        );
    }

    #[test]
    fn the_shape_table_separates_sourced_edges_from_recalled_ones() {
        let a = audit(
            &[
                e("place", "within", Some("verified"), Some("a.md")),
                e("place", "within", Some("inference"), None),
                e("place", "governed-by", Some("inference"), None),
            ],
            &catalog(),
        );
        assert!(a.is_clean());
        assert_eq!(a.by_tag["verified"], 1);
        assert_eq!(a.by_tag["inference"], 2);
        assert_eq!(a.by_shape["place --within->"]["verified"], 1);
        assert_eq!(a.by_shape["place --governed-by->"].get("verified"), None);
    }
}
