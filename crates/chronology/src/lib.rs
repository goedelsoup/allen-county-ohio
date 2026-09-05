//! One interval per node, and what this corpus means when it records no end.
//!
//! Twelve classes date themselves in ten different ways — `occurred`/`occurred_through`,
//! `began`/`ended`, `as_of`, `erected`/`abolished`, `built`/`ceased`, `founded`/`dissolved`,
//! `born`/`died`, `effective_from`/`effective_to`, `opened`/`closed`, `established` — and a
//! map that travels through time needs all of them reduced to one shape. That reduction is
//! [`normalize`], and it is pure: reading `.yidam/corpus/` is [`load`]'s job.
//!
//! # The part that is actually hard
//!
//! 135 of the corpus's 484 dated spans record no end, and **the absence does not mean one
//! thing**. This repository already holds two opposite readings of it, each right about its
//! own sources:
//!
//! - `succession` reads a tenure with no `ended` as running to the present, because the last
//!   entry in a sheriff's roster is the sitting sheriff.
//! - `covering` reads a division with no `effective_to` as vouching for nothing past its
//!   start, because Ohio redistricted after 2020 and the corpus does not know the date the
//!   map was superseded.
//!
//! Writing the table out produced a third and a fourth. An **event** with no
//! `occurred_through` is not unbounded at all — that property exists to give a happening
//! duration, so its absence asserts the happening had none. And an **organization**, a
//! **person** or a **site** with no end is neither of the above: the corpus genuinely does
//! not know whether the company dissolved, and says so.
//!
//! So the reading is a property of the class, declared in [`POLICY`] with its argument beside
//! it, rather than a default somebody picked once. See
//! `.yidam/decisions/an-absent-end-date-means-four-things.yml`.
//!
//! # Admitted is not vouched
//!
//! [`Span::admits`] and [`Span::vouched`] are deliberately different questions, and keeping
//! them apart is the whole discipline `covering` was built on: members whose dates admit a
//! year and members the corpus positively supports at that year are returned in **separate
//! lists** and never merged. A division effective from 2020 admits 2024 and vouches for
//! nothing there. A drawing that merges the two invents a fact.

pub mod load;

pub use load::Dated;

use std::collections::BTreeMap;

// ── dates ────────────────────────────────────────────────────────────────────

/// How much of a date the source actually gave.
///
/// The corpus records `YYYY`, `YYYY-MM` or `YYYY-MM-DD` and the ontologies ask for the
/// precision actually known rather than a padded one. Carrying it means a reader can tell the
/// windstorm of 22 June 2006 from a plant that closed sometime in 1979.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precision {
    Year,
    Month,
    Day,
}

impl std::fmt::Display for Precision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Precision::Year => write!(f, "year"),
            Precision::Month => write!(f, "month"),
            Precision::Day => write!(f, "day"),
        }
    }
}

/// A date as the corpus wrote it: a year, and how much more was given.
///
/// Only the year is kept as a number. Every question this crate answers is asked at year
/// grain — the map travels by year, `covering` and `succession` both compare years — and a
/// month held here would be a precision the callers do not use and the sources mostly do not
/// have. The `precision` is kept so that nothing downstream mistakes 1979 for 1 January 1979.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub precision: Precision,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.precision {
            Precision::Year => write!(f, "{}", self.year),
            Precision::Month => write!(f, "{} (to the month)", self.year),
            Precision::Day => write!(f, "{} (to the day)", self.year),
        }
    }
}

/// Read `YYYY`, `YYYY-MM` or `YYYY-MM-DD`.
///
/// Anything else is `None` rather than a guess. A date field this cannot read is reported by
/// [`load`] instead of skipped, for the reason `succession::load` gives about the same class
/// of failure: a node whose dates cannot be read is exactly the node a calculator would
/// otherwise silently omit, and a silent omission here would empty a year on the map.
pub fn parse_date(raw: &str) -> Option<Date> {
    let s = raw.trim();
    let mut parts = s.split('-');
    let year: i32 = parts.next()?.parse().ok()?;
    if !(1000..=9999).contains(&year) {
        return None;
    }
    let precision = match (parts.next(), parts.next(), parts.next()) {
        (None, _, _) => Precision::Year,
        (Some(m), None, _) if two_digits(m) => Precision::Month,
        (Some(m), Some(d), None) if two_digits(m) && two_digits(d) => Precision::Day,
        _ => return None,
    };
    Some(Date { year, precision })
}

fn two_digits(s: &str) -> bool {
    s.len() == 2 && s.bytes().all(|b| b.is_ascii_digit())
}

// ── the policy table ─────────────────────────────────────────────────────────

/// What an absent end date means for a class.
///
/// Four readings, because the corpus has four kinds of silence. Two of them —
/// [`OpenEnd::Unvouched`] and [`OpenEnd::Unknown`] — answer [`Span::admits`] and
/// [`Span::vouched`] identically and are still separate: they differ in what a reader should
/// conclude and in how the span is drawn, and collapsing them would lose the distinction
/// between *the corpus expects this has ended and cannot date it* and *the corpus has no idea
/// either way*.
///
/// Declaration order is strongest claim first, and the derived `Ord` follows it: an instant is
/// the most the corpus can say about an end, an unknown the least.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenEnd {
    /// The end property exists to give the thing duration, so its absence asserts none.
    ///
    /// Neither `succession` nor `covering` has this reading, and without it a one-day fire is
    /// drawn as a fire still burning.
    Instant,
    /// The source's last line is the current state, so the span runs to the present.
    ///
    /// `succession`'s reading. An unabolished jurisdiction still stands; an unclosed question
    /// is open by definition; the last holder in a roster is the sitting holder.
    Running,
    /// A start, and nothing vouched past it.
    ///
    /// `covering`'s reading, and the class it was written for. The corpus's own division nodes
    /// say `effective_to` is absent because the supersession date is unknown — *not* because
    /// the boundary still stands.
    Unvouched,
    /// The corpus does not know whether it ended.
    ///
    /// Not a suspicion that it has, as [`OpenEnd::Unvouched`] is, and not a claim that it has
    /// not, as [`OpenEnd::Running`] is. Drawn as a fading tail rather than a hard edge.
    Unknown,
}

impl std::fmt::Display for OpenEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenEnd::Instant => write!(f, "instantaneous"),
            OpenEnd::Running => write!(f, "runs to the present"),
            OpenEnd::Unvouched => write!(f, "vouches for nothing past the start"),
            OpenEnd::Unknown => write!(f, "unknown"),
        }
    }
}

/// How one class records time, and what its silence means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Policy {
    pub class: &'static str,
    /// The property carrying the start. A node of this class without it is undated.
    pub start: &'static str,
    /// The property carrying the end.
    ///
    /// `None` where the class records a moment and has no end property at all — a measure's
    /// `as_of`, a place's `established`. Nothing can be absent, so there is nothing for
    /// `open_end` to decide and it is [`OpenEnd::Instant`] by construction. `policy_is_coherent`
    /// holds the two fields to that.
    pub end: Option<&'static str>,
    pub open_end: OpenEnd,
    /// Why this class reads its silence the way it does. Printed by `when --table`, so the
    /// argument travels with the rule rather than living only here.
    pub because: &'static str,
}

/// Every class the corpus dates, and what an absent end means for it.
///
/// This is the table, and it is the thing to argue with. It is not a default with exceptions:
/// a class that reaches the feed carrying dates and is not named here fails
/// `tests/corpus.rs`, and so does an entry naming a class that no longer carries any — the
/// second half being the one that survives contact with time, for the reason
/// `design/departures.md` and `.yidam/lint-baseline.yml` both give. A list of exceptions
/// permitted to be wrong drifts, and one that over-lists silently re-permits whatever a later
/// edit puts in its place.
///
/// Ordered as the decision node argues them: the instants, then the three readings of a real
/// absence, weakest claim last.
pub const POLICY: &[Policy] = &[
    Policy {
        class: "measure",
        start: "as_of",
        end: None,
        open_end: OpenEnd::Instant,
        because: "A figure describes a date, not a stretch. The class has no end property, so \
                  there is nothing for an absence to mean.",
    },
    Policy {
        class: "place",
        start: "established",
        end: None,
        open_end: OpenEnd::Running,
        because: "Ground does not stop being ground. The class declares no end because a place \
                  does not have one, so `established` opens a span that has never closed — \
                  Lima was laid out in 1831 and is on the 1900 map for that reason.",
    },
    Policy {
        class: "event",
        start: "occurred",
        end: Some("occurred_through"),
        open_end: OpenEnd::Instant,
        because: "`occurred_through` exists to give a happening duration, so its absence \
                  asserts the happening had none. Reading it as unbounded would draw a \
                  one-day fire as a fire still burning.",
    },
    Policy {
        class: "tenure",
        start: "began",
        end: Some("ended"),
        open_end: OpenEnd::Running,
        because: "The last entry in a roster is the sitting holder. This is `succession`'s \
                  reading and the source it was written against.",
    },
    Policy {
        class: "jurisdiction",
        start: "erected",
        end: Some("abolished"),
        open_end: OpenEnd::Running,
        because: "A unit of government that was never abolished still exists. Every dated \
                  jurisdiction in this corpus is open, and all of them still stand.",
    },
    Policy {
        class: "office",
        start: "established",
        end: Some("abolished"),
        open_end: OpenEnd::Running,
        because: "An office never abolished still stands, and its holders are counted against \
                  it at every year it stood. Same reading as the jurisdiction it sits in, and \
                  for the same reason.",
    },
    Policy {
        class: "question",
        start: "opened",
        end: Some("closed"),
        open_end: OpenEnd::Running,
        because: "An unclosed question is open by definition — the one class where the \
                  absence is the state rather than a gap in the record.",
    },
    Policy {
        class: "period",
        start: "began",
        end: Some("ended"),
        open_end: OpenEnd::Running,
        because: "The class says it: absent `ended` means the corpus treats the span as \
                  ongoing. The one instance is the depopulation, which has not stopped.",
    },
    Policy {
        class: "division",
        start: "effective_from",
        end: Some("effective_to"),
        open_end: OpenEnd::Unvouched,
        because: "`covering`'s reading, and the class it was written for. The corpus's own \
                  district node says `effective_to` is absent because it does not know when \
                  the map was superseded, not because the map still stands.",
    },
    Policy {
        class: "organization",
        start: "founded",
        end: Some("dissolved"),
        open_end: OpenEnd::Unknown,
        because: "A company that stopped trading in 1931 and a company still trading look \
                  identical in this field. The corpus has no record either way.",
    },
    Policy {
        class: "person",
        start: "born",
        end: Some("died"),
        open_end: OpenEnd::Unknown,
        because: "An absent `died` is a death this corpus has not found, not a claim that \
                  somebody born in 1802 is living.",
    },
    Policy {
        class: "site",
        start: "built",
        end: Some("ceased"),
        open_end: OpenEnd::Unknown,
        because: "`ceased` is when the work stopped operating in the role that makes it \
                  notable, which is exactly the fact a source is most likely to omit. \
                  `status` says what is standing; it does not say when that changed.",
    },
];

/// The policy for a class, or `None` if the corpus does not date that class.
pub fn policy_for(class: &str) -> Option<&'static Policy> {
    POLICY.iter().find(|p| p.class == class)
}

// ── the span ─────────────────────────────────────────────────────────────────

/// What the corpus records about one node's own extent in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Span {
    /// A moment. Either the class has no end property, or it has one and the class reads its
    /// absence as asserting no duration.
    Instant(Date),
    /// Both ends recorded.
    Bounded { from: Date, to: Date },
    /// A start, an absent end, and the class's reading of that absence.
    ///
    /// `reading` is never [`OpenEnd::Instant`] — that case is [`Span::Instant`].
    OpenEnded { from: Date, reading: OpenEnd },
}

impl Span {
    /// The first year the corpus places this node in.
    pub fn from_year(&self) -> i32 {
        match self {
            Span::Instant(d) => d.year,
            Span::Bounded { from, .. } | Span::OpenEnded { from, .. } => from.year,
        }
    }

    /// The last year the corpus places it in, where there is one.
    ///
    /// `None` for every open end, including the ones read as running to the present. "Now" is
    /// not a date the corpus recorded, and putting it here would make a build's clock into a
    /// claim about the world — which is the failure `covering`'s module docs warn about in its
    /// first paragraph.
    pub fn to_year(&self) -> Option<i32> {
        match self {
            Span::Instant(d) => Some(d.year),
            Span::Bounded { to, .. } => Some(to.year),
            Span::OpenEnded { .. } => None,
        }
    }

    /// Whether this node could be present in `year`.
    ///
    /// The generous question, and the one that decides whether a mark is a candidate at all.
    pub fn admits(&self, year: i32) -> bool {
        match self {
            Span::Instant(d) => d.year == year,
            Span::Bounded { from, to } => year >= from.year && year <= to.year,
            Span::OpenEnded { from, .. } => year >= from.year,
        }
    }

    /// Whether the corpus positively supports this node being present in `year`.
    ///
    /// The strict question. It differs from [`Span::admits`] only past an open end, and only
    /// where the class's reading declines to carry the span forward: an unvouched or unknown
    /// end supports its start year and nothing after it. Keeping the two apart is what lets a
    /// caller show both without merging them.
    pub fn vouched(&self, year: i32) -> bool {
        match self {
            Span::OpenEnded {
                from,
                reading: OpenEnd::Unvouched | OpenEnd::Unknown,
            } => year == from.year,
            other => other.admits(year),
        }
    }

    /// The weakest precision either end was recorded at.
    pub fn precision(&self) -> Precision {
        match self {
            Span::Instant(d) => d.precision,
            Span::Bounded { from, to } => from.precision.min(to.precision),
            Span::OpenEnded { from, .. } => from.precision,
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Span::Instant(d) => write!(f, "{d}, instantaneous"),
            Span::Bounded { from, to } => write!(f, "{from}–{to}"),
            Span::OpenEnded { from, reading } => write!(f, "{from}–, {reading}"),
        }
    }
}

// ── normalization ────────────────────────────────────────────────────────────

/// Why a node has no span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Undated {
    /// The corpus does not date this class at all — `office`, `natural-feature`.
    ClassNotDated,
    /// The class is dated and this instance omits the start property.
    NoStart { property: &'static str },
    /// A date field that is not a date. Named rather than skipped.
    Unreadable { property: String, value: String },
}

impl std::fmt::Display for Undated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Undated::ClassNotDated => write!(f, "the corpus does not date this class"),
            Undated::NoStart { property } => write!(f, "no `{property}`"),
            Undated::Unreadable { property, value } => {
                write!(f, "`{property}` is not a date: {value:?}")
            }
        }
    }
}

/// Reduce one node's properties to a span, using its class's policy.
///
/// Pure, and the only place the table is applied. An end recorded without a start is an error
/// of the node rather than a shape this returns: the start property is what the class dates
/// itself by, and a span hanging off nothing has no year to be drawn at.
pub fn normalize(class: &str, properties: &BTreeMap<String, String>) -> Result<Span, Undated> {
    let Some(policy) = policy_for(class) else {
        return Err(Undated::ClassNotDated);
    };

    let raw_start = properties
        .get(policy.start)
        .map(String::as_str)
        .filter(|v| !v.trim().is_empty())
        .ok_or(Undated::NoStart {
            property: policy.start,
        })?;
    let from = parse_date(raw_start).ok_or_else(|| Undated::Unreadable {
        property: policy.start.to_string(),
        value: raw_start.to_string(),
    })?;

    // A class with no end property can only ever be an instant, so the lookup is skipped
    // rather than returning `None` from an absent key that means something different.
    let raw_end = policy.end.and_then(|key| {
        properties
            .get(key)
            .map(String::as_str)
            .filter(|v| !v.trim().is_empty())
    });

    match raw_end {
        Some(raw) => {
            let to = parse_date(raw).ok_or_else(|| Undated::Unreadable {
                property: policy.end.unwrap_or_default().to_string(),
                value: raw.to_string(),
            })?;
            Ok(Span::Bounded { from, to })
        }
        None => match policy.open_end {
            OpenEnd::Instant => Ok(Span::Instant(from)),
            reading => Ok(Span::OpenEnded { from, reading }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn props(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn reads_the_three_precisions_the_corpus_writes() {
        assert_eq!(
            parse_date("1885"),
            Some(Date {
                year: 1885,
                precision: Precision::Year
            })
        );
        assert_eq!(
            parse_date("1917-04").map(|d| d.precision),
            Some(Precision::Month)
        );
        assert_eq!(
            parse_date("1933-10-12").map(|d| d.precision),
            Some(Precision::Day)
        );
    }

    #[test]
    fn refuses_a_date_it_cannot_read_rather_than_guessing() {
        for bad in [
            "",
            "c. 1885",
            "1885?",
            "18850",
            "185",
            "1885-4",
            "1885-04-1",
            "not a year",
        ] {
            assert_eq!(parse_date(bad), None, "{bad:?} should not parse");
        }
    }

    #[test]
    fn an_event_without_a_through_date_is_an_instant_not_an_open_end() {
        // The reading neither `succession` nor `covering` has, and the whole reason the table
        // has four entries rather than two.
        let span = normalize("event", &props(&[("occurred", "1933-10-12")])).unwrap();
        assert!(matches!(span, Span::Instant(_)));
        assert!(span.admits(1933));
        assert!(!span.admits(1934), "a one-night raid is not still going on");
    }

    #[test]
    fn an_event_with_a_through_date_is_bounded() {
        let span = normalize(
            "event",
            &props(&[("occurred", "1917-04"), ("occurred_through", "1918-11-11")]),
        )
        .unwrap();
        assert_eq!(span.from_year(), 1917);
        assert_eq!(span.to_year(), Some(1918));
        assert!(span.admits(1918) && !span.admits(1919));
    }

    #[test]
    fn a_sitting_holder_runs_to_the_present_and_is_vouched_there() {
        let span = normalize("tenure", &props(&[("began", "2021")])).unwrap();
        assert!(span.admits(2026));
        assert!(
            span.vouched(2026),
            "succession reads the roster's last line as the sitting holder"
        );
    }

    #[test]
    fn a_division_admits_later_years_and_vouches_for_none_of_them() {
        // The distinction `covering` exists to make. Both lists are real; merging them is the
        // failure.
        let span = normalize("division", &props(&[("effective_from", "2020")])).unwrap();
        assert!(span.admits(2024), "it may well still stand");
        assert!(!span.vouched(2024), "and the corpus does not say so");
        assert!(span.vouched(2020));
    }

    #[test]
    fn an_unknown_end_admits_but_does_not_vouch() {
        let span = normalize("person", &props(&[("born", "1802")])).unwrap();
        assert!(span.admits(1900));
        assert!(!span.vouched(1900));
        assert!(matches!(
            span,
            Span::OpenEnded {
                reading: OpenEnd::Unknown,
                ..
            }
        ));
    }

    #[test]
    fn unvouched_and_unknown_agree_on_the_arithmetic_and_differ_in_what_they_say() {
        let division = normalize("division", &props(&[("effective_from", "1990")])).unwrap();
        let site = normalize("site", &props(&[("built", "1990")])).unwrap();
        for year in [1990, 1991, 2026] {
            assert_eq!(division.admits(year), site.admits(year));
            assert_eq!(division.vouched(year), site.vouched(year));
        }
        assert_ne!(division.to_string(), site.to_string());
    }

    #[test]
    fn a_measure_is_an_instant_and_has_no_end_to_be_absent() {
        let span = normalize("measure", &props(&[("as_of", "2023")])).unwrap();
        assert_eq!(
            span,
            Span::Instant(Date {
                year: 2023,
                precision: Precision::Year
            })
        );
        // An `ended` on a measure is not an end date, and must not become one.
        let noise = normalize("measure", &props(&[("as_of", "2023"), ("ended", "2030")])).unwrap();
        assert_eq!(span, noise);
    }

    #[test]
    fn a_place_opens_a_span_that_never_closes() {
        // The class has no end property, and the reading is still not "instantaneous": Lima
        // was laid out in 1831 and is on the map for every year after it.
        let span = normalize("place", &props(&[("established", "1831")])).unwrap();
        assert!(span.admits(1900) && span.vouched(1900));
        assert_eq!(span.to_year(), None);
    }

    #[test]
    fn an_office_outlives_its_holders_and_is_abolished_only_when_the_corpus_says_so() {
        let standing = normalize("office", &props(&[("established", "1831")])).unwrap();
        assert!(standing.vouched(2026));
        let gone = normalize(
            "office",
            &props(&[("established", "1831"), ("abolished", "1852")]),
        )
        .unwrap();
        assert!(gone.admits(1840) && !gone.admits(1900));
    }

    #[test]
    fn an_undated_class_is_named_as_such_rather_than_erroring_vaguely() {
        assert_eq!(
            normalize("natural-feature", &props(&[("began", "1831")])),
            Err(Undated::ClassNotDated)
        );
    }

    #[test]
    fn a_dated_class_missing_its_start_names_the_property_it_wanted() {
        assert_eq!(
            normalize("tenure", &props(&[("ended", "1893")])),
            Err(Undated::NoStart { property: "began" })
        );
    }

    #[test]
    fn an_unreadable_date_is_reported_with_its_value() {
        let err = normalize("event", &props(&[("occurred", "sometime in the 1880s")])).unwrap_err();
        assert!(matches!(err, Undated::Unreadable { .. }));
        assert!(err.to_string().contains("sometime in the 1880s"));
    }

    #[test]
    fn precision_takes_the_weaker_of_the_two_ends() {
        let span = normalize(
            "period",
            &props(&[("began", "1861-04-14"), ("ended", "1865")]),
        )
        .unwrap();
        assert_eq!(span.precision(), Precision::Year);
    }

    #[test]
    fn no_span_ever_reports_the_present_as_a_recorded_end() {
        // `to_year` is what a caller would reach for to draw a bar. If an open end answered it
        // with a build's clock, the drawing would assert something no source says.
        for class in ["tenure", "division", "person", "jurisdiction"] {
            let policy = policy_for(class).unwrap();
            let span = normalize(class, &props(&[(policy.start, "1900")])).unwrap();
            assert_eq!(span.to_year(), None, "{class}");
        }
    }

    #[test]
    fn policy_is_coherent() {
        let mut seen = std::collections::BTreeSet::new();
        for p in POLICY {
            assert!(seen.insert(p.class), "{} is listed twice", p.class);
            assert!(
                !p.because.trim().is_empty(),
                "{} states no argument for its reading",
                p.class
            );
            // A class with no end property has nothing that can be *absent*, so its reading
            // has to be structural: the thing is a moment, or it is a thing that opens and
            // never closes. `Unvouched` and `Unknown` both say "the end field is empty and we
            // cannot tell", which presupposes an end field. Declaring one against a class that
            // has none would be an argument about a value that cannot exist.
            if p.end.is_none() {
                assert!(
                    matches!(p.open_end, OpenEnd::Instant | OpenEnd::Running),
                    "{} has no end property but reads its absence as {}",
                    p.class,
                    p.open_end
                );
            }
            assert_ne!(p.start, p.end.unwrap_or_default(), "{}", p.class);
        }
    }
}
