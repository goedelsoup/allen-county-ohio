//! The county's ground key: township, range, section — and how it is written down.
//!
//! Allen County writes this code in two places and **not in the same way**, which is the one
//! thing to know before joining anything to it.
//!
//! | where | T3 R7 S17 | T4 R6 S1 |
//! |---|---|---|
//! | GIS layer 55 `T_R_S` | `3717` | `461` |
//! | a parcel number's first four digits | `3717` | `4601` |
//!
//! The layer writes the section as the county writes it — unpadded — and the parcel number
//! pads it to two digits, because a parcel number is fixed-width. Ninety-six of the county's
//! 404 sections have a single-digit number, so a string join between the two forms drops
//! **24 per cent of the county** and reports no error while doing it.
//!
//! This type is the fix: parse from either form, print in either form, and never let a caller
//! compare the strings.

use std::fmt;

/// One section of the rectangular survey, as Allen County numbers them.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Ground {
    /// Township south of the base line. 1–4 in this county.
    pub township: u8,
    /// Range east of the First Principal Meridian. 4–8 in this county.
    pub range: u8,
    /// Section within the township. 1–36.
    pub section: u8,
}

impl Ground {
    pub fn new(township: u8, range: u8, section: u8) -> Option<Self> {
        // A code outside these bounds is not a near miss, it is a misparse: township and range
        // are one digit each by construction, so a two-digit township would have eaten the
        // range. Rejecting rather than clamping is what makes the parse below trustworthy.
        (1..=9)
            .contains(&township)
            .then_some(())
            .and((1..=9).contains(&range).then_some(()))
            .and((1..=36).contains(&section).then_some(()))?;
        Some(Ground {
            township,
            range,
            section,
        })
    }

    /// Parse either form. Length disambiguates: three characters is a single-digit section.
    pub fn parse(code: &str) -> Option<Self> {
        let c = code.trim();
        if !c.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let d = |i: usize| c.as_bytes()[i] - b'0';
        match c.len() {
            3 => Ground::new(d(0), d(1), d(2)),
            4 => Ground::new(d(0), d(1), d(2) * 10 + d(3)),
            _ => None,
        }
    }

    /// As GIS layer 55 writes it — section unpadded. `T4 R6 S1` is `461`.
    pub fn layer_code(&self) -> String {
        format!("{}{}{}", self.township, self.range, self.section)
    }

    /// As a parcel number's first four digits — section padded. `T4 R6 S1` is `4601`.
    pub fn parcel_prefix(&self) -> String {
        format!("{}{}{:02}", self.township, self.range, self.section)
    }
}

impl fmt::Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T{}S R{}E §{}", self.township, self.range, self.section)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_forms_of_a_single_digit_section_are_different_strings() {
        // The whole reason this module exists. 96 of the county's 404 sections are written
        // one way by the GIS layer and another way by every parcel number on that ground.
        let g = Ground::new(4, 6, 1).unwrap();
        assert_eq!(g.layer_code(), "461");
        assert_eq!(g.parcel_prefix(), "4601");
        assert_ne!(g.layer_code(), g.parcel_prefix());
    }

    #[test]
    fn a_two_digit_section_writes_the_same_in_both() {
        let g = Ground::new(3, 7, 17).unwrap();
        assert_eq!(g.layer_code(), "3717");
        assert_eq!(g.parcel_prefix(), "3717");
    }

    #[test]
    fn both_forms_round_trip_to_the_same_ground() {
        let g = Ground::new(4, 6, 1).unwrap();
        assert_eq!(Ground::parse("461"), Some(g));
        assert_eq!(Ground::parse("4601"), Some(g));
    }

    #[test]
    fn a_code_that_cannot_be_a_section_is_rejected_rather_than_clamped() {
        assert_eq!(Ground::parse("4637"), None, "section 37 does not exist");
        assert_eq!(Ground::parse("4600"), None, "sections are 1-based");
        assert_eq!(Ground::parse("46"), None);
        assert_eq!(Ground::parse("46011"), None);
        assert_eq!(Ground::parse("46a1"), None);
    }
}
