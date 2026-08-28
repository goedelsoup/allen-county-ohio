//! Gaps and overlaps in an office's line of holders.
//!
//! The audit itself is pure: it takes terms and a seat count and returns findings, with no
//! network and no filesystem. Reading `.yidam/corpus/` is [`load`]'s job and is kept
//! separate so the interesting logic is testable without a corpus on disk.
//!
//! # Year precision
//!
//! The source this was written against — the Allen County Sheriff's Office roster — gives
//! year ranges and no months. Consecutive terms therefore share a boundary year: O'Neill is
//! `1889–1893` and Fisher is `1893–1898`. Read as closed intervals, every adjacent pair in a
//! 39-term line overlaps, and all 38 of those overlaps are artifacts of the source's
//! precision rather than facts about the office.
//!
//! So a term is modelled as the **half-open** interval `[began, ended)`. Contiguity is the
//! default reading of a shared boundary year, and an overlap is reported only where one term
//! genuinely runs past the next one's start.
//!
//! That is the right model for the audit and the wrong model for a point query, because the
//! transition really did happen at some unrecorded point during the shared year and both
//! holders have a claim on it. [`holders_in`] answers that question separately and says when
//! the answer is ambiguous.

pub mod load;

use std::collections::BTreeMap;

/// One person's holding of one office over one interval, as the audit needs it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    /// Corpus path of the tenure node, so a finding can name the file it came from.
    pub node: String,
    /// Corpus path of the person node.
    pub holder: String,
    pub began: i32,
    /// `None` means the term is open — still serving, or the end is not recorded.
    /// Which of those it is cannot be told from the field and is not guessed here.
    pub ended: Option<i32>,
    pub term_number: Option<u32>,
}

impl Term {
    /// Whether the half-open interval `[began, ended)` contains `year`.
    fn spans(&self, year: i32) -> bool {
        year >= self.began && self.ended.is_none_or(|e| year < e)
    }
}

/// An interval in which the office has no recorded holder.
///
/// Named for what it is: a hole in the *record*. The corpus cannot distinguish an office that
/// stood vacant from one whose holder nobody wrote down, so neither can this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gap {
    pub from: i32,
    pub to: i32,
    pub after: String,
    pub before: String,
}

/// An interval in which more terms are open at once than the office has seats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overlap {
    pub from: i32,
    pub to: i32,
    pub seats: u32,
    pub concurrent: u32,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Audit {
    /// Terms in the order they were held.
    pub line: Vec<Term>,
    pub gaps: Vec<Gap>,
    pub overlaps: Vec<Overlap>,
}

impl Audit {
    pub fn is_clean(&self) -> bool {
        self.gaps.is_empty() && self.overlaps.is_empty()
    }
}

/// Audit one office's line of holders.
///
/// `seats` is how many people may hold the office at once. A three-member board with
/// staggered terms produces concurrent tenures that are entirely correct, and reading `seats`
/// is the only thing that keeps those from being reported as defects.
///
/// An office with no terms returns an empty audit and **no findings**. A line nobody has
/// recorded yet is a state, not a defect, and reporting it as one would fire on every office
/// the moment it was created.
pub fn audit(seats: u32, terms: &[Term]) -> Audit {
    let mut line: Vec<Term> = terms.to_vec();
    // An open term extends furthest, so it sorts last among terms sharing a start year.
    // `Option`'s own ordering puts `None` first, which would put the current holder ahead of
    // a single-year predecessor — Everett 2017-2017 and Treglia 2017-present are exactly that
    // pair, and the corpus test caught it.
    line.sort_by_key(|t| (t.began, t.ended.unwrap_or(i32::MAX), t.node.clone()));

    if line.is_empty() {
        return Audit::default();
    }

    // Sweep the boundaries. Every change in coverage happens at some term's begin or end, so
    // evaluating the half-open segments between consecutive boundaries is exact — no year is
    // skipped and none is counted twice.
    let mut bounds: Vec<i32> = Vec::new();
    for t in &line {
        bounds.push(t.began);
        if let Some(e) = t.ended {
            bounds.push(e);
        }
    }
    bounds.sort_unstable();
    bounds.dedup();

    let mut gaps = Vec::new();
    let mut overlaps = Vec::new();

    for w in bounds.windows(2) {
        let (from, to) = (w[0], w[1]);
        let covering: Vec<&Term> = line.iter().filter(|t| t.spans(from)).collect();
        let n = covering.len() as u32;

        if n == 0 {
            // Only interior holes are gaps. Before the first term is not a gap in the line;
            // it is the line not having started.
            let after = line
                .iter()
                .filter(|t| t.ended.is_some_and(|e| e <= from))
                .max_by_key(|t| t.ended)
                .map(|t| t.node.clone())
                .unwrap_or_default();
            let before = line
                .iter()
                .find(|t| t.began >= to)
                .map(|t| t.node.clone())
                .unwrap_or_default();
            if !after.is_empty() && !before.is_empty() {
                gaps.push(Gap {
                    from,
                    to,
                    after,
                    before,
                });
            }
        } else if n > seats {
            overlaps.push(Overlap {
                from,
                to,
                seats,
                concurrent: n,
                nodes: covering.iter().map(|t| t.node.clone()).collect(),
            });
        }
    }

    Audit {
        line,
        gaps,
        overlaps,
    }
}

/// Who held the office in a given year, and whether the answer is ambiguous.
///
/// This deliberately uses **closed** intervals where [`audit`] uses half-open ones. A roster
/// giving `1889–1893` and `1893–1898` records a transition that happened at an unrecorded
/// point inside 1893, so both holders have a claim on that year and reporting one of them
/// would be inventing a precision the source does not have.
///
/// Returns the matching terms; more than one means the year is a shared boundary.
pub fn holders_in(terms: &[Term], year: i32) -> Vec<&Term> {
    let mut out: Vec<&Term> = terms
        .iter()
        .filter(|t| year >= t.began && t.ended.is_none_or(|e| year <= e))
        .collect();
    out.sort_by_key(|t| t.began);
    out
}

/// Group terms by the office node they point at.
pub fn by_office(terms: Vec<(String, Term)>) -> BTreeMap<String, Vec<Term>> {
    let mut m: BTreeMap<String, Vec<Term>> = BTreeMap::new();
    for (office, t) in terms {
        m.entry(office).or_default().push(t);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(node: &str, began: i32, ended: Option<i32>) -> Term {
        Term {
            node: node.to_string(),
            holder: format!("person/{node}"),
            began,
            ended,
            term_number: None,
        }
    }

    #[test]
    fn a_shared_boundary_year_is_contiguity_not_overlap() {
        // The case the Allen County roster is full of: O'Neill 1889–1893, Fisher 1893–1898.
        // Read as closed intervals this is an overlap; it is not one.
        let a = audit(
            1,
            &[t("oneill", 1889, Some(1893)), t("fisher", 1893, Some(1898))],
        );
        assert!(
            a.overlaps.is_empty(),
            "shared boundary year reported as overlap"
        );
        assert!(a.gaps.is_empty());
        assert!(a.is_clean());
    }

    #[test]
    fn a_long_contiguous_line_is_clean() {
        // 38 adjacent pairs, every one sharing a boundary year — the shape that produces 38
        // spurious findings under closed intervals.
        let mut terms = Vec::new();
        for i in 0..39 {
            let b = 1831 + i * 4;
            terms.push(t(&format!("s{i}"), b, Some(b + 4)));
        }
        let a = audit(1, &terms);
        assert_eq!(a.line.len(), 39);
        assert!(a.is_clean(), "gaps={:?} overlaps={:?}", a.gaps, a.overlaps);
    }

    #[test]
    fn a_real_gap_is_reported_with_its_neighbours() {
        let a = audit(
            1,
            &[t("early", 1900, Some(1905)), t("late", 1907, Some(1910))],
        );
        assert_eq!(a.gaps.len(), 1);
        let g = &a.gaps[0];
        assert_eq!((g.from, g.to), (1905, 1907));
        assert_eq!(g.after, "early");
        assert_eq!(g.before, "late");
        assert!(a.overlaps.is_empty());
    }

    #[test]
    fn a_real_overlap_on_a_single_seat_office_is_reported() {
        let a = audit(1, &[t("a", 1900, Some(1910)), t("b", 1905, Some(1915))]);
        assert_eq!(a.overlaps.len(), 1);
        let o = &a.overlaps[0];
        assert_eq!((o.from, o.to), (1905, 1910));
        assert_eq!(o.concurrent, 2);
        assert_eq!(o.seats, 1);
        assert!(a.gaps.is_empty());
    }

    #[test]
    fn concurrent_terms_within_the_seat_count_are_not_defects() {
        // A three-member board with staggered terms. All three overlap by design.
        let terms = vec![
            t("seat_a", 1900, Some(1906)),
            t("seat_b", 1902, Some(1908)),
            t("seat_c", 1904, Some(1910)),
        ];
        assert!(
            audit(3, &terms).is_clean(),
            "staggered board reported as defective"
        );
        // The same data on a single-seat office is a real finding.
        assert!(!audit(1, &terms).overlaps.is_empty());
    }

    #[test]
    fn a_fourth_concurrent_holder_on_a_three_seat_board_is_reported() {
        let terms = vec![
            t("a", 1900, Some(1910)),
            t("b", 1900, Some(1910)),
            t("c", 1900, Some(1910)),
            t("d", 1902, Some(1904)),
        ];
        let a = audit(3, &terms);
        assert_eq!(a.overlaps.len(), 1);
        assert_eq!(a.overlaps[0].concurrent, 4);
    }

    #[test]
    fn an_office_with_no_tenures_is_not_a_defect() {
        // mayor-of-lima is in exactly this state, and it must not read as a finding.
        let a = audit(1, &[]);
        assert!(a.is_clean());
        assert!(a.line.is_empty());
    }

    #[test]
    fn an_open_term_extends_to_the_present_and_ends_the_line() {
        let a = audit(1, &[t("past", 2009, Some(2017)), t("now", 2017, None)]);
        assert!(a.is_clean(), "gaps={:?} overlaps={:?}", a.gaps, a.overlaps);
        assert!(holders_in(&a.line, 2099).iter().any(|x| x.node == "now"));
    }

    #[test]
    fn time_before_the_first_term_is_not_a_gap() {
        let a = audit(1, &[t("first", 1831, Some(1835))]);
        assert!(
            a.gaps.is_empty(),
            "the line not having started is not a hole in it"
        );
    }

    #[test]
    fn a_boundary_year_has_two_claimants_and_the_query_says_so() {
        let terms = [t("oneill", 1889, Some(1893)), t("fisher", 1893, Some(1898))];
        let both = holders_in(&terms, 1893);
        assert_eq!(both.len(), 2, "the transition year belongs to both holders");
        assert_eq!(holders_in(&terms, 1890).len(), 1);
        assert_eq!(holders_in(&terms, 1895).len(), 1);
    }

    #[test]
    fn non_consecutive_terms_by_one_person_are_two_terms_not_an_overlap() {
        // John Keller, 1835–1839 and 1843–1845, with Beatty between.
        let terms = vec![
            t("keller_1", 1835, Some(1839)),
            t("beatty", 1839, Some(1843)),
            t("keller_2", 1843, Some(1845)),
        ];
        let a = audit(1, &terms);
        assert!(a.is_clean());
        assert_eq!(a.line.len(), 3);
    }
}
