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

pub mod load;

use std::collections::BTreeMap;

/// Relationships that describe the corpus rather than the world, and take no claim tag.
pub const STRUCTURAL: [&str; 3] = ["instance-of", "concerns", "subject-of"];

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
}

impl Edge {
    pub fn is_structural(&self) -> bool {
        STRUCTURAL.contains(&self.relationship.as_str())
    }
    pub fn tag(&self) -> Option<Tag> {
        self.raw_tag.as_deref().and_then(Tag::parse)
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
}

impl std::fmt::Display for DefectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DefectKind::Untagged => "no claim_tag",
            DefectKind::UnknownTag => "claim_tag is not verified|inference|open",
            DefectKind::VerifiedUnsourced => "verified with no source",
            DefectKind::SourceNotInCatalog => "source is not a catalog entry",
            DefectKind::TaggedStructural => "structural edge carrying a claim_tag",
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

        let tag = e.tag().map(|t| t.to_string()).unwrap_or("—".into());
        *a.by_tag.entry(tag.clone()).or_default() += 1;
        *a.by_shape
            .entry(format!("{} --{}->", e.class, e.relationship))
            .or_default()
            .entry(tag)
            .or_default() += 1;
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
