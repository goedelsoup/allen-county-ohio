//! The three claim tags, and the one rule that governs how far a claim travels.
//!
//! [`Tier`] orders the tags by strength — `Verified` strongest, `Open` weakest — so that
//! "the minimum tag across the whole supporting chain" is [`Tier::weakest`], and a policy
//! ceiling is a simple comparison. The ordering is the type's whole job, so it is derived
//! here rather than written at each call site where it could be got backwards.

use serde::Serialize;

/// A claim tag, ordered strongest to weakest.
///
/// `Verified` < `Inference` < `Open`, so `max` is the weakest of a set and `<=` against a
/// policy ceiling asks "is this claim at least this strong".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Verified,
    Inference,
    Open,
}

impl Tier {
    pub fn parse(s: &str) -> Option<Tier> {
        match s.trim() {
            "verified" => Some(Tier::Verified),
            "inference" => Some(Tier::Inference),
            "open" => Some(Tier::Open),
            _ => None,
        }
    }

    /// The tier a derived assertion inherits from everything beneath it.
    ///
    /// Returns `None` for an empty chain — an assertion resting on nothing. That is not
    /// `Verified`; callers must decide what to do with an unsupported assertion rather than
    /// receive a default that flatters it.
    pub fn weakest(chain: impl IntoIterator<Item = Tier>) -> Option<Tier> {
        chain.into_iter().max()
    }

    /// Whether a claim at this tier may appear in material published at `ceiling`.
    pub fn reaches(self, ceiling: Tier) -> bool {
        self <= ceiling
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Tier::Verified => "verified",
            Tier::Inference => "inference",
            Tier::Open => "open",
        }
    }
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strength_orders_verified_first_and_open_last() {
        assert!(Tier::Verified < Tier::Inference);
        assert!(Tier::Inference < Tier::Open);
    }

    #[test]
    fn the_weakest_link_sets_the_tier() {
        let chain = [Tier::Verified, Tier::Verified, Tier::Inference];
        assert_eq!(Tier::weakest(chain), Some(Tier::Inference));
    }

    #[test]
    fn one_open_claim_beneath_an_assertion_makes_the_assertion_open() {
        // The rule that matters: a chain of otherwise sound work does not launder an
        // unresolved claim into a publishable one.
        let chain = [Tier::Verified, Tier::Open, Tier::Verified];
        assert_eq!(Tier::weakest(chain), Some(Tier::Open));
    }

    #[test]
    fn an_assertion_resting_on_nothing_has_no_tier() {
        assert_eq!(Tier::weakest([]), None);
    }

    #[test]
    fn open_reaches_nothing_a_reader_outside_this_repository_can_see() {
        assert!(!Tier::Open.reaches(Tier::Inference));
        assert!(!Tier::Open.reaches(Tier::Verified));
        assert!(Tier::Open.reaches(Tier::Open));
    }

    #[test]
    fn a_verified_claim_reaches_every_ceiling() {
        for ceiling in [Tier::Verified, Tier::Inference, Tier::Open] {
            assert!(Tier::Verified.reaches(ceiling));
        }
    }

    #[test]
    fn an_unknown_tag_is_not_silently_a_tier() {
        assert_eq!(Tier::parse("verifed"), None);
        assert_eq!(Tier::parse(""), None);
        assert_eq!(Tier::parse("verified"), Some(Tier::Verified));
    }
}
