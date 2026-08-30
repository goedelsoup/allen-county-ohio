//! Cutting a node's prose into claims, and finding the sentences that refuse one.
//!
//! A corpus node's `description` is not one claim; it is several paragraphs, each carrying
//! its own tag. Publishing the description whole would mean publishing every paragraph at
//! the tier of the strongest one in it, which is exactly backwards. So prose is cut at blank
//! lines into [`Block`]s, and each block travels on its own tag.
//!
//! # An untagged block does not publish
//!
//! Ninety-nine of this corpus's 620 blocks carry no tag, and reading them says why: they are
//! commentary about the corpus rather than claims about the county — *this node exists for
//! the same reason that one does*, *that makes this the sharpest argument for the class*.
//! There is no tag to travel on and no reader outside this repository is served by them.
//! Refusing them is also the conservative direction: an untagged block that *is* a claim is
//! an untagged inference, which the conventions call a defect, and publishing it would be
//! publishing the defect.

use crate::tier::Tier;
use serde::Serialize;

/// A paragraph of a node's description, with the tag it travels on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Block {
    pub text: String,
    /// The weakest tag appearing in the block; `None` when it carries no tag at all.
    pub tier: Option<Tier>,
    /// A sentence in this block that refuses an inference, if there is one.
    pub refusal: Option<String>,
}

impl Block {
    /// Whether this block may appear in material published at `ceiling`.
    pub fn publishable(&self, ceiling: Tier) -> bool {
        self.tier.is_some_and(|t| t.reaches(ceiling))
    }
}

/// The weakest claim tag in `text`, or `None` if it carries none.
///
/// Used for prose blocks and for property values alike. The two are then treated
/// differently — see [`publishable_property`] — because a `geoid` with no tag is structured
/// data, while a paragraph with no tag is untagged prose.
pub fn tier_of(text: &str) -> Option<Tier> {
    Tier::weakest(tags(text))
}

/// Whether a property value may appear in material published at `ceiling`.
///
/// The rule inverts the one for prose. An untagged paragraph does not publish, because prose
/// asserting something without saying what kind of claim it is has no tag to travel on. An
/// untagged property *does*, because `geoid: "39003"` is a structured field and not a claim
/// anyone tagged. What a property may not do is carry a tag too weak for the ceiling — which
/// is how three `[open]` `boundary_basis` fields were, before this existed, the one thing on
/// this site that had left the repository without passing a rule.
pub fn publishable_property(value: &str, ceiling: Tier) -> bool {
    tier_of(value).is_none_or(|t| t.reaches(ceiling))
}

/// Every `[verified]`, `[inference]` or `[open]` marker in `text`, in order.
fn tags(text: &str) -> Vec<Tier> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut i = 0;
    while let Some(open) = text[i..].find('[') {
        let start = i + open;
        match text[start..].find(']') {
            Some(close) => {
                let end = start + close;
                if let Some(t) = Tier::parse(&text[start + 1..end]) {
                    found.push(t);
                }
                i = end + 1;
            }
            None => break,
        }
    }
    let _ = bytes;
    found
}

/// Sentence openings that mark a refusal rather than an ordinary negation.
///
/// Deliberately narrow. A loose pattern matches seventy-five of this corpus's blocks, almost
/// all of them ordinary prose containing the word *not*; these phrasings match four, and all
/// four are the thing the rule is about — a node stating a fact and declining the inference
/// from it in the next sentence.
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

/// Abbreviations whose period is not a sentence end.
///
/// Narrow on purpose, like `REFUSALS`. Without this a refusal that names St. Rita's is quoted
/// from the middle of the markdown link — `Rita's](mercy-health-st-ritas-medical-center.yml),
/// or of neither, the corpus does not know.` — which is what the public page rendered before
/// this list existed.
const ABBREVIATIONS: [&str; 7] = ["St.", "Mr.", "Mrs.", "Ms.", "Dr.", "Jr.", "Sr."];

/// The index just after the last real sentence end before `at`, or the start of the block.
///
/// The block start is the right fallback and a line break is not. Node prose is hard-wrapped
/// at about 95 columns — that is why `normalize` exists — so treating `\n` as a boundary
/// truncated every refusal whose sentence ran past one wrap. The rule this corpus wrote down
/// was "a refusal must start its own block"; the rule the code enforced was "…and fit on one
/// line", and nothing said so.
fn sentence_start(text: &str, at: usize) -> usize {
    let mut head = &text[..at];
    while let Some(i) = head.rfind(". ") {
        if !ABBREVIATIONS.iter().any(|a| head[..=i].ends_with(a)) {
            return i + 2;
        }
        head = &head[..i];
    }
    0
}

/// The index just after the first real sentence end at or after `at`, or the end of the block.
fn sentence_end(text: &str, at: usize) -> usize {
    let mut from = at;
    while let Some(i) = text[from..].find(". ") {
        let period = from + i;
        if !ABBREVIATIONS.iter().any(|a| text[..=period].ends_with(a)) {
            return period + 1;
        }
        from = period + 2;
    }
    text.len()
}

/// The first refusing sentence in `text`, verbatim, if there is one.
///
/// The search runs over the *normalized* block, not the raw one. Node prose is hard-wrapped at
/// about 95 columns, so a trigger phrase can straddle a wrap — `this corpus does not\n  know` —
/// and a raw substring search does not see it. Three of the corpus's twenty-six refusals were
/// invisible to this function for that reason, and an assertion citing one of those nodes would
/// have published without the caveat. That is the exact failure the gate exists to prevent, and
/// it failed silently, because a refusal nobody detects looks identical to a node with none.
fn refusal(raw: &str) -> Option<String> {
    let text = &normalize(raw);
    let lower = text.to_lowercase();
    let at = REFUSALS.iter().filter_map(|p| lower.find(p)).min()?;

    // Widen the hit to the sentence around it, so the reader gets the refusal and not a
    // fragment. Sentence ends are approximated by `. ` outside a short abbreviation list —
    // good enough for prose already cut into paragraphs, and erring long is the safe direction.
    let start = sentence_start(text, at);
    let end = sentence_end(text, at);

    // Claim markers are annotation, not prose. A refusal quoted with a stray `[verified]`
    // in front of it reads as though the tag belonged to the refusal.
    let mut quote = text[start..end].to_string();
    for marker in ["[verified]", "[inference]", "[open]"] {
        quote = quote.replace(marker, " ");
    }
    Some(normalize(&quote))
}

/// Collapse the corpus's hard-wrapped prose into single-spaced text.
///
/// Node descriptions are wrapped at about 95 columns, so a sentence quoted out of one
/// carries newlines that are an artifact of the file rather than of the claim. Spans are
/// compared after this, which is what lets an assertion cite a sentence that happens to
/// straddle a line break.
pub fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Cut a description into blocks at blank lines.
pub fn blocks(description: &str) -> Vec<Block> {
    description
        .split("\n\n")
        .map(str::trim)
        .filter(|b| !b.is_empty())
        .map(|b| Block {
            tier: tier_of(b),
            refusal: refusal(b),
            text: normalize(b),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_block_travels_on_its_weakest_tag() {
        let b = blocks("Two claims here. [verified] And a softer one. [inference]");
        assert_eq!(b[0].tier, Some(Tier::Inference));
    }

    #[test]
    fn blank_lines_cut_prose_into_separately_tagged_blocks() {
        let b = blocks("Solid ground. [verified]\n\nA guess. [open]");
        assert_eq!(b.len(), 2);
        assert_eq!(b[0].tier, Some(Tier::Verified));
        assert_eq!(b[1].tier, Some(Tier::Open));
        // The point of cutting: the verified half still publishes.
        assert!(b[0].publishable(Tier::Inference));
        assert!(!b[1].publishable(Tier::Inference));
    }

    #[test]
    fn an_untagged_block_publishes_nowhere() {
        let b = blocks("This node exists for the same reason that one does.");
        assert_eq!(b[0].tier, None);
        for ceiling in [Tier::Verified, Tier::Inference, Tier::Open] {
            assert!(!b[0].publishable(ceiling));
        }
    }

    #[test]
    fn a_bracket_that_is_not_a_tag_is_not_a_tag() {
        // Corpus prose is full of markdown links; `[2020 Census]` is not a claim tag.
        let b = blocks("Land area is 402.545 square miles. [2020 Census Gazetteer](x.md)");
        assert_eq!(b[0].tier, None);
    }

    #[test]
    fn an_unclosed_bracket_does_not_run_off_the_end() {
        assert_eq!(blocks("A sentence with [an unclosed bracket")[0].tier, None);
    }

    #[test]
    fn a_refusal_is_caught_and_quoted_as_a_whole_sentence() {
        let b = blocks(
            "The county counted 111,144 in 1970. [verified] It does not establish that 1970 \
             is the start. More follows.",
        );
        let r = b[0].refusal.as_deref().expect("refusal found");
        assert_eq!(r, "It does not establish that 1970 is the start.");
    }

    #[test]
    fn a_refusal_that_runs_past_a_line_wrap_is_quoted_whole() {
        // The defect this test is named for shipped to the public site. Node prose is wrapped
        // at about 95 columns, and the widener treated a wrap as a sentence boundary, so a
        // refusal in a block's first sentence was published from the second line onward.
        let text = "Whether a quarter of this hospital's workforce left the county or moved\n\
                    to another employer inside the same system, the corpus cannot say. A renaming \
                    that coincides is as good an explanation. [open]";
        let b = blocks(text);
        let r = b[0].refusal.as_deref().expect("refusal found");
        assert!(
            r.starts_with("Whether a quarter"),
            "quoted from the wrong place: {r}"
        );
        assert!(
            r.ends_with("the corpus cannot say."),
            "quoted to the wrong place: {r}"
        );
    }

    #[test]
    fn an_abbreviation_is_not_a_sentence_end() {
        // Also shipped: `Rita's](mercy-health-st-ritas-medical-center.yml), or of neither, the
        // corpus does not know.` — a refusal quoted from inside a markdown link, because the
        // period in "St." looked like the end of the previous sentence.
        let text = "Whether that society is an ancestor of this hospital, of \
                    [St. Rita's](mercy-health-st-ritas-medical-center.yml), or of neither, the \
                    corpus does not know. [open]";
        let b = blocks(text);
        let r = b[0].refusal.as_deref().expect("refusal found");
        assert!(
            r.starts_with("Whether that society"),
            "quoted from the wrong place: {r}"
        );
    }

    #[test]
    fn a_trigger_phrase_split_by_a_line_wrap_is_still_found() {
        // The worst of the three wrapping defects, because it fails to nothing. The phrase
        // straddles the wrap, the raw substring search misses it, and the block looks like
        // prose with no refusal in it — so an assertion citing the node publishes with no
        // caveat and no complaint. Three of the corpus's refusals were hidden this way.
        let text = "What changed in 1994 is who filed. Whether who operated changed with it,\n\
                    this corpus does not\n  know, and the rest is a compliance arrangement. [open]";
        let b = blocks(text);
        let r = b[0]
            .refusal
            .as_deref()
            .expect("refusal found across the wrap");
        assert!(r.contains("does not know"), "quoted wrong: {r}");
    }

    #[test]
    fn ordinary_negation_is_not_a_refusal() {
        // The failure a loose pattern has: this is a claim, not a refusal of one.
        let b = blocks("Shawnee Township contains no incorporated place at all. [verified]");
        assert_eq!(b[0].refusal, None);
        let b = blocks("This is not the map in force now. [verified]");
        assert_eq!(b[0].refusal, None);
    }

    #[test]
    fn an_untagged_property_is_structured_data_and_publishes() {
        assert!(publishable_property("39003", Tier::Verified));
        assert!(publishable_property(
            "40.771627, -84.106103",
            Tier::Verified
        ));
    }

    #[test]
    fn a_property_tagged_open_does_not_publish() {
        // The leak this function was written for: `boundary_basis` on a period node.
        assert!(!publishable_property(
            "Both bounds are unsourced approximations. [open]",
            Tier::Inference
        ));
    }

    #[test]
    fn a_property_tagged_inference_publishes_under_an_inference_ceiling() {
        let v = "St. Marys, in present-day Auglaize County, Ohio. [inference]";
        assert!(publishable_property(v, Tier::Inference));
        assert!(!publishable_property(v, Tier::Verified));
    }

    #[test]
    fn hard_wrapped_prose_normalizes_to_one_line() {
        assert_eq!(
            normalize("a sentence\n  broken over\n  lines"),
            "a sentence broken over lines"
        );
    }
}
