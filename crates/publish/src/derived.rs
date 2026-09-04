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
        statement: "Allen County peaked in 1980, not 1970. It has fallen at every count since, \
                    from 112,241 to 100,866.",
        topic: "population",
        // This assertion used to read "fallen at every measured point since 1970" and cite a
        // span saying "no reversal at any measured point". Both were true of the data the
        // corpus held and false of the county: every figure began at 1970, and the one decade
        // that rose was the one decade outside the window. The gate caught the change when the
        // corpus corrected itself, which is the whole reason the spans are quoted.
        supports: &[
            support!(
                "measure/allen-county-population-1940-1990.yml",
                "**The county's peak is 1980, not 1970, and the corpus has been saying 1970 since genesis.**"
            ),
            support!(
                "period/depopulation.yml",
                "Between 1970 and 1980 the county *grew*, from 111,144 to **112,241** — its highest count ever — before falling to 109,755 in 1990 and 108,464 in 2000"
            ),
            support!(
                "period/depopulation.yml",
                "a loss of 10,278 people, or 9.2%, across five decades: 111,144 (1970), 108,464 (2000), 106,331 (2010), 102,217 (2020), 100,866 (2024)"
            ),
        ],
        answers: &[
            // Arrived by propagation when the corpus gained life expectancy by tract: the
            // period node now refuses the reading that Lima's mortality is the county's loss.
            "It does not follow that Lima's mortality is what makes the county smaller",
            "It does not establish that the county's population decline was caused",
            "It does not establish that either mechanism reaches back before 2020",
            "It does not establish that the decline has ended",
            // Added when `depopulation` gained the age measure. The gate demanded it of all three
            // assertions resting on that node, which is the check working: a node grew a refusal
            // and nothing built on it could go out without carrying it.
            "does not establish that Lima is emptying because it is aging",
        ],
        // The chart is the corpus's own enumerations, now spanning the peak the earlier version
        // of this assertion could not see.
        figures: &[
            Figure { label: "1970", value: 111_144.0, literal: "111,144" },
            Figure { label: "1980", value: 112_241.0, literal: "112,241" },
            Figure { label: "1990", value: 109_755.0, literal: "109,755" },
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
            "period/depopulation.yml",
            "between 2000 and 2010 Lima fell 3.8% while the balance of Allen County — everything outside every incorporated place — fell 1.5%, from 50,809 to 50,048"
        )],
        answers: &[
            // Arrived by propagation when the corpus gained life expectancy by tract: the
            // period node now refuses the reading that Lima's mortality is the county's loss.
            "It does not follow that Lima's mortality is what makes the county smaller",
            "It does not establish that the county's population decline was caused",
            "It does not establish that either mechanism reaches back before 2020",
            "It does not establish that the decline has ended",
            // Added when `depopulation` gained the age measure. The gate demanded it of all three
            // assertions resting on that node, which is the check working: a node grew a refusal
            // and nothing built on it could go out without carrying it.
            "does not establish that Lima is emptying because it is aging",
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
            "period/depopulation.yml",
            "eleven of the county's thirteen civil subdivisions lost population"
        )],
        answers: &[
            // Arrived by propagation when the corpus gained life expectancy by tract: the
            // period node now refuses the reading that Lima's mortality is the county's loss.
            "It does not follow that Lima's mortality is what makes the county smaller",
            "It does not establish that the county's population decline was caused",
            "It does not establish that either mechanism reaches back before 2020",
            "It does not establish that the decline has ended",
            // Added when `depopulation` gained the age measure. The gate demanded it of all three
            // assertions resting on that node, which is the check working: a node grew a refusal
            // and nothing built on it could go out without carrying it.
            "does not establish that Lima is emptying because it is aging",
        ],
        figures: &[],
    },
    Assertion {
        id: "lima-grew-fivefold-after-the-oil",
        statement: "The oil made the city: Lima went from 7,567 people in 1880 to 41,326 in \
                    1920, and on to its peak of 53,734 in 1970.",
        topic: "population",
        // Cited to the federal series rather than to the three county histories this assertion
        // used to rest on. Two of those three figures the Census confirms to the person and one
        // it contradicts — 41,306 for 1920, against 41,326 — so the histories are now the
        // corroboration and the Census is the citation.
        supports: &[
            support!(
                "measure/lima-population-1850-1960.yml",
                "**Lima held 7,567 people in 1880, 21,723 in 1900, 41,326 in 1920, 42,287 in 1930, 44,711 in 1940, 50,246 in 1950 and 51,037 in 1960.**"
            ),
            support!(
                "measure/lima-population-1850-1960.yml",
                "**Lima peaked at the 1970 census.**"
            ),
            support!(
                "measure/lima-population-1970-1990.yml",
                "**53,734 in 1970, 47,827 in 1980 and 45,549 in 1990.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1880", value: 7_567.0, literal: "7,567" },
            Figure { label: "1900", value: 21_723.0, literal: "21,723" },
            Figure { label: "1920", value: 41_326.0, literal: "41,326" },
            Figure { label: "1930", value: 42_287.0, literal: "42,287" },
            Figure { label: "1940", value: 44_711.0, literal: "44,711" },
            Figure { label: "1950", value: 50_246.0, literal: "50,246" },
            Figure { label: "1960", value: 51_037.0, literal: "51,037" },
            Figure { label: "1970", value: 53_734.0, literal: "53,734" },
        ],
    },
    Assertion {
        id: "the-county-has-a-complete-series",
        statement: "Allen County has a population count at every decennial census from 1830 to \
                    2020: 578 at the first and 102,217 at the last, with the peak in 1980.",
        topic: "population",
        // Twenty-one figures, which is more than any other chart on this site plots, and the
        // reason to plot them is that a reader who wants to know how big this county has ever
        // been should not have to assemble the answer from four nodes.
        supports: &[support!(
            "place/allen-county.yml",
            "**578 (1830), 9,079 (1840), 12,100 (1850), 19,185 (1860), 23,623 (1870), 31,314 (1880), 40,644 (1890), 47,976 (1900), 56,580 (1910), 68,223 (1920), 69,419 (1930), 73,303 (1940), 88,183 (1950), 103,691 (1960), 111,144 (1970), 112,241 (1980), 109,755 (1990), 108,464 (2000), 106,331 (2010), 102,217 (2020) and 100,866 (2024).**"
        )],
        answers: &[],
        figures: &[
            Figure { label: "1830", value: 578.0, literal: "578" },
            Figure { label: "1840", value: 9_079.0, literal: "9,079" },
            Figure { label: "1850", value: 12_100.0, literal: "12,100" },
            Figure { label: "1860", value: 19_185.0, literal: "19,185" },
            Figure { label: "1870", value: 23_623.0, literal: "23,623" },
            Figure { label: "1880", value: 31_314.0, literal: "31,314" },
            Figure { label: "1890", value: 40_644.0, literal: "40,644" },
            Figure { label: "1900", value: 47_976.0, literal: "47,976" },
            Figure { label: "1910", value: 56_580.0, literal: "56,580" },
            Figure { label: "1920", value: 68_223.0, literal: "68,223" },
            Figure { label: "1930", value: 69_419.0, literal: "69,419" },
            Figure { label: "1940", value: 73_303.0, literal: "73,303" },
            Figure { label: "1950", value: 88_183.0, literal: "88,183" },
            Figure { label: "1960", value: 103_691.0, literal: "103,691" },
            Figure { label: "1970", value: 111_144.0, literal: "111,144" },
            Figure { label: "1980", value: 112_241.0, literal: "112,241" },
            Figure { label: "1990", value: 109_755.0, literal: "109,755" },
            Figure { label: "2000", value: 108_464.0, literal: "108,464" },
            Figure { label: "2010", value: 106_331.0, literal: "106,331" },
            Figure { label: "2020", value: 102_217.0, literal: "102,217" },
        ],
    },
    Assertion {
        id: "three-peaks-thirty-years-apart",
        statement: "The city peaked in 1970, the county in 1980, and the county outside the city \
                    in 2000.",
        topic: "population",
        // The one assertion on this page whose subject is arithmetic the corpus performed rather
        // than a figure it was given, which is why it cites the node that carries the method.
        supports: &[
            support!(
                "measure/allen-county-outside-lima-1890-2020.yml",
                "**So the county has three peaks and they are thirty years apart.** Lima peaked in 1970, the county as a whole in 1980, and the county outside Lima in 2000."
            ),
            support!(
                "measure/allen-county-outside-lima-1890-2020.yml",
                "**the county outside Lima held 24,663 people in 1890, 26,253 in 1900, 26,072 in 1910, 26,897 in 1920, 27,132 in 1930, 28,592 in 1940, 37,937 in 1950, 52,654 in 1960, 57,410 in 1970, 64,414 in 1980, 64,206 in 1990, 68,157 in 2000, 67,560 in 2010 and 66,627 in 2020.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1890", value: 24_663.0, literal: "24,663" },
            Figure { label: "1900", value: 26_253.0, literal: "26,253" },
            Figure { label: "1910", value: 26_072.0, literal: "26,072" },
            Figure { label: "1920", value: 26_897.0, literal: "26,897" },
            Figure { label: "1930", value: 27_132.0, literal: "27,132" },
            Figure { label: "1940", value: 28_592.0, literal: "28,592" },
            Figure { label: "1950", value: 37_937.0, literal: "37,937" },
            Figure { label: "1960", value: 52_654.0, literal: "52,654" },
            Figure { label: "1970", value: 57_410.0, literal: "57,410" },
            Figure { label: "1980", value: 64_414.0, literal: "64,414" },
            Figure { label: "1990", value: 64_206.0, literal: "64,206" },
            Figure { label: "2000", value: 68_157.0, literal: "68,157" },
            Figure { label: "2010", value: 67_560.0, literal: "67,560" },
            Figure { label: "2020", value: 66_627.0, literal: "66,627" },
        ],
    },
    Assertion {
        id: "every-township-grew",
        statement: "Every one of the county's twelve townships is larger now than it was in \
                    1930 — and larger than in 1890 too, which is the harder test, because 1930 \
                    was near the bottom.",
        topic: "population",
        // No figures. Twelve ratios on one axis would be a chart of arithmetic rather than of
        // the county, and the numbers behind them are in the node's own table where a reader
        // who wants them can see what they were computed from.
        supports: &[
            support!(
                "measure/allen-county-townships-1930-1950.yml",
                "**Every one of the twelve is larger now than it was in 1930, and the comparison is sound.**"
            ),
            support!(
                "measure/allen-county-townships-1930-1950.yml",
                "The whole balance is two and a half times its 1930 size while the county is below its 1980 peak and the city has lost a third of itself."
            ),
            support!(
                "measure/allen-county-townships-1930-1950.yml",
                "**But 1930 was near the bottom, and the second column is why that matters.**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-only-employment-figures",
        statement: "Lima had 3,607 manufacturing wage earners in 1909, up from 1,980 in 1899 — \
                    the earliest measurement of work this corpus holds, and sixty years earlier \
                    than the series that records the decline.",
        topic: "population",
        // Three points, sixty years before the period they are shown beside. The chart is small
        // and the sentence beneath it is the point: this is a baseline, not a mechanism. Three
        // more censuses were added later; see `the-workforce-stopped-growing-in-1914`.
        supports: &[
            support!(
                "measure/lima-manufactures-1899-1909.yml",
                "**Lima had 85 manufacturing establishments in 1909 employing 3,607 wage earners on average, against 2,733 in 1904 and 1,980 in 1899.**"
            ),
            support!(
                "measure/lima-manufactures-1899-1909.yml",
                "**The city's workforce grew 82 per cent in a decade and its output grew 25.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1899", value: 1_980.0, literal: "1,980" },
            Figure { label: "1904", value: 2_733.0, literal: "2,733" },
            Figure { label: "1909", value: 3_607.0, literal: "3,607" },
        ],
    },
    Assertion {
        id: "the-farms-did-not-go-anywhere",
        statement: "Twenty-five years after the oil strike, Allen County had more farms than \
                    before it — 2,939 against 2,858 — on 92.5 per cent of its land area.",
        topic: "population",
        // Two bars that are almost the same height, which is the whole finding. The corpus
        // described this county as reoriented away from farming for nine phases before it
        // measured a farm.
        supports: &[
            support!(
                "measure/allen-county-farms-1900-1910.yml",
                "**The county had 2,939 farms in 1910 and 2,858 in 1900, on 240,472 acres against 245,283.**"
            ),
            support!(
                "measure/allen-county-farms-1900-1910.yml",
                "**Nine-tenths of Allen County was farmland in 1910 — 240,472 acres of 259,840, or 92.5 per cent.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1900", value: 245_283.0, literal: "245,283" },
            Figure { label: "1910", value: 240_472.0, literal: "240,472" },
        ],
    },
    Assertion {
        id: "nine-offices-eleven-officers",
        statement: "Nine separately elected offices govern Allen County and eleven people hold \
                    them, because the board of commissioners seats three.",
        topic: "government",
        // No chart. The finding is a set and its size, and a bar of nine against a bar of
        // eleven would be decoration.
        supports: &[
            support!(
                "jurisdiction/allen-county-government.yml",
                "**Nine separately elected offices govern Allen County, and eleven people hold them, because the board of commissioners seats three.**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-schools-empty-faster-than-the-county",
        statement: "Enrolment in the twelve school districts holding ground in Allen County fell \
                    from 17,985 to 16,113 between 2009 and 2022 — 10.4 per cent, where the county's \
                    population fell 3.9 per cent to the 2020 census.",
        topic: "schools",
        supports: &[
            support!(
                "measure/allen-county-school-enrolment-2010-2023.yml",
                "**The series, annually: 17,985, 17,830, 17,766, 17,565, 17,634, 17,656, 17,518, 17,333, 17,107, 16,921, 16,774, 16,124, 16,127 and 16,113.**"
            ),
            support!(
                "measure/allen-county-school-enrolment-2010-2023.yml",
                "The twelve lost 1,872 pupils, 10.4 per cent, over a span in which Allen County's population fell 3.9 per cent to the 2020 census and 5.1 per cent to the 2024 estimate."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2009", value: 17_985.0, literal: "17,985" },
            Figure { label: "2010", value: 17_830.0, literal: "17,830" },
            Figure { label: "2011", value: 17_766.0, literal: "17,766" },
            Figure { label: "2012", value: 17_565.0, literal: "17,565" },
            Figure { label: "2013", value: 17_634.0, literal: "17,634" },
            Figure { label: "2014", value: 17_656.0, literal: "17,656" },
            Figure { label: "2015", value: 17_518.0, literal: "17,518" },
            Figure { label: "2016", value: 17_333.0, literal: "17,333" },
            Figure { label: "2017", value: 17_107.0, literal: "17,107" },
            Figure { label: "2018", value: 16_921.0, literal: "16,921" },
            Figure { label: "2019", value: 16_774.0, literal: "16,774" },
            Figure { label: "2020", value: 16_124.0, literal: "16,124" },
            Figure { label: "2021", value: 16_127.0, literal: "16,127" },
            Figure { label: "2022", value: 16_113.0, literal: "16,113" },
        ],
    },
    Assertion {
        id: "lima-schools-spend-most-and-raise-least",
        statement: "Lima City Schools spends more per pupil than any district in Allen County and \
                    raises the smallest share of it locally — 18.4 per cent, against 59.2 from the \
                    state and 22.4 from the federal government.",
        topic: "schools",
        supports: &[
            support!(
                "measure/allen-county-school-finance-2023.yml",
                "Lima City is the county's largest district and its highest spender at $19,321, and 18.4 per cent of its revenue is local where every other district here is between 37.0 and 65.8: 59.2 per cent comes from the state and 22.4 from the federal government."
            ),
            support!(
                "measure/allen-county-school-finance-2023.yml",
                "Lima raises $3,098 a pupil in property tax where Shawnee raises $7,727 — a ratio of 2.49. The median house in Lima is worth $95,900 and in Shawnee Township $210,200, a ratio of 2.19."
            ),
        ],
        answers: &["does not establish that any district is spending well or badly"],
        figures: &[
            Figure { label: "Local", value: 18.4, literal: "18.4" },
            Figure { label: "State", value: 59.2, literal: "59.2" },
            Figure { label: "Federal", value: 22.4, literal: "22.4" },
        ],
    },
    Assertion {
        id: "the-only-district-published-as-a-number",
        statement: "Eleven of the 120 district-years of graduation data for Allen County carry an exact rate; ten of them are Lima City, the district with the worst rate. Every other district is published as a band, and the width of the band is set by the size of the cohort.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-graduation-rates-2010-2019.yml", "**One district in twelve is published as a number and it is the district with the worst rate.** 11 of this county's 120 district-years carry an exact graduation rate and 109 are published as a band; ten of the 11 are Lima and the eleventh is Shawnee in 2017."),
        ],
        answers: &["cannot say what any other district's graduation rate for poor children is"],
        figures: &[
            Figure { label: "exact", value: 11.0, literal: "11" },
            Figure { label: "banded", value: 109.0, literal: "109" },
        ],
    },
    Assertion {
        id: "limas-graduation-rate-has-not-moved",
        statement: "Lima City's four-year graduation rate was 70 per cent in 2010 and 63 in 2019, and nine of the ten years sit between 63 and 70. It is the only rate in the county published as a number every year.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-graduation-rates-2010-2019.yml", "**Lima's rate is the one figure in the table that can be tracked, and it has not moved much.** 70 per cent in 2010, 78 in 2011, then 70, 66, 63, 69, 63, 65, 67 and 63."),
        ],
        answers: &["cannot say what any other district's graduation rate for poor children is"],
        figures: &[
            Figure { label: "2010", value: 70.0, literal: "70" },
            Figure { label: "2011", value: 78.0, literal: "78" },
            Figure { label: "2013", value: 66.0, literal: "66" },
            Figure { label: "2015", value: 69.0, literal: "69" },
            Figure { label: "2017", value: 65.0, literal: "65" },
            Figure { label: "2019", value: 63.0, literal: "63" },
        ],
    },
    Assertion {
        id: "the-county-graduation-rate-fell-and-the-states-rose",
        statement: "Allen County's graduation rate can only be bounded, and the bounds are enough: 87.0 to 90.9 per cent in 2011 and 82.5 to 86.4 in 2019, two intervals that do not touch. The county stood clear of Ohio every year to 2015 and has overlapped it since 2016.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-graduation-rates-2010-2019.yml", "Weighting each district's band by its cohort gives 87.0 to 90.9 per cent in 2011 and 82.5 to 86.4 in 2019 \u{2014} two intervals that do not touch, so the rate fell by at least 0.6 points and the fall does not depend on any assumption about where inside a band a district sits."),
            support!("measure/allen-county-graduation-rates-2010-2019.yml", "**The county was clear of the state for six years and has not been since.** Its lower bound stood above Ohio's upper bound in every year from 2010 to 2015; from 2016 the two intervals overlap, and in 2019 Ohio's band sits higher than the county's within the overlap."),
        ],
        answers: &["cannot say what any other district's graduation rate for poor children is"],
        figures: &[
            Figure { label: "2011 lower", value: 87.0, literal: "87.0" },
            Figure { label: "2011 upper", value: 90.9, literal: "90.9" },
            Figure { label: "2019 lower", value: 82.5, literal: "82.5" },
            Figure { label: "2019 upper", value: 86.4, literal: "86.4" },
        ],
    },
    Assertion {
        id: "no-racial-gap-in-limas-graduation-rate",
        statement: "In Lima City, the only district here whose cohorts are large enough to publish by race, the white and Black graduation bands are disjoint in seven years of ten \u{2014} and the higher one is white in three and Black in four.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-graduation-rates-2010-2019.yml", "**In the county's only legible district the graduation rate does not differ by race.** Lima's white and Black bands are disjoint in seven of the ten years, and the higher one is white in three of those and Black in four; in the other three years the two bands are identical."),
        ],
        answers: &["cannot say what any other district's graduation rate for poor children is"],
        figures: &[],
    },
    Assertion {
        id: "lima-reads-at-forty-two-per-cent",
        statement: "Lima City reads 42 per cent proficient in 2018 against 62 in Perry and 83 in Bluffton, and 42 per cent in mathematics against 57 and 87. The figures rest on between 250 and 1,904 tests apiece.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**The largest district in the county is twenty points below the second-lowest and forty-one below the highest.** Lima reads 42 per cent proficient in reading in 2018 against Perry's 62 and Bluffton's 83, and 42 in mathematics against Perry's 57 and Bluffton's 87."),
        ],
        answers: &["cannot say whether that gap widened"],
        figures: &[
            Figure { label: "Lima", value: 42.0, literal: "42" },
            Figure { label: "Perry", value: 62.0, literal: "62" },
            Figure { label: "Bluffton", value: 83.0, literal: "83" },
        ],
    },
    Assertion {
        id: "twelve-districts-fell-in-the-same-two-years",
        statement: "Reading proficiency fell in every one of Allen County's twelve districts between 2013 and 2015 \u{2014} by 18 points in Bluffton and 40 in Perry \u{2014} and the state fell 28.7. Twelve independently governed districts do not move together in two years unless the test does.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "Reading proficiency between 2013 and 2015 fell 18 points in Bluffton, 19 in Spencerville, 22 in Shawnee, and on up to 40 in Perry \u{2014} every district in the county, no exceptions, and the state fell 28.7 points over the same two years."),
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**Measured against the state, this county now does better than it used to.** Its reading proficiency was 0.8 points below Ohio's in 2009 and 4.5 points above it in 2018; in mathematics it was 0.3 above and is 7.2 above."),
        ],
        answers: &["cannot say whether that gap widened"],
        figures: &[
            Figure { label: "Bluffton", value: 18.0, literal: "18" },
            Figure { label: "Spencerville", value: 19.0, literal: "19" },
            Figure { label: "Shawnee", value: 22.0, literal: "22" },
            Figure { label: "Perry", value: 40.0, literal: "40" },
        ],
    },
    Assertion {
        id: "the-county-passed-the-state-without-improving",
        statement: "Allen County's reading proficiency was 0.8 points below Ohio's in 2009 and 4.5 above it in 2018; in mathematics, 0.3 above and then 7.2. The county did not improve \u{2014} the state fell further.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**Measured against the state, this county now does better than it used to.** Its reading proficiency was 0.8 points below Ohio's in 2009 and 4.5 points above it in 2018; in mathematics it was 0.3 above and is 7.2 above."),
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**The eleven districts other than Lima are 34 points above it and were 22 points above it.** The gap in reading was 22.3 points in 2009 and 33.7 in 2018, and in mathematics 25.2 and 34.6."),
        ],
        answers: &["cannot say whether that gap widened"],
        figures: &[
            Figure { label: "reading, 2018", value: 4.5, literal: "4.5" },
            Figure { label: "mathematics, 2018", value: 7.2, literal: "7.2" },
        ],
    },
    Assertion {
        id: "half-the-countys-poor-children-are-in-one-district",
        statement: "Lima City Schools holds a quarter of the twelve districts' school-age children \
                    and very nearly half of the poor ones — 1,626 of 3,368 — on a child poverty \
                    rate of 31.7 per cent against Bluffton's 5.6.",
        topic: "schools",
        supports: &[
            support!(
                "measure/allen-county-school-district-poverty-2023.yml",
                "**Lima City Schools is 31.7 per cent child poverty and Bluffton is 5.6.** Lima holds a quarter of the twelve districts' school-age children and very nearly half of their poor ones — 1,626 of 3,368."
            ),
        ],
        answers: &["does not establish a rate that can be ranked against another district"],
        figures: &[],
    },
    Assertion {
        id: "the-ring-was-built-while-the-city-was-full",
        statement: "Allen County's biggest building decade was the 1970s — 8,298 homes — and 6,333 \
                    of them went up outside Lima, in the decade the city reached its own peak.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-housing-age-2023.yml",
                "**The county's biggest building decade was the 1970s**, 8,298 units ± 585 against 6,529 ± 563 for the 1950s."
            ),
            support!(
                "measure/allen-county-housing-age-2023.yml",
                "**Six thousand three hundred and thirty-three of those 8,298 were built outside Lima.** The city took 1,965 of the decade in which it reached its own peak of 53,734 and held 48.4 per cent of the county."
            ),
        ],
        answers: &[
            "does not establish that they are older",
            "This does not establish why the housing was built where it was.",
        ],
        figures: &[],
    },
    Assertion {
        id: "lima-rents-and-the-townships-own",
        statement: "Allen County is 68.3 per cent owner-occupied, which is an ordinary American \
                    figure made of Lima at 46.4 per cent and twelve townships between 72.0 and 91.0.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-housing-stock-2023.yml",
                "Allen County is 68.3 per cent owner-occupied against 65.0 nationally — an unremarkable figure made of Lima at 46.4 per cent and twelve townships between 72.0 and 91.0, the city being the only subdivision below 70."
            ),
        ],
        answers: &["does not establish that it is the county's emptiest place"],
        figures: &[
            Figure { label: "United States", value: 65.0, literal: "65.0" },
            Figure { label: "Allen County", value: 68.3, literal: "68.3" },
            Figure { label: "Lima", value: 46.4, literal: "46.4" },
        ],
    },
    Assertion {
        id: "borrowing-against-a-house-in-lima",
        statement: "Across 2018–2024, mortgages to buy a house in Lima's census tracts were denied \
                    at 13.3 per cent against 10.8 elsewhere in the county. Loans against a house \
                    already owned there were denied at 37.7 per cent against 21.5.",
        topic: "housing",
        // This replaces an assertion withdrawn on 30 August 2026. It read "a mortgage to buy a
        // house in Lima is denied at the rate it is denied anywhere in Allen County", which was
        // read off 2023 — the one year of seven in which Lima's purchase rate is the lower one.
        // See `.yidam/decisions/a-rule-is-not-a-habit.yml`.
        supports: &[
            support!(
                "measure/allen-county-home-lending-2018-2024.yml",
                "Over 2018–2024 the purchase denial rate in Lima's tracts is 13.3 per cent against 10.8 elsewhere — 363 denials of 2,737 against 645 of 5,996 — and Lima's rate is the higher one in six of the seven years."
            ),
            support!(
                "measure/allen-county-home-lending-2018-2024.yml",
                "On everything but purchase, Lima's rate is 37.7 per cent against 21.5 — 1,104 denials of 2,926 against 2,000 of 9,289."
            ),
        ],
        answers: &["does not establish why either gap is there"],
        figures: &[
            Figure { label: "To buy, in Lima", value: 13.3, literal: "13.3" },
            Figure { label: "To buy, elsewhere", value: 10.8, literal: "10.8" },
            Figure { label: "Every other purpose, in Lima", value: 37.7, literal: "37.7" },
            Figure { label: "Every other purpose, elsewhere", value: 21.5, literal: "21.5" },
        ],
    },
    Assertion {
        id: "the-denial-gap-is-seven-years-old",
        statement: "Across 2018–2024, mortgage applications from Black applicants in Allen County \
                    were denied at 32.4 per cent and from white applicants at 17.8, and the gap \
                    appears in all seven years.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-lending-denial-gap-2018-2024.yml",
                "**It is a pattern and not a year.** Pooled over the seven, 32.4 per cent of applications from Black applicants were denied against 17.8 per cent from white applicants, and the yearly gap runs between 5.9 and 17.0 percentage points without once reversing."
            ),
        ],
        answers: &["does not establish that any applicant was treated differently"],
        figures: &[
            Figure { label: "Black applicants", value: 32.4, literal: "32.4" },
            Figure { label: "White applicants", value: 17.8, literal: "17.8" },
        ],
    },
    Assertion {
        id: "the-lending-gap-is-not-composition",
        statement: "Sorted into cells of loan purpose, income, debt-to-income and loan-to-value, \
                    955 applications from Black applicants in Allen County drew 223 denials where \
                    132.5 were expected at the white rate in the same cells.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-lending-denial-gap-2018-2024.yml",
                "Those 955 drew **223 denials against 132.5 expected at the white rate in the same cells**, a ratio of 1.68 and 9.3 standard deviations from the null."
            ),
        ],
        answers: &["does not establish that any applicant was treated differently"],
        figures: &[
            Figure { label: "Denials observed", value: 223.0, literal: "223" },
            Figure { label: "Expected at white rates", value: 132.5, literal: "132.5" },
        ],
    },
    Assertion {
        id: "the-denial-gap-that-does-not-resolve",
        statement: "Mortgage applications from Black applicants in Allen County were denied at 37.9 \
                    per cent and from white applicants at 22.6, and the gap holds inside every \
                    income band this corpus can cut.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-home-lending-2023.yml",
                "**Applications from Black applicants were denied at 37.9 per cent and from white applicants at 22.6** — 66 of 174 against 458 of 2,023 — and the gap holds inside every income band the corpus cut. Under $50,000: 45.3 against 33.4. From $50,000 to $99,000: 38.0 against 20.5. At $100,000 and over: 25.6 against 15.8."
            ),
        ],
        answers: &["does not establish that those applicants were treated differently"],
        figures: &[
            Figure { label: "Under $50k, Black", value: 45.3, literal: "45.3" },
            Figure { label: "Under $50k, white", value: 33.4, literal: "33.4" },
            Figure { label: "$50–99k, Black", value: 38.0, literal: "38.0" },
            Figure { label: "$50–99k, white", value: 20.5, literal: "20.5" },
            Figure { label: "$100k+, Black", value: 25.6, literal: "25.6" },
            Figure { label: "$100k+, white", value: 15.8, literal: "15.8" },
        ],
    },
    Assertion {
        id: "appointment-is-a-form-of-local-government",
        statement: "Thirty-nine of the 166 people holding local elective office in Allen County \
                    were appointed to the seat rather than elected to it. None of the county's \
                    twenty-two officers and judges was.",
        topic: "government",
        // No chart. Five bars of small integers would flatter the finding into a trend; what
        // matters is one ratio against another, and the sentence carries both.
        supports: &[
            support!(
                "measure/allen-county-elected-seats-2026.yml",
                "Of the 166 seated township, village, city and school officers, 39 hold seats they were appointed to fill."
            ),
            support!(
                "measure/allen-county-elected-seats-2026.yml",
                "Every one of the twenty-two county offices and judgeships is filled, and every one of them by election."
            ),
        ],
        answers: &["does not establish that these seats go unfilled"],
        figures: &[],
    },
    Assertion {
        id: "two-villages-cannot-fill-their-governments",
        statement: "Cairo's six-member council is five appointees and a vacancy, and Harrod has \
                    ten elective seats of which one is held by someone its voters chose.",
        topic: "government",
        supports: &[
            support!(
                "jurisdiction/village-of-cairo.yml",
                "**No elected member sits on this village's council.**"
            ),
            support!(
                "jurisdiction/village-of-harrod.yml",
                "**One person in this village's government of ten seats was put there by its voters.**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-townships-are-named",
        statement: "Every township in Allen County has three trustees and a fiscal officer, and \
                    this corpus now names all forty-eight of them.",
        topic: "government",
        supports: &[
            support!(
                "jurisdiction/allen-county-government.yml",
                "the forty-eight trustees and fiscal officers of the twelve townships arrived when the roster was read across all eighty-eight of its precinct pages rather than one."
            ),
            support!(
                "jurisdiction/shawnee-township.yml",
                "Every township in this county has three trustees and one fiscal officer, each on a four-year term"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-factories-stopped-leaving",
        statement: "Allen County manufacturing employment peaked in 1973, at 18,400 — three years \
                    after Lima began to empty — and bottomed at 7,775 in 2009.",
        topic: "population",
        // The chart used to start in 1986, which is the one year in this series most likely to
        // mislead: a recovery high partway along, read as a stage of the fall. It now starts at
        // the peak, because the peak is the finding.
        supports: &[
            support!(
                "measure/allen-county-manufacturing-employment-1969-2022.yml",
                "**Manufacturing employment in Allen County peaked in 1973, at 18,400.**"
            ),
            support!(
                "measure/allen-county-manufacturing-employment-1969-2022.yml",
                "**The trough is 7,775, in 2009 — a fall of 58 per cent from the peak, across thirty-six years.** [verified] — same source. Employment has risen since, to 8,715 in 2022, and has not been below 7,775 in any year on this series."
            ),
            support!(
                "measure/allen-county-manufacturing-employment-1969-2022.yml",
                "**1986 was a recovery year, not a stage of the fall.** Manufacturing bottomed at 14,349 in 1982, rose four years running to 17,163 in 1986, and only then began the long decline."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1973", value: 18_400.0, literal: "18,400" },
            Figure { label: "1982", value: 14_349.0, literal: "14,349" },
            Figure { label: "1986", value: 17_163.0, literal: "17,163" },
            Figure { label: "2009", value: 7_775.0, literal: "7,775" },
            Figure { label: "2022", value: 8_715.0, literal: "8,715" },
        ],
    },
    Assertion {
        id: "health-care-fell-too",
        statement: "Health care passed manufacturing to become Allen County's largest private \
                    employer sector, and then it fell too — from 12,431 in 2013 to 9,151 in 2022. \
                    Hospitals are 1,866 of the 2,593 jobs the sector has lost since 2010.",
        topic: "population",
        // The figures are the composition of the loss rather than the series, because the
        // series was what the corpus got wrong: it was read at five-year steps and the peak
        // fell in a year that was never sampled. See a-sample-is-not-a-series.
        supports: &[
            support!(
                "measure/allen-county-employment-by-sector-1986-2022.yml",
                "Sector 62 peaked at **12,431 in 2013**"
            ),
            support!(
                "measure/allen-county-employment-by-sector-1986-2022.yml",
                "Of the 2,593 lost since 2010, hospitals are 1,866, nursing and residential care 549, social assistance 170, and ambulatory care 8."
            ),
            support!(
                "measure/allen-county-health-care-employment-2010-2022.yml",
                "**The sector peaked at 12,431 in 2013, in a year nobody had looked at**, and hospitals are where the fall since went"
            ),
        ],
        // The corpus can now say what contracted and still cannot say why, and the site carries
        // the second half beside the first rather than letting a composition chart imply a cause.
        answers: &["does not establish why any of it fell"],
        figures: &[
            Figure { label: "Hospitals", value: 1_866.0, literal: "1,866" },
            Figure { label: "Nursing and residential care", value: 549.0, literal: "549" },
            Figure { label: "Social assistance", value: 170.0, literal: "170" },
            Figure { label: "Ambulatory care", value: 8.0, literal: "8" },
        ],
    },
    Assertion {
        id: "five-hospitals-one-locally-owned",
        statement: "Allen County has five Medicare-certified hospitals. One of them has no \
                    corporate owner at all. The other four answer to Cincinnati, Findlay and \
                    Louisville.",
        topic: "population",
        // The first assertion this site makes about a private employer operating in the county
        // today. Cited to the five organization nodes rather than to a measure, because the
        // claim is about who they are and not about a number.
        supports: &[
            support!(
                "organization/lima-memorial-health-system.yml",
                "The federal owners file lists no organizational owner for it at all"
            ),
            support!(
                "organization/mercy-health-st-ritas-medical-center.yml",
                "Bon Secours Mercy Health Inc holds 100 per cent of it, recorded from 1 January 2020"
            ),
            support!(
                "organization/bluffton-hospital.yml",
                "Blanchard Valley Health System of Findlay, in Hancock County, holds 100 per cent of it"
            ),
            support!(
                "organization/kindred-hospital-lima.yml",
                "**Its owners run thirteen deep and all of them are in Louisville, Kentucky.**"
            ),
            support!(
                "organization/institute-for-orthopaedic-surgery.yml",
                "Mercy Health\u{2013}St. Rita's holds 51 per cent of it directly, from 1 April 2004"
            ),
        ],
        // Three refusals from three of the five nodes, carried rather than routed around. Two of
        // them are the same shape — a figure moved and the corpus cannot say whether the thing it
        // measures moved with it — and the third is about a founding this corpus has not found.
        answers: &[
            "the corpus does not know",
            "the corpus cannot say",
            "does not establish that the pandemic caused",
        ],
        figures: &[],
    },
    Assertion {
        id: "the-refinery-outgrew-them",
        statement: "The county's oldest industrial installation is bigger than it has ever been. \
                    The Lima refinery could run 145,000 barrels of crude a day in 1994 and can run \
                    183,000 now — the largest refinery in Ohio, and 36th of the 124 in the United \
                    States.",
        topic: "population",
        // The counterweight to every other employment chart on this page. Manufacturing
        // employment halved across the same span in which this plant's capacity rose a quarter,
        // which is the difference between a county's output and its payroll.
        supports: &[
            support!(
                "measure/lima-refinery-capacity-1994-2026.yml",
                "**The refinery is larger than it has ever been in this series, and by a quarter.** 145,000 barrels per calendar day in 1994 against 183,000 in 2026"
            ),
            support!(
                "measure/lima-refinery-capacity-1994-2026.yml",
                "**It is the largest refinery in Ohio.** 183,000 against 172,800 and 150,800 at Toledo and 100,000 at Canton"
            ),
            support!(
                "measure/lima-refinery-capacity-1994-2026.yml",
                "**The trough is 146,120 in 2007, the last Premcor year, and the recovery is the Husky years.** Capacity fell 15,380 from the 161,500 it held through 2003"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1994", value: 145_000.0, literal: "145,000" },
            Figure { label: "2003", value: 161_500.0, literal: "161,500" },
            Figure { label: "2007", value: 146_120.0, literal: "146,120" },
            Figure { label: "2026", value: 183_000.0, literal: "183,000" },
        ],
    },
    Assertion {
        id: "the-plants-are-outside-every-city",
        statement: "All three of Allen County's large industrial installations stand outside every \
                    municipality. The refinery and the tank plant are in Shawnee Township, the Ford \
                    engine plant in Bath. All three have Lima postal addresses.",
        topic: "population",
        // Five instances of a-postal-address-is-not-a-municipality, and the last two were checked
        // before the claim was written rather than after it.
        supports: &[
            support!(
                "site/ford-lima-engine-plant.yml",
                "**So all three of the county's large industrial installations stand outside every municipality.**"
            ),
            support!(
                "site/lima-refinery.yml",
                "**It has a Lima address and is not in Lima.**"
            ),
            support!(
                "site/lima-army-tank-plant.yml",
                "**It is not in Lima.** The plant stands in [Shawnee Township](../place/shawnee-township.yml)"
            ),
        ],
        // Found only because the wrap fix in claim.rs made it visible. This refusal had been in
        // the tank plant node and invisible to the gate, which is what the fix was about. The
        // second entry arrived the same way and by a different route: a later phase added a
        // refusal to that node about which wartime installation stands on this ground, and this
        // assertion — which is about which township the ground is in — failed the build until it
        // carried it. Propagation working, on an assertion nobody was editing.
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[],
    },
    Assertion {
        id: "what-the-concentration-is-made-of",
        statement: "This corpus has said for ten phases that the county's decline is concentrated \
                    in Lima. Lima is 34.7 per cent of the county's people, 64.1 per cent of its \
                    poor, and 76.7 per cent of its Black residents.",
        topic: "population",
        // The three shares close exactly against the county's own rows, which is why they are
        // plotted together: they are the same city measured three ways, not three estimates.
        supports: &[
            support!(
                "measure/allen-county-income-and-poverty-2023.yml",
                "35,304 of 101,685 residents — 34.7 per cent — and 8,214 of the 12,815 people below the poverty line, or 64.1 per cent."
            ),
            support!(
                "measure/allen-county-race-2023.yml",
                "8,290 of 10,805, or 76.7 per cent ± 4.6, against a 34.7 per cent share of the population."
            ),
            support!(
                "measure/allen-county-income-and-poverty-2023.yml",
                "$43,370 against\n  Shawnee Township's $91,134, a gap of $47,764 against a combined margin of $6,359."
            ),
        ],
        answers: &[
            "This does not establish that Lima has the lowest household income in the county.",
            "This does not establish why the county is distributed this way.",
        ],
        figures: &[
            Figure { label: "Population", value: 34.7, literal: "34.7" },
            Figure { label: "People in poverty", value: 64.1, literal: "64.1" },
            Figure { label: "Black residents", value: 76.7, literal: "76.7" },
        ],
    },
    Assertion {
        id: "lima-is-the-young-end",
        statement: "And Lima is the young end of this county, not the old one — median age 35.4 \
                    against the county's 39.7, with eleven of the twelve townships older.",
        topic: "population",
        // Published because two things this site already asserts compose into a false story if
        // nobody says otherwise: natural decrease as a component of the loss, and the loss
        // concentrated in Lima.
        supports: &[
            support!(
                "measure/allen-county-age-structure-2023.yml",
                "Its median age is 35.4 against the\n  county's 39.7, its 65-and-over share is 15.6 per cent against 18.7"
            ),
            support!(
                "measure/allen-county-age-structure-2023.yml",
                "Eleven of the twelve townships are older than the city by\n  the 65-and-over measure."
            ),
        ],
        answers: &["The corpus does not establish that."],
        figures: &[],
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
        id: "three-quarters-of-the-county-drains-badly",
        statement: "74.7 per cent of Allen County is poorly drained or worse and 2.4 per cent is well drained. Poorly drained is a measurement of this ground rather than an epithet for it.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-soils-2026.yml", "**Three quarters of this county drains badly and one fortieth of it drains well.** 74.7 per cent is poorly drained or worse and 2.4 per cent is well drained."),
            support!("measure/allen-county-soils-2026.yml", "**35.3 per cent of the county is hydric soil, which is ground that formed under standing water.** 91,953 acres."),
        ],
        answers: &["does not assert that the hydric acres are the Great Black Swamp"],
        figures: &[
            Figure { label: "poorly drained or worse", value: 74.7, literal: "74.7" },
            Figure { label: "hydric soil", value: 35.3, literal: "35.3" },
            Figure { label: "well drained", value: 2.4, literal: "2.4" },
        ],
    },
    Assertion {
        id: "prime-only-because-it-was-drained",
        statement: "Five acres in six of Allen County's prime farmland are prime only because they were drained: 76.3 per cent of the county is classed prime if drained against 14.9 per cent prime as it lies.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-soils-2026.yml", "**Five acres in six of this county's prime farmland are prime only because they were drained.** 76.3 per cent of the county is classed prime *if drained* and 14.9 per cent is prime as it lies, so of the 91.3 per cent that can be prime farmland, 83.6 per cent of it needs the water taken off first."),
        ],
        answers: &["does not assert that the hydric acres are the Great Black Swamp"],
        figures: &[
            Figure { label: "prime if drained", value: 76.3, literal: "76.3" },
            Figure { label: "prime as it lies", value: 14.9, literal: "14.9" },
        ],
    },
    Assertion {
        id: "the-black-swamp-here-is-clay",
        statement: "The organic soils a person picturing a swamp would expect cover 14 acres of Allen County's 260,340. Water stood on this ground because the glacial till beneath it would not let water through.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-soils-2026.yml", "**The Black Swamp here is clay and not peat.** Histosols \u{2014} the organic soils a person picturing a swamp would expect \u{2014} cover 14 acres of 260,340. The county is 58.7 per cent Alfisols and 32.9 per cent Mollisols."),
        ],
        answers: &["does not assert that the hydric acres are the Great Black Swamp"],
        figures: &[
            Figure { label: "Alfisols", value: 58.7, literal: "58.7" },
            Figure { label: "Mollisols", value: 32.9, literal: "32.9" },
        ],
    },
    Assertion {
        id: "two-files-a-hundred-and-thirty-times-apart",
        statement: "One federal file finds 704 acres of wetland in Allen County and another rates 91,953 acres hydric. Both are right: one counts what stands there now, the other what the water made and left behind.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-soils-2026.yml", "**Two federal files count wetland in this county and they are 130 times apart.** The Cropland Data Layer finds 704 acres of wetland in 2024 and this survey rates 91,953 acres hydric."),
        ],
        answers: &["does not assert that the hydric acres are the Great Black Swamp"],
        figures: &[
            Figure { label: "wetland, 2024", value: 704.0, literal: "704" },
            Figure { label: "hydric soil", value: 91953.0, literal: "91,953" },
        ],
    },
    Assertion {
        id: "the-wet-ground-runs-north-and-west",
        statement: "Marion Township is 58.0 per cent hydric soil and Bath 21.1. The four townships north of 40.80\u{b0} average 46.0 per cent against 30.9 for the other eight \u{2014} but Spencer, in the far south-west, is 45.2.",
        topic: "geography",
        supports: &[
            support!("natural-feature/great-black-swamp.yml", "**The direction this node guessed at is right and the compass point is not.** The four townships whose centres lie north of 40.80\u{b0} \u{2014} Marion, Sugar Creek, Monroe and Richland \u{2014} average 46.0 per cent hydric against 30.9 for the other eight, but Spencer in the far south-west is 45.2 and Richland in the north-east is 33.8."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "north of 40.80\u{b0}", value: 46.0, literal: "46.0" },
            Figure { label: "the other eight", value: 30.9, literal: "30.9" },
            Figure { label: "Spencer", value: 45.2, literal: "45.2" },
        ],
    },
    Assertion {
        id: "the-county-seat-is-on-the-driest-ground",
        statement: "Lima is 13.4 per cent hydric soil against 74.7 per cent poorly drained for the county, and Bath \u{2014} the township the city grew into \u{2014} is the driest of the twelve at 21.1 per cent.",
        topic: "geography",
        supports: &[
            support!("natural-feature/great-black-swamp.yml", "**The driest ground in the county is the ground the county seat is on.** Lima is 13.4 per cent hydric and 33.2 per cent poorly drained or worse, against 74.7 for the county. [verified] \u{2014} [the same survey](../../catalog/usda-ssurgo-soil-survey.md), clipped the same way. Bath, the township the city grew into, is the driest of the twelve at 21.1."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Lima", value: 13.4, literal: "13.4" },
            Figure { label: "Bath", value: 21.1, literal: "21.1" },
            Figure { label: "the county", value: 74.7, literal: "74.7" },
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
        id: "the-county-had-two-tank-installations",
        statement: "Allen County had two tank installations in the Second World War, a year and a \
                    category apart: the Lima Locomotive Works, which built medium tanks from 1941, \
                    and the Army's own plant south of the city, begun in May 1942, which finished \
                    and forwarded vehicles built elsewhere and stands there still.",
        topic: "history",
        supports: &[
            support!(
                "site/lima-locomotive-works-plant.yml",
                "In February 1941 the works had \"under construction a new $290,000 factory building\", which the local press read as confirmation that it was switching part of its activity to defense production; by that August medium tank production was scheduled to begin \"this fall\"."
            ),
            support!(
                "site/lima-army-tank-plant.yml",
                "**This installation is the other one, and it is the Lima Tank Depot.** Construction began here in May 1942; the Ordnance Corps turned it from a gun-tube plant into a depot for modifying and processing combat vehicles before it was finished; United Motors Service, a General Motors subsidiary, took it over under contract in November 1942; and more than a hundred thousand vehicles passed through before the war ended."
            ),
        ],
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[],
    },
    Assertion {
        id: "the-war-turned-the-locomotive-works-books",
        statement: "The Lima Locomotive Works booked 51 locomotives in 1940 against 14 in 1939, \
                    and turned a net profit of $87,007 after a net loss of $134,326.",
        topic: "history",
        supports: &[support!(
            "organization/lima-locomotive-works.yml",
            "The company booked orders for 51 locomotives in 1940 against 14 in 1939, closed 1940 with a backlog of 32 engines and 19 delivered in the year, and turned a net profit of $87,007 after a net loss of $134,326 in 1939."
        )],
        answers: &[],
        figures: &[
            Figure { label: "1939", value: 14.0, literal: "14" },
            Figure { label: "1940", value: 51.0, literal: "51" },
        ],
    },
    Assertion {
        id: "the-night-the-jail-was-taken",
        statement: "On the night of 12 October 1933 three armed men walked into the Allen County \
                    jail at Lima, shot the sheriff and freed John Dillinger. It is the county's \
                    first recorded event of the twentieth century.",
        topic: "history",
        supports: &[support!(
            "event/allen-county-jail-raid-1933.yml",
            "freed the prisoner John Dillinger from his cell, locked the sheriff's wife and his deputy in it, stripped the building of its guns and ammunition and drove west out of town."
        )],
        answers: &[
            "does not establish where the jail stood",
            "does not establish what the trials cost the county",
        ],
        figures: &[],
    },
    Assertion {
        id: "the-office-passed-from-father-to-son",
        statement: "The sheriff's roster shows Jess L. Sarber ending in 1933 and Donald F. Sarber \
                    beginning, and gives no reason for either. The reason is that the first was \
                    killed in office and the second was his son.",
        topic: "history",
        supports: &[
            support!(
                "question/two-irregular-sheriff-transitions.yml",
                "Jess L. Sarber was shot dead in the Allen County jail on the night of 12 October 1933 by three men who came to free John Dillinger, and his son Donald succeeded him."
            ),
            support!(
                "tenure/sheriff-1931-jess-l-sarber.yml",
                "Jess L. Sarber held the office of Sheriff of Allen County from 1931 until he was shot dead in the county jail on 12 October 1933."
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-man-in-the-cell-was-charged-with-an-allen-county-bank",
        statement: "Dillinger was in this county's jail because he was charged with robbing a bank \
                    in this county — the Commercial Bank at Bluffton, of $2200.",
        topic: "history",
        supports: &[support!(
            "place/bluffton.yml",
            "He was brought up from Dayton in the autumn of 1933 charged with robbing \"the Bluffton, O., Commercial Bank of $2200\", and was awaiting trial at Lima when three men shot the sheriff and took him out of the cell."
        )],
        // Arrived by propagation, from a phase about the survey grid rather than about a bank
        // robbery: Bluffton's two witnesses to its own ground disagree because the village
        // straddles a county line, so the node refuses to name its section and every assertion
        // citing the node has to carry that.
        answers: &["The corpus does not assert its section"],
        figures: &[Figure { label: "Charged sum", value: 2_200.0, literal: "2200" }],
    },
    Assertion {
        id: "the-county-is-two-thirds-farmland",
        statement: "Nine-tenths of Allen County was farmland in 1910 and two-thirds of it is \
                    now. Nine-tenths of what left went in the ninety-two years no census in \
                    this catalog covers.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-farms-2002-2022.yml",
                "**Allen County was 92.5 per cent farmland in 1910 and is 69.4 per cent farmland now** — 178,921 acres of the 257,629 in its 402.545 square miles."
            ),
            support!(
                "measure/allen-county-farms-2002-2022.yml",
                "Land in farms was 240,472 acres in 1910 and 188,150 in 2002 — 52,322 acres gone — against 9,229 more across the twenty years since."
            ),
            support!(
                "measure/allen-county-farms-2002-2022.yml",
                "188,150 acres in 2002, 187,238 in 2007, 183,186 in 2012, 186,623 in 2017 and 178,921 in 2022, against 240,472 in 1910."
            ),
        ],
        // The node's whole finding is that it has one observation at each end of a century and
        // none in between, so the refusal is the assertion's other half rather than a caveat on it.
        // The refusal this answered is gone: the corpus can now say when, and the
        // measure says so instead. Withdrawn with it.
        answers: &[],
        // Bars and not a line. A line between 1910 and 2002 draws a segment through ninety-two
        // years that nothing measured, and the shape of that segment is exactly what this corpus
        // refuses to assert.
        figures: &[
            Figure { label: "1910", value: 240_472.0, literal: "240,472" },
            Figure { label: "2002", value: 188_150.0, literal: "188,150" },
            Figure { label: "2007", value: 187_238.0, literal: "187,238" },
            Figure { label: "2012", value: 183_186.0, literal: "183,186" },
            Figure { label: "2017", value: 186_623.0, literal: "186,623" },
            Figure { label: "2022", value: 178_921.0, literal: "178,921" },
        ],
    },
    Assertion {
        id: "two-thousand-eight-hundred-wells-with-no-plugging-on-file",
        statement: "Ohio's register holds 4,849 oil and gas wells in Allen County. 2,851 of them are recorded as having produced and not one carries a date of plugging; of the 1,715 recorded as plugged, 84 carry the date it happened.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-oil-and-gas-wells-2026.yml", "Every well the state of Ohio holds a record of in this county: 4,849 of them, all vertical."),
            support!("measure/allen-county-oil-and-gas-wells-2026.yml", "**Three wells in five are recorded as having produced and are not recorded as having been plugged.** 2,851 of the 4,834 in the feature service carry the status *historical production well*, and not one of them carries a date of plugging. [verified] \u{2014} the same source, the feature service. 1,715 are recorded as plugged and abandoned, and 84 of those 1,715 carry the date it happened."),
        ],
        answers: &["cannot say how many wells the boom drilled"],
        figures: &[
            Figure { label: "produced, no plugging on file", value: 2851.0, literal: "2,851" },
            Figure { label: "plugged and abandoned", value: 1715.0, literal: "1,715" },
            Figure { label: "with a plugging date", value: 84.0, literal: "84" },
        ],
    },
    Assertion {
        id: "a-hundred-and-eight-orphan-wells",
        statement: "A hundred and eight of Allen County's wells are in Ohio's orphan-well programme: 52 referred, 35 scheduled, 11 plugged under it, 7 under the federal programme, 2 permitted and 1 plugged as an emergency.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-oil-and-gas-wells-2026.yml", "**A hundred and eight of them are in the state's orphan-well programme.** 52 referred, 35 scheduled under the traditional programme, 11 already plugged under it, 7 scheduled under the federal programme, 2 permitted and 1 plugged as an emergency."),
        ],
        answers: &["cannot say how many wells the boom drilled"],
        figures: &[
            Figure { label: "referred", value: 52.0, literal: "52" },
            Figure { label: "scheduled", value: 35.0, literal: "35" },
            Figure { label: "plugged under it", value: 11.0, literal: "11" },
            Figure { label: "federal programme", value: 7.0, literal: "7" },
        ],
    },
    Assertion {
        id: "the-oil-field-is-a-shape-east-of-the-city",
        statement: "Bath and Perry townships, which wrap Lima on the north and east, hold 1,615 wells across 63.4 square miles. Auglaize and Monroe, at the county's far edges, hold 42 across 72.1 \u{2014} a 116-fold range in density inside one county.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-oil-and-gas-wells-2026.yml", "**The field is a shape and its centre is just east of the city.** Bath and Perry, which wrap Lima on the north and east, hold 1,615 wells between 63.4 square miles; Auglaize and Monroe, at the county's south-east and north edges, hold 42 between 72.1. That is 25.51 wells to the square mile in Bath and 25.44 in Perry against 0.94 in Monroe and 0.22 in Auglaize \u{2014} the densest township has 116 times the wells per square mile of the thinnest."),
        ],
        answers: &["cannot say how many wells the boom drilled"],
        figures: &[
            Figure { label: "Bath", value: 25.51, literal: "25.51" },
            Figure { label: "Perry", value: 25.44, literal: "25.44" },
            Figure { label: "Monroe", value: 0.94, literal: "0.94" },
            Figure { label: "Auglaize", value: 0.22, literal: "0.22" },
        ],
    },
    Assertion {
        id: "one-formation-at-one-depth",
        statement: "39 of the 43 Allen County wells that name a producing formation name the Trenton Limestone, and 230 of the 290 recording a total depth fall between 1,200 and 1,399 feet. The county's oil is one bed at one depth.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-oil-and-gas-wells-2026.yml", "**They are one formation at one depth.** 39 of the 43 wells in the county that name a producing formation name the Trenton Limestone; of the 290 that record a total depth, the median is 1,332 feet and 230 fall between 1,200 and 1,399."),
            support!("period/lima-oil-boom.yml", "**The field left 4,849 holes in the ground and this node had never counted them.** The state's register holds that many wells in Allen County, all vertical, 39 of the 43 that name a producing formation naming the Trenton Limestone, and a median total depth of 1,332 feet across the 290 that record one."),
        ],
        answers: &["cannot say how many wells the boom drilled"],
        figures: &[
            Figure { label: "median depth, feet", value: 1332.0, literal: "1,332" },
            Figure { label: "wells recording a depth", value: 290.0, literal: "290" },
            Figure { label: "of those, 1,200 to 1,399 ft", value: 230.0, literal: "230" },
        ],
    },
    Assertion {
        id: "the-register-begins-after-the-field-does",
        statement: "The earliest completion date Ohio's well register holds for Allen County is 1890, five years after the strike, and thirty of its wells carry a completion date before 1920. The boom's holes are countable and its drilling is not.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-oil-and-gas-wells-2026.yml", "**The register begins five years after the field does.** Its earliest completion date for this county is 1890 and [the strike](../event/lima-oil-strike.yml) was 9 May 1885; thirty wells here carry a completion date before 1920."),
            support!("measure/allen-county-oil-and-gas-wells-2026.yml", "This corpus cannot say how many wells the boom drilled."),
        ],
        answers: &["cannot say how many wells the boom drilled"],
        figures: &[],
    },
    Assertion {
        id: "the-hogs-multiplied-as-the-keepers-halved",
        statement: "Allen County kept 31,741 hogs on 59 places in 2002 and 235,800 on 28 in \
                    2022. The herd multiplied more than sevenfold while the places keeping it \
                    halved.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-livestock-2002-2022.yml",
                "31,741 head on 59 operations in 2002 against 235,800 on 28 in 2022 — an average herd of 538 becoming one of 8,421."
            ),
            support!(
                "measure/allen-county-livestock-2002-2022.yml",
                "31,741 hogs in 2002, 62,910 in 2007, 80,372 in 2012, 178,781 in 2017 and 235,800 in 2022, kept on 59, 60, 46, 56 and 28 operations."
            ),
            support!(
                "measure/allen-county-livestock-2002-2022.yml",
                "Of 377,816 head sold in 2022, 281,194 were produced under production contract on twelve operations — the census's own term for an animal fed by one party and owned by another."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2002", value: 31_741.0, literal: "31,741" },
            Figure { label: "2007", value: 62_910.0, literal: "62,910" },
            Figure { label: "2012", value: 80_372.0, literal: "80,372" },
            Figure { label: "2017", value: 178_781.0, literal: "178,781" },
            Figure { label: "2022", value: 235_800.0, literal: "235,800" },
        ],
    },
    Assertion {
        id: "most-of-the-farmland-is-rented",
        statement: "More than half of Allen County's farmland is worked by someone who \
                    does not own it, and 268 operations farm 78.4 per cent of the ground.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-farms-2002-2022.yml",
                "94,325 of 178,921 acres are rented — 90,661 by 268 part-owner operations and 3,664 by 38 tenants — and the 591 operations that own everything they farm hold 34,912 acres between them, a fifth of the total."
            ),
            support!(
                "measure/allen-county-farms-2002-2022.yml",
                "**So 268 operations farm 78.4 per cent of the ground.**"
            ),
        ],
        // The refusal this answered is gone: the corpus can now say when, and the
        // measure says so instead. Withdrawn with it.
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "an-acre-yields-five-times-what-it-did",
        statement: "An acre of Allen County corn yields five times what it did in 1909 and an \
                    acre of wheat six times — and the county's largest crop today was not \
                    grown here at all.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-crops-2022.yml",
                "**An acre of Allen County corn yields five times what it did**, 192.7 bushels against 38.7, and an acre of wheat six times, 81.8 against 13.4."
            ),
            support!(
                "measure/allen-county-crops-2022.yml",
                "Soybeans took 76,200 acres in 2022, more than corn, and the 1910 census names no soybean acreage in Allen County at all."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Corn, 1909", value: 38.7, literal: "38.7" },
            Figure { label: "Corn, 2022", value: 192.7, literal: "192.7" },
            Figure { label: "Wheat, 1909", value: 13.4, literal: "13.4" },
            Figure { label: "Wheat, 2022", value: 81.8, literal: "81.8" },
        ],
    },
    Assertion {
        id: "lima-was-a-railroad-town-before-an-oil-town",
        statement: "Lima was an industrial city six years before the oil. In 1879 it stood at \
                    the crossing of three railroads, all of which kept repair shops there, and \
                    the largest establishment in town was a locomotive shop.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-railroads-1883-1921.yml",
                "In March 1879, at a population of about 7,000, the *Allen County Republican* described the city as standing at the crossing of the Pittsburg, Fort Wayne & Chicago, the Dayton & Michigan and the Lake Erie & Louisville, with all three keeping repair shops there, a street railroad running from the Union Depot, and the Dayton & Michigan shops covering some eight acres and employing 200 to 300 men."
            ),
            support!(
                "measure/allen-county-railroads-1883-1921.yml",
                "there is no rival in the 1879 list: the next largest establishments named are furniture factories of seven to twenty-five hands."
            ),
            support!(
                "event/the-first-railroad-reaches-allen-county.yml",
                "Within a decade Lima stood at the crossing of the two roads and Delphos, the canal town, did not."
            ),
        ],
        // The node refuses a workforce total and this assertion needs none: it rests on what one
        // newspaper listed in one week, which is a ranking of the establishments it names and not
        // a count of anybody.
        answers: &["The corpus cannot say how large the railroad workforce of this county ever was"],
        figures: &[],
    },
    Assertion {
        id: "no-passenger-train-calls-in-this-county",
        statement: "Five railroads still cross Allen County at 201 open crossings, 108 of them \
                    in Lima. Not one of the 201 sees a passenger train.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-railroads-2026.yml",
                "**Not one of the 201 open crossings sees a passenger train.**"
            ),
            support!(
                "measure/allen-county-railroads-2026.yml",
                "**Lima holds 108 of the 201.** Then Spencerville with 16, Lafayette 15, Beaverdam 14, Delphos 13, Elida 12, Cairo 10 and Bluffton 7."
            ),
            support!(
                "measure/allen-county-railroads-1883-1921.yml",
                "**By 1906 two shops employed more than five hundred men each**"
            ),
        ],
        answers: &[
            "The corpus cannot say how large the railroad workforce of this county ever was",
            "this corpus cannot say whether the network shrank, moved or merely changed hands",
        ],
        figures: &[
            Figure { label: "Lima", value: 108.0, literal: "108" },
            Figure { label: "Spencerville", value: 16.0, literal: "16" },
            Figure { label: "Lafayette", value: 15.0, literal: "15" },
            Figure { label: "Beaverdam", value: 14.0, literal: "14" },
            Figure { label: "Delphos", value: 13.0, literal: "13" },
            Figure { label: "Elida", value: 12.0, literal: "12" },
            Figure { label: "Cairo", value: 10.0, literal: "10" },
            Figure { label: "Bluffton", value: 7.0, literal: "7" },
        ],
    },
    Assertion {
        id: "lima-elects-by-ward-and-fills-its-seats",
        statement: "Lima elects all seven of its council members by ward and none at large — the \
                    only government in Allen County chosen by district. It fills eleven seats and \
                    one of them is an appointee, where the seven villages fill fifty-two and \
                    twenty-five are.",
        topic: "government",
        supports: &[
            support!(
                "measure/lima-city-government-2026.yml",
                "**Eleven seats, eleven offices, none vacant, and one filled by appointment.**"
            ),
            support!(
                "measure/lima-city-government-2026.yml",
                "Its seven council members are chosen one per ward and it elects none at large; every village here elects its council at large, both township boards are elected township-wide, and Delphos elects three at large beside two by ward."
            ),
            support!(
                "measure/allen-county-elected-seats-2026.yml",
                "Twenty-five of the forty-seven seated village officers were appointed and five village seats stand empty, against one appointment and no vacancy across all forty-eight township seats."
            ),
        ],
        answers: &["This does not establish that these seats go unfilled because nobody stands for them"],
        figures: &[],
    },
    Assertion {
        id: "lima-tracts-live-five-years-less",
        statement: "A person born in one of Lima's census tracts could expect about five fewer \
                    years than one born elsewhere in Allen County. Across the county's thirty \
                    tracts the range is 13.7 years.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-life-expectancy-2010-2015.yml",
                "From **69.1** years in tract `39003012600` to **82.8** in `39003011300`, on a median of 78.0."
            ),
            support!(
                "measure/allen-county-life-expectancy-2010-2015.yml",
                "their median is **72.7** years against **79.0** for the fifteen outside, and weighted by tract population, 73.9 against 79.1."
            ),
            support!(
                "measure/allen-county-life-expectancy-2010-2015.yml",
                "**The two groups barely overlap.** The highest Lima tract is 78.2 and the lowest tract outside the city is 74.0"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Lowest tract", value: 69.1, literal: "69.1" },
            Figure { label: "Lima tracts, median", value: 72.7, literal: "72.7" },
            Figure { label: "Other tracts, median", value: 79.0, literal: "79.0" },
            Figure { label: "Highest tract", value: 82.8, literal: "82.8" },
        ],
    },
    Assertion {
        id: "the-health-gap-is-a-gap-in-conditions",
        statement: "Thirteen of CDC's forty health measures separate Lima from its county by more \
                    than their confidence limits allow. Five of the top eight are not diseases, \
                    and no chronic-disease gap clears at all.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-health-2023.yml",
                "**Thirteen of the forty differ between Lima and its county by more than their confidence limits allow**, and eleven of the thirteen are Lima the worse."
            ),
            support!(
                "measure/allen-county-health-2023.yml",
                "Food stamps, food insecurity, housing insecurity, utility shut-off threat and lack of reliable transportation are the measures CDC groups as health-related social needs, and they are where the city and the county separate most."
            ),
            support!(
                "measure/allen-county-health-2023.yml",
                "Diabetes, high blood pressure, coronary heart disease, stroke, asthma, chronic obstructive pulmonary disease, depression and obesity all estimate higher in Lima and every one of those differences sits inside its own confidence limits."
            ),
            support!(
                "measure/allen-county-health-2023.yml",
                "CDC's estimate of adults receiving food stamps runs from 4.9 to 46.1 per cent, food insecurity from 7.2 to 41.3, fair or poor health from 13.9 to 40.5, and total tooth loss among the over-65s from 7.6 to 42.7."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Food stamps, lowest tract", value: 4.9, literal: "4.9" },
            Figure { label: "Food stamps, highest", value: 46.1, literal: "46.1" },
            Figure { label: "Food insecurity, lowest", value: 7.2, literal: "7.2" },
            Figure { label: "Food insecurity, highest", value: 41.3, literal: "41.3" },
            Figure { label: "Fair or poor health, lowest", value: 13.9, literal: "13.9" },
            Figure { label: "Fair or poor health, highest", value: 40.5, literal: "40.5" },
            Figure { label: "Teeth lost 65+, lowest", value: 7.6, literal: "7.6" },
            Figure { label: "Teeth lost 65+, highest", value: 42.7, literal: "42.7" },
        ],
    },
    Assertion {
        id: "the-biggest-employer-is-not-the-biggest-payer",
        statement: "Health care is the largest employer in Allen County and manufacturing is the \
                    largest payer. Retail and food service together employ more people than \
                    manufacturing and take a tenth of the county's wages.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-wages-2024.yml",
                "Health care 9,767 jobs at $60,502 a year; manufacturing 8,779 at $92,088; retail trade 5,261 at $34,261; accommodation and food services 4,737 at $20,844; educational services 3,899 at $47,735."
            ),
            support!(
                "measure/allen-county-wages-2024.yml",
                "It is the only sector in the county whose share of pay is more than half again its share of employment."
            ),
            support!(
                "measure/allen-county-wages-2024.yml",
                "9,998 jobs against 8,779, and 9.7 per cent of $2,884,327,418 in total annual wages."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Manufacturing", value: 92_088.0, literal: "92,088" },
            Figure { label: "Health care", value: 60_502.0, literal: "60,502" },
            Figure { label: "Education", value: 47_735.0, literal: "47,735" },
            Figure { label: "Retail trade", value: 34_261.0, literal: "34,261" },
            Figure { label: "Food service", value: 20_844.0, literal: "20,844" },
        ],
    },
    Assertion {
        id: "half-the-jobs-here-are-held-by-outsiders",
        statement: "Half of Allen County's jobs are held by people who live somewhere else, and \
                    two fifths of its employed residents work outside it. 42,643 people cross the \
                    county line on a working day.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-commuting-2022.yml",
                "24,461 people both live and work in Allen County, 24,269 commute into it, and 18,374 residents leave it for work — on 48,730 jobs located here and 42,835 held by people who live here."
            ),
            support!(
                "measure/allen-county-commuting-2022.yml",
                "**The county draws 5,895 more workers than it sends**, and 42,643 people cross its line in one direction or the other on a working day."
            ),
            support!(
                "measure/allen-county-commuting-2022.yml",
                "Of jobs held by residents working in the county, 47.1 per cent are in the file's top earnings band, above $3,333 a month; of in-commuters' jobs, 50.8 per cent; of out-commuters', 49.4."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Live and work here", value: 24_461.0, literal: "24,461" },
            Figure { label: "Commute in", value: 24_269.0, literal: "24,269" },
            Figure { label: "Commute out", value: 18_374.0, literal: "18,374" },
        ],
    },
    Assertion {
        id: "one-person-in-ten-went-to-war",
        statement: "Allen County had 19,185 people in 1860 and put 1,920 men into United States \
                    service over the four years of the Civil War — one person in ten, counting \
                    women, children and the old.",
        topic: "history",
        supports: &[
            support!(
                "period/the-civil-war-in-allen-county.yml",
                "The four years in which a county of 19,185 people put **1,920 men** into United States service — one person in ten, counting women, children and the old."
            ),
            support!(
                "measure/allen-county-civil-war-service.yml",
                "**The 1,920 is a roster total and the roster is printed**, forty-nine pages of it"
            ),
            support!(
                "measure/allen-county-civil-war-service.yml",
                "There is no conflict. A first-year cohort and a two-and-a-half-year credit are different measurements, and the 1921 book is setting them against each other because the 1906 book dropped four words."
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-roster-never-totalled-its-own-dead",
        statement: "The 1885 county history records the fate of each of its 1,920 soldiers and \
                    never adds them up. Counted, at least 164 did not come home, and only about \
                    one death in five was in action.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-civil-war-dead.yml",
                "**At least 164 of the 1,920 did not come home** — 8.5 per cent of the men of record — and the figure is a floor, not a total."
            ),
            support!(
                "measure/allen-county-civil-war-dead.yml",
                "**About one death in five was in action.** By cause, taking the lower reading of each so that every figure is a floor: **35** killed or mortally wounded, **5** dead of wounds, **3** drowned or lost at sea, and **121** whom the roster gives a place and a date and no cause at all."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Killed or mortally wounded", value: 35.0, literal: "35" },
            Figure { label: "Died of wounds", value: 5.0, literal: "5" },
            Figure { label: "Drowned or lost at sea", value: 3.0, literal: "3" },
            Figure { label: "No cause recorded", value: 121.0, literal: "121" },
        ],
    },
    Assertion {
        id: "more-died-at-corinth-than-anywhere",
        statement: "More Allen County men died at Corinth, Mississippi than at any other named \
                    place — where the 81st Ohio, the regiment the county filled most, fought in \
                    October 1862 and then wintered.",
        topic: "history",
        supports: &[support!(
            "measure/allen-county-civil-war-dead.yml",
            "**More Allen County men died at Corinth, Mississippi, than anywhere else.** Taking the lower of the two readings at each place: Corinth **11**, Atlanta **9**, Nashville **9**, Chattanooga **4**, Rome **3**, Andersonville **2**, Resaca **2**, Knoxville **2**."
        )],
        answers: &[],
        figures: &[
            Figure { label: "Corinth, Miss.", value: 11.0, literal: "11" },
            Figure { label: "Atlanta, Ga.", value: 9.0, literal: "9" },
            Figure { label: "Nashville, Tenn.", value: 9.0, literal: "9" },
            Figure { label: "Chattanooga, Tenn.", value: 4.0, literal: "4" },
            Figure { label: "Rome, Ga.", value: 3.0, literal: "3" },
            Figure { label: "Andersonville, Ga.", value: 2.0, literal: "2" },
            Figure { label: "Resaca, Ga.", value: 2.0, literal: "2" },
            Figure { label: "Knoxville, Tenn.", value: 2.0, literal: "2" },
        ],
    },
    Assertion {
        id: "three-thousand-people-live-outside-households",
        statement: "3,522 people in Allen County were not living in a household when the 2020 \
                    census counted them — one in twenty-nine, in 43 of its 3,552 blocks.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-group-quarters-2020.yml",
                "The 3,522 people the 2020 census counted in Allen County who were not living in a household — 3.4 per cent of its 102,206, in 43 of its 3,552 blocks."
            ),
            support!(
                "measure/allen-county-group-quarters-2020.yml",
                "**Lima holds nearly two thirds of the county's group quarters on a third of its people.** 2,210 of the 3,522, including every one of the 1,513 in correctional facilities."
            ),
            support!(
                "measure/allen-county-group-quarters-2020.yml",
                "**The nursing homes are the county's most distributed institution.** 966 people in fifteen blocks across the city and seven townships, the largest holding 132"
            ),
            support!(
                "measure/allen-county-group-quarters-2020.yml",
                "the prisons are two blocks, and 494 of the 788 in student housing are on one campus."
            ),
            support!(
                "measure/allen-county-group-quarters-2020.yml",
                "The county has 255 such residents, and 124 of them are in six blocks of one downtown Lima tract."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Correctional facilities", value: 1_513.0, literal: "1,513" },
            Figure { label: "Nursing facilities", value: 966.0, literal: "966" },
            Figure { label: "Student housing", value: 788.0, literal: "788" },
            Figure { label: "Shelters and group homes", value: 255.0, literal: "255" },
        ],
    },
    Assertion {
        id: "one-lima-housing-unit-in-nine-is-empty",
        statement: "Allen County had 44,563 housing units in 2020 and 3,628 of them stood empty. \
                    Lima's vacancy rate is 11.2 per cent, and no township in the county comes \
                    within three points of it.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-occupancy-2020.yml",
                "**40,935 were occupied and 3,628 were vacant** — a vacancy rate of 8.14 per cent."
            ),
            support!(
                "measure/allen-county-occupancy-2020.yml",
                "**One in nine housing units in Lima had nobody in it.** 11.2 per cent against 8.14 for the county and 4.5 for Amanda Township, and no township comes within three points of the city."
            ),
            support!(
                "measure/allen-county-occupancy-2020.yml",
                "3,522 people were not in households at all, so 98,684 were, in 40,935 occupied units: **2.411 people per household**."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Lima city", value: 11.2, literal: "11.2" },
            Figure { label: "Allen County", value: 8.14, literal: "8.14" },
            Figure { label: "Amanda township", value: 4.5, literal: "4.5" },
        ],
    },
    Assertion {
        id: "the-census-drew-the-block-on-the-fence",
        statement: "Two Allen County census blocks are its two state prisons. The block and the \
                    Census Bureau's own landmark carry the same land area to the square metre and \
                    the same internal point to seven decimal places.",
        topic: "geography",
        supports: &[support!(
            "measure/allen-county-group-quarters-2020.yml",
            "**Two of the county's blocks are its two state prisons, and the census drew the block on the fence.** Block `390030112001004` holds 1,360 people, all of them in adult correctional facilities; the Census Bureau's own landmark file gives *Allen Correctional Instn* the same land area to the square metre — 364,210 — and the same internal point to seven decimal places, `+40.7751673 -084.0996846`."
        )],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-county-lost-a-third-of-its-college-students",
        statement: "Allen County's six colleges enrolled 16,702 students in 2010 and 10,789 in \
                    2023 — a fall of 35.4 per cent, against a county that fell 3.9 per cent \
                    between its last two censuses. All six lost students.",
        topic: "schools",
        supports: &[
            support!(
                "measure/allen-county-college-enrollment-2010-2023.yml",
                "**The county's colleges have lost a third of their students in thirteen years, and all six of them have lost students.**"
            ),
            support!(
                "measure/allen-county-college-enrollment-2010-2023.yml",
                "**16,702 to 10,789 is a fall of 35.4 per cent.** [inference] — computed here. Institution by institution: the Ohio State Beauty Academy down 55.6 per cent, Ohio State's Lima campus 47.9, Apollo Career Center 45.7, the University of Northwestern Ohio 36.6, Bluffton University 38.1, and Rhodes State College 23.5 — the smallest fall, at the largest institution."
            ),
            support!(
                "measure/allen-county-college-enrollment-2010-2023.yml",
                "Allen County went from 106,331 people in 2010 to 102,206 in 2020, a fall of 3.9 per cent."
            ),
            support!(
                "measure/allen-county-college-enrollment-2010-2023.yml",
                "The fall from 2010 to 2019 is 4,017 students, or 24.0 per cent, and it is complete before Covid"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Ohio State Beauty Academy", value: 55.6, literal: "55.6" },
            Figure { label: "Ohio State-Lima", value: 47.9, literal: "47.9" },
            Figure { label: "Apollo Career Center", value: 45.7, literal: "45.7" },
            Figure { label: "Bluffton University", value: 38.1, literal: "38.1" },
            Figure { label: "Univ of Northwestern Ohio", value: 36.6, literal: "36.6" },
            Figure { label: "Rhodes State College", value: 23.5, literal: "23.5" },
        ],
    },
    Assertion {
        id: "in-state-tuition-here-runs-eightfold",
        statement: "A year of tuition and fees in Allen County costs between $4,560 and $35,498 \
                    depending which of its colleges you attend.",
        topic: "schools",
        supports: &[support!(
            "measure/allen-county-higher-education-2023.yml",
            "**In-state tuition and fees run from $4,560 to $35,498, an eightfold spread.** Rhodes State $4,560, Ohio State-Lima $8,937, Northwestern Ohio $11,352, Bluffton $35,498."
        )],
        answers: &[],
        figures: &[
            Figure { label: "Rhodes State College", value: 4_560.0, literal: "4,560" },
            Figure { label: "Ohio State-Lima", value: 8_937.0, literal: "8,937" },
            Figure { label: "Univ of Northwestern Ohio", value: 11_352.0, literal: "11,352" },
            Figure { label: "Bluffton University", value: 35_498.0, literal: "35,498" },
        ],
    },
    Assertion {
        id: "the-second-largest-field-of-study-is-not-a-local-one",
        statement: "Allen County's colleges award 2,918 credentials a year, of which 85.1 per \
                    cent are below a bachelor's degree. The largest field is health; the second \
                    is vehicle repair, taught at one institution that draws two thirds of its \
                    entering students from outside Ohio.",
        topic: "schools",
        supports: &[
            support!(
                "measure/allen-county-higher-education-2023.yml",
                "Six post-secondary institutions, 8,615 students in the autumn and 10,789 over the twelve months, and 2,918 credentials awarded in the academic year."
            ),
            support!(
                "measure/allen-county-higher-education-2023.yml",
                "2,482 of the 2,918 are below the baccalaureate, which is 85.1 per cent."
            ),
            support!(
                "measure/allen-county-higher-education-2023.yml",
                "**Every one of the 706 awards in vehicle repair is from one institution, and two thirds of its entering students come from outside Ohio.** Of 877 first-time students at the University of Northwestern Ohio in autumn 2023, 295 were Ohio residents and 582 were not — 101 from Michigan, 83 from Indiana, 48 from Pennsylvania, 31 from New York. At Rhodes State 432 of 436 were Ohio residents, and at Ohio State's Lima campus 255 of 263."
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-mayoral-line-reaches-1922",
        statement: "Lima had twenty-four mayors between 1842 and 1886, and the office it \
                    filled stopped existing on 1 January 1922.",
        topic: "government",
        supports: &[
            support!(
                "measure/lima-mayors-1842-1886.yml",
                "Every mayor of Lima from the town's organization to the printing of the county's first history — **twenty-four men over twenty-seven terms in forty-four years.**"
            ),
            support!(
                "measure/lima-mayors-1842-1886.yml",
                "**Three men served twice**, and none of the three consecutively: Thomas Delzell in 1846 and 1849, Thomas Milligan in 1850 and 1853, and Samuel A. Baxter in 1851 and 1854."
            ),
            support!(
                "measure/lima-mayors-1842-1886.yml",
                "**Two mayors resigned in office and a third resigned and was reappointed.**"
            ),
            support!(
                "measure/lima-mayors-1886-1922.yml",
                "**The line ends rather than continues.** F. A. Burkhardt was the last mayor of Lima under the\n  aldermanic form"
            ),
        ],
        answers: &["The corpus reads them as two and cannot show it"],
        figures: &[],
    },
    Assertion {
        id: "the-1906-list-lost-a-year",
        statement: "Three books print Lima's mayors from 1842. The 1906 list dropped one man and \
                    left a year with no mayor in it, which is how the corpus knows it dropped him.",
        topic: "government",
        supports: &[
            support!(
                "measure/lima-mayors-1842-1886.yml",
                "**The 1906 list has a year with nobody in it, and that is what settles it.**"
            ),
            support!(
                "measure/lima-mayors-1842-1886.yml",
                "leaving **April 1857 to April 1858 with no mayor at all**, the only such hole in\n  thirty-four entries"
            ),
            support!(
                "measure/lima-mayors-1842-1886.yml",
                "**The 1921 list lost one more, at the same place.**"
            ),
            support!(
                "measure/lima-mayors-1842-1886.yml",
                "**One name in this table is simply wrong, and the book it came from says so three times.**"
            ),
        ],
        answers: &["The corpus reads them as two and cannot show it"],
        figures: &[],
    },
    Assertion {
        id: "the-annexations-are-dated-now",
        statement: "Seven Allen County municipalities annexed 3,029.8 acres in fifty-seven \
                    dated acts between 1990 and 2024, and eight of Lima's fall in as many years.",
        topic: "geography",
        supports: &[
            support!(
                "measure/allen-county-annexations-1990-2024.yml",
                "**Fifty-seven annexations, all of them dated, none of them held by this corpus until now.**"
            ),
            support!(
                "measure/allen-county-annexations-1990-2024.yml",
                "Written out: **Delphos reported twenty-six annexations totalling 784.4 acres, Lima eleven totalling 622.0, Bluffton nine totalling 477.5, Elida five totalling 912.9, Spencerville four totalling 209.7, and Cairo and Harrod one each of 10.4 and 12.9 — fifty-seven records and 3,029.8 acres in thirty-five years.**"
            ),
            support!(
                "measure/allen-county-annexations-1990-2024.yml",
                "**The shape is a decade of nothing, then a burst, then nothing again.**"
            ),
            support!(
                "measure/allen-county-annexations-1990-2024.yml",
                "**Lima reported 601.0 acres of annexation effective inside the 2000s, and the Census Bureau's own measurement of Lima grew by 582.4 acres over the same decade.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Elida", value: 912.9, literal: "912.9" },
            Figure { label: "Delphos", value: 784.4, literal: "784.4" },
            Figure { label: "Lima", value: 622.0, literal: "622.0" },
            Figure { label: "Bluffton", value: 477.5, literal: "477.5" },
            Figure { label: "Spencerville", value: 209.7, literal: "209.7" },
            Figure { label: "Harrod", value: 12.9, literal: "12.9" },
            Figure { label: "Cairo", value: 10.4, literal: "10.4" },
        ],
    },
    Assertion {
        id: "the-county-lost-land-it-never-lost",
        statement: "Allen County's land area falls 1,235 acres between two censuses in which it \
                    gained ground, so no township's area change can be read.",
        topic: "geography",
        supports: &[
            support!(
                "measure/allen-county-land-area-2000-2024.yml",
                "**Allen County's land area falls from 404.43 to 402.50 square miles between the two volumes**, and its total area does not move — 406.88 against 406.85."
            ),
            support!(
                "measure/allen-county-land-area-2000-2024.yml",
                "**So eleven of the twelve townships lost ground and none of them lost ground.**"
            ),
            support!(
                "measure/allen-county-land-area-2000-2024.yml",
                "**Only Lima grows**, by 499 acres of land and 582 of total area, against 601 acres of annexation it reported in the same decade."
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-county-is-settled-from-outside-itself",
        statement: "Allen County's first settlement, its first post office and the fort a township \
                    is named for all stand nine hundred and ninety feet outside the county.",
        topic: "history",
        supports: &[
            support!(
                "event/settlers-occupy-fort-amanda-1817.yml",
                "**The first American settlement in Allen County was three men moving into an abandoned army post.**"
            ),
            support!(
                "site/fort-amanda.yml",
                "**It is not in Allen County and this corpus is the first thing here to say so.**"
            ),
            support!(
                "site/fort-amanda.yml",
                "**It misses by a fifth of a mile.** Walking the same meridian north from the fort until the county's map begins to answer puts the line at latitude 40.685776, which is 0.188 statute miles — about 990 feet — north of the fort."
            ),
            support!(
                "site/fort-amanda.yml",
                "**It was in Allen County until 1848**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "sixteen-of-twenty-nine-are-one-days-work",
        statement: "Allen County has twenty-nine properties on the National Register, sixteen of \
                    them signed on one day in 1982, and nothing it made anything in.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-national-register.yml",
                "**Twenty-nine listings, and seventeen of them are one year.**"
            ),
            support!(
                "measure/allen-county-national-register.yml",
                "Written out: **twenty-nine listings between 15 October 1966 and 27 October 2004 — twenty-six buildings, two structures and one historic district — of which sixteen were signed on a single day, 7 October 1982, under the Lima Multiple Resource Area submission.**"
            ),
            support!(
                "measure/allen-county-national-register.yml",
                "**Twenty of the twenty-nine stand in one square mile.**"
            ),
            support!(
                "measure/allen-county-national-register.yml",
                "**Allen County has a single National Historic Landmark and it is the Deep Cut**"
            ),
        ],
        answers: &["The dataset carries `STATUS: Listed` on all twenty-nine and would simply omit a delisted property, so it cannot answer its own question"],
        figures: &[],
    },
    Assertion {
        id: "the-two-war-lists-do-not-join",
        statement: "Allen County counted its World War soldiers twice and got different \
                    answers, and the man its Legion post is named for is in only one of them.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-world-war-service.yml",
                "**The narrative says about thirty-five hundred and the roster twenty pages later holds more than four thousand names.**"
            ),
            support!(
                "measure/allen-county-world-war-service.yml",
                "Written out: **the American Legion's figure is 3,260 enlistments, rising to about 3,500 with officers and men credited elsewhere; the printed roster yields at least 4,158 names.**"
            ),
            support!(
                "measure/allen-county-world-war-dead.yml",
                "**At most forty-seven of the ninety-five dead are absent from the roster of the served**, matched on surname and first initial."
            ),
            support!(
                "measure/allen-county-world-war-dead.yml",
                "**One of the absent is the man the post is named for.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Legion enlistments", value: 3_260.0, literal: "3,260" },
            Figure { label: "With officers and men credited elsewhere", value: 3_500.0, literal: "3,500" },
            Figure { label: "Names in the printed roster", value: 4_158.0, literal: "4,158" },
        ],
    },
    Assertion {
        id: "the-page-adds-up-and-the-forecast-does-not",
        statement: "The 1906 county history totals its own railroad columns to the unit, and the \
                    forecast on the same page does not follow from its own figures.",
        topic: "history",
        supports: &[
            support!(
                "measure/lima-railroad-traffic-1905.yml",
                "The steam figures sum to **49**, the electric to **28**, and the two to **77**."
            ),
            support!(
                "measure/lima-railroad-traffic-1905.yml",
                "That is 73 added to 77, or **150**; the book says **145**."
            ),
            support!(
                "measure/lima-railroad-traffic-1905.yml",
                "**six of the fourteen numbers in that block are products of the other\n  eight**"
            ),
        ],
        answers: &[
            "Six are arithmetic, one does not follow, and one settles an argument the book has with itself",
        ],
        figures: &[
            Figure { label: "Steam trains landing daily, 1905", value: 49.0, literal: "49" },
            Figure { label: "Electric trains landing daily, 1905", value: 28.0, literal: "28" },
            Figure { label: "Total, as the book prints it", value: 77.0, literal: "77" },
            Figure { label: "Forecast for 1906, as the book prints it", value: 145.0, literal: "145" },
            Figure { label: "Forecast implied by its own additions", value: 150.0, literal: "150" },
        ],
    },
    Assertion {
        id: "not-one-state-bridge-is-in-poor-condition",
        statement: "In the 2025 edition of the federal inventory, Allen County's 364 highway \
                    bridges are in poor condition or not according to who owns them, and the \
                    state's hundred and three are all sound.",
        topic: "geography",
        supports: &[
            support!(
                "measure/allen-county-bridges-2025.yml",
                "Written out: **not one of the state's 103 bridges here is\n  in poor condition, 29 of the county's 242 are, and 4 of the municipalities' 19 — 0.0, 12.0 and\n  21.1 per cent.**"
            ),
            support!(
                "measure/allen-county-bridges-2025.yml",
                "**No township owns a bridge, and that is the law rather than an accident.**"
            ),
            support!(
                "measure/allen-county-bridges-2025.yml",
                "**The county's own count is 371 and this one is 364, and neither is wrong.**"
            ),
        ],
        answers: &[
            "does not know why the state replaced its bad bridges and the county did not",
        ],
        figures: &[
            Figure { label: "State-owned, poor", value: 0.0, literal: "0.0" },
            Figure { label: "County-owned, poor", value: 12.0, literal: "12.0" },
            Figure { label: "Municipally owned, poor", value: 21.1, literal: "21.1" },
        ],
    },
    Assertion {
        id: "ten-declarations-eight-incidents",
        statement: "Allen County has been federally declared a disaster area ten times since 1965 \
                    for eight incidents, and three of the ten put money into households.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-disaster-declarations-1965-2020.yml",
                "**Ten declarations in fifty-five years, for eight distinct incidents.**"
            ),
            support!(
                "measure/allen-county-disaster-declarations-1965-2020.yml",
                "**Three of the ten brought money to households, and this node said two.**"
            ),
            support!(
                "measure/allen-county-disaster-declarations-1965-2020.yml",
                "**A county nine hundred miles from the Gulf has a hurricane declaration.**"
            ),
            support!(
                "measure/allen-county-disaster-declarations-1965-2020.yml",
                "**The longest incident in the file is 1,207 days.**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-gauge-was-not-there",
        statement: "Allen County has four federal declarations for flooding and a gauged river, \
                    and the gauge was switched off for three of them.",
        topic: "geography",
        supports: &[
            support!(
                "measure/ottawa-river-peak-flows-1924-2025.yml",
                "**That ten-year hole swallowed three of the county's four flood declarations.**"
            ),
            support!(
                "measure/ottawa-river-peak-flows-1924-2025.yml",
                "**And the largest measured flood in the modern record brought no federal declaration at all.**"
            ),
            support!(
                "measure/ottawa-river-peak-flows-1924-2025.yml",
                "**The two records overlap on one weather declaration in sixty years.**"
            ),
            support!(
                "measure/ottawa-river-peak-flows-1924-2025.yml",
                "**The Ottawa River has been gauged for seventy-nine water years and the record is in two halves\n  that cannot be joined.**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-city-drinks-from-the-river-nobody-measures",
        statement: "Lima's water system serves sixty-five per cent of Allen County out of six \
                    reservoirs and four river intakes, two of them on a river with no flow record.",
        topic: "geography",
        supports: &[
            support!(
                "measure/allen-county-water-systems-2026.yml",
                "**Lima's system serves more than the county seat by a factor of two.**"
            ),
            support!(
                "measure/allen-county-water-systems-2026.yml",
                "**Everything Lima drinks is surface water, and it is stored.**"
            ),
            support!(
                "measure/allen-county-water-systems-2026.yml",
                "**The city drinks from the river nobody measures.**"
            ),
            support!(
                "measure/allen-county-water-violations-1984-2026.yml",
                "**Twenty violations, ten resolutions.**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "a-step-that-is-not-a-crime-wave",
        statement: "The Allen County Sheriff's violent offenses nearly tripled in 2021 and his \
                    property offenses fell, and the month between them is the month he changed \
                    how he counts.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-sheriff-offenses-2015-2024.yml",
                "**The sheriff's violent offenses nearly tripled between 2020 and 2021, and his property offenses\n  fell.**"
            ),
            support!(
                "measure/allen-county-sheriff-offenses-2015-2024.yml",
                "**The line between those two periods is 1 December 2020, and it is in a different file.**"
            ),
            support!(
                "measure/allen-county-sheriff-offenses-2015-2024.yml",
                "**A change that triples one measure and lowers another at the same agency in the same month is a\n  change in counting.**"
            ),
            support!(
                "measure/allen-county-law-enforcement-agencies-2026.yml",
                "**A village of nineteen hundred people converted sixteen years before the county seat.**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-blizzard-is-not-in-the-weather-file",
        statement: "Allen County's severe weather record grew thirty-fold between the 1950s and \
                    the 2010s and its tornado count did not. Thirteen of its sixteen event types \
                    were born in 1996 or later, and the blizzard that brought the county a \
                    federal emergency in 1978 is not in the file at all.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-storm-events-1950-2026.yml",
                "**The count rises monotonically across seven decades and the tornadoes do not.**"
            ),
            support!(
                "measure/allen-county-storm-events-1950-2026.yml",
                "**The seven decade totals are 4, 8, 12, 32, 58, 95 and 121, and the tornado column beside them is\n  1, 2, 4, 3, 2, 3, 4.**"
            ),
            support!(
                "measure/allen-county-storm-events-1950-2026.yml",
                "**Sixteen event types appear here and thirteen of them start in 1996 or later.**"
            ),
            support!(
                "measure/allen-county-storm-events-1950-2026.yml",
                "**Eleven people have been killed by weather in this county since 1950 and all eleven died in the\n  same minute.**"
            ),
            support!(
                "measure/allen-county-disaster-declarations-1965-2020.yml",
                "**The sixth is the blizzard of January 1978 and the weather file does not contain it.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1950s", value: 4.0, literal: "4" },
            Figure { label: "1960s", value: 8.0, literal: "8" },
            Figure { label: "1970s", value: 12.0, literal: "12" },
            Figure { label: "1980s", value: 32.0, literal: "32" },
            Figure { label: "1990s", value: 58.0, literal: "58" },
            Figure { label: "2000s", value: 95.0, literal: "95" },
            Figure { label: "2010s", value: 121.0, literal: "121" },
        ],
    },
    Assertion {
        id: "eighty-four-cents-in-the-dollar",
        statement: "Allen County's applicants have been paid $10.6 million of federal disaster \
                    money since 2005. Eighty-four cents in the dollar is the pandemic, \
                    eighty-three cents is one hospital, and the ledger is keyed to where an \
                    applicant is registered rather than to where the work was done.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-disaster-assistance-2005-2025.yml",
                "**$10,608,108 of federal money on 95 projects, and eighty-four cents of every dollar is the\n  pandemic.**"
            ),
            support!(
                "measure/allen-county-disaster-assistance-2005-2025.yml",
                "**The pandemic is 84.28 per cent of the money on 7.4 per cent of the projects.** Eighty-eight\n  weather projects across fifteen years came to $1,667,603, against $8,940,505 for seven pandemic\n  projects."
            ),
            support!(
                "measure/allen-county-disaster-assistance-2005-2025.yml",
                "**What a federal disaster buys in this county is cleanup.**"
            ),
            support!(
                "measure/allen-county-disaster-aid-applicants-2005-2025.yml",
                "**Twenty-eight applicants, and one hospital took 82.74 per cent of the money.**"
            ),
            support!(
                "measure/allen-county-disaster-aid-applicants-2005-2025.yml",
                "**A disaster is the one occasion on which nearly every government in this county files the same\n  form.**"
            ),
            support!(
                "organization/lima-memorial-health-system.yml",
                "**Having no corporate parent is why that money is in this county's ledger.**"
            ),
        ],
        answers: &[
            "the corpus cannot say what any of that $183 million was spent on in Lima",
            "the corpus does not know",
        ],
        figures: &[
            Figure { label: "Weather, 2005-2012", value: 1_667_603.0, literal: "1,667,603" },
            Figure { label: "Pandemic, 2020", value: 8_940_505.0, literal: "8,940,505" },
        ],
    },
    Assertion {
        id: "the-villages-have-birthdays-now",
        statement: "Eight of Allen County's nine municipal corporations now carry a founding \
                    date. Four of the four that were missing were in books this corpus had \
                    already read for other things; the ninth is bounded to a five-year window; \
                    and one township's blank belongs to the record rather than to the corpus.",
        topic: "geography",
        supports: &[
            support!(
                "measure/allen-county-municipal-incorporations-1842-1904.yml",
                "**Eight of the county's nine municipal corporations have a founding date, and every one of them\n  was in a book this corpus already held.**"
            ),
            support!(
                "measure/allen-county-municipal-incorporations-1842-1904.yml",
                "**\"Incorporated\" is three different legal acts here and the corpus's one field flattens them.**"
            ),
            support!(
                "measure/allen-county-municipal-incorporations-1842-1904.yml",
                "**The gap that is left is one village and it is bounded.**"
            ),
            support!(
                "jurisdiction/bath-township.yml",
                "**A second compiler looked thirty-six years later and reported the same absence.**"
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "twelve-boards-and-nine-ballots",
        statement: "Twelve boards of education govern school districts holding ground in Allen \
                    County, and only nine of them appear on any ballot here. The county's own \
                    roster of officials answers that question two ways, and this corpus had read \
                    only one of them.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-school-boards-2026.yml",
                "**Twelve boards of education, sixty seats, and only forty-five of them are on a ballot in this\n  county.**"
            ),
            support!(
                "measure/allen-county-school-boards-2026.yml",
                "**The county's own roster answers this question two ways and the two answers differ.**"
            ),
            support!(
                "measure/allen-county-school-boards-2026.yml",
                "**The neighbouring counties' rosters carry the same fifteen people under different names.**"
            ),
            support!(
                "measure/allen-county-elected-seats-2026.yml",
                "**This counts the ballot and not the register, and the roster answers both ways.**"
            ),
        ],
        answers: &["This does not establish that these seats go unfilled because nobody stands for them"],
        figures: &[],
    },
    Assertion {
        id: "the-register-dates-them-but-cannot-place-them",
        statement: "Every judge who sits in Allen County is an Ohio attorney admitted by \
                    examination, and the register that proves it cannot say where any of them \
                    lives. Its county is the county of a business address, and it holds a \
                    hundred-year roll of 334 people of whom 144 may still practise.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-attorney-register-2026.yml",
                "**Three hundred and thirty-four people have given the Supreme Court of Ohio an address of record\n  in Allen County, and one hundred and forty-four of them may practice law.**"
            ),
            support!(
                "measure/allen-county-attorney-register-2026.yml",
                "**Thirteen of the 334 are officers this corpus already names.**"
            ),
            support!(
                "measure/allen-county-attorney-register-2026.yml",
                "**The county in this register is the county of a business address.**"
            ),
            support!(
                "measure/allen-county-attorney-register-2026.yml",
                "**The eleven decade totals are 20, 23, 24, 23, 26, 44, 46, 42, 23, 41 and 21, and the active\n  column beside them is 0, 0, 0, 0, 2, 10, 26, 28, 21, 36 and 21.**"
            ),
            support!(
                "measure/allen-county-attorney-register-2026.yml",
                "**The count is a floor, and the county's own geography is what shows it.**"
            ),
        ],
        answers: &["This corpus cannot say from it where any of the thirteen lives."],
        figures: &[
            Figure { label: "1920s", value: 20.0, literal: "20" },
            Figure { label: "1930s", value: 23.0, literal: "23" },
            Figure { label: "1940s", value: 24.0, literal: "24" },
            Figure { label: "1950s", value: 23.0, literal: "23" },
            Figure { label: "1960s", value: 26.0, literal: "26" },
            Figure { label: "1970s", value: 44.0, literal: "44" },
            Figure { label: "1980s", value: 46.0, literal: "46" },
            Figure { label: "1990s", value: 42.0, literal: "42" },
            Figure { label: "2000s", value: 23.0, literal: "23" },
            Figure { label: "2010s", value: 41.0, literal: "41" },
            Figure { label: "2020s", value: 21.0, literal: "21" },
        ],
    },
    Assertion {
        id: "the-ninety-nine-year-hole-is-dotted",
        statement: "Lima's mayoralty was said to be unrecorded for ninety-nine years. A weekly \
                    newspaper published twelve miles away names four of its mayors between 1939 \
                    and 1956, and not one of the mentions is an election report — so the office \
                    gains four holders and no term dates at all.",
        topic: "government",
        supports: &[
            support!(
                "measure/lima-mayors-1939-1956.yml",
                "**Not one of those fourteen mentions is an election report.**"
            ),
            support!(
                "measure/lima-mayors-1939-1956.yml",
                "**What it does establish is that Lima had a mayor, and a council, and a charter.**"
            ),
            support!(
                "measure/lima-mayors-1939-1956.yml",
                "**One mayor died in office and the estate is the evidence.**"
            ),
            support!(
                "measure/bluffton-and-beaverdam-elections-1939-1959.yml",
                "**1945 — a write-in beat a man who had no opponent.**"
            ),
            support!(
                "measure/bluffton-and-beaverdam-elections-1939-1959.yml",
                "**The line does not close, and the paper is why.**"
            ),
        ],
        answers: &["This corpus cannot say who was mayor of Lima on any day but the fourteen printed above."],
        figures: &[],
    },
    Assertion {
        id: "a-grand-total-is-not-a-budget",
        statement: "Allen County's books total $374,841,547 and the county government runs on \
                    $39,567,583 of it. Nearly half the rest is tax the treasurer collects for \
                    other governments and hands straight on, and the sheriff's office alone costs \
                    half again as much as the county's other seven elected offices together.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-funds-2025.yml",
                "**The county's revenue report ends at $374,841,546.99 and the county government does not have\n  $374,841,546.99.**"
            ),
            support!(
                "measure/allen-county-funds-2025.yml",
                "**Nearly half of the grand total is money the county holds for somebody else.**"
            ),
            support!(
                "measure/allen-county-funds-2025.yml",
                "**Fifteen funds took in and paid out the same amount to the cent.**"
            ),
            support!(
                "measure/allen-county-general-fund-2025.yml",
                "**The county government runs on $39,567,582.57, and more than half of it is sales tax.**"
            ),
            support!(
                "measure/allen-county-general-fund-2025.yml",
                "**In order, the eight are 10,786,007, 3,304,189, 1,394,273, 713,258, 497,075, 475,391, 388,472\n  and 341,014.**"
            ),
            support!(
                "measure/allen-county-net-position-2023.yml",
                "**Liabilities rose $36,758,891 in a year in which the county borrowed nothing.**"
            ),
        ],
        answers: &["This corpus cannot say what any of them costs in total, because an office may also draw on funds outside this one, as the engineer does."],
        figures: &[
            Figure { label: "Sheriff", value: 10786007.0, literal: "10,786,007" },
            Figure { label: "Commissioners", value: 3304189.0, literal: "3,304,189" },
            Figure { label: "Prosecutor", value: 1394273.0, literal: "1,394,273" },
            Figure { label: "Clerk of Courts", value: 713258.0, literal: "713,258" },
            Figure { label: "Auditor", value: 497075.0, literal: "497,075" },
            Figure { label: "Coroner", value: 475391.0, literal: "475,391" },
            Figure { label: "Recorder", value: 388472.0, literal: "388,472" },
            Figure { label: "Treasurer", value: 341014.0, literal: "341,014" },
        ],
    },
    Assertion {
        id: "a-county-column-is-a-filing-decision",
        statement: "The federal gazetteer files 109 named features in Allen County and 120 of \
                    them stand on Allen County ground. Every one of the eleven the column loses \
                    is a stream that rises here and is filed where it ends — the Ottawa River \
                    among them, which runs through the middle of Lima and is recorded under \
                    Putnam County.",
        topic: "geography",
        supports: &[
            support!(
                "measure/allen-county-named-features-2026.yml",
                "**The federal gazetteer files 109 named features in Allen County and 120 of them stand on Allen\n  County ground.**"
            ),
            support!(
                "measure/allen-county-named-features-2026.yml",
                "**By coordinate the nine classes run 38 populated places, 33 streams, 23 civil divisions, 13\n  reservoirs, 7 canals, 3 census areas, 1 lake, 1 channel and 1 military installation.**"
            ),
            support!(
                "measure/allen-county-named-features-2026.yml",
                "**Every one of the eleven the column misses is a stream that rises here and ends somewhere\n  else.**"
            ),
            support!(
                "measure/allen-county-named-features-2026.yml",
                "**The twelfth is the mirror case and it breaks the rule the catalog states.**"
            ),
            support!(
                "measure/allen-county-named-features-2026.yml",
                "**Three names in the county are used twice.**"
            ),
            support!(
                "measure/allen-county-named-features-2026.yml",
                "**The gazetteer and the city do not agree about Lima's reservoirs.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Populated Place", value: 38.0, literal: "38" },
            Figure { label: "Stream", value: 33.0, literal: "33" },
            Figure { label: "Civil", value: 23.0, literal: "23" },
            Figure { label: "Reservoir", value: 13.0, literal: "13" },
            Figure { label: "Canal", value: 7.0, literal: "7" },
            Figure { label: "Census", value: 3.0, literal: "3" },
            Figure { label: "Lake", value: 1.0, literal: "1" },
            Figure { label: "Channel", value: 1.0, literal: "1" },
            Figure { label: "Military", value: 1.0, literal: "1" },
        ],
    },
    Assertion {
        id: "the-third-war-is-counted-by-its-dead",
        statement: "One hundred and seventy-one men from Allen County died or were still missing \
                    in the Army between May 1941 and January 1946 — one for every 429 people the \
                    county held in 1940, and a third of them died away from combat. It is the \
                    county's third named war and the first the corpus counts by its dead rather \
                    than by its service.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-world-war-ii-dead-1941-1946.yml",
                "**One hundred and seventy-one men from Allen County died or were still missing in the Army of the\n  United States between 27 May 1941 and 31 January 1946.**"
            ),
            support!(
                "measure/allen-county-world-war-ii-dead-1941-1946.yml",
                "**One in three of them did not die in battle.**"
            ),
            support!(
                "measure/allen-county-world-war-ii-dead-1941-1946.yml",
                "**The six categories run 87 killed in action, 13 dead of wounds, 0 dead of injuries, 56 dead\n  non-battle, 14 declared dead under Public Law 490 and 1 still missing.**"
            ),
            support!(
                "measure/allen-county-world-war-ii-dead-1941-1946.yml",
                "**Against the county's 73,303 people in 1940 that is one death for every 429.**"
            ),
            support!(
                "measure/allen-county-world-war-ii-dead-1941-1946.yml",
                "**The county here is the county of an address given at enlistment, not of a home and not of a\n  grave.**"
            ),
            support!(
                "period/the-second-world-war-in-allen-county.yml",
                "**The county's war began before the country's, and two independent sources say so.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Killed in action", value: 87.0, literal: "87" },
            Figure { label: "Died, non-battle", value: 56.0, literal: "56" },
            Figure { label: "Finding of death", value: 14.0, literal: "14" },
            Figure { label: "Died of wounds", value: 13.0, literal: "13" },
            Figure { label: "Missing", value: 1.0, literal: "1" },
            Figure { label: "Died of injuries", value: 0.0, literal: "0" },
        ],
    },
    Assertion {
        id: "ninety-nine-papers-and-one-of-them-online",
        statement: "Ninety-nine newspapers have been printed in Allen County and one of them is \
                    digitized. Forty-three have no library reporting a copy. This site read the \
                    one for six phases and called the other ninety-eight the county's press.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-newspapers-1843-2026.yml",
                "**Ninety-nine bibliographic records carry Allen County and ninety-seven of them were printed in\n  it.** By place of publication that is 66 in Lima, 18 in Delphos, 6 in Spencerville, 5 in Bluffton,\n  1 in Elida and 1 in Lafayette."
            ),
            support!(
                "measure/allen-county-newspapers-1843-2026.yml",
                "**One of the ninety-nine is digitized.**"
            ),
            support!(
                "measure/allen-county-newspapers-1843-2026.yml",
                "**Forty-three of the ninety-nine have no library reporting a copy.**"
            ),
            support!(
                "measure/allen-county-newspapers-1843-2026.yml",
                "**The first was a weekly called The Porcupine and it lasted seven months.**"
            ),
            support!(
                "measure/allen-county-newspapers-1843-2026.yml",
                "**Most of them are older than the century.**"
            ),
            support!(
                "organization/the-lima-news.yml",
                "**Nine titles carry one unbroken issue count from 27 October 1884 to now.**"
            ),
            support!(
                "event/the-lima-news-strike-of-1957.yml",
                "**The strikers printed their own paper on the day the presses stopped.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Lima", value: 66.0, literal: "66" },
            Figure { label: "Delphos", value: 18.0, literal: "18" },
            Figure { label: "Spencerville", value: 6.0, literal: "6" },
            Figure { label: "Bluffton", value: 5.0, literal: "5" },
            Figure { label: "Elida", value: 1.0, literal: "1" },
            Figure { label: "Lafayette", value: 1.0, literal: "1" },
        ],
    },
    Assertion {
        id: "three-per-cent-of-the-library-money-is-local",
        statement: "Three per cent of what Allen County's libraries live on is local government \
                    money and seventy-one per cent is the state's — the inverse of the American \
                    average. That is Ohio's funding structure, not this county's choice, and the \
                    same dollar is a conduit fund in the county auditor's books.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-libraries-2024.yml",
                "**Three per cent of what these libraries spend is local government money and seventy-one per cent\n  is the state's.** Income of $6,528,885 across the three: $194,728 local, $4,622,683 state, $417\n  federal and $1,711,057 from everything else."
            ),
            support!(
                "measure/allen-county-libraries-2024.yml",
                "**That is Ohio, not Allen County.** Ohio's 251 systems take 47.7 per cent of income locally and\n  42.9 per cent from the state, and among the state's 88 counties Allen ranks sixteenth from the\n  bottom on the local share."
            ),
            support!(
                "measure/allen-county-libraries-2024.yml",
                "**The county seat's library reports no local government revenue whatsoever.**"
            ),
            support!(
                "measure/allen-county-libraries-2024.yml",
                "**Every resident of this county is inside exactly one library's legal service area.**"
            ),
            support!(
                "measure/allen-county-libraries-2024.yml",
                "**Eight buildings, 101,109 square feet, and two of the nine municipalities have none.**"
            ),
            support!(
                "measure/allen-county-funds-2025.yml",
                "**One of those fifteen conduits is what pays for the county's libraries.** \"Local government\n  library and township parks\", $4,495,636.25 in and $4,495,636.25 out, is this county's share of\n  Ohio's public library fund"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "State", value: 4622683.0, literal: "4,622,683" },
            Figure { label: "Other", value: 1711057.0, literal: "1,711,057" },
            Figure { label: "Local", value: 194728.0, literal: "194,728" },
            Figure { label: "Federal", value: 417.0, literal: "417" },
        ],
    },
    Assertion {
        id: "a-hundred-and-sixty-congregations",
        statement: "A hundred and sixty congregations of forty-nine religious bodies report 58,696 \
                    adherents in Allen County — 57.4 per cent of it. A survey a century earlier put \
                    church-going at a quarter. The two figures answer different questions and this \
                    site does not draw a line between them.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-congregations-2020.yml",
                "A hundred and sixty congregations, forty-nine religious bodies, and 58,696 people reported as\n  adherents of one of them."
            ),
            support!(
                "measure/allen-county-congregations-2020.yml",
                "**That is 57.4 per cent of the county, and the denominator is the census count.**"
            ),
            support!(
                "measure/allen-county-congregations-2020.yml",
                "**Two bodies hold nearly half of it.**"
            ),
            support!(
                "measure/allen-county-congregations-2020.yml",
                "Reading the eight counties around this one: Mercer 77.6 per cent, Putnam 72.0\n  and Shelby 63.9 to the south and west; Auglaize 52.2 due south; and Hancock 41.2, Logan 38.7, Van\n  Wert 35.5 and Hardin 30.8 to the north and east. Allen at 57.4 sits between the two groups"
            ),
            support!(
                "measure/allen-county-churches-1906-1921.yml",
                "**A survey in about 1920 put the county's church-going at a quarter of its people and its\n  communicants at something like seventeen thousand.**"
            ),
            support!(
                "measure/allen-county-churches-1906-1921.yml",
                "**It is not comparable with the 2020 figure and the corpus does not trend the two.**"
            ),
            support!(
                "measure/allen-county-churches-1906-1921.yml",
                "**Lima's synagogue was built in 1914.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Mercer", value: 77.6, literal: "77.6" },
            Figure { label: "Putnam", value: 72.0, literal: "72.0" },
            Figure { label: "Shelby", value: 63.9, literal: "63.9" },
            Figure { label: "Allen", value: 57.4, literal: "57.4" },
            Figure { label: "Auglaize", value: 52.2, literal: "52.2" },
            Figure { label: "Hancock", value: 41.2, literal: "41.2" },
            Figure { label: "Logan", value: 38.7, literal: "38.7" },
            Figure { label: "Van Wert", value: 35.5, literal: "35.5" },
            Figure { label: "Hardin", value: 30.8, literal: "30.8" },
        ],
    },
    Assertion {
        id: "the-houses-went-where-the-city-is-not",
        statement: "Allen County authorized 5,437 new houses in thirty-six years and Lima, a third \
                    of the county, got 367 of them. Half went to the three townships that wrap \
                    around the city. The county lost 7,549 people over the same span and Lima lost \
                    9,970, so the ground outside it gained.",
        topic: "geography",
        supports: &[
            support!(
                "measure/allen-county-new-houses-by-place-1990-2025.yml",
                "**Lima holds a third of the county's people and got 367 of its 5,437 new houses — 6.8 per cent.**"
            ),
            support!(
                "measure/allen-county-new-houses-by-place-1990-2025.yml",
                "**Half of them went to the three townships that wrap around it.** American Township 1,098,\n  Shawnee Township 981 and Bath Township 643 — 2,722 between them, 50.1 per cent of the county's\n  new houses on ground that touches the city line."
            ),
            support!(
                "measure/allen-county-new-houses-by-place-1990-2025.yml",
                "**A new house in Shawnee Township was reported at $211,559 and one in Lima at $95,879.**"
            ),
            support!(
                "measure/allen-county-new-houses-by-place-1990-2025.yml",
                "**The county did not shrink. Lima did, and the ground around it grew.** Between 1990 and 2020 the\n  county lost 7,549 people and Lima lost 9,970, which means everywhere else gained 2,421."
            ),
            support!(
                "measure/allen-county-building-permits-1990-2025.yml",
                "**The county authorized 2,252 houses in the 1990s and 587 in the 2010s.**"
            ),
            support!(
                "measure/allen-county-building-permits-1990-2025.yml",
                "**The peak is 2004 at 331 houses and the floor is 2011 at 35 — a fall of 89.4 per cent in seven\n  years.**"
            ),
            support!(
                "measure/allen-county-building-permits-1990-2025.yml",
                "**A permit is not a house and this node never treats it as one.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "American Twp", value: 1098.0, literal: "1,098" },
            Figure { label: "Shawnee Twp", value: 981.0, literal: "981" },
            Figure { label: "Bath Twp", value: 643.0, literal: "643" },
            Figure { label: "Lima", value: 367.0, literal: "367" },
        ],
    },
    Assertion {
        id: "a-line-of-holders-for-every-office",
        statement: "This site knew the county's sheriffs back to 1831 and, for every other county \
                    office, only who holds it today. A roster printed in 1906 supplies 169 names \
                    across nine offices — and corroborates the sheriffs, name for name, from a \
                    book and a website 120 years apart.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-officers-1831-1906.yml",
                "The county's roster of every elected officer from its organization to the year the book was\n  printed — nine offices, seventy-five years, and until now the corpus held one holder of each."
            ),
            support!(
                "measure/allen-county-officers-1831-1906.yml",
                "**The sheriffs are the check on all of it, and the book loses two years to the printer.** The\n  1906 roster and\n  [the sheriff's own modern roster](../../catalog/acso-past-sheriffs.md) name the same twenty-one\n  men in the same order from 1831 to 1905."
            ),
            support!(
                "measure/allen-county-officers-1831-1906.yml",
                "**The county went thirteen years without a coroner in the record.**"
            ),
            support!(
                "measure/allen-county-officers-1831-1906.yml",
                "**The office now called County Engineer was the County Surveyor for its first seventy-three\n  years.**"
            ),
            support!(
                "measure/allen-county-officers-1831-1906.yml",
                "**The 1921 book contradicts its own roster forty pages later.**"
            ),
            support!(
                "measure/allen-county-officers-1831-1906.yml",
                "**In order, the nine counts are 27 treasurers, 22 prosecuting attorneys, 21 sheriffs, 20\n  recorders, 20 coroners, 19 auditors, 14 surveyors, 14 clerks of the common pleas and 12 probate\n  judges — 169 named holdings in seventy-five years.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Treasurer", value: 27.0, literal: "27" },
            Figure { label: "Prosecutor", value: 22.0, literal: "22" },
            Figure { label: "Sheriff", value: 21.0, literal: "21" },
            Figure { label: "Recorder", value: 20.0, literal: "20" },
            Figure { label: "Coroner", value: 20.0, literal: "20" },
            Figure { label: "Auditor", value: 19.0, literal: "19" },
            Figure { label: "Surveyor", value: 14.0, literal: "14" },
            Figure { label: "Clerk", value: 14.0, literal: "14" },
            Figure { label: "Probate Judge", value: 12.0, literal: "12" },
        ],
    },
    Assertion {
        id: "three-times-a-mob-came-to-the-jail",
        statement: "A crowd has come to the Allen County jail three times in sixty-one years and \
                    this site held only the last. Fifty men guarded it in 1872, a mob nearly hanged \
                    the sheriff in 1916 for hiding a Black prisoner, and a sheriff was shot dead in \
                    it in 1933.",
        topic: "history",
        supports: &[
            support!(
                "event/the-mob-at-the-allen-county-jail-1916.yml",
                "On 30 August 1916 a mob came to the county jail in Lima to take a Black prisoner, and when the\n  sheriff would not say where he had hidden him they put a rope round the sheriff's neck."
            ),
            support!(
                "event/the-mob-at-the-allen-county-jail-1916.yml",
                "**It is the second of three times a mob has come to this jail and the corpus held only the\n  third.**"
            ),
            support!(
                "event/the-execution-of-andrew-brentlinger-1872.yml",
                "**Fifty men guarded the jail the night before.**"
            ),
            support!(
                "event/the-execution-of-andrew-brentlinger-1872.yml",
                "**Neither date the book offers was a Friday.** 7 April 1872 fell on a Sunday and 15 April on a\n  Monday; the Fridays that month were the 5th, 12th, 19th and 26th."
            ),
            support!(
                "measure/allen-county-black-community-1916-1921.yml",
                "**Two thousand five hundred people, and nearly all of them in Lima.**"
            ),
            support!(
                "measure/allen-county-black-community-1916-1921.yml",
                "**Two voter counts, four years apart, and the second one includes women.** \"In 1916, there were\n  552 negro voters in Lima; there has been an exodus of negroes to Allen County within the last four\n  years, and in 1920, counting the women, there were almost 1,000 negro voters.\""
            ),
            support!(
                "measure/allen-county-black-community-1916-1921.yml",
                "**On segregation the book contradicts itself in a page.**"
            ),
            support!(
                "measure/allen-county-jewish-community-1850-1921.yml",
                "**A dozen families in 1878 and about a hundred in 1921.**"
            ),
        ],
        answers: &[
            // The Brentlinger node refuses a day for the hanging. This assertion dates the
            // crowd at the jail to the year and to the night before, and to nothing finer.
            "cannot say which day it happened",
        ],
        figures: &[
            Figure { label: "1916", value: 552.0, literal: "552" },
            Figure { label: "1920", value: 1000.0, literal: "1,000" },
        ],
    },
    Assertion {
        id: "three-rosters-and-only-one-can-be-checked",
        statement: "The 1906 county history prints three rosters of Allen County's legislators in \
                    one chapter. Nineteen of the twenty entries in the congressional one survive an \
                    entry-by-entry check against the federal record; the other two give seventy \
                    names in order and not a single date.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-in-congress-1831-1933.yml",
                "**In figures: 20 entries, 18 men, 10 home counties, 3 of the 18 from Allen, and 1 entry that fails\n  the check against the federal record.**"
            ),
            support!(
                "measure/allen-county-in-congress-1831-1933.yml",
                "**Nineteen of the twenty entries survive the check and one does not.** Laid against the federal\n  record Congress by Congress, every name, district and span in the book is confirmed except\n  1891\u{2013}1896: the book files Fernando C. Layton of Auglaize under the Fourth for that whole span, and\n  Layton sat for the Fifth in the 52nd Congress and the Fourth in the 53rd and 54th."
            ),
            support!(
                "measure/allen-county-in-congress-1831-1933.yml",
                "**The county changed congressional districts four times in seventy-five years and never held the\n  same number twice running.**"
            ),
            support!(
                "measure/allen-county-general-assembly-1833-1906.yml",
                "**In figures: 35 representatives against at least 39 General Assemblies, and 35 senate entries \u{2014}\n  32 people \u{2014} against at least 43.**"
            ),
            support!(
                "measure/allen-county-general-assembly-1833-1906.yml",
                "**Neither list carries a year, and the book's own datings show they cannot be read as one name per\n  assembly.**"
            ),
            support!(
                "measure/allen-county-general-assembly-1833-1906.yml",
                "**What that costs is not hypothetical.** This corpus already holds \"the Boesel railroad bill\",\n  passed by the Legislature in April 1872, which put $100,000 of bonds to a vote in Lima and Ottawa\n  Township and carried with eight votes against. A Charles Boesel stands sixteenth in this senate\n  roster."
            ),
            support!(
                "measure/allen-county-common-pleas-bench-1833-1920.yml",
                "**The bench's odd term dates are at least a hundred and eighteen years old.**"
            ),
        ],
        answers: &[
            // The General Assembly measure refuses to identify the Boesel of the roster with
            // the Boesel of the 1872 bond bill. That refusal is not routed around here: it is
            // the assertion. A list with no years in it cannot be joined to a dated event, and
            // saying so is the whole claim.
            "cannot say they are the same man",
        ],
        figures: &[
            Figure { label: "Representatives named", value: 35.0, literal: "35" },
            Figure { label: "Assemblies since the 36th", value: 39.0, literal: "39" },
            Figure { label: "Senate entries", value: 35.0, literal: "35" },
            Figure { label: "Assemblies since the 32nd", value: 43.0, literal: "43" },
        ],
    },
    Assertion {
        id: "the-board-is-printed-as-a-shift-register",
        statement: "The 1906 county history prints the Allen County board once a year for \
                    seventy-five years, and each row is the row before it moved a place right. \
                    Fifty-eight of the sixty-three transitions after 1842 obey it exactly, and \
                    every one that does not is either an event the book never mentions or a hole \
                    in the printing.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-commissioners-1831-1920.yml",
                "**The 1906 book prints the board at the end of every year from 1831 to 1905 \u{2014} seventy-five rows,\n  three names each, and fifty-three men.**"
            ),
            support!(
                "measure/allen-county-commissioners-1831-1920.yml",
                "**Every row is the row before it, shifted one place right.** The new commissioner is written\n  first, the previous first name second, the previous second name third \u{2014} and the man in third place\n  is the one who goes. Of the sixty-three transitions from 1842 to 1905, fifty-eight are exactly\n  that and five are not."
            ),
            support!(
                "measure/allen-county-commissioners-1831-1920.yml",
                "**In figures: 75 rows and 74 transitions between them \u{2014} 50 in which a new commissioner arrives and\n  is written first, 2 in which a new man arrives and is not, and 22 in which nobody arrives and the\n  order rotates anyway.**"
            ),
            support!(
                "measure/allen-county-commissioners-1831-1920.yml",
                "**The rule holds even in the twenty-two years when nobody new arrives.** In every one of those the\n  three men are the same three and the order still turns over: the third name becomes the first,\n  twenty-two times out of twenty-two, without a single exception in seventy-five years."
            ),
            support!(
                "measure/allen-county-commissioners-1831-1920.yml",
                "**Both books explain that column and neither explains all of it.**"
            ),
            support!(
                "measure/allen-county-commissioners-1831-1920.yml",
                "**The book skips 1889 and says nothing about it.** It runs 1888 to 1890 with no row between, and\n  the hole manufactures two false careers: John Akerman and William Bice both appear to leave the\n  board and return, and neither did."
            ),
            support!(
                "tenure/commissioner-1884-alexander-shenk.yml",
                "**It is a node because the book gives a reason and not only a date**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "New man, written first", value: 50.0, literal: "50" },
            Figure { label: "New man, written second", value: 2.0, literal: "2" },
            Figure { label: "No new man; order rotates", value: 22.0, literal: "22" },
        ],
    },
    Assertion {
        id: "a-fifth-of-the-farmland-went-in-four-years",
        statement: "Allen County lost 61,551 acres of farmland between 1910 and 2022, and 12,926 \
                    of them went in the four years from 1950 to 1954 \u{2014} a fifth of the century's \
                    loss, at ten times the rate of the forty years before it.",
        topic: "geography",
        supports: &[
            support!(
                "measure/allen-county-farms-1949-1987.yml",
                "**The county's farmland fell 61,551 acres between 1910 and 2022, and a fifth of that went in four\n  years.** Between 1950 and 1954 it lost 12,926 acres \u{2014} 21.0 per cent of the whole century's loss,\n  at 3,232 acres a year, against 313 a year for the forty years before it."
            ),
            support!(
                "measure/allen-county-farms-1949-1987.yml",
                "**In figures: 12,528 acres went in the 40 years to 1950, 12,926 in the 4 years to 1954, 17,762 in\n  the 28 years to 1982 and 18,335 in the 40 since.**"
            ),
            support!(
                "measure/allen-county-farms-1949-1987.yml",
                "**It is not a local event and it is probably not a definition.** Ohio lost 4.66 per cent of its\n  land in farms over the same four years and Allen County 5.67; Ohio lost 11.2 per cent of its farms\n  and Allen 18.1."
            ),
            support!(
                "measure/allen-county-farms-1949-1987.yml",
                "**Between 1954 and 1982 the county lost farmland and grew more crops on what was left.** Land in\n  farms fell 17,762 acres; harvested cropland rose 16,749."
            ),
            support!(
                "measure/allen-county-farms-1949-1987.yml",
                "**The only rise in the whole series is the 1980s farm crisis.** Land in farms went *up* 3,934\n  acres between 1982 and 1987 while the value of an acre fell 25.7 per cent, from $2,023 to $1,504,\n  and the value of a farm 17.0 per cent."
            ),
            support!(
                "measure/allen-county-farms-1949-1987.yml",
                "**Soybeans passed oats between 1949 and 1954.** In 1949 the county harvested 26,213 acres of oats\n  and 17,393 of soybeans for beans; in 1954, 22,498 of oats and 22,991 of soybeans."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1910\u{2013}1950, 40 years", value: 12528.0, literal: "12,528" },
            Figure { label: "1950\u{2013}1954, 4 years", value: 12926.0, literal: "12,926" },
            Figure { label: "1954\u{2013}1982, 28 years", value: 17762.0, literal: "17,762" },
            Figure { label: "1982\u{2013}2022, 40 years", value: 18335.0, literal: "18,335" },
        ],
    },
    Assertion {
        id: "the-census-counted-what-the-history-estimated",
        statement: "The 1921 county history estimated 2,500 Black residents in Allen County. The \
                    census taken the year before counted 1,385, and 1,243 of those were in Lima. \
                    The same book said nobody had counted the county's foreign-born; the census \
                    had counted 2,753 of them, from twenty-one countries.",
        topic: "population",
        supports: &[
            support!(
                "measure/allen-county-population-by-race-1920.yml",
                "**The 1921 history's estimate of the county's Black population is almost twice the census\n  count.** It says \"it is estimated that there are 2,500 negroes in Allen County\" and that they are\n  \"nearly all in Lima\". The census counted 1,385 in the county and 1,243 in Lima."
            ),
            support!(
                "measure/allen-county-population-by-race-1920.yml",
                "**In figures: the book estimated 2500 for the county, the census counted 1385 there and 1243 in\n  Lima, and the book's thousand voters would be 80.5 per cent of that 1243.**"
            ),
            support!(
                "measure/allen-county-population-by-race-1920.yml",
                "**Lima held nine-tenths of the county's Black residents and a third of its white ones.** 1,243 of\n  1,385, or 89.7 per cent, against a 60.6 per cent share of the county's population."
            ),
            support!(
                "measure/allen-county-population-by-race-1920.yml",
                "**The county's total is twenty higher than the figure this corpus has been using.**"
            ),
            support!(
                "measure/allen-county-foreign-born-1920.yml",
                "**In figures, the nine largest: Germany 627, Italy 312, Ireland 309, Canada 174, Switzerland 171,\n  England 167, Austria 158, Russia 131 and Wales 120.**"
            ),
            support!(
                "measure/allen-county-foreign-born-1920.yml",
                "**One person in six in this county was an immigrant or an immigrant's child.** 2,753 foreign-born\n  white, 4,422 native white of foreign parentage and 4,099 of mixed parentage \u{2014} 11,274 of 68,223, or\n  16.5 per cent. In Lima it was 7,490 of 41,326, or 18.1 per cent."
            ),
            support!(
                "measure/allen-county-foreign-born-1920.yml",
                "**The county's own historical society had no count of this and said so.**"
            ),
        ],
        answers: &[
            // Both refusals are gone from the cited nodes: the four censuses between 1920 and 1960
            // have been read, so neither node refuses a trend any more, and an answer to a refusal
            // nobody makes is withdrawn with it. See `the-migration-arrived-in-the-forties`.
        ],
        figures: &[
            Figure { label: "Germany", value: 627.0, literal: "627" },
            Figure { label: "Italy", value: 312.0, literal: "312" },
            Figure { label: "Ireland", value: 309.0, literal: "309" },
            Figure { label: "Canada", value: 174.0, literal: "174" },
            Figure { label: "Switzerland", value: 171.0, literal: "171" },
            Figure { label: "England", value: 167.0, literal: "167" },
            Figure { label: "Austria", value: 158.0, literal: "158" },
            Figure { label: "Russia", value: 131.0, literal: "131" },
            Figure { label: "Wales", value: 120.0, literal: "120" },
        ],
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
    Assertion {
        id: "the-migration-arrived-in-the-forties",
        statement: "Allen County's Black population grew by 531 people in the twenty years after \
                    1920 and by 2,475 in the ten years after that. The Great Migration of 1916 to \
                    1930 went past this county; the one that followed the war did not. Its \
                    foreign-born population moved the other way over the same forty years, falling \
                    by 46 per cent while the county grew by 29.",
        topic: "population",
        supports: &[
            support!(
                "measure/allen-county-population-by-race-1930-1960.yml",
                "**The county's Black population grew by 531 people in the twenty years to 1940 and by 2,475 in\n  the ten years after.** 1,385 in 1920, 1,686 in 1930, 1,916 in 1940, then 4,391 in 1950 and 7,322\n  in 1960."
            ),
            support!(
                "measure/allen-county-population-by-race-1930-1960.yml",
                "**In figures: 1385 in 1920, 1686 in 1930, 1916 in 1940, 4391 in 1950 and 7322 in 1960.**"
            ),
            support!(
                "measure/allen-county-population-by-race-1930-1960.yml",
                "**The migration that reached most northern industrial cities between 1916 and 1930 did not reach\n  this one.** The decades in which Detroit, Cleveland and Youngstown were transformed added 301\n  people and then 230 to Allen County."
            ),
            support!(
                "measure/allen-county-population-by-race-1930-1960.yml",
                "**Lima's share of the county's Black residents fell across every decade this node can measure.**\n  1,243 of 1,385 in 1920, or 89.7 per cent; 1,561 of 1,916 in 1940, or 81.5; 3,278 of 4,391 in\n  1950, or 74.7."
            ),
            support!(
                "measure/allen-county-population-by-race-1930-1960.yml",
                "**In 1940 three of every four Black residents of the county outside Lima were in Bath Township.**\n  The county held 1,916 and Lima 1,561, leaving 355; Bath Township held 260 of them, in a rural\n  township of 3,438 people."
            ),
            support!(
                "measure/allen-county-foreign-born-1930-1950.yml",
                "**The county's foreign-born population fell by 46 per cent in thirty years while the county grew\n  by 29.** 2,753 down to 1,485, against 68,223 up to 88,183."
            ),
            support!(
                "measure/allen-county-foreign-born-1930-1950.yml",
                "**Italy has passed Germany.** 246 against 222, where in 1920 it was Germany 627 against Italy 312."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1920", value: 1_385.0, literal: "1385" },
            Figure { label: "1930", value: 1_686.0, literal: "1686" },
            Figure { label: "1940", value: 1_916.0, literal: "1916" },
            Figure { label: "1950", value: 4_391.0, literal: "4391" },
            Figure { label: "1960", value: 7_322.0, literal: "7322" },
        ],
    },
    Assertion {
        id: "the-workforce-stopped-growing-in-1914",
        statement: "Lima's factory workforce grew by 146 per cent in the fifteen years to 1914 and \
                    by twenty-five people in the five years after it, and was smaller again by \
                    1929. The county's first manufacturing measurement, taken that year, is 6,588 \
                    wage earners — fewer than the county recorded in 2022.",
        topic: "population",
        supports: &[
            support!(
                "measure/lima-manufactures-1914-1929.yml",
                "**Lima's factory workforce grew by 146 per cent in the fifteen years to 1914 and by twenty-five\n  people in the five years after it.** 1,980 wage earners in 1899, 4,876 in 1914, 4,901 in 1919."
            ),
            support!(
                "measure/lima-manufactures-1914-1929.yml",
                "**In figures: 1980 in 1899, 2733 in 1904, 3607 in 1909, 4876 in 1914, 4901 in 1919 and 4548 in\n  1929.**"
            ),
            support!(
                "measure/lima-manufactures-1914-1929.yml",
                "**A third of the city's establishments appear to vanish in the 1920s and about half of that is a\n  definition.** 108 in 1919 against 72 in 1929."
            ),
            support!(
                "measure/lima-manufactures-1914-1929.yml",
                "**Two out of five establishments were corporations and they held nine in ten of the jobs.** In\n  1919, 48 of Lima's 108 establishments were owned by corporations, and those 48 employed 4,432 of\n  the 4,901 wage earners and made $21,817,237 of the $23,638,764."
            ),
            support!(
                "measure/allen-county-manufactures-1929.yml",
                "**Allen County had 108 manufacturing establishments in 1929, employing 6,588 wage earners, and\n  Lima held two thirds of the first and 69 per cent of the second.**"
            ),
            support!(
                "measure/allen-county-manufactures-1929.yml",
                "**The county was less of a factory county in 1929 than it was forty years later.** 6,588 wage\n  earners in a county of 69,419 in 1929; 17,623 manufacturing employees in a county of 111,144 in\n  1969."
            ),
            support!(
                "measure/allen-county-manufactures-1929.yml",
                "**The third of the county's industry that stood outside Lima paid better per head and added\n  less.** The 2,040 wage earners outside the city line averaged $1,309 in wages against $1,246\n  inside it, and produced $2,526 of value added each against $2,944."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1899", value: 1_980.0, literal: "1980" },
            Figure { label: "1904", value: 2_733.0, literal: "2733" },
            Figure { label: "1909", value: 3_607.0, literal: "3607" },
            Figure { label: "1914", value: 4_876.0, literal: "4876" },
            Figure { label: "1919", value: 4_901.0, literal: "4901" },
            Figure { label: "1929", value: 4_548.0, literal: "4548" },
        ],
    },
    Assertion {
        id: "a-colonel-of-that-name-in-the-war-of-1812",
        statement: "The earliest printed account of Ohio's counties says Allen County was named \
                    for a colonel of the War of 1812, and cannot name him. Of the fourteen \
                    counties the 1820 act laid out, Allen and Wood are the only two Howe \
                    attributes to a man whose name he withholds \u{2014} and Ethan Allen, the \
                    other candidate usually offered, died in 1789.",
        topic: "history",
        supports: &[
            support!(
                "question/namesake-of-allen-county.yml",
                "**The earliest source held here makes the attribution, and it rules out one of the two\n  candidates.** Howe's *Historical Collections of Ohio*, 1847: \"Allen was formed April 1st, 1820,\n  from Indian territory, and named in honor of a colonel of that name in the war of 1812: it was\n  temporarily attached to Mercer county for judicial purposes.\""
            ),
            support!(
                "measure/ohio-counties-of-1820-and-their-names.yml",
                "**Eight of the ten persons Howe names in full, and two he identifies by rank and war alone.**\n  The two are Allen and Wood, and the description he gives each is the same: an officer of the war\n  of 1812."
            ),
            support!(
                "measure/ohio-counties-of-1820-and-their-names.yml",
                "**In figures: 14 counties, 11 attributions, 8 named in full and 2 given only a rank and a war.**"
            ),
            support!(
                "measure/ohio-counties-of-1820-and-their-names.yml",
                "**Three of the fourteen commemorate one event.** Paulding, Van Wert and Williams are the three\n  militiamen who took Major Andr\u{e9} in 1780, given a county each and adjoining ground in the\n  north-west of the state."
            ),
            support!(
                "question/namesake-of-allen-county.yml",
                "**The Ethan Allen attribution has a source, a date, and an author who marked it as a guess.**"
            ),
            support!(
                "question/namesake-of-allen-county.yml",
                "**A claim this node made about the 1885 history is withdrawn.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Counties in the act", value: 14.0, literal: "14" },
            Figure { label: "Given a namesake", value: 11.0, literal: "11" },
            Figure { label: "Named in full", value: 8.0, literal: "8" },
            Figure { label: "Rank and war only", value: 2.0, literal: "2" },
        ],
    },
    Assertion {
        id: "one-hundred-and-eighty-seven-opinions",
        statement: "One hundred and eighty-seven published opinions name Lima State Hospital, from \
                    21 December 1920 to 1 November 2021. Ohio's own courts wrote 138 of them, the \
                    federal courts 22.",
        topic: "history",
        supports: &[
            support!("measure/lima-state-hospital-in-the-reports-1920-2021.yml", "**One hundred and eighty-seven published opinions name this institution, from 21 December 1920 to 1 November 2021.** [verified] \u{2014} [CourtListener](../../catalog/courtlistener.md), a phrase search over published opinions."),
            support!("measure/lima-state-hospital-in-the-reports-1920-2021.yml", "**Ohio's own courts wrote 138 of them and the federal courts 22.** The court of appeals 101, the Ohio Supreme Court 37, the Sixth Circuit 13, the Northern District of Ohio 6, the Southern District 3, three courts of common pleas 6, two probate courts 2, and one each from Oklahoma and Mississippi. [verified] \u{2014} same source, counted here."),
        ],
        answers: &["cannot say when Lima State Hospital stopped being Lima State Hospital"],
        figures: &[
            Figure { label: "Sixth Circuit", value: 13.0, literal: "13" },
            Figure { label: "Ohio Supreme Court", value: 37.0, literal: "37" },
            Figure { label: "Ohio court of appeals", value: 101.0, literal: "101" },
        ],
    },
    Assertion {
        id: "the-nineteen-seventies-peak-is-the-law",
        statement: "Forty-six of the opinions naming Lima State Hospital fall in the 1970s and eight \
                    in the 1990s. The right to treatment was recognised, litigated and largely \
                    settled inside that decade, so the series measures how much law was in dispute.",
        topic: "history",
        supports: &[
            support!("measure/lima-state-hospital-in-the-reports-1920-2021.yml", "**The peak is the 1970s and it is a fact about the law, not about the hospital.** 46 opinions in the decade of [the class action](../event/davis-v-watkins.yml), against 8 in the 1990s. The right to treatment was recognised, litigated and largely settled inside that decade, and a count of opinions rises when a doctrine is unsettled and falls when it is not. This series measures how much law was in dispute. [inference] \u{2014} the reasoning is this corpus's."),
        ],
        answers: &["cannot say when Lima State Hospital stopped being Lima State Hospital"],
        figures: &[
            Figure { label: "1970s", value: 46.0, literal: "46" },
            Figure { label: "1990s", value: 8.0, literal: "8" },
        ],
    },
    Assertion {
        id: "litigated-in-other-peoples-counties",
        statement: "8 of the 101 appellate opinions naming Lima State Hospital carry Allen County's \
                    own docket form; the other ninety-three open with Hamilton County's C-, \
                    Franklin's 06AP-, Mahoning's 07 MA, or a bare sequence number.",
        topic: "history",
        supports: &[
            support!("measure/lima-state-hospital-in-the-reports-1920-2021.yml", "**8 of the 101 appellate opinions carry Allen County's own docket form.** Ohio's Third District numbers a case with a county code first and this county's is 1; the remaining 93 open with Hamilton County's `C-`, Franklin's `06AP-`, Mahoning's `07 MA`, or a bare sequence number. The institution stood here and was litigated about somewhere else, which is what a maximum-security hospital serving a whole state looks like from inside one county. [verified] \u{2014} the same source, its docket numbers, parsed here; the reading is this corpus's. [inference]"),
        ],
        answers: &["cannot say when Lima State Hospital stopped being Lima State Hospital"],
        figures: &[
            Figure { label: "this county's docket form", value: 8.0, literal: "8" },
            Figure { label: "all appellate opinions", value: 101.0, literal: "101" },
        ],
    },
    Assertion {
        id: "three-opinions-one-docket-one-judge",
        statement: "A federal class action for everyone confined in Allen County's state hospital \
                    ran from 1973 to 1980 on one docket before one judge and produced three \
                    published opinions. The Civil Rights Division of the United States Department of \
                    Justice appeared in it.",
        topic: "history",
        supports: &[
            support!("event/davis-v-watkins.yml", "**A federal class action on behalf of everyone confined in Allen County's state hospital, brought in 1973 and decided in three published opinions over six years.** All three are the United States District Court for the Northern District of Ohio on docket C 73-205 before Judge Walinski, and the defendant's name changes each time because the superintendent did: *Davis v. Watkins*, 384 F. Supp. 1196 (9 September 1974); *Davis v. Balson*, 461 F. Supp. 842 (28 September 1978); *Davis v. Hubbard*, 506 F. Supp. 915 (16 September 1980). [verified] \u{2014} [the Caselaw Access Project](../../catalog/caselaw-access-project.md), the three opinions."),
            support!("event/davis-v-watkins.yml", "**The Civil Rights Division of the United States Department of Justice appeared in it.** The reported counsel are attorneys from Toledo and from A.B.L.E. for the plaintiffs, the Ohio Attorney General for the defendants, and the Assistant Attorney General and two attorneys of the Civil Rights Division for the United States. [verified] \u{2014} [CourtListener](../../catalog/courtlistener.md), the reported appearances."),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "a-remedy-copied-from-alabama",
        statement: "The 1974 order requiring eighty square feet per patient, one toilet for every \
                    six and one shower for every eight states in its own second paragraph that many \
                    of its paragraphs were excerpted verbatim from Wyatt v. Stickney \u{2014} an \
                    Alabama case about Alabama institutions.",
        topic: "history",
        supports: &[
            support!("event/davis-v-watkins.yml", "**The first opinion is a remedy and it says whose.** It states that the court \"agrees almost totally with the reasoning\" of *Wyatt v. Stickney* and that \"many of these paragraphs have been excerpted verbatim from that opinion\" \u{2014} an Alabama case about Alabama institutions. Its requirements are therefore a standard and not a description of this place. [verified] \u{2014} same source, 384 F. Supp. 1196; see [a remedy is not a finding](../../decisions/a-remedy-is-not-a-finding.yml)."),
            support!("event/davis-v-watkins.yml", "**What it required.** Eighty square feet per patient in a shared room, one toilet for every six patients and one shower for every eight, forty square feet of dayroom and ten of dining room per patient, linen changed at least every seven days, a physician on call at all times, no censorship of books or newspapers, unmonitored telephone calls, and a prohibition on \"repetitive, nonfunctional, degrading, and unnecessary tasks (so-called 'make work') such as buffing a waxed floor that has already been sufficiently buffed, polishing brass, or shining employees' shoes.\" [verified] \u{2014} the same source. Every patient then held was to be evaluated by a three-person team beginning 15 September 1974, and released, transferred or retained on the team's finding. [verified] \u{2014} same source."),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "a-thousand-two-hundred-and-ninety-five-to-four-hundred",
        statement: "Lima State Hospital held 1,295 patients in March 1971, 783 on 31 August 1973 and \
                    slightly over 400 in 1980, in two buildings of 24 and 4 wards behind a \
                    thirteen-foot fence with seven guard towers manned round the clock.",
        topic: "history",
        supports: &[
            support!("event/davis-v-watkins.yml", "**What the later opinions found, which is the part that is about this county.** The hospital held 1,295 patients in March 1971, 783 on 31 August 1973 and 761 that November, and slightly over 400 by 1980; 73 per cent of them were on a psychotropic drug and 85 per cent were able to decide rationally whether to consent to one. [verified] \u{2014} the same source, 461 F. Supp. 842 and 506 F. Supp. 915."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "March 1971", value: 1295.0, literal: "1,295" },
            Figure { label: "August 1973", value: 783.0, literal: "783" },
            Figure { label: "November 1973", value: 761.0, literal: "761" },
        ],
    },
    Assertion {
        id: "fifty-four-square-feet-against-an-ordered-eighty",
        statement: "The rooms in Lima State Hospital's old building were six feet by nine \u{2014} \
                    fifty-four square feet \u{2014} against an order six years earlier requiring \
                    eighty. The court refused to order a hundred, finding no judicial precedent for \
                    it, and held that patients had a constitutional right to refuse psychotropic \
                    medication.",
        topic: "history",
        supports: &[
            support!("event/davis-v-watkins.yml", "**A room of six feet by nine is fifty-four square feet, and the order six years earlier had required eighty.** The corpus states both because neither alone is the measurement. [inference] \u{2014} computed here; see [a remedy is not a finding](../../decisions/a-remedy-is-not-a-finding.yml)."),
            support!("event/davis-v-watkins.yml", "**The court declined the plaintiffs' central demand on space and granted the one on medication.** It found no judicial precedent for a hundred square feet as a constitutional minimum and refused to order new construction, noting the reduced population; and it held that patients had a constitutional right to refuse psychotropic medication. [verified] \u{2014} same source, 506 F. Supp. 915."),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "forty-four-point-eight-per-cent",
        statement: "The parties in the 1980 litigation stipulated that Lima State Hospital's patient \
                    population was 44.8 per cent Black, and that 73 per cent of patients were on a \
                    psychotropic drug while 85 per cent were able to decide for themselves whether \
                    to take one.",
        topic: "history",
        supports: &[
            support!("event/davis-v-watkins.yml", "**The composition finding lands on something this corpus had already inferred and could not show.** The Lima State Hospital node read the 1940 census's race and sex ratios in Bath Township \u{2014} 260 Black residents running 204 men to 56 \u{2014} as describing a committed population rather than a settled one. A stipulated fact from 1980 puts the institution at 44.8 per cent Black in a county that was nothing like it. [verified] \u{2014} the same source, against [the county by race](../measure/allen-county-population-by-race-1930-1960.yml). Forty years separate the two and neither is evidence for the other; they are two observations of the same arrangement. [inference]"),
            support!("event/davis-v-watkins.yml", "**What the later opinions found, which is the part that is about this county.** The hospital held 1,295 patients in March 1971, 783 on 31 August 1973 and 761 that November, and slightly over 400 by 1980; 73 per cent of them were on a psychotropic drug and 85 per cent were able to decide rationally whether to consent to one. [verified] \u{2014} the same source, 461 F. Supp. 842 and 506 F. Supp. 915."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Black", value: 44.8, literal: "44.8" },
            Figure { label: "on a psychotropic drug", value: 73.0, literal: "73" },
            Figure { label: "able to decide", value: 85.0, literal: "85" },
        ],
    },
    Assertion {
        id: "the-category-split-in-2000",
        statement: "Allen County's Black population grew at every census from 1920 to 2000 \
                    including the twenty years after the county's own peak, and then appears to \
                    stop. 2000 is also the first census at which a person could be counted as more \
                    than one race. Counted the way the nine censuses before it counted, the 2020 \
                    figure is 15,636 rather than 12,573.",
        topic: "population",
        // The plateau and the split are the same three censuses read two ways, which is why the
        // alone series is plotted rather than the combination one: the flat line is the artifact.
        supports: &[
            support!(
                "measure/allen-county-population-by-race-1970-2020.yml",
                "**The county's Black population grew at every census from 1920 to 2000.** 1,385 \u{b7} 1,686 \u{b7} 1,916 \u{b7}\n  4,391 \u{b7} 7,322 \u{b7} 9,234 \u{b7} 10,975 \u{b7} 12,313 \u{b7} 13,225, across eighty years and nine counts with no\n  reversal in any of them."
            ),
            support!(
                "measure/allen-county-population-by-race-1970-2020.yml",
                "**In figures: 9234 in 1970, 10975 in 1980, 12313 in 1990, 13225 in 2000, 12639 in 2010 and 12573\n  in 2020.**"
            ),
            support!(
                "measure/allen-county-population-by-race-1970-2020.yml",
                "**Two of those decades are decades in which the county was emptying.** Allen County peaked at\n  112,241 in 1980 and has fallen at every census since; between 1980 and 2000 it lost 3,768 people\n  and gained 2,250 Black residents."
            ),
            support!(
                "measure/allen-county-population-by-race-1970-2020.yml",
                "**Counted the way every census before 2000 counted, the county's Black population in 2020 is\n  15,636.**"
            ),
            support!(
                "measure/allen-county-population-by-race-1970-2020.yml",
                "**Lima's share of the county's Black residents did not fall in every decade, and this corpus has\n  said that it did.**"
            ),
            support!(
                "measure/allen-county-population-by-race-1970-2020.yml",
                "**The census and the survey disagree by 1,768 people on a figure this corpus publishes.** The 2020\n  census counted 12,573 Black-alone residents of Allen County; the American Community Survey's\n  2019\u{2013}2023 five-year estimate is 10,805 \u{b1} 658."
            ),
        ],
        answers: &[
            "This corpus does not establish why they differ.",
        ],
        figures: &[
            Figure { label: "1970", value: 9_234.0, literal: "9234" },
            Figure { label: "1980", value: 10_975.0, literal: "10975" },
            Figure { label: "1990", value: 12_313.0, literal: "12313" },
            Figure { label: "2000", value: 13_225.0, literal: "13225" },
            Figure { label: "2010", value: 12_639.0, literal: "12639" },
            Figure { label: "2020", value: 12_573.0, literal: "12573" },
        ],
    },
    Assertion {
        id: "the-farmland-did-not-drain",
        statement: "Allen County's farmland did not go once. Nine of the seventeen intervals in a \
                    now-complete record are rises: 87,824 acres left its farms between 1910 and \
                    2022 and 26,273 came back, three returned for every ten lost. The peak is 1920 \
                    and not 1910, and the largest single loss is the 13,239 acres of 1920 to 1925 \
                    \u{2014} larger than the 1950\u{2013}54 collapse this corpus had published as the sharpest \
                    stretch of the century, and invisible to it.",
        topic: "land",
        // The acreage is plotted rather than the share of the county's land area: the land-area
        // denominator moves between volumes — 259,840 acres in 1925, 262,405 in 1969 — and a line
        // of percentages would carry that as if it were farmland.
        supports: &[
            support!(
                "measure/allen-county-farmland-1910-2022.yml",
                "**The farmland did not drain. It sawtoothed.** Nine of the seventeen intervals are rises. Across\n  the whole record 87,824 acres left the county's farms and 26,273 came back, for a net loss of\n  61,551 \u{2014} **three acres returned for every ten lost**."
            ),
            support!(
                "measure/allen-county-farmland-1910-2022.yml",
                "**In figures: 240,472 acres in 1910, 241,488 in 1920, 235,791 in 1935, 215,018 in 1954, 206,047 in\n  1959, 215,862 in 1969, 195,604 in 1978 and 178,921 in 2022.**"
            ),
            support!(
                "measure/allen-county-farmland-1910-2022.yml",
                "**The county's farmland peak is 1920, not 1910.** 241,488 acres, a thousand above the 1910 figure\n  this corpus has treated as the top of the series since it first held one."
            ),
            support!(
                "measure/allen-county-farmland-1910-2022.yml",
                "**The largest single loss in the record is one nobody here had seen.** Allen County lost 13,239\n  acres of farmland between 1920 and 1925 \u{2014} more than the 12,926 it lost between 1950 and 1954,\n  which this corpus had published as the sharpest stretch of the century."
            ),
            support!(
                "measure/allen-county-farmland-1910-2022.yml",
                "**And that published finding survives anyway, on the measure it was made on.** 1950 to 1954 is\n  four years and 1920 to 1925 is five, so the rate is 3,232 acres a year against 2,648, and 1950\u{2013}54\n  is still the fastest loss in the record."
            ),
            support!(
                "measure/allen-county-farmland-1910-2022.yml",
                "**The farm count peaks in the Depression.** 3,016 farms in 1935 against 2,939 in 1910 and 2,684 in\n  1930 \u{2014} and the farmland with them, 5,367 acres more in 1935 than five years before."
            ),
            support!(
                "measure/allen-county-farmland-1910-2022.yml",
                "**The nineteen-sixties gained farmland, inside one definition.** 206,047 acres in 1959 and 215,862\n  in 1969, a rise of 9,815 or 4.8 per cent, across three censuses that share a farm definition."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1910", value: 240_472.0, literal: "240,472" },
            Figure { label: "1920", value: 241_488.0, literal: "241,488" },
            Figure { label: "1935", value: 235_791.0, literal: "235,791" },
            Figure { label: "1954", value: 215_018.0, literal: "215,018" },
            Figure { label: "1959", value: 206_047.0, literal: "206,047" },
            Figure { label: "1969", value: 215_862.0, literal: "215,862" },
            Figure { label: "1978", value: 195_604.0, literal: "195,604" },
            Figure { label: "2022", value: 178_921.0, literal: "178,921" },
        ],
    },
    Assertion {
        id: "the-county-dies-younger-than-its-state",
        statement: "Allen County has gone from healthier than America to a quarter worse than it. \
                    Its rate of years lost before seventy-five was below the national rate in the \
                    first three windows of the record and above it in the twenty-one since; deaths \
                    before seventy-five rose 46.1 per cent between 2011\u{2013}2013 and 2020\u{2013}2022 \
                    while the under-75 population fell 4.0 per cent. And it is not the opioid \
                    county the state's reputation would predict: its overdose rate is 15 per cent \
                    below Ohio's, and what it exceeds the state on is suicide, road deaths, \
                    homicide and firearms.",
        topic: "health",
        // Nine windows of the county's own rate are plotted rather than all twenty-four: the
        // windows overlap by two years, so consecutive points are not independent and a line
        // through every one of them would read as more measurement than the file contains.
        supports: &[
            support!(
                "measure/allen-county-premature-death-1997-2022.yml",
                "**The county has gone from healthier than America to a quarter worse than it, and the crossing is\n  early.** Allen County is below the national rate in the first three windows only \u{2014} 7,162 against\n  7,705 in 1997\u{2013}1999 \u{2014} and above it in the twenty-one since, ending 25.5 per cent above."
            ),
            support!(
                "measure/allen-county-premature-death-1997-2022.yml",
                "**In figures, taking every third window: the county's rate runs 7,162, 7,599, 7,819, 7,428, 6,759,\n  7,142, 9,033, 8,518 and 10,482, against a national 7,705, 7,535, 7,345, 7,090, 6,704, 6,601,\n  6,901, 7,282 and 8,352.**"
            ),
            support!(
                "measure/allen-county-premature-death-1997-2022.yml",
                "**The count says the same thing without a denominator in it, which matters here.** Deaths before\n  seventy-five in Allen County were 1,306 in the three years 2011\u{2013}2013 and **1,908** in 2020\u{2013}2022, a\n  rise of **46.1 per cent**, while the county's under-75 population in the same two windows fell 4.0\n  per cent, from 293,386 to 281,611 person-years."
            ),
            support!(
                "measure/allen-county-early-deaths-by-cause-2020-2022.yml",
                "**A person born in Allen County in these years could expect 74.3 years, which is 2.8 fewer than an\n  American and 0.9 fewer than an Ohioan.**"
            ),
            support!(
                "measure/allen-county-early-deaths-by-cause-2020-2022.yml",
                "**It is not the opioid county the state's reputation would predict.** Allen County's overdose rate\n  of 38.1 is **15 per cent below Ohio's** 44.7, and it is one of only four rows where the county\n  does better than the state. What it exceeds the state on is suicide by 14 per cent, motor vehicle\n  crashes by 10 per cent, homicide by 9 and firearms by 5."
            ),
            support!(
                "measure/allen-county-early-deaths-by-cause-2020-2022.yml",
                "**The gap between the county's Black and white residents is wider here than in the state or the\n  country.** Life expectancy is 68.1 years for Black residents and 75.0 for white ones, a gap of\n  **6.91 years**, against 5.48 in Ohio and 5.08 nationally."
            ),
        ],
        answers: &[
            "does not infer a cause from a curve",
            "cannot say from these figures whether the cause is who lives where or what happens to whom",
        ],
        figures: &[
            Figure { label: "1997\u{2013}99", value: 7_162.0, literal: "7,162" },
            Figure { label: "2000\u{2013}02", value: 7_599.0, literal: "7,599" },
            Figure { label: "2003\u{2013}05", value: 7_819.0, literal: "7,819" },
            Figure { label: "2006\u{2013}08", value: 7_428.0, literal: "7,428" },
            Figure { label: "2009\u{2013}11", value: 6_759.0, literal: "6,759" },
            Figure { label: "2012\u{2013}14", value: 7_142.0, literal: "7,142" },
            Figure { label: "2015\u{2013}17", value: 9_033.0, literal: "9,033" },
            Figure { label: "2018\u{2013}20", value: 8_518.0, literal: "8,518" },
            Figure { label: "2020\u{2013}22", value: 10_482.0, literal: "10,482" },
        ],
    },
    Assertion {
        id: "overdose-deaths-fell-by-two-thirds",
        statement: "Drug overdose deaths in Allen County peaked in the twelve months to February \
                    2023 at fifty-four and the twelve months to December 2025 count seventeen \u{2014} a \
                    fall of 68.5 per cent, steeper than Ohio's 56.1 and the nation's 36.6, and \
                    below where the county's record starts.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-drug-overdose-deaths-2020-2025.yml",
                "**The epidemic peaked here in the twelve months to February 2023 at fifty-four deaths, and the\n  twelve months to December 2025 count seventeen \u{2014} a fall of 68.5 per cent.**"
            ),
            support!(
                "measure/allen-county-drug-overdose-deaths-2020-2025.yml",
                "**The fall is steeper than the state's and much steeper than the country's.** Measured to the same\n  endpoint of December 2025, Ohio is down **56.1 per cent** from its peak of 5,582 in the twelve\n  months to April 2021, and the United States down **36.6 per cent** from 111,466 in the twelve\n  months to June 2023."
            ),
            support!(
                "measure/allen-county-drug-overdose-deaths-2020-2025.yml",
                "**This is the one health figure the corpus holds twice, from two pipelines, and they agree\n  exactly.** The three December windows of 2020, 2021 and 2022 sum to 116 deaths; the final NCHS\n  mortality file, by way of\n  [County Health Rankings](../../catalog/county-health-rankings.md), gives Allen County 116 drug\n  overdose deaths for 2020\u{2013}2022."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Allen County", value: 68.5, literal: "68.5" },
            Figure { label: "Ohio", value: 56.1, literal: "56.1" },
            Figure { label: "United States", value: 36.6, literal: "36.6" },
        ],
    },
    Assertion {
        id: "the-county-did-not-turn-with-1896",
        statement: "The realignment of 1896 did not reach Allen County. William McKinley, Ohio's \
                    own governor at the head of the ticket, gained five points here over 1892 and \
                    still lost the county by 1,401 votes \u{2014} its widest Democratic margin of the \
                    six elections. The county did not turn at an election: across the seven from \
                    1884 to 1908 its margin never left a band 331 votes wide while the vote cast \
                    grew 74 per cent, so a fixed block of votes was diluted rather than converted.",
        topic: "elections",
        // The margin is plotted as a share rather than as a count of votes: the count is the thing
        // that does not move, and a line of it would be flat where the finding is a slope.
        supports: &[
            support!(
                "measure/allen-county-presidential-vote-1888-1908.yml",
                "**The realignment of 1896 did not reach this county.** William McKinley was Ohio's own governor\n  and the head of the ticket, and he gained five points here over Harrison's 1892 share \u{2014} and still\n  lost Allen County by 1,401 votes, the widest Democratic margin of the six."
            ),
            support!(
                "measure/allen-county-presidential-vote-1888-1908.yml",
                "**The county did not turn at an election. A fixed block of votes was diluted.** Across the seven\n  presidential elections from 1884 to 1908 the margin never leaves a band 331 votes wide \u{2014} 1,070,\n  1,362, 1,366, 1,401, 1,259, 1,111 and 1,354 \u{2014} while the vote cast grows from 7,903 to 13,718, or\n  74 per cent. As a share the Democratic edge falls almost monotonically: 13.5 points, 14.9, 14.5,\n  12.2, 10.5, \u{2212}9.6 and 9.9, the negative term being 1904 and the only one the Republican carried."
            ),
            support!(
                "measure/allen-county-presidential-vote-1888-1908.yml",
                "**The county's first Republican presidential majority since the Civil War is 1904, and it lasted\n  one election.** Roosevelt takes 52.5 per cent; four years later Bryan takes 7,195 votes, the\n  largest Democratic vote anywhere in this run, and the county goes back."
            ),
            support!(
                "measure/allen-county-presidential-vote-1888-1908.yml",
                "**Warren G. Harding lost this county before he won it.** Running for governor in 1910 he took\n  3,825 votes here against Judson Harmon's 5,837, on a total of 10,574; ten years later he carried\n  it for president 13,978 to 11,658."
            ),
            support!(
                "measure/allen-county-presidential-vote-1888-1908.yml",
                "**The office printed one of these figures two ways.** McKinley's 1896 vote here is 4,959 in the\n  abstract and in the 1904 volume, and 4,956 in the 1908 volume."
            ),
            support!(
                "measure/allen-county-presidential-vote-1920.yml",
                "**Six of the eight elections in between are now held, and not one of them is the turn.** The\n  county went Democratic in 1888, 1892, 1896, 1900 and 1908, and Republican once, in 1904, by 1,111\n  votes \u{2014} a margin inside the same band as every Democratic one."
            ),
        ],
        answers: &["does not know who the arriving voters were"],
        figures: &[
            Figure { label: "1884", value: 13.5, literal: "13.5" },
            Figure { label: "1888", value: 14.9, literal: "14.9" },
            Figure { label: "1892", value: 14.5, literal: "14.5" },
            Figure { label: "1896", value: 12.2, literal: "12.2" },
            Figure { label: "1900", value: 10.5, literal: "10.5" },
            Figure { label: "1904", value: -9.6, literal: "\u{2212}9.6" },
            Figure { label: "1908", value: 9.9, literal: "9.9" },
        ],
    },
    Assertion {
        id: "the-county-changed-sides",
        statement: "Allen County was Democratic ground for most of the nineteenth century. The \
                    Democrat carried it at seven of the eight presidential elections its own 1885 \
                    history records, and in 1884 Lima was the Republican end of the county and the \
                    townships the Democratic end \u{2014} the reverse of the arrangement it has today.",
        topic: "elections",
        // The share series is plotted rather than the vote counts: the electorate triples across
        // this range, and the counts would show that instead of the thing the assertion is about.
        supports: &[
            support!(
                "measure/allen-county-presidential-vote-1856-1884.yml",
                "**The Democrat carried Allen County at seven of these eight elections, and the exception is the\n  one the book says it is estimating.** 1856, 1860, 1868, 1872, 1876, 1880 and 1884 all go the\n  Democratic way; only 1864 does not, and its two figures are the only round numbers in the\n  chapter."
            ),
            support!(
                "measure/allen-county-presidential-vote-1856-1884.yml",
                "**In figures: the Democratic share was 50.0 per cent in 1856, 50.5 in 1860, 45.7 in 1864, 58.9 in\n  1868, 54.7 in 1872, 58.5 in 1876, 56.8 in 1880 and 56.2 in 1884.**"
            ),
            support!(
                "measure/allen-county-presidential-vote-1884-by-township.yml",
                "**Lima went Republican while the county went Democratic.** The city's four wards give 1,156 to\n  1,118; the county gives 3,372 to 4,442."
            ),
            support!(
                "measure/allen-county-presidential-vote-1884-by-township.yml",
                "**The Republican column adds to its printed total and the Democratic column does not.** The twenty\n  Republican figures sum to 3,372, exactly as printed. The twenty Democratic figures sum to 4,441\n  against a printed 4,442."
            ),
            support!(
                "measure/allen-county-presidential-vote-1920.yml",
                "**11,658 for Cox and 13,978 for Harding**, with 429 for Debs and 17 for Macauley \u{2014} a Republican\n  plurality of 2,320 on 26,082 votes."
            ),
            support!(
                "measure/allen-county-presidential-vote-1920.yml",
                "**This is the first official state election return this corpus holds.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1856", value: 50.0, literal: "50.0" },
            Figure { label: "1860", value: 50.5, literal: "50.5" },
            Figure { label: "1864", value: 45.7, literal: "45.7" },
            Figure { label: "1868", value: 58.9, literal: "58.9" },
            Figure { label: "1872", value: 54.7, literal: "54.7" },
            Figure { label: "1876", value: 58.5, literal: "58.5" },
            Figure { label: "1880", value: 56.8, literal: "56.8" },
            Figure { label: "1884", value: 56.2, literal: "56.2" },
        ],
    },
    Assertion {
        id: "the-winters-warmed-and-the-summers-did-not",
        statement: "Allen County is 2.03 \u{b0}F warmer than it was a century ago and its summers are \
                    not. The mean daily maximum of June, July and August has moved eleven \
                    hundredths of a degree since 1895\u{2013}1924; December, January and February have \
                    moved 3.32. The hottest summers in the record are still 1934 and 1936.",
        topic: "land",
        // The decade rainfall series is plotted rather than the temperature one: the temperature
        // finding is about the shape inside the year, which a line of annual means would hide —
        // which is the thing the assertion is against.
        supports: &[
            support!(
                "measure/allen-county-temperature-1895-2025.yml",
                "**The county is two degrees warmer than it was, and none of it is in summer.** The mean daily\n  maximum temperature of June, July and August was 82.48 \u{b0}F across 1895\u{2013}1924 and is 82.59 \u{b0}F across\n  1996\u{2013}2025 \u{2014} a change of **eleven hundredths of a degree in a century**."
            ),
            support!(
                "measure/allen-county-temperature-1895-2025.yml",
                "**In figures: December, January and February warmed by 3.32 degrees on average and June, July and\n  August by 1.01.**"
            ),
            support!(
                "measure/allen-county-temperature-1895-2025.yml",
                "**The hottest summers in the record are still the 1930s.** Ranked by mean daily maximum for June\n  to August: 1934 at 88.8 \u{b0}F, 1936 at 88.2, 1933 at 86.9, 1988 at 86.8, 1952 at 86.6, 1944 at 86.4.\n  **None is later than 1988.**"
            ),
            support!(
                "measure/allen-county-temperature-1895-2025.yml",
                "**The nights warmed faster than the days**, by 2.36 degrees against 1.70."
            ),
            support!(
                "measure/allen-county-precipitation-1895-2025.yml",
                "**Almost all of the additional rain falls in the growing season.** Of 3.66 inches added to the\n  year, 3.27 fall between April and September; March is **drier** by half an inch and January by a\n  seventh."
            ),
            support!(
                "measure/allen-county-precipitation-1895-2025.yml",
                "**Both of those months are years the corpus already had a reason to look at.**"
            ),
        ],
        answers: &[
            "It cannot say what was in the river; it can say what fell on the ground.",
        ],
        figures: &[
            Figure { label: "Winter", value: 3.32, literal: "3.32" },
            Figure { label: "Summer", value: 1.01, literal: "1.01" },
        ],
    },
    Assertion {
        id: "the-factories-left-lima-and-not-the-county",
        statement: "Allen County's factories did not leave the county. They left Lima. The city \
                    held 77.4 per cent of the county's factory workforce in 1939 and 75.4 per cent \
                    as late as 1954; by 1963 it held 35.9, having lost 5,727 people while the \
                    ground outside its line gained 5,933. The industrial peak nobody could see \
                    across those forty years is not in them.",
        topic: "work",
        // The share is plotted rather than the levels because the levels are not one series:
        // 1929 and 1939 count wage earners and the rest count all employees. The city and the
        // county change column together in the same table, so the ratio survives the break that
        // the two levels do not — which is the argument the node makes and the reason it is safe
        // to draw seven points here.
        supports: &[
            support!(
                "measure/allen-county-manufacturing-outside-lima-1929-1967.yml",
                "**In figures: Lima held 69.0 per cent of the county's factory workforce in 1929, 77.4 in 1939,\n  77.2 in 1947, 75.4 in 1954, 44.5 in 1958, 35.9 in 1963 and 40.9 in 1967.**"
            ),
            support!(
                "measure/allen-county-manufacturing-outside-lima-1929-1967.yml",
                "**Between 1954 and 1958 the county's factory employment fell by 1,165 and Lima's fell by 5,055.**\n  Outside the city line it rose from 3,618 to 7,508 — it more than doubled in four years."
            ),
            support!(
                "measure/allen-county-manufacturing-outside-lima-1929-1967.yml",
                "**Across the nine years to 1963 the two move almost exactly against each other.** Lima lost 5,727\n  and the ground outside it gained 5,933, while the county as a whole gained 206."
            ),
            support!(
                "measure/allen-county-manufactures-1939-1967.yml",
                "**The forty years this corpus could not see are measured, and the county's industrial peak is not\n  in them.** The highest figure any census inside the gap returns is 16,400 in 1967, and the federal\n  employment series that begins two years later opens at 17,623 and peaks at 18,400 in 1973."
            ),
            support!(
                "measure/lima-manufactures-1939-1967.yml",
                "**The city did not lose factories. It lost large ones.** Lima had 84 manufacturing\n  establishments in 1954 and 88 in 1958 — four more — while its employment fell by 5,055. The\n  average establishment went from 132 people to 68."
            ),
            support!(
                "measure/allen-county-manufacturing-outside-lima-1929-1967.yml",
                "**Lima's share of the county's people peaked one census before its share of the county's work.**\n  The city held 61.0 per cent of the county's population in 1940 and never more; it held 77.4 per\n  cent of the county's factory workforce in 1939 and 75.4 as late as 1954."
            ),
        ],
        answers: &[
            "cannot say that the plant on Bible Road is where those people went",
        ],
        figures: &[
            Figure { label: "1929", value: 69.0, literal: "69.0" },
            Figure { label: "1939", value: 77.4, literal: "77.4" },
            Figure { label: "1947", value: 77.2, literal: "77.2" },
            Figure { label: "1954", value: 75.4, literal: "75.4" },
            Figure { label: "1958", value: 44.5, literal: "44.5" },
            Figure { label: "1963", value: 35.9, literal: "35.9" },
            Figure { label: "1967", value: 40.9, literal: "40.9" },
        ],
    },
    Assertion {
        id: "the-frost-free-season-is-three-weeks-longer",
        statement: "Allen County's frost-free season averaged 160.5 days across 1902\u{2013}1931 and \
                    averages 181.0 across 1996\u{2013}2025 \u{2014} the last freezing night of spring nine \
                    days earlier, the first of autumn twelve days later. It is not a trend but a \
                    step, and it is in the 1990s. At the other end of the year nothing has moved: \
                    the hottest day ever measured here is still 109 \u{b0}F, in 1936.",
        topic: "land",
        // The decade series is plotted rather than the two thirty-year windows, because the shape
        // is the argument: a flat century and then a step. Two windows would draw a slope.
        supports: &[
            support!(
                "measure/allen-county-frost-free-season-1902-2025.yml",
                "**Allen County's frost-free season is twenty and a half days longer than it was a century ago.**\n  The last freezing night of spring has moved nine days earlier and the first of autumn twelve days\n  later."
            ),
            support!(
                "measure/allen-county-frost-free-season-1902-2025.yml",
                "**In figures: the frost-free season averaged 156.8 days in the 1900s, 162.4 in the 1910s, 159.4\n  in the 1920s, 164.8 in the 1930s, 153.8 in the 1940s, 155.5 in the 1950s, 154.8 in the 1960s,\n  163.1 in the 1970s, 167.6 in the 1980s, 178.4 in the 1990s, 179.1 in the 2000s, 186.0 in the\n  2010s and 182.5 in the 2020s.**"
            ),
            support!(
                "measure/allen-county-frost-free-season-1902-2025.yml",
                "**It is not a trend. It is a step, and it is in the 1990s.** Every decade from the 1900s to the\n  1980s falls between 153.8 and 167.6 days; every decade from the 1990s falls between 178.4 and\n  186.0. There is no decade in between."
            ),
            support!(
                "measure/allen-county-frost-free-season-1902-2025.yml",
                "**The station stands in a city and the finding is not the city.** Van Wert 1 S, forty kilometres\n  west in another county and a town a third of Lima's size, gives 181.4 days for 1996\u{2013}2025 against\n  Lima's 181.0 \u{2014} four tenths of a day apart \u{2014} and its decade series steps in the same place, from\n  161.0 in the 1980s to 183.6 in the 1990s."
            ),
            support!(
                "measure/allen-county-temperature-extremes-1901-2026.yml",
                "**The hottest day ever recorded in Allen County is 109 \u{b0}F, on 14 July 1936.** The coldest is\n  \u{2212}21 \u{b0}F, reached twice \u{2014} on 20 January 1985 and again on 19 January 1994."
            ),
            support!(
                "measure/allen-county-temperature-extremes-1901-2026.yml",
                "**Four of them are after 1954.** Two on 15 and 16 July 1988, two on 29 June and 7 July 2012, and\n  nothing since."
            ),
            support!(
                "measure/lima-snowfall-1901-2025.yml",
                "**This record cannot be trended and the reason is the finding.** From the 1978\u{2013}79 season through\n  2005\u{2013}06 the station reported snowfall on **two days in twenty-eight winters**, while recording\n  hundreds of days of precipitation falling at or below 34 \u{b0}F."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1900s", value: 156.8, literal: "156.8" },
            Figure { label: "1910s", value: 162.4, literal: "162.4" },
            Figure { label: "1920s", value: 159.4, literal: "159.4" },
            Figure { label: "1930s", value: 164.8, literal: "164.8" },
            Figure { label: "1940s", value: 153.8, literal: "153.8" },
            Figure { label: "1950s", value: 155.5, literal: "155.5" },
            Figure { label: "1960s", value: 154.8, literal: "154.8" },
            Figure { label: "1970s", value: 163.1, literal: "163.1" },
            Figure { label: "1980s", value: 167.6, literal: "167.6" },
            Figure { label: "1990s", value: 178.4, literal: "178.4" },
            Figure { label: "2000s", value: 179.1, literal: "179.1" },
            Figure { label: "2010s", value: 186.0, literal: "186.0" },
            Figure { label: "2020s", value: 182.5, literal: "182.5" },
        ],
    },
    Assertion {
        id: "the-state-replaced-its-bridges",
        statement: "In 1992 the state owned the worst bridges in Allen County \u{2014} 21.9 per cent of \
                    them in poor condition against the county's 5.8. It has had none since 2019. \
                    Eighteen of those twenty-one are gone from the inventory while the state's \
                    holding grew, so it replaced them rather than repaired them; and its stock is \
                    the older of the two, built at a median of 1970 against the county's 1982.",
        topic: "geography",
        // Twelve editions of one file at three-year intervals, plotted for the state alone. The
        // county's line is in the table beside it and not on the chart, because its last point is
        // the one the corpus has just shown to be an artefact and a line would draw the eye to it.
        supports: &[
            support!(
                "measure/allen-county-bridge-condition-1992-2025.yml",
                "**In figures: 21.9 per cent of the state's bridges in this county were in poor condition in 1992,\n  16.0 in 1995, 19.1 in 1998, 5.1 in 2001, 2.1 in 2004, 2.1 in 2007, 2.9 in 2010, 1.0 in 2013, 1.0\n  in 2016, 0.0 in 2019, 0.0 in 2022 and 0.0 in 2025.**"
            ),
            support!(
                "measure/allen-county-bridge-condition-1992-2025.yml",
                "**In 1992 the state owned the worst bridges in Allen County.** Twenty-one of its ninety-six were\n  in poor condition, 21.9 per cent, against sixteen of the county's two hundred and seventy-seven,\n  5.8 per cent."
            ),
            support!(
                "measure/allen-county-bridge-condition-1992-2025.yml",
                "**It did not repair them. It replaced them.** Of the twenty-one state bridges rated poor in 1992,\n  eighteen are absent from the 2025 file altogether and three survive under the same structure\n  number with better ratings. Every one of the eighteen was built between 1940 and 1962, and the\n  state's holding rose from 96 structures to 103 across the same years, so they left by replacement\n  rather than by abandonment."
            ),
            support!(
                "measure/allen-county-bridge-condition-1992-2025.yml",
                "**Age was the other reading and the file refutes it.** The state's median bridge in this county\n  was built in 1970 and the county's in 1982; 57 of the state's 103 predate 1980 against 112 of the\n  county's 242, so the state's stock is the older of the two on both measures."
            ),
            support!(
                "measure/allen-county-bridge-condition-1992-2025.yml",
                "**And the ratings moved both ways at once.** Among the county's own bridges, superstructures rated\n  4 went from 3 to 14 between those two editions and substructures from 3 to 13, while decks rated 9\n  went from 22 to 39 and superstructures rated 9 from 22 to 38."
            ),
            support!(
                "site/hay-road-bridge.yml",
                "**Its condition is the best it has been recorded at, and nothing was done to it.** The deck, the\n  superstructure and the substructure were rated 5, 6 and 5 in the 1992 edition, 5, 5 and 4 in 2010,\n  6, 5 and 5 in 2022, and **7, 7 and 7 in 2025**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1992", value: 21.9, literal: "21.9" },
            Figure { label: "1995", value: 16.0, literal: "16.0" },
            Figure { label: "1998", value: 19.1, literal: "19.1" },
            Figure { label: "2001", value: 5.1, literal: "5.1" },
            Figure { label: "2004", value: 2.1, literal: "2.1" },
            Figure { label: "2007", value: 2.1, literal: "2.1" },
            Figure { label: "2010", value: 2.9, literal: "2.9" },
            Figure { label: "2013", value: 1.0, literal: "1.0" },
            Figure { label: "2016", value: 1.0, literal: "1.0" },
            Figure { label: "2019", value: 0.0, literal: "0.0" },
            Figure { label: "2022", value: 0.0, literal: "0.0" },
            Figure { label: "2025", value: 0.0, literal: "0.0" },
        ],
    },
    Assertion {
        id: "two-thousand-two-hundred-and-twenty-five-miles",
        statement: "Allen County has 2,225.6 miles of driveable road in 8,313 segments \u{2014} 5.53 \
                    miles for every square mile of the county, and one bridge for every 6.11 miles \
                    of it.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-roads-2010-2024.yml", "**Allen County has 2,225.6 miles of driveable road, 5.53 miles for every square mile of it.** [verified] \u{2014} [the Census Bureau's road linework](../../catalog/census-tiger-roads.md), 2024 edition, computed here against [the county's land area](allen-county-land-area-2020.yml)."),
        ],
        answers: &["cannot say how many miles of road Allen County maintains"],
        figures: &[
            Figure { label: "miles of road", value: 2225.6, literal: "2,225.6" },
            Figure { label: "miles per square mile", value: 5.53, literal: "5.53" },
        ],
    },
    Assertion {
        id: "one-interstate-drawn-twice",
        statement: "The Census Bureau draws 46.31 miles of Interstate 75 in Allen County and the \
                    Federal Highway Administration measures 23.12. The interstate is divided and one \
                    agency draws a line per carriageway; the ratios for I-75 and US 30 are 2.0030 \
                    and 1.9991.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-roads-2010-2024.yml", "**The four primary-road segments are two roads drawn twice.** Interstate 75 appears as two lines of 23.167 and 23.139 miles and US 30 as two of 1.952 and 1.737, because the file draws one line per carriageway of a divided highway. The federal highway file measures the same interstate once, at 23.12 route miles, and the same US 30 at 24.06 against this file's 48.098 \u{2014} ratios of 2.0030 and 1.9991. [verified] \u{2014} the same source against [the federal-aid file](../../catalog/fhwa-hpms-public-release.md). Every figure in this node is therefore centerline miles and not route miles; see [a centerline is not a road](../../decisions/a-centerline-is-not-a-road.yml)."),
        ],
        answers: &["cannot say how many miles of road Allen County maintains"],
        figures: &[
            Figure { label: "US 30 ratio", value: 1.9991, literal: "1.9991" },
            Figure { label: "I-75 ratio", value: 2.003, literal: "2.0030" },
        ],
    },
    Assertion {
        id: "one-mile-in-six-has-an-owner",
        statement: "One file names the owner of a road in Allen County and it covers 359.2 of the \
                    county's 2,225.6 miles. Half of that sixth belongs to the state, which carries \
                    72.0 per cent of the traffic on it against the county's 16.7.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-federal-aid-highways-2018.yml", "**One file names the owner of a road in this county, and it covers a sixth of the network.** 359.2 miles of the county's 2,225.6, in 87 routes and 2,035 sections. [verified] \u{2014} [the HPMS public release](../../catalog/fhwa-hpms-public-release.md), against [the county's road linework](allen-county-roads-2010-2024.yml)."),
            support!("measure/allen-county-federal-aid-highways-2018.yml", "**Half of it belongs to the state and the state carries seventy-two per cent of the traffic.** The state highway agency owns 182.1 of the 359.2 miles, the county 117.0, the municipalities 39.3 and the townships 20.8; the state's share of the daily vehicle-miles is 72.0 per cent against the county's 16.7, the municipalities' 8.3 and the townships' 3.0. [verified] \u{2014} the same source, computed here."),
        ],
        answers: &["cannot say who is responsible for the other 1,866 miles"],
        figures: &[
            Figure { label: "township", value: 20.8, literal: "20.8" },
            Figure { label: "city or municipal", value: 39.3, literal: "39.3" },
            Figure { label: "county", value: 117.0, literal: "117.0" },
            Figure { label: "state", value: 182.1, literal: "182.1" },
        ],
    },
    Assertion {
        id: "a-hundredth-of-the-road-and-a-third-of-the-traffic",
        statement: "Interstate 75 is 1.04 per cent of Allen County's road and carries 32.6 per cent \
                    of the traffic on everything the federal file measures \u{2014} 835,274 \
                    vehicle-miles a day over 23.12 miles, from 29,733 at the county line to 43,120 \
                    through Lima.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-federal-aid-highways-2018.yml", "**Interstate 75 is 1.04 per cent of the county's road and a third of the traffic on everything this file measures.** 23.12 miles carrying 835,274 vehicle-miles a day \u{2014} 32.6 per cent of the federal-aid total \u{2014} with a daily count that runs from 29,733 at the county line to 43,120 through Lima. [verified] \u{2014} same source, by route."),
        ],
        answers: &["cannot say who is responsible for the other 1,866 miles"],
        figures: &[
            Figure { label: "at the county line", value: 29733.0, literal: "29,733" },
            Figure { label: "through Lima", value: 43120.0, literal: "43,120" },
        ],
    },
    Assertion {
        id: "a-thirtieth-of-the-ground-and-an-eighth-of-the-road",
        statement: "Lima holds 3.38 per cent of Allen County's land and 12.61 per cent of its road: \
                    20.61 miles of street to the square mile against 5.00 for everywhere else in the \
                    county.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-roads-2010-2024.yml", "**Lima is 3.38 per cent of the county's land and 12.61 per cent of its road.** The city carries 20.61 miles of street to the square mile against 5.00 for everywhere else. [verified] \u{2014} the same file, computed here against [Lima's land area](lima-land-area-2020.yml)."),
        ],
        answers: &["cannot say how many miles of road Allen County maintains"],
        figures: &[
            Figure { label: "Lima", value: 20.61, literal: "20.61" },
            Figure { label: "the rest of the county", value: 5.0, literal: "5.00" },
        ],
    },
    Assertion {
        id: "three-times-the-road-per-head-outside-lima",
        statement: "A person living outside Lima has 29.4 miles of road per thousand residents to a \
                    Lima resident's 8.1 \u{2014} 3.63 times as much \u{2014} and no municipality is \
                    responsible for most of it.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-roads-2010-2024.yml", "**Turned round, the ratio reverses and it is the more consequential figure.** A person living outside Lima has 29.4 miles of road per thousand residents to a Lima resident's 8.1 \u{2014} **3.63 times as much** \u{2014} and no municipality is responsible for most of it. [inference] \u{2014} the same linework against [Lima's population](lima-population-2024.yml) and [the county's](allen-county-population-2024.yml). The corpus does not read that as a claim about cost, because this file carries no surface, no width and no condition and a mile of township gravel is not a mile of city street. [inference]"),
        ],
        answers: &["cannot say how many miles of road Allen County maintains"],
        figures: &[
            Figure { label: "Lima", value: 8.1, literal: "8.1" },
            Figure { label: "outside Lima", value: 29.4, literal: "29.4" },
        ],
    },
    Assertion {
        id: "the-map-stopped-before-the-county-did",
        statement: "Allen County's road linework reads 2,226.7 miles in 2019, 2,225.6 in 2022 and \
                    2,225.6 in 2024, across years in which seven municipalities were annexing \
                    ground. Between 2016 and 2019 US 30's mileage is 48.098 in both editions while \
                    3.689 miles of it move from secondary road to primary.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-roads-2010-2024.yml", "**The file's last three editions are identical and its earlier moves are the cartographer.** The county's driveable mileage reads 2,358.5 in 2010, 2,299.1 in 2013, 2,304.2 in 2016, 2,226.7 in 2019, 2,225.6 in 2022 and 2,225.6 in 2024. [verified] \u{2014} the same source, six editions of it, computed here."),
            support!("measure/allen-county-roads-2010-2024.yml", "**Nothing in this county was built or removed to make those numbers.** Between the 2016 and 2019 editions US 30's mileage is 48.098 in both, to a thousandth, while 3.689 miles of it move from secondary road to primary and the county's secondary total falls by 3.743. Between 2016 and 2019 the file also loses 1,273 segments and 77.5 driveable miles, in three years during which seven municipalities were annexing ground and the county was authorising new houses. [verified] \u{2014} the same source, two editions of it, against [the annexations](allen-county-annexations-1990-2024.yml). Read as a series this file measures the Bureau's map maintenance; see [a column can empty into its neighbour](../../decisions/a-column-can-empty-into-its-neighbour.yml)."),
        ],
        answers: &["cannot say how many miles of road Allen County maintains"],
        figures: &[
            Figure { label: "2010", value: 2358.5, literal: "2,358.5" },
            Figure { label: "2016", value: 2304.2, literal: "2,304.2" },
            Figure { label: "2019", value: 2226.7, literal: "2,226.7" },
            Figure { label: "2024", value: 2225.6, literal: "2,225.6" },
        ],
    },
    Assertion {
        id: "every-ownership-change-is-a-corporation-line",
        statement: "23 of the 87 federal-aid routes in Allen County change owner along their length, \
                    and every change is into a municipality \u{2014} 19 county roads and 4 township \
                    roads. Not one changes between state and county, or county and township.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-federal-aid-highways-2018.yml", "**23 of the 87 routes change owner along their length, and every change is into a municipality.** 19 county roads become city or municipal and 4 township roads do; not one route in the county changes between state and county, or between county and township. The only ownership boundary this file records is a corporation line. [verified] \u{2014} same source, by route."),
        ],
        answers: &["cannot say who is responsible for the other 1,866 miles"],
        figures: &[
            Figure { label: "routes that change owner", value: 23.0, literal: "23" },
            Figure { label: "routes in the county", value: 87.0, literal: "87" },
        ],
    },
    Assertion {
        id: "the-rate-converged-and-the-workforce-did-not",
        statement: "Allen County's unemployment rate ran above Ohio's for twenty-four years \
                    running and then stopped: the gap was 0.4 points in 2013, 0.0 in 2014, and has \
                    averaged a tenth of a point since. Over the same record Ohio's labour force \
                    grew 10.3 per cent and the county's fell 8.3, so the county stopped having a \
                    higher share of its workers out of work partly by having fewer workers.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-unemployment-1990-2026.yml",
                "**The county's rate was above its state's for twenty-four years running, and then it stopped.**\n  Every year from 1990 to 2013 Allen County's annual rate exceeded Ohio's, by a mean of 1.0 points\n  and by as much as 2.1 in 1996. From 2014 the mean gap is 0.10 points and the county is at or below\n  the state in six of twelve years."
            ),
            support!(
                "measure/allen-county-unemployment-1990-2026.yml",
                "**It converged on the state and not on the country.** Against the national rate the county's mean\n  gap is +1.14 points in the 1990s, +1.25 in the 2000s and +0.28 from 2014 \u{2014} narrower, but it has\n  been below the nation in only two of the last twelve years."
            ),
            support!(
                "measure/allen-county-unemployment-1990-2026.yml",
                "**The convergence is a subtraction.** Across the record Ohio's labour force grew 10.3 per cent and\n  Allen County's fell 8.3."
            ),
            support!(
                "measure/allen-county-unemployment-1990-2026.yml",
                "**Fewer people work in this county now than in 1990.** Employed residents peaked at 49,671 in 2006\n  and stand at 45,950 in 2025 \u{2014} 3,721 fewer, a fall of 7.5 per cent \u{2014} and the 2025 figure is 2,623\n  below 1990's."
            ),
            support!(
                "measure/allen-county-unemployment-1990-2026.yml",
                "**April 2020 is the worst month in the record and nothing else is close.** The county's rate was\n  18.6 per cent, against a previous worst of 12.7 in March 2009."
            ),
            support!(
                "measure/allen-county-unemployment-1990-2026.yml",
                "**In figures: the county's annual rate was 7.7 per cent in 1990, 4.7 in 2000, 11.4 in 2009, 5.8 in\n  2014, 4.0 in 2019, 8.5 in 2020 and 4.8 in 2025**, and its labour force fell from 52,911 in 2006 to\n  48,288 in 2025."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1990", value: 7.7, literal: "7.7" },
            Figure { label: "2000", value: 4.7, literal: "4.7" },
            Figure { label: "2009", value: 11.4, literal: "11.4" },
            Figure { label: "2014", value: 5.8, literal: "5.8" },
            Figure { label: "2019", value: 4.0, literal: "4.0" },
            Figure { label: "2020", value: 8.5, literal: "8.5" },
            Figure { label: "2025", value: 4.8, literal: "4.8" },
        ],
    },
    Assertion {
        id: "eleven-years-of-more-jobs-than-workers",
        statement: "Two programmes that never consult each other \u{2014} employers' insurance filings \
                    for jobs, a household estimate for workers \u{2014} put more jobs inside Allen \
                    County than the county has employed residents, in every one of the eleven years \
                    both cover. The smallest difference is 3,362 and the largest 4,449.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-commuting-2022.yml",
                "**A third pair of programmes says the same thing for eleven years running.** The county's jobs are\n  counted where they are by the employer's insurance filings and its working residents are counted\n  where they live by a household estimate, and in every year both have covered, the first number is\n  larger than the second."
            ),
            support!(
                "measure/allen-county-commuting-2022.yml",
                "**Eleven years, every difference positive, none smaller than 3,362 and none larger than 4,449.**"
            ),
            support!(
                "measure/allen-county-commuting-2022.yml",
                "**In figures, the eleven differences: 4,132 in 2014, then 4,344, 3,892, 4,065, 4,449, 3,481,\n  3,673, 3,362, 3,800, 3,895 and 3,970 in 2024.**"
            ),
            support!(
                "measure/allen-county-commuting-2022.yml",
                "**The three programmes agree on the sign and not on the size, and the reason is definitional.**\n  For 2022 the pair above puts the net at +3,800 where the table at the head of this node puts it at\n  +5,895."
            ),
            support!(
                "measure/allen-county-commuting-2022.yml",
                "**The net is small and the gross is enormous, and only the gross is a fact about commuting.**\n  Somewhere between 3,800 and 5,895 more people work here than live-and-work here, on 42,643\n  crossings of the county line in a working day."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2014", value: 4132.0, literal: "4,132" },
            Figure { label: "2015", value: 4344.0, literal: "4,344" },
            Figure { label: "2016", value: 3892.0, literal: "3,892" },
            Figure { label: "2017", value: 4065.0, literal: "4,065" },
            Figure { label: "2018", value: 4449.0, literal: "4,449" },
            Figure { label: "2019", value: 3481.0, literal: "3,481" },
            Figure { label: "2020", value: 3673.0, literal: "3,673" },
            Figure { label: "2021", value: 3362.0, literal: "3,362" },
            Figure { label: "2022", value: 3800.0, literal: "3,800" },
            Figure { label: "2023", value: 3895.0, literal: "3,895" },
            Figure { label: "2024", value: 3970.0, literal: "3,970" },
        ],
    },
    Assertion {
        id: "the-democratic-column-lost-a-third-of-itself",
        statement: "2008 is Allen County's modern high-water mark on both sides \u{2014} its largest \
                    total vote, its largest Democratic vote, and the only Republican share under \
                    sixty in the modern run. Over the eight years to 2016 the Democratic vote fell \
                    6,228, a third of itself, while the total vote fell 4,448 and the Republican \
                    vote rose 547: the whole of the turnout decline and more came out of one \
                    column.",
        topic: "elections",
        supports: &[
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**2008 is the modern high-water mark, and it is high on both sides.** It is the largest total\n  vote of the six modern elections at 50,263, the largest Democratic vote at 19,522, and the\n  Republicans' worst share at 59.6 per cent \u{2014} their only showing under sixty in the whole modern\n  run."
            ),
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**The Democratic column lost a third of itself in eight years, and it lost more than turnout\n  did.** From 2008 to 2016 the Democratic vote fell 6,228 \u{2014} 31.9 per cent \u{2014} while the total vote\n  fell 4,448. The Republican vote *rose* by 547 across the same eight years."
            ),
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**The county's Republican share is not a straight line and 2008 is the dip, not the start.** It\n  runs 65.4, 66.1, 59.6, 61.2, 66.5 and 69.0 across the six modern elections: two above\n  sixty-five, a two-election trough, and then a climb past where it began."
            ),
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**In figures, the Republican share: 65.4 per cent in 2000, 66.1 in 2004, 59.6 in 2008, 61.2 in\n  2012, 66.5 in 2016 and 69.0 in 2020.**"
            ),
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**The margin in 2020 is the largest in this corpus's whole record of this county, and the\n  largest the other way is 1,354.** R+18,967 in 2020 against Bryan's D+1,354 in 1908. Allen County\n  gave Bryan 55.2 per cent in 1896 and Biden 29.5 per cent in 2020."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2000", value: 65.4, literal: "65.4" },
            Figure { label: "2004", value: 66.1, literal: "66.1" },
            Figure { label: "2008", value: 59.6, literal: "59.6" },
            Figure { label: "2012", value: 61.2, literal: "61.2" },
            Figure { label: "2016", value: 66.5, literal: "66.5" },
            Figure { label: "2020", value: 69.0, literal: "69.0" },
        ],
    },
    Assertion {
        id: "two-compilers-one-canvass-and-one-vote",
        statement: "Two independent compilations of Ohio's certified county canvass disagree by \
                    exactly one vote in two of the five elections both cover. In 2008 the two \
                    errors cancel, so both files report the same total and no arithmetic in either \
                    can catch it. In 2012 the Secretary of State's own workbook settles it, and \
                    the compilation that matches the workbook is the volunteer transcription.",
        topic: "elections",
        supports: &[
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**Two compilations of one canvass disagree twice in five elections, each time by one vote.** For\n  2008 the lab gives McCain 29,941 and Obama 19,521 where OpenElections gives 29,940 and 19,522 \u{2014}\n  one vote each way, so both files total 50,263 and no arithmetic in either can catch it. For 2012\n  the lab gives Obama 17,913 against OpenElections' 17,914, and there the totals differ too."
            ),
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**The 2012 disagreement is settled and the lab is wrong.** The Ohio Secretary of State's own\n  final-results workbook, which OpenElections ships beside its transcription, gives Allen County\n  Obama 17,914, Romney 29,502 and a presidential total of 48,236 \u{2014} the transcription exactly."
            ),
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**The residue is where the two compilations really part.** For 2016 both give Trump 30,487 and\n  Clinton 13,294 and then differ by 79 on everything else, 2,034 against 2,113. That is a\n  disagreement about which write-ins count rather than a transcription error, and it is not read\n  here as one side being wrong."
            ),
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**2004 rests on one witness and the others on two.** OpenElections has no Ohio general-election\n  file for 2004 at all, so that row has no second reading."
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "half-the-run-and-the-hole-is-one-block",
        statement: "The corpus now holds 21 of the 42 presidential elections from 1856 to 2020 \u{2014} \
                    exactly half. The 21 it does not hold are 1912, 1916, and then every election \
                    from 1924 to 1996 without a break, with 1920 sitting inside that block as the \
                    only year read.",
        topic: "elections",
        supports: &[
            support!(
                "measure/allen-county-presidential-vote-2000-2016.yml",
                "**This closes half the run and leaves the other half in one piece.** With these five the corpus\n  holds 21 of the 42 presidential elections from 1856 to 2020. The 21 it does not hold are 1912\n  and 1916, and then every election from 1924 to 1996 without a break \u{2014} a nineteen-election block\n  with 1920 sitting inside it as the only year read."
            ),
            support!(
                "measure/allen-county-presidential-vote-1856-1884.yml",
                "**The other end of the run is now held too, and the hole between them has a shape.** Five modern\n  elections were read from two compilations of the state canvass, so the corpus holds 21 of the 42\n  presidential elections from 1856 to 2020 \u{2014} exactly half."
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "four-catholic-schools-hold-four-fifths",
        statement: "Ten private schools stand in Allen County and every one of them is in Lima. \
                    Four Catholic schools hold 880 of their 1,108 pupils \u{2014} four fifths of the \
                    county's private enrolment inside one church's system \u{2014} and the other six \
                    hold 228 between them.",
        topic: "schools",
        supports: &[
            support!(
                "measure/allen-county-private-schools-2013-2021.yml",
                "Ten private schools, all of them in Lima, and 1,108 children in them."
            ),
            support!(
                "measure/allen-county-private-schools-2013-2021.yml",
                "**Four Catholic schools hold 880 of the 1,108 \u{2014} four fifths of the county's private enrolment in\n  one church's system.** In figures: St Charles 368, Lima Central Catholic 195, St Gerard 184 and\n  St Rose 133. The remaining six are five small evangelical schools and one school for autistic and\n  dyslexic children, listed twice, and they hold 228 between them."
            ),
            support!(
                "measure/allen-county-private-schools-2013-2021.yml",
                "**Every private school in this county is in Lima.** Not one of the twelve townships, three\n  villages or the county's second city has one."
            ),
            support!(
                "measure/allen-county-private-schools-2013-2021.yml",
                "**Two community schools stand in the county and hold 313 children.** Heir Force Community School\n  with 238 and West Central Learning Academy II with 75, out of the 36 public schools located here."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "St Charles", value: 368.0, literal: "368" },
            Figure { label: "Lima Central Catholic", value: 195.0, literal: "195" },
            Figure { label: "St Gerard", value: 184.0, literal: "184" },
            Figure { label: "St Rose", value: 133.0, literal: "133" },
            Figure { label: "The other six", value: 228.0, literal: "228" },
        ],
    },
    Assertion {
        id: "the-private-school-fall-is-in-the-file",
        statement: "Allen County's recorded private enrolment falls 46.9 per cent between the \
                    2019\u{2013}20 and 2021\u{2013}22 editions of the federal survey, while Ohio's rises \
                    5.6 per cent over the same two editions. Three schools leaving the file account \
                    for 96 per cent of the county's fall, and the seven present in both net 39 \
                    pupils. The collapse is in the survey's frame, not in the county.",
        topic: "schools",
        supports: &[
            support!(
                "measure/allen-county-private-schools-2013-2021.yml",
                "**In figures, the county's recorded private enrolment: 2,338 in 2013\u{2013}14, 1,713 in 2015\u{2013}16, 1,706\n  in 2017\u{2013}18, 2,086 in 2019\u{2013}20 and 1,108 in 2021\u{2013}22.**"
            ),
            support!(
                "measure/allen-county-private-schools-2013-2021.yml",
                "[verified] \u{2014} the five public-use files. The fall from 2019\u{2013}20 to 2021\u{2013}22 is 978 pupils and 939 of\n  it \u{2014} 96 per cent \u{2014} is three schools leaving the file: Delphos St John's at 603, Temple Christian\n  School at 237 and Golden Bridge Academy at 99. The seven schools present in both editions net 39."
            ),
            support!(
                "measure/allen-county-private-schools-2013-2021.yml",
                "**Over the same two editions Ohio's private enrolment rose.** 145,882 to 154,033, up 5.6 per\n  cent, while this county's fell 46.9 per cent. Statewide, 185 schools leave the file and 215\n  arrive and the churn cancels; in one county nothing cancels."
            ),
            support!(
                "measure/allen-county-private-schools-2013-2021.yml",
                "**Two schools this county has are not in the newest file at all.** Delphos St John's, at 603 the\n  largest private school ever recorded here, and Temple Christian School in Lima at 237, are absent\n  from Ohio's entire 2021\u{2013}22 return under any spelling."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2013\u{2013}14", value: 2338.0, literal: "2,338" },
            Figure { label: "2015\u{2013}16", value: 1713.0, literal: "1,713" },
            Figure { label: "2017\u{2013}18", value: 1706.0, literal: "1,706" },
            Figure { label: "2019\u{2013}20", value: 2086.0, literal: "2,086" },
            Figure { label: "2021\u{2013}22", value: 1108.0, literal: "1,108" },
        ],
    },
    Assertion {
        id: "fifth-worst-in-ohio-for-heart-disease",
        statement: "Allen County's heart disease death rate rose 17.7 per cent across four \
                    vintages of one federal measure while Ohio's fell 2.7 per cent. The county \
                    went from 11.4 per cent above the state to 34.7 per cent above it, and from \
                    the 21st worst county in Ohio to the 5th worst of 88.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**Allen County is now the fifth worst county in Ohio for heart disease.** [verified] \u{2014}\n  [the 2022\u{2013}2024 vintage](../../catalog/cdc-heart-disease-stroke-county.md), ranked here over\n  Ohio's 88 counties."
            ),
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**The county's rate rose 17.7 per cent across the four while Ohio's\n  fell 2.7 per cent, and the county went from 11.4 per cent above the state to 34.7 per cent\n  above.**"
            ),
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**In figures, the county's heart disease rate per 100,000 at ages 35 and over: 415.0 in 2018\u{2013}2020,\n  428.6 in 2019\u{2013}2021, 465.2 in 2021\u{2013}2023 and 488.4 in 2022\u{2013}2024, against Ohio's 372.5, 380.8, 375.6\n  and 362.6.**"
            ),
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**Stroke went the other way, and the county is now seventh best in Ohio.** 74.0 per 100,000 in\n  the 2013\u{2013}2015 vintage against 71.3 in 2022\u{2013}2024, while Ohio moved from 78.4 to 91.1 \u{2014} so the\n  county fell from rank 58 of 88 to rank 82."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2018\u{2013}2020", value: 415.0, literal: "415.0" },
            Figure { label: "2019\u{2013}2021", value: 428.6, literal: "428.6" },
            Figure { label: "2021\u{2013}2023", value: 465.2, literal: "465.2" },
            Figure { label: "2022\u{2013}2024", value: 488.4, literal: "488.4" },
        ],
    },
    Assertion {
        id: "the-improvement-stopped-at-working-age",
        statement: "Cardiovascular death rates among Allen County residents aged 35 to 64 fell \
                    29.8 per cent from 1999 to 2010 and then rose 15.3 per cent to 2019. Among \
                    those 65 and over the same rates fell 35.1 per cent and went on falling. The \
                    county's old people kept improving and its working-age people stopped \u{2014} and \
                    so did those of 77 of Ohio's 88 counties.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**The improvement stopped at working age, and it stopped in 2010.** For cardiovascular disease\n  among people aged 35 to 64, the programme's own fitted trend is **\u{2212}29.8 per cent from 1999 to\n  2010 and +15.3 per cent from 2010 to 2019**. For the same disease among people 65 and over it is\n  \u{2212}35.1 per cent and then \u{2212}2.8 per cent."
            ),
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**All heart disease, ages 35 to 64, is the sharpest version of it:** \u{2212}31.3 per cent to 2010, then\n  **+16.6 per cent** \u{2014} a rate of 99.4 per 100,000 in 2010 and 112.4 in 2019, undoing eight of the\n  eleven years of gains."
            ),
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**That reversal is not this county's own.** Seventy-seven of Ohio's 88 counties show a rising\n  working-age cardiovascular rate from 2010 to 2019, and Allen ranks 22nd of the 88 on the size of\n  the rise and 37th of 88 on the 2019 level."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "35\u{2013}64, 1999\u{2013}2010", value: -29.8, literal: "\u{2212}29.8" },
            Figure { label: "35\u{2013}64, 2010\u{2013}2019", value: 15.3, literal: "15.3" },
            Figure { label: "65+, 1999\u{2013}2010", value: -35.1, literal: "\u{2212}35.1" },
            Figure { label: "65+, 2010\u{2013}2019", value: -2.8, literal: "\u{2212}2.8" },
        ],
    },
    Assertion {
        id: "middling-on-hearts-worst-fifth-on-strokes",
        statement: "Measured every five years for twenty, Allen County sits in the middle of \
                    Ohio for working-age heart disease deaths and in the worst fifth for \
                    working-age strokes \u{2014} ranks of 38th, 48th, 43rd and 39th against 15th, \
                    14th, 16th and 16th of 88. Its Black-to-white ratio for those deaths is large \
                    and ranks in the middle of Ohio, so the county's unusually wide racial gap in \
                    life expectancy is not a cardiovascular gap.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**Where the county is genuinely unusual is stroke at working age, and it has been for twenty\n  years.** Its rank among Ohio's 88 counties for stroke deaths at ages 35 to 64 was 15th worst in\n  2005, 14th in 2010, 16th in 2015 and 16th in 2019, while its rank for heart disease over the same\n  four years ran 38th, 48th, 43rd and 39th."
            ),
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**The racial gap is large and it is ordinary for Ohio.** Among 35-to-64-year-olds in 2019, Black\n  residents' cardiovascular death rate is 208.2 against 131.1 for white \u{2014} a ratio of 1.59 \u{2014} and for\n  stroke 33.6 against 14.9, a ratio of 2.26."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Black, cardiovascular", value: 208.2, literal: "208.2" },
            Figure { label: "White, cardiovascular", value: 131.1, literal: "131.1" },
            Figure { label: "Black, stroke", value: 33.6, literal: "33.6" },
            Figure { label: "White, stroke", value: 14.9, literal: "14.9" },
        ],
    },
    Assertion {
        id: "the-county-kept-making-things",
        statement: "Production is still Allen County's largest occupational group and its most \
                    concentrated: 5,920 jobs at 2.10 times the national share, against 6,010 at \
                    1.84 ten years earlier. The jobs held roughly still while the country's fell \
                    away beneath them, on a county total that moved 0.8 per cent in the decade.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**The county still makes things, and relative to America it makes more of them than it did.**\n  Production occupations are its largest major group at 5,920 and its most concentrated at a\n  location quotient of 2.10 \u{2014} twice the national share of employment. Ten years earlier the count\n  was 6,010 and the quotient 1.84. The jobs held roughly still while the country's fell away\n  beneath them."
            ),
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**Total employment did not move in a decade: 49,260 in 2014 and 49,640 in 2024, a rise of 380 or\n  0.8 per cent.** The median wage went from $31,450 to $46,430, up 47.6 per cent before any\n  allowance for prices."
            ),
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**In figures, the seven largest major groups in 2024: production 5,920, food preparation 4,970,\n  transportation 4,900, office and administrative support 4,820, healthcare practitioners 4,450,\n  sales 4,170 and management 2,950.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Production", value: 5920.0, literal: "5,920" },
            Figure { label: "Food preparation", value: 4970.0, literal: "4,970" },
            Figure { label: "Transportation", value: 4900.0, literal: "4,900" },
            Figure { label: "Office and admin", value: 4820.0, literal: "4,820" },
            Figure { label: "Healthcare practitioners", value: 4450.0, literal: "4,450" },
            Figure { label: "Sales", value: 4170.0, literal: "4,170" },
            Figure { label: "Management", value: 2950.0, literal: "2,950" },
        ],
    },
    Assertion {
        id: "the-fingerprint-names-the-landmarks",
        statement: "Allen County's seven most concentrated occupations are chemical equipment \
                    operators at 5.58 times the national share, machine tool setters at 5.02, food \
                    batchmakers at 4.90, computer-controlled tool operators at 3.63, correctional \
                    officers at 2.98, welders at 2.93 and industrial engineers at 2.61. The first \
                    is the refinery and the fifth is the state's two prisons: a survey that has \
                    never heard of either draws them anyway.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**The county's occupational fingerprint names its landmarks.** The seven most concentrated\n  occupations with 200 or more workers are chemical equipment operators at a location quotient of\n  5.58, multiple machine tool setters at 5.02, food batchmakers at 4.90, computer-controlled tool\n  operators at 3.63, correctional officers at 2.98, welders at 2.93 and industrial engineers at\n  2.61."
            ),
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**The occupations this county is shortest of are the ones the country grew.** Computer and\n  mathematical work has a location quotient of 0.33 on 550 jobs \u{2014} one third the national rate and\n  the lowest of the 22 major groups. Legal is 0.38, farming 0.41 and business and financial\n  operations 0.57."
            ),
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**Sixty people work in farming, fishing and forestry, in a county two thirds covered by farms.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Chemical equipment operators", value: 5.58, literal: "5.58" },
            Figure { label: "Machine tool setters", value: 5.02, literal: "5.02" },
            Figure { label: "Food batchmakers", value: 4.9, literal: "4.90" },
            Figure { label: "CNC tool operators", value: 3.63, literal: "3.63" },
            Figure { label: "Correctional officers", value: 2.98, literal: "2.98" },
            Figure { label: "Welders", value: 2.93, literal: "2.93" },
            Figure { label: "Industrial engineers", value: 2.61, literal: "2.61" },
        ],
    },
    Assertion {
        id: "the-clerks-went-and-the-warehouse-did-not-arrive",
        statement: "Read straight off the major groups, Allen County lost 2,520 clerical jobs and \
                    gained 1,090 in transportation over ten years. But 920 of those people never \
                    changed jobs: the code they are filed under moved between the two groups in \
                    the 2018 revision. Repaired, the clerical fall is 1,600 and the warehouse rise \
                    is 170.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**Underneath the flat total, the largest single change is clerical and it is smaller than it\n  looks.** Office and administrative support reads 7,340 in 2014 and 4,820 in 2024, a fall of\n  2,520. But `43-5081 Stock Clerks and Order Fillers`, 920 people here in 2014, was moved by the\n  2018 revision of the occupation codes into transportation and material moving, where it appears\n  as `53-7065 Stockers and Order Fillers` with 1,100."
            ),
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**Repaired for that one move, the clerical fall is about a quarter and the warehouse rise is\n  not a rise.** On a comparable base office and administrative support goes 6,420 to 4,820, down\n  1,600 or 24.9 per cent, and transportation and material moving goes 4,730 to 4,900, up 170 or\n  3.6 per cent."
            ),
            support!(
                "measure/allen-county-occupations-2014-2024.yml",
                "**In figures, the two groups as read and as repaired: office and administrative support falls by\n  2,520 as read and by 1,600 repaired; transportation and material moving rises by 1,090 as read\n  and by 170 repaired.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Clerical fall, as read", value: 2520.0, literal: "2,520" },
            Figure { label: "Clerical fall, repaired", value: 1600.0, literal: "1,600" },
            Figure { label: "Transport rise, as read", value: 1090.0, literal: "1,090" },
            Figure { label: "Transport rise, repaired", value: 170.0, literal: "170" },
        ],
    },
    // ── Environment ──
    Assertion {
        id: "the-county-leads-ohio-in-what-it-releases",
        statement: "Allen County's facilities reported releasing more listed toxic chemical in \
                    2024 than those of any other county in Ohio: 9,417,778 pounds, 18.0 per cent \
                    of the state's total, on 0.85 per cent of its people. Two thirds of it went \
                    down a deep injection well, a method only one other Ohio county used at all \
                    that year; set the wells aside and the county is fifth.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-toxic-releases-1987-2024.yml",
                "**Allen County released more listed toxic chemical in 2024 than any other county in Ohio.**\n  9,417,778 pounds, 18.0 per cent of the state's 52.2 million \u{2014} ahead of Ashtabula's 6.2 million\n  and Cuyahoga's 4.7 million, from 28 and 122 reporting facilities against this county's 17.\n  [verified] \u{2014} same dataset, the whole state for 2024, summed here by county. The county holds\n  0.85 per cent of Ohio's people."
            ),
            support!(
                "measure/allen-county-toxic-releases-1987-2024.yml",
                "**That first place is made by a disposal method almost nobody else in Ohio uses.** 6.4 million of\n  the 9.4 million went down a deep injection well, and only two counties in the state injected\n  anything at all in 2024 \u{2014} this one and Sandusky. Set the wells aside and Allen is fifth, behind\n  Ashtabula, Gallia, Cuyahoga and Coshocton. [verified] \u{2014} same source, `UNINJ` media codes."
            ),
        ],
        answers: &["cannot say that the fall in reported releases caused"],
        figures: &[
            Figure { label: "Allen", value: 9.4, literal: "9.4" },
            Figure { label: "Ashtabula", value: 6.2, literal: "6.2" },
            Figure { label: "Cuyahoga", value: 4.7, literal: "4.7" },
        ],
    },
    Assertion {
        id: "a-fifth-of-the-fall-is-the-list-changing",
        statement: "Reported toxic releases in Allen County fall 86.5 per cent between 1987 and \
                    2024. Restricted to the 85 chemicals reportable in every year of the series, \
                    the fall is 64.2 per cent \u{2014} the difference is one chemical that left the \
                    list. The air stream needs no such repair: 69.8 per cent raw against 69.9 \
                    like-for-like.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-toxic-releases-1987-2024.yml",
                "**The headline fall is 86.5 per cent and the honest one is 64.2.** From 69,836,754 pounds in 1987\n  to 9,417,778 in 2024 on the raw totals; from 25,249,928 to 9,047,255 on the 85 chemicals\n  reportable throughout. The gap is one chemical: ammonium sulfate (solution), 126 million pounds\n  reported here in three years, off the list since 1993. [verified] \u{2014} same source, against the\n  chemical dictionary's `active_date` and `inactive_date`. See\n  [a revision that changes the roll](../../decisions/a-revision-that-changes-the-roll.yml)."
            ),
            support!(
                "measure/allen-county-toxic-releases-1987-2024.yml",
                "**The air series needs no such repair.** Air releases fall from 9,392,220 pounds to 2,836,097, a\n  drop of 69.8 per cent raw and 69.9 per cent like-for-like \u{2014} the delisted chemicals went almost\n  entirely down the wells, so they never sat in this stream. [verified] \u{2014} same dataset, the `AIR\n  FUG` and `AIR STACK` media codes."
            ),
            support!(
                "measure/allen-county-toxic-releases-1987-2024.yml",
                "**In figures, the four largest single-year totals and the four smallest**: 69,837 thousand pounds\n  in 1987, 59,213 in 1988, 58,030 in 1989 and 28,715 in 1991; 8,637 in 2023, 9,418 in 2024, 10,228\n  in 2020 and 10,630 in 2022. [inference] \u{2014} read from the table above."
            ),
        ],
        answers: &["cannot say that the fall in reported releases caused"],
        figures: &[
            Figure { label: "Fall, as read", value: 86.5, literal: "86.5" },
            Figure { label: "Fall, like-for-like", value: 64.2, literal: "64.2" },
            Figure { label: "Fall, air only", value: 69.8, literal: "69.8" },
        ],
    },
    Assertion {
        id: "what-goes-to-the-air-fell-and-the-wells-did-not",
        statement: "Carcinogenic releases to the air over Allen County fell 91.7 per cent between \
                    1987 and 2024. Carcinogenic releases into its injection wells fell 9.9 per \
                    cent and have been flat for two decades. The air stream also changed hands: \
                    the nitriles plant was 88.3 per cent of it and a fertiliser works is 87.7 per \
                    cent of it now.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-toxic-releases-1987-2024.yml",
                "**What goes into the air fell and what goes down the well did not.** Carcinogenic releases to air\n  fall from 546,216 pounds in 1987 to 45,369 in 2024, down 91.7 per cent. Carcinogenic releases to\n  injection wells go from 2,160,000 to 1,947,066, down 9.9 per cent, and average 2.59 million a\n  year over the first decade against 2.08 million over the last. [verified] \u{2014} same dataset,\n  `carc_ind` in the chemical dictionary."
            ),
            support!(
                "measure/allen-county-toxic-releases-1987-2024.yml",
                "**The air stream changed hands.** In 1987 the nitriles plant put 8,295,100 pounds into the air,\n  88.3 per cent of the county's total; in 2024 it put out 77,227. The largest air emitter now is\n  PCS Nitrogen at 2,485,957 pounds, 87.7 per cent of a much smaller total, and almost all of it\n  ammonia. One plant cut its air releases by ninety-nine per cent and a fertiliser works became the\n  county's chimney. [verified] \u{2014} same source, by facility."
            ),
        ],
        answers: &["cannot say that the fall in reported releases caused"],
        figures: &[
            Figure { label: "Carcinogens to air, fall", value: 91.7, literal: "91.7" },
            Figure { label: "Carcinogens to wells, fall", value: 9.9, literal: "9.9" },
        ],
    },
    Assertion {
        id: "what-ended-was-the-bad-day",
        statement: "Allen County's median air-quality day is where it was in 1987 \u{2014} AQI 44 \
                    then, 42 now. Its 90th percentile went from 112 to 58, and days unhealthy for \
                    sensitive groups from 41 in 1998 to none in four of the last six years. What \
                    improved over forty-five years of measurement was the bad day, not the \
                    ordinary one.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-air-quality-1980-2024.yml",
                "**The improvement is large and it is in the tail, not the middle.** The median day was AQI 44 in\n  1987 and 42 in 2024, which is barely a change. The 90th percentile went from 112 to 58, and days\n  at unhealthy for sensitive groups or worse from 32 to none. What ended was the bad day.\n  [verified] \u{2014} same file, `Median AQI` and `90th Percentile AQI`."
            ),
            support!(
                "measure/allen-county-air-quality-1980-2024.yml",
                "**Nineteen ninety-eight was the worst year in the record and 41 of its days were unhealthy for\n  somebody.** 17.3 per cent of the days monitored \u{2014} against 15.5 per cent in 1991, 16.4 in 1994 and\n  15.1 in both 1987 and 2002. No year since 2012 has passed 2.5 per cent. [verified] \u{2014} same source,\n  the share column above."
            ),
            support!(
                "measure/allen-county-air-quality-1980-2024.yml",
                "**In figures, the 90th-percentile AQI at five points in the record: 100 in 1990, 74 in 2000, 83\n  in 2010, 61 in 2019 and 58 in 2024.** [verified] \u{2014} read from the table above, same file."
            ),
        ],
        answers: &["does not know what happened that"],
        figures: &[
            Figure { label: "1990", value: 100.0, literal: "100" },
            Figure { label: "2000", value: 74.0, literal: "74" },
            Figure { label: "2010", value: 83.0, literal: "83" },
            Figure { label: "2019", value: 61.0, literal: "61" },
            Figure { label: "2024", value: 58.0, literal: "58" },
        ],
    },
    // ── The property tax ──
    Assertion {
        id: "what-a-mill-is-worth-here",
        statement: "One mill on Allen County's taxable property raises $2,451,332. The base is \
                    $2,451,331,690 in the newest audited report, up 31.6 per cent since the 2010 \
                    report, and 62.7 per cent of it is residential property. Agricultural value \
                    went the other way, down 24.2 per cent since 2015.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-assessed-valuation-2010-2023.yml",
                "**The county's taxable base was $2,451,331,690 in the newest audited report, and one mill on it\n  raises $2,451,332.** [verified] \u{2014}\n  [the audited financial statements](../../catalog/allen-county-auditor-financials.md), the\n  *Property Taxes* note in the report for the year ended 31 December 2023."
            ),
            support!(
                "measure/allen-county-assessed-valuation-2010-2023.yml",
                "**Residential property is 62.7 per cent of what this county taxes.** $1,537,113,750 of\n  $2,451,331,690 in the 2023 report, against $474,900,790 commercial, industrial and mineral,\n  $257,374,000 public utility and $181,943,150 agricultural. [verified] \u{2014} same source. In every year\n  that prints the five classes, they sum to the printed total exactly."
            ),
            support!(
                "measure/allen-county-assessed-valuation-2010-2023.yml",
                "**Agricultural value fell by a quarter while everything else rose.** From $240,015,570 in the 2015\n  report to $181,943,150 in the 2023 report, down 24.2 per cent, in the same years residential rose\n  33.7 per cent. Agricultural land here is taxed on what it yields rather than what it would sell\n  for, under the current agricultural use valuation the Auditor administers. [verified] \u{2014} same\n  source; see [the Auditor](../office/allen-county-auditor.yml)."
            ),
            support!(
                "measure/allen-county-assessed-valuation-2010-2023.yml",
                "**In figures, the total assessed base at four reports: $1,862.8 million in 2010, $1,941.9 million\n  in 2015, $2,153.9 million in 2021 and $2,451.3 million in 2023.** [verified] \u{2014} read from the table\n  above, same source."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2010", value: 1862.8, literal: "1,862.8" },
            Figure { label: "2015", value: 1941.9, literal: "1,941.9" },
            Figure { label: "2021", value: 2153.9, literal: "2,153.9" },
            Figure { label: "2023", value: 2451.3, literal: "2,451.3" },
        ],
    },
    Assertion {
        id: "the-column-changed-and-the-rate-did-not",
        statement: "The county column in Allen County's tax rate summaries reads 6.150 mills \
                    through tax year 2015 and 11.400 from 2016, and nothing was levied: five mills \
                    moved out of the neighbouring column when the report changed layout. What the \
                    county actually charged went $8.70, $9.70, $11.40 \u{2014} a third, not four \
                    fifths.",
        topic: "government",
        supports: &[
            support!(
                "measure/allen-county-property-tax-rates-2012-2025.yml",
                "**The county column is not a series.** It reads 6.150 in every tax set from 2012 through 2015 and\n  11.400 in every tax set from 2016, and across that boundary the total full rate of the\n  thirty-six districts present in both years moves by between \u{2212}1.710 and +0.700 mills, mean \u{2212}0.332.\n  Five mills moved out of *Library/Other* and into *County* when the report changed layout.\n  [verified] \u{2014} same files, differenced by tax set. See\n  [a rule written for a classification caught a layout](../../decisions/a-rule-written-for-a-classification-caught-a-layout.yml)."
            ),
            support!(
                "measure/allen-county-property-tax-rates-2012-2025.yml",
                "**What the county actually charges is in a different document, and it rose by a third.** The\n  audited statements put the full rate for all county operations at $8.70 per $1,000 for the reports\n  of 2010 to 2013, $9.70 for 2014 and 2015, and $11.40 from 2017 to 2023. [verified] \u{2014}\n  [the audited financial statements](../../catalog/allen-county-auditor-financials.md), the\n  *Property Taxes* note in each; see\n  [the tax base](allen-county-assessed-valuation-2010-2023.yml)."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Column, to 2015", value: 6.15, literal: "6.150" },
            Figure { label: "Column, from 2016", value: 11.4, literal: "11.400" },
            Figure { label: "Audited, to 2013", value: 8.7, literal: "8.70" },
            Figure { label: "Audited, 2014\u{2013}15", value: 9.7, literal: "9.70" },
            Figure { label: "Audited, from 2017", value: 11.4, literal: "11.40" },
        ],
    },
    Assertion {
        id: "the-school-district-is-the-tax-bill",
        statement: "School millage is between 50.1 and 73.2 per cent of the full tax rate in every \
                    one of Allen County's thirty-six taxing districts. And the district that votes \
                    most is not the one that pays most: Delphos City S.D. votes 70.250 mills and \
                    collects 34.460, where Bath votes 51.497 and collects 44.322.",
        topic: "schools",
        supports: &[
            support!(
                "measure/allen-county-property-tax-rates-2012-2025.yml",
                "**The school district is most of the bill everywhere.** Local school plus joint vocational millage\n  is between 50.1 and 73.2 per cent of the full rate in every one of the thirty-six districts.\n  [inference] \u{2014} computed here from the 2025 table."
            ),
            support!(
                "measure/allen-county-property-tax-rates-2012-2025.yml",
                "**The highest and lowest bills are not the highest and lowest votes.** In 2025 Shawnee L35 has\n  both the highest full rate at 71.974 and the highest effective rate at 54.233. But Delphos City\n  S.D. votes 70.250 mills and collects 34.460, half of it reduced away, while Bath L.S.D. votes\n  51.497 and collects 44.322. The lowest effective rate in the county is Pandora-Gilboa's 30.562 on\n  a full rate of 52.750. [verified] \u{2014} same source."
            ),
            support!(
                "measure/allen-county-property-tax-rates-2012-2025.yml",
                "**In figures, the 2025 effective residential rates at four points: 54.233 in Shawnee L35, 44.322\n  in Bath, 34.460 in Delphos City S.D. and 30.562 in Pandora-Gilboa.** [verified] \u{2014} read from the\n  table below, same source."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Shawnee L35", value: 54.233, literal: "54.233" },
            Figure { label: "Bath", value: 44.322, literal: "44.322" },
            Figure { label: "Delphos City", value: 34.46, literal: "34.460" },
            Figure { label: "Pandora-Gilboa", value: 30.562, literal: "30.562" },
        ],
    },
    Assertion {
        id: "the-village-stopped-levying-and-the-township-took-it",
        statement: "Allen County's tax rate summary for 2012 has the Village of Fort Shawnee \
                    levying 2.150 mills and its residents paying 10.200 mills of township tax \
                    against their neighbours' 15.600. The 2013 file has no such district, and when \
                    the label returns it charges the full township rate. A county office and a \
                    federal map end the village in the same twelve months.",
        topic: "government",
        supports: &[
            support!(
                "question/what-happened-to-the-village-of-fort-shawnee.yml",
                "**A county taxing authority puts the change in the same year, and it is not a federal product.**\n  The Auditor's tax rate summary for tax year 2012 carries tax set L36, *Fort Shawnee Corp.*, levying\n  2.150 mills of village tax, and charging its residents 10.200 mills of township tax where the rest\n  of Shawnee Township pays 15.600. The tax year 2013 summary has no such set. In 2014 and 2015 the\n  set reappears with no village millage and a township rate of 15.600 \u{2014} identical to the township's\n  own set in every one of its columns and to the third decimal \u{2014} and from tax year 2016 it is gone.\n  [verified] \u{2014}\n  [the Auditor's tax rate summaries](../../catalog/allen-county-auditor-tax-rates.md), tax sets L35\n  and L36; see [the rates](../measure/allen-county-property-tax-rates-2012-2025.yml)."
            ),
            support!(
                "question/what-happened-to-the-village-of-fort-shawnee.yml",
                "**That is a second witness to the year and a first witness that is local.** The gazetteer draws\n  Fort Shawnee as a village in 2012 and as a census designated place in 2013; the office that sets\n  this county's tax rates has it levying in 2012 and not in 2013. Two instruments with nothing in\n  common \u{2014} a federal geography file and a county taxing authority \u{2014} put the end of the corporation\n  in the same twelve months. [inference] \u{2014} the reasoning is this corpus's."
            ),
            support!(
                "question/what-happened-to-the-village-of-fort-shawnee.yml",
                "**The tax set outliving the levy is the sharper detail.** A village that had merely stopped\n  levying would keep a lower township rate, because a village's residents are outside some township\n  levies; L36's township rate rises to the full 15.600 in the same move. The set that survives into\n  2014 and 2015 is a label on rows identical to the township's, which is what an administrative\n  record looks like after the thing it names has gone. [inference]"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Village tax, 2012", value: 2.15, literal: "2.150" },
            Figure { label: "Township tax, 2012", value: 10.2, literal: "10.200" },
            Figure { label: "Township tax, 2014", value: 15.6, literal: "15.600" },
        ],
    },
    // ── Cancer ──
    Assertion {
        id: "cancer-is-the-ordinary-half",
        statement: "Cancer kills about 220 people a year in Allen County, at 159.0 per 100,000 \
                    against Ohio's 160.3 and America's 145.4, and the rate is falling. The county \
                    ranks 62nd of Ohio's 88 counties \u{2014} the lower middle \u{2014} where for \
                    heart disease it is fifth at 1.35 times the state rate.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-cancer-2018-2023.yml",
                "**Cancer kills about 220 people a year in Allen County, at 159.0 per 100,000 against Ohio's 160.3\n  and America's 145.4 \u{2014} and the rate is falling.** [verified] \u{2014}\n  [State Cancer Profiles](../../catalog/nci-state-cancer-profiles.md), all sites, both sexes,\n  age-adjusted deaths 2019\u{2013}2023."
            ),
            support!(
                "measure/allen-county-cancer-2018-2023.yml",
                "**This is not what the county dies of unusually.** Its all-sites cancer death rate is within a\n  point of Ohio's and it ranks 62nd of the state's 88 counties \u{2014} the lower middle. Set against the\n  same corpus's finding that this county is fifth of 88 for heart disease at 1.35 times the state\n  rate, cancer is the ordinary half of its mortality. [inference] \u{2014} computed here against\n  [heart disease and stroke](allen-county-heart-disease-and-stroke-1999-2024.yml)."
            ),
            support!(
                "measure/allen-county-cancer-2018-2023.yml",
                "**In figures, the four sites the county loses most people to each year: lung and bronchus 59,\n  pancreas 19, colon and rectum 16 and breast 12.** [verified] \u{2014} read from the table above, same\n  source."
            ),
        ],
        answers: &["does not assert one"],
        figures: &[
            Figure { label: "Lung and bronchus", value: 59.0, literal: "59" },
            Figure { label: "Pancreas", value: 19.0, literal: "19" },
            Figure { label: "Colon and rectum", value: 16.0, literal: "16" },
            Figure { label: "Breast", value: 12.0, literal: "12" },
        ],
    },
    Assertion {
        id: "the-cancer-burden-is-a-mens-burden",
        statement: "Allen County men die of cancer at 201.9 per 100,000, above Ohio's 192.3 and 18 \
                    per cent above America's 171.5. Its women die at 129.6, below Ohio's 137.4. \
                    124 of the county's 220 annual cancer deaths are men's, and the sites it \
                    exceeds the nation on are lung, oesophagus and pancreas.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-cancer-2018-2023.yml",
                "**The burden is a men's burden.** Allen County men die of cancer at 201.9 per 100,000 against\n  Ohio's 192.3 and America's 171.5 \u{2014} 18 per cent above the national rate, and 43rd of 88. Its women\n  die at 129.6 against Ohio's 137.4 and America's 126.3, below the state and 74th of 88. [verified]\n  \u{2014} same source, sex-specific all-sites rates. 124 of the county's 220 annual cancer deaths are\n  men's."
            ),
            support!(
                "measure/allen-county-cancer-2018-2023.yml",
                "**The sites the county exceeds the nation on are few and they are alike.** Lung and bronchus at\n  41.7 against 31.5, oesophagus at 5.6 against 3.7, pancreas at 13.7 against 11.3 \u{2014} the three\n  largest excesses over the national rate. It is *below* the nation on colon and rectum, breast,\n  liver and leukemia, and below Ohio on nine of the fifteen sites it has a published rate for.\n  [inference] \u{2014} computed here from the table above."
            ),
        ],
        answers: &["does not assert one"],
        figures: &[
            Figure { label: "Allen men", value: 201.9, literal: "201.9" },
            Figure { label: "Ohio men", value: 192.3, literal: "192.3" },
            Figure { label: "Allen women", value: 129.6, literal: "129.6" },
            Figure { label: "Ohio women", value: 137.4, literal: "137.4" },
        ],
    },
    Assertion {
        id: "a-county-rank-comes-with-an-interval",
        statement: "Allen County's cancer death rank of 62nd out of 88 carries a 95 per cent \
                    interval of 29th to 80th, and its pancreas rank of 16th an interval of 2nd to \
                    73rd. On nineteen deaths a year a rank is barely a statement, and this corpus \
                    has published ranks it computed itself with no interval at all.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-cancer-2018-2023.yml",
                "**The rank carries an interval and it is wide.** 62nd of 88 has a 95 per cent interval of 29th to\n  80th; the pancreas rank of 16th has an interval of 2nd to 73rd. On nineteen deaths a year a rank\n  is barely a statement. [verified] \u{2014} same source, the `CI*Rank` columns. See\n  [a rank is an estimate](../../decisions/a-rank-is-an-estimate.yml)."
            ),
            support!(
                "measure/allen-county-heart-disease-and-stroke-1999-2024.yml",
                "**The ratio is the firmer half of that sentence and the rank is the softer.** 1.347 times the\n  state rate rests on two estimates; fifth of 88 rests on eighty-eight of them, sorted, with no\n  interval carried into the sort \u{2014} and a source that does publish rank intervals puts a comparable\n  county rank's 95 per cent bounds tens of places wide. The rank here is a point estimate of a rank\n  and is not withdrawn; it is stated as one. [inference] \u{2014} see\n  [a rank is an estimate](../../decisions/a-rank-is-an-estimate.yml) and\n  [cancer](allen-county-cancer-2018-2023.yml)."
            ),
        ],
        answers: &["does not assert one"],
        figures: &[],
    },
    Assertion {
        id: "cancer-is-not-where-the-racial-gap-lives",
        statement: "Black residents of Allen County die of cancer at 169.1 per 100,000 and white \
                    residents at 160.8 \u{2014} a gap of 8.3 points, where Ohio's is 11.6 and the \
                    nation's 15.3. The county's life-expectancy gap by race is the widest of the \
                    three, so whatever makes it wide, cancer is not it.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-cancer-2018-2023.yml",
                "**Cancer is not where this county's racial mortality gap lives.** Its Black residents die of\n  cancer at 169.1 per 100,000 and its white residents at 160.8, a gap of 8.3 points; in Ohio the\n  same gap is 11.6 and nationally 15.3. Both of this county's figures are below Ohio's for the same\n  group. [verified] \u{2014} same source, non-Hispanic race categories. The corpus separately holds a\n  life-expectancy gap of 6.91 years here against 5.48 in Ohio and 5.08 nationally \u{2014} the widest of\n  the three \u{2014} so whatever makes that gap unusually wide, it is not this. [inference] \u{2014} against\n  [what the county dies of early](allen-county-early-deaths-by-cause-2020-2022.yml)."
            ),
        ],
        answers: &["does not assert one"],
        figures: &[
            Figure { label: "Allen County", value: 8.3, literal: "8.3" },
            Figure { label: "Ohio", value: 11.6, literal: "11.6" },
            Figure { label: "United States", value: 15.3, literal: "15.3" },
        ],
    },
    // ── The ground itself ──
    Assertion {
        id: "what-the-farmland-became",
        statement: "Seventy per cent of Allen County is cropland and pasture, 17.8 per cent is \
                    developed and 10.4 per cent is forest. The county was 92.5 per cent farmland in \
                    1910; what the missing third became is built ground and woodland, in that \
                    order, and there is more than twice as much of the first.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**Seventy per cent of Allen County is cropland and pasture, eighteen per cent is developed and\n  ten per cent is forest.** [verified] \u{2014}\n  [the Cropland Data Layer](../../catalog/usda-cropscape-cdl.md), 2024, every thirty-metre pixel in\n  the county classified and counted."
            ),
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**The corpus knew the county was 92.5 per cent farmland in 1910 and 69.4 per cent in 2022, and\n  had nothing that said what the other thirty per cent had become.** It is developed ground and\n  woodland, in that order, and there is more than twice as much of the first as of the second.\n  [inference] \u{2014} computed here against\n  [land in farms](allen-county-farmland-1910-2022.yml); see\n  [the question](../question/when-the-farmland-went.yml)."
            ),
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**Two instruments land within a point of each other on how much of the county is farmed.** This\n  file puts crops and pasture at 70.3 per cent in 2024; the Census of Agriculture puts land in farms\n  at 69.4 per cent in 2022. One counts pixels of growing things and the other counts acres a farm\n  operator says they operate, so they are not the same quantity, and they agree anyway. [inference]\n  \u{2014} computed here against\n  [the census of agriculture](../../catalog/usda-census-of-agriculture.md)."
            ),
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**In figures, the county's ground in 2024: 183,047 acres of crops and pasture, 46,307 developed,\n  27,037 forest and 3,076 water.** [verified] \u{2014} read from the table above, same source."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Crops and pasture", value: 183047.0, literal: "183,047" },
            Figure { label: "Developed", value: 46307.0, literal: "46,307" },
            Figure { label: "Forest", value: 27037.0, literal: "27,037" },
            Figure { label: "Water", value: 3076.0, literal: "3,076" },
        ],
    },
    Assertion {
        id: "the-spring-the-county-was-not-planted",
        statement: "In 2019 one acre in five of Allen County's cropland was classified fallow \
                    \u{2014} 37,726 acres against a median of 55 \u{2014} with corn 25,865 acres \
                    below its neighbouring years. April to June that year brought 19.21 inches of \
                    rain against a long-run mean of 11.09, the third wettest planting season in a \
                    hundred and thirty-one.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**In 2019 one acre in five of this county's cropland was not planted.** `Fallow/Idle Cropland`\n  holds 37,726 acres, against a median of 55 across the other sixteen years and a maximum elsewhere\n  of 999. Corn fell 25,865 acres below the mean of the years either side and soybeans 7,854.\n  [verified] \u{2014} same source."
            ),
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**The county's own rain record puts that year third wettest in a hundred and thirty-one.** April\n  to June 2019 brought 19.21 inches against a long-run mean of 11.09 \u{2014} behind only 2015 and 1957 \u{2014}\n  and May alone brought 7.38, the third wettest May of the record. [verified] \u{2014}\n  [NOAA nClimDiv](../../catalog/noaa-nclimdiv-county.md), county precipitation, ranked here over the\n  131 complete years; see [the precipitation record](allen-county-precipitation-1895-2025.yml)."
            ),
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**Two instruments with nothing in common say the same thing about one spring.** A satellite\n  classifier that has never heard of a weather station, and a gridded rain record that has never\n  seen a field, put the unplanted ground and the rain in the same twelve months. [inference] \u{2014}\n  the reasoning is this corpus's. Neither file records a farmer's decision, and the step from\n  *wet* to *not planted* is not in either of them."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Acres fallow, 2019", value: 37726.0, literal: "37,726" },
            Figure { label: "Corn shortfall", value: 25865.0, literal: "25,865" },
            Figure { label: "Soybean shortfall", value: 7854.0, literal: "7,854" },
        ],
    },
    Assertion {
        id: "half-a-file-can-be-differenced",
        statement: "The same file that catches 2019 to the acre cannot measure development. Its \
                    developed class ranges over 3,226 acres in seventeen years and fits at minus \
                    thirty-five acres a year, in a county that annexed fifty-seven times between \
                    1990 and 2024. One file, two classes, two epistemic statuses, and nothing in \
                    the file says which is which.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**The crop columns of that table may be differenced and the last two may not.** Developed ground\n  ranges over 3,226 acres across the seventeen years \u{2014} 6.9 per cent of the class \u{2014} and its fitted\n  slope is **minus 35 acres a year**, in a county that recorded fifty-seven annexations between 1990\n  and 2024 and builds two hundred-odd houses a year. Forest ranges over 21 per cent of itself and\n  fits at plus 253. Those two columns are the classifier re-deciding, not the ground changing.\n  [verified] \u{2014} same source, computed here; see\n  [one file, two reliabilities](../../decisions/one-file-two-reliabilities.yml) and\n  [the annexations](allen-county-annexations-1990-2024.yml)."
            ),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "soybeans-never-lost-the-lead",
        statement: "Soybeans have been Allen County's larger crop in every one of the seventeen \
                    years the pixels have been counted, never once reaching parity with corn. \
                    Winter wheat, meanwhile, lost two thirds of its ground between 2009 and 2012 \
                    and has not come back.",
        topic: "land",
        supports: &[
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**Soybeans have been the larger crop in every one of the seventeen years.** The ratio of corn to\n  soybean acreage runs between 0.63 and 0.90 and never reaches parity; the widest gap is 2017, with\n  97,740 acres of soybeans against 61,171 of corn. [verified] \u{2014} same source, computed here."
            ),
            support!(
                "measure/allen-county-land-cover-2008-2024.yml",
                "**Winter wheat lost two thirds of its ground in four years and never came back.** 21,483 acres in\n  2008, 25,210 in 2009, then 15,003, 17,196 and 6,688 in 2012; the highest of the twelve years since\n  is 10,348. [verified] \u{2014} same source."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2008", value: 21483.0, literal: "21,483" },
            Figure { label: "2009", value: 25210.0, literal: "25,210" },
            Figure { label: "2010", value: 15003.0, literal: "15,003" },
            Figure { label: "2011", value: 17196.0, literal: "17,196" },
            Figure { label: "2012", value: 6688.0, literal: "6,688" },
        ],
    },
    // ── Proprietors ──
    Assertion {
        id: "one-job-in-five-is-nobodys-payroll",
        statement: "Allen County had 11,964 proprietors against 52,274 wage and salary jobs in \
                    2022 \u{2014} 18.6 per cent of all employment, the highest in fifty-four years of \
                    record. The share ran between 12.0 and 14.9 per cent for the whole of the \
                    1969\u{2013}2000 series and has not stopped rising since 2001.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-proprietors-1969-2022.yml",
                "**Nearly one job in five in Allen County belongs to somebody working for themselves, and that has\n  never been true before in fifty-four years of record.** 11,964 proprietors against 52,274 wage and\n  salary jobs in 2022 \u{2014} 18.6 per cent of all employment, the highest figure in the series.\n  [verified] \u{2014} [BEA Regional Economic Accounts](../../catalog/bea-county-employment.md), CAEMP25S\n  for 1969\u{2013}2000 and CAEMP25N for 2001\u{2013}2022, lines 10, 20, 40, 50 and 60."
            ),
            support!(
                "measure/allen-county-proprietors-1969-2022.yml",
                "**In figures, the proprietor share of all employment at six points: 12.1 per cent in 1969, 13.8 in\n  1980, 12.4 in 1990, 14.0 in 2000, 16.8 in 2010 and 18.6 in 2022.** [inference] \u{2014} computed here\n  from the table above."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1969", value: 12.1, literal: "12.1" },
            Figure { label: "1980", value: 13.8, literal: "13.8" },
            Figure { label: "1990", value: 12.4, literal: "12.4" },
            Figure { label: "2000", value: 14.0, literal: "14.0" },
            Figure { label: "2010", value: 16.8, literal: "16.8" },
            Figure { label: "2022", value: 18.6, literal: "18.6" },
        ],
    },
    Assertion {
        id: "every-job-lost-since-2001-is-a-payroll-job",
        statement: "Allen County's payroll employment fell 8,061 between 2001 and 2022 while its \
                    proprietors rose 2,474, so total employment fell 5,587. For thirty-one years \
                    before that the two grew together \u{2014} payroll up 26.3 per cent, proprietors \
                    up 48.9. The county's job loss is entirely on the payroll side.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-proprietors-1969-2022.yml",
                "**The two halves of the county's workforce moved together for thirty-one years and then parted.**\n  Between 1969 and 2000 payroll jobs rose 26.3 per cent and proprietors 48.9 per cent, both growing.\n  Between 2001 and 2022 payroll jobs fell 13.4 per cent and proprietors rose 26.1. [inference] \u{2014}\n  computed here within each segment."
            ),
            support!(
                "measure/allen-county-proprietors-1969-2022.yml",
                "**Every job this county has lost since 2001 is a payroll job.** Wage and salary employment went\n  from 60,335 to 52,274, a loss of 8,061; total employment fell 5,587 over the same years, because\n  proprietors added 2,474. [inference] \u{2014} computed here from the NAICS segment."
            ),
            support!(
                "measure/allen-county-proprietors-1969-2022.yml",
                "**The proprietor share is at its record in the newest year and rose fastest in the three before\n  it.** It ran between 12.0 and 14.9 per cent for the whole SIC segment, and from 13.6 per cent in\n  2001 to 18.6 in 2022. The rise from 9,946 in 2019 to 11,964 in 2022 is **+2,018, or 20.3 per\n  cent** \u{2014} the largest three-year rise in either segment of the record. [verified] \u{2014} same source,\n  computed here."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Payroll jobs lost", value: 8061.0, literal: "8,061" },
            Figure { label: "Proprietors gained", value: 2474.0, literal: "2,474" },
            Figure { label: "Net jobs lost", value: 5587.0, literal: "5,587" },
        ],
    },
    Assertion {
        id: "the-farm-proprietors-halved",
        statement: "Allen County had 1,668 farm proprietors in 1969 and 802 in 2022. The fall runs \
                    through both halves of a series broken by a classification change and across \
                    the join itself, which is what a real trend looks like when the counting \
                    changes underneath it. Every net proprietor the county has gained is a nonfarm \
                    one.",
        topic: "work",
        supports: &[
            support!(
                "measure/allen-county-proprietors-1969-2022.yml",
                "**Farm proprietors have more than halved and the fall is the one thing spanning the break.**\n  1,668 in 1969, 1,401 in 1980, 1,266 in 1990, 1,060 in 2000, 1,013 in 2001, 812 in 2010 and 802 in\n  2022. It falls in the SIC segment and in the NAICS segment and across the join, which is what a\n  real trend looks like when a classification changes underneath it. [verified] \u{2014} same source."
            ),
            support!(
                "measure/allen-county-proprietors-1969-2022.yml",
                "**It answers a question the occupational survey left open.** That survey found sixty people in\n  farming, fishing and forestry occupations in a county two thirds covered by farms, and said the\n  reason was that it counts no self-employed person. This file counts 802 farm proprietors and 946\n  farm jobs of all kinds in the same county in 2022. [verified] \u{2014} same source, lines 50 and 70; see\n  [the occupational survey](allen-county-occupations-2014-2024.yml)."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1969", value: 1668.0, literal: "1,668" },
            Figure { label: "1980", value: 1401.0, literal: "1,401" },
            Figure { label: "1990", value: 1266.0, literal: "1,266" },
            Figure { label: "2000", value: 1060.0, literal: "1,060" },
            Figure { label: "2010", value: 812.0, literal: "812" },
            Figure { label: "2022", value: 802.0, literal: "802" },
        ],
    },
    Assertion {
        id: "one-renter-household-in-six",
        statement: "One renter household in six in Allen County has a federal subsidy attached to \
                    it — 2,251 subsidized units against 12,968 renter households. Inside Lima it is \
                    one in four; outside Lima it is one in seventeen.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-subsidized-housing-2005-2025.yml",
                "**One renter household in six in this county has a federal subsidy attached to it.** 2,251\n  subsidized units against 12,968 renter-occupied households \u{2014} 17.4 per cent. [inference] \u{2014}\n  computed here against table B25003 of the\n  [American Community Survey](../../catalog/census-acs-summary-file.md), 2023 five-year estimates,\n  which gives Allen County 40,928 occupied units of which 27,960 are owner-occupied."
            ),
            support!(
                "measure/allen-county-subsidized-housing-2005-2025.yml",
                "**Inside Lima it is one renter household in four; outside Lima it is one in seventeen.** 1,938\n  subsidized units against Lima's 7,500 renter households is 25.8 per cent; 313 against the other\n  5,468 is 5.7 per cent. [inference] \u{2014} the place file against table B25003, which gives Lima 13,985\n  occupied units of which 6,485 are owner-occupied."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Lima", value: 25.8, literal: "25.8" },
            Figure { label: "Allen County", value: 17.4, literal: "17.4" },
            Figure { label: "outside Lima", value: 5.7, literal: "5.7" },
        ],
    },
    Assertion {
        id: "more-subsidized-housing-than-the-five-around-it",
        statement: "Allen County holds more subsidized housing than Hancock, Hardin, Auglaize, Van \
                    Wert and Putnam together — 2,251 units against 1,733 — and all of the excess is \
                    Lima. Outside the city the county sits where its neighbours do.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-subsidized-housing-2005-2025.yml",
                "**Allen County holds more subsidized housing than the five counties around it put together.**\n  2,251 units against 1,733, on less than half their people. [verified] \u{2014}\n  [the county file](../../catalog/hud-picture-of-subsidized-households.md), 2025 edition."
            ),
            support!(
                "measure/allen-county-subsidized-housing-2005-2025.yml",
                "**And the comparison is about the city, not the county.** Allen's 22.3 per thousand is eighteenth\n  of Ohio's eighty-eight counties. Lima's own rate is 55.9 \u{2014} higher than any county in the state,\n  Cuyahoga's 31.4 included \u{2014} and the rest of Allen County's is 4.7, which would place it\n  seventy-eighth, between Union and Geauga and beside the rural neighbours it looks like.\n  [inference] \u{2014} computed here from the same two files. The range inside this one county, 51.1, is\n  larger than the range across the whole state, 30.5. See\n  [a county rate can describe nowhere](../../decisions/a-county-rate-can-describe-nowhere.yml)."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Lima", value: 55.9, literal: "55.9" },
            Figure { label: "Allen County", value: 22.3, literal: "22.3" },
            Figure { label: "outside Lima", value: 4.7, literal: "4.7" },
        ],
    },
    Assertion {
        id: "the-same-homes-hold-fewer-people",
        statement: "Allen County's stock of subsidized homes has barely moved in seventeen years — \
                    2,401 units in 2009 and 2,251 now — while the number of people living in them \
                    fell from 4,801 to 4,266. The households got older and had fewer children.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-subsidized-housing-2005-2025.yml",
                "Units fell 6.2 per cent across the seventeen years and never left the band 2,173 to 2,401. People\n  fell 11.1 per cent, from 4,801 to 4,266. [inference] \u{2014} arithmetic this corpus's, on the table\n  above. **The same number of subsidized homes now hold five hundred and thirty-five fewer people.**"
            ),
            support!(
                "measure/allen-county-subsidized-housing-2005-2025.yml",
                "**The households in them are older and have fewer children.** The share aged 62 or over was 21 per\n  cent in 2016 and is 32 now; the share that is a woman with children fell from 47 per cent in 2008\n  to 36. [verified] \u{2014} the same files. That is where the missing five hundred are: an elderly\n  household is smaller than a family, and the stock did not shrink, its occupants changed."
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "subsidized units", value: 6.2, literal: "6.2" },
            Figure { label: "people in them", value: 11.1, literal: "11.1" },
        ],
    },
    Assertion {
        id: "renters-carry-what-owners-do-not",
        statement: "In Allen County a renter is two and a half times as likely as an owner to be \
                    paying more than thirty per cent of income for housing, and more than three \
                    times as likely to be paying more than half.",
        topic: "housing",
        supports: &[support!(
            "measure/allen-county-housing-cost-burden-2006-2022.yml",
            "**A renter in this county is two and a half times as likely as an owner to be paying more than thirty per cent of income for housing**, 35.7 per cent against 14.2, and more than three times as likely to be paying more than half, 17.4 per cent against 5.3."
        )],
        answers: &["cannot say whether Lima's renters are more often cost-burdened"],
        figures: &[
            Figure { label: "renters over 30%", value: 35.7, literal: "35.7" },
            Figure { label: "owners over 30%", value: 14.2, literal: "14.2" },
            Figure { label: "renters over 50%", value: 17.4, literal: "17.4" },
            Figure { label: "owners over 50%", value: 5.3, literal: "5.3" },
        ],
    },
    Assertion {
        id: "one-renter-household-in-fifteen-is-sued",
        statement: "An eviction case is filed against about one Allen County renter household in fifteen every year. The filing rate runs 5.69 to 7.99 per cent across seventeen years and averages 6.79.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-evictions-2001-2018.yml", "**One renter household in fifteen has an eviction filed against it in this county every year.** The rate runs between 5.69 and 7.99 per cent across seventeen years and averages 6.79."),
        ],
        answers: &["cannot say how many people were put out of their homes"],
        figures: &[
            Figure { label: "lowest year, 2017", value: 5.69, literal: "5.69" },
            Figure { label: "mean", value: 6.79, literal: "6.79" },
            Figure { label: "highest year, 2008", value: 7.99, literal: "7.99" },
        ],
    },
    Assertion {
        id: "the-most-dangerous-work-here-is-care",
        statement: "Health care injures the people who do it more often than manufacturing does, in every one of nine years. The sector records 2,201 recordable cases at a rate of 5.59 per 100 full-time workers; manufacturing records 1,841 at 2.87, on sixty per cent more hours.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-workplace-injuries-2016-2024.yml", "**The most dangerous work in this county is care, and it is not close.** Health care and social assistance record 2,201 cases on 78,692,114 hours, a rate of 5.59; manufacturing records 1,841 on 128,109,489 hours, a rate of 2.87. Health care stands above manufacturing in **every one of the nine years**, from 5.10 against 4.11 in 2016 to 4.10 against 2.34 in 2024."),
        ],
        answers: &["cannot say how many people were hurt at work in Allen County"],
        figures: &[
            Figure { label: "health care, cases per 100 workers", value: 5.59, literal: "5.59" },
            Figure { label: "manufacturing", value: 2.87, literal: "2.87" },
        ],
    },
    Assertion {
        id: "the-refinery-is-safer-than-the-nursing-home",
        statement: "The Lima refinery is the safest large workplace in Allen County at 0.73 recordable cases per 100 full-time workers, and the county's nursing and residential care homes are the most dangerous at 8.38 \u{2014} eleven times higher.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-workplace-injuries-2016-2024.yml", "**The refinery is the safest large workplace here and the nursing homes are the most dangerous.** [The Lima refinery](../site/lima-refinery.yml) records 35 cases on 9,574,775 hours, a rate of **0.73**; the county's nursing and residential care homes record 559 on 13,338,203, a rate of **8.38**, eleven times higher."),
        ],
        answers: &["cannot say how many people were hurt at work in Allen County"],
        figures: &[
            Figure { label: "Lima refinery", value: 0.73, literal: "0.73" },
            Figure { label: "nursing and residential care", value: 8.38, literal: "8.38" },
        ],
    },
    Assertion {
        id: "five-thousand-seven-hundred-recordable-injuries",
        statement: "The establishments that owe OSHA an injury summary in Allen County recorded 5,712 injuries and illnesses on 309,534,737 hours between 2016 and 2024, with three deaths and 106,208 days away from work or on restricted duty.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-workplace-injuries-2016-2024.yml", "**5,712 recordable injuries and illnesses on 309,534,737 hours, three deaths, and 106,208 days away from work or on restricted duty.**"),
        ],
        answers: &["cannot say how many people were hurt at work in Allen County"],
        figures: &[
            Figure { label: "recordable cases", value: 5712.0, literal: "5,712" },
            Figure { label: "days away or restricted", value: 106208.0, literal: "106,208" },
        ],
    },
    Assertion {
        id: "the-pandemic-is-in-the-injury-logs",
        statement: "216 respiratory illnesses were recorded in this county's workplaces in 2020 against two in 2016 and none in 2018, and 211 of the 216 are in health care.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-workplace-injuries-2016-2024.yml", "**The pandemic is legible in the logbooks, and it is most of the nursing homes' figure.** 216 respiratory illnesses were recorded in 2020 against two in 2016 and none in 2018, **211 of the 216 in health care**."),
        ],
        answers: &["cannot say how many people were hurt at work in Allen County"],
        figures: &[
            Figure { label: "respiratory illnesses, 2020", value: 216.0, literal: "216" },
            Figure { label: "of which in health care", value: 211.0, literal: "211" },
        ],
    },
    Assertion {
        id: "the-fall-is-real-and-the-trend-is-not",
        statement: "Allen County's workplace injury rate fell from 5.03 in 2020 to 2.82 in 2024, and the nine-year trend does not clear its own noise: 0.126 a year on a standard error of 0.076. The level does \u{2014} 2019's rate would have produced 765 cases in 2024 and 534 were recorded.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-workplace-injuries-2016-2024.yml", "**The fall is real and the trend is not established.** A straight line through the nine rates falls 0.126 a year on a standard error of 0.076 \u{2014} 1.01 across the run, where two standard errors is 1.21 \u{2014} so the line does not clear its own noise. The level does: at 2019's rate, 2024's hours would have produced 765 cases and 534 were recorded, which is 8.3 Poisson standard deviations below."),
        ],
        answers: &["cannot say how many people were hurt at work in Allen County"],
        figures: &[
            Figure { label: "expected at the 2019 rate", value: 765.0, literal: "765" },
            Figure { label: "recorded", value: 534.0, literal: "534" },
        ],
    },
    Assertion {
        id: "three-workplace-deaths-and-that-is-ordinary",
        statement: "Three people died at work in this county's reporting establishments in nine years \u{2014} at International Tank Service in 2016, Colonial Surface Solutions in 2019 and Ford's Lima Engine plant in 2021. At the national rate in the same files the county's hours would have produced 3.94.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-workplace-injuries-2016-2024.yml", "**Three deaths in nine years, and that is an ordinary number.** One in 2016 at International Tank Service, one in 2019 at Colonial Surface Solutions, one in 2021 at [Ford's Lima Engine plant](../site/ford-lima-engine-plant.yml). At the national rate in these same files the county's hours would have produced 3.94, and three or more arrives three times in four."),
        ],
        answers: &["cannot say how many people were hurt at work in Allen County"],
        figures: &[
            Figure { label: "expected at the national rate", value: 3.94, literal: "3.94" },
        ],
    },
    Assertion {
        id: "a-plant-headcount-arrived-in-a-safety-filing",
        statement: "The Ford engine plant's employment, a question this corpus had carried open since genesis, is answered by its injury summaries: 1,365 in 2016 rising to 1,550 in 2022. Its people worked 22,432,575 hours across the nine years and 429 were hurt badly enough to record, a rate of 3.82.",
        topic: "work",
        supports: &[
            support!("site/ford-lima-engine-plant.yml", "**What it makes and how many people work in it.** The plant files an annual injury summary with OSHA under NAICS **333618** and describes its own industry as *engine manufacture*. Its annual average employment runs **1,365 in 2016**, 1,442, 1,413, 1,386, 1,497, 1,460, **1,550 in 2022** \u{2014} the highest of the nine \u{2014} and 1,500 in each of 2023 and 2024."),
            support!("site/ford-lima-engine-plant.yml", "**Its people worked 22,432,575 hours in those nine years and 429 of them were hurt badly enough to record** \u{2014} a rate of 3.82 per 100 full-time workers, against 3.69 for the county's reporting establishments as a whole and 2.87 for its manufacturing."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2016", value: 1365.0, literal: "1,365" },
            Figure { label: "2022", value: 1550.0, literal: "1,550" },
            Figure { label: "cases per 100 workers", value: 3.82, literal: "3.82" },
        ],
    },
    Assertion {
        id: "one-workplace-in-ten-owes-a-safety-filing",
        statement: "One Allen County workplace in ten owes the federal government an annual injury summary, and between them they hold half the county's employees: 222 establishments of 2,239 in 2023, covering 22,622 employees of 44,251.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-private-employers-1986-2023.yml", "**One of these workplaces in ten owes the federal government an injury summary, and between them they hold half the county's employees.** 222 Allen County establishments filed a Form 300A for 2023 against the 2,239 counted here, and the 22,622 employees they cover are 51.1 per cent of the 44,251."),
        ],
        answers: &["cannot say whether that flatness is wages standing still or hours doing so"],
        figures: &[
            Figure { label: "establishments filing", value: 222.0, literal: "222" },
            Figure { label: "private establishments", value: 2239.0, literal: "2,239" },
        ],
    },
    Assertion {
        id: "twelve-homes-and-eight-hundred-and-eleven-beds",
        statement: "Allen County has twelve certified nursing homes holding 811 beds and 712.5 residents a day. They are 87.9 per cent full, against 83.6 per cent for Ohio's 922 homes and 80.3 for the country's 14,690.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-nursing-homes-2026.yml", "`staff` is total nurse staffing hours per resident per day, `turn` annual nursing-staff turnover. **Twelve homes, 811 certified beds, 712.5 residents a day \u{2014} 87.9 per cent occupancy against 83.6 for Ohio's 922 homes and 80.3 for the country's 14,690.** [verified] \u{2014} the same file."),
        ],
        answers: &["cannot say how well anybody here is cared for"],
        figures: &[
            Figure { label: "Allen County", value: 87.9, literal: "87.9" },
            Figure { label: "Ohio", value: 83.6, literal: "83.6" },
            Figure { label: "United States", value: 80.3, literal: "80.3" },
        ],
    },
    Assertion {
        id: "two-thirds-of-the-beds-are-run-from-elsewhere",
        statement: "Four chains run 545 of the county's 811 nursing-home beds \u{2014} Trilogy Health Services, HCF Management, Vancrest and CareCore Health, none of them local. Nine of the twelve homes are for profit; the three that are not hold 152 beds.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-nursing-homes-2026.yml", "**Two thirds of the county's beds are run from somewhere else.** 545 of the 811 belong to four chains \u{2014} Trilogy Health Services (123 homes, three of these), HCF Management (22, two), Vancrest (13, one) and CareCore Health (10, one). Nine of the twelve are for profit; the three that are not hold 152 beds."),
        ],
        answers: &["cannot say how well anybody here is cared for"],
        figures: &[
            Figure { label: "chain-run beds", value: 545.0, literal: "545" },
            Figure { label: "all certified beds", value: 811.0, literal: "811" },
            Figure { label: "non-profit beds", value: 152.0, literal: "152" },
        ],
    },
    Assertion {
        id: "better-rated-and-no-better-staffed",
        statement: "Allen County's nursing homes are rated well above the state's and staffed no better. Mean overall rating 3.75 against Ohio's 3.15 and the country's 2.98, nursing-staff turnover 39.5 per cent at the median against 48.5 and 44.9, and total nurse staffing 3.57 hours per resident per day against 3.57 and 3.68.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-nursing-homes-2026.yml", "**The county's homes are fuller, better rated and less churned than the state's, and no better staffed.** Mean overall rating 3.75 against Ohio's 3.15 and the country's 2.98; nursing-staff turnover 39.5 per cent at the median against 48.5 and 44.9; total nurse staffing 3.57 hours per resident per day against 3.57 and 3.68."),
        ],
        answers: &["cannot say how well anybody here is cared for"],
        figures: &[
            Figure { label: "Allen County rating", value: 3.75, literal: "3.75" },
            Figure { label: "Ohio", value: 3.15, literal: "3.15" },
            Figure { label: "United States", value: 2.98, literal: "2.98" },
        ],
    },
    Assertion {
        id: "two-hundred-and-eighty-seven-deficiencies",
        statement: "Inspectors cited Allen County's twelve nursing homes 287 times between February 2019 and May 2026. Nine citations were for harm actually done to a resident and two reached immediate jeopardy \u{2014} Springview Manor for accident hazards in January 2024 and Liberty Retirement Community for respiratory care in March 2026.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-nursing-homes-2026.yml", "**287 deficiencies in seven years, and 11 of them for harm actually done.** Inspectors cited the twelve homes 287 times between 21 February 2019 and 20 May 2026. Nine citations reached scope-and-severity G \u{2014} actual harm to a resident \u{2014} and **two more reached immediate jeopardy**: Springview Manor on 10 January 2024 for accident hazards, Liberty Retirement Community on 19 March 2026 for respiratory care."),
        ],
        answers: &["cannot say how well anybody here is cared for"],
        figures: &[
            Figure { label: "all citations", value: 287.0, literal: "287" },
            Figure { label: "citations for harm done", value: 11.0, literal: "11" },
        ],
    },
    Assertion {
        id: "a-five-star-home-is-the-most-dangerous-workplace",
        statement: "The Springs of Lima carries five stars overall and five on health inspection, with no deficiency in its latest cycle and no fine \u{2014} and its workers recorded 89 injuries on 883,063 hours, a rate of 20.16 per 100 full-time workers against 3.69 for the county.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-nursing-homes-2026.yml", "**A five-star home can be the most dangerous workplace in the county.** The Springs of Lima carries five stars overall, five on health inspection, no deficiency in its latest cycle and no fine \u{2014} and its workers recorded 89 injuries on 883,063 hours, a rate of 20.16 against 3.69 for the county."),
        ],
        answers: &["cannot say how well anybody here is cared for"],
        figures: &[
            Figure { label: "The Springs of Lima", value: 20.16, literal: "20.16" },
            Figure { label: "all reporting workplaces in the county", value: 3.69, literal: "3.69" },
        ],
    },
    Assertion {
        id: "the-homes-are-a-seventh-smaller-than-in-2013",
        statement: "Allen County's nursing homes delivered 320,868 resident days in 2013 and 272,194 in 2023, 15.2 per cent below the peak, with the bottom at 256,683 in 2022.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-nursing-home-days-2011-2023.yml", "**The county's nursing homes peaked in 2013 and are a seventh smaller now.** 320,868 resident days then, 256,683 at the bottom in 2022, 272,194 in 2023 \u{2014} **15.2 per cent below the peak**."),
        ],
        answers: &["cannot say how many Allen County people are in a nursing home outside it"],
        figures: &[
            Figure { label: "2013", value: 320868.0, literal: "320,868" },
            Figure { label: "2022", value: 256683.0, literal: "256,683" },
            Figure { label: "2023", value: 272194.0, literal: "272,194" },
        ],
    },
    Assertion {
        id: "medicaid-pays-for-three-days-in-five",
        statement: "Medicaid pays for between 55.9 and 65.2 per cent of Allen County's nursing-home days across thirteen years and 57.3 per cent now. Medicare days, which buy the short rehabilitation stay, fell 28.7 per cent over the same run.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-nursing-home-days-2011-2023.yml", "**Medicaid pays for three days in five and always has.** Its share runs 55.9 to 65.2 per cent across the thirteen years and stands at 57.3. [verified] \u{2014} the same files. Medicare, which pays for the short rehabilitation stay rather than the long residence, fell furthest: 32,476 days in 2011 to 23,153 in 2023, **down 28.7 per cent**, while everything else rose 1.9."),
        ],
        answers: &["cannot say how many Allen County people are in a nursing home outside it"],
        figures: &[
            Figure { label: "lowest Medicaid share", value: 55.9, literal: "55.9" },
            Figure { label: "2023", value: 57.3, literal: "57.3" },
            Figure { label: "highest", value: 65.2, literal: "65.2" },
        ],
    },
    Assertion {
        id: "the-fourth-district-for-twenty-years",
        statement: "Allen County has voted in Ohio's 4th congressional district at every general election from 2000 to 2020, and the Republican share of its vote for Congress runs 59.9 to 72.2 per cent, averaging 68.5. Its lowest year is 2006, when the seat changed hands.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-in-congress-2000-2020.yml", "**The county has not given a Democrat as much as two votes in five for Congress in twenty years.** The Republican share runs 59.9 to 72.2 per cent and averages 68.5. Its lowest year is 2006, which is the year the seat changed hands."),
        ],
        answers: &["cannot say who this county has sent to Congress since 2020"],
        figures: &[
            Figure { label: "lowest, 2006", value: 59.9, literal: "59.9" },
            Figure { label: "mean", value: 68.5, literal: "68.5" },
            Figure { label: "highest, 2010", value: 72.2, literal: "72.2" },
        ],
    },
    Assertion {
        id: "one-name-on-this-ballot-for-twenty-years",
        statement: "A candidate named Jim Jordan won Ohio's 4th district in Allen County at eight consecutive general elections from 2006, and a candidate of that name won the county's 12th state senate district in 2000.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-in-congress-2000-2020.yml", "**One name has been on this ballot for twenty years.** A candidate named Jim Jordan won the 4th district here at eight consecutive general elections from 2006, and a candidate of that name won the county's 12th state senate district in 2000."),
        ],
        answers: &["cannot say who this county has sent to Congress since 2020"],
        figures: &[
        ],
    },
    Assertion {
        id: "sherrod-brown-came-closest",
        statement: "Sherrod Brown took 46.0 per cent of Allen County's Senate vote in 2006 and 40.2 in 2018 \u{2014} the nearest a Democrat has come here. No Democrat running for the House reached 40 per cent in any of ten elections.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-in-congress-2000-2020.yml", "**The Senate is where a Democrat has come closest.** Sherrod Brown took 46.0 per cent here in 2006 and 40.2 in 2018; no Democrat in the House column reached 40 in any year."),
        ],
        answers: &["cannot say who this county has sent to Congress since 2020"],
        figures: &[
            Figure { label: "2006", value: 46.0, literal: "46.0" },
            Figure { label: "2018", value: 40.2, literal: "40.2" },
        ],
    },
    Assertion {
        id: "the-county-is-its-own-house-district",
        statement: "Ohio House district 4 is Allen County and nothing else: all 176 of the county's State Representative precinct rows in 2018 carry district 4, and every one of the 88 precincts recorded in that district anywhere in Ohio is an Allen County precinct.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-in-the-general-assembly-2000-2020.yml", "**Ohio House district 4 is Allen County and nothing else.** All 176 of the county's State Representative precinct rows in 2018 carry district 4, and every one of the 88 precincts recorded in that district anywhere in Ohio is an Allen County precinct."),
        ],
        answers: &["cannot say who has represented this county in Columbus since 2020"],
        figures: &[
            Figure { label: "precinct rows", value: 176.0, literal: "176" },
            Figure { label: "precincts", value: 88.0, literal: "88" },
        ],
    },
    Assertion {
        id: "four-contests-with-one-name-on-the-ballot",
        statement: "Four of the thirteen Ohio General Assembly contests in Allen County's returns had a single candidate \u{2014} the Ohio House seat in 2008, 2016 and 2020, and the 12th senate district in 2016.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-in-the-general-assembly-2000-2020.yml", "**Four of these thirteen contests had one name on the ballot.** The Ohio House seat in 2008, 2016 and 2020, and the senate seat in 2016; no second candidate appears for those districts in any county of the statewide file."),
        ],
        answers: &["cannot say who has represented this county in Columbus since 2020"],
        figures: &[
        ],
    },
    Assertion {
        id: "a-fifth-will-not-mark-an-unopposed-line",
        statement: "About a fifth of Allen County's voters skip an unopposed ballot line. The Ohio House race loses 20.5 per cent of the top-of-ticket vote in 2008, 24.7 in 2016 and 19.6 in 2020, against 3.3, 0.6 and 0.3 per cent in the years somebody was running against the winner.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-in-the-general-assembly-2000-2020.yml", "**A fifth of the county's voters will not mark an unopposed ballot line.** Against the top of the ticket, the Ohio House race loses 20.5 per cent of the votes cast in 2008, **24.7 per cent** in 2016 and 19.6 in 2020 \u{2014} and 3.3 per cent in 2012, 0.6 in 2014 and 0.3 in 2018, when somebody was running against the winner."),
        ],
        answers: &["cannot say who has represented this county in Columbus since 2020"],
        figures: &[
            Figure { label: "2008, unopposed", value: 20.5, literal: "20.5" },
            Figure { label: "2016, unopposed", value: 24.7, literal: "24.7" },
            Figure { label: "2020, unopposed", value: 19.6, literal: "19.6" },
            Figure { label: "2012, opposed", value: 3.3, literal: "3.3" },
        ],
    },
    Assertion {
        id: "one-seat-one-man-four-years-apart",
        statement: "Matt Huffman's Ohio Senate district lost 22.6 per cent of Allen County's voters in 2016, when nobody opposed him, and 2.9 per cent in 2020, when somebody did.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-in-the-general-assembly-2000-2020.yml", "**The cleanest reading is one seat, one man, four years apart.** Matt Huffman's senate district loses **22.6 per cent** of the county's voters in 2016, when nobody opposed him, and **2.9 per cent** in 2020, when somebody did."),
        ],
        answers: &["cannot say who has represented this county in Columbus since 2020"],
        figures: &[
            Figure { label: "2016, unopposed", value: 22.6, literal: "22.6" },
            Figure { label: "2020, opposed", value: 2.9, literal: "2.9" },
        ],
    },
    Assertion {
        id: "the-serious-court-shrank-and-the-everyday-one-grew",
        statement: "Filings in Allen County's court of common pleas fell 37.3 per cent between 2007 and 2017, from 9,165 to 5,749, while the Lima Municipal Court's rose 12.0 per cent to 25,021.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-court-caseloads-2007-2017.yml", "**The county's serious court shrank by more than a third while its everyday court grew.** Common pleas filings fell from 9,165 to 5,749, down **37.3 per cent**; the Lima Municipal Court's rose from 22,346 to 25,021, up 12.0."),
        ],
        answers: &["cannot say what has happened in these courts since 2017"],
        figures: &[
            Figure { label: "common pleas, 2007", value: 9165.0, literal: "9,165" },
            Figure { label: "common pleas, 2017", value: 5749.0, literal: "5,749" },
            Figure { label: "municipal, 2007", value: 22346.0, literal: "22,346" },
            Figure { label: "municipal, 2017", value: 25021.0, literal: "25,021" },
        ],
    },
    Assertion {
        id: "juvenile-filings-fell-by-two-fifths",
        statement: "Juvenile filings in Allen County fell from 4,655 in 2007 to 2,680 in 2017 \u{2014} 42.4 per cent, or 43 cases per thousand residents down to 25. Every common pleas division fell and this one fell furthest.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-court-caseloads-2007-2017.yml", "**Every common pleas division fell and the juvenile division fell furthest.** Juvenile filings went from 4,655 to 2,680, down 42.4 per cent \u{2014} 43 per thousand residents to 25. General division down 38.3, domestic relations 32.6, probate 18.2."),
        ],
        answers: &["cannot say what has happened in these courts since 2017"],
        figures: &[
            Figure { label: "2007", value: 4655.0, literal: "4,655" },
            Figure { label: "2017", value: 2680.0, literal: "2,680" },
        ],
    },
    Assertion {
        id: "crime-is-the-part-that-did-not-move",
        statement: "Criminal filings in Allen County's general division ran 400 to 537 across eleven years and ended where they began, 416 in 2007 and 415 in 2017, while the same court's civil filings fell from 1,391 to 702.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-court-caseloads-2007-2017.yml", "**Crime is the part that did not move.** The general division's criminal filings run 400 to 537 across the eleven years and end where they began, 416 in 2007 and 415 in 2017; its civil filings fell from 1,391 to 702."),
        ],
        answers: &["cannot say what has happened in these courts since 2017"],
        figures: &[
            Figure { label: "criminal, 2007", value: 416.0, literal: "416" },
            Figure { label: "criminal, 2017", value: 415.0, literal: "415" },
            Figure { label: "civil, 2007", value: 1391.0, literal: "1,391" },
            Figure { label: "civil, 2017", value: 702.0, literal: "702" },
        ],
    },
    Assertion {
        id: "seven-judges-hear-all-of-it",
        statement: "Seven judges hear every case of record in Allen County \u{2014} two in the general division, one each in domestic relations, probate and juvenile, and two on the municipal bench \u{2014} and the count did not change in eleven years. Each general division judge took 588 new cases in 2017 against 953 in 2007.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-court-caseloads-2007-2017.yml", "**Seven judges hear all of it.** Two in the general division and one each in domestic relations, probate and juvenile, plus two on the municipal bench; the count does not change in any of the eleven years. Each general division judge took 588 new cases in 2017 against 953 in 2007."),
        ],
        answers: &["cannot say what has happened in these courts since 2017"],
        figures: &[
            Figure { label: "per general division judge, 2007", value: 953.0, literal: "953" },
            Figure { label: "per general division judge, 2017", value: 588.0, literal: "588" },
        ],
    },
    Assertion {
        id: "six-thousand-six-hundred-a-head",
        statement: "The federal government spent $668,254,916 in Allen County in federal year 2025 \
                    \u{2014} $6,625 for every resident \u{2014} and 93.5 per cent of it was payments \
                    to individuals rather than grants or contracts.",
        topic: "government",
        supports: &[
            support!("measure/federal-money-in-allen-county-2008-2025.yml", "**The answer is $668,254,916 in federal year 2025 \u{2014} $6,625 for every resident \u{2014} and 93.5 per cent of it is payments to individuals.** [verified] \u{2014} [USAspending](../../catalog/usaspending.md), place of performance, against [the county's population](allen-county-population-2024.yml)."),
        ],
        answers: &["cannot say what share of this money stays here"],
        figures: &[
            Figure { label: "per resident", value: 6625.0, literal: "6,625" },
        ],
    },
    Assertion {
        id: "four-social-security-programmes",
        statement: "Four Social Security programmes paid $536,386,261 into Allen County in 2025 \
                    \u{2014} retirement $402,802,985, disability $64,160,105, survivors $44,657,548 \
                    and supplemental security income $24,765,623 \u{2014} which is $5,318 for every \
                    resident of the county.",
        topic: "government",
        supports: &[
            support!("measure/federal-money-in-allen-county-2008-2025.yml", "**Four Social Security programmes are four fifths of the whole year.** Retirement insurance $402,802,985, disability $64,160,105, survivors $44,657,548 and supplemental security income $24,765,623 \u{2014} $536,386,261 between them, $5,318 a head across everybody in the county. Veterans' compensation for service-connected disability adds $40,574,698. [verified] \u{2014} same source, by assistance listing. What a federal budget does in a county like this one is mostly send cheques to old people. [inference]"),
        ],
        answers: &["cannot say what share of this money stays here"],
        figures: &[
            Figure { label: "supplemental security income", value: 24765623.0, literal: "24,765,623" },
            Figure { label: "survivors", value: 44657548.0, literal: "44,657,548" },
            Figure { label: "disability", value: 64160105.0, literal: "64,160,105" },
            Figure { label: "retirement", value: 402802985.0, literal: "402,802,985" },
        ],
    },
    Assertion {
        id: "the-biggest-federal-grant-here-is-roads",
        statement: "Highway planning and construction has brought Allen County $255,335,761 since \
                    2008, more than three times Head Start's $69,991,874 and the largest federal \
                    grant programme in the county. Across its 359.2 miles of federal-aid highway \
                    that is $710,846 a mile.",
        topic: "government",
        supports: &[
            support!("measure/federal-money-in-allen-county-2008-2025.yml", "**The largest grant programme over eighteen years is roads.** Highway planning and construction has brought $255,335,761 since 2008, against Head Start's $69,991,874, the education stabilization fund's $38,535,550 and the health centre programme's $49,416,292 across two listings. [verified] \u{2014} same source. Spread across the 359.2 miles of federal-aid highway this county has, that is $710,846 a mile in eighteen years. [inference] \u{2014} computed here against [the federal-aid system](allen-county-federal-aid-highways-2018.yml)."),
        ],
        answers: &["cannot say what share of this money stays here"],
        figures: &[
            Figure { label: "Head Start", value: 69991874.0, literal: "69,991,874" },
            Figure { label: "highways", value: 255335761.0, literal: "255,335,761" },
        ],
    },
    Assertion {
        id: "one-billion-and-one-hundred-and-fifty-five-million",
        statement: "Federal contract dollars whose place of performance is Allen County come to \
                    $1,088,368,417 since 2008; contract dollars whose recipient is located in Allen \
                    County come to $154,761,289. Neither is wrong \u{2014} one counts where the work \
                    was done and the other where the payee's address is.",
        topic: "government",
        supports: &[
            support!("measure/federal-money-in-allen-county-2008-2025.yml", "**The same file answers this question twice and the two answers differ by a factor of seven on contracts.** The ratio of the place-of-performance total to the recipient-location total is 7.03 on contracts and 1.97 across all award types. [verified] \u{2014} the same source, both scopes, summed here."),
            support!("measure/federal-money-in-allen-county-2008-2025.yml", "**Neither is wrong.** One counts where the work was done and the other where the payee's address is, and the gap is what it means for a county's largest installations to be owned and operated from elsewhere. [inference] \u{2014} see [a dollar has two addresses](../../decisions/a-dollar-has-two-addresses.yml) and [the contracts](federal-contracts-in-allen-county-2008-2025.yml)."),
        ],
        answers: &["cannot say what share of this money stays here"],
        figures: &[
            Figure { label: "ratio on all award types", value: 1.97, literal: "1.97" },
            Figure { label: "ratio on contracts", value: 7.03, literal: "7.03" },
        ],
    },
    Assertion {
        id: "ninety-seven-per-cent-defence",
        statement: "The Department of Defense bought $1,060,110,812 of the $1,088,368,417 of federal \
                    contract work performed in Allen County since 2008 \u{2014} 97.4 per cent. \
                    Veterans Affairs is the next largest at $16,594,375 and the whole of the rest of \
                    the federal government is $134,589.",
        topic: "government",
        supports: &[
            support!("measure/federal-contracts-in-allen-county-2008-2025.yml", "**$1,088,368,417 of federal contract work has been performed in Allen County since 2008, and 97.4 per cent of it was bought by the Department of Defense.** Veterans Affairs is the next largest at $16,594,375 and the General Services Administration at $11,081,336; the whole of the rest of the federal government comes to $134,589. [verified] \u{2014} [USAspending](../../catalog/usaspending.md), place of performance, by awarding agency."),
        ],
        answers: &["cannot say how many people in Allen County are paid out of these contracts"],
        figures: &[
            Figure { label: "everything else", value: 134589.0, literal: "134,589" },
            Figure { label: "General Services Administration", value: 11081336.0, literal: "11,081,336" },
            Figure { label: "Veterans Affairs", value: 16594375.0, literal: "16,594,375" },
        ],
    },
    Assertion {
        id: "four-firms-and-one-is-from-here",
        statement: "Four firms account for 92.2 per cent of the federal contract work performed in \
                    Allen County \u{2014} tanks, jet fuel, the tank plant's utilities and warhead \
                    casings. Three of them vanish when the same file is asked who was paid: \
                    $901,951,528 of work done here is recorded against corporate addresses \
                    elsewhere.",
        topic: "government",
        supports: &[
            support!("measure/federal-contracts-in-allen-county-2008-2025.yml", "**Four firms are 92.2 per cent of it and only one of them is from here.** General Dynamics Land Systems $453,692,926, Husky Marketing and Supply $328,850,129, Siemens Government Technologies $119,408,473 and Superior Forge & Steel $101,917,494. [verified] \u{2014} same source, by recipient."),
            support!("measure/federal-contracts-in-allen-county-2008-2025.yml", "**Three of those four vanish when the same file is asked who was paid.** $901,951,528 of work done on this county's ground is recorded, in the recipient view, against corporate addresses elsewhere; Superior Forge & Steel is a Lima firm and appears in both. [verified] \u{2014} same source, both scopes. See [a dollar has two addresses](../../decisions/a-dollar-has-two-addresses.yml)."),
        ],
        answers: &["cannot say how many people in Allen County are paid out of these contracts"],
        figures: &[
            Figure { label: "Superior Forge & Steel", value: 101917494.0, literal: "101,917,494" },
            Figure { label: "Siemens", value: 119408473.0, literal: "119,408,473" },
            Figure { label: "Husky Marketing and Supply", value: 328850129.0, literal: "328,850,129" },
            Figure { label: "General Dynamics Land Systems", value: 453692926.0, literal: "453,692,926" },
        ],
    },
    Assertion {
        id: "forty-two-tanks-for-saudi-arabia",
        statement: "The largest single federal award performed in Allen County is $71,709,279 to \
                    General Dynamics Land Systems, December 2010 to July 2012, for forty-two \
                    material sets converting M1A2 tanks to the M1A2S configuration for the Kingdom \
                    of Saudi Arabia.",
        topic: "government",
        supports: &[
            support!("measure/federal-contracts-in-allen-county-2008-2025.yml", "**The largest single award names the customer and it is not the United States.** $71,709,279 to General Dynamics Land Systems, 30 December 2010 to 2 July 2012, for \"(42) MATERIAL SETS FOR THE CONVERSION OF M1A2 TANKS TO M1A2S CONFIGURATION FOR THE KINGDOM OF SAUDI ARABIA\". [verified] \u{2014} same source, the award endpoint. The plant is [the Army's](../site/lima-army-tank-plant.yml), the work is a foreign military sale, and the money is an Army contract. [inference]"),
        ],
        answers: &["cannot say how many people in Allen County are paid out of these contracts"],
        figures: &[
            Figure { label: "the Saudi award", value: 71709279.0, literal: "71,709,279" },
        ],
    },
    Assertion {
        id: "the-forge-and-the-refinery",
        statement: "The Lima refinery sold the Defense Logistics Agency $327,872,832 of JP8 turbine \
                    fuel in three contracts, and a Lima forge holds $94,389,046 of Air Force awards \
                    for penetrator warhead production and massive ordnance penetrator warhead cases.",
        topic: "government",
        supports: &[
            support!("measure/federal-contracts-in-allen-county-2008-2025.yml", "**The refinery sells the military jet fuel.** Three awards to Husky Marketing and Supply from the Defense Logistics Agency for `TURBINE FUEL, JP8` \u{2014} $178,794,000 running from March 2009, $84,717,360 from April 2010 and $64,361,472 from April 2008. [verified] \u{2014} same source. Husky is the name [the Lima refinery](../site/lima-refinery.yml) was operated under from 2010, and JP8 is a kerosene the refinery's own product slate would carry. [inference]"),
            support!("measure/federal-contracts-in-allen-county-2008-2025.yml", "**A Lima forge makes the casings for the largest conventional bomb the Air Force has.** Superior Forge & Steel Corp holds five Air Force awards worth $94,389,046 between 2018 and 2025 for \"BLU-137/B PENETRATOR WARHEAD PRODUCTION\" and \"MASSIVE ORDNANCE PENETRATOR WARHEAD CASES\". [verified] \u{2014} same source, by award. This is the county's own firm in the county's largest industry, and the corpus had it in no other file. [inference]"),
        ],
        answers: &["cannot say how many people in Allen County are paid out of these contracts"],
        figures: &[
            Figure { label: "the forge's warhead awards", value: 94389046.0, literal: "94,389,046" },
            Figure { label: "one JP8 contract", value: 178794000.0, literal: "178,794,000" },
        ],
    },
    Assertion {
        id: "one-collapsed-and-the-other-did-not-move",
        statement: "Foreclosure filings in Allen County peaked at 996 in 2008 and fell to 264 by 2017, down 73.5 per cent. Eviction filings over the same eleven years ran 798 to 982, averaging 874.7 with a standard deviation of 55.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-foreclosures-and-evictions-2007-2017.yml", "**One of these collapsed and the other did not move.** Foreclosure filings peaked at 996 in 2008 and stand at 264, down **73.5 per cent**; eviction filings run 798 to 982 across the same eleven years with a mean of 874.7 and a standard deviation of 55."),
        ],
        answers: &["cannot say how many of these filings ended with somebody leaving"],
        figures: &[
            Figure { label: "foreclosures, 2008", value: 996.0, literal: "996" },
            Figure { label: "foreclosures, 2017", value: 264.0, literal: "264" },
            Figure { label: "evictions, mean", value: 874.7, literal: "874.7" },
        ],
    },
    Assertion {
        id: "three-renters-for-every-owner",
        statement: "In 2008 Allen County's courts took 996 foreclosure filings against 982 eviction filings. By 2016 the figures were 279 and 913 \u{2014} three renters losing their home for every owner.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-foreclosures-and-evictions-2007-2017.yml", "**In 2008 the county lost about as many owners as renters, and by 2016 it lost three renters for every owner.** 996 against 982 in the first year; 279 against 913 in the last full swing."),
        ],
        answers: &["cannot say how many of these filings ended with somebody leaving"],
        figures: &[
            Figure { label: "foreclosures, 2008", value: 996.0, literal: "996" },
            Figure { label: "evictions, 2008", value: 982.0, literal: "982" },
            Figure { label: "foreclosures, 2016", value: 279.0, literal: "279" },
            Figure { label: "evictions, 2016", value: 913.0, literal: "913" },
        ],
    },
    Assertion {
        id: "the-same-numbers-from-two-covers",
        statement: "The Supreme Court of Ohio and the Eviction Lab publish the same eviction counts for Allen County \u{2014} 982 in 2008, 922 in 2011 and 863 in 2014, to the unit \u{2014} and differ by four in 2017, 802 against 806.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-foreclosures-and-evictions-2007-2017.yml", "**The court's own eviction counts and the Eviction Lab's are the same numbers.** 982 in 2008, 922 in 2011 and 863 in 2014 appear in both to the unit; 2017 differs by four, 802 here against 806 there."),
        ],
        answers: &["cannot say how many of these filings ended with somebody leaving"],
        figures: &[
            Figure { label: "2008", value: 982.0, literal: "982" },
            Figure { label: "2011", value: 922.0, literal: "922" },
            Figure { label: "2014", value: 863.0, literal: "863" },
        ],
    },
    Assertion {
        id: "six-registrations-two-working-faces",
        statement: "The federal mine regulator has registered six mines in Allen County and two are still worked. Employment across the pair averaged 44.2 in 2006 and 24.0 in 2018 and stands at 31.8, on 1,748,718 hours between 2000 and 2025.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-quarries-2000-2025.yml", "**Six registrations, two working faces, and about thirty people.** Employment across the two active quarries averaged 44.2 in 2006 and 24.0 in 2018 and stands at 31.8; they worked **1,748,718 hours between 2000 and 2025**."),
        ],
        answers: &["cannot say how much stone has come out of this county"],
        figures: &[
            Figure { label: "2006", value: 44.2, literal: "44.2" },
            Figure { label: "2018", value: 24.0, literal: "24.0" },
            Figure { label: "2025", value: 31.8, literal: "31.8" },
        ],
    },
    Assertion {
        id: "a-quarry-inside-a-village",
        statement: "Both of Allen County's working quarries are surface limestone. The National Lime & Stone plant is in Bath Township and in no incorporated place; Bluffton Stone is in Richland Township and inside the village of Bluffton.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-quarries-2000-2025.yml", "**Both are surface limestone quarries and both stand where the corpus can put them.** The National Lime & Stone plant is at 40.750833, -84.086111 \u{2014} in [Bath Township](../place/bath-township.yml) and in no incorporated place. Bluffton Stone is at 40.889167, -83.890278, in [Richland Township](../place/richland-township.yml) and **inside the village of Bluffton**."),
        ],
        answers: &["cannot say how much stone has come out of this county"],
        figures: &[
        ],
    },
    Assertion {
        id: "the-quarry-and-the-engine-plant",
        statement: "The National Lime & Stone quarry stands 1.62 miles from Ford's Lima engine plant, in the same township, and its operator is the company that took the plant's ground from the National Quarries Company in January 1944 and conveyed it on in 1955.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-quarries-2000-2025.yml", "**The quarry and the engine plant are 1.62 miles apart in one township.** The company still working the first sold the ground under the second: National Lime & Stone took the Ford site from the National Quarries Company in January 1944 and conveyed it on in 1955."),
        ],
        answers: &["cannot say how much stone has come out of this county"],
        figures: &[
            Figure { label: "miles apart", value: 1.62, literal: "1.62" },
        ],
    },
    Assertion {
        id: "thirty-four-injuries-and-no-death",
        statement: "Allen County's quarries recorded 34 reportable injuries in twenty-six years and no death \u{2014} 3.89 per 200,000 hours, 3.63 at the Lima plant and 4.44 at Bluffton, with 622 days lost and one permanent disability in December 2011.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-quarries-2000-2025.yml", "**Thirty-four reportable injuries in twenty-six years and no death.** A rate of **3.89 per 200,000 hours** \u{2014} 3.63 at the Lima plant and 4.44 at Bluffton \u{2014} with 622 days lost and 275 restricted, and one permanent disability, on 16 December 2011."),
        ],
        answers: &["cannot say how much stone has come out of this county"],
        figures: &[
            Figure { label: "Lima plant", value: 3.63, literal: "3.63" },
            Figure { label: "both quarries", value: 3.89, literal: "3.89" },
            Figure { label: "Bluffton Stone", value: 4.44, literal: "4.44" },
        ],
    },
    Assertion {
        id: "the-registers-oldest-date-is-not-a-date",
        statement: "The mine register dates National Lime & Stone's control of its Lima quarry to 1 January 1950 \u{2014} the same day 41,309 of the file's 169,890 controller records begin, and earlier than none of them.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-quarries-2000-2025.yml", "**The register's oldest date is not a date.** National Lime & Stone's control of the Lima plant is recorded as beginning 1 January 1950, which is the same day 41,309 of the file's 169,890 controller records begin and earlier than none of them."),
        ],
        answers: &["cannot say how much stone has come out of this county"],
        figures: &[
            Figure { label: "records beginning that day", value: 41309.0, literal: "41,309" },
            Figure { label: "records in the file", value: 169890.0, literal: "169,890" },
        ],
    },
    Assertion {
        id: "the-eviction-count-does-not-move",
        statement: "Allen County's eviction filings run 766 to 982 across seventeen years \u{2014} mean 865 \u{2014} through a foreclosure crisis, a recession and a quarter more renter households. The rate drifts down 0.92 points over the run, which is about the smallest movement seventeen readings could have found.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-evictions-2001-2018.yml", "**The count does not move and the rate barely does.** Filings run from 766 to 982 with a mean of 865 and a standard deviation of 56, across seventeen years that contain a foreclosure crisis, a recession and a quarter more renter households. [verified] \u{2014} the same file. The filing rate falls by 0.058 points a year, which is 0.92 points over the run against a mean of 6.79, on a standard error of 0.024 \u{2014} a t of 2.4 on seventeen annual points."),
        ],
        answers: &["cannot say how many people were put out of their homes"],
        figures: &[
            Figure { label: "lowest", value: 766.0, literal: "766" },
            Figure { label: "mean", value: 865.0, literal: "865" },
            Figure { label: "highest", value: 982.0, literal: "982" },
        ],
    },
    Assertion {
        id: "busy-eviction-years-with-no-direction",
        statement: "The year-to-year spread in Allen County's eviction filings is twice what chance alone would give \u{2014} a standard deviation of 56 against a Poisson 29 \u{2014} and it goes nowhere. The county has busy years and quiet ones without having a trend.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-evictions-2001-2018.yml", "**The year-to-year movement is twice what chance alone would give and it goes nowhere.** A Poisson draw at a mean of 865 has a standard deviation of 29 and this series has 56."),
        ],
        answers: &["cannot say how many people were put out of their homes"],
        figures: &[
            Figure { label: "observed", value: 56.0, literal: "56" },
            Figure { label: "chance alone", value: 29.0, literal: "29" },
        ],
    },
    Assertion {
        id: "below-the-state-and-high-on-its-list",
        statement: "Allen County's eviction filing rate is below Ohio's and in the top fifth of Ohio's counties, and both are true. The state figure is weighted towards the large urban counties where the rate is highest; the ranking gives every county one place.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-evictions-2001-2018.yml", "**This county is below the state's rate and in the top fifth of its counties, and both are true.** The state figure is filings over renter households across all 87 reporting counties, so it is weighted towards the large urban ones where the rate is highest; the ranking gives every county one place regardless of size. Allen's rank runs from 8th to 22nd across the seventeen years, with a median of 14th."),
        ],
        answers: &["cannot say how many people were put out of their homes"],
        figures: &[],
    },
    Assertion {
        id: "a-filing-is-not-a-household",
        statement: "925 eviction cases were filed in Allen County in 2018 against 831 households. Between a tenth and a fifth of filings in any year are repeats against a household already sued that year.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-evictions-2001-2018.yml", "**A filing is not a household and neither is an eviction.** 925 cases were filed in 2018 against 831 households; the ratio runs from 1.100 to 1.195 across the run, so between a tenth and a fifth of filings in any year are repeats against a household already sued."),
            support!("measure/allen-county-evictions-2001-2018.yml", "This corpus cannot say how many people were put out of their homes."),
        ],
        answers: &["cannot say how many people were put out of their homes"],
        figures: &[
            Figure { label: "cases filed", value: 925.0, literal: "925" },
            Figure { label: "households sued", value: 831.0, literal: "831" },
        ],
    },
    Assertion {
        id: "the-burden-is-all-at-the-bottom",
        statement: "This county's housing cost problem is almost entirely a problem of its poorest \
                    households. Of those living on less than 30 per cent of area median income, \
                    61.9 per cent pay more than half of it for housing; above 80 per cent of area \
                    median the burden is close to absent.",
        topic: "housing",
        supports: &[
            support!(
                "measure/allen-county-housing-cost-burden-2006-2022.yml",
                "**The burden is almost entirely at the bottom of the income scale**, and above the area median it is close to absent. Of the 4,060 households living on less than 30 per cent of area median income, 2,515 pay more than half of it for housing \u{2014} 61.9 per cent. Of the 23,700 households above 80 per cent of the area median, 680 pay more than 30 per cent of income, which is 2.9 per cent of them."
            ),
            support!(
                "measure/allen-county-housing-cost-burden-2006-2022.yml",
                "1,600 of the county's 2,540 renter households below 30 per cent of area median income pay more than half of that income for shelter, which is 63.0 per cent of them; among owners in the same income band it is 915 of 1,520, or 60.2."
            ),
        ],
        answers: &["cannot say whether Lima's renters are more often cost-burdened"],
        figures: &[
            Figure { label: "all households under 30% of area median", value: 61.9, literal: "61.9" },
            Figure { label: "renters under 30%", value: 63.0, literal: "63.0" },
            Figure { label: "owners under 30%", value: 60.2, literal: "60.2" },
        ],
    },
    Assertion {
        id: "the-county-moved-and-the-country-did-not",
        statement: "Allen County's renters were as likely to be cost-burdened as the nation's \
                    through two five-year windows. By 2018\u{2013}2022 they were 8.7 points less \
                    likely, on a margin of 3.7 points.",
        topic: "housing",
        supports: &[support!(
            "measure/allen-county-housing-cost-burden-2006-2022.yml",
            "**Against the country, this county has moved and the country has not.** The county's four windows read 44.2, 45.9, 40.5 and 35.7 per cent and the nation's read 45.0, 45.8, 43.9 and 44.4. In the two earliest windows Allen County's renters were as likely to be cost-burdened as the nation's; in the most recent they are 8.7 points less likely, and the county's own margin on that rate is 3.7 points."
        )],
        answers: &["cannot say whether Lima's renters are more often cost-burdened"],
        figures: &[
            Figure { label: "Allen 2006\u{2013}2010", value: 44.2, literal: "44.2" },
            Figure { label: "Allen 2011\u{2013}2015", value: 45.9, literal: "45.9" },
            Figure { label: "Allen 2015\u{2013}2019", value: 40.5, literal: "40.5" },
            Figure { label: "Allen 2018\u{2013}2022", value: 35.7, literal: "35.7" },
            Figure { label: "US 2006\u{2013}2010", value: 45.0, literal: "45.0" },
            Figure { label: "US 2011\u{2013}2015", value: 45.8, literal: "45.8" },
            Figure { label: "US 2015\u{2013}2019", value: 43.9, literal: "43.9" },
            Figure { label: "US 2018\u{2013}2022", value: 44.4, literal: "44.4" },
        ],
    },
    Assertion {
        id: "thirty-nine-homes-for-a-hundred-households",
        statement: "For a hundred of Allen County's poorest renter households there are eighty-one \
                    homes cheap enough for them and thirty-nine they could actually move into. The \
                    rest of the cheap stock is lived in by someone who earns more.",
        topic: "housing",
        supports: &[support!(
            "measure/allen-county-housing-cost-burden-2006-2022.yml",
            "**For a hundred of the county's poorest renter households there are eighty-one homes they could afford and thirty-nine they could move into.** 2,050 rental units in the county rent at or below what a household at 30 per cent of area median could pay \u{2014} 1,945 occupied and 105 standing empty and for rent \u{2014} against 2,540 such households; but 1,065 of the occupied ones house a household with a higher income, leaving 985."
        )],
        answers: &["cannot say whether Lima's renters are more often cost-burdened"],
        figures: &[
            Figure { label: "households needing one", value: 2540.0, literal: "2,540" },
            Figure { label: "homes they could afford", value: 2050.0, literal: "2,050" },
            Figure { label: "of those, occupied by a higher income", value: 1065.0, literal: "1,065" },
            Figure { label: "available to them", value: 985.0, literal: "985" },
        ],
    },
    Assertion {
        id: "a-black-renter-pays-more-of-less",
        statement: "A Black renter household in Allen County is 1.7 times as likely as a white one \
                    to be paying more than half its income for housing \u{2014} 26.5 per cent \
                    against 15.6.",
        topic: "housing",
        supports: &[support!(
            "measure/allen-county-housing-cost-burden-2006-2022.yml",
            "**A Black renter household is 1.7 times as likely to be paying more than half its income.** Of 9,310 white non-Hispanic renter households 1,450 do, 15.6 per cent; of 2,435 Black non-Hispanic renter households 645 do, 26.5. On the 30 per cent threshold the gap is 32.5 per cent against 43.5."
        )],
        answers: &["cannot say whether Lima's renters are more often cost-burdened"],
        figures: &[
            Figure { label: "white renters over 50%", value: 15.6, literal: "15.6" },
            Figure { label: "Black renters over 50%", value: 26.5, literal: "26.5" },
            Figure { label: "white renters over 30%", value: 32.5, literal: "32.5" },
            Figure { label: "Black renters over 30%", value: 43.5, literal: "43.5" },
        ],
    },
    Assertion {
        id: "a-house-worth-less-than-in-1975",
        statement: "A house in Allen County is worth 14.5 per cent less than it was fifty years ago. \
                    The price index stands at 511.78 against a 1975 base of 100 and the consumer \
                    price level stands 5.98 times as high, so 5.12 times the money buys less of a \
                    house than it did.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-1975-2025.yml", "**Nominally a house here is worth 5.12 times its 1975 level and the price level is 5.98 times its 1975 level, so in real terms it is worth 14.5 per cent less than it was fifty years ago.** The index reads 511.78 against 100 and the consumer price index 321.943 against 53.8. [verified] \u{2014} the same source and [the CPI](../../catalog/bls-cpi.md), deflated here as [a deflator is a choice](../../decisions/a-deflator-is-a-choice.yml) requires."),
        ],
        answers: &["cannot say what a house in Allen County sold for in 1975"],
        figures: &[
            Figure { label: "index, 1975 = 100", value: 511.78, literal: "511.78" },
            Figure { label: "real fall since 1975, per cent", value: 14.5, literal: "14.5" },
        ],
    },
    Assertion {
        id: "the-peak-was-1979",
        statement: "The most a house in Allen County has ever been worth was in 1979. In 1975 \
                    dollars the index reached 103.4 that year, fell 30.8 per cent in the three years \
                    to 1982, and has climbed back to 85.5 \u{2014} still 17.3 per cent below the \
                    reading of forty-six years ago.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-1975-2025.yml", "**The peak was 1979.** In 1975 dollars the index reaches 103.4 that year and has never been there since. It fell 30.8 per cent in the three years to 1982, to 71.6, and the forty-three years after that are a long trough and a recent climb that has reached 85.5 \u{2014} still 17.3 per cent below the 1979 reading. [verified] \u{2014} the same file, deflated as above. Nothing in this series is a recovery to a previous level, because there has not been one."),
        ],
        answers: &["cannot say what a house in Allen County sold for in 1975"],
        figures: &[
            Figure { label: "1979 peak, 1975 = 100", value: 103.4, literal: "103.4" },
            Figure { label: "real fall to 1982, per cent", value: 30.8, literal: "30.8" },
        ],
    },
    Assertion {
        id: "the-floor-was-2014",
        statement: "Read in dollars this county barely had a housing bust \u{2014} 9.6 per cent from \
                    294.66 in 2006 to 266.33 in 2013. Read in what those dollars bought it had a \
                    thirty-five-year one: the real index fell in 20 of the 35 years from 1980 and \
                    bottomed at 60.8 in 2014, 41.3 per cent below the 1979 peak.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-1975-2025.yml", "**The floor is 2014, not 2011.** The real index falls in 20 of the 35 years from 1980, bottoms at 60.8 in 2014 \u{2014} 41.3 per cent below the 1979 peak \u{2014} and has risen in all 11 years since. Nominally the fall is far smaller and later: 294.66 in 2006 to 266.33 in 2013, 9.6 per cent. [verified] \u{2014} the same file. Read in dollars this county barely had a housing bust; read in what those dollars bought, it had a thirty-five-year one."),
        ],
        answers: &["cannot say what a house in Allen County sold for in 1975"],
        figures: &[
            Figure { label: "real trough, 1975 = 100", value: 60.8, literal: "60.8" },
            Figure { label: "nominal fall 2006-2013, per cent", value: 9.6, literal: "9.6" },
        ],
    },
    Assertion {
        id: "forty-four-counties-below-1975",
        statement: "Forty-four counties in America are worth less in real terms than they were in \
                    1975, and eight of them are in Ohio \u{2014} more than any other state. Of the \
                    419 counties with an index in both years the median stands at 146.6 and Allen \
                    County ranks fifteenth from the bottom at 85.5.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-1975-2025.yml", "**Forty-four counties in America are worth less in real terms than in 1975, and eight of them are in Ohio.** 419 counties have an index in both 1975 and 2025; their median stands at 146.6 in 1975 dollars, Allen County ranks 15th from the bottom of them at 85.5, and no state contributes more of the 44 than Ohio. Within Ohio's 29 measured counties only Marion at 69.6 and Lucas at 75.1 are lower. [verified] \u{2014} the same file, ranked here. A county entering the file later is not in this comparison at all, so the denominator is stated: 419 of 2,795. [inference]"),
        ],
        answers: &["cannot say what a house in Allen County sold for in 1975"],
        figures: &[
            Figure { label: "counties measured in both years", value: 419.0, literal: "419" },
            Figure { label: "counties below their 1975 level", value: 44.0, literal: "44" },
        ],
    },
    Assertion {
        id: "the-nation-gained-what-this-county-lost",
        statement: "The country's houses are worth 91.1 per cent more in real terms than they were \
                    in 1975 and Allen County's are worth 14.5 per cent less. On the same estimator \
                    and the same base the United States index reads 191.1 in 1975 dollars against \
                    this county's 85.5, a ratio of 2.23.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-1975-2025.yml", "**The nation's houses gained what this county's lost.** On the same estimator and the same base year, the United States index stands at 191.1 in 1975 dollars in 2025 against Allen County's 85.5 \u{2014} a ratio of 2.23. The country's houses are worth 91.1 per cent more in real terms than in 1975 and this county's are worth 14.5 per cent less. [verified] \u{2014} the same source, its national quarterly series, averaged over its four quarters and deflated here."),
        ],
        answers: &["cannot say what a house in Allen County sold for in 1975"],
        figures: &[
            Figure { label: "United States, 1975 = 100", value: 191.1, literal: "191.1" },
            Figure { label: "Allen County, 1975 = 100", value: 85.5, literal: "85.5" },
        ],
    },
    Assertion {
        id: "the-jobs-came-back-the-prices-did-not",
        statement: "The three years house prices collapsed here are the three years the factories \
                    emptied \u{2014} manufacturing employment 18,389 in 1979 and 14,349 in 1982, a \
                    fall of 4,040 jobs and 22.0 per cent, against a real price fall of 30.8 per \
                    cent. Then the jobs came back and the prices did not: manufacturing recovered \
                    19.6 per cent by 1986 while the index moved 2.9.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-1975-2025.yml", "**The three years the price collapsed are the three years the factories emptied.** Manufacturing employment in this county was 18,389 in 1979 and 14,349 in 1982 \u{2014} 4,040 jobs, 22.0 per cent \u{2014} and the real index fell 30.8 per cent across the same three years. [verified] \u{2014} [the employment series](allen-county-manufacturing-employment-1969-2022.yml) and [the index](../../catalog/fhfa-house-price-index.md) above."),
            support!("measure/allen-county-house-prices-1975-2025.yml", "**Then the jobs came back and the prices did not.** Manufacturing recovered to 17,163 by 1986, 19.6 per cent above its 1982 floor, while the real index moved from 71.6 to 73.7 \u{2014} 2.9 per cent. Over the twenty-three years after that manufacturing lost 55 per cent of what was left and the real index moved 2.5 per cent, downward. [inference] \u{2014} computed here from the same two series. The market repriced once, at the first shock, and the larger losses that followed moved it hardly at all."),
        ],
        answers: &["cannot say what a house in Allen County sold for in 1975"],
        figures: &[
            Figure { label: "manufacturing jobs lost 1979-1982", value: 4040.0, literal: "4,040" },
            Figure { label: "manufacturing recovery to 1986, per cent", value: 19.6, literal: "19.6" },
        ],
    },
    Assertion {
        id: "seven-lima-tracts-with-no-price",
        statement: "The federal price index holds 28 of Allen County's 35 census tracts, and all \
                    seven it omits are in Lima. Nineteen of nineteen tracts outside the city have an \
                    index and nine of the city's sixteen do, leaving 5,698 housing units and 11,543 \
                    people \u{2014} 35.6 per cent of Lima's housing stock and none of anybody else's \
                    \u{2014} with no house price index at all.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-by-tract-1986-2025.yml", "**The file holds 28 of the county's 35 census tracts, and all seven it omits are in Lima.** Nineteen of nineteen tracts outside the city have an index. Nine of the city's sixteen do. [verified] \u{2014} the same file, against [the block assignment](../../catalog/census-block-assignment-2020.md) that decides which tracts are majority-Lima. Five of the seven lie entirely inside the city, one is 95.3 per cent inside and one 71.3."),
            support!("measure/allen-county-house-prices-by-tract-1986-2025.yml", "**That is 5,698 housing units and 11,543 people with no house price index at all**, 35.6 per cent of Lima's housing stock and none of anybody else's. [verified] \u{2014} the same file and the block assignment, computed here."),
        ],
        answers: &["cannot say what happened to house prices in the seven Lima tracts the index does not reach"],
        figures: &[
            Figure { label: "Lima housing units with no index", value: 5698.0, literal: "5,698" },
            Figure { label: "people in those tracts", value: 11543.0, literal: "11,543" },
        ],
    },
    Assertion {
        id: "the-index-stops-where-the-lending-stops",
        statement: "The seven tracts with no price index are exactly the seven where the fewest \
                    mortgages in the county are written \u{2014} ranks 1 through 7 of 35 on \
                    originations per thousand housing units and ranks 1 through 7 again on the loans \
                    Fannie Mae and Freddie Mac went on to buy, with no overlap at the boundary. \
                    Under any seven of thirty-five the chance of that is one in 6,724,520.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-by-tract-1986-2025.yml", "**The seven are exactly the seven tracts where the fewest mortgages are written.** Ranked by loans originated per thousand housing units across 2018\u{2013}2024, they are ranks 1 through 7 of 35; ranked by loans that Fannie Mae or Freddie Mac went on to buy, they are ranks 1 through 7 of 35 again. The two lists are the same seven tracts in a different order, and the groups do not overlap at the boundary: the highest unindexed tract writes 38.7 enterprise-purchased loans per thousand units and the lowest indexed tract writes 41.0. [verified] \u{2014} [the lending register](../../catalog/hmda-loan-application-register.md) and the tract file, computed here."),
            support!("measure/allen-county-house-prices-by-tract-1986-2025.yml", "**Two federal agencies draw the same line through one city and neither of them draws it.** The Federal Housing Finance Agency publishes a price index and says only that it withholds one where the sample is small; the Consumer Financial Protection Bureau publishes a register of applications and says nothing about indexes. If the seven omitted tracts were any seven of thirty-five, the chance of their being precisely the seven with the least lending is one in 6,724,520. [inference] \u{2014} computed here. What the index cannot measure is what the enterprises did not buy, and this is the measurement of it; see [a gap in an index maps its instrument](../../decisions/a-gap-in-an-index-maps-its-instrument.yml)."),
        ],
        answers: &["cannot say what happened to house prices in the seven Lima tracts the index does not reach"],
        figures: &[
            Figure { label: "highest unindexed tract, GSE loans per 1,000 units", value: 38.7, literal: "38.7" },
            Figure { label: "lowest indexed tract", value: 41.0, literal: "41.0" },
        ],
    },
    Assertion {
        id: "where-the-index-sees-lima",
        statement: "Where the index reaches inside Lima the city moved with the county: a median \
                    real change of 108.7 against 111.1 outside across 2000 to 2025, with the \
                    county's highest and lowest tracts both inside the city. That covers nine of \
                    Lima's sixteen tracts and 10,324 of its 16,022 homes, and those nine are the \
                    nine that rank highest on the one variable deciding whether a tract appears in \
                    the file at all.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-by-tract-1986-2025.yml", "**Where the index can see Lima, Lima moves with the county.** Across 2000 to 2025 the median tract change in 1975-style real terms is 108.7 for the five Lima tracts with both endpoints and 111.1 for the eighteen outside \u{2014} 2.4 points apart over twenty-five years \u{2014} and the county's highest and lowest tracts are both inside the city, at 131.4 and 87.9. [verified] \u{2014} the same file and [the CPI](../../catalog/bls-cpi.md), computed here."),
            support!("measure/allen-county-house-prices-by-tract-1986-2025.yml", "**The nine indexed Lima tracts are not a sample of Lima.** They are the nine of sixteen that rank highest on the one variable that decides whether a tract appears in the file at all. So the finding above is a statement about 10,324 of the city's 16,022 homes and cannot be widened to the other 5,698 by any argument from the nine. [inference] See [what a tract page may be quoted for](../../decisions/what-a-tract-page-may-be-quoted-for.yml)."),
        ],
        answers: &["cannot say what happened to house prices in the seven Lima tracts the index does not reach"],
        figures: &[
            Figure { label: "Lima tracts, median real change", value: 108.7, literal: "108.7" },
            Figure { label: "tracts outside the city", value: 111.1, literal: "111.1" },
        ],
    },
    Assertion {
        id: "a-tax-base-at-half-the-market-pace",
        statement: "Allen County's residential tax base rose 33.7 per cent between the 2014 and 2023 \
                    audited reports while the market for the same houses rose 59.6 per cent across \
                    the January values those reports rest on. The base gains every house built in \
                    those nine years and the index measures only houses that changed hands twice, so \
                    an assessment tracking the market would have outrun 59.6 rather than reached \
                    33.7.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-house-prices-1975-2025.yml", "**The tax base trails the market, and the gap is large enough to matter to a levy.** Residential assessed valuation in the county's audited reports rose 33.7 per cent between the 2014 and 2023 reports, from $1,149.3 million to $1,537.1 million. Those reports rest on January values of 2013 and 2022, across which the index rose 59.6 per cent. [inference] \u{2014} computed here from [the audited reports](allen-county-assessed-valuation-2010-2023.yml) and the index above. The base moves in steps at reappraisal and includes new houses the index excludes, so it should outrun the market and instead runs at half its pace."),
        ],
        answers: &["cannot say what a house in Allen County sold for in 1975"],
        figures: &[
            Figure { label: "assessed residential, per cent", value: 33.7, literal: "33.7" },
            Figure { label: "price index, per cent", value: 59.6, literal: "59.6" },
        ],
    },
    Assertion {
        id: "income-rose-and-earnings-barely-did",
        statement: "Income per person in Allen County is 74.3 per cent higher in real terms than it \
                    was in 1969 and the money people are paid for working is 27.0 per cent higher \
                    \u{2014} $25,565 to $32,471 in 2024 dollars, which is 0.44 per cent a year \
                    across fifty-five years.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-personal-income-1969-2024.yml", "**Income per person here is 74.3 per cent higher in real terms than it was in 1969 and the money people are paid for working is 27.0 per cent higher.** $32,164 to $56,066 against $25,565 to $32,471, both in 2024 dollars. [verified] \u{2014} the same source, deflated by [the CPI](../../catalog/bls-cpi.md) as [a deflator is a choice](../../decisions/a-deflator-is-a-choice.yml) requires. Over fifty-five years that is 0.44 per cent a year on the earnings line."),
        ],
        answers: &["cannot say what a person in Allen County is paid"],
        figures: &[
            Figure { label: "real income per person, 1969", value: 32164.0, literal: "32,164" },
            Figure { label: "2024", value: 56066.0, literal: "56,066" },
        ],
    },
    Assertion {
        id: "twenty-years-to-get-back-to-1979",
        statement: "Real earnings per person in Allen County fell 20.1 per cent in the three years \
                    to 1982 and no year stood above the 1979 figure until 1999. Those are the same \
                    three years in which the county's house prices fell 30.8 per cent in real terms \
                    and never recovered at all.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-personal-income-1969-2024.yml", "**Real earnings per person fell 20.1 per cent in the three years to 1982 and took until 1999 to get back.** $27,696 in 1979, $22,140 in 1982, and no year above the 1979 figure until 1999's $28,746. [verified] \u{2014} the same file. Twenty years is the length of that hole and it is the same three years in which [house prices](allen-county-house-prices-1975-2025.yml) fell 30.8 per cent in real terms and never recovered at all."),
        ],
        answers: &["cannot say what a person in Allen County is paid"],
        figures: &[
            Figure { label: "real earnings per person, 1979", value: 27696.0, literal: "27,696" },
            Figure { label: "1982", value: 22140.0, literal: "22,140" },
        ],
    },
    Assertion {
        id: "a-job-here-pays-what-it-paid-in-1969",
        statement: "A job in Allen County pays about what a job in Allen County paid in 1969. \
                    Earnings by place of work divided by total jobs gives $62,412 in 1969 and \
                    $65,065 in 2022 in the same dollars \u{2014} 4.3 per cent higher after \
                    fifty-three years, and 1.2 per cent below the 1979 figure of $65,825.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-personal-income-1969-2024.yml", "**A job here pays about what a job here paid in 1969.** Earnings by place of work divided by total jobs gives $62,412 in 1969, $65,825 in 1979, $59,047 in 1982 and $65,065 in 2022, all in 2024 dollars \u{2014} 4.3 per cent above the first figure after fifty-three years and 1.2 per cent below the second. [inference] \u{2014} computed here from the same file and [the employment series](allen-county-total-employment-1969-2022.yml)."),
        ],
        answers: &["cannot say what a person in Allen County is paid"],
        figures: &[
            Figure { label: "1969", value: 62412.0, literal: "62,412" },
            Figure { label: "1979", value: 65825.0, literal: "65,825" },
            Figure { label: "1982", value: 59047.0, literal: "59,047" },
            Figure { label: "2022", value: 65065.0, literal: "65,065" },
        ],
    },
    Assertion {
        id: "half-the-growth-is-transfers",
        statement: "More than half of fifty-five years of income growth in Allen County is \
                    government transfers. Of the $23,902 that real income per person gained between \
                    1969 and 2024, transfers are $12,701 \u{2014} 53.1 per cent \u{2014} against \
                    28.9 per cent from net earnings and 18.0 from dividends, interest and rent.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-personal-income-1969-2024.yml", "**More than half of fifty-five years of income growth is transfer payments.** Of the $23,902 that real income per person gained between 1969 and 2024, transfers are $12,701 \u{2014} 53.1 per cent \u{2014} against 28.9 per cent from net earnings and 18.0 from dividends, interest and rent. [inference] \u{2014} computed here from the table above."),
        ],
        answers: &["cannot say what a person in Allen County is paid"],
        figures: &[
            Figure { label: "transfers, share of growth", value: 53.1, literal: "53.1" },
            Figure { label: "net earnings", value: 28.9, literal: "28.9" },
            Figure { label: "dividends, interest and rent", value: 18.0, literal: "18.0" },
        ],
    },
    Assertion {
        id: "a-quarter-of-the-income-is-a-transfer",
        statement: "Government transfers went from 7.2 per cent of Allen County's income in 1969 to \
                    26.8 per cent in 2024. Per person and in 2024 dollars they are 6.52 times what \
                    they were \u{2014} $2,299 against $15,000, having peaked at $17,918 in 2021.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-personal-income-1969-2024.yml", "**Transfer receipts went from 7.2 per cent of this county's income to 26.8.** Per person and in 2024 dollars they are 6.52 times what they were \u{2014} $2,299 in 1969 against $15,000 in 2024, having peaked at $17,918 in 2021. [verified] \u{2014} the same source. What they consist of is [its own question](allen-county-transfer-receipts-1969-2024.yml)."),
        ],
        answers: &["cannot say what a person in Allen County is paid"],
        figures: &[
            Figure { label: "transfers per person, 1969", value: 2299.0, literal: "2,299" },
            Figure { label: "2021", value: 17918.0, literal: "17,918" },
            Figure { label: "2024", value: 15000.0, literal: "15,000" },
        ],
    },
    Assertion {
        id: "ordinary-for-ohio-and-ohio-is-not-its-median",
        statement: "Allen County ranks 41st of Ohio's 88 counties on transfer share and 39th on \
                    income per person \u{2014} ordinary on both \u{2014} while the state aggregate \
                    sits at 21.1 per cent and $64,464, because four large counties hold enough of \
                    Ohio's income to move its total and not its middle.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-personal-income-1969-2024.yml", "**The county is ordinary for Ohio and Ohio is not its own median.** Allen County ranks 41st of Ohio's 88 counties on transfer share at 26.8 per cent against a county median of 26.0, and 39th on income per person at $56,066 against a county median of $55,326. The state aggregate is 21.1 per cent and $64,464, because four large counties hold enough of Ohio's income to move its total and not its middle. [verified] \u{2014} the same file, all 88 Ohio rows, ranked here. A county compared against a state total is being compared against its four richest neighbours."),
        ],
        answers: &["cannot say what a person in Allen County is paid"],
        figures: &[
            Figure { label: "Allen County, transfer share", value: 26.8, literal: "26.8" },
            Figure { label: "Ohio county median", value: 26.0, literal: "26.0" },
            Figure { label: "Ohio aggregate", value: 21.1, literal: "21.1" },
        ],
    },
    Assertion {
        id: "the-gap-with-the-nation-is-in-the-growth-rate",
        statement: "Real income per person rose 74.3 per cent in Allen County between 1969 and 2024, \
                    90.6 per cent in Ohio and 117.9 per cent in the United States. The transfer \
                    share in 2024 is 26.8 per cent here against 18.3 nationally.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-personal-income-1969-2024.yml", "**Against the nation the gap is real and it is in the growth rate.** Real income per person rose 74.3 per cent here, 90.6 in Ohio and 117.9 in the United States between 1969 and 2024, and the transfer share in 2024 is 26.8 per cent here against 18.3 nationally. [verified] \u{2014} the same file, the Ohio and United States rows."),
        ],
        answers: &["cannot say what a person in Allen County is paid"],
        figures: &[
            Figure { label: "Allen County", value: 74.3, literal: "74.3" },
            Figure { label: "Ohio", value: 90.6, literal: "90.6" },
            Figure { label: "United States", value: 117.9, literal: "117.9" },
        ],
    },
    Assertion {
        id: "half-a-billion-in-earnings-goes-home-elsewhere",
        statement: "Allen County pays out more in earnings than its residents take home, and has in \
                    every one of the fifty-six years measured. The adjustment for residence is \
                    \u{2212}$516,666,000 in 2024, 12.0 per cent of the $4,310,695,000 earned inside \
                    the county line, against a commuting file that counts 13.8 per cent more jobs \
                    here than resident job-holders.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-personal-income-1969-2024.yml", "**The county pays out more in earnings than its residents take home, and has in every one of the fifty-six years.** The adjustment for residence is \u{2212}$516,666,000 in 2024, 12.0 per cent of the $4,310,695,000 earned inside the county line; it was \u{2212}13.3 per cent in 1969 and reached \u{2212}16.0 in 1990. [verified] \u{2014} the same file. Against that, [the commuting file](allen-county-commuting-2022.yml) counts 48,730 jobs located here and 42,835 held by residents, 13.8 per cent more jobs than resident job-holders. Two agencies, two methods and 1.8 points apart; BEA estimates its adjustment from journey-to-work data, so this is a close agreement between measurements that share an ancestor rather than two independent ones. [inference] See [an exact match is a question](../../decisions/an-exact-match-is-a-question.yml)."),
        ],
        answers: &["cannot say what a person in Allen County is paid"],
        figures: &[
            Figure { label: "earnings by place of work, $000", value: 4310695.0, literal: "4,310,695" },
            Figure { label: "earnings paid out to non-residents, $000", value: 516666.0, literal: "516,666" },
        ],
    },
    Assertion {
        id: "half-the-transfers-are-medical",
        statement: "A quarter of everything the people of Allen County are paid is a government \
                    transfer, and half of that is medical care: $668,013,000 in 2022, or $6,608 a \
                    resident, of which Medicare is $389,761,000 and public assistance medical care \
                    $276,551,000. Social Security is $4,131 a resident.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-transfer-receipts-1969-2024.yml", "A quarter of everything the people of this county are paid is a government transfer, and half of that is medical care: **$668,013,000 of medical benefits in 2022, or $6,608 a resident**, of which Medicare is $389,761,000 and public assistance medical care $276,551,000, against $4,131 a resident of Social Security. [verified] \u{2014} [BEA's transfer receipts tables](../../catalog/bea-county-personal-income.md), CAINC4 for the totals and CAINC35 for what they are made of."),
        ],
        answers: &["cannot say how many people in Allen County receive any of this money"],
        figures: &[
            Figure { label: "Medicare, $000", value: 389761.0, literal: "389,761" },
            Figure { label: "public assistance medical care, $000", value: 276551.0, literal: "276,551" },
        ],
    },
    Assertion {
        id: "the-composition-inverted",
        statement: "In 1969 retirement and disability benefits were 59.6 per cent of Allen County's \
                    government transfers and medical benefits were 12.3. In 2022 medical benefits \
                    are 48.6 per cent and retirement and disability 31.2. The largest single thing \
                    the government now does for the people here is buy them health care.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-transfer-receipts-1969-2024.yml", "**The composition inverted inside one lifetime.** In 1969 retirement and disability were 59.6 per cent of this county's transfers and medical benefits were 12.3; in 2022 medical benefits are 48.6 and retirement and disability 31.2. [verified] \u{2014} the same table, both years. The largest single thing the government now does for the people of Allen County is buy them health care."),
        ],
        answers: &["cannot say how many people in Allen County receive any of this money"],
        figures: &[
            Figure { label: "medical, 1969", value: 12.3, literal: "12.3" },
            Figure { label: "medical, 2022", value: 48.6, literal: "48.6" },
            Figure { label: "retirement, 1969", value: 59.6, literal: "59.6" },
            Figure { label: "retirement, 2022", value: 31.2, literal: "31.2" },
        ],
    },
    Assertion {
        id: "two-federal-accounts-of-one-county",
        statement: "Two federal accounts of Allen County differ by a factor of two and a half. \
                    USAspending records $626,459,645 of federal money here in federal year 2024; the \
                    Bureau of Economic Analysis records $1,513,000,000 of transfer receipts in \
                    calendar 2024. One counts awards obligated to a place and the other counts \
                    income received by the people who live in it.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-transfer-receipts-1969-2024.yml", "**Two federal accounts of the same county in the same year differ by a factor of two and a half.** USAspending records $626,459,645 of federal money in Allen County in federal year 2024, of which $573,202,179 is direct payments to individuals. BEA records $1,513,000,000 of transfer receipts in calendar 2024. [verified] \u{2014} [USAspending](../../catalog/usaspending.md), place of performance, and the same BEA file; see [the federal money node](federal-money-in-allen-county-2008-2025.yml)."),
        ],
        answers: &["cannot say how many people in Allen County receive any of this money"],
        figures: &[
            Figure { label: "USAspending, all award types", value: 626459645.0, literal: "626,459,645" },
            Figure { label: "BEA transfer receipts", value: 1513000000.0, literal: "1,513,000,000" },
        ],
    },
    Assertion {
        id: "the-difference-is-medicare-and-medicaid",
        statement: "The gap between the two federal accounts is medical care, and both sides of it \
                    can be checked. In 2022 the difference is $878,386,986 and BEA's medical \
                    benefits for that year are $668,013,000 \u{2014} 76.0 per cent of it. All 99 of \
                    USAspending's assistance listings for the county that year total $550,548,594 \
                    and not one of them is Medicare or Medicaid.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-transfer-receipts-1969-2024.yml", "**The difference is medical care, and both sides of that can be checked.** In 2022 the gap between BEA's transfers and USAspending's direct payments is $878,386,986, and BEA's medical benefits for that year are $668,013,000 \u{2014} 76.0 per cent of it. On the other side, USAspending's 99 assistance listings for Allen County in federal year 2022 total $550,548,594 and **not one of them is Medicare or Medicaid**; its five listings from the Department of Health and Human Services come to $6,415,154, of which $5,825,306 is Head Start. [verified] \u{2014} the same source and the same award file, the assistance listing endpoint queried for that year. A Medicare payment goes to a hospital and is not an award to anybody in this county; BEA books it to the patient's county all the same. See [a receipt is not an award](../../decisions/a-receipt-is-not-an-award.yml)."),
        ],
        answers: &["cannot say how many people in Allen County receive any of this money"],
        figures: &[
            Figure { label: "the gap", value: 878386986.0, literal: "878,386,986" },
            Figure { label: "BEA medical benefits", value: 668013000.0, literal: "668,013,000" },
        ],
    },
    Assertion {
        id: "three-hundred-and-thirty-seven-feet",
        statement: "Allen County runs from 733.5 feet to 1,071.2 feet above sea level \u{2014} 337.7 \
                    feet of natural relief across 402 square miles, on a mean of 859.1 and a median \
                    of 842.3. These are the first elevations this corpus has held for any ground in \
                    the county.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-elevation-2026.yml", "**The county runs from 733.5 feet to 1,071.2 feet and that is 337.7 feet of relief across 402 square miles.** Its mean is 859.1 feet and its median 842.3. [verified] \u{2014} the same source, computed here, with the quarry described below excluded."),
        ],
        answers: &["cannot say how much stone has come out of that hole"],
        figures: &[
            Figure { label: "lowest natural ground, feet", value: 733.5, literal: "733.5" },
            Figure { label: "highest, feet", value: 1071.2, literal: "1,071.2" },
        ],
    },
    Assertion {
        id: "the-high-point-is-on-the-divide",
        statement: "The highest ground in Allen County is where the continental divide enters it \
                    \u{2014} 200 metres from the point at which the boundary between the Great Lakes \
                    and Ohio hydrologic regions crosses the county's southern line. The lowest \
                    natural ground is at the opposite corner.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-elevation-2026.yml", "**Its highest ground is where the continental divide enters it.** The high point is at 40.644716, \u{2212}83.900309, in the county's south-east corner in [Auglaize Township](../place/auglaize-township.yml), 200 metres from the point where the boundary between the Great Lakes and Ohio hydrologic regions crosses the county's southern line. [verified] \u{2014} the same surface and [the watershed boundary](../../catalog/usgs-watershed-boundary.md), computed here. The lowest natural ground is at 40.883216, \u{2212}83.891034, at the opposite corner."),
        ],
        answers: &["cannot say how much stone has come out of that hole"],
        figures: &[],
    },
    Assertion {
        id: "nine-miles-of-continental-divide",
        statement: "The boundary between the Great Lakes and Ohio hydrologic regions runs 9.196 \
                    miles inside Allen County, and every foot of it is in Auglaize Township. It \
                    enters at the county's southern line and leaves at the eastern one, cutting off \
                    the south-east corner.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-elevation-2026.yml", "**The divide runs 9.196 miles inside Allen County and every foot of it is in Auglaize Township.** It enters at the county's southern line at 40.644643, \u{2212}83.902735 and leaves at the eastern line at 40.721021, \u{2212}83.879880, cutting off the county's south-east corner. [verified] \u{2014} [the WBDLine layer](../../catalog/usgs-watershed-boundary.md) at `hudigit` 2, clipped to [TIGER's county polygon](../../catalog/census-tiger-roads.md) and measured in EPSG:6549."),
        ],
        answers: &["cannot say how much stone has come out of that hole"],
        figures: &[
            Figure { label: "miles of divide inside the county", value: 9.196, literal: "9.196" },
        ],
    },
    Assertion {
        id: "a-divide-nine-miles-long-and-seventy-four-feet-tall",
        statement: "Ninety-eight elevations spaced 500 feet along Allen County's continental divide \
                    run from 991.3 to 1,065.9 feet \u{2014} 74.6 feet of rise and fall over nine \
                    miles, on a standard deviation of 20.2, with 55 of the 98 within twenty feet of \
                    the median. A person walking east across it would pass from the St. Lawrence \
                    watershed to the Gulf of Mexico without noticing a hill.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-elevation-2026.yml", "**It is one of the flattest pieces of continental divide there is.** Ninety-eight samples spaced 500 feet along it run from 991.3 to 1,065.9 feet \u{2014} 74.6 feet of rise and fall over nine miles, on a mean of 1,022.9 and a standard deviation of 20.2, with 55 of the 98 within 20 feet of the median. [verified] \u{2014} [the elevation point service](../../catalog/usgs-3dep-elevation.md), 98 queries. A person walking east across it would cross from the St. Lawrence watershed to the Gulf of Mexico without noticing a hill."),
        ],
        answers: &["cannot say how much stone has come out of that hole"],
        figures: &[
            Figure { label: "lowest sample, feet", value: 991.3, literal: "991.3" },
            Figure { label: "highest, feet", value: 1065.9, literal: "1,065.9" },
            Figure { label: "standard deviation, feet", value: 20.2, literal: "20.2" },
        ],
    },
    Assertion {
        id: "two-thousand-three-hundred-acres-drain-to-the-gulf",
        statement: "2,303.0 acres of Allen County drain to the Gulf of Mexico \u{2014} 3.5984 square \
                    miles and 0.885 per cent of it, all in Auglaize Township, against the 0.10 per \
                    cent of its population who live there. Nine times as much of this county's \
                    ground is on the far side of the divide as of its people, because the corner is \
                    farmland.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-elevation-2026.yml", "**The ground on the far side is 2,303.0 acres.** The Ohio Region occupies 3.5984 square miles of Allen County, 0.885 per cent of it, all in Auglaize Township, against the 0.10 per cent of the county's population who live there. [verified] \u{2014} [the same source's](../../catalog/usgs-watershed-boundary.md) region polygons, clipped to the county; see [the Scioto basin](../natural-feature/scioto-river-basin.yml). Nine times as much of this county drains to the Gulf as lives there, because the corner is farmland."),
        ],
        answers: &["cannot say how much stone has come out of that hole"],
        figures: &[
            Figure { label: "acres on the Ohio River side", value: 2303.0, literal: "2,303.0" },
            Figure { label: "per cent of the county", value: 0.885, literal: "0.885" },
        ],
    },
    Assertion {
        id: "the-lowest-point-is-a-hole-somebody-dug",
        statement: "The lowest elevation anywhere in Allen County is 474.7 feet, and it is a \
                    contiguous 47.7-acre depression whose extent contains the coordinate this corpus \
                    already held for the National Lime & Stone quarry. Its floor lies 365.3 feet \
                    below the median of the cells that ring it.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-elevation-2026.yml", "**The lowest elevation anywhere in this county is 474.7 feet and it is a hole somebody dug.** A contiguous depression of 47.7 acres between 40.74906 and 40.75237 north falls 365.3 feet below the median of the cells that ring it, and the coordinate this corpus already held for [the National Lime & Stone quarry](allen-county-quarries-2000-2025.yml) \u{2014} 40.750833, \u{2212}84.086111 \u{2014} lies inside it. [verified] \u{2014} the same surface, flood-filled from its minimum, against [the mine register](../../catalog/msha-mine-data.md)."),
        ],
        answers: &["cannot say how much stone has come out of that hole"],
        figures: &[
            Figure { label: "floor, feet", value: 474.7, literal: "474.7" },
            Figure { label: "acres", value: 47.7, literal: "47.7" },
            Figure { label: "depth below the rim, feet", value: 365.3, literal: "365.3" },
        ],
    },
    Assertion {
        id: "the-hole-is-deeper-than-the-county",
        statement: "The quarry pit in Bath Township is deeper than Allen County is tall: 365.3 feet \
                    from rim to floor against 337.7 feet between the county's highest and lowest \
                    natural ground, with the floor 258.8 feet below any ground in the county that \
                    nobody excavated.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-elevation-2026.yml", "**The hole is deeper than the county.** 365.3 feet from rim to floor against 337.7 feet between the county's highest and lowest natural ground, and the floor sits 258.8 feet below any ground in Allen County that nobody excavated. [inference] \u{2014} computed here from the figures above. That comparison is why the county's minimum cannot be published as a fact about its terrain; see [an extreme in a surface has an address](../../decisions/an-extreme-in-a-surface-has-an-address.yml)."),
        ],
        answers: &["cannot say how much stone has come out of that hole"],
        figures: &[
            Figure { label: "pit depth, feet", value: 365.3, literal: "365.3" },
            Figure { label: "county natural relief, feet", value: 337.7, literal: "337.7" },
        ],
    },
    Assertion {
        id: "nine-parts-beech",
        statement: "Allen County was nine parts beech forest and less than one part swamp. Two of \
                    the thirteen vegetation types Gordon mapped from the earliest land surveys occur \
                    here at all: 237,924.9 acres of beech forest and 22,433.9 acres of elm-ash swamp \
                    forest, which is 8.6 per cent of the county.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-original-vegetation.yml", "**Allen County was nine parts beech forest and less than one part swamp.** Two of Gordon's thirteen types occur here at all: **237,924.9 acres of beech forest**, and the swamp forest that gives [the Great Black Swamp](../natural-feature/great-black-swamp.yml) its name at **22,433.9 acres** of 260,358.7 \u{2014} 8.6 per cent. [verified] \u{2014} the same source. The two polygons' areas sum to the county's own area exactly, which is the check that the whole county is accounted for."),
        ],
        answers: &["cannot say when any of this was cleared"],
        figures: &[
            Figure { label: "beech forest, acres", value: 237924.9, literal: "237,924.9" },
            Figure { label: "elm-ash swamp forest, acres", value: 22433.9, literal: "22,433.9" },
        ],
    },
    Assertion {
        id: "it-is-not-one-swamp-it-is-nineteen",
        statement: "The Great Black Swamp inside Allen County is not one swamp. It is nineteen \
                    separate bodies, the largest 5,007.9 acres and eleven of them lying wholly \
                    inside the county line, and the biggest polygon that reaches in from outside \
                    extends west into Van Wert County rather than north.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-original-vegetation.yml", "**It is not one swamp. It is nineteen.** The swamp forest inside this county falls into 19 separate bodies, the largest 5,007.9 acres and eleven of them lying wholly inside the county line. The biggest polygon that reaches in from outside, 14,414.4 acres, extends **west** into Van Wert County rather than north. [verified] \u{2014} the same file, by polygon."),
        ],
        answers: &["cannot say when any of this was cleared"],
        figures: &[
            Figure { label: "separate bodies", value: 19.0, literal: "19" },
            Figure { label: "largest, acres", value: 5007.9, literal: "5,007.9" },
        ],
    },
    Assertion {
        id: "the-hydric-proxy-was-four-times-too-big",
        statement: "The hydric-soil proxy this corpus had been using for the Great Black Swamp \
                    overstated it by a factor of 4.1. Against 91,953 acres of hydric soil and 35.3 \
                    per cent of the county, the mapped swamp forest is 22,433.9 acres \u{2014} 24.4 \
                    per cent of the hydric acreage.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-original-vegetation.yml", "**The hydric-soil proxy overstated the swamp by a factor of 4.1.** This corpus has carried 91,953 acres of hydric soil, 35.3 per cent of the county, as \"the nearest thing to an answer\" for where the swamp was. The mapped swamp forest is 22,433.9 acres, 24.4 per cent of the hydric acreage. [verified] \u{2014} this file against [the soil survey](../../catalog/usda-ssurgo-soil-survey.md); see [the soils](allen-county-soils-2026.yml) and [a proxy must outlast what it proxies](../../decisions/a-proxy-must-outlast-what-it-proxies.yml)."),
        ],
        answers: &["cannot say when any of this was cleared"],
        figures: &[
            Figure { label: "hydric soil, acres", value: 91953.0, literal: "91,953" },
            Figure { label: "mapped swamp forest, acres", value: 22433.9, literal: "22,433.9" },
        ],
    },
    Assertion {
        id: "the-proxy-got-the-order-and-not-the-places",
        statement: "Across Allen County's twelve townships the hydric-soil proxy and the mapped \
                    swamp forest correlate at a Spearman 0.685 \u{2014} and Perry Township, fifth of \
                    twelve on hydric soil at 34.9 per cent, carries no swamp forest at all, as do \
                    Jackson, Bath and the city.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-original-vegetation.yml", "**The proxy got the order roughly and the places wrongly.** Ranked across the twelve townships the two measures agree at a Spearman correlation of 0.685, and the disagreements are not small: Perry Township is fifth on hydric soil at 34.9 per cent and has no swamp forest at all, while Shawnee is tenth on hydric at 26.2 and fifth on swamp at 10.8. [inference] \u{2014} computed here from the two files. Four townships with a third of their ground rated hydric \u{2014} Perry, Jackson, Bath and the city \u{2014} carried no swamp forest whatever."),
        ],
        answers: &["cannot say when any of this was cleared"],
        figures: &[
            Figure { label: "Spearman correlation", value: 0.685, literal: "0.685" },
            Figure { label: "Perry Township, per cent hydric", value: 34.9, literal: "34.9" },
        ],
    },
    Assertion {
        id: "three-quarters-of-the-wet-ground-was-beech",
        statement: "Three quarters of Allen County's hydric acres carried beech forest rather than \
                    swamp forest, which is what a lake plain that drains badly and still grows trees \
                    looks like. The wet ground and the swamp are different facts and this county is \
                    mostly the first.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-original-vegetation.yml", "**The wet ground and the swamp are different facts and the county is mostly the first.** Three quarters of this county's hydric acres carried beech forest rather than swamp forest, which is what a lake plain that drains badly and still grows trees looks like. [inference] \u{2014} computed from the two files above."),
        ],
        answers: &["cannot say when any of this was cleared"],
        figures: &[],
    },
    Assertion {
        id: "more-children-per-woman-than-ohio",
        statement: "Allen County has more children per woman than Ohio in every one of the \
                    twenty-two years measured \u{2014} a total fertility rate of 2,360.7 per 1,000 \
                    women against the state's 2,020.4 in 2000, and 1,954.2 against 1,712.4 in 2020. \
                    The lead runs between 6.6 and 19.0 per cent and never closes.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-birth-outcomes-2000-2021.yml", "**This county has more children per woman than Ohio, in every one of the twenty-two years.** Its total fertility rate is 2,360.7 per 1,000 women in 2000 against the state's 2,020.4, and 1,954.2 against 1,712.4 in 2020 \u{2014} a lead of between 6.6 and 19.0 per cent that never closes and never reverses. [verified] \u{2014} the same source, both geographies. Both fell by about the same proportion: 17.2 per cent here and 15.2 in Ohio."),
        ],
        answers: &["cannot say whether these outcomes differ by race in Allen County"],
        figures: &[
            Figure { label: "Allen County, 2000", value: 2360.7, literal: "2,360.7" },
            Figure { label: "Ohio, 2000", value: 2020.4, literal: "2,020.4" },
            Figure { label: "Allen County, 2020", value: 1954.2, literal: "1,954.2" },
            Figure { label: "Ohio, 2020", value: 1712.4, literal: "1,712.4" },
        ],
    },
    Assertion {
        id: "below-replacement-eleven-years-after-ohio",
        statement: "Allen County's fertility crossed below the replacement level in 2010, eleven \
                    years after Ohio was already there. It last stood above about 2,100 births per \
                    1,000 women in 2009.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-birth-outcomes-2000-2021.yml", "**It crossed below replacement in 2010 and Ohio was already there in 2000.** A rate of about 2,100 per 1,000 women is the level at which a population replaces itself without migration; this county last stood above it in 2009. [inference] \u{2014} computed here from the same series. That is eleven years later than the state and it is the demographic form of a fact [the children node](allen-county-children-2010-2024.yml) have carried as a falling birth count."),
        ],
        answers: &["cannot say whether these outcomes differ by race in Allen County"],
        figures: &[],
    },
    Assertion {
        id: "prematurity-fell-and-birthweight-did-not",
        statement: "Preterm births in Allen County fell by a quarter, from 11.5 per cent in 2000 to \
                    8.0 in 2019, while low birthweight did not move \u{2014} 8.5 per cent at the \
                    start and 7.7 at the end of the pre-2021 record. The county is above Ohio on low \
                    birthweight in 14 of 21 comparable years and on prematurity in 13.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-birth-outcomes-2000-2021.yml", "**Prematurity fell by a quarter and low birthweight did not move.** Preterm births run 11.5 per cent in 2000 and 8.0 in 2019, while low birthweight begins at 8.5 and ends the pre-2021 record at 7.7. [verified] \u{2014} the same source. Allen County is above Ohio on low birthweight in 14 of the 21 comparable years and above it on prematurity in 13."),
        ],
        answers: &["cannot say whether these outcomes differ by race in Allen County"],
        figures: &[
            Figure { label: "preterm, 2000", value: 11.5, literal: "11.5" },
            Figure { label: "preterm, 2019", value: 8.0, literal: "8.0" },
            Figure { label: "low birthweight, 2000", value: 8.5, literal: "8.5" },
        ],
    },
    Assertion {
        id: "infant-mortality-halved-then-stopped",
        statement: "Infant mortality in Allen County nearly halved and then stopped falling: 10.33 \
                    deaths per 1,000 live births in the five years to 2005, 6.37 in the five to \
                    2013, and 6.95 in the five to 2020, against Ohio's 7.81, 7.42 and 7.03. The \
                    county was well above the state for a decade and is at it now.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-birth-outcomes-2000-2021.yml", "**Infant mortality nearly halved and then stopped falling.** The five-year rate runs 10.33 deaths per 1,000 live births in the window ending 2005, 6.37 in the window ending 2013, and 6.95 in the window ending 2020, against Ohio's 7.81, 7.42 and 7.03. [verified] \u{2014} the same source. This county was well above the state for the first decade and is at it for the second."),
        ],
        answers: &["cannot say whether these outcomes differ by race in Allen County"],
        figures: &[
            Figure { label: "Allen County, to 2005", value: 10.33, literal: "10.33" },
            Figure { label: "to 2013", value: 6.37, literal: "6.37" },
            Figure { label: "to 2020", value: 6.95, literal: "6.95" },
        ],
    },
    Assertion {
        id: "twenty-twenty-one-is-the-worst-year",
        statement: "2021 is the worst year in Allen County's birth record on both measures \u{2014} \
                    10.2 per cent low birthweight and 10.8 per cent preterm, against 7.7 and 8.6 the \
                    year before \u{2014} while Ohio moved 6.9 to 7.1 and 8.6 to 8.9. The \
                    low-birthweight count went from 88 to 120, which is about 2.2 times the square \
                    root of the counts: a reading rather than a finding.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-birth-outcomes-2000-2021.yml", "**2021 is the worst year in the file on both birth measures and one year is not a trend.** Low birthweight reads 10.2 per cent and prematurity 10.8, against 7.7 and 8.6 the year before, while Ohio moved 6.9 to 7.1 and 8.6 to 8.9. [verified] \u{2014} the same source. The low-birthweight count went from 88 to 120: 32 babies on a base of 88, which is about 2.2 times the square root of the counts and is a reading rather than a finding. See [a count of tens is a draw](../../decisions/a-count-of-tens-is-a-draw.yml). The state not moving is what makes it worth recording at all."),
        ],
        answers: &["cannot say whether these outcomes differ by race in Allen County"],
        figures: &[
            Figure { label: "Allen County low birthweight, 2021", value: 10.2, literal: "10.2" },
            Figure { label: "Ohio", value: 7.1, literal: "7.1" },
            Figure { label: "Allen County preterm, 2021", value: 10.8, literal: "10.8" },
        ],
    },
    Assertion {
        id: "the-overlap-says-it-is-the-denominator",
        statement: "CDC publishes three of Allen County's birth measures twice, split at 2018, and \
                    the three shared years say what changed. Low birthweight, prematurity and their \
                    counts are identical to the digit across the break; the fertility rate differs \
                    in all three years \u{2014} 2,063.7 against 2,064.8 in 2018 and 1,954.2 against \
                    1,965.8 in 2020. The quantity with a population denominator moved and the \
                    quantities counted off birth certificates did not.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-birth-outcomes-2000-2021.yml", "**The publisher splits three of these measures at 2018 and the overlap says what changed.** Low birthweight, prematurity and their counts are identical to the digit in all three overlapping years across the two measures; the fertility rate differs in all three \u{2014} 2,063.7 against 2,064.8 in 2018 and 1,954.2 against 1,965.8 in 2020. [verified] \u{2014} [the same source](../../catalog/cdc-tracking-network.md), the two measures of each pair. The quantity with a population denominator moved and the quantities counted off birth certificates did not, and the same holds at state grain. See [an overlap names what changed](../../decisions/an-overlap-names-what-changed.yml)."),
        ],
        answers: &["cannot say whether these outcomes differ by race in Allen County"],
        figures: &[
            Figure { label: "fertility rate, old variant 2020", value: 1954.2, literal: "1,954.2" },
            Figure { label: "new variant 2020", value: 1965.8, literal: "1,965.8" },
        ],
    },
    Assertion {
        id: "fewer-patients-about-as-long",
        statement: "Allen County's hospitals are treating fewer people, not the same people for less \
                    time. Discharges fell 15.1 per cent between 2011 and 2023, from 26,924 to \
                    22,858, while the average stay rose from 4.37 days to 4.49.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-hospital-activity-2011-2023.yml", "**They are treating fewer people, not the same people for less time.** Discharges fell 15.1 per cent between 2011 and 2023, from 26,924 to 22,858, while the average stay went from 4.37 days to 4.49 \u{2014} up, not down. [verified] \u{2014} the same source, computed here. That answers the question [the bed node](allen-county-hospital-beds-2011-2023.yml) left open when it could see beds falling and staff rising and could not tell which of the two explanations held."),
        ],
        answers: &["cannot say what any of this was paid for"],
        figures: &[
            Figure { label: "discharges, 2011", value: 26924.0, literal: "26,924" },
            Figure { label: "2023", value: 22858.0, literal: "22,858" },
            Figure { label: "average stay 2011, days", value: 4.37, literal: "4.37" },
            Figure { label: "2023", value: 4.49, literal: "4.49" },
        ],
    },
    Assertion {
        id: "the-work-went-outpatient",
        statement: "Outpatient work passed half of what Allen County's hospitals charge for. \
                    Outpatient charges were 46.9 per cent of the total in 2011 and 55.3 in 2023, \
                    rising from $785,241,733 to $2,149,902,419 against inpatient's $888,010,922 to \
                    $1,741,256,842.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-hospital-activity-2011-2023.yml", "**The work went outpatient, and it can be counted.** Outpatient charges were 46.9 per cent of the county's hospital charges in 2011 and 55.3 in 2023, having risen from $785,241,733 to $2,149,902,419 while inpatient charges went from $888,010,922 to $1,741,256,842. [verified] \u{2014} the same source. Charges are list prices and not receipts, and neither figure is deflated; the share is the finding and the levels are context."),
        ],
        answers: &["cannot say what any of this was paid for"],
        figures: &[
            Figure { label: "outpatient share, 2011", value: 46.9, literal: "46.9" },
            Figure { label: "2023", value: 55.3, literal: "55.3" },
        ],
    },
    Assertion {
        id: "occupancy-did-not-move-because-the-beds-came-out",
        statement: "Occupancy in Allen County's hospitals was 55.6 per cent in 2011 and 56.0 in \
                    2023, on a bed count that fell from 580 to 502 and patient days that fell 12.9 \
                    per cent. A system that closes beds as fast as it loses admissions looks \
                    unchanged on the number a reader is most likely to ask for.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-hospital-activity-2011-2023.yml", "**Occupancy did not move, because the beds came out with the patients.** 55.6 per cent in 2011 and 56.0 in 2023, across a bed count that fell from 580 to 502 and patient days that fell 12.9 per cent. [verified] \u{2014} the same source. A hospital system that closes beds as fast as it loses admissions looks unchanged on the one number a reader is most likely to ask for."),
        ],
        answers: &["cannot say what any of this was paid for"],
        figures: &[
            Figure { label: "occupancy 2011", value: 55.6, literal: "55.6" },
            Figure { label: "2023", value: 56.0, literal: "56.0" },
            Figure { label: "beds 2011", value: 580.0, literal: "580" },
            Figure { label: "beds 2023", value: 502.0, literal: "502" },
        ],
    },
    Assertion {
        id: "bluffton-has-stopped-taking-inpatients",
        statement: "Bluffton Hospital has stopped taking inpatients in any ordinary sense: 389 \
                    discharges in 2011 and 120 in 2023, on twenty-five beds it has reported in all \
                    thirteen years. In 2021 it took 96 and ran at 2.7 per cent occupancy \u{2014} \
                    246 patient days out of 9,125 available.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-hospital-activity-2011-2023.yml", "**Bluffton Hospital has stopped taking inpatients in any ordinary sense.** Its discharges went from 389 in 2011 to 120 in 2023 \u{2014} 96 in 2021 \u{2014} and its occupancy from 15.4 per cent to 8.8, on twenty-five beds it has reported in all thirteen years. [verified] \u{2014} the same source. In 2021 it ran at 2.7 per cent: 246 patient days on 9,125 available."),
        ],
        answers: &["cannot say what any of this was paid for"],
        figures: &[
            Figure { label: "discharges 2011", value: 389.0, literal: "389" },
            Figure { label: "2023", value: 120.0, literal: "120" },
            Figure { label: "2021 occupancy", value: 2.7, literal: "2.7" },
        ],
    },
    Assertion {
        id: "the-long-term-hospital-emptied-too",
        statement: "Kindred Hospital Lima, the county's long-term acute care hospital and the only \
                    one of the five whose stays are measured in weeks, went from 96.6 per cent \
                    occupancy in 2011 to 60.1 in 2023 on the same twenty-six beds, with its average \
                    stay falling from 27.5 days to 23.7.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-hospital-activity-2011-2023.yml", "**The long-term hospital emptied too.** Kindred's occupancy ran 96.6 per cent in 2011 and 60.1 in 2023 on the same twenty-six beds, with its average stay falling from 27.5 days to 23.7. [verified] \u{2014} the same source. It is the only one of the five whose stays are measured in weeks."),
        ],
        answers: &["cannot say what any of this was paid for"],
        figures: &[
            Figure { label: "occupancy 2011", value: 96.6, literal: "96.6" },
            Figure { label: "2023", value: 60.1, literal: "60.1" },
            Figure { label: "average stay 2011, days", value: 27.5, literal: "27.5" },
        ],
    },
    Assertion {
        id: "the-two-big-hospitals-fell-in-proportion",
        statement: "St. Rita's and Lima Memorial account for Allen County's inpatient medicine and \
                    both lost about the same share of it: St. Rita's took 18,335 discharges in 2011 \
                    and 15,892 in 2023, Lima Memorial 7,192 and 6,023, so St. Rita's share of the \
                    two moved only from 71.8 per cent to 72.5.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-hospital-activity-2011-2023.yml", "**Two of the five account for the county's inpatient medicine and the split between them moved.** St. Rita's took 18,335 discharges in 2011 and 15,892 in 2023; Lima Memorial 7,192 and 6,023. [verified] \u{2014} the same source. St. Rita's share of the two is 71.8 per cent at the start and 72.5 at the end, so the fall was shared almost exactly in proportion. [inference] \u{2014} computed here."),
        ],
        answers: &["cannot say what any of this was paid for"],
        figures: &[
            Figure { label: "St. Rita's 2011", value: 18335.0, literal: "18,335" },
            Figure { label: "2023", value: 15892.0, literal: "15,892" },
            Figure { label: "Lima Memorial 2011", value: 7192.0, literal: "7,192" },
            Figure { label: "2023", value: 6023.0, literal: "6,023" },
        ],
    },
    Assertion {
        id: "seven-hundred-and-sixty-on-the-roads",
        statement: "Seven hundred and sixty people died in 666 crashes on Allen County's roads \
                    between 1975 and 2024. The rate fell by two fifths across four decades and \
                    then rose again.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-road-deaths-1975-2024.yml",
                "Seven hundred and sixty people died in 666 crashes on this county's roads in the fifty years from 1975 to 2024."
            ),
            support!(
                "measure/allen-county-road-deaths-1975-2024.yml",
                "The five decades run 17.3, 16.5, 14.3, 10.2 and 11.9 deaths per 100,000 people: the county's worst decade is its first and its best is 2005\u{2013}2014, and the decade just ended is worse than that one by fourteen deaths."
            ),
        ],
        answers: &["cannot say what happened in 2021"],
        figures: &[
            Figure { label: "1975\u{2013}1984", value: 17.3, literal: "17.3" },
            Figure { label: "1985\u{2013}1994", value: 16.5, literal: "16.5" },
            Figure { label: "1995\u{2013}2004", value: 14.3, literal: "14.3" },
            Figure { label: "2005\u{2013}2014", value: 10.2, literal: "10.2" },
            Figure { label: "2015\u{2013}2024", value: 11.9, literal: "11.9" },
        ],
    },
    Assertion {
        id: "the-roads-followed-ohio-not-the-country",
        statement: "Allen County's road deaths fell with Ohio's and not with the nation's. \
                    Indexed to 1975\u{2013}1984, the last decade stands at 62.9 here, 64.4 in Ohio \
                    and 82.7 in the United States.",
        topic: "health",
        supports: &[support!(
            "measure/allen-county-road-deaths-1975-2024.yml",
            "**It is Ohio's shape and not the country's.** Indexed to 1975\u{2013}1984, the last decade stands at 62.9 here, 64.4 in Ohio and 82.7 in the United States."
        )],
        answers: &["cannot say what happened in 2021"],
        figures: &[
            Figure { label: "Allen County", value: 62.9, literal: "62.9" },
            Figure { label: "Ohio", value: 64.4, literal: "64.4" },
            Figure { label: "United States", value: 82.7, literal: "82.7" },
        ],
    },
    Assertion {
        id: "the-worst-road-year-since-1978",
        statement: "Twenty-five people died on this county's roads in 2021, the most since 1978, \
                    in a decade averaging under eleven. Ohio rose ten per cent that year and the \
                    nation eleven; Allen County rose a hundred and fifty.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-road-deaths-1975-2024.yml",
                "**One year in the fifty stands outside the rest of the record.** In 2021 there were 23 fatal crashes and 25 deaths, the most since 1978, in a county whose other nine years of that decade average 10.8."
            ),
            support!(
                "measure/allen-county-road-deaths-1975-2024.yml",
                "Ohio rose 10.1 per cent that year and the United States 10.8; Allen County rose 150. None of the 23 crashes killed more than two people, so it is not one catastrophe but twenty-three separate ones, seven of them in November."
            ),
        ],
        answers: &["cannot say what happened in 2021"],
        figures: &[
            Figure { label: "Allen County", value: 150.0, literal: "150" },
            Figure { label: "Ohio", value: 10.1, literal: "10.1" },
            Figure { label: "United States", value: 10.8, literal: "10.8" },
        ],
    },
    Assertion {
        id: "the-drink-share-never-moved",
        statement: "A third of this county's fatal crashes involved a drinking driver in the \
                    1970s and a third of them still did in 2020, while the number of fatal crashes \
                    fell by a third.",
        topic: "health",
        supports: &[support!(
            "measure/allen-county-road-deaths-1975-2024.yml",
            "**A third of the fatal crashes involve a drinking driver and that share has not moved in forty-six years.** By decade it runs 33.3, 36.7, 29.6, 35.4 and 38.6 per cent \u{2014} 221 drinking drivers across 611 crashes."
        )],
        answers: &["cannot say what happened in 2021"],
        figures: &[
            Figure { label: "1975\u{2013}1984", value: 33.3, literal: "33.3" },
            Figure { label: "1985\u{2013}1994", value: 36.7, literal: "36.7" },
            Figure { label: "1995\u{2013}2004", value: 29.6, literal: "29.6" },
            Figure { label: "2005\u{2013}2014", value: 35.4, literal: "35.4" },
            Figure { label: "2015\u{2013}2020", value: 38.6, literal: "38.6" },
        ],
    },
    Assertion {
        id: "the-crossings-stopped-killing-people",
        statement: "Twenty-eight people died at Allen County railway crossings between 1980 and \
                    2005 \u{2014} one road death in fifteen \u{2014} and none has died at one since. \
                    Every crossing here known to have killed someone is now shut or gated.",
        topic: "health",
        supports: &[
            support!(
                "measure/allen-county-rail-crossing-deaths-1980-2005.yml",
                "Twenty-eight people died in 25 crashes where an Allen County road meets a railway, between 1980 and 2005, and none has died at one since."
            ),
            support!(
                "measure/allen-county-rail-crossing-deaths-1980-2005.yml",
                "**Three of the nineteen have since been closed and all sixteen that remain open have gate arms**, against 94 of the county's 163 open public crossings, or 57.7 per cent."
            ),
            support!(
                "measure/allen-county-rail-crossing-deaths-1980-2005.yml",
                "Those 28 deaths are 6.6 per cent of the 423 who died on this county's roads between 1980 and 2005."
            ),
        ],
        answers: &["cannot say whether the gates came before these deaths or after them"],
        figures: &[
            Figure { label: "died at a crossing, 1980\u{2013}2005", value: 28.0, literal: "28" },
            Figure { label: "died on the county's roads, same years", value: 423.0, literal: "423" },
        ],
    },
    Assertion {
        id: "a-fifth-of-the-workplaces-are-gone",
        statement: "Allen County has 2,239 private workplaces where it had 2,763 in 1986 \u{2014} a \
                    fifth fewer \u{2014} while the number of jobs in them fell by a twenty-fifth. The \
                    average workplace went from 16.6 employees to 19.8.",
        topic: "work",
        supports: &[support!(
            "measure/allen-county-private-employers-1986-2023.yml",
            "**The county lost a fifth of its workplaces and a twenty-fifth of its jobs.** Establishments fell from 2,763 to 2,239, or 19.0 per cent, while employment fell from 45,917 to 44,251, or 3.6. Average establishment size rose from 16.6 employees to 19.8."
        )],
        answers: &["cannot say whether that flatness is wages standing still or hours doing so"],
        figures: &[
            Figure { label: "workplaces, 1986", value: 2763.0, literal: "2,763" },
            Figure { label: "workplaces, 2023", value: 2239.0, literal: "2,239" },
        ],
    },
    Assertion {
        id: "the-small-workplaces-are-what-went",
        statement: "The county's whole net loss of workplaces is at the bottom of the size scale: \
                    621 fewer with under ten employees, 97 more with ten or more, and exactly as \
                    many with a hundred or more as in 1986.",
        topic: "work",
        supports: &[support!(
            "measure/allen-county-private-employers-1986-2023.yml",
            "**Almost the whole loss is in the smallest workplaces.** The two smallest classes lost 621 establishments between them and every class of ten or more gained 97."
        )],
        answers: &["cannot say whether that flatness is wages standing still or hours doing so"],
        figures: &[
            Figure { label: "workplaces under 10 employees, lost", value: 621.0, literal: "621" },
            Figure { label: "workplaces of 10 or more, gained", value: 97.0, literal: "97" },
        ],
    },
    Assertion {
        id: "pay-that-has-not-moved-in-thirty-seven-years",
        statement: "Payroll per private employee in this county is worth about what it was worth in \
                    1986. Whether it is slightly less or slightly more depends on which price index \
                    does the deflating.",
        topic: "work",
        supports: &[support!(
            "measure/allen-county-private-employers-1986-2023.yml",
            "$19,658 in 1986 and $52,858 in 2023 is a rise of 169 per cent in the money of each year. Deflated by the national consumer price index the 1986 figure is $54,651 in 2023 dollars, so real pay per employee is **3.3 per cent lower** than it was; deflated by the Midwest index it is $51,467, so real pay is **2.7 per cent higher**."
        )],
        answers: &["cannot say whether that flatness is wages standing still or hours doing so"],
        figures: &[
            Figure { label: "1986 pay, national index", value: 54651.0, literal: "54,651" },
            Figure { label: "1986 pay, Midwest index", value: 51467.0, literal: "51,467" },
            Figure { label: "2023 pay", value: 52858.0, literal: "52,858" },
        ],
    },
    Assertion {
        id: "retail-lost-more-than-manufacturing",
        statement: "Retail trade has shed more of this county's jobs since 1998 than manufacturing \
                    has \u{2014} 2,364 against 753 \u{2014} along with 167 of its stores.",
        topic: "work",
        supports: &[support!(
            "measure/allen-county-private-employers-1986-2023.yml",
            "**Retail lost more jobs than manufacturing did.** Between 1998 and 2023 retail trade fell from 7,806 employees to 5,442 and from 535 establishments to 368; manufacturing fell from 9,886 to 9,133."
        )],
        answers: &["cannot say whether that flatness is wages standing still or hours doing so"],
        figures: &[
            Figure { label: "retail, 1998", value: 7806.0, literal: "7,806" },
            Figure { label: "retail, 2023", value: 5442.0, literal: "5,442" },
            Figure { label: "manufacturing, 1998", value: 9886.0, literal: "9,886" },
            Figure { label: "manufacturing, 2023", value: 9133.0, literal: "9,133" },
        ],
    },
    Assertion {
        id: "the-two-largest-sectors-are-level-again",
        statement: "Health care overtook manufacturing as this county's largest private sector in \
                    2002 and led for twenty-one years. In 2023 the two are seventeen employees \
                    apart, health care having fallen a quarter from its 2013 peak.",
        topic: "work",
        supports: &[support!(
            "measure/allen-county-private-employers-1986-2023.yml",
            "**And the two largest sectors are now the same size.** Manufacturing 9,133 and health care 9,150 in 2023, seventeen apart, after twenty-one years in which health care led \u{2014} it passed manufacturing in 2002 and peaked at 12,431 in 2013."
        )],
        answers: &["cannot say whether that flatness is wages standing still or hours doing so"],
        figures: &[
            Figure { label: "health care, 2013 peak", value: 12431.0, literal: "12,431" },
            Figure { label: "health care, 2023", value: 9150.0, literal: "9,150" },
            Figure { label: "manufacturing, 2023", value: 9133.0, literal: "9,133" },
        ],
    },
    Assertion {
        id: "the-one-election-it-gave-a-democrat",
        statement: "Allen County voted for Ted Strickland, a Democrat, for governor in 2006 — \
                    18,000 to 17,184. It is the only election in this corpus's modern record, five \
                    for governor and six for president, that the county has given to a Democrat.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-governor-vote-2002-2018.yml", "**In 2006 Allen County voted for a Democrat, by 816 votes.** It is the only election in this\n  corpus's modern record \u{2014} five for governor and six for president, from 2000 forward \u{2014} that this\n  county has given to a Democratic candidate. [verified] \u{2014} the same files and\n  [the presidential series](allen-county-presidential-vote-2000-2016.yml)."),
            support!("measure/allen-county-governor-vote-2002-2018.yml", "**In figures: the Republican share of the two-party vote was 69.1 per cent in 2002, 48.8 in 2006,\n  59.8 in 2010, 73.4 in 2014 and 69.1 in 2018.** The balance moves 24.6 points in the eight years\n  between 2006 and 2014, and the two ends of the run sit a twentieth of a point apart. [inference] \u{2014}\n  arithmetic this corpus's, on the table above."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2002", value: 69.1, literal: "69.1" },
            Figure { label: "2006", value: 48.8, literal: "48.8" },
            Figure { label: "2010", value: 59.8, literal: "59.8" },
            Figure { label: "2014", value: 73.4, literal: "73.4" },
            Figure { label: "2018", value: 69.1, literal: "69.1" },
        ],
    },
    Assertion {
        id: "one-democrat-carried-it-and-seven-did-not",
        statement: "On the same 2006 ballot the county gave Strickland 51.2 per cent of the \
                    two-party vote and every other Democrat less — down to 40.2 per cent for \
                    Congress. Sherrod Brown lost this county by 2,924 votes on a night he won Ohio.",
        topic: "elections",
        supports: &[support!("measure/allen-county-ballot-2006.yml", "**One Democrat carried this county and seven did not, on the same piece of paper.** Ted\n  Strickland took 51.2 per cent of the two-party vote for governor; the next-best Democrat on the\n  ballot, Sherrod Brown, took 45.9 and lost the county by 2,924 while winning Ohio; the worst,\n  Richard Siferd for Congress, took 40.2. The spread between the best and worst Democratic showings\n  is eleven points. [verified] \u{2014} the workbook; shares computed here. Split-ticket voting on that\n  scale is a fact about the ballot and not about any candidate, and this return cannot say which of\n  the eight races was the unusual one. [inference]")],
        answers: &["cannot say which of the eight races was the unusual one"],
        figures: &[
            Figure { label: "Governor", value: 51.2, literal: "51.2" },
            Figure { label: "U.S. Senate", value: 45.9, literal: "45.9" },
            Figure { label: "U.S. House", value: 40.2, literal: "40.2" },
        ],
    },
    Assertion {
        id: "the-presidential-premium-is-twenty-three-points",
        statement: "Allen County turned out 71.8 per cent of its registered voters in 2020 against \
                    54.7 per cent in 2006 and 48.4 in 2010. The two low years are midterms, so the \
                    twenty-three points between them are the presidential premium and not a trend.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-turnout-2020.yml", "**71.8 per cent is the third of six presidential elections here, and an ordinary one.**"),
            support!("measure/allen-county-turnout-2020.yml", "**The midterms are the county's other shape.** The Secretary of State's own precinct files put Allen County at **54.7 per cent in 2006** \u{2014} 37,605 ballots against 68,797 registered \u{2014} and at **48.4 per cent in 2010**, 33,867 against 69,931."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2006 midterm", value: 54.7, literal: "54.7" },
            Figure { label: "2010 midterm", value: 48.4, literal: "48.4" },
            Figure { label: "2020 presidential", value: 71.8, literal: "71.8" },
        ],
    },
    Assertion {
        id: "two-thousand-and-twenty-was-an-ordinary-year",
        statement: "Allen County turned out 71.8 per cent of its registered voters in 2020 \u{2014} the third of six presidential elections here, behind 2004 and 2008 and ahead of 2024, 2012 and 2016. Twenty years of presidential turnout fit inside 4.7 points.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-turnout-2004-2024.yml", "**2020 was an ordinary presidential election here.** Its 71.8 per cent is the third of six, below 2004's 73.3 and 2008's 72.1 and above 2024's 70.8, 2012's 70.1 and 2016's 68.7. Twenty years of presidential turnout in this county fit inside 4.7 points."),
        ],
        answers: &["cannot say why nearly a quarter of this county's provisional ballots were rejected in 2024"],
        figures: &[
            Figure { label: "2004", value: 73.3, literal: "73.3" },
            Figure { label: "2008", value: 72.1, literal: "72.1" },
            Figure { label: "2020", value: 71.8, literal: "71.8" },
            Figure { label: "2024", value: 70.8, literal: "70.8" },
            Figure { label: "2012", value: 70.1, literal: "70.1" },
            Figure { label: "2016", value: 68.7, literal: "68.7" },
        ],
    },
    Assertion {
        id: "the-county-used-to-out-vote-ohio",
        statement: "This county turned out two to three points above the Ohio average in 2004, 2006 and 2008 and one to three points below it in every election since 2016. The swing is about five points relative to the state.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-turnout-2004-2024.yml", "**This county used to vote more than Ohio does and now it votes less.** It stood 2.9, 2.0 and 2.2 points above the state in 2004, 2006 and 2008, and 2.7, 3.1, 2.2, 2.8 and 1.7 points below it in 2016, 2018, 2020, 2022 and 2024."),
        ],
        answers: &["cannot say why nearly a quarter of this county's provisional ballots were rejected in 2024"],
        figures: &[],
    },
    Assertion {
        id: "election-day-stopped-being-the-day",
        statement: "Election day carried 85.6 per cent of Allen County's ballots in 2006, 68.9 in 2012 and 47.2 in 2020. The pandemic's mail voting has since gone away and its early in-person voting has not: 13,090 early ballots in 2024, the most in the record.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-turnout-2004-2024.yml", "The election-day share was 85.6 per cent in 2006 and 76.5 in 2008; it fell to 68.9 in 2012 and has not been above 73 since."),
            support!("measure/allen-county-turnout-2004-2024.yml", "In 2020 it was 47.2 per cent \u{2014} fewer than half the county's ballots were cast on the day \u{2014} and in 2024 it was 58.6."),
            support!("measure/allen-county-turnout-2004-2024.yml", "Between 2020 and 2024 mail ballots fell by 7,079 and in-person early ballots rose by 1,342 to 13,090, the most in the record; election-day ballots rose by 4,611."),
        ],
        answers: &["cannot say why nearly a quarter of this county's provisional ballots were rejected in 2024"],
        figures: &[
            Figure { label: "2006", value: 85.6, literal: "85.6" },
            Figure { label: "2008", value: 76.5, literal: "76.5" },
            Figure { label: "2012", value: 68.9, literal: "68.9" },
            Figure { label: "2020", value: 47.2, literal: "47.2" },
            Figure { label: "2024", value: 58.6, literal: "58.6" },
        ],
    },
    Assertion {
        id: "sixty-one-thousand-names-off-a-sixty-six-thousand-roll",
        statement: "The county board took 61,662 registrations off its voter roll across nine federal elections, against a roll that ends the run holding 66,650 names. The commonest reason is not death or moving away \u{2014} it is that a confirmation letter went unanswered.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-voter-roll-removals-2006-2022.yml", "61,662 registrations were taken off this county's voter roll across the nine federal elections from 2006 to 2022. The roll at the end of that run held 66,650 names."),
            support!("measure/allen-county-voter-roll-removals-2006-2022.yml", "**The largest single reason a name comes off this roll is that a letter went unanswered.** Over the eight elections from 2008 the board removed 55,243 registrations and 24,086 of them \u{2014} 43.6 per cent \u{2014} were removed for failing to respond to a confirmation notice. [verified] \u{2014} the same file; 2006 is left out of the table because that year's total of 6,419 carries no breakdown. Dying and moving away, the two reasons a reader would expect to dominate, are 15.1 and 30.1 per cent."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "no response to a notice", value: 43.6, literal: "43.6" },
            Figure { label: "moved away", value: 30.1, literal: "30.1" },
            Figure { label: "died", value: 15.1, literal: "15.1" },
        ],
    },
    Assertion {
        id: "two-hundred-and-fifty-two-people-run-it",
        statement: "Allen County's polling places went from 50 in 2008 to 33 now, and the people staffing them from 674 in 2016 to 252 in 2024. Of those 252, 182 were 61 or over and 15 were 40 or under.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-polling-places-2004-2024.yml", "**The polling places went with them and then kept going.** 50 in 2008 and 33 in 2022 and 2024, a fall of a third, against a precinct count that has not moved since 2012."),
            support!("measure/allen-county-polling-places-2004-2024.yml", "In 2024, 182 of the 252 were 61 or over \u{2014} 72.2 per cent \u{2014} 55 were 41 to 60, and 15 were 40 or under."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "40 or under", value: 15.0, literal: "15" },
            Figure { label: "41 to 60", value: 55.0, literal: "55" },
            Figure { label: "61 and over", value: 182.0, literal: "182" },
        ],
    },
    Assertion {
        id: "a-quarter-of-the-provisional-ballots-were-rejected",
        statement: "211 of Allen County's 898 provisional ballots were rejected in 2024 \u{2014} 23.5 per cent, the highest rate in the twenty-year record, against 18.6 in 2020 and 4.9 in 2014.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-turnout-2004-2024.yml", "Provisional ballots are the exception: 211 of 898 were rejected in 2024, 23.5 per cent, the highest rate in the record, against 18.6 in 2020 and 4.9 in 2014."),
            support!("measure/allen-county-turnout-2004-2024.yml", "This corpus cannot say why nearly a quarter of this county's provisional ballots were rejected in 2024."),
        ],
        answers: &["cannot say why nearly a quarter of this county's provisional ballots were rejected in 2024"],
        figures: &[
            Figure { label: "2014", value: 4.9, literal: "4.9" },
            Figure { label: "2020", value: 18.6, literal: "18.6" },
            Figure { label: "2024", value: 23.5, literal: "23.5" },
        ],
    },
    Assertion {
        id: "a-third-of-the-schoolchildren-are-gone",
        statement: "Allen County's twelve school districts held 22,760 pupils in 1988 and hold \
                    15,850 — 30.4 per cent fewer, against a county population down 8.1 per cent \
                    over the same span. Lima City Schools have lost half their children.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-enrolment-1988-2024.yml", "**Allen County has 6,910 fewer schoolchildren than it had in 1988 \u{2014} 30.4 per cent of them \u{2014} and\n  the high of the whole run is its first year.** [verified] \u{2014} the same files. The county's\n  population went from 109,755 in 1990 to 100,866 in 2024, a fall of 8.1 per cent, so the schools\n  emptied close to four times as fast as the county did. [inference] \u{2014} against\n  [1990](allen-county-population-1940-1990.yml) and\n  [2024](allen-county-population-2024.yml)."),
            support!("measure/allen-county-school-enrolment-1988-2024.yml", "**Lima City Schools have lost half their children, and the city has not lost half its people.**\n  6,969 pupils in 1988 and 3,370 now, a fall of 51.6 per cent, against a city that went from 45,549\n  residents in 1990 to 34,690 in 2024 \u{2014} 23.8 per cent. The district emptied at more than twice the\n  rate of the city it serves. [inference] \u{2014} the same files against\n  [1990](lima-population-1970-1990.yml) and [2024](lima-population-2024.yml). Elida is the\n  next-steepest at \u{2212}38.8 per cent, and it borders the city on the north and west."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "county population", value: 8.1, literal: "8.1" },
            Figure { label: "city of Lima", value: 23.8, literal: "23.8" },
            Figure { label: "all schoolchildren", value: 30.4, literal: "30.4" },
            Figure { label: "Lima's district", value: 51.6, literal: "51.6" },
        ],
    },
    Assertion {
        id: "lima-holds-fewer-of-the-black-pupils",
        statement: "Lima City Schools held 82.3 per cent of Allen County's Black pupils in 1988 \
                    and hold 66.4 per cent now. Elida went from 5.5 per cent of them to 14.4, and \
                    Perry Local is the county's second most heavily Black district at 21.7.",
        topic: "schools",
        supports: &[support!("measure/allen-county-school-enrolment-by-race-1988-2024.yml", "**Black children are less concentrated in Lima than they were, and by a long way.** The city's\n  district held 82.3 per cent of the county's Black pupils in 1988, 79.2 in 2000, 69.6 in 2012 and\n  66.4 now. [verified] \u{2014} the same files, computed here. Over the same thirty-seven years Lima's\n  share of *all* the county's pupils fell from 30.6 per cent to 21.3, so the two shares moved the\n  same way and the Black share moved further. [inference]")],
        answers: &["cannot say how much of that is the new category"],
        figures: &[
            Figure { label: "1988", value: 82.3, literal: "82.3" },
            Figure { label: "2000", value: 79.2, literal: "79.2" },
            Figure { label: "2012", value: 69.6, literal: "69.6" },
            Figure { label: "2024", value: 66.4, literal: "66.4" },
        ],
    },
    Assertion {
        id: "the-share-that-held-because-both-halves-emptied",
        statement: "Lima City Schools were 38.6 per cent Black in 1988 and are 38.8 per cent now, \
                    across a district that lost half its children. The share held because both \
                    halves emptied — Black enrolment 2,689 to 1,307, white-alone 4,155 to 1,067.",
        topic: "schools",
        supports: &[support!("measure/allen-county-school-enrolment-by-race-1988-2024.yml", "**The Black share of the district is the figure that did not move.** 38.6 per cent in 1988 and\n  38.8 now, across a district that lost half its children. [verified] \u{2014} same files. It is not a\n  share that held still through stability: Black enrolment fell from 2,689 to 1,307 and white-alone\n  enrolment from 4,155 to 1,067, and the ratio survived because both halves emptied. [inference]")],
        answers: &["cannot say how much of that is the new category"],
        figures: &[
            Figure { label: "white-alone 1988", value: 4155.0, literal: "4,155" },
            Figure { label: "white-alone 2024", value: 1067.0, literal: "1,067" },
            Figure { label: "Black 1988", value: 2689.0, literal: "2,689" },
            Figure { label: "Black 2024", value: 1307.0, literal: "1,307" },
        ],
    },
    Assertion {
        id: "the-class-got-smaller-as-the-county-emptied",
        statement: "Allen County's twelve school districts lost 28.40 per cent of their children \
                    between 1992 and 2024 and 19.11 per cent of their teachers, so the class got \
                    smaller: 17.85 pupils to a teacher then and 15.80 now.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-staffing-1992-2024.yml", "**The children fell by 28.40 per cent and the teachers by 19.11.** 22,137 pupils and 1,240 teachers in 1992 against 15,850 and 1,003 now, so the class got smaller: 17.85 pupils to a teacher then and 15.80 now. [verified] \u{2014} the same file, computed here, against [the enrolment series](allen-county-school-enrolment-1988-2024.yml)."),
        ],
        answers: &["cannot say who teaches in Allen County"],
        figures: &[
            Figure { label: "pupils lost", value: 28.4, literal: "28.40" },
            Figure { label: "teachers lost", value: 19.11, literal: "19.11" },
            Figure { label: "pupils per teacher 1992", value: 17.85, literal: "17.85" },
            Figure { label: "pupils per teacher 2024", value: 15.8, literal: "15.80" },
        ],
    },
    Assertion {
        id: "every-adult-but-the-librarian",
        statement: "Every kind of adult these schools employ is more common per child than it was in \
                    1992 except one. Teachers and aides go from 58.36 full-time equivalents per \
                    thousand pupils to 73.06 and administration from 12.51 to 18.30, while library \
                    staff go from 1.63 to 0.95.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-staffing-1992-2024.yml", "**Every other kind of adult in these buildings is more common per child than it was, and one is very much less.** Teachers and aides go from 58.36 full-time equivalents per thousand pupils to 73.06, administration from 12.51 to 18.30 and guidance counsellors from 1.99 to 2.71, while library staff go from 1.63 to 0.95. [verified] \u{2014} the same source, its district directory, computed here."),
        ],
        answers: &["cannot say who teaches in Allen County"],
        figures: &[
            Figure { label: "library staff 2024", value: 0.95, literal: "0.95" },
            Figure { label: "library staff 1992", value: 1.63, literal: "1.63" },
            Figure { label: "administration 1992", value: 12.51, literal: "12.51" },
            Figure { label: "administration 2024", value: 18.3, literal: "18.30" },
            Figure { label: "instruction 1992", value: 58.36, literal: "58.36" },
            Figure { label: "instruction 2024", value: 73.06, literal: "73.06" },
        ],
    },
    Assertion {
        id: "eleven-districts-with-no-librarian",
        statement: "Eleven of Allen County's twelve school districts employ no certified school \
                    librarian. There were 24 of them here in 1992 and 4 now, all four in Lima, and \
                    the library support column beside them did not take them in \u{2014} it reads 12 \
                    in 1992 and 11 in 2024.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-staffing-1992-2024.yml", "**Eleven of the twelve districts employ no certified school librarian.** There were 24 of them here in 1992, at least one in every district and five in Lima. The whole county reported 2 in 2020, one in Lima and one in Perry, and the 4 it reports in 2024 are all in Lima \u{2014} a fall of 83.33 per cent in thirty-two years. [verified] \u{2014} the same source, its district directory, by district."),
            support!("measure/allen-county-school-staffing-1992-2024.yml", "**That fall is a fall and not a reclassification, and the check is on the same form.** The column next to it holds library support staff, which is where a certified librarian's work would land if a district replaced the post with an assistant: it reads 12 in 1992 and 11 in 2024, and the two columns together fall from 36 to 15. In 1992 this county had two certified librarians for every library assistant and it now has one for every three. [verified] \u{2014} same source, both columns."),
        ],
        answers: &["cannot say who teaches in Allen County"],
        figures: &[
            Figure { label: "certified librarians 2024", value: 4.0, literal: "4" },
            Figure { label: "library support 2024", value: 11.0, literal: "11" },
            Figure { label: "library support 1992", value: 12.0, literal: "12" },
            Figure { label: "certified librarians 1992", value: 24.0, literal: "24" },
        ],
    },
    Assertion {
        id: "the-administrators-that-did-not-leave",
        statement: "Allen County's districts appear to have shed two administrators in three in \
                    2006, when the district-officer column fell from 76 full-time equivalents to 25. \
                    All four administrative columns together fell 10.87 per cent, and administration \
                    now stands higher per pupil than in any year before 2001.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-staffing-1992-2024.yml", "**The administrators are the same test run the other way, and it comes out the other way.** District officers fall from 76 full-time equivalents in 2005 to 25 in 2006, a drop of 67.11 per cent in one year, and eleven of the twelve districts land on two or fewer and stay there. Building principals rise from 42 to 55 in the same year. Read as four columns the county sacked two administrators in three; read as the group the form defines, administration went from 15.91 per thousand pupils to 14.18, a fall of 10.87 per cent, and it is higher now than in any year before 2001. [verified] \u{2014} the same source, its district directory, computed here. See [a column can empty into its neighbour](../../decisions/a-column-can-empty-into-its-neighbour.yml)."),
        ],
        answers: &["cannot say who teaches in Allen County"],
        figures: &[
            Figure { label: "district officers 2006", value: 25.0, literal: "25" },
            Figure { label: "all four columns", value: 10.87, literal: "10.87" },
            Figure { label: "district officers 2005", value: 76.0, literal: "76" },
        ],
    },
    Assertion {
        id: "one-aide-for-every-hundred-children",
        statement: "Allen County's schools employed 52 instructional aides in 1992 and employ 155 \
                    \u{2014} one for every 425.7 children then and one for every 102.3 now, against \
                    23.85 teachers to an aide then and 6.47 now.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-staffing-1992-2024.yml", "**Instructional aides went from 52 to 155.** One aide for every 425.7 children in 1992 and one for every 102.3 now; there were 23.85 teachers to an aide and there are 6.47. [verified] \u{2014} the same file. The corpus does not read that as a change in what teaching is, because the same period carries a federal special-education mandate and a change in what a district must count as an aide, and this file separates neither. [inference]"),
        ],
        answers: &["cannot say who teaches in Allen County"],
        figures: &[
            Figure { label: "aides 1992", value: 52.0, literal: "52" },
            Figure { label: "teachers per aide 2024", value: 6.47, literal: "6.47" },
            Figure { label: "teachers per aide 1992", value: 23.85, literal: "23.85" },
            Figure { label: "aides 2024", value: 155.0, literal: "155" },
        ],
    },
    Assertion {
        id: "the-psychologists-belong-to-the-service-center",
        statement: "Nine of Allen County's twelve school districts report no school psychologist and \
                    the twelve report 7.0 full-time equivalents between them. The county's \
                    educational service center reports 14.45 against an enrolment of zero \u{2014} \
                    twice the districts put together.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-staffing-1992-2024.yml", "**The county's school psychologists work for the service center, not the districts.** The column opens in 2019 and the twelve districts report 7.0 full-time equivalents between them in 2023, nine of them reporting none. In the same file and the same year the [Allen County Educational Service Center](../jurisdiction/allen-county-educational-service-center.yml) reports 14.45 \u{2014} twice the twelve districts put together, against an enrolment of zero. [verified] \u{2014} the same source, its district directory, the service center's own row."),
        ],
        answers: &["cannot say who teaches in Allen County"],
        figures: &[
            Figure { label: "the twelve districts", value: 7.0, literal: "7.0" },
            Figure { label: "the service center", value: 14.45, literal: "14.45" },
        ],
    },
    Assertion {
        id: "eight-buildings-and-seven-were-limas",
        statement: "Allen County's twelve school districts held 51 school buildings in 1986 and hold \
                    43. Lima City accounts for seven of the eight-building difference, and five \
                    districts have reported the same number every year for 39 years.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**This county reports eight fewer school buildings than it did in 1986, and seven of the difference is Lima City.** The county held 51 in 1986 and holds 43; Lima went from 16 to 9, Elida and Spencerville lost one each, Bath gained one, and the remaining eight districts end where they began \u{2014} so nine buildings left the count and one entered it. [verified] \u{2014} the same source, its district directory, by district."),
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**Five of the twelve have reported the same number every year since 1986.** Columbus Grove and Pandora-Gilboa 3, Shawnee 4, Perry and Waynesfield-Goshen 2 \u{2014} one distinct value each across all 39 years. Six more move by exactly one building and back, and Lima's count runs from 16 down to 9. [verified] \u{2014} same source, computed here."),
        ],
        answers: &["cannot say where any of these buildings stood"],
        figures: &[
            Figure { label: "buildings 2024", value: 43.0, literal: "43" },
            Figure { label: "Lima 2024", value: 9.0, literal: "9" },
            Figure { label: "Lima 1986", value: 16.0, literal: "16" },
            Figure { label: "buildings 1986", value: 51.0, literal: "51" },
        ],
    },
    Assertion {
        id: "two-answers-that-arrive-at-the-same-building",
        statement: "Lima lost 3,599 pupils and closed seven schools; Allen County's other eleven \
                    districts lost 3,311 between them and closed one. They arrive at the same \
                    building \u{2014} 374.4 pupils per school in Lima and 367.1 in the other eleven.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**The two halves of the county lost nearly the same number of children and answered differently.** Lima went from 6,969 pupils in 1988 to 3,370, a loss of 3,599, and closed seven buildings. The other eleven districts lost 3,311 between them and closed one. [verified] \u{2014} same file, against [the enrolment series](allen-county-school-enrolment-1988-2024.yml)."),
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**They arrive at the same building.** Lima held 435.6 pupils per school in 1988 and holds 374.4; the other eleven held 464.4 and hold 367.1. Two opposite policies over thirty-six years converge within eight pupils of each other. [verified] \u{2014} the same two files, computed here. The corpus does not read that as either district aiming at a number, because nothing in these files is a decision and a school's capacity is fixed by the building rather than chosen each year. [inference]"),
        ],
        answers: &["cannot say where any of these buildings stood"],
        figures: &[
            Figure { label: "Lima per school", value: 374.4, literal: "374.4" },
            Figure { label: "the other eleven per school", value: 367.1, literal: "367.1" },
            Figure { label: "Lima per school 1988", value: 435.6, literal: "435.6" },
            Figure { label: "the other eleven 1988", value: 464.4, literal: "464.4" },
        ],
    },
    Assertion {
        id: "the-children-went-missing-faster-than-the-classrooms",
        statement: "Allen County's under-18 population fell 7.3 per cent between the July estimates \
                    of 2010 and 2019, while enrolment in its twelve school districts fell 5.9 per \
                    cent over the same two autumns.",
        topic: "population",
        supports: &[support!("measure/allen-county-children-2010-2024.yml", "**This is most of why the schools emptied, and it is not all of it.** Across the 2010s series the\n  county's under-18 population fell 7.3 per cent between the July estimates of 2010 and 2019 \u{2014}\n  25,377 to 23,527 \u{2014} while enrolment in the twelve districts fell 5.9 per cent over the same two\n  autumns, 17,830 to 16,774. [inference] \u{2014} against\n  [the enrolment series](allen-county-school-enrolment-1988-2024.yml). The children went missing\n  faster than the classrooms did, which is the reverse of what a story about children leaving these\n  districts for other schools would predict, and it is one decade of evidence rather than a rule.")],
        answers: &[],
        figures: &[
            Figure { label: "under-18 population", value: 7.3, literal: "7.3" },
            Figure { label: "district enrolment", value: 5.9, literal: "5.9" },
        ],
    },
    Assertion {
        id: "the-under-fives-are-the-births",
        statement: "Births in Allen County fell from 1,318 in 2011 to 1,200 in 2024, and the \
                    county's under-five population is those births and nothing else — it matches \
                    the five preceding years of them to within 1.6 per cent in every year checked.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-children-2010-2024.yml", "**Births fell nine per cent across the run and the low is 2021.** 1,318 in 2011 against 1,200 in\n  2024, with the fewest in the record \u{2014} 1,171 \u{2014} in the first full year after the pandemic began.\n  [verified] \u{2014} [the components files](../../catalog/census-popest-2024.md). **2010 and 2020 are\n  missing from that table on purpose**: each vintage's first year covers only the three months from\n  the census date to 30 June, 339 births in 2010 and 293 in 2020, and putting a partial year in a\n  column of full ones is how a false trough gets published. [verified] \u{2014} the same files."),
            support!("measure/allen-county-children-2010-2024.yml", "**The county's under-five population is its own births and nothing else.** For every year of the\n  2010s where all five preceding birth counts are published in full, the estimated under-five\n  population matches their sum to within 1.6 per cent: 6,372 against 6,403 in 2015, 6,357 against\n  6,347 in 2016, 6,367 against 6,353 in 2017, 6,362 against 6,314 in 2018 and 6,303 against 6,206 in\n  2019. [inference] \u{2014} computed here from the age and component files. Migration moves this county's\n  adults; it does not appear to move its babies. The same check cannot be run on the 2020s series\n  because the 2020 birth count in these files is a three-month stub."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "2011", value: 1318.0, literal: "1,318" },
            Figure { label: "2021", value: 1171.0, literal: "1,171" },
            Figure { label: "2024", value: 1200.0, literal: "1,200" },
        ],
    },
    Assertion {
        id: "two-vintages-disagree-about-the-children",
        statement: "Two vintages of the Census Bureau's county estimates reach 1 July 2020 and \
                    disagree. They put Allen County 157 people apart and its children 811 apart — \
                    a seventh of a per cent on the total and 3.5 per cent on the part.",
        topic: "population",
        supports: &[support!("measure/allen-county-children-2010-2024.yml", "**1 July 2020 appears twice because two series reach it and they disagree.** The rows above the\n  break are the Vintage 2020 estimates, carried forward from the 2010 census; the rows below are\n  Vintage 2024, carried forward from the 2020 one. At the shared date they put the county 157 people\n  apart \u{2014} a seventh of one per cent \u{2014} and its children **811 apart**, which is 3.5 per cent.\n  [verified] \u{2014} [the two age files](../../catalog/census-popest-2024.md). See\n  [an estimate is anchored to a census](../../decisions/an-estimate-is-anchored-to-a-census.yml).")],
        answers: &[],
        figures: &[
            Figure { label: "people", value: 157.0, literal: "157" },
            Figure { label: "children under 18", value: 811.0, literal: "811" },
        ],
    },
    Assertion {
        id: "one-adult-in-five-holds-a-degree",
        statement: "One Allen County adult in five holds a bachelor's degree. The county reads 19.9 \
                    per cent against Ohio's 30.9 and the nation's 35.0 \u{2014} 11.0 points below \
                    the state and 15.1 below the country, both several times their own margins.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-educational-attainment-2009-2023.yml", "**19.9 per cent of Allen County's adults hold a bachelor's degree or more, against 30.9 per cent of Ohio's and 35.0 of the nation's.** [verified] \u{2014} [the American Community Survey](../../catalog/census-acs-summary-file.md), table B15002, 2019\u{2013}2023 five-year estimates. The county is 11.0 points below the state and 15.1 below the nation, and both gaps clear their combined margins several times over."),
        ],
        answers: &["cannot say whether Allen County's adults finish high school at a different rate"],
        figures: &[
            Figure { label: "bachelor's degree or more, per cent of adults 25 and over", value: 19.9, literal: "19.9" },
            Figure { label: "Ohio", value: 30.9, literal: "30.9" },
            Figure { label: "United States", value: 35.0, literal: "35.0" },
        ],
    },
    Assertion {
        id: "the-deficit-is-all-at-the-far-end",
        statement: "The county's education deficit is not in its schooling. Below the diploma Allen \
                    County is unremarkable or better \u{2014} 1.7 per cent never reached ninth grade \
                    where the nation has 4.7 \u{2014} and what is out of the ordinary is the 40.7 \
                    per cent whose schooling ended at a high school diploma, 8.4 points above Ohio \
                    on a combined margin of 1.3.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-educational-attainment-2009-2023.yml", "**The deficit is not in the schooling. It is entirely at the far end.** 40.7 per cent of the county's adults hold a high school diploma and nothing further, against 32.3 per cent for Ohio and 26.2 for the nation \u{2014} 8.4 points above the state on a combined margin of 1.3. Below the diploma the county is unremarkable or better: 1.7 per cent never reached ninth grade where Ohio has 2.6 and the nation 4.7. [verified] \u{2014} the same table. What is rare here is not finishing school. It is carrying on afterwards."),
        ],
        answers: &["cannot say whether Allen County's adults finish high school at a different rate"],
        figures: &[
            Figure { label: "adults whose schooling ended at a diploma, per cent", value: 40.7, literal: "40.7" },
            Figure { label: "Ohio", value: 32.3, literal: "32.3" },
            Figure { label: "United States", value: 26.2, literal: "26.2" },
        ],
    },
    Assertion {
        id: "the-county-gained-and-the-state-gained-faster",
        statement: "Over fourteen years with no sample year in common, Allen County's degree-holding \
                    rose 4.1 points and Ohio's rose 7.4, so a gap of 7.8 points became one of 11.0. \
                    The county did not fall behind by standing still.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-educational-attainment-2009-2023.yml", "**Fourteen years apart, with no year in common, the county gained and the state gained faster.** The 2005\u{2013}2009 five-year file and the 2019\u{2013}2023 file share no sample year at all. Between them Allen County's bachelor's-or-more share rose 4.1 points and Ohio's rose 7.4, so a gap of 7.8 points became one of 11.0. [verified] \u{2014} [the 2009 summary file](../../catalog/census-acs-summary-file-2009.md) against [the 2023 one](../../catalog/census-acs-summary-file.md), table B15002 in both; see [an overlap names what changed](../../decisions/an-overlap-names-what-changed.yml) for why those two vintages and not the ones between."),
        ],
        answers: &["cannot say whether Allen County's adults finish high school at a different rate"],
        figures: &[
            Figure { label: "Allen County, change in bachelor's or more, points", value: 4.1, literal: "4.1" },
            Figure { label: "Ohio", value: 7.4, literal: "7.4" },
        ],
    },
    Assertion {
        id: "the-same-adults-better-educated",
        statement: "The widening gap is not dilution. Allen County's population aged 25 and over was \
                    68,304 in 2005\u{2013}2009 and 68,410 in 2019\u{2013}2023, a change of 106 \
                    people, while Ohio's grew 5.9 per cent. The county's adults are the same number \
                    of people, better educated than they were and further behind the state.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-educational-attainment-2009-2023.yml", "**The county did not fall behind by standing still.** Its population aged 25 and over was 68,304 in the earlier window and 68,410 in the later, a change of 106 people, while Ohio's grew 5.9 per cent \u{2014} so the widening gap is not newcomers arriving somewhere else and diluting a share. [verified] \u{2014} the same file in two vintages. The county's adults are the same number of people, better educated than they were, and further behind the state than they were."),
        ],
        answers: &["cannot say whether Allen County's adults finish high school at a different rate"],
        figures: &[
            Figure { label: "adults 25 and over, 2005\u{2013}2009", value: 68304.0, literal: "68,304" },
            Figure { label: "2019\u{2013}2023", value: 68410.0, literal: "68,410" },
        ],
    },
    Assertion {
        id: "lima-did-not-move-at-either-end",
        statement: "Lima has not measurably moved at either end of the scale in fourteen years. Its \
                    bachelor's-or-more share went 9.9 to 11.8 on a margin of 2.1 and its \
                    diploma-only share went 44.6 to 45.4 on a margin of 3.3 \u{2014} two tests on \
                    23,000 adults, and neither clears.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-educational-attainment-2009-2023.yml", "**Lima has not measurably moved at either end.** Its bachelor's-or-more share went 9.9 to 11.8 on a margin of 2.1, and the share whose schooling ended at a diploma went 44.6 to 45.4 on a margin of 3.3. Two tests fourteen years apart on 23,000 adults, and neither clears. [verified] \u{2014} the same file in two vintages. The county's own diploma-only share fell 42.2 to 40.7 on a margin of 1.7 and does not clear either."),
        ],
        answers: &["cannot say whether Allen County's adults finish high school at a different rate"],
        figures: &[
            Figure { label: "Lima, bachelor's or more, per cent", value: 11.8, literal: "11.8" },
            Figure { label: "margin", value: 2.1, literal: "2.1" },
        ],
    },
    Assertion {
        id: "one-township-above-the-national-rate",
        statement: "Richland Township, where Bluffton University stands, is the only ground in Allen \
                    County above the national rate for degree-holding, at 39.2 per cent. Lima reads \
                    11.8. Inside one county of 102,000 people the range is better than three to one.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-educational-attainment-2009-2023.yml", "**Inside the county the range is four to one.** [verified] \u{2014} the same table, by county subdivision."),
            support!("measure/allen-county-educational-attainment-2009-2023.yml", "Richland Township at 39.2 per cent is above the national rate and the only part of this county that is; it is also where [Bluffton University](../organization/bluffton-university.yml) stands, and the university's own students are mostly under 25 and outside this table's universe, so what is being counted there is the people who work at it and live near it. [inference] \u{2014} the same source against [the age structure](allen-county-age-structure-2023.yml)."),
        ],
        answers: &["cannot say whether Allen County's adults finish high school at a different rate"],
        figures: &[
            Figure { label: "Richland Township, bachelor's or more, per cent", value: 39.2, literal: "39.2" },
        ],
    },
    Assertion {
        id: "the-disability-rate-is-not-a-county-rate",
        statement: "Allen County's disability rate of 15.9 per cent is not a rate over Allen County. \
                    It is a rate over 99,436 people where the county has 101,685, and the 2,249 left \
                    out are in the two state prisons and the twelve nursing homes.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**The county's disability rate is 15.9 per cent and it is not a rate over the county.** It is a rate over 99,436 people where the county has 101,685, and the 2,249 left out are in the two state prisons and the twelve nursing homes. [verified] \u{2014} [the American Community Survey](../../catalog/census-acs-summary-file.md), tables B01001 and B18101, 2019\u{2013}2023 five-year estimates; see [the gap between two universes is a measurement](../../decisions/the-gap-between-two-universes-is-a-measurement.yml)."),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[
            Figure { label: "civilian noninstitutionalized population", value: 99436.0, literal: "99,436" },
            Figure { label: "total population", value: 101685.0, literal: "101,685" },
        ],
    },
    Assertion {
        id: "five-tables-five-populations",
        statement: "Five tables in one survey file describe five different populations of Allen \
                    County \u{2014} 101,685, 99,436, 97,786, 77,628 and 68,410 \u{2014} and the data \
                    files say so nowhere. The universe is printed in the table shells and only \
                    there.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**Five tables from one file describe five different populations of this county, and the data files never say so.** The universe is printed in the table shells and nowhere else. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[],
    },
    Assertion {
        id: "subtracting-two-universes-counts-an-institution",
        statement: "Subtracting the survey's civilian noninstitutionalized population from its total \
                    population, cell by cell across six age brackets, leaves 1,672 men and 577 women \
                    \u{2014} a census of an institutional population from a survey that tabulates no \
                    such thing.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**B01001 and B18101 are published on the same six age brackets by sex, so the difference between them can be taken cell by cell \u{2014} and it is a census of an institutional population that the survey tabulates nowhere.** 1,672 men and 577 women. [verified] \u{2014} the same tables, differenced here."),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[
            Figure { label: "men outside the civilian noninstitutionalized universe", value: 1672.0, literal: "1,672" },
            Figure { label: "women", value: 577.0, literal: "577" },
        ],
    },
    Assertion {
        id: "two-instruments-eleven-per-cent-apart",
        statement: "The subtraction gives 2,185 institutionalized residents of Allen County against \
                    2,479 the 2020 census counted \u{2014} two instruments built for different \
                    purposes, neither of them a count of institutions, eleven per cent apart.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**So 2,185 of the 2,249 are institutionalized, against 2,479 the census counted in 2020.** Two instruments built for different purposes, neither of them a count of institutions, eleven per cent apart. [verified] \u{2014} the same file, differenced as above, against [the group quarters](allen-county-group-quarters-2020.yml). The survey figure is the lower of the two and it averages five years to the census's one day, four of them after April 2020; Ohio's prison population fell in that period. [inference]"),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[
            Figure { label: "survey, institutionalized", value: 2185.0, literal: "2,185" },
            Figure { label: "2020 census", value: 2479.0, literal: "2,479" },
        ],
    },
    Assertion {
        id: "the-profile-names-the-institution",
        statement: "The shape of the excluded population names which institution it is, without \
                    either file naming one. 1,236 of them are men aged 18 to 64 against 1,513 the \
                    census counted in adult correctional facilities, and 897 are 65 or over against \
                    966 in nursing facilities. Under five the difference is zero in both sexes.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**The profile names which institution without the file naming either.** 1,236 of the excluded are men aged 18 to 64, against 1,513 people the census counted in adult correctional facilities; 897 are 65 or over, against 966 in nursing facilities. Under five the difference is zero in both sexes, as it must be. [verified] \u{2014} the same sources. The two lumps sit where the county's two kinds of institution are, and the residual is 116 people."),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[
            Figure { label: "men 18 to 64 excluded", value: 1236.0, literal: "1,236" },
            Figure { label: "people 65 and over excluded", value: 897.0, literal: "897" },
        ],
    },
    Assertion {
        id: "the-working-age-sex-ratio-is-a-prison",
        statement: "Allen County has more working-age men than working-age women and it does not. \
                    Over everybody there are 29,885 men aged 18 to 64 to 28,819 women, 103.7 to 100; \
                    over the civilian noninstitutionalized population it is 99.8 to 100. The excess \
                    is 1,236 people and it is a prison.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**Allen County has more working-age men than working-age women, and it does not.** Counted over everybody there are 29,885 men aged 18 to 64 to 28,819 women, 103.7 to 100. Counted over the civilian noninstitutionalized population there are 28,649 to 28,718, which is 99.8 to 100. [verified] \u{2014} the same tables. The county's excess of working-age men is 1,236 people and it is a prison. See [the age structure](allen-county-age-structure-2023.yml)."),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[
            Figure { label: "men per 100 women, 18 to 64, total population", value: 103.7, literal: "103.7" },
            Figure { label: "civilian noninstitutionalized", value: 99.8, literal: "99.8" },
        ],
    },
    Assertion {
        id: "in-lima-the-ratio-crosses-over",
        statement: "In Lima the same pair of numbers crosses over: 107.0 working-age men to 100 \
                    women over the city's total population and 96.3 over its civilian \
                    noninstitutionalized population, a swing of 10.7 where the national gap is 2.4. \
                    Both state prisons stand inside the city limits.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**In Lima the same pair of numbers crosses over.** The city reads 107.0 working-age men to 100 women over its total population and 96.3 over its civilian noninstitutionalized population, a swing of 10.7 \u{2014} from the most male working-age population in the county to one with fewer men than Ohio's. [verified] \u{2014} the same tables. Ohio reads 100.0 and 98.3, the nation 100.5 and 98.1; the gap between the two ratios is 2.4 points nationally and 10.7 in this city. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[
            Figure { label: "Lima, men per 100 women 18 to 64, total population", value: 107.0, literal: "107.0" },
            Figure { label: "civilian noninstitutionalized", value: 96.3, literal: "96.3" },
        ],
    },
    Assertion {
        id: "the-same-subtraction-finds-the-armed-forces",
        statement: "Run on the United States, the same subtraction returns the armed forces: \
                    1,286,167 people who are 18 or over and counted but not civilian. In Allen \
                    County it returns 64.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**The same subtraction, run on the United States, returns the armed forces.** The veteran table's universe is civilian and the disability table's is civilian *and* noninstitutionalized, so the difference between total population 18 and over and civilian population 18 and over is the active-duty force: 1,286,167 nationally. In Allen County it is 64. [verified] \u{2014} the same file, the national and county rows. This corpus holds no Defense Department strength report to check the national figure against, and records the agreement in order of magnitude rather than as a test. [inference]"),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[
            Figure { label: "national active-duty force implied by the subtraction", value: 1286167.0, literal: "1,286,167" },
            Figure { label: "Allen County", value: 64.0, literal: "64" },
        ],
    },
    Assertion {
        id: "a-history-of-conscription",
        statement: "Allen County's veteran share is 7.7 per cent against Ohio's 6.8 and the nation's \
                    6.4, and among men it is a history of conscription: 40.8 per cent of the \
                    county's men aged 75 and over are veterans, 26.6 per cent of those 65 to 74, and \
                    3.6 per cent of those 18 to 34.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-institutional-population-2023.yml", "**The veteran table is the one that keeps them.** Its universe is the civilian population 18 and over, institutions included, so Allen County's 5,948 veterans are a share of 77,628 rather than of 75,443 \u{2014} 7.7 per cent against Ohio's 6.8 and the nation's 6.4, a difference that clears its margin. [verified] \u{2014} the same file, table B21001. Among men the rate is 14.0 per cent and it is a history of conscription: 40.8 per cent of the county's men aged 75 and over are veterans, 26.6 per cent of those 65 to 74, and 3.6 per cent of those 18 to 34. [verified] \u{2014} the same table."),
        ],
        answers: &["cannot say what share of Allen County's people have a disability"],
        figures: &[
            Figure { label: "veterans, per cent of the civilian population 18 and over", value: 7.7, literal: "7.7" },
            Figure { label: "men 75 and over who are veterans, per cent", value: 40.8, literal: "40.8" },
        ],
    },
    Assertion {
        id: "the-poverty-rate-is-over-ninety-seven-thousand",
        statement: "Allen County's poverty rate of 13.1 per cent is a share of 97,786 people and not \
                    of the county's 101,685, because poverty status is not determined for anyone in \
                    an institution, in military quarters, in a dormitory, or under 15 and unrelated. \
                    Over everybody the same 12,815 people are 12.6 per cent.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-income-and-poverty-2023.yml", "**The 13.1 per cent is a share of 97,786 people and not of the county.** Poverty status is determined for everyone except people in institutional group quarters, people in military quarters, unrelated individuals under 15 and college students in dormitories, so the rate this node published rests on a base 3,899 smaller than the county's population. Over everybody the 12,815 are 12.6 per cent. [verified] \u{2014} [the same source](../../catalog/census-acs-summary-file.md), table B17001, its universe read from the table shells. Lima's base is 33,076 against a population of 35,304, so its 24.8 per cent is 23.3 over the whole city. [verified] \u{2014} the same table."),
        ],
        answers: &["does not establish that Lima has the lowest household income"],
        figures: &[
            Figure { label: "population for whom poverty status is determined", value: 97786.0, literal: "97,786" },
            Figure { label: "below poverty", value: 12815.0, literal: "12,815" },
        ],
    },
    Assertion {
        id: "what-the-county-gives",
        statement: "In the 2023\u{2014}24 cycle 1,331 people with an Allen County address gave \
                    $611,489 to federal campaigns, parties and political committees. In the \
                    1979\u{2014}80 cycle sixty-one people gave $35,606.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**In the 2023\u{2013}24 cycle 1,331 people with an Allen County address gave $611,489 to federal campaigns, parties and political committees.** Twelve presidential cycles, in dollars weighted to the county's share of each ZIP code's population. [verified] \u{2014} [the Commission's bulk files](../../catalog/fec-bulk-individual-contributions.md), deduplicated and netted of refunds here; see [weight a crosswalk by what it carries](../../decisions/weight-a-crosswalk-by-what-it-carries.yml)."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "itemized federal contributions, 2023\u{2013}24 cycle, dollars", value: 611489.0, literal: "611,489" },
            Figure { label: "donors", value: 1331.0, literal: "1,331" },
        ],
    },
    Assertion {
        id: "five-times-the-real-giving",
        statement: "Real federal giving per Allen County resident is five times what it was in 1980 \
                    \u{2014} $1.21 a head then, $6.06 in 2024 and $7.46 at its 2020 peak \u{2014} \
                    and the number of givers is twenty-two times. The county lost 11,375 residents \
                    over the same forty-four years.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**Real giving per resident is five times what it was in 1980 and the number of givers is twenty-two times.** $1.21 a head to $6.06, with a peak of $7.46 in 2020; 61 people to 1,331. [verified] \u{2014} the same file, deflated by [the consumer price index](../../catalog/bls-cpi.md). The county lost 11,375 residents over the same forty-four years. [verified] \u{2014} [the population](allen-county-population-2024.yml)."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "per resident, 2024 dollars, 2024 cycle", value: 6.06, literal: "6.06" },
            Figure { label: "1980 cycle", value: 1.21, literal: "1.21" },
            Figure { label: "residents lost since 1980", value: 11375.0, literal: "11,375" },
        ],
    },
    Assertion {
        id: "the-median-gift-fell-a-hundredfold",
        statement: "The median itemized contribution from Allen County fell from $860 in 1980 to $24 \
                    in 2024 \u{2014} from $3,274 to $24 in the same dollars, a factor of a hundred \
                    and thirty-six \u{2014} while the largest single gift rose from $1,000, the \
                    legal maximum then, to $50,000.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**The median itemized gift fell from $860 to $24.** In 2024 dollars the 1980 median is $3,274 and the 2024 median is $24, a fall of a hundred and thirty-six times, while the largest single contribution rose from $1,000 \u{2014} the legal maximum then \u{2014} to $50,000. [verified] \u{2014} the same source, deflated the same way. Both tails grew at once: the top ten givers were 20.4 per cent of the county's money in 2012 and 37.0 in 2020."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "median gift, 2024, dollars", value: 24.0, literal: "24" },
            Figure { label: "median gift 1980 in 2024 dollars", value: 3274.0, literal: "3,274" },
            Figure { label: "largest single gift, 2024", value: 50000.0, literal: "50,000" },
        ],
    },
    Assertion {
        id: "both-tails-grew-at-once",
        statement: "Allen County's political money got smaller and larger at the same time. The top \
                    ten givers were 20.4 per cent of it in 2012 and 37.0 per cent in 2020, in the \
                    same years the median gift fell below fifty dollars.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**The median itemized gift fell from $860 to $24.** In 2024 dollars the 1980 median is $3,274 and the 2024 median is $24, a fall of a hundred and thirty-six times, while the largest single contribution rose from $1,000 \u{2014} the legal maximum then \u{2014} to $50,000. [verified] \u{2014} the same source, deflated the same way. Both tails grew at once: the top ten givers were 20.4 per cent of the county's money in 2012 and 37.0 in 2020."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "top ten givers' share, 2020", value: 37.0, literal: "37.0" },
            Figure { label: "2012", value: 20.4, literal: "20.4" },
        ],
    },
    Assertion {
        id: "no-factory-worker-in-thirty-three-years",
        statement: "In thirty-three years of federal contribution records Allen County's itemized \
                    donors included no factory worker and one welder. In the three cycles since 2016 \
                    there are 939 contributions from factory workers and 353 from welders, and the \
                    distinct occupations recorded rose from 336 to 739.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**In thirty-three years of these records the county's itemized donors included no factory worker and one welder. In the three cycles since 2016 there are 939 contributions from factory workers and 353 from welders.** The distinct occupations recorded rose from 336 to 739. [verified] \u{2014} the same file, occupation strings as filed. Whether working people in Allen County began giving or merely began being written down is not something this file can separate: a contribution is itemized once it aggregates above $200 at one committee, and money routed through a conduit aggregates at the conduit, which is far easier to cross. [inference]"),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "factory-worker contributions, 2016\u{2013}2024", value: 939.0, literal: "939" },
            Figure { label: "welders", value: 353.0, literal: "353" },
            Figure { label: "distinct occupations, 2016\u{2013}2024", value: 739.0, literal: "739" },
        ],
    },
    Assertion {
        id: "the-congressman-is-the-largest-recipient",
        statement: "The largest single recipient of Allen County's federal money between 2000 and \
                    2012 is the county's own congressman: $167,879 to Jim Jordan for Congress, more \
                    than twice the next committee.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**The county's own congressman is the largest single recipient of its money in the middle era.** $167,879 to Jim Jordan for Congress across 2000 to 2012, more than twice the next committee. [verified] \u{2014} the same source. In the most recent three cycles the two largest are a national party committee and a super PAC that received one contribution of $50,000, the largest gift any Allen County resident has made in this record. [verified] \u{2014} the same file."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "to Jim Jordan for Congress, 2000\u{2013}2012", value: 167879.0, literal: "167,879" },
        ],
    },
    Assertion {
        id: "the-money-is-more-republican-than-the-vote",
        statement: "Where both can be measured, Allen County's money is more Republican than its \
                    vote and the gap has closed. In 2008 the county gave McCain 60.5 per cent of its \
                    two-party vote and Republican committees 89.0 per cent of its labelled money; in \
                    2020 it gave Trump 70.1 per cent and Republican committees 77.0.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**Where both can be measured, the money is more Republican than the vote, and the gap has closed from twenty-eight points to seven.** In 2008 the county gave McCain 60.5 per cent of its two-party vote and Republican committees 89.0 per cent of its labelled money; in 2020 it gave Trump 70.1 per cent and Republican committees 77.0. [verified] \u{2014} the same source against [the presidential vote](allen-county-presidential-vote-2000-2016.yml) and [the 2020 return](allen-county-presidential-vote-2020.yml)."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "Republican share of labelled money, 2008", value: 89.0, literal: "89.0" },
            Figure { label: "Republican share of the two-party vote, 2008", value: 60.5, literal: "60.5" },
        ],
    },
    Assertion {
        id: "half-the-money-has-no-party",
        statement: "The share of Allen County's federal political money that the Election \
                    Commission's own files can assign to a party has fallen from 98.9 per cent to \
                    52.1. Joint fundraising committees and conduits carry no party and no candidate \
                    to borrow one from, and they went from nothing to half the money.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**The share of this money that the Commission's own files can assign to a party has fallen from 98.9 per cent to 52.1.** Joint fundraising committees and conduits carry no party in the committee master and no candidate to borrow one from, and they went from nothing to half the money. [verified] \u{2014} the same source against [the committee and candidate masters](../../catalog/fec-bulk-individual-contributions.md)."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "labelled share, 1980", value: 98.9, literal: "98.9" },
            Figure { label: "2024", value: 52.1, literal: "52.1" },
        ],
    },
    Assertion {
        id: "the-1984-row-is-the-file",
        statement: "Allen County's 1984 row shows six donors and $2,884, against sixty-one and \
                    $35,606 four years earlier. The Commission's 1983\u{2014}84 archive is smaller \
                    than its 1979\u{2014}80 one nationally, so the collapse is in the record and not \
                    in the county.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**The 1984 row is the file and not the county.** Six donors and $2,884, against sixty-one and $35,606 four years earlier \u{2014} and the Commission's 1983\u{2013}84 archive is smaller than its 1979\u{2013}80 one, nationally, at 6.8 megabytes against 8.1. [verified] \u{2014} the same file, its archive size and its rows. Nothing is read from that row."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "given in the 1984 cycle, dollars", value: 2884.0, literal: "2,884" },
            Figure { label: "given four years earlier", value: 35606.0, literal: "35,606" },
        ],
    },
    Assertion {
        id: "reading-the-archive-twice",
        statement: "Read without care the 2024 file says Allen County gave $1,528,070. Every record \
                    is in the archive twice from 2012 on, and a contribution made through a conduit \
                    is filed again by the committee that banks it. The figure is $611,489 \u{2014} \
                    the naive reading is two and a half times it.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**Read without care the 2024 file says this county gave $1,528,070.** Every record is in the archive twice from 2012 on, because the zip holds `itcont.txt` and a `by_date/` partition of the same rows; and a contribution made through a conduit is filed again by the committee that banks it. Deduplicating on the record identifier halves the figure and deduplicating the conduits takes another fifth off it. The naive reading is two and a half times the $611,489 above. [verified] \u{2014} the same file, read three ways."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "naive total, 2024", value: 1528070.0, literal: "1,528,070" },
            Figure { label: "after deduplication", value: 611489.0, literal: "611,489" },
        ],
    },
    Assertion {
        id: "the-zip-rule-is-worth-a-factor-of-two",
        statement: "Assigning a ZIP-coded contribution to Allen County needs a rule, and the rule is \
                    worth a factor of two: $454,593 under the eight wholly-inside codes, $650,446 \
                    under the thirteen with a majority of their people here, and $959,382 under \
                    every code that touches the county at all. The corpus publishes $611,489, \
                    weighted by population.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-federal-contributions-1980-2024.yml", "**The rule for turning a ZIP code into a county is worth a factor of two.** The 2024 total is $454,593 under the eight postal codes that lie wholly in Allen County, $650,446 under the thirteen with a majority of their people here, and $959,382 under every code that touches the county at all. This node publishes $611,489, weighted by population, and prints the others because a reader comparing it with a figure from elsewhere has no other way to know the rules differed. [verified] \u{2014} the same file against [the crosswalk](allen-county-zip-codes-2020.yml)."),
        ],
        answers: &["cannot say whether Allen County's political money has become less Republican"],
        figures: &[
            Figure { label: "eight wholly-inside codes", value: 454593.0, literal: "454,593" },
            Figure { label: "every code that touches", value: 959382.0, literal: "959,382" },
            Figure { label: "population-weighted", value: 611489.0, literal: "611,489" },
        ],
    },
    Assertion {
        id: "twenty-postal-areas-touch-the-county",
        statement: "Twenty ZIP-code areas touch Allen County and eight lie wholly inside it. Those \
                    eight hold 73,421 of the county's 102,206 people; the other 28,785 live under a \
                    postal address they share with one, two or three other counties.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-zip-codes-2020.yml", "**Twenty ZIP-code areas touch Allen County and eight lie wholly inside it.** Between them the eight hold 73,421 of the county's 102,206 people \u{2014} 71.8 per cent \u{2014} and the other 28,785 live under a postal address they share with one, two or three other counties. [verified] \u{2014} [the Census Bureau's ZCTA relationship files](../../catalog/census-zcta-relationship-files.md) joined to [the 2020 redistricting file](../../catalog/census-2020-redistricting-file.md), computed here."),
        ],
        answers: &["cannot say which county a person with a Delphos address lives in"],
        figures: &[
            Figure { label: "people in the eight wholly-inside codes", value: 73421.0, literal: "73,421" },
            Figure { label: "people under a shared postal address", value: 28785.0, literal: "28,785" },
        ],
    },
    Assertion {
        id: "the-join-closes-on-the-county",
        statement: "The Allen County parts of twenty postal areas sum to 102,206 people, which is \
                    the county's census population to the person. Twenty areas built to deliver mail \
                    and a block file built to apportion legislatures agree exactly on a county \
                    neither of them names.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-zip-codes-2020.yml", "**The Allen County parts sum to 102,206, which is the county.** Twenty postal areas built to deliver mail and a block file built to apportion legislatures agree to the person on a county neither of them names. [verified] \u{2014} the same sources, joined. Nothing here was scaled or reconciled; the closure is the check that the block assignment is right."),
        ],
        answers: &["cannot say which county a person with a Delphos address lives in"],
        figures: &[
            Figure { label: "people in the Allen County parts", value: 102206.0, literal: "102,206" },
        ],
    },
    Assertion {
        id: "land-is-the-wrong-weight-for-people",
        statement: "Land and population disagree by up to thirty-two points about how much of a ZIP \
                    code is in Allen County, and in both directions. Bluffton's is 49.7 per cent of \
                    the county by land and 81.3 by people; the rural code north-west of Elida runs \
                    the other way, 32.8 by land against 21.2 by people.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-zip-codes-2020.yml", "**Land and population disagree by up to thirty-two points, and in both directions.** Bluffton's 45817 is 49.7 per cent of Allen County by land and 81.3 per cent by people; Spencerville's 45887 is 55.2 against 82.3; Harrod's 45850 is 80.7 against 91.8. Going the other way, the rural 45830 is 32.8 per cent Allen by land and 21.2 by people, and Ada's 45810 is 8.0 against 2.9. [verified] \u{2014} the same files. A village sits on one side of a county line and the farmland around it does not, so the weight that is easy to publish is the weight that is wrong. See [weight a crosswalk by what it carries](../../decisions/weight-a-crosswalk-by-what-it-carries.yml)."),
        ],
        answers: &["cannot say which county a person with a Delphos address lives in"],
        figures: &[
            Figure { label: "Bluffton's ZIP, per cent of Allen by land", value: 49.7, literal: "49.7" },
            Figure { label: "by people", value: 81.3, literal: "81.3" },
        ],
    },
    Assertion {
        id: "three-towns-a-land-weight-misplaces",
        statement: "Three of Allen County's nine municipal corporations sit on a ZIP that is mostly \
                    outside the county by land and mostly inside it by people \u{2014} Delphos at \
                    45.9 per cent of land against 55.3 of people, Bluffton at 49.7 against 81.3, \
                    Spencerville at 55.2 against 82.3. They are the three the county shares with a \
                    neighbour.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-zip-codes-2020.yml", "**Three of the county's nine municipal corporations sit on a ZIP that is mostly outside the county by land and mostly inside it by people.** [Delphos](../place/delphos.yml), [Bluffton](../place/bluffton.yml) and [Spencerville](../place/spencerville.yml) \u{2014} 45833 at 45.9 per cent of land and 55.3 of people, 45817 at 49.7 and 81.3, 45887 at 55.2 and 82.3. [verified] \u{2014} the same source, against [the nine corporations](allen-county-roads-2010-2024.yml). Delphos is the hardest of the three: the city itself straddles the Allen\u{2013}Van Wert line, so neither its ZIP nor its corporation lies in one county. [verified] \u{2014} [the city](../place/delphos.yml)."),
        ],
        answers: &["cannot say which county a person with a Delphos address lives in"],
        figures: &[
            Figure { label: "Spencerville's ZIP, per cent of Allen by people", value: 82.3, literal: "82.3" },
        ],
    },
    Assertion {
        id: "one-black-child-in-three-suspended",
        statement: "In the 2013\u{2014}14 school year 30.6 per cent of Allen County's Black \
                    schoolchildren were suspended out of school and 8.1 per cent of its white ones. \
                    Nearly one Black child in three.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-discipline-2011-2021.yml", "**In the 2013\u{2013}14 school year 30.6 per cent of Allen County's Black schoolchildren were suspended out of school, and 8.1 per cent of its white ones.** Nearly one Black child in three. [verified] \u{2014} [the Civil Rights Data Collection](../../catalog/crdc-civil-rights-data-collection.md), the county's twelve districts, its own enrolment as the denominator."),
        ],
        answers: &["cannot say how many children were expelled from an Allen County school in 2013"],
        figures: &[
            Figure { label: "Black pupils suspended out of school, per cent, 2013\u{2013}14", value: 30.6, literal: "30.6" },
            Figure { label: "white pupils", value: 8.1, literal: "8.1" },
        ],
    },
    Assertion {
        id: "the-county-suspends-more-of-everybody",
        statement: "Allen County's racial disparity in suspension is smaller than Ohio's and its \
                    rate is higher for everybody. The Black-to-white ratio runs 3.2 to 4.1 here \
                    against 3.4 to 4.8 statewide, and to 2015 both of the county's rates run above \
                    both of the state's \u{2014} 8.1 against 4.9 for white children and 30.6 against \
                    19.2 for Black ones.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-discipline-2011-2021.yml", "**The county's racial disparity is smaller than Ohio's and its rate is higher for everybody.** The ratio between the Black and white rates is 3.2 to 4.1 here against 3.4 to 4.8 statewide, and in every year to 2015 both of this county's rates run above both of the state's \u{2014} 8.1 against 4.9 for white children in 2013 and 30.6 against 19.2 for Black ones. [verified] \u{2014} the same source. What is unusual about Allen County is not that it suspends Black children at several times the rate of white ones, which Ohio does too; it is that it suspends more of both. [inference]"),
        ],
        answers: &["cannot say how many children were expelled from an Allen County school in 2013"],
        figures: &[
            Figure { label: "Ohio white pupils suspended, per cent, 2013\u{2013}14", value: 4.9, literal: "4.9" },
            Figure { label: "Ohio Black pupils", value: 19.2, literal: "19.2" },
        ],
    },
    Assertion {
        id: "suspension-halved-in-two-years",
        statement: "Out-of-school suspension in Allen County halved between 2015 and 2017, from 11.7 \
                    per cent of pupils to 6.0, while Ohio's rate went from 7.2 to 6.7. Nothing \
                    retrieved names a policy, a superintendent or a board resolution.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-discipline-2011-2021.yml", "**Suspension halved between 2015 and 2017 and Ohio's barely moved.** The county went from 11.7 per cent to 6.0 while the state went from 7.2 to 6.7. [verified] \u{2014} the same source. Nothing retrieved here names a policy, a superintendent or a board resolution, and the corpus records the size and the date of the fall without a cause. [inference]"),
        ],
        answers: &["cannot say how many children were expelled from an Allen County school in 2013"],
        figures: &[
            Figure { label: "county suspension rate, per cent, 2015\u{2013}16", value: 11.7, literal: "11.7" },
            Figure { label: "2017\u{2013}18", value: 6.0, literal: "6.0" },
        ],
    },
    Assertion {
        id: "ohio-stopped-suspending-and-this-county-did-not",
        statement: "In the pandemic year Ohio nearly stopped suspending children, its rate falling \
                    by more than two thirds from 6.7 per cent to 2.1. Allen County's fell from 6.0 \
                    to 5.5, and the county suspended at 2.6 times the state rate \u{2014} the widest \
                    gap in the series.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-discipline-2011-2021.yml", "**In the pandemic year Ohio nearly stopped suspending children and this county did not.** The state's rate fell from 6.7 per cent to 2.1 \u{2014} a fall of more than two thirds \u{2014} while Allen County's fell from 6.0 to 5.5. The county suspended at 2.6 times the state rate that year, the widest gap in the series. [verified] \u{2014} the same source. Both figures cover 2020\u{2013}21, a year in which children were in a building for part of it and at home for the rest, in proportions this file does not give. [inference]"),
        ],
        answers: &["cannot say how many children were expelled from an Allen County school in 2013"],
        figures: &[
            Figure { label: "Ohio suspension rate, per cent, 2020\u{2013}21", value: 2.1, literal: "2.1" },
            Figure { label: "Allen County", value: 5.5, literal: "5.5" },
        ],
    },
    Assertion {
        id: "a-third-of-suspensions-a-seventh-of-the-roll",
        statement: "Black children are between 13.9 and 15.7 per cent of Allen County's school roll \
                    and between 30.5 and 39.6 per cent of its out-of-school suspensions, in every \
                    one of the six years the collection covers. The share of suspensions rose while \
                    the share of the roll did not.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-discipline-2011-2021.yml", "**Black children are between 13.9 and 15.7 per cent of the county's roll and between 30.5 and 39.6 per cent of its out-of-school suspensions in every one of the six years.** [verified] \u{2014} the same source, computed here. The share of suspensions rose while the share of the roll did not."),
        ],
        answers: &["cannot say how many children were expelled from an Allen County school in 2013"],
        figures: &[
            Figure { label: "Black share of the roll, highest year, per cent", value: 15.7, literal: "15.7" },
            Figure { label: "Black share of suspensions, highest year", value: 39.6, literal: "39.6" },
        ],
    },
    Assertion {
        id: "no-expulsion-figure-for-2013",
        statement: "Allen County's schools report 836 expulsions in 2013 against 97 in 2011 and 83 \
                    in 2015, and the figure is an artefact: six of them enter the same number in \
                    both of two mutually exclusive columns. Across Ohio, 438 of the 570 schools \
                    reporting both columns that year report them equal.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-discipline-2011-2021.yml", "**This corpus cannot say how many children were expelled from an Allen County school in 2013.** The county's schools report 836 expulsions that year against 97 in 2011 and 83 in 2015, and the figure is an artefact: six of them enter the same number in both of two mutually exclusive columns \u{2014} 110 and 110, 71 and 71, 67 and 67. Across Ohio, 438 of the 570 schools reporting both columns in 2013 report them equal, against 23 to 48 per cent of schools in every other year. [verified] \u{2014} the same file, counted here. Halving the duplicates still leaves 442, which is four times any neighbouring year, so the column is not repairable and no expulsion figure is published for 2013."),
        ],
        answers: &["cannot say how many children were expelled from an Allen County school in 2013"],
        figures: &[
            Figure { label: "expulsions reported, 2013", value: 836.0, literal: "836" },
            Figure { label: "Ohio schools reporting the two columns equal", value: 438.0, literal: "438" },
        ],
    },
    Assertion {
        id: "absenteeism-doubled",
        statement: "Chronic absenteeism in Allen County's schools doubled between 2013 and 2021, \
                    from 14.9 per cent of pupils to 30.0. A chronically absent child is one who \
                    misses fifteen school days or more in a year.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-chronic-absenteeism-2013-2022.yml", "**Chronic absenteeism in Allen County's schools doubled between 2013 and 2021, from 14.9 per cent of pupils to 30.0.** A chronically absent child is one who misses fifteen school days or more in a year. [verified] \u{2014} [the Civil Rights Data Collection](../../catalog/crdc-civil-rights-data-collection.md), the county's twelve districts, its own enrolment as the denominator."),
        ],
        answers: &["cannot say what share of Allen County's children were chronically absent in 2022"],
        figures: &[
            Figure { label: "chronically absent, per cent, 2013\u{2013}14", value: 14.9, literal: "14.9" },
            Figure { label: "2021\u{2013}22", value: 30.0, literal: "30.0" },
        ],
    },
    Assertion {
        id: "half-the-black-children-absent",
        statement: "Nearly half of Allen County's Black schoolchildren were chronically absent in \
                    2021\u{2014}22 \u{2014} 44.5 per cent, against 25.0 per cent of white children \
                    and 18.9 per cent of Black children eight years earlier. The gap between the two \
                    rates opened from 5.7 points to 19.5.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-chronic-absenteeism-2013-2022.yml", "**Nearly half of the county's Black schoolchildren were chronically absent in 2021\u{2013}22.** 44.5 per cent, against 25.0 for white children and 18.9 for Black children eight years earlier. [verified] \u{2014} the same source. The gap between the two rates opened from 5.7 points in 2013 to 19.5 in 2021."),
        ],
        answers: &["cannot say what share of Allen County's children were chronically absent in 2022"],
        figures: &[
            Figure { label: "Black pupils chronically absent, per cent, 2021\u{2013}22", value: 44.5, literal: "44.5" },
            Figure { label: "white pupils", value: 25.0, literal: "25.0" },
        ],
    },
    Assertion {
        id: "better-on-absence-worse-on-suspension",
        statement: "On absence Allen County is close to Ohio and better than it for Black children \
                    in every year, by 6.4 points in 2013 and 11.2 in 2021. On suspension it runs \
                    above the state on every rate. Absence and suspension are not one thing measured \
                    twice.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-chronic-absenteeism-2013-2022.yml", "**On this measure the county is close to Ohio and better than it for Black children.** Its overall rate runs within a point or two of the state's in four of the five years and 3.5 points below it in 2021; its Black rate is below the state's in every year, 18.9 against 25.3 in 2013 and 44.5 against 55.7 in 2021. [verified] \u{2014} the same source. That is the opposite of what [the discipline record](allen-county-school-discipline-2011-2021.yml) says, where this county runs above Ohio on every rate. Absence and suspension are not the same thing measured twice. [inference]"),
        ],
        answers: &["cannot say what share of Allen County's children were chronically absent in 2022"],
        figures: &[
            Figure { label: "county Black absence rate, per cent, 2021\u{2013}22", value: 44.5, literal: "44.5" },
            Figure { label: "Ohio", value: 55.7, literal: "55.7" },
        ],
    },
    Assertion {
        id: "two-collections-disagree-about-the-children",
        statement: "Two federal collections count Allen County's schoolchildren in the same years \
                    and disagree by 0.6 to 8.0 per cent \u{2014} 15,461 against 16,127 in 2021, \
                    14,839 against 16,124 in the pandemic year. A rate built from one collection's \
                    numerator over the other's denominator is wrong before any measuring starts.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-chronic-absenteeism-2013-2022.yml", "**The two collections disagree about how many children are in this county, and by enough to matter.** 15,461 against 16,127 in 2021, and 14,839 against 16,124 in the pandemic year. [verified] \u{2014} the same source against [the Common Core of Data](../../catalog/nces-common-core-of-data.md)."),
        ],
        answers: &["cannot say what share of Allen County's children were chronically absent in 2022"],
        figures: &[
            Figure { label: "civil-rights collection, 2021", value: 15461.0, literal: "15,461" },
            Figure { label: "district files", value: 16127.0, literal: "16,127" },
        ],
    },
    Assertion {
        id: "the-net-of-seven-is-a-turnover-of-nineteen",
        statement: "Lima reports seven fewer school buildings than it did in 1986, and the seven are \
                    a turnover of nineteen: thirteen buildings left the count and six entered it. \
                    Only three identifiers run the whole way \u{2014} North, South and West junior \
                    highs.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**The net of seven is a turnover of nineteen.** The school-level directory, read at fourteen dates between 1986 and 2024, shows thirteen of Lima's sixteen buildings leaving the count and six entering it. Only three identifiers run the whole way: North, South and West junior highs, now North Middle, South Science-Technology Magnet K-8 and West Middle. [verified] \u{2014} [the same collection](../../catalog/nces-common-core-of-data.md), its school directory, differenced here."),
        ],
        answers: &["cannot say where any of these buildings stood"],
        figures: &[],
    },
    Assertion {
        id: "ten-buildings-in-four-years",
        statement: "Between 2002 and 2005 Lima closed nine elementary schools and its high school \
                    and opened five elementary schools. A district that reported sixteen buildings \
                    in 1986 and seventeen in 2002 reported twelve in 2005: what the count shows as a \
                    slow decline was a rebuild compressed into four years.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**Ten of the thirteen left in four years.** Between 2002 and 2005 Lima closed nine elementary schools and its high school and opened five elementary schools, so a district that had reported sixteen buildings in 1986 and seventeen in 2002 reported twelve in 2005. [verified] \u{2014} the same file. What the count of buildings shows as a slow decline was, on the ground, a rebuild compressed into four years."),
        ],
        answers: &["cannot say where any of these buildings stood"],
        figures: &[],
    },
    Assertion {
        id: "named-for-people-named-for-ideas",
        statement: "The Lima schools that closed are named for people and the schools that replaced \
                    them are named for ideas. Edison, Emerson, Faurot, Horace Mann, Irving, \
                    Jefferson, Lincoln, Lowell, Roosevelt, Washington McKinley, Westwood and \
                    Whittier gave way to Freedom, Heritage, Independence, Liberty and Unity.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**The schools that closed are named for people and the schools that replaced them are named for ideas.** Edison, Emerson, Faurot, Horace Mann, Irving, Jefferson, Lincoln, Lowell, Roosevelt, Washington McKinley, Westwood and Whittier gave way to Freedom, Heritage, Independence, Liberty and Unity. [verified] \u{2014} the same file. Nothing retrieved here says who chose the names."),
        ],
        answers: &["cannot say where any of these buildings stood"],
        figures: &[],
    },
    Assertion {
        id: "one-identifier-two-schools",
        statement: "One record number in the federal school directory is Lincoln Elementary in 1986 \
                    and Lima Alternative by 1995, and it runs to 2013. A reader tracking that number \
                    across the series is tracking two schools.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**One identifier changed schools underneath the count.** The record numbered `390442201155` is Lincoln Elementary in 1986 and Lima Alternative by 1995, and it runs to 2013 \u{2014} so a reader tracking that number across the series is tracking two schools. [verified] \u{2014} the same file; see [a total is checked against one it did not come from](../../decisions/a-total-is-checked-against-one-it-did-not-come-from.yml)."),
        ],
        answers: &["cannot say where any of these buildings stood"],
        figures: &[],
    },
    Assertion {
        id: "the-water-in-the-cut-does-not-move",
        statement: "The water standing in the Miami and Erie Canal's Deep Cut is level at 835.29 \
                    feet above sea level over 7,375 feet of channel, to a standard deviation of 1.8 \
                    inches. That flatness is what distinguishes a canal pound from a ditch.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**The water in the cut stands at 835.29 feet and it does not move.** Over 7,375 feet \u{2014} 1.397 miles \u{2014} the surface reads 835.29 feet above sea level with a standard deviation of 1.8 inches and a full range of 18. [verified] \u{2014} [USGS 3DEP](../../catalog/usgs-3dep-elevation.md), a 1,614 \u{d7} 2,605 export at one metre, traced along the channel bottom row by row. That flatness is the measurement: it is what a canal pound is, and no silted ditch or natural channel produces it."),
        ],
        answers: &["cannot say how much earth was moved to dig it"],
        figures: &[
            Figure { label: "water surface, feet above sea level", value: 835.29, literal: "835.29" },
            Figure { label: "length of the level pound, feet", value: 7375.0, literal: "7,375" },
        ],
    },
    Assertion {
        id: "fifty-two-feet-and-forty-five-point-eight",
        statement: "The National Park Service says the Deep Cut \u{201c}ranges at places to 52 feet \
                    in depth\u{201d}. The elevation surface gives 45.8 feet from the crest of the \
                    spoil bank to the water and 27.8 feet from the fields at the same section. All \
                    three are right, and they differ because they run between different pairs of \
                    surfaces.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**The nomination says the cut runs to 52 feet deep, and the ground gives three answers.** [verified] \u{2014} [the nomination](../../catalog/nrhp-nomination-documents.md), whose description section reads *\"As shallow as five feet, the Deep Cut section of the Canal ranges at places to 52 feet in depth\"* and *\"The Deep Cut extends over a mile.\"*"),
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**Eighteen feet of the deepest reading is earth that came out of the hole.** At the section where the cut measures 45.8 feet from its bank crest it measures 27.8 feet from the fields, because the spoil was cast into banks standing 18.0 feet above the ground it was thrown onto. Averaged over the pound the banks stand 5.0 feet proud. [verified] \u{2014} the same source, in cross-section every metre."),
        ],
        answers: &["cannot say how much earth was moved to dig it"],
        figures: &[
            Figure { label: "the nomination's depth, feet", value: 52.0, literal: "52" },
            Figure { label: "spoil crest to water, feet", value: 45.8, literal: "45.8" },
            Figure { label: "fields to water, feet", value: 27.8, literal: "27.8" },
        ],
    },
    Assertion {
        id: "five-feet-of-water-closes-the-gap",
        statement: "The elevation model stops at the water, and Miller's 1906 history gives the \
                    Miami extension five feet of it. 45.8 and 5 make 50.8 against the nomination's \
                    52, so a disagreement that looked like six feet is one.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**The third line is the nomination's, and it is reached by adding five feet of water.** The model returns the topmost surface, so a depth taken from it stops at whatever floats on the thing; [Miller's history](../../catalog/miller-allen-county-1906.md) gives the Miami extension five feet of water, 36 feet of width at the bottom and 50 at the top. 45.8 and 5 make 50.8 against a claim of 52. [inference] \u{2014} computed here; see [a depth needs both of its ends named](../../decisions/a-depth-needs-both-of-its-ends-named.yml)."),
        ],
        answers: &["cannot say how much earth was moved to dig it"],
        figures: &[
            Figure { label: "crest to canal bottom, feet", value: 50.8, literal: "50.8" },
        ],
    },
    Assertion {
        id: "eighteen-feet-of-the-depth-came-out-of-the-hole",
        statement: "Where the Deep Cut measures 45.8 feet from the crest of its bank it measures \
                    27.8 feet from the fields, because the spoil was cast into banks standing 18.0 \
                    feet above the ground it was thrown onto. Over the whole pound the banks stand \
                    5.0 feet proud.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**Eighteen feet of the deepest reading is earth that came out of the hole.** At the section where the cut measures 45.8 feet from its bank crest it measures 27.8 feet from the fields, because the spoil was cast into banks standing 18.0 feet above the ground it was thrown onto. Averaged over the pound the banks stand 5.0 feet proud. [verified] \u{2014} the same source, in cross-section every metre."),
        ],
        answers: &["cannot say how much earth was moved to dig it"],
        figures: &[
            Figure { label: "spoil above the fields at the deepest section, feet", value: 18.0, literal: "18.0" },
            Figure { label: "spoil above the fields, mean, feet", value: 5.0, literal: "5.0" },
        ],
    },
    Assertion {
        id: "the-landmark-is-thirty-seven-per-cent-in-another-county",
        statement: "Allen County's only National Historic Landmark crosses the county line. 4,617 \
                    feet of the Deep Cut's level pound lie in Spencer Township and 2,759 in Salem \
                    Township, Auglaize County, and the listing is filed under Allen alone.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**Allen County's only Landmark is 37 per cent in Auglaize County.** The pound crosses the county line at 40.685033, \u{2212}84.365726: 4,617 feet of it lie in Spencer Township and 2,759 in Salem Township, Auglaize. [verified] \u{2014} [the county-subdivision file](../../catalog/census-tiger-hydrography.md), point in polygon along the traced channel. The listing is filed under Allen alone, and Auglaize's twenty-five listings include no canal and no Landmark. [verified] \u{2014} [the National Register](../../catalog/nrhp-national-register.md), both counties queried."),
        ],
        answers: &["cannot say how much earth was moved to dig it"],
        figures: &[
            Figure { label: "in Allen County, feet", value: 4617.0, literal: "4,617" },
            Figure { label: "in Auglaize County, feet", value: 2759.0, literal: "2,759" },
        ],
    },
    Assertion {
        id: "the-registers-boundary-is-a-box-over-half-of-it",
        statement: "The National Register's polygon for the Deep Cut is a box drawn round bounding \
                    coordinates at 1:24,000. It runs 3,871 feet against the 7,375 feet of level \
                    channel, and it straddles the county line too.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**The Register's own polygon is a box, and it covers half the pound.** It runs 3,871 feet against the level channel's 7,375, it carries `BND_TYPE` \"Circumscribed polygon\" derived from bounding coordinates at 1:24,000 with \u{b1}12 metres claimed, and it straddles the county line as well. [verified] \u{2014} the same dataset, its boundary fields."),
        ],
        answers: &["cannot say how much earth was moved to dig it"],
        figures: &[
            Figure { label: "the Register's box, feet", value: 3871.0, literal: "3,871" },
            Figure { label: "the level channel, feet", value: 7375.0, literal: "7,375" },
        ],
    },
    Assertion {
        id: "the-water-is-as-wide-as-the-canals-floor",
        statement: "The water in the Deep Cut is 36.1 feet wide. Miller's history gives the canal 36 \
                    feet at the bottom and 50 at the top, so what stands in it is a canal well below \
                    its working level.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**The water is 36.1 feet wide, which is the width of the canal's floor and not of its top.** [verified] \u{2014} the same source, measured as the run of surface within five inches of the pound level; the tenth and ninetieth percentiles are 16.4 and 45.9 feet. Against Miller's 36 at the bottom and 50 at the top, a canal holding water to its floor width is a canal well below its working level. [inference]"),
            support!("measure/miami-and-erie-canal-in-allen-county-2026.yml", "**As built it was 5 feet deep, 36 feet wide at the bottom and 50 at the top.** The Miami extension was completed in 1845 and ran 114 miles. [verified] \u{2014} [Miller, 1906](../../catalog/miller-allen-county-1906.md). The name is younger than the work: by an act of 14 March 1849 the Miami Canal, the Miami Extension Canal and the Wabash and Erie became one Miami and Erie Canal. [verified] \u{2014} the same book."),
        ],
        answers: &["cannot say how much earth was moved to dig it", "cannot say how many miles of the canal lie in Allen County"],
        figures: &[
            Figure { label: "width of the standing water, feet", value: 36.1, literal: "36.1" },
        ],
    },
    Assertion {
        id: "deep-cut-was-a-place-before-it-was-a-landmark",
        statement: "Deep Cut was a settlement on the canal before it was a Landmark on it. J. H. \
                    Dunathan, an ex-commissioner of Auglaize County, moved his general store from \
                    Deep Cut into Spencerville in August 1881.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**Deep Cut was a settlement before it was a Landmark.** J. H. Dunathan, an ex-commissioner of Auglaize County, moved his general store \"from Deep Cut on the canal\" into Spencerville in August 1881. [verified] \u{2014} [Miller, 1906](../../catalog/miller-allen-county-1906.md)."),
        ],
        answers: &["cannot say how much earth was moved to dig it"],
        figures: &[],
    },
    Assertion {
        id: "how-much-earth-was-moved-is-a-choice-of-datum",
        statement: "The excavation still open below the surrounding grade at the Deep Cut measures \
                    449,450 cubic yards if the original ground is taken 80 metres out and 706,339 if \
                    it is taken 170 metres out. There is no third source to choose between them, so \
                    the corpus publishes the range.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-deep-cut-2026.yml", "**This corpus cannot say how much earth was moved to dig it.** The excavation still open below the surrounding grade measures 449,450 cubic yards if the original ground is taken 80 metres out and 706,339 if it is taken 170 metres out, and there is no third source to choose between them. [verified] \u{2014} [the elevation surface](../../catalog/usgs-3dep-elevation.md), integrated four ways. What has silted into the trench, and what was carted off to build embankment elsewhere, are in neither figure."),
        ],
        answers: &["cannot say how much earth was moved to dig it"],
        figures: &[
            Figure { label: "void, grade taken 80 m out, cubic yards", value: 449450.0, literal: "449,450" },
            Figure { label: "void, grade taken 170 m out, cubic yards", value: 706339.0, literal: "706,339" },
        ],
    },
    Assertion {
        id: "one-canal-three-features-two-spellings",
        statement: "Allen County's water file names the Miami and Erie Canal three times under two \
                    spellings, running 6.1157 miles in three pieces, and none of the three is the \
                    stretch that is a National Historic Landmark.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-in-allen-county-2026.yml", "**The county's water file names it three times, under two spellings, and none of the three is the Landmark.** `Miami & Erie Cnl` carries two features and `Miami-Erie Cnl` a third; together they run 32,290.7 feet \u{2014} 6.1157 miles \u{2014} and all of them lie north of the Deep Cut. [verified] \u{2014} [TIGER/Line hydrography](../../catalog/census-tiger-hydrography.md), projected to EPSG:26916."),
        ],
        answers: &["cannot say how many miles of the canal lie in Allen County"],
        figures: &[
            Figure { label: "canal linework named for it, miles", value: 6.1157, literal: "6.1157" },
        ],
    },
    Assertion {
        id: "what-carries-the-name-falls-six-feet-to-the-mile",
        statement: "Along the 6.1157 miles the water file draws, the channel bottom descends 41.9 \
                    feet at 5.94 feet to the mile and fits a straight line with an R\u{b2} of 0.976. \
                    A canal is level between its locks; the longest stretch of this one level to ten \
                    inches is 2,297 feet.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-in-allen-county-2026.yml", "**A canal is level between its locks, and what carries the name here falls the whole way.** Along the 6.1157 miles the file draws, the channel bottom descends 41.9 feet at 5.94 feet to the mile, and a straight line fits it with an R\u{b2} of 0.976. [verified] \u{2014} [the elevation surface](../../catalog/usgs-3dep-elevation.md) sampled every ten metres along the mapped line. The longest stretch of it level to within ten inches is 2,297 feet."),
        ],
        answers: &["cannot say how many miles of the canal lie in Allen County"],
        figures: &[
            Figure { label: "fall over the mapped canal, feet", value: 41.9, literal: "41.9" },
            Figure { label: "gradient, feet per mile", value: 5.94, literal: "5.94" },
        ],
    },
    Assertion {
        id: "a-field-ditch-in-the-canals-line",
        statement: "What lies in the canal's line north of the Deep Cut is a field ditch 26.2 feet \
                    across a metre above its bottom and 5.9 feet below the fields, with one berm \
                    surviving. The Deep Cut holds 36.1 feet of standing water and lies 17.1 feet \
                    below its fields.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-in-allen-county-2026.yml", "**What is in the canal's line north of Spencerville is a field ditch.** It is 26.2 feet across a metre above its bottom and lies 5.9 feet below the fields, with a berm surviving along one side; the Deep Cut holds 36.1 feet of standing water and lies 17.1 feet below its fields. [verified] \u{2014} the same source, in cross-section every ten metres. Whether the ditch was cut in the canal's own prism or beside it is a question this file has no field for. [inference]"),
        ],
        answers: &["cannot say how many miles of the canal lie in Allen County"],
        figures: &[
            Figure { label: "width of the ditch, feet", value: 26.2, literal: "26.2" },
            Figure { label: "depth below the fields, feet", value: 5.9, literal: "5.9" },
        ],
    },
    Assertion {
        id: "the-canal-got-its-name-four-years-after-the-boats",
        statement: "As built the Miami extension ran 114 miles, 5 feet deep, 36 feet wide at the \
                    bottom and 50 at the top. The name the corpus calls it by is younger than the \
                    work: an act of 14 March 1849 folded three canals into one Miami and Erie, four \
                    years after boats were running through Allen County.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-in-allen-county-2026.yml", "**As built it was 5 feet deep, 36 feet wide at the bottom and 50 at the top.** The Miami extension was completed in 1845 and ran 114 miles. [verified] \u{2014} [Miller, 1906](../../catalog/miller-allen-county-1906.md). The name is younger than the work: by an act of 14 March 1849 the Miami Canal, the Miami Extension Canal and the Wabash and Erie became one Miami and Erie Canal. [verified] \u{2014} the same book."),
        ],
        answers: &["cannot say how many miles of the canal lie in Allen County"],
        figures: &[
            Figure { label: "length of the Miami extension, miles", value: 114.0, literal: "114" },
        ],
    },
    Assertion {
        id: "the-first-boat-through-delphos-was-the-marshall",
        statement: "The first canalboat through Delphos was the Marshall, on 4 July 1845, and the \
                    first passenger packet came in 1846 carrying Governor-elect William Bebb, met by \
                    the town's businessmen at a lock a mile away with fresh horses. It is the only \
                    mention of a lock in this county in any of the three county histories.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-in-allen-county-2026.yml", "**The first boat through Delphos was the *Marshall*, on 4 July 1845.** The first passenger packet came in 1846 carrying Governor-elect William Bebb, and the businessmen of Delphos met it \"at a lock a mile away\" and put in fresh horses. [verified] \u{2014} [Rusler, 1921](../../catalog/rusler-allen-county-1921.md), which is the only mention of a lock in this county the corpus has found in any of its three histories."),
        ],
        answers: &["cannot say how many miles of the canal lie in Allen County"],
        figures: &[],
    },
    Assertion {
        id: "six-point-nine-nine-miles-and-five-more-nobody-draws",
        statement: "Six and 99 hundredths of a mile of this canal have been measured in Allen \
                    County, and at least 5.1 miles more are drawn by no file the corpus holds \
                    \u{2014} through Spencerville, between two mapped segments, and north from the \
                    last mapped vertex to the Putnam County line.",
        topic: "history",
        supports: &[
            support!("measure/miami-and-erie-canal-in-allen-county-2026.yml", "**This corpus cannot say how many miles of the canal lie in Allen County.** It has measured 6.99 \u{2014} the 6.1157 the water file draws plus the 4,617 feet of the Deep Cut that fall on the Allen side \u{2014} and it has located at least 5.1 miles more that no file it holds draws at all: 1.160 miles between the cut and the first mapped segment, through Spencerville; 1.084 miles between the second mapped segment and the third; and about 2.9 miles from the last mapped vertex north to the Putnam County line, which passes through [Delphos](../place/delphos.yml). [verified] \u{2014} the same files, measured and differenced here."),
        ],
        answers: &["cannot say how many miles of the canal lie in Allen County"],
        figures: &[
            Figure { label: "measured in Allen County, miles", value: 6.99, literal: "6.99" },
        ],
    },
    Assertion {
        id: "one-nomination-in-twenty-nine-is-readable",
        statement: "Twenty-eight of Allen County's twenty-nine National Register nominations have \
                    never been scanned. Every reference number returns a PDF and twenty-eight of \
                    them are a placeholder served with HTTP 200; the one real document is the \
                    Landmark's.",
        topic: "history",
        supports: &[
            support!("measure/allen-county-national-register.yml", "**Twenty-eight of the twenty-nine nominations have never been scanned, and the one that has is the Landmark.** Every reference number returns a PDF from the Park Service's gallery; for twenty-eight of them it is a one-page placeholder, byte-identical across listings, reading \"The PDF file for this National Register record has not yet been digitized\" \u{2014} served with HTTP 200. [verified] \u{2014} [the nomination documents](../../catalog/nrhp-nomination-documents.md), all twenty-nine requested and hashed. So the words behind this county's list are readable for one listing in twenty-nine, and it is the one that already had the most attached to it."),
        ],
        answers: &["The dataset carries `STATUS: Listed` on all twenty-nine and would simply omit a delisted property, so it cannot answer its own question"],
        figures: &[],
    },
    Assertion {
        id: "six-per-cent-of-the-county-is-floodplain",
        statement: "Six per cent of Allen County is in the special flood hazard area \u{2014} 15,622 \
                    acres, which is 6.06 per cent of its land.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-hazard-2026.yml", "**Six per cent of Allen County is in the special flood hazard area \u{2014} 15,622 acres.** That is 6.06 per cent of the county's 402.545 square miles of land, and 6.00 per cent of the polygon the map is drawn on, which includes its water. [verified] \u{2014} [the National Flood Hazard Layer](../../catalog/fema-nfhl.md), 1,625 polygons for `DFIRM_ID` 39003C, dissolved and clipped to [the county polygon](../../catalog/census-tiger-roads.md) in EPSG:26916, against [the land area](allen-county-land-area-2020.yml)."),
        ],
        answers: &["cannot say how many people live in the floodplain"],
        figures: &[
            Figure { label: "special flood hazard area, acres", value: 15622.0, literal: "15,622" },
            Figure { label: "per cent of the county's land", value: 6.06, literal: "6.06" },
        ],
    },
    Assertion {
        id: "the-five-hundred-year-band-is-a-sixteenth",
        statement: "The five-hundred-year flood band in Allen County is 943 acres against the \
                    hundred-year zone's 15,622 \u{2014} a sixteenth of it, which is what a county \
                    mapped without a second modelled flood beside the first looks like.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-hazard-2026.yml", "**The five-hundred-year band is a sixteenth of the hundred-year one.** 943 acres against 15,622. [verified] \u{2014} the same source, its `ZONE_SUBTY` field. A county whose 0.2 per cent zone is that thin has had most of its ground mapped without a second modelled flood beside the first. [inference]"),
        ],
        answers: &["cannot say how many people live in the floodplain"],
        figures: &[
            Figure { label: "0.2 per cent annual chance zone, acres", value: 943.0, literal: "943" },
        ],
    },
    Assertion {
        id: "one-acre-in-seven-has-no-elevation",
        statement: "One acre in seven of Allen County's floodplain is zone A \u{2014} special flood \
                    hazard with no base flood elevation determined. That is 2,293 of the 15,622 \
                    acres, and zone A is what a stream gets when nobody has run a hydraulic study on \
                    it.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-hazard-2026.yml", "**One acre in seven of the floodplain has no base flood elevation on the map.** Zone A \u{2014} special flood hazard with no elevation determined \u{2014} is 2,293 of the 15,622 acres, and the rest is AE or AO. [verified] \u{2014} the same source. Zone A is the approximate method, which is what a stream gets when nobody has run a hydraulic study on it."),
        ],
        answers: &["cannot say how many people live in the floodplain"],
        figures: &[
            Figure { label: "zone A, acres", value: 2293.0, literal: "2,293" },
        ],
    },
    Assertion {
        id: "the-flood-map-is-three-maps",
        statement: "Allen County's flood map is three maps: of its 48 printed panels, 37 took effect \
                    on 20 June 2024, ten on 2 May 2013 and one on 4 May 2015.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-hazard-2026.yml", "**The map is three maps.** Forty-eight printed panels cover this county: 37 took effect on 20 June 2024, ten on 2 May 2013 and one on 4 May 2015, and five of the 48 were never printed. [verified] \u{2014} the same source, layer 3."),
        ],
        answers: &["cannot say how many people live in the floodplain"],
        figures: &[
            Figure { label: "panels effective June 2024", value: 37.0, literal: "37" },
            Figure { label: "printed panels", value: 48.0, literal: "48" },
        ],
    },
    Assertion {
        id: "wet-ground-and-flood-zone-are-not-the-same-ground",
        statement: "Across Allen County's twelve townships and its city, the share of ground that is \
                    hydric soil and the share inside the flood map's special hazard area correlate \
                    at a Spearman 0.099. Marion is the wettest township in the county and is 5.73 \
                    per cent floodplain.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-hazard-2026.yml", "**Wet ground and flood zone are close to unrelated here.** Across the twelve townships and the city, the share of ground that is hydric soil and the share that is mapped floodplain correlate at a Spearman 0.099. Marion is the wettest township in the county at 58.0 per cent hydric and is 5.73 per cent floodplain; Monroe is 44.3 and 3.34. [inference] \u{2014} computed here against [the soils](allen-county-soils-2026.yml). A flood map traces channels; a soil map records where water sits."),
            support!("measure/allen-county-soils-2026.yml", "**35.3 per cent of the county is hydric soil, which is ground that formed under standing water.** 91,953 acres. [verified] \u{2014} the same file, `hydricrating`. Read against the drainage column it is almost exactly the very-poorly-drained share, 34.5 per cent, which is what the two ratings ought to do and is worth checking rather than assuming."),
        ],
        answers: &["cannot say how many people live in the floodplain", "does not assert that the hydric acres are the Great Black Swamp"],
        figures: &[
            Figure { label: "Spearman correlation, hydric against floodplain", value: 0.099, literal: "0.099" },
            Figure { label: "Marion, per cent floodplain", value: 5.73, literal: "5.73" },
        ],
    },
    Assertion {
        id: "four-villages-have-no-floodplain-at-all",
        statement: "Four of Allen County's nine incorporated places have no mapped floodplain at all \
                    \u{2014} Beaverdam, Harrod, Spencerville and Cairo \u{2014} and the village with \
                    the most of it is not the city on the biggest river.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-hazard-2026.yml", "**Four of the county's nine incorporated places have no mapped floodplain at all**, and the village with the most of it is not the city on the biggest river. [verified] \u{2014} the same source against [the place file](../../catalog/census-tiger-roads.md), each place clipped to this county first because Bluffton and Delphos are not wholly inside it. Gomer, Westminster and Fort Shawnee are census designated places and not corporations; the other nine are the county's villages and its city."),
        ],
        answers: &["cannot say how many people live in the floodplain"],
        figures: &[],
    },
    Assertion {
        id: "gomer-is-a-third-floodplain",
        statement: "Nearly a third of Gomer is in the floodplain \u{2014} 448.4 of its 1,469 acres, \
                    30.53 per cent \u{2014} which is more than any incorporated place in Allen \
                    County, against 17.43 per cent for Bluffton and 4.27 for Lima.",
        topic: "geography",
        supports: &[
            support!("place/gomer.yml", "**Nearly a third of it is in the floodplain, which is more than any incorporated place in the county.** 448.4 of its 1,469 acres \u{2014} 30.53 per cent \u{2014} lie in the special flood hazard area, against 17.43 per cent for Bluffton and 4.27 for Lima. [verified] \u{2014} [the National Flood Hazard Layer](../../catalog/fema-nfhl.md); see [the mapped floodplain](../measure/allen-county-flood-hazard-2026.yml). It sits in the county's most flood-mapped township, [Sugar Creek](sugar-creek-township.yml) at 17.04 per cent, and it is the only settlement in it."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "Gomer, per cent floodplain", value: 30.53, literal: "30.53" },
            Figure { label: "Bluffton, per cent floodplain", value: 17.43, literal: "17.43" },
        ],
    },
    Assertion {
        id: "how-many-people-live-in-the-floodplain",
        statement: "Of Allen County's 3,552 census blocks, 849 touch the special flood hazard area \
                    and 94 lie wholly inside it. Those 94 hold 235 people and the 849 hold 23,721, \
                    so the corpus publishes a bracket rather than an estimate.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-hazard-2026.yml", "**This corpus cannot say how many people live in the floodplain.** Of the county's 3,552 census blocks, 849 touch the special flood hazard area and 94 lie wholly inside it. Those 94 hold 235 people; the 849 hold 23,721; weighting each block by the share of its area inside gives 4,448. [verified] \u{2014} [the 2020 block file](../../catalog/census-tiger-roads.md), `POP20` against the dissolved hazard area. A bracket from 0.23 per cent of the county to 23.21 per cent is not an estimate, and the area weight in the middle is the weight this corpus has already found wrong for people; see [weight a crosswalk by what it carries](../../decisions/weight-a-crosswalk-by-what-it-carries.yml). The housing figures behave identically: 96, 1,945 and 10,475 of 44,563."),
        ],
        answers: &["cannot say how many people live in the floodplain"],
        figures: &[
            Figure { label: "people in blocks wholly inside", value: 235.0, literal: "235" },
            Figure { label: "people in blocks touching", value: 23721.0, literal: "23,721" },
        ],
    },
    Assertion {
        id: "more-rain-than-river",
        statement: "More of Allen County's flood insurance claims are coded to accumulation of \
                    rainfall or snowmelt than to a stream, river or lake leaving its channel \
                    \u{2014} 109 against 103.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**More of it comes from rain than from rivers.** The file codes the cause of every loss, and 109 are accumulation of rainfall or snowmelt against 103 from a stream, river or lake leaving its channel. [verified] \u{2014} the same file, its `causeOfDamage` field against [the published code list](../../catalog/openfema-nfip.md). In a county three-quarters of which is poorly drained, the water that gets into a building is as often water that had nowhere to go. [inference] \u{2014} see [the soils](allen-county-soils-2026.yml)."),
        ],
        answers: &["cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[
            Figure { label: "rainfall or snowmelt", value: 109.0, literal: "109" },
            Figure { label: "stream, river or lake overflow", value: 103.0, literal: "103" },
        ],
    },
    Assertion {
        id: "tidal-water-five-hundred-miles-from-the-sea",
        statement: "Twenty-seven flood claims in Allen County are coded tidal water overflow, in a \
                    county five hundred miles from tidewater. Twenty-four of them are dated 1978 to \
                    1986 and carry no rated zone, and they total $76,630.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**Twenty-seven claims in this county are coded tidal water overflow.** Allen County is five hundred miles from tidewater. Twenty-four of the twenty-seven are dated 1978 to 1986 and carry no rated zone at all; they total $76,630. [verified] \u{2014} the same file. It is a coding artefact of the programme's first decade and not a fact about water. [inference]"),
        ],
        answers: &["cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[
            Figure { label: "paid on the tidal-coded claims, dollars", value: 76630.0, literal: "76,630" },
        ],
    },
    Assertion {
        id: "a-village-of-four-thousand-holds-a-third-of-the-money",
        statement: "Bluffton holds 27 per cent of Allen County's flood insurance claims and 31 per \
                    cent of the money paid on them \u{2014} 70 claims against Lima's 22, in a place \
                    a ninth of Lima's size \u{2014} and the largest single payment in the county's \
                    record, $394,032 in 2007, was paid there.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**A village of four thousand holds 27 per cent of the county's flood claims and 31 per cent of its money.** Bluffton's 70 claims against Lima's 22, in a place a ninth of Lima's size, and the largest single payment in the county's record \u{2014} $394,032 in 2007 \u{2014} was paid there. [verified] \u{2014} the same file, summed by community. The gap is historical rather than current: over 2009 to 2026, per hundred policy terms, Bluffton's postal area produced 4.49 claims and Lima's three produced 3.99. [inference] \u{2014} the same file against [the policy file](../../catalog/openfema-nfip.md), cut to the years both cover."),
        ],
        answers: &["cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[
            Figure { label: "Bluffton claims", value: 70.0, literal: "70" },
            Figure { label: "Lima claims", value: 22.0, literal: "22" },
            Figure { label: "largest single payment, dollars", value: 394032.0, literal: "394,032" },
        ],
    },
    Assertion {
        id: "june-2015-and-nobody-declared-it",
        statement: "June 2015 was the costliest month in Allen County's flood insurance record \
                    \u{2014} 45 claims, 32 of them outside any municipality, and 30.3 per cent of \
                    everything ever paid here \u{2014} and no federal disaster was declared for it.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**June 2015 was the costliest month in the record and no federal disaster was declared for it.** Forty-five claims fell in that one month, 32 of them outside any municipality, and the year's $1,243,724 is 30.3 per cent of everything ever paid here. [verified] \u{2014} the same file against [the declarations](allen-county-disaster-declarations-1965-2020.yml), which has no 2015 row."),
        ],
        answers: &["cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[
            Figure { label: "share of all money ever paid, per cent", value: 30.3, literal: "30.3" },
            Figure { label: "claims outside any municipality", value: 32.0, literal: "32" },
        ],
    },
    Assertion {
        id: "ten-claim-years-and-one-declaration",
        statement: "Of the ten years that produced the most flood insurance claims in Allen County, \
                    one \u{2014} 2007 \u{2014} carries a federal disaster declaration. The \
                    declaration of August 2012 produced no flood claim in the county at all.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**Of the ten years with the most claims, one was a declared disaster.** 2015, 2011, 2019, 1981, 2017, 1997, 2008, 1979 and 1992 were not; 2007 was. Going the other way, the declaration of August 2012 produced no flood insurance claim in this county at all. [verified] \u{2014} the same sources. The two records are counting different things \u{2014} a declaration is public damage from any peril, a claim is insured flood damage to a building \u{2014} and the disagreement is what that difference looks like from inside one county. [inference]"),
            support!("measure/allen-county-disaster-declarations-1965-2020.yml", "**Ten declarations in fifty-five years, for eight distinct incidents.** [verified] \u{2014} [OpenFEMA](../../catalog/fema-disaster-declarations.md), disaster declarations summaries, Ohio county 003."),
        ],
        answers: &["cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[],
    },
    Assertion {
        id: "twenty-seven-buildings-and-one-buyout",
        statement: "Twenty-seven buildings in Allen County have been paid on more than once by the \
                    flood insurance programme and twenty-six of them have not been mitigated; one \
                    has been paid on thirteen times. Against them the county's whole federal buyout \
                    record is one house bought and one refused.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**Twenty-seven buildings here have been paid on more than once, and twenty-six have not been mitigated.** Twelve are in Lima, nine in Bluffton, four in Delphos, one in Elida and one in Spencerville; three have four losses, three have five, one has seven and one has thirteen. Eleven of the twenty-seven are in zone X. Twenty are no longer insured. [verified] \u{2014} [the repeat-loss file](../../catalog/openfema-nfip.md). Against them the county has one completed buyout and one refused; see [the mitigation record](allen-county-hazard-mitigation-2003-2026.yml)."),
            support!("measure/allen-county-hazard-mitigation-2003-2026.yml", "**So the county's whole federal flood-buyout record is two houses, one bought and one not.** [inference] \u{2014} computed here. Neither is located: `projectCounties` reads ALLEN and there is no coordinate, address or watershed in the file. [verified] \u{2014} same source."),
        ],
        answers: &["cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[],
    },
    Assertion {
        id: "half-the-claims-were-rated-outside-the-map",
        statement: "Half of Allen County's flood insurance claims were rated outside the mapped \
                    floodplain: of the 210 that carry a rated zone, 104 are in the special flood \
                    hazard area and 106 are not.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**Half the claims were rated outside the mapped floodplain.** Of the 210 that carry a rated zone, 104 are A, AE or AO and 106 are B, C, D or X. [verified] \u{2014} the same file. Per hundred policy terms over 2009 to 2026, in the eight postal areas wholly inside this county, a policy rated outside produced 4.27 claims and one rated inside produced 2.76. [inference] \u{2014} computed here."),
        ],
        answers: &["cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[
            Figure { label: "rated inside the hazard area", value: 104.0, literal: "104" },
            Figure { label: "rated outside it", value: 106.0, literal: "106" },
        ],
    },
    Assertion {
        id: "the-ratio-is-about-who-had-to-buy",
        statement: "A flood policy rated outside Allen County's mapped floodplain produced 4.27 \
                    claims per hundred policy terms and one rated inside produced 2.76 \u{2014} and \
                    the corpus declines to read that as a statement about the map, because cover \
                    inside is compulsory and outside it is not.",
        topic: "geography",
        supports: &[
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**This corpus cannot say whether Allen County's flood map is drawn in the wrong place.** Cover is compulsory inside the special flood hazard area for anyone with a federally backed mortgage and voluntary outside it, so the two denominators above were recruited by different rules and the ratio between the two rates is about the recruitment at least as much as the water. [verified] \u{2014} [the programme's own purchase requirement](../../catalog/openfema-nfip.md); see [a compulsory denominator is not a voluntary one](../../decisions/a-compulsory-denominator-is-not-a-voluntary-one.yml)."),
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**Half the claims were rated outside the mapped floodplain.** Of the 210 that carry a rated zone, 104 are A, AE or AO and 106 are B, C, D or X. [verified] \u{2014} the same file. Per hundred policy terms over 2009 to 2026, in the eight postal areas wholly inside this county, a policy rated outside produced 4.27 claims and one rated inside produced 2.76. [inference] \u{2014} computed here."),
        ],
        answers: &["cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[
            Figure { label: "claims per 100 policy terms, outside", value: 4.27, literal: "4.27" },
            Figure { label: "claims per 100 policy terms, inside", value: 2.76, literal: "2.76" },
        ],
    },
    Assertion {
        id: "the-jail-rate-more-than-tripled",
        statement: "Allen County held 80 people in its jail in 1970 and 284 in 2019, on a \
                    working-age population that fell over the same years. The rate went from 120 to \
                    443 for every 100,000 residents aged 15 to 64.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-jail-1970-2023.yml", "**The county held 80 people in its jail in 1970 and 284 in 2019, on a working-age population that fell from 66,664 to 64,117.** [verified] \u{2014} [Vera's Incarceration Trends](../../catalog/vera-incarceration-trends.md), county 39003. The rate went from 120 to 443 for every 100,000 residents aged 15 to 64, and stood at 300 in 2023; see [the Justice Center](../site/allen-county-justice-center.yml)."),
        ],
        answers: &["cannot say where the county's jail population was counted in the 2020 census"],
        figures: &[
            Figure { label: "jail population, 1970", value: 80.0, literal: "80" },
            Figure { label: "jail population, 2019", value: 284.0, literal: "284" },
            Figure { label: "rate per 100,000 aged 15-64, 2019", value: 443.0, literal: "443" },
        ],
    },
    Assertion {
        id: "twenty-one-years-over-capacity",
        statement: "Allen County's jail has been over its rated capacity in 21 of the 29 years that \
                    carry both figures, and it was 139 per cent full in 2019.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-jail-1970-2023.yml", "**The jail has been over its rated capacity in 21 of the 29 years that carry both figures**, and it was 139 per cent full in 2019, its worst year. [verified] \u{2014} the same source, computed here. It was under capacity in the four sampled years to 1988 and in the four since 2020, and over it in every year between that carries a figure."),
        ],
        answers: &["cannot say where the county's jail population was counted in the 2020 census"],
        figures: &[
            Figure { label: "years over capacity", value: 21.0, literal: "21" },
            Figure { label: "years carrying both figures", value: 29.0, literal: "29" },
            Figure { label: "occupancy in 2019, per cent", value: 139.0, literal: "139" },
        ],
    },
    Assertion {
        id: "three-quarters-of-the-jail-is-pretrial",
        statement: "Three quarters of the people in Allen County's jail have not been convicted of \
                    anything. The unsentenced share was 46.2 per cent in 1970 and 40.2 in 2010; it \
                    is 76.9 in 2023.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-jail-1970-2023.yml", "**Three quarters of the people in it have not been convicted of anything.** The unsentenced share was 46.2 per cent in 1970 and 40.2 in 2010; it is 52.8 in 2019, 68.3 in 2022 and 76.9 in 2023. [verified] \u{2014} the same source. The pretrial and sentenced counts in this file are apportioned to sum to the average daily population rather than measured beside it, so the share is a share of one number and not a ratio of two; see [the codebook's own warning](../../catalog/vera-incarceration-trends.md)."),
        ],
        answers: &["cannot say where the county's jail population was counted in the 2020 census"],
        figures: &[
            Figure { label: "unsentenced share, 1970", value: 46.2, literal: "46.2" },
            Figure { label: "unsentenced share, 2023", value: 76.9, literal: "76.9" },
        ],
    },
    Assertion {
        id: "this-county-jails-above-the-state",
        statement: "Allen County jails at about 1.6 times Ohio's rate \u{2014} 443 against 276 in \
                    2019 and 300 against 219 in 2023 \u{2014} and the gap has been open since at \
                    least 1993.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-jail-1970-2023.yml", "**This county jails at 1.6 times the state's rate.** 443 against Ohio's 276 in 2019, 300 against 219 in 2023, and 316 against 164 in 1993. [verified] \u{2014} the same dataset's state file. The gap has been open since at least 1993 and has never closed. [inference]"),
        ],
        answers: &["cannot say where the county's jail population was counted in the 2020 census"],
        figures: &[
            Figure { label: "Allen County, 2019", value: 443.0, literal: "443" },
            Figure { label: "Ohio, 2019", value: 276.0, literal: "276" },
        ],
    },
    Assertion {
        id: "an-eighth-of-the-county-and-a-third-of-the-jail",
        statement: "Black residents are 12.3 per cent of Allen County and 36.6 per cent of its jail. \
                    Per 100,000 aged 15 to 64 the Black rate was 1,190 in 2019 and the white rate \
                    268.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-jail-1970-2023.yml", "**Black residents are an eighth of the county and between a third and a half of its jail.** 36.6 per cent of the jail in 2019 and 37.6 in 2023, against 12.3 per cent of the county's people in 2020. [verified] \u{2014} the same source against [the county by race](allen-county-population-by-race-1970-2020.yml). Per 100,000 aged 15 to 64 the Black rate was 1,190 in 2019 and the white rate 268 \u{2014} 4.4 times \u{2014} and the ratio was 7.4 in 1999 and 6.6 in 1993. [verified] \u{2014} the same source. It has narrowed while both rates rose. [inference]"),
        ],
        answers: &["cannot say where the county's jail population was counted in the 2020 census"],
        figures: &[
            Figure { label: "Black share of the county, per cent", value: 12.3, literal: "12.3" },
            Figure { label: "Black share of the jail, per cent", value: 36.6, literal: "36.6" },
            Figure { label: "Black jail rate per 100,000", value: 1190.0, literal: "1,190" },
        ],
    },
    Assertion {
        id: "the-jail-series-is-a-sample",
        statement: "Allen County's jail figures exist for 1970, 1978, 1983, 1988 and 1993 and then \
                    for every year from 1999, because the federal Census of Jails runs every five to \
                    eight years and the annual survey between them reaches about a third of jails. \
                    The blank years are years nobody asked.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-jail-1970-2023.yml", "**The series is a sample and its gaps are the survey's.** Figures exist for 1970, 1978, 1983, 1988 and 1993 and then for every year from 1999, because the federal Census of Jails runs every five to eight years and the annual survey between them reaches about a third of jails. [verified] \u{2014} the same source's codebook. The blank years are years nobody asked, not years without a jail."),
        ],
        answers: &["cannot say where the county's jail population was counted in the 2020 census"],
        figures: &[],
    },
    Assertion {
        id: "two-rows-that-pass-every-range-check",
        statement: "Two of Allen County's jail rows are unusable and neither is marked: the year \
                    2000 records 23 people, none of them men, between 224 in 1999 and 229 in 2001, \
                    and 2020 records 3,614 admissions against 1,268 the year before in the twelve \
                    months the population fell by half.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-jail-1970-2023.yml", "**Two rows are unusable and neither is marked.** The year 2000 records 23 people, 0 of them men and 23 women, between 224 in 1999 and 229 in 2001; the year 2020 records 3,614 admissions against 1,268 the year before and 585 the year after, in the twelve months the population fell by half. Both are excluded from every figure above. [verified] \u{2014} the same source, read against its neighbours."),
        ],
        answers: &["cannot say where the county's jail population was counted in the 2020 census"],
        figures: &[
            Figure { label: "jail population recorded for 2000", value: 23.0, literal: "23" },
            Figure { label: "admissions recorded for 2020", value: 3614.0, literal: "3,614" },
        ],
    },
    Assertion {
        id: "where-was-the-jail-counted",
        statement: "The 2020 census put 1,513 people in adult correctional facilities in Allen \
                    County and all 1,513 are the two blocks that are the two state prisons. The \
                    county jail's average daily population that year was 144, and the corpus \
                    declines to say where they were counted.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-jail-1970-2023.yml", "**This corpus cannot say where the county's jail population was counted in the 2020 census.** That census put 1,513 people in adult correctional facilities in Allen County, and all 1,513 are accounted for by the two blocks that are the two state prisons; the jail's average daily population that year was 144. [verified] \u{2014} [the group quarters](allen-county-group-quarters-2020.yml) against the same source."),
        ],
        answers: &["cannot say where the county's jail population was counted in the 2020 census"],
        figures: &[
            Figure { label: "people in adult correctional facilities, 2020 census", value: 1513.0, literal: "1,513" },
            Figure { label: "jail average daily population, 2020", value: 144.0, literal: "144" },
        ],
    },
    Assertion {
        id: "from-two-hundred-and-twenty-four-to-nine-hundred-and-seventy-eight",
        statement: "Allen County's courts had 156 people in Ohio's prisons in 1983 and 627 in 2019 \
                    \u{2014} a rate rising from 224 to 978 per 100,000 residents aged 15 to 64, on a \
                    working-age population that fell by 5,579 over the same years.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-in-state-prison-1983-2019.yml", "**The county's courts had 156 people in Ohio's prisons in 1983 and 627 in 2019.** [verified] \u{2014} [Vera's Incarceration Trends](../../catalog/vera-incarceration-trends.md), county 39003, counted by county of commitment and not by where anyone is held. The rate went from 224 to 978 for every 100,000 residents aged 15 to 64, on a working-age population that fell by 5,579 over the same years."),
        ],
        answers: &["cannot say how long anyone sentenced from this county spends in prison"],
        figures: &[
            Figure { label: "in prison from the county, 1983", value: 156.0, literal: "156" },
            Figure { label: "in prison from the county, 2019", value: 627.0, literal: "627" },
            Figure { label: "rate, 2019", value: 978.0, literal: "978" },
        ],
    },
    Assertion {
        id: "ohio-stopped-and-this-county-did-not",
        statement: "Allen County was below Ohio's imprisonment rate when the series opens, crossed \
                    it in 1988 and has been above it every year since. Ohio's rate has sat between \
                    590 and 682 in every year since 1995; this county's went from 737 to 978.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-in-state-prison-1983-2019.yml", "**It was below the state's rate when the series opens and half again above it when it closes.** Allen County crossed Ohio's rate in 1988 and has been above it every year since. [verified] \u{2014} the same dataset's state file. Ohio's rate has moved between 590 and 682 in every year since 1995 and was 662 in 2019; this county's went from 737 to 978 over the same twenty-five years. [inference] \u{2014} computed here. Whatever stopped the state's growth did not stop this county's."),
        ],
        answers: &["cannot say how long anyone sentenced from this county spends in prison"],
        figures: &[
            Figure { label: "Allen County, 2019", value: 978.0, literal: "978" },
            Figure { label: "Ohio, 2019", value: 662.0, literal: "662" },
        ],
    },
    Assertion {
        id: "fewer-sent-and-more-inside",
        statement: "Allen County's admissions to state prison peaked at 282 in 1992 and were 171 in \
                    2019, down 39.4 per cent, while the number of its people in prison rose 47.9 per \
                    cent. Fewer entering and more present is possible only if people are staying \
                    longer.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-in-state-prison-1983-2019.yml", "**Fewer people are being sent and more are inside.** Admissions peaked at 282 in 1992 and were 171 in 2019 \u{2014} down 39.4 per cent \u{2014} while the population sentenced from here rose 47.9 per cent. [verified] \u{2014} the same source. Fewer entering and more present is possible only if people are staying longer, which follows from conservation and needs no model. [inference]"),
        ],
        answers: &["cannot say how long anyone sentenced from this county spends in prison"],
        figures: &[
            Figure { label: "prison admissions, 1992", value: 282.0, literal: "282" },
            Figure { label: "prison admissions, 2019", value: 171.0, literal: "171" },
            Figure { label: "fall in admissions, per cent", value: 39.4, literal: "39.4" },
        ],
    },
    Assertion {
        id: "the-ratio-is-not-a-length-of-stay",
        statement: "Allen County's prison population divided by that year's admissions is 1.50 in \
                    1992 and 3.67 in 2019. The corpus publishes that as a ratio of a stock to a flow \
                    and not as an average length of stay, because the reading holds only where the \
                    population is stationary and this one grew by half.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-in-state-prison-1983-2019.yml", "**The ratio of the two, and what it is not.** 424 over 282 is 1.50 and 627 over 171 is 3.67. [inference] \u{2014} the same source, divided here. Those are ratios of a population to a year's admissions and this corpus publishes them as that and not as an average length of stay, because that reading holds only where the population is stationary and this one grew by half; see [a stock divided by a flow is not a length of stay](../../decisions/a-stock-divided-by-a-flow-is-not-a-length-of-stay.yml)."),
        ],
        answers: &["cannot say how long anyone sentenced from this county spends in prison"],
        figures: &[
            Figure { label: "population over admissions, 1992", value: 1.5, literal: "1.50" },
            Figure { label: "population over admissions, 2019", value: 3.67, literal: "3.67" },
        ],
    },
    Assertion {
        id: "eight-times-as-likely",
        statement: "A Black resident of Allen County was eight times as likely to be in an Ohio \
                    prison as a white one in 2019 \u{2014} 4,028 per 100,000 aged 15 to 64 against \
                    486 \u{2014} and eleven times as likely in 2009. Black people are 12.3 per cent \
                    of the county and 56.1 per cent of the people it sends to prison.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-in-state-prison-1983-2019.yml", "**A Black resident of this county was eight times as likely to be in prison as a white one.** 4,028 per 100,000 aged 15 to 64 against 486 in 2019, and 3,758 against 332 \u{2014} 11.3 times \u{2014} in 2009. [verified] \u{2014} the same source. **Black people are 12.3 per cent of Allen County and 56.1 per cent of the people it has sent to prison.** [verified] \u{2014} the same source against [the county by race](allen-county-population-by-race-1970-2020.yml). The ratio has narrowed because the white rate rose by 46 per cent over the decade while the Black rate rose by 7. [inference] \u{2014} computed here."),
        ],
        answers: &["cannot say how long anyone sentenced from this county spends in prison"],
        figures: &[
            Figure { label: "Black rate per 100,000", value: 4028.0, literal: "4,028" },
            Figure { label: "white rate per 100,000", value: 486.0, literal: "486" },
            Figure { label: "Black share of the county's prison population, per cent", value: 56.1, literal: "56.1" },
        ],
    },
    Assertion {
        id: "sent-from-here-and-held-here",
        statement: "627 people were in prison from Allen County in 2019 and 1,513 were held in adult \
                    correctional facilities inside it in 2020. A county that sends six hundred and \
                    holds fifteen hundred is doing two different things, and only one of them is a \
                    fact about its courts.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-in-state-prison-1983-2019.yml", "**This number and the county's prisons are about different people.** 627 people were in prison from Allen County in 2019; 1,513 people were held in adult correctional facilities inside Allen County in the 2020 census, and every one of them was in the two blocks that are the two state prisons. [verified] \u{2014} [the group quarters](allen-county-group-quarters-2020.yml) against the same source. A county that sends six hundred and holds fifteen hundred is doing two different things, and only one of them is a fact about its courts; see [located here is not of here](../../decisions/located-here-is-not-of-here.yml)."),
        ],
        answers: &["cannot say how long anyone sentenced from this county spends in prison"],
        figures: &[
            Figure { label: "sent from the county", value: 627.0, literal: "627" },
            Figure { label: "held inside the county", value: 1513.0, literal: "1,513" },
        ],
    },
    Assertion {
        id: "the-criminal-caseload-did-not-move",
        statement: "Allen County's general division filed 416 criminal cases in 2007 and 415 in \
                    2017; the county sent 188 people to state prison in the first year and 173 in \
                    the second; and it had 533 of its people in prison at the start and 627 by 2019.",
        topic: "government",
        supports: &[
            support!("measure/allen-county-court-caseloads-2007-2017.yml", "**The flat criminal caseload rules out the obvious explanation for a rising prison population.** The general division's criminal filings were 416 in 2007 and 415 in 2017; the county's admissions to state prison were 188 and 173 over the same two years, and the number of its people in prison went from 533 to 566 and then to 627 by 2019. [verified] \u{2014} [Vera's Incarceration Trends](../../catalog/vera-incarceration-trends.md) against this table; see [the prison figures](allen-county-in-state-prison-1983-2019.yml). A court filing the same number of criminal cases and sending the same share of them away, with more of its people inside at the end, is a county where people are staying longer and not one where more is happening. [inference]"),
        ],
        answers: &["cannot say what has happened in these courts since 2017"],
        figures: &[
            Figure { label: "criminal filings, 2007", value: 416.0, literal: "416" },
            Figure { label: "criminal filings, 2017", value: 415.0, literal: "415" },
            Figure { label: "prison admissions, 2017", value: 173.0, literal: "173" },
        ],
    },
    Assertion {
        id: "the-census-lost-a-hundred-and-forty-people",
        statement: "Allen County's Justice Center has a rated capacity of 216 and averaged 144 \
                    people a day in 2020 \u{2014} the year the census recorded nobody in the \
                    building at all.",
        topic: "government",
        supports: &[
            support!("site/allen-county-justice-center.yml", "**It has a size now, and somebody was in it.** Its rated capacity is 216 and has been 204 or more since 1999; its average daily population in 2020 \u{2014} the year of that census \u{2014} was 144, and it has run between 176 and 284 in every other year since 1999. [verified] \u{2014} [Vera's Incarceration Trends](../../catalog/vera-incarceration-trends.md); see [the jail](../measure/allen-county-jail-1970-2023.yml). So the census's zero is not a building standing empty, and the open question below narrows to how the enumeration lost about a hundred and forty people. [inference]"),
        ],
        answers: &["The corpus does not know whether the census missed them, whether disclosure avoidance moved them, or whether the landmark and the working jail are no longer the same building"],
        figures: &[
            Figure { label: "rated capacity", value: 216.0, literal: "216" },
            Figure { label: "average daily population, 2020", value: 144.0, literal: "144" },
        ],
    },
    Assertion {
        id: "the-largest-kind-of-empty-is-neither",
        statement: "The largest class of empty house in Allen County is the one that is neither for \
                    sale nor for rent. 1,312 of the county's 3,628 vacant units are in it, against \
                    1,265 offered to a tenant and 402 offered to a buyer.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**The largest kind of empty house in this county is the kind that is not for sale and not for rent.** 1,312 units, more than the 1,265 offered to a tenant and more than three times the 402 offered to a buyer. [verified] \u{2014} the same table; the shares are computed here."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[
            Figure { label: "other vacant", value: 1312.0, literal: "1,312" },
            Figure { label: "for rent", value: 1265.0, literal: "1,265" },
            Figure { label: "for sale only", value: 402.0, literal: "402" },
        ],
    },
    Assertion {
        id: "four-hundred-and-two-houses-for-sale",
        statement: "On census day 402 houses in Allen County were for sale - nine tenths of one per \
                    cent of its 44,563 housing units.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**On any given day 402 houses in Allen County were for sale.** Nine tenths of one per cent of the county's 44,563 units, against 1,265 for rent. [verified] \u{2014} the same table, against [the housing units](allen-county-housing-units-2020.yml). A market that thin is what the price series has been describing from the other end. [inference] \u{2014} see [the price index](allen-county-house-prices-1975-2025.yml)."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[
            Figure { label: "houses for sale", value: 402.0, literal: "402" },
            Figure { label: "housing units", value: 44563.0, literal: "44,563" },
        ],
    },
    Assertion {
        id: "the-vacancy-rates-omit-the-largest-class",
        statement: "The two vacancy rates the census publishes exclude seasonal, migrant-worker and \
                    other-vacant units from both their numerators and their denominators, so 1,568 \
                    empty housing units in Allen County - 3.5 per cent of the stock - appear in no \
                    published vacancy rate at all.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**The two vacancy rates the Bureau publishes leave the largest class out of both of them.** The homeowner rate is for-sale-only over owner-occupied plus for-sale plus sold-not-occupied; the rental rate is for-rent over renter-occupied plus for-rent plus rented-not-occupied. Seasonal units, migrant-worker units and other-vacant units appear on neither side of either line. [verified] \u{2014} [the same appendix](../../catalog/census-2020-dhc.md), B-21."),
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**So 1,568 of the county's housing units \u{2014} 3.5 per cent of the stock \u{2014} are empty and invisible to every vacancy rate the census publishes.** [inference] \u{2014} the Bureau's own three formulas, applied here to its own eight rows. Both rates fell over the decade and the residual fell faster, which is a fact about the county the rates do not carry."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[
            Figure { label: "units outside every rate", value: 1568.0, literal: "1,568" },
            Figure { label: "share of the stock, per cent", value: 3.5, literal: "3.5" },
        ],
    },
    Assertion {
        id: "the-county-ended-the-decade-with-fewer-houses",
        statement: "Allen County had 44,999 housing units in 2010 and 44,563 in 2020, a fall of 436, \
                    while its occupied units rose from 40,619 to 40,935 and its empty ones fell by \
                    752.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**The county ended the decade with 752 fewer empty houses and 436 fewer houses.** 44,999 units in 2010 and 44,563 in 2020, while occupied units rose from 40,619 to 40,935. [verified] \u{2014} the same file and its 2010 predecessor, table H3 in each. The stock did not merely stop growing; it fell, and the fall is larger than the rise in occupancy."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[
            Figure { label: "housing units, 2010", value: 44999.0, literal: "44,999" },
            Figure { label: "housing units, 2020", value: 44563.0, literal: "44,563" },
            Figure { label: "fewer empty houses", value: 752.0, literal: "752" },
        ],
    },
    Assertion {
        id: "it-built-less-than-it-lost",
        statement: "1,025 dwellings were authorised in Allen County between the two censuses and the \
                    county finished with 436 fewer housing units, which puts the decade's removals \
                    at about fifteen hundred - some 146 a year.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**1,025 dwellings were authorised in the same ten years, so something took away about fifteen hundred.** 587 of them houses and 252 of them in buildings of five units or more. [verified] \u{2014} [the Building Permits Survey](../../catalog/census-building-permits.md), 2010 through 2019 county files, which sum to the same 1,025 at place grain. If every permit became a unit \u{2014} an upper bound, since a permit is not a house and some are never built \u{2014} then 1,461 units left this county's housing stock in a decade, about 146 a year. [inference] \u{2014} computed here; see [the permits](allen-county-building-permits-1990-2025.yml), whose own caution this uses rather than sets aside."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[
            Figure { label: "dwellings authorised", value: 1025.0, literal: "1,025" },
            Figure { label: "units removed, upper bound", value: 1461.0, literal: "1,461" },
        ],
    },
    Assertion {
        id: "three-quarters-of-the-loss-is-limas",
        statement: "Lima held 16,784 housing units in 2010 and 16,028 in 2020 - a fall of 756 \
                    against a gain of 320 in the twelve townships - and it annexed once in the \
                    decade, so the fall is not a boundary.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**Three quarters of the loss is Lima's and it is not a boundary.** The city held 16,784 units in 2010 and 16,028 in 2020, a fall of 756, while the other twelve subdivisions gained 320 between them. Lima annexed once in the decade, in June 2017. [verified] \u{2014} [the 2020 file](../../catalog/census-2020-dhc.md) and [the 2010 file](../../catalog/census-2010-sf1.md) at subdivision grain, against [the annexation record](allen-county-annexations-1990-2024.yml). The city was authorised 338 dwellings over the same years, 23 of them houses, so its own removals are of the order of eleven hundred. [inference] \u{2014} the place files of the same survey, computed here."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[
            Figure { label: "Lima units, 2010", value: 16784.0, literal: "16,784" },
            Figure { label: "Lima units, 2020", value: 16028.0, literal: "16,028" },
            Figure { label: "units lost", value: 756.0, literal: "756" },
        ],
    },
    Assertion {
        id: "every-kind-of-empty-fell-but-the-transactions",
        statement: "Between 2010 and 2020 every category of empty house in Allen County fell except \
                    the three that mean a sale or a letting is in progress. Other vacant fell 27.8 \
                    per cent, for-sale 38.2 and for-rent 11.3, while rented-not-occupied rose 131.6.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**Every category of empty fell except the three that mean a transaction is in progress.** [verified] \u{2014} the same table against [the 2010 summary file](../../catalog/census-2010-sf1.md), table H5."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[],
    },
    Assertion {
        id: "a-hundred-and-thirty-eight-empty-blocks",
        statement: "138 census blocks in Allen County hold housing units and report nobody living in \
                    any of them, 530 units in all. The corpus states the figure once and builds \
                    nothing on it, because a block's housing-unit count is enumerated and its \
                    occupancy is not.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**138 blocks in this county hold housing units and report nobody in any of them.** 530 units, the largest such block holding 22, four holding ten or more. [verified] \u{2014} the same file, block records. That figure is stated once and not built on: the housing-unit count on a block is enumerated and the occupancy of it is not, so the corpus can say exactly how many units stand there and not that they are empty. See [the total is enumerated and the split is not](../../decisions/the-total-is-enumerated-and-the-split-is-not.yml)."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[
            Figure { label: "blocks", value: 138.0, literal: "138" },
            Figure { label: "units", value: 530.0, literal: "530" },
        ],
    },
    Assertion {
        id: "the-county-is-ordinary-and-its-interior-is-not",
        statement: "Allen County's 8.14 per cent vacancy rate is 46th of Ohio's 88 counties and its \
                    share of other-vacant housing 48th, in a state whose range runs from Ottawa \
                    County's 37.94 per cent to Warren County's 4.49. Inside the county Lima is 11.2 \
                    per cent vacant and Amanda Township 4.5.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-vacancy-status-2010-2020.yml", "**Against Ohio's other eighty-seven counties this one is exactly ordinary.** Its 8.14 per cent vacancy rate is 46th of 88, its other-vacant share of stock 48th. Ottawa County on Lake Erie is 37.94 per cent vacant and Warren County outside Cincinnati is 4.49. [verified] \u{2014} the same file, all 88 county records read here. The middle of the state's range is where the county sits and its own interior does not: Lima is 11.2 per cent vacant and Amanda Township 4.5."),
        ],
        answers: &["cannot say why any particular house in Allen County is empty"],
        figures: &[
            Figure { label: "vacancy rate, per cent", value: 8.14, literal: "8.14" },
            Figure { label: "Ottawa County, per cent", value: 37.94, literal: "37.94" },
            Figure { label: "Warren County, per cent", value: 4.49, literal: "4.49" },
        ],
    },
    Assertion {
        id: "the-mortgaged-owners-left",
        statement: "Allen County lost 1,342 households owning with a mortgage between 2010 and 2020 \
                    and gained 616 owning outright and 1,042 renting. The count of owners fell by \
                    726 - little more than half as far as the mortgaged did.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-tenure-2010-2020.yml", "The county's owner-occupancy rate is the one figure of this kind the corpus already had, from a survey. [verified] \u{2014} [the housing stock](allen-county-housing-stock-2023.yml). Counted instead of estimated, and split three ways rather than two, it is a different story: **the county lost 1,342 households that own with a mortgage and gained 616 that own outright and 1,042 that rent.** [verified] \u{2014} [the 2020 characteristics file](../../catalog/census-2020-dhc.md) and [the 2010 summary file](../../catalog/census-2010-sf1.md), table H4 in both."),
            support!("measure/allen-county-tenure-2010-2020.yml", "**The survey does not publish the middle two rows and they are where the decade happened.** Owner occupancy fell 2.3 points and the count of owners fell by 726, but the mortgaged fell by 1,342 \u{2014} nearly twice as far \u{2014} because 616 households moved the other way and finished paying. [inference] \u{2014} the same two tables, differenced here. A count of owners is two populations that behave differently and this corpus had been reading them as one."),
        ],
        answers: &["cannot say how much of the county's thirty-six-point ownership gap is the city"],
        figures: &[
            Figure { label: "mortgaged owners lost", value: 1342.0, literal: "1,342" },
            Figure { label: "outright owners gained", value: 616.0, literal: "616" },
            Figure { label: "renter households gained", value: 1042.0, literal: "1,042" },
        ],
    },
    Assertion {
        id: "two-in-five-owners-here-owe-nothing",
        statement: "37.8 per cent of Allen County's owner households hold no mortgage, against 33.9 \
                    per cent for Ohio. The state's mortgaged owners fell 4.4 per cent over the \
                    decade and this county's fell 7.3.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-tenure-2010-2020.yml", "**Ohio did the same thing and this county did it harder.** The state's mortgaged owners fell 4.4 per cent over the decade and Allen County's fell 7.3; the state's outright owners rose 14.0 per cent and this county's 6.3. [verified] \u{2014} the same file and its 2010 predecessor, state rows. So the direction is the state's and the size is not, and the county ends the decade with 37.8 per cent of its owners holding no mortgage against Ohio's 33.9."),
        ],
        answers: &["cannot say how much of the county's thirty-six-point ownership gap is the city"],
        figures: &[
            Figure { label: "free and clear, per cent of owners", value: 37.8, literal: "37.8" },
            Figure { label: "Ohio, per cent", value: 33.9, literal: "33.9" },
        ],
    },
    Assertion {
        id: "seventy-third-of-eighty-eight-on-owning",
        statement: "Allen County is 73rd of Ohio's 88 counties on home ownership at 66.9 per cent, \
                    between Geauga County's 86.6 and Franklin County's 51.3, while Putnam County \
                    next door is second in the state at 83.8.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-tenure-2010-2020.yml", "**On the ownership rate itself it is 73rd of 88, and that is not the middle.** 66.9 per cent against Geauga County's 86.6 and Franklin County's 51.3; Putnam County, next door, is second in the state at 83.8. [verified] \u{2014} the same file, all 88 county rows."),
        ],
        answers: &["cannot say how much of the county's thirty-six-point ownership gap is the city"],
        figures: &[
            Figure { label: "ownership rate, per cent", value: 66.9, literal: "66.9" },
            Figure { label: "Geauga County, per cent", value: 86.6, literal: "86.6" },
            Figure { label: "Franklin County, per cent", value: 51.3, literal: "51.3" },
        ],
    },
    Assertion {
        id: "lima-stopped-being-a-city-of-owners",
        statement: "Lima was 50.6 per cent owner-occupied in 2010 and 45.9 per cent in 2020, on a \
                    number of occupied units that barely moved: 663 owner households became 679 \
                    renter households.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-tenure-2010-2020.yml", "**The county seat stopped being a city of owners between the two censuses.** Lima was 50.6 per cent owner-occupied in 2010 and 45.9 in 2020, and it did it with the number of occupied units almost unchanged \u{2014} 14,221 and 14,237. 663 owner households became 679 renter households. [verified] \u{2014} the same file and its 2010 predecessor, at place grain; the arithmetic is computed here."),
        ],
        answers: &["cannot say how much of the county's thirty-six-point ownership gap is the city"],
        figures: &[
            Figure { label: "owner-occupied, 2010, per cent", value: 50.6, literal: "50.6" },
            Figure { label: "owner-occupied, 2020, per cent", value: 45.9, literal: "45.9" },
        ],
    },
    Assertion {
        id: "owning-not-owing-divides-the-county",
        statement: "Urban Allen County is 59.7 per cent owner-occupied and rural Allen County 84.4, \
                    and of those owners 37.8 and 37.9 per cent respectively hold no mortgage. What \
                    separates the two halves is whether a household owns at all, not whether it has \
                    finished paying.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-tenure-2010-2020.yml", "**Urban and rural Allen County are 24.7 points apart on ownership and identical on mortgages.** The county's urban part is 59.7 per cent owner-occupied and its rural part 84.4; of those owners, 37.8 per cent and 37.9 per cent respectively own free and clear. [verified] \u{2014} the same file, geographic components 01 and 43 of the county record. The thing that separates the two halves of this county is whether a household owns at all, not whether it has finished paying. [inference]"),
        ],
        answers: &["cannot say how much of the county's thirty-six-point ownership gap is the city"],
        figures: &[
            Figure { label: "urban ownership, per cent", value: 59.7, literal: "59.7" },
            Figure { label: "rural ownership, per cent", value: 84.4, literal: "84.4" },
        ],
    },
    Assertion {
        id: "twice-as-likely-to-own",
        statement: "A White householder in Allen County is twice as likely to own their home as a \
                    Black one: 72.5 per cent of 33,629 White-alone households against 36.4 per cent \
                    of 4,908 Black-alone households, a gap of 36.1 points.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-tenure-2010-2020.yml", "**A White householder in this county is twice as likely to own as a Black one.** 72.5 per cent against 36.4 \u{2014} a gap of 36.1 points on 33,629 and 4,908 households. [verified] \u{2014} the same file, table H10, tenure by race of householder."),
        ],
        answers: &["cannot say how much of the county's thirty-six-point ownership gap is the city"],
        figures: &[
            Figure { label: "White ownership, per cent", value: 72.5, literal: "72.5" },
            Figure { label: "Black ownership, per cent", value: 36.4, literal: "36.4" },
        ],
    },
    Assertion {
        id: "the-county-gap-is-wider-than-either-half",
        statement: "Allen County's 36.1-point ownership gap by race is wider than the gap in either \
                    half of the county - 20.3 points inside Lima and 29.8 outside it - because Black \
                    households are 78.6 per cent Lima's and White households 27.7 per cent.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-tenure-2010-2020.yml", "**The county-wide gap is wider than the gap in either half of the county, and both halves are real.** Inside Lima it is 20.3 points, 52.9 per cent against 32.6. Outside Lima it is 29.8, 80.0 per cent against 50.2. The county figure of 36.1 exceeds both because Black households are 78.6 per cent Lima's and White households 27.7 per cent, and Lima is the only subdivision in the county where a majority of households rent. [verified] \u{2014} the same table, county and place records, with the outside-Lima figures the difference between them. See [a county rate can describe nowhere](../../decisions/a-county-rate-can-describe-nowhere.yml), which this sharpens: a county *difference* can be larger than the difference in every part of the county."),
        ],
        answers: &["cannot say how much of the county's thirty-six-point ownership gap is the city"],
        figures: &[
            Figure { label: "county gap, points", value: 36.1, literal: "36.1" },
            Figure { label: "gap inside Lima, points", value: 20.3, literal: "20.3" },
            Figure { label: "gap outside Lima, points", value: 29.8, literal: "29.8" },
        ],
    },
    Assertion {
        id: "black-households-grew-and-black-owners-did-not",
        statement: "The number of Black-alone households in Allen County rose by 379 between 2010 \
                    and 2020 and the number owning their home fell by 71, from 1,856 to 1,785, while \
                    Black renter households rose from 2,673 to 3,123.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-tenure-2010-2020.yml", "**Over the decade the number of Black households in this county rose by 379 and the number that own fell by 71.** 4,529 to 4,908 households, 1,856 to 1,785 owners, 2,673 to 3,123 renters. [inference] \u{2014} [the 2010 file](../../catalog/census-2010-sf1.md) against the 2020 one. This is the one comparison here that the change in race coding can reach: the corpus has established that the 2020 write-in coding moved households between the White-alone and multiple-race rows, and this county's multiple-race households went from 517 to 1,567 on it. [verified] \u{2014} see [the race series](allen-county-population-by-race-1970-2020.yml). What limits the damage is that the Black-alone *population* of the county barely moved across the same two censuses, 12,639 to 12,573."),
        ],
        answers: &["cannot say how much of the county's thirty-six-point ownership gap is the city"],
        figures: &[
            Figure { label: "owner households, 2010", value: 1856.0, literal: "1,856" },
            Figure { label: "owner households, 2020", value: 1785.0, literal: "1,785" },
        ],
    },
    Assertion {
        id: "the-total-is-counted-and-the-split-is-not",
        statement: "Allen County's 44,563 housing units are an enumeration carried unchanged from \
                    every one of its blocks, because the 2020 census held block housing-unit counts \
                    invariant. The 40,935 occupied and 3,628 vacant beside them are the output of \
                    the algorithm that protected everything else.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-occupancy-2020.yml", "**The units column above is a count and the two beside it are not.** 44,563 is carried unchanged from every block, because the number of housing units in a block is one of the three quantities the 2020 census held invariant; 40,935 and 3,628 are the output of the algorithm that protected everything else, and so is every occupancy figure in the subdivision table. [verified] \u{2014} [the technical documentation](../../catalog/census-2020-dhc.md), chapter 4; see [the total is enumerated and the split is not](../../decisions/the-total-is-enumerated-and-the-split-is-not.yml). Nothing above is withdrawn on that account. The claims made from it are about the county and its thirteen subdivisions, which is the scale the algorithm is built to preserve."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "housing units", value: 44563.0, literal: "44,563" },
            Figure { label: "occupied", value: 40935.0, literal: "40,935" },
            Figure { label: "vacant", value: 3628.0, literal: "3,628" },
        ],
    },
    Assertion {
        id: "something-was-pulled-down",
        statement: "The permit record and the census now say together what neither said alone: 1,025 \
                    dwellings were authorized in Allen County over the decade between the censuses \
                    and the county ended it with 436 fewer housing units. It built less than it \
                    lost.",
        topic: "housing",
        supports: &[
            support!("measure/allen-county-building-permits-1990-2025.yml", "**And something was pulled down.** 1,025 dwellings were authorized in the ten years between the two censuses and the county's housing stock fell by 436 units over the same span, 44,999 to 44,563. [verified] \u{2014} same source, 2010 through 2019, against [the two censuses](allen-county-vacancy-status-2010-2020.yml). Taken as an upper bound the other way round, that puts the decade's removals at about fifteen hundred units, and three quarters of the net fall is Lima's. [inference] The paragraph above still holds \u{2014} nothing here says which permit became a house \u{2014} but the direction is no longer open. This county built less than it lost."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "dwellings authorized", value: 1025.0, literal: "1,025" },
            Figure { label: "net units lost", value: 436.0, literal: "436" },
        ],
    },
    Assertion {
        id: "limas-houses-came-down",
        statement: "756 of Lima's housing units are gone between the two censuses - 16,784 to 16,028 \
                    - over a decade in which the city was authorized 338 dwellings and annexed once. \
                    Its vacancy rate fell from 15.3 per cent to 11.2 over the same years.",
        topic: "housing",
        supports: &[
            support!("place/lima.yml", "**And 756 of its housing units are gone.** 16,784 in 2010 against 16,028 in 2020, over a decade in which the city was authorized 338 dwellings and annexed once. [verified] \u{2014} [the two censuses](../../catalog/census-2020-dhc.md), read in [the vacancy record](../measure/allen-county-vacancy-status-2010-2020.yml), against [the annexations](../measure/allen-county-annexations-1990-2024.yml). Its vacancy rate fell across the same ten years, 15.3 per cent to 11.2, and 501 of the 772 fewer empty units were in the class that is neither for sale nor for rent. [inference] \u{2014} the same two censuses. A city can empty and tighten at once if the empty houses come down."),
        ],
        answers: &["does not establish that Lima is the poorest subdivision in the county"],
        figures: &[
            Figure { label: "units lost", value: 756.0, literal: "756" },
            Figure { label: "vacancy rate, 2010, per cent", value: 15.3, literal: "15.3" },
            Figure { label: "vacancy rate, 2020, per cent", value: 11.2, literal: "11.2" },
        ],
    },
    Assertion {
        id: "a-hundred-and-twelve-questions",
        statement: "Allen County's voters decided 112 local ballot questions on sixteen election \
                    days between May 2003 and November 2012, and passed 88 of them.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**A hundred and twelve questions on sixteen election days, and this county said yes to eighty-eight of them.** [verified] \u{2014} [the Secretary of State's issue reports](../../catalog/ohio-sos-local-issue-reports.md), ninety of them, read here. Everything below the candidates: levies, bonds, income taxes, a charter, a zoning referendum, an electric contract and the surrender of a village."),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[],
    },
    Assertion {
        id: "the-county-renews-and-does-not-add",
        statement: "Of the eighty tax and bond questions Allen County decided in these ten years, 45 \
                    asked to renew a levy and 44 passed, 13 asked to replace one and 13 passed, and \
                    17 asked for an additional levy and 6 passed. Fifty-seven of fifty-eight against \
                    six of seventeen.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**The county renews and it does not add.** Of the eighty tax and bond questions, 45 asked to renew a levy and 44 passed; 13 asked to replace one and 13 passed; 17 asked for an additional levy and 6 passed. That is 57 of 58 against 6 of 17 \u{2014} a gap of 62 points between asking for the same money again and asking for more. [verified] \u{2014} the same source, classified here on the word the ballot itself uses."),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[
            Figure { label: "renewals passed", value: 44.0, literal: "44" },
            Figure { label: "renewals asked", value: 45.0, literal: "45" },
            Figure { label: "additional passed", value: 6.0, literal: "6" },
            Figure { label: "additional asked", value: 17.0, literal: "17" },
        ],
    },
    Assertion {
        id: "the-renewal-that-failed-was-not-one",
        statement: "The only renewal Allen County refused in ten years had an increase attached to \
                    it. Bath Local School District's 9.56-mill renewal with a 1.74-mill increase \
                    lost 1,369 to 1,894 in March 2004 - 42.0 per cent, worse than any additional \
                    levy that passed.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**The one renewal that failed was not a renewal.** Bath Local School District, 2 March 2004: \"9.56 mills (renewal), and increase of 1.74 mills\". It lost 1,369 to 1,894 \u{2014} 42.0 per cent, the worst result any renewal got in ten years and eight points below the worst additional levy that passed. [verified] \u{2014} the same source. A renewal with an increase attached is read by this electorate as an increase. [inference]"),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[
            Figure { label: "yes", value: 1369.0, literal: "1,369" },
            Figure { label: "no", value: 1894.0, literal: "1,894" },
            Figure { label: "yes share, per cent", value: 42.0, literal: "42.0" },
        ],
    },
    Assertion {
        id: "a-replacement-passes-like-a-renewal",
        statement: "Thirteen replacement levies were put to Allen County voters between 2003 and \
                    2012 and thirteen passed, at a mean yes share of 64.3 per cent against the \
                    renewals' 64.9. The two are indistinguishable in this record even though a \
                    replacement collects more.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**A replacement is not a renewal either, and this county treats it as one.** Thirteen replacement levies were put and thirteen passed, at a mean yes share of 64.3 per cent against the renewals' 64.9 \u{2014} the two are indistinguishable in this record. [verified] \u{2014} the same source; the shares are computed here. What separates the words is what the tax collects; see [the rates](allen-county-property-tax-rates-2012-2025.yml). [inference]"),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[
            Figure { label: "replacement mean yes, per cent", value: 64.3, literal: "64.3" },
            Figure { label: "renewal mean yes, per cent", value: 64.9, literal: "64.9" },
        ],
    },
    Assertion {
        id: "new-borrowing-is-the-hardest-ask",
        statement: "Three of the eight bond issues put to Allen County voters between 2003 and 2012 \
                    passed. Elida Local School District was refused twice and Bath Local School \
                    District once, six months before the same district renewed a levy at 62.3 per \
                    cent.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**New borrowing is the hardest thing to ask for.** Three of eight bond issues passed. Elida Local School District was refused twice, in May 2007 at 42.9 per cent and in November 2006 at 41.0; Columbus Grove twice; Bath Local School District once, in May 2011 at 45.5 per cent, six months before the same district renewed a levy at 62.3. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[
            Figure { label: "Bath renewal yes share, per cent", value: 62.3, literal: "62.3" },
        ],
    },
    Assertion {
        id: "beaverdam-refused-three-times",
        statement: "A municipal income tax was refused four times in five in this county. Lima's \
                    rise from 1.5 to 1.9 per cent lost 2,335 to 4,447 in March 2004, and Beaverdam \
                    asked three times for one per cent and was refused three times - once on a \
                    return of 57 yes and 57 no.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**And a municipal income tax is refused four times in five.** Lima put a rise from 1.5 to 1.9 per cent on 2 March 2004 and lost 2,335 to 4,447 \u{2014} 34.4 per cent, the lowest yes share of any question in the decade that was not a village dissolving. Beaverdam asked three times for one per cent, in 2006, 2007 and 2009, and was refused each time; the 2007 return is **57 yes and 57 no**, and a question that does not carry a majority does not pass. [verified] \u{2014} the same source. Bluffton's quarter-per-cent levy for a village building, on an initiative petition in 2005, is the one that carried. [verified]"),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[
            Figure { label: "Lima yes", value: 2335.0, literal: "2,335" },
            Figure { label: "Lima no", value: 4447.0, literal: "4,447" },
            Figure { label: "Beaverdam yes, 2007", value: 57.0, literal: "57" },
        ],
    },
    Assertion {
        id: "everything-not-about-money-passed",
        statement: "Every question in Allen County between 2003 and 2012 that was not about money \
                    passed: three Lima charter amendments, a Bath Township zoning referendum at 78.8 \
                    per cent, two ordinances by petition, and seven of eight electric aggregations.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**Everything that was not about money passed.** Three Lima charter amendments, a Bath Township zoning referendum at 78.8 per cent, two ordinances by petition, and seven of eight electric aggregations. [verified] \u{2014} the same source. The single aggregation that failed is [Elida's](../place/elida.yml), and its published figure cannot be right."),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[
            Figure { label: "zoning referendum yes, per cent", value: 78.8, literal: "78.8" },
        ],
    },
    Assertion {
        id: "the-busiest-ballot-was-an-odd-year",
        statement: "The most local questions Allen County ever decided at once in this record was \
                    nineteen, on 8 November 2011, and eighteen of them passed. The two presidential \
                    ballots in the same decade carried nine and twelve.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**The busiest ballot in the decade was an odd-numbered year.** Nineteen questions on 8 November 2011 and eighteen of them passed; sixteen on 7 November 2006 and twelve passed. The two presidential years carried nine and twelve. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[],
    },
    Assertion {
        id: "thirteen-questions-filed-elsewhere",
        statement: "Thirteen of the 112 questions Allen County's voters were asked between 2003 and \
                    2012 are not printed under this county at all - Columbus Grove's bonds under \
                    Putnam, Waynesfield-Goshen's levies under Auglaize - because Allen is the \
                    smaller partner in those districts.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**Thirteen of the hundred and twelve are not printed under this county at all.** Columbus Grove's bonds and income tax are filed under Putnam, Waynesfield-Goshen's levies and Pandora-Gilboa's income tax under Putnam or Auglaize, because Allen is the smaller partner in each district. [verified] \u{2014} the same source, read across all eighty-eight counties. Thirty-seven of the hundred and twelve are multi-county in one direction or the other."),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[],
    },
    Assertion {
        id: "the-village-surrendered-its-corporate-power",
        statement: "The Village of Fort Shawnee was ended by its own electors on 6 November 2012. \
                    Asked \"Shall the Village known as Fort Shawnee surrender its corporate \
                    power?\", they answered 1,058 to 858.",
        topic: "elections",
        supports: &[
            support!("jurisdiction/village-of-fort-shawnee.yml", "**It was ended by a vote, on a petition, on 6 November 2012.** The question was \"Shall the Village known as Fort Shawnee surrender its corporate power?\" and the answer was **1,058 to 858** \u{2014} 55.2 per cent, on 1,916 votes. [verified] \u{2014} [the Secretary of State's report of miscellaneous questions](../../catalog/ohio-sos-local-issue-reports.md) for that election, under ALLEN COUNTY. This node exists because that return closes [a question this corpus had carried open since its first week](../question/what-happened-to-the-village-of-fort-shawnee.yml)."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "yes", value: 1058.0, literal: "1,058" },
            Figure { label: "no", value: 858.0, literal: "858" },
            Figure { label: "yes share, per cent", value: 55.2, literal: "55.2" },
        ],
    },
    Assertion {
        id: "fort-shawnee-refused-to-pay-then-dissolved",
        statement: "Fort Shawnee refused an additional four-mill levy 435 to 796 in November 2011, \
                    refused a 3.25-mill levy 592 to 1,323 on the ballot that dissolved it, and voted \
                    to surrender its corporate power on that same day. The second levy's 30.9 per \
                    cent is the lowest yes share on any question put anywhere in the county in ten \
                    years.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**Fort Shawnee refused to tax itself twice and then abolished itself.** An additional four-mill levy lost 435 to 796 in November 2011 and a 3.25-mill levy lost 592 to 1,323 in November 2012 \u{2014} 30.9 per cent, the lowest of the decade. On the same 2012 ballot the village's electors were asked whether it should surrender its corporate power and answered 1,058 to 858. [verified] \u{2014} the same source; see [the village](../jurisdiction/village-of-fort-shawnee.yml)."),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[
            Figure { label: "2012 levy yes", value: 592.0, literal: "592" },
            Figure { label: "2012 levy no", value: 1323.0, literal: "1,323" },
            Figure { label: "dissolution yes", value: 1058.0, literal: "1,058" },
        ],
    },
    Assertion {
        id: "two-instruments-put-the-end-in-the-same-year",
        statement: "The county Auditor's tax set for Fort Shawnee levies 2.150 mills of village tax \
                    in tax year 2012 and none in 2013, and the Census Bureau's gazetteer draws a \
                    village in the 2012 vintage and a census designated place in the 2013. The vote \
                    that ended the corporation falls between them.",
        topic: "elections",
        supports: &[
            support!("jurisdiction/village-of-fort-shawnee.yml", "**Two instruments with nothing in common put that end inside the next twelve months.** The county Auditor's tax set L36, *Fort Shawnee Corp.*, levies 2.150 mills of village tax in tax year 2012 and none in 2013; the Census Bureau's gazetteer carries the place with legal code 47, village, in the 2012 vintage and code 57, census designated place, in the 2013. [verified] \u{2014} [the Auditor's tax rate summaries](../../catalog/allen-county-auditor-tax-rates.md) and [the gazetteer](../../catalog/census-gazetteer-2020.md). The vote falls between them, five weeks before the first tax year in which nothing was levied."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "village millage, tax year 2012", value: 2.15, literal: "2.150" },
        ],
    },
    Assertion {
        id: "the-survey-misses-what-a-government-gives-up",
        statement: "Ohio's boundary and annexation survey holds fifty-seven records for Allen County \
                    between 1990 and 2024 and every one is an annexation. The county's one \
                    dissolution in that window is not in the file: the survey misses what a \
                    government gives up, and the largest thing it can give up is itself.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-annexations-1990-2024.yml", "**The one boundary change of consequence in this window is not in the file either, and it is not an annexation.** Fort Shawnee's electors ended the village on 6 November 2012 and the survey has no row for it. [verified] \u{2014} [the Secretary of State's report of miscellaneous questions](../../catalog/ohio-sos-local-issue-reports.md); see [the Village of Fort Shawnee](../jurisdiction/village-of-fort-shawnee.yml). That sharpens the reading above rather than replacing it: the survey misses what a government gives up, and the largest thing a government can give up is itself. [inference]"),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "thirty-liquor-questions-precinct-by-precinct",
        statement: "Allen County decided thirty local option liquor questions in eight elections \
                    between 2004 and 2012, precinct by precinct, and passed twenty-five. 5,528 votes \
                    yes and 3,599 no in ten years.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-local-option-elections-2003-2012.yml", "**Thirty liquor questions in eight elections, precinct by precinct, and this county passed twenty-five of them.** [verified] \u{2014} [the Secretary of State's local option reports](../../catalog/ohio-sos-local-issue-reports.md), sixteen of them read here. 5,528 votes yes and 3,599 no across ten years: the smallest electorate and the most particular question on any ballot in Ohio."),
        ],
        answers: &["cannot say whether any of these questions changed what a shop sold"],
        figures: &[
            Figure { label: "yes votes", value: 5528.0, literal: "5,528" },
            Figure { label: "no votes", value: 3599.0, literal: "3,599" },
        ],
    },
    Assertion {
        id: "an-olive-garden-and-a-red-lobster",
        statement: "Perry Township precinct N refused an Olive Garden 119 to 125 and a Red Lobster \
                    119 to 124 on the same day in November 2005, refused a third question 139 to 151 \
                    in 2010, and has passed none in this record.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-local-option-elections-2003-2012.yml", "**Two restaurants in one precinct were refused on one day, by six votes and by five.** Perry Township precinct N, 8 November 2005: the Olive Garden 119 to 125 and the Red Lobster 119 to 124. The same precinct refused a third question in November 2010, 139 to 151, and has passed none in this record. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say whether any of these questions changed what a shop sold"],
        figures: &[
            Figure { label: "Olive Garden yes", value: 119.0, literal: "119" },
            Figure { label: "Olive Garden no", value: 125.0, literal: "125" },
            Figure { label: "Red Lobster no", value: 124.0, literal: "124" },
        ],
    },
    Assertion {
        id: "carried-by-two-votes",
        statement: "The narrowest thing Allen County has approved in this record is a filling \
                    station: the Duke & Duchess BP on the Harding Highway in Bath Township precinct \
                    D carried 136 to 134 on 2 November 2004, out of 270 votes.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-local-option-elections-2003-2012.yml", "**The narrowest thing this county has ever approved is a filling station.** The Duke & Duchess BP on the Harding Highway in Bath Township precinct D carried 136 to 134 on 2 November 2004 \u{2014} a two-vote margin out of 270. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say whether any of these questions changed what a shop sold"],
        figures: &[
            Figure { label: "yes", value: 136.0, literal: "136" },
            Figure { label: "no", value: 134.0, literal: "134" },
        ],
    },
    Assertion {
        id: "one-precinct-two-answers",
        statement: "Spencerville precinct B answered two liquor questions oppositely on one visit to \
                    the booth on 8 November 2005: form 5-R1 carried 104 to 71 and form 5-R2 lost 81 \
                    to 96, a difference of 32 points.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-local-option-elections-2003-2012.yml", "**One precinct answered two questions oppositely on the same day.** Spencerville precinct B, 8 November 2005: form 5-R1 carried 104 to 71 and form 5-R2 lost 81 to 96. Whatever separates the two forms, this electorate distinguished them by 32 points in one visit to the booth. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say whether any of these questions changed what a shop sold"],
        figures: &[
            Figure { label: "R1 yes", value: 104.0, literal: "104" },
            Figure { label: "R2 yes", value: 81.0, literal: "81" },
            Figure { label: "R2 no", value: 96.0, literal: "96" },
        ],
    },
    Assertion {
        id: "no-report-says-what-the-forms-ask",
        statement: "All eight local option questions Allen County decided on form 5-R1 passed and \
                    seventeen of twenty-two on form 5-R2 did, with every refusal an R2 - and no copy \
                    of any report this corpus holds says what the two forms ask.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-local-option-elections-2003-2012.yml", "**And the separation holds across the county.** Eight questions on form 5-R1 were put in these ten years and eight passed, at a mean yes share of 65.3 per cent; twenty-two were put on form 5-R2 and seventeen passed, at 58.0. All five refusals are R2. [verified] \u{2014} the same source; the shares are computed here. **No copy of any report this corpus holds says what R1 and R2 are.** The 2004 reports print the lettered question texts \u{2014} off-premise sale, on-premise sale, spirituous liquor by the glass, Sunday sale \u{2014} and the columns that carry the returns are labelled by form and number instead, with no key. [verified] \u{2014} the same source, all sixteen of them."),
        ],
        answers: &["cannot say whether any of these questions changed what a shop sold"],
        figures: &[
            Figure { label: "R1 mean yes, per cent", value: 65.3, literal: "65.3" },
            Figure { label: "R2 mean yes, per cent", value: 58.0, literal: "58.0" },
        ],
    },
    Assertion {
        id: "the-elida-return-cannot-be-right",
        statement: "The Secretary of State's report gives Elida Village 3,622 yes and 5,454 no on an \
                    electric aggregation question in November 2012 - 9,076 votes in a village of \
                    about nineteen hundred people. Nothing in the file flags it and no correction is \
                    archived.",
        topic: "elections",
        supports: &[
            support!("place/elida.yml", "**A published election return gives this village nine thousand votes.** *Proposed Electric Aggregation \u{2014} Elida Village*, 6 November 2012: 3,622 yes and 5,454 no, 9,076 in all, in a place whose entire population is about nineteen hundred. [verified] \u{2014} [the Secretary of State's report of miscellaneous questions](../../catalog/ohio-sos-local-issue-reports.md) against the estimates above. Nothing in the file flags it and no correction is archived; the figure is 4.8 times the village's people and cannot be its vote. [inference] \u{2014} see [the ballot record](../measure/allen-county-ballot-questions-2003-2012.yml), which counts the question and refuses the number."),
        ],
        answers: &["What the excess is, this corpus does not know"],
        figures: &[
            Figure { label: "yes", value: 3622.0, literal: "3,622" },
            Figure { label: "no", value: 5454.0, literal: "5,454" },
            Figure { label: "total votes", value: 9076.0, literal: "9,076" },
        ],
    },
    Assertion {
        id: "a-return-under-a-county-is-not-its-vote",
        statement: "One question under the ALLEN COUNTY heading in November 2012 carries 48,543 \
                    votes, and the county cast 48,708 ballots that day on everything together. The \
                    report files a district that crosses a county line under whichever county holds \
                    most of it, with the whole district's vote.",
        topic: "elections",
        supports: &[
            support!("measure/allen-county-ballot-questions-2003-2012.yml", "**This corpus cannot say how Allen County voted on any question that crossed its own line.** The reports give one figure per question and it is the whole subdivision's; a district straddling two counties has one number and no split. [verified] \u{2014} [the reports' own convention](../../catalog/ohio-sos-local-issue-reports.md), `* denotes most populous county`; see [a return filed under a county is not that county's vote](../../decisions/a-return-filed-under-a-county-is-not-that-countys-vote.yml)."),
        ],
        answers: &["cannot say how Allen County voted on any question that crossed its own line"],
        figures: &[],
    },
    Assertion {
        id: "the-uninsured-fell-by-half-in-three-years",
        statement: "Allen County had 4,904 fewer people under 65 without health insurance in 2016 \
                    than in 2013, a fall of more than half, and 4,628 of it is people aged 18 to 64.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-insurance-2008-2024.yml", "Seventeen years of a modelled county estimate, and the county's uninsured population fell by more than half in three of them. [verified] \u{2014} [the Small Area Health Insurance Estimates](../../catalog/census-sahie.md), under 65, all incomes, both sexes."),
            support!("measure/allen-county-health-insurance-2008-2024.yml", "**What fell was the adults.** The rate for people aged 18 to 64 went from 16.1 per cent in 2013 to 8.6 in 2016; the rate for people under 19 went from 5.2 to 4.0 and has never been above 7.4 in any year of the series. [verified] \u{2014} the same source, age categories 1 and 4. Of the 4,904 fewer uninsured people between 2013 and 2016, 4,628 are the fall in the 18-to-64 count alone."),
        ],
        answers: &["cannot say how many people in Allen County are enrolled in Medicaid"],
        figures: &[
            Figure { label: "fewer uninsured, 2013 to 2016", value: 4904.0, literal: "4,904" },
            Figure { label: "of that fall aged 18 to 64", value: 4628.0, literal: "4,628" },
        ],
    },
    Assertion {
        id: "the-children-were-already-covered",
        statement: "The uninsured rate for Allen County residents aged 18 to 64 went from 16.1 per \
                    cent in 2013 to 8.6 in 2016. The rate for those under 19 went from 5.2 to 4.0, \
                    and it has never been above 7.4 in seventeen years.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-insurance-2008-2024.yml", "**What fell was the adults.** The rate for people aged 18 to 64 went from 16.1 per cent in 2013 to 8.6 in 2016; the rate for people under 19 went from 5.2 to 4.0 and has never been above 7.4 in any year of the series. [verified] \u{2014} the same source, age categories 1 and 4. Of the 4,904 fewer uninsured people between 2013 and 2016, 4,628 are the fall in the 18-to-64 count alone."),
        ],
        answers: &["cannot say how many people in Allen County are enrolled in Medicaid"],
        figures: &[
            Figure { label: "uninsured, 18\u{2013}64, 2013, per cent", value: 16.1, literal: "16.1" },
            Figure { label: "uninsured, 18\u{2013}64, 2016, per cent", value: 8.6, literal: "8.6" },
            Figure { label: "uninsured, under 19, 2016, per cent", value: 4.0, literal: "4.0" },
        ],
    },
    Assertion {
        id: "the-income-gradient-closed",
        statement: "In 2013 Allen County residents at or below 138 per cent of the poverty line were \
                    21.1 per cent uninsured against 12.7 per cent of those between 138 and 400 per \
                    cent. By 2019 it was 9.7 against 9.0.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-insurance-2008-2024.yml", "**The income gradient did not narrow so much as close.** In 2013 the county's residents at or below 138 per cent of the poverty line were 21.1 per cent uninsured against 12.7 per cent for those between 138 and 400. By 2019 it was 9.7 against 9.0, and the poorest band was better covered than the band at or below 200 per cent. [verified] \u{2014} the same file, income categories 3 and 5. Ohio expanded Medicaid to 138 per cent of poverty on 1 January 2014."),
        ],
        answers: &["cannot say how many people in Allen County are enrolled in Medicaid"],
        figures: &[
            Figure { label: "uninsured at or below 138% of poverty, 2013", value: 21.1, literal: "21.1" },
            Figure { label: "uninsured 138\u{2013}400% of poverty, 2013", value: 12.7, literal: "12.7" },
            Figure { label: "uninsured at or below 138% of poverty, 2019", value: 9.7, literal: "9.7" },
        ],
    },
    Assertion {
        id: "the-model-was-told-about-the-programme",
        statement: "Medicaid enrolment by age and sex is one of the inputs to the model that \
                    produces this county's uninsured rate, so the file cannot be used to show that \
                    expanding Medicaid is what cut it.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-insurance-2008-2024.yml", "**That coincidence is as far as this file may be taken.** Medicaid enrolment by age and sex is one of the model's own inputs. [verified] \u{2014} the model input data page cited in [the source entry](../../catalog/census-sahie.md). See [a model fitted to a survey is not a second witness](../../decisions/a-model-fitted-to-a-survey-is-not-a-second-witness.yml)."),
        ],
        answers: &["cannot say how many people in Allen County are enrolled in Medicaid"],
        figures: &[],
    },
    Assertion {
        id: "men-are-less-insured-in-every-year",
        statement: "Men in Allen County have been less insured than women in every one of the \
                    seventeen years the series covers, by between 1.2 and 3.4 points \u{2014} 14.5 \
                    per cent against 11.1 in 2013, 7.8 against 6.5 in 2024.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-insurance-2008-2024.yml", "**Men in this county are less insured than women in every one of the seventeen years.** The gap runs from 1.2 points to 3.4 and never closes or reverses: 14.5 against 11.1 in 2013, 7.8 against 6.5 in 2024. [verified] \u{2014} the same source, sex categories 1 and 2."),
        ],
        answers: &["cannot say how many people in Allen County are enrolled in Medicaid"],
        figures: &[
            Figure { label: "narrowest gap, points", value: 1.2, literal: "1.2" },
            Figure { label: "widest gap, points", value: 3.4, literal: "3.4" },
            Figure { label: "men uninsured, 2013, per cent", value: 14.5, literal: "14.5" },
            Figure { label: "women uninsured, 2013, per cent", value: 11.1, literal: "11.1" },
        ],
    },
    Assertion {
        id: "the-model-and-its-own-survey-part",
        statement: "The survey this model is fitted to counts 4,410 county residents under 65 \
                    without coverage in 2024, on a margin of 1,084; the model estimates 5,613. The \
                    gap of 1,203 people is a property of the model.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-insurance-2008-2024.yml", "**The survey the model is fitted to can be read on its own, and in 2024 it does not agree.** The American Community Survey's one-year estimate puts 4,410 \u{b1} 1,084 county residents under 65 without coverage in 2024, a rate of 5.6 \u{b1} 1.4 per cent, against the model's 5,613 and 7.1 \u{b1} 0.8. In 2021, 2022 and 2023 the two run within a quarter of a point of each other. [verified] \u{2014} [the survey's table B27001](../../catalog/census-acs-summary-file.md), one-year files for 2021 to 2024, summed here over the seven age bands under 65. The gap of 1,203 people is a property of the model."),
        ],
        answers: &["cannot say how many people in Allen County are enrolled in Medicaid"],
        figures: &[
            Figure { label: "ACS one-year uninsured under 65, 2024", value: 4410.0, literal: "4,410" },
            Figure { label: "SAHIE uninsured under 65, 2024", value: 5613.0, literal: "5,613" },
            Figure { label: "gap, people", value: 1203.0, literal: "1,203" },
        ],
    },
    Assertion {
        id: "a-rank-that-moved-twenty-seven-places",
        statement: "Allen County's rank among Ohio's 88 counties on its uninsured rate was 17th in \
                    2011, 63rd in 2017 and 22nd in 2024, and it moved 27 places between 2017 and \
                    2018 on a change of 0.3 points \u{2014} well inside its own margin.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-insurance-2008-2024.yml", "**The county's rank among Ohio's 88 is worth less than its rate.** Allen was 17th in 2011, 63rd in 2017 and 22nd in 2024, and between 2017 and 2018 it moved 27 places on a change of 0.3 points \u{2014} well inside its own margin. [verified] \u{2014} the same source, all 88 county rows read for each year. See [a rank is an estimate](../../decisions/a-rank-is-an-estimate.yml)."),
        ],
        answers: &["cannot say how many people in Allen County are enrolled in Medicaid"],
        figures: &[
            Figure { label: "places moved, 2017 to 2018", value: 27.0, literal: "27" },
            Figure { label: "change in rate, points", value: 0.3, literal: "0.3" },
        ],
    },
    Assertion {
        id: "an-older-series-that-does-not-join",
        statement: "The same programme published county estimates for 2005 to 2007 from a different \
                    survey: Allen County at 12.2 per cent in 2006 on a margin of 2.0, and 12.5 in \
                    2007 on 1.7 \u{2014} intervals roughly twice as wide as anything after.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-insurance-2008-2024.yml", "**There is an older series and it is not the beginning of this one.** The same programme published county estimates for 2005 to 2007 from the Current Population Survey: Allen County at 12.2 \u{b1} 2.0 per cent in 2006 and 12.5 \u{b1} 1.7 in 2007, on intervals roughly twice as wide as anything after. [verified] \u{2014} [the same source](../../catalog/census-sahie.md), the CPS-based directory. The 2007 interval overlaps 2008's, and an overlap across a change of method is not a join. See [a before and after needs a before](../../decisions/a-before-and-after-needs-a-before.yml)."),
        ],
        answers: &["cannot say how many people in Allen County are enrolled in Medicaid"],
        figures: &[
            Figure { label: "uninsured, 2006, per cent", value: 12.2, literal: "12.2" },
            Figure { label: "margin, 2006, points", value: 2.0, literal: "2.0" },
            Figure { label: "uninsured, 2007, per cent", value: 12.5, literal: "12.5" },
        ],
    },
    Assertion {
        id: "a-third-of-the-children-are-on-medicaid",
        statement: "8,595 of Allen County's 25,250 residents under 19 hold Medicaid and nothing \
                    else. In Lima it is 5,336 of 8,990 \u{2014} three children in five.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-coverage-types-2023.yml", "**A third of the county's children are on Medicaid alone, and in Lima it is three in five.** 8,595 of 25,250 under 19 across the county; 5,336 of 8,990 in the city. [verified] \u{2014} the same table, at county and place grain. Employer coverage runs the other way: 47.5 per cent of the county's children and 26.5 per cent of Lima's."),
        ],
        answers: &["cannot say what any of this coverage pays for"],
        figures: &[
            Figure { label: "children on Medicaid only, county", value: 8595.0, literal: "8,595" },
            Figure { label: "children under 19, county", value: 25250.0, literal: "25,250" },
            Figure { label: "children on Medicaid only, Lima", value: 5336.0, literal: "5,336" },
            Figure { label: "children under 19, Lima", value: 8990.0, literal: "8,990" },
        ],
    },
    Assertion {
        id: "one-resident-in-five-holds-public-coverage",
        statement: "At least 19,767 people in Allen County \u{2014} one in five \u{2014} hold \
                    Medicaid or other means-tested public coverage; 17,685 have it and nothing else \
                    and 2,082 hold it with Medicare.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-coverage-types-2023.yml", "**At least 19,767 people in this county \u{2014} one in five \u{2014} hold means-tested public coverage.** 17,685 have it and nothing else and 2,082 hold it with Medicare; the residual combination rows will contain more. [verified] \u{2014} the same table, lines 7, 23, 39, 13, 29, 46 and 62, summed here."),
        ],
        answers: &["cannot say what any of this coverage pays for"],
        figures: &[
            Figure { label: "on means-tested public coverage", value: 19767.0, literal: "19,767" },
            Figure { label: "on it and nothing else", value: 17685.0, literal: "17,685" },
            Figure { label: "with Medicare as well", value: 2082.0, literal: "2,082" },
        ],
    },
    Assertion {
        id: "forty-one-uninsured-people-over-sixty-five",
        statement: "41 of Allen County's 18,091 residents aged 65 and over have no health insurance. \
                    At least 16,836 people in the county hold Medicare in some form, and 2,380 of \
                    them are under 65.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-coverage-types-2023.yml", "**Nobody over 65 is uninsured.** 41 people of 18,091, which is 0.2 per cent, against 4.0 per cent of the county's under-65 residents. [verified] \u{2014} the same source. Only 5,660 of those 18,091 hold Medicare and nothing else; 11,968 hold two kinds or more. At least 16,836 people in the county hold Medicare in some form, and 2,380 of them are under 65. [verified] \u{2014} the same table, the Medicare-only and Medicare-combination lines of all four age groups, summed here."),
        ],
        answers: &["cannot say what any of this coverage pays for"],
        figures: &[
            Figure { label: "uninsured aged 65 and over", value: 41.0, literal: "41" },
            Figure { label: "residents aged 65 and over", value: 18091.0, literal: "18,091" },
            Figure { label: "holding Medicare in some form", value: 16836.0, literal: "16,836" },
            Figure { label: "of them under 65", value: 2380.0, literal: "2,380" },
        ],
    },
    Assertion {
        id: "limas-coverage-is-not-its-poverty-rate",
        statement: "Lima's uninsured rate is 7.0 per cent against its county's 6.3, a difference of \
                    0.7 points against a combined margin of 1.4. The city holds 33.9 per cent of the \
                    people in these tables and 64.1 per cent of the county's poor.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-coverage-types-2023.yml", "**Lima's uninsured rate is not distinguishable from its county's, and its poverty rate is roughly double.** 7.0 \u{b1} 1.2 per cent against 6.3 \u{b1} 0.7, a difference of 0.7 points against a combined margin of 1.4. [verified] \u{2014} the same tables, aggregated here with the margins combined in quadrature. The city holds 33.9 per cent of the people in these tables and 64.1 per cent of the county's poor. [verified] \u{2014} [the poverty tables](allen-county-income-and-poverty-2023.yml). What covers the poor is why. [inference]"),
        ],
        answers: &["cannot say what any of this coverage pays for"],
        figures: &[
            Figure { label: "Lima uninsured, per cent", value: 7.0, literal: "7.0" },
            Figure { label: "county uninsured, per cent", value: 6.3, literal: "6.3" },
            Figure { label: "Lima's share of the county's poor, per cent", value: 64.1, literal: "64.1" },
        ],
    },
    Assertion {
        id: "the-uninsured-are-men-in-their-thirties",
        statement: "14.7 per cent of Allen County men aged 26 to 34 and 13.2 per cent of those aged \
                    35 to 44 have no coverage, against 6.7 and 6.1 per cent of women the same ages. \
                    In Lima the figure for men aged 35 to 44 is 23.8 per cent.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-coverage-types-2023.yml", "**The county's uninsured are men in their working middle years.** 14.7 per cent of men aged 26 to 34 and 13.2 per cent of men aged 35 to 44, against 6.7 and 6.1 per cent of women the same ages. [verified] \u{2014} the same source, table B27001. In Lima the figure for men aged 35 to 44 is 23.8 per cent \u{2014} 413 people of 1,733, on a margin of 191 \u{2014} against 3.5 per cent of the city's women that age."),
        ],
        answers: &["cannot say what any of this coverage pays for"],
        figures: &[
            Figure { label: "men 26\u{2013}34 uninsured, per cent", value: 14.7, literal: "14.7" },
            Figure { label: "men 35\u{2013}44 uninsured, per cent", value: 13.2, literal: "13.2" },
            Figure { label: "Lima men 35\u{2013}44 uninsured, per cent", value: 23.8, literal: "23.8" },
        ],
    },
    Assertion {
        id: "ninety-seven-of-three-hundred-and-four-trained-abroad",
        statement: "Allen County has 304 physicians in patient care and 97 of them are graduates of \
                    institutions outside the United States and Canada, against 207 who trained in \
                    the United States.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-workforce-2023.yml", "**Nearly a third of the doctors seeing patients here trained abroad.** 97 of 304 are graduates of institutions outside the United States and Canada, against 207 who trained in the United States. [verified] \u{2014} the same file."),
        ],
        answers: &["cannot say whether any of these doctors saw a patient from Allen County"],
        figures: &[
            Figure { label: "graduates of institutions outside the US and Canada", value: 97.0, literal: "97" },
            Figure { label: "physicians in patient care", value: 304.0, literal: "304" },
            Figure { label: "US graduates", value: 207.0, literal: "207" },
        ],
    },
    Assertion {
        id: "fourth-in-ohio-for-primary-care-residents",
        statement: "Allen County's 38 primary care residents are 37.7 per 100,000 residents, fourth \
                    of Ohio's 88 counties behind Hamilton, Cuyahoga and Lucas and ahead of Summit. \
                    Forty of the 88 train none.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-workforce-2023.yml", "**This county trains primary care doctors at Cleveland's rate.** Its 38 residents are 37.7 per 100,000 residents, fourth of Ohio's 88 counties behind Hamilton, Cuyahoga and Lucas and ahead of Summit; forty of the 88 train none. [verified] \u{2014} the same file, all 88 Ohio county rows read here against the file's own 2023 population estimate of 100,838."),
        ],
        answers: &["cannot say whether any of these doctors saw a patient from Allen County"],
        figures: &[
            Figure { label: "primary care residents", value: 38.0, literal: "38" },
            Figure { label: "per 100,000 residents", value: 37.7, literal: "37.7" },
        ],
    },
    Assertion {
        id: "below-the-state-on-the-doctors-who-stay",
        statement: "On primary care physicians who are not in training Allen County is eighteenth of \
                    88 and below its state: 67 of them, 66.4 per 100,000 against Ohio's 76.7.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-workforce-2023.yml", "**On the primary care doctors who are not in training it is eighteenth, and below the state.** 67 of them, 66.4 per 100,000 against Ohio's 76.7. [verified] \u{2014} the same source. A resident is a doctor seeing patients and is also a doctor who will leave."),
        ],
        answers: &["cannot say whether any of these doctors saw a patient from Allen County"],
        figures: &[
            Figure { label: "primary care physicians, excluding residents", value: 67.0, literal: "67" },
            Figure { label: "per 100,000", value: 66.4, literal: "66.4" },
            Figure { label: "Ohio per 100,000", value: 76.7, literal: "76.7" },
        ],
    },
    Assertion {
        id: "six-psychiatrists-and-no-child-psychiatrist",
        statement: "Allen County has six psychiatrists \u{2014} 6.0 per 100,000 against Ohio's 11.4 \
                    \u{2014} of whom three are 65 or older, and it has no child psychiatrist. \
                    Twenty-eight Ohio counties have no psychiatrist at all.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-workforce-2023.yml", "**The county has six psychiatrists and no child psychiatrist.** Six is 6.0 per 100,000 against the state's 11.4; three of the six are 65 or older and one is over 75. The single child psychiatrist on the 2022 file, aged between 55 and 64, is not on the 2023 file. [verified] \u{2014} the same source, by specialty and age band. Twenty-eight Ohio counties have no psychiatrist at all and fifty-eight have no child psychiatrist, so the county is not unusual; it is ordinary."),
        ],
        answers: &["cannot say whether any of these doctors saw a patient from Allen County"],
        figures: &[
            Figure { label: "psychiatrists per 100,000", value: 6.0, literal: "6.0" },
            Figure { label: "Ohio psychiatrists per 100,000", value: 11.4, literal: "11.4" },
        ],
    },
    Assertion {
        id: "six-of-eight-obstetricians-are-fifty-five-or-older",
        statement: "Of Allen County's eight obstetrician-gynaecologists, one is aged 35 to 44, one \
                    45 to 54, four 55 to 64 and two 65 to 74.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-workforce-2023.yml", "**Six of the county's eight obstetrician-gynaecologists are 55 or older.** One is aged 35 to 44, one 45 to 54, four 55 to 64 and two 65 to 74; two further doctors of osteopathy practise the specialty and the file does not break out their ages. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say whether any of these doctors saw a patient from Allen County"],
        figures: &[],
    },
    Assertion {
        id: "more-practitioners-than-physicians",
        statement: "Allen County has 221 nurse practitioners and 127 physician assistants against \
                    304 physicians in patient care, and on nurse practitioners it is seventh of \
                    Ohio's 88 counties at 219.2 per 100,000 against the state's 158.7.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-health-workforce-2023.yml", "**There are more nurse practitioners and physician assistants than physicians.** 221 and 127 against 304 in patient care, and on nurse practitioners the county is seventh of Ohio's 88 at 219.2 per 100,000 against the state's 158.7. [verified] \u{2014} the same source. These are counts of registrations with Medicare at an address in the county and not of the same universe as the masterfiles, which is why the ADA finds 40 dentists in private practice here and the identifier file finds 63. See [an address of record is not a residence](../../decisions/an-address-of-record-is-not-a-residence.yml)."),
        ],
        answers: &["cannot say whether any of these doctors saw a patient from Allen County"],
        figures: &[
            Figure { label: "nurse practitioners", value: 221.0, literal: "221" },
            Figure { label: "physician assistants", value: 127.0, literal: "127" },
            Figure { label: "nurse practitioners per 100,000", value: 219.2, literal: "219.2" },
        ],
    },
    Assertion {
        id: "the-designation-is-the-countys-poor",
        statement: "The federal primary care shortage designation covering Allen County is drawn on \
                    its 32,355 low-income residents wherever in the county they live, of whom 14,781 \
                    are recorded as underserved for primary care and 19,346 for dental.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-shortage-designations-1985-2026.yml", "**The county's designation is its poor, and it is not drawn on a map.** `LI- Allen County` covers the 32,355 residents at or below the low-income threshold wherever in the county they live, of whom 14,781 are recorded as underserved for primary care and 19,346 for dental. [verified] \u{2014} the same register. Saying \"Allen County is a shortage area\" describes a third of it; see [a designation is not a county](../../decisions/a-designation-is-not-a-county.yml)."),
        ],
        answers: &["cannot say whether anyone went without care"],
        figures: &[
            Figure { label: "low-income residents covered", value: 32355.0, literal: "32,355" },
            Figure { label: "underserved, primary care", value: 14781.0, literal: "14,781" },
            Figure { label: "underserved, dental", value: 19346.0, literal: "19,346" },
        ],
    },
    Assertion {
        id: "half-a-psychiatrist-for-fifty-thousand-people",
        statement: "The county's mental health shortage designation records 0.5386 \
                    full-time-equivalent psychiatrists against 52,022 low-income people \u{2014} a \
                    ratio of 96,587 to one, measured against a goal of 20,000 to one.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-shortage-designations-1985-2026.yml", "**The mental health ratio is enormous because the provider count is a fraction.** 96,587 to one is 0.5386 full-time-equivalent psychiatrists against 52,022 low-income people in Mental Health Catchment Area 62, and the goal it is measured against is 20,000 to one. [verified] \u{2014} the same source. The county has six psychiatrists in all; see [the workforce](allen-county-health-workforce-2023.yml). The catchment area is not the county."),
        ],
        answers: &["cannot say whether anyone went without care"],
        figures: &[
            Figure { label: "full-time-equivalent psychiatrists", value: 0.5386, literal: "0.5386" },
            Figure { label: "low-income people covered", value: 52022.0, literal: "52,022" },
            Figure { label: "people per psychiatrist", value: 96587.0, literal: "96,587" },
        ],
    },
    Assertion {
        id: "the-map-came-down-after-forty-one-years",
        statement: "South Side Lima \u{2014} census tracts 136, 137 and 138 \u{2014} was a shortage \
                    area on the map from 4 April 1985. Its dental designation was withdrawn on 2 \
                    October 1995 covering 10,234 people, and its primary care designation on 1 July \
                    2026 covering 4,593.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-shortage-designations-1985-2026.yml", "**The map that stood for forty-one years came down on 1 July 2026.** `South Side Lima` was a geographic designation over census tracts 136, 137 and 138, made on 4 April 1985. Its dental half was withdrawn on 2 October 1995, covering 10,234 people of whom 32.0 per cent were below the poverty line; its primary care half was withdrawn on **1 July 2026**, covering 4,593 people of whom 29.5 per cent were. [verified] \u{2014} the same register, withdrawn rows retained with their dates. For most of the period this corpus covers, the federal record of shortage here was three tracts of a city. It is now a list of the poor of a county. [inference]"),
        ],
        answers: &["cannot say whether anyone went without care"],
        figures: &[
            Figure { label: "people covered, dental, at withdrawal", value: 10234.0, literal: "10,234" },
            Figure { label: "people covered, primary care, at withdrawal", value: 4593.0, literal: "4,593" },
        ],
    },
    Assertion {
        id: "the-prison-is-designated-on-its-own",
        statement: "Allen Oakwood Correctional Institution carries shortage designations of its own \
                    in all three disciplines, and the primary care one records 0.0000 full-time \
                    equivalents for 1,500 people.",
        topic: "health",
        supports: &[
            support!("measure/allen-county-shortage-designations-1985-2026.yml", "**The prison is designated on its own, in all three disciplines.** Allen Oakwood Correctional Institution at 2338 North West Street carries a dental designation from 22 September 2011 at 0.6075 full-time equivalents for 1,663 people, a mental health designation from 14 March 2022 at 0.675, and a primary care designation from 15 November 2018 that records **0.0000 full-time equivalents for 1,500 people**. [verified] \u{2014} the same register. A zero in that column is what the file holds and may be an absence of reporting rather than an absence of clinicians. [inference]"),
        ],
        answers: &["cannot say whether anyone went without care"],
        figures: &[
            Figure { label: "people covered", value: 1500.0, literal: "1,500" },
        ],
    },
    Assertion {
        id: "fifty-nine-thousand-four-hundred-and-thirty-three-buildings",
        statement: "Allen County holds 59,433 buildings and their footprints cover 185,800,263 \
                    square feet \u{2014} 4,265 acres, one and two thirds per cent of the county's \
                    402.545 square miles. That is one building for every 1.72 residents.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**Fifty-nine thousand four hundred and thirty-three buildings stand in Allen County, and together they cover one and two thirds per cent of it.** 185,800,263 square feet of footprint is 4,265 acres against the county's 402.545 square miles. [verified] \u{2014} [USA Structures](../../catalog/fema-usa-structures.md) against [the land area](allen-county-land-area-2020.yml). That is one building for every 1.72 residents."),
        ],
        answers: &["cannot say when any of these buildings went up"],
        figures: &[
            Figure { label: "square feet of footprint", value: 185800263.0, literal: "185,800,263" },
            Figure { label: "acres", value: 4265.0, literal: "4,265" },
            Figure { label: "residents per building", value: 1.72, literal: "1.72" },
        ],
    },
    Assertion {
        id: "industry-is-a-fortieth-and-a-seventh",
        statement: "Allen County's 1,591 industrial buildings average 16,546 square feet against \
                    2,276 for a residential one, so 2.68 per cent of its buildings hold 14.17 per \
                    cent of its enclosed area.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**Industry is a fortieth of the buildings and a seventh of the floor.** 1,591 industrial structures average 16,546 square feet against 2,276 for a residential one, so 2.68 per cent of the county's buildings hold 14.17 per cent of its enclosed area. [verified] \u{2014} the same source, summed here."),
        ],
        answers: &["cannot say when any of these buildings went up"],
        figures: &[
            Figure { label: "industrial buildings", value: 1591.0, literal: "1,591" },
            Figure { label: "mean industrial sq ft", value: 16546.0, literal: "16,546" },
            Figure { label: "share of floor area, per cent", value: 14.17, literal: "14.17" },
        ],
    },
    Assertion {
        id: "a-structure-is-not-a-housing-unit",
        statement: "An aerial survey finds 48,336 dwelling structures in Allen County where the 2020 \
                    census counted 44,563 housing units: 43,753 single family dwellings at a median \
                    2,000 square feet, 2,604 multi-family and 1,979 manufactured homes.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**The file finds 48,336 dwelling structures where the census counted 44,563 housing units.** 43,753 single family dwellings at a median 2,000 square feet, 2,604 multi-family at a median 2,813, and 1,979 manufactured homes at a median 1,706. [verified] \u{2014} the same source, against [the 2020 housing units](allen-county-housing-units-2020.yml). A structure is not a unit in either direction \u{2014} one multi-family roof holds many units, and a unit above a shop is filed under Commercial \u{2014} so the near-agreement of the two totals is a coincidence of opposite errors. [inference]"),
        ],
        answers: &["cannot say when any of these buildings went up"],
        figures: &[
            Figure { label: "dwelling structures", value: 48336.0, literal: "48,336" },
            Figure { label: "housing units, 2020", value: 44563.0, literal: "44,563" },
            Figure { label: "single family dwellings", value: 43753.0, literal: "43,753" },
        ],
    },
    Assertion {
        id: "thirty-four-cellblocks-filed-as-houses",
        statement: "Thirty-four of Allen County's thirty-five buildings classed Residential / \
                    Institutional Dormitory are cellblocks at the two state prisons north of Lima. \
                    The largest is 63,770 square feet.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**Thirty-four of the county's thirty-five institutional dormitories are cellblocks.** They stand in the two census blocks north of Lima that hold [Allen Correctional](../site/allen-correctional-institution.yml) and [Oakwood](../site/oakwood-correctional-facility.yml), and the file classes every one of them *Residential*. [verified] \u{2014} the same source, by primary occupancy and coordinate. The thirty-fifth stands in Lima."),
        ],
        answers: &["cannot say when any of these buildings went up"],
        figures: &[],
    },
    Assertion {
        id: "thirteen-subdivisions-partition-the-roofs",
        statement: "Lima and the twelve townships hold 59,433 buildings between them with nothing \
                    left over. Lima has 15,766 of them \u{2014} 26.5 per cent of the county's \
                    buildings on 3.4 per cent of its ground, at 1,141 to the square mile against \
                    Amanda Township's 53.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**The thirteen subdivisions partition the county's buildings exactly.** Lima and the twelve townships sum to 59,433 with nothing left over, and Lima holds 15,766 of them \u{2014} 26.5 per cent of the buildings on 3.4 per cent of the ground, at 1,141 to the square mile against American Township's 306 and Amanda's 53. [verified] \u{2014} the same source against [the county subdivision file](../../catalog/census-tiger-roads.md), each centroid tested for containment."),
        ],
        answers: &["cannot say when any of these buildings went up"],
        figures: &[
            Figure { label: "Lima's buildings", value: 15766.0, literal: "15,766" },
            Figure { label: "share of the county's buildings, per cent", value: 26.5, literal: "26.5" },
            Figure { label: "per square mile in Lima", value: 1141.0, literal: "1,141" },
        ],
    },
    Assertion {
        id: "the-largest-roof-in-the-county",
        statement: "The largest building in Allen County covers 1,865,159 square feet \u{2014} 42.8 \
                    acres under one roof \u{2014} and the file that draws it gives it no name. The \
                    corpus's coordinate for the Ford Lima Engine Plant lies 104 metres from it and \
                    nothing else within 150 metres of that point is bigger than 2,129 square feet.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**The largest building in Allen County covers 42.8 acres and the file does not name it.** 1,865,159 square feet, classed *Industrial / Light*, with no address in the columns taken. The corpus's coordinate for [the Ford Lima Engine Plant](../site/ford-lima-engine-plant.yml) lies 104 metres from it, and the only other buildings within 150 metres of that point are 2,129 and 609 square feet. [verified] \u{2014} the same source, queried by point. On that evidence the county's largest building is the engine plant. [inference]"),
            support!("site/ford-lima-engine-plant.yml", "**It is almost certainly the largest building in Allen County.** The county's biggest footprint is 1,865,159 square feet \u{2014} 42.8 acres under one roof \u{2014} classed *Industrial / Light* and carrying no name. This node's coordinate lies 104 metres from it, and the only other buildings within 150 metres of that point are 2,129 and 609 square feet. [verified] \u{2014} [USA Structures](../../catalog/fema-usa-structures.md); see [the buildings](../measure/allen-county-buildings-2019.yml). Nothing else of any size stands near the point, so the identification is the corpus's rather than the file's. [inference]"),
        ],
        answers: &["cannot say when any of these buildings went up"],
        figures: &[
            Figure { label: "square feet", value: 1865159.0, literal: "1,865,159" },
            Figure { label: "acres", value: 42.8, literal: "42.8" },
            Figure { label: "metres from the plant's coordinate", value: 104.0, literal: "104" },
        ],
    },
    Assertion {
        id: "eight-of-ten-coordinates-hit-no-building",
        statement: "Eight of the ten site coordinates this corpus already held fall inside no \
                    building at all \u{2014} the courthouse, both prisons, the tank plant, the \
                    refinery, the engine plant, the depot and the quarry. Memorial Hall's lands in a \
                    17,560-square-foot building the file calls Commercial.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**Eight of the ten coordinates this corpus already held fall inside no building.** The courthouse, both prisons, the tank plant, the refinery, the engine plant, the depot and the quarry all miss; [the Justice Center](../site/allen-county-justice-center.yml) lands in a 27,927-square-foot building the file calls Government, and [Memorial Hall](../site/lima-memorial-hall.yml) lands in a 17,560-square-foot one it calls Commercial. [verified] \u{2014} the same source, ten point queries. A coordinate of record is a label on a place, not a position on a roof; see [an address of record is not a residence](../../decisions/an-address-of-record-is-not-a-residence.yml)."),
        ],
        answers: &["cannot say when any of these buildings went up"],
        figures: &[
            Figure { label: "square feet at Memorial Hall's point", value: 17560.0, literal: "17,560" },
        ],
    },
    Assertion {
        id: "more-buildings-than-addresses",
        statement: "There are 8,723 more buildings in Allen County than there are address points: \
                    59,433 against the 50,710 the county publishes. Barns, machine sheds and \
                    detached garages have a roof and no street number.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**There are 8,723 more buildings here than there are addresses.** 59,433 against the 50,710 address points the county publishes. [verified] \u{2014} the same source against [the county's address file](../../catalog/allen-county-gis-downloads.md). Barns, machine sheds and detached garages over 450 square feet have a roof and no street number. [inference]"),
        ],
        answers: &["cannot say when any of these buildings went up"],
        figures: &[
            Figure { label: "more buildings than addresses", value: 8723.0, literal: "8,723" },
            Figure { label: "address points", value: 50710.0, literal: "50,710" },
        ],
    },
    Assertion {
        id: "two-hundred-and-fifty-eight-religious-roofs",
        statement: "An aerial survey finds 258 religious buildings in Allen County against 160 \
                    congregations, and 217 school buildings against a district roster that has never \
                    said where a building stood. A congregation can own a hall and a manse, and a \
                    church that closed still has a church-shaped roof.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-2019.yml", "**Two counts the corpus already held come out higher here, and both should.** The file finds 258 religious buildings against 160 congregations, and 217 Pre-K\u{2013}12 school buildings against a district roster that has never named where a building stood. [verified] \u{2014} the same source against [the congregations](allen-county-congregations-2020.yml) and [the school buildings](allen-county-school-buildings-1986-2024.yml). A congregation can own a hall and a manse, and a church that closed still has a church-shaped roof."),
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**Something now says where school buildings stand, and it is not a roster.** An automated read of aerial photography finds 217 buildings in this county whose roofs it classes *Pre-K \u{2013} 12 Schools*, each with a coordinate. [verified] \u{2014} [USA Structures](../../catalog/fema-usa-structures.md); see [the buildings](allen-county-buildings-2019.yml). It names none of them, it attaches none of them to a district, and it counts a gymnasium and a bus garage beside the schoolhouse they stand by, so 217 is not five times the 43 the districts report. The refusal above stands, and what is new is that the county's school roofs now have coordinates at all. [inference]"),
        ],
        answers: &["cannot say when any of these buildings went up", "cannot say where any of these buildings stood"],
        figures: &[
            Figure { label: "religious buildings", value: 258.0, literal: "258" },
            Figure { label: "congregations", value: 160.0, literal: "160" },
            Figure { label: "school buildings on the imagery", value: 217.0, literal: "217" },
        ],
    },
    Assertion {
        id: "nine-hundred-and-six-buildings-in-the-floodplain",
        statement: "906 of Allen County's 59,433 buildings stand inside the special flood hazard \
                    area \u{2014} one and a half per cent of them \u{2014} and their footprints are \
                    1,948,457 square feet, 1.05 per cent of the county's enclosed area.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**Nine hundred and six of the county's 59,433 buildings stand inside the special flood hazard area \u{2014} one and a half per cent of them, on six per cent of the ground.** [verified] \u{2014} [USA Structures](../../catalog/fema-usa-structures.md) against [the mapped floodplain](allen-county-flood-hazard-2026.yml), every building centroid tested against the dissolved hazard area in EPSG:26916. Their footprints come to 1,948,457 square feet, 1.05 per cent of the county's enclosed area."),
        ],
        answers: &["cannot say which of these buildings would take water"],
        figures: &[
            Figure { label: "square feet of footprint in the hazard area", value: 1948457.0, literal: "1,948,457" },
            Figure { label: "share of enclosed area, per cent", value: 1.05, literal: "1.05" },
        ],
    },
    Assertion {
        id: "the-county-built-away-from-its-water",
        statement: "6.06 per cent of Allen County's land is mapped floodplain and 1.52 per cent of \
                    its buildings are on it \u{2014} a ratio of three to one.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**This county built away from its water, and the ratio is three to one.** 6.06 per cent of the land is mapped floodplain and 1.52 per cent of the buildings are on it. [inference] \u{2014} the same two sources."),
        ],
        answers: &["cannot say which of these buildings would take water"],
        figures: &[
            Figure { label: "share of land, per cent", value: 6.06, literal: "6.06" },
            Figure { label: "share of buildings, per cent", value: 1.52, literal: "1.52" },
        ],
    },
    Assertion {
        id: "thirty-eight-of-a-hundred-metals-buildings",
        statement: "Thirty-eight of Allen County's hundred metals and minerals processing buildings \
                    stand in the floodplain \u{2014} the highest share of any occupancy in the \
                    county by a factor of seven.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**Thirty-eight of the county's hundred metals and minerals processing buildings are in the floodplain.** [verified] \u{2014} the same source, by primary occupancy. Heavy processing needs water and went to it, and 38 per cent is the highest share of any occupancy in the county by a factor of seven."),
        ],
        answers: &["cannot say which of these buildings would take water"],
        figures: &[],
    },
    Assertion {
        id: "a-manufactured-home-is-twice-as-likely",
        statement: "62 of Allen County's 1,979 manufactured homes are in the floodplain against 594 \
                    of its 43,753 single family dwellings \u{2014} 3.13 per cent against 1.36.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**A manufactured home here is more than twice as likely to be in the floodplain as a house.** 62 of 1,979 against 594 of 43,753 \u{2014} 3.13 per cent against 1.36. [verified] \u{2014} the same source."),
        ],
        answers: &["cannot say which of these buildings would take water"],
        figures: &[
            Figure { label: "manufactured homes in the floodplain", value: 62.0, literal: "62" },
            Figure { label: "per cent of manufactured homes", value: 3.13, literal: "3.13" },
            Figure { label: "per cent of single family dwellings", value: 1.36, literal: "1.36" },
        ],
    },
    Assertion {
        id: "thirteen-hundred-and-seventy-six-people",
        statement: "Weighting each census block by the share of its residential buildings inside the \
                    hazard area puts 1,376 people and 628 housing units in Allen County's \
                    floodplain, against the 4,448 and 1,945 an area weight gave.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**Weighted by buildings instead of by ground, 1,376 people live in the floodplain.** That is 1.35 per cent of the county, against 4,448 and 4.35 per cent when each block is weighted by the share of its area inside \u{2014} and it falls inside the bracket of 235 to 23,721 this corpus published and near the bottom of it. Housing behaves the same way: 628 units against an area-weighted 1,945. [verified] \u{2014} [the 2020 blocks](../../catalog/census-tiger-roads.md), each block's population apportioned by the share of its residential buildings inside the hazard area. See [weight a crosswalk by what it carries](../../decisions/weight-a-crosswalk-by-what-it-carries.yml), whose second case this is: the weight should be the thing being counted, and people live in buildings rather than in acres."),
            support!("measure/allen-county-flood-hazard-2026.yml", "**The bracket is now a figure, and it is near the bottom of it.** Weighting each block by the share of its residential buildings inside the hazard area rather than by the share of its acres gives **1,376 people and 628 housing units**, against the 4,448 and 1,945 an area weight gave. [verified] \u{2014} [USA Structures](../../catalog/fema-usa-structures.md); see [the buildings in the floodplain](allen-county-buildings-in-the-floodplain-2026.yml), which counts 906 of the county's 59,433 buildings inside this boundary. The refusal above stands: a crosswalk weighted by buildings is a better estimate and is still not a count of people. [inference]"),
        ],
        answers: &["cannot say which of these buildings would take water", "cannot say how many people live in the floodplain"],
        figures: &[
            Figure { label: "people, building-weighted", value: 1376.0, literal: "1,376" },
            Figure { label: "housing units, building-weighted", value: 628.0, literal: "628" },
            Figure { label: "people, area-weighted", value: 4448.0, literal: "4,448" },
        ],
    },
    Assertion {
        id: "the-crosswalk-leaves-a-hundred-and-eight-blocks",
        statement: "All 59,433 of Allen County's buildings fall inside one of its 3,552 census \
                    blocks. 2,804 blocks hold at least one residential building; 108 hold 1,881 \
                    people between them and no residential building at all, and those people cannot \
                    be weighted.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**The crosswalk closes on the county and leaves 108 blocks behind.** All 59,433 buildings fall inside one of the county's 3,552 blocks; 2,804 blocks hold at least one residential building, and 108 blocks hold 1,881 people between them and no residential building at all. Those people are unweightable and are outside the 1,376. [verified] \u{2014} the same file."),
        ],
        answers: &["cannot say which of these buildings would take water"],
        figures: &[
            Figure { label: "blocks with no residential building", value: 108.0, literal: "108" },
            Figure { label: "people in them", value: 1881.0, literal: "1,881" },
            Figure { label: "blocks with at least one", value: 2804.0, literal: "2,804" },
        ],
    },
    Assertion {
        id: "elida-and-delphos-run-the-other-way",
        statement: "Elida has 13.70 per cent of its land in the floodplain and 0.4 per cent of its \
                    buildings. Delphos has 13.10 per cent of its land and 17.7 per cent of its \
                    buildings \u{2014} the only place in Allen County where the building share is \
                    the higher of the two.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**A share of a village's ground is not a share of its buildings, and the two do not even run in the same order.** Elida has 13.70 per cent of its land in the floodplain and 0.4 per cent of its buildings; Delphos has 13.10 per cent of its land and 17.7 per cent of its buildings, the only place in the county where the building share is the higher of the two. [verified] \u{2014} the same source against [the mapped floodplain](allen-county-flood-hazard-2026.yml)'s own land shares."),
        ],
        answers: &["cannot say which of these buildings would take water"],
        figures: &[
            Figure { label: "Elida's land, per cent", value: 13.7, literal: "13.70" },
            Figure { label: "Delphos's land, per cent", value: 13.1, literal: "13.10" },
            Figure { label: "Delphos's buildings, per cent", value: 17.7, literal: "17.7" },
        ],
    },
    Assertion {
        id: "five-villages-with-none",
        statement: "Spencerville, Cairo, Beaverdam, Harrod and Lafayette have no building in the \
                    mapped floodplain at all. Among the townships the range runs from Marion at 8.0 \
                    per cent and Sugar Creek at 7.7 down to Spencer at none.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**Five of the county's villages have no building in the floodplain at all.** Spencerville, Cairo, Beaverdam, Harrod and Lafayette. [verified] \u{2014} the same source. Among the townships the range runs from Marion at 8.0 per cent and Sugar Creek at 7.7 down to Spencer at none."),
        ],
        answers: &["cannot say which of these buildings would take water"],
        figures: &[
            Figure { label: "Marion Township, per cent", value: 8.0, literal: "8.0" },
            Figure { label: "Sugar Creek, per cent", value: 7.7, literal: "7.7" },
        ],
    },
    Assertion {
        id: "one-claim-for-every-nine-buildings-inside",
        statement: "One paid flood insurance claim for every 8.7 buildings inside Allen County's \
                    mapped floodplain, against one for every 552 outside it. Among policyholders the \
                    outside-rated claimed more; among buildings the floodplain is sixty-three times \
                    the risk.",
        topic: "land",
        supports: &[
            support!("measure/allen-county-buildings-in-the-floodplain-2026.yml", "**The claim record turns into a rate per building, which is what it never was.** One paid flood insurance claim for every 8.7 buildings inside the mapped floodplain, against one for every 552 outside it. [inference] \u{2014} [the claims](allen-county-flood-insurance-1978-2023.yml), 104 rated inside and 106 rated outside, against the counts here. Among policyholders the outside-rated produced more claims than the inside-rated; among buildings the floodplain is sixty-three times the risk, and the difference between those two sentences is who bought a policy. See [a compulsory denominator is not a voluntary one](../../decisions/a-compulsory-denominator-is-not-a-voluntary-one.yml)."),
            support!("measure/allen-county-flood-insurance-1978-2023.yml", "**That count now exists, and among buildings the map is in the right place.** 58,527 of Allen County's 59,433 buildings stand outside the special flood hazard area and 906 stand inside it, so the 104 claims rated inside are one for every 8.7 buildings and the 106 rated outside are one for every 552. [verified] \u{2014} [USA Structures](../../catalog/fema-usa-structures.md); see [the buildings in the floodplain](allen-county-buildings-in-the-floodplain-2026.yml). Per policyholder the outside-rated claimed more; per building the floodplain is sixty-three times the risk. Both are true, and the recruitment rule above is the whole distance between them. [inference]"),
        ],
        answers: &["cannot say which of these buildings would take water", "cannot say whether Allen County's flood map is drawn in the wrong place"],
        figures: &[
            Figure { label: "buildings per claim inside", value: 8.7, literal: "8.7" },
            Figure { label: "buildings per claim outside", value: 552.0, literal: "552" },
            Figure { label: "buildings outside the hazard area", value: 58527.0, literal: "58,527" },
        ],
    },
    Assertion {
        id: "no-building-stands-where-the-tank-plant-is-mapped",
        statement: "No building at all stands within a hundred and fifty metres of the coordinate \
                    this corpus holds for the Lima Army Tank Plant. The nearest large footprint is \
                    993,868 square feet \u{2014} 22.8 acres, the third largest in the county \
                    \u{2014} and it lies 632 metres away.",
        topic: "land",
        supports: &[
            support!("site/lima-army-tank-plant.yml", "**No building stands within a hundred and fifty metres of this node's coordinate.** The nearest large footprint is 993,868 square feet \u{2014} 22.8 acres, classed *Government / Non-Civilian Structures*, the third largest in the county \u{2014} and it lies 632 metres away. [verified] \u{2014} [USA Structures](../../catalog/fema-usa-structures.md), queried by point; see [the buildings](../measure/allen-county-buildings-2019.yml). A coordinate of record for a plant of this size names the installation and not a roof on it; see [a photograph is not a register](../../decisions/a-photograph-is-not-a-register.yml)."),
        ],
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[
            Figure { label: "square feet", value: 993868.0, literal: "993,868" },
            Figure { label: "acres", value: 22.8, literal: "22.8" },
            Figure { label: "metres away", value: 632.0, literal: "632" },
        ],
    },
    Assertion {
        id: "forty-five-governments",
        statement: "Forty-five governments operate in Allen County. They employ 3,806 people full \
                    time and 984 part time, and their full-time payroll for March 2022 was \
                    $17,986,357.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-governments-and-their-employees-2022.yml", "**Forty-five governments operate in Allen County and they employ 3,806 people full time and 984 part time.** Their full-time payroll for March 2022 was $17,986,357 and their part-time payroll $921,759. [verified] \u{2014} [the 2022 Census of Governments employment file](../../catalog/census-public-employment-and-payroll.md), every unit with FIPS county 39003."),
        ],
        answers: &["cannot say what any of these people are paid"],
        figures: &[
            Figure { label: "full-time employees", value: 3806.0, literal: "3,806" },
            Figure { label: "part-time employees", value: 984.0, literal: "984" },
            Figure { label: "March full-time payroll, dollars", value: 17986357.0, literal: "17,986,357" },
        ],
    },
    Assertion {
        id: "three-governments-are-half-the-payroll",
        statement: "Allen County government employs 969 people full time, Lima City School District \
                    657 and the City of Lima 402 \u{2014} 2,028 of the county's 3,806 full-time \
                    local government employees, or 53.3 per cent.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-governments-and-their-employees-2022.yml", "**Three governments are more than half of it.** Allen County itself employs 969 full time, Lima City School District 657 and the City of Lima 402 \u{2014} 2,028 of the 3,806, or 53.3 per cent. [verified] \u{2014} the same file. The county government alone is a quarter of every full-time local government job in the county."),
        ],
        answers: &["cannot say what any of these people are paid"],
        figures: &[
            Figure { label: "Allen County government", value: 969.0, literal: "969" },
            Figure { label: "Lima City Schools", value: 657.0, literal: "657" },
            Figure { label: "City of Lima", value: 402.0, literal: "402" },
            Figure { label: "share, per cent", value: 53.3, literal: "53.3" },
        ],
    },
    Assertion {
        id: "the-townships-are-mostly-part-time",
        statement: "Allen County's twelve townships employ 117 people full time and 123 part time \
                    \u{2014} the only class of government in the county with more part-time \
                    employees than full-time. Seven of the twelve employ two people or fewer full \
                    time.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-governments-and-their-employees-2022.yml", "**The twelve townships are the only class of government here with more part-time employees than full-time.** 117 against 123. [verified] \u{2014} the same file. Seven of the twelve employ two people or fewer full time and Jackson Township employs none at all."),
        ],
        answers: &["cannot say what any of these people are paid"],
        figures: &[
            Figure { label: "full-time", value: 117.0, literal: "117" },
            Figure { label: "part-time", value: 123.0, literal: "123" },
        ],
    },
    Assertion {
        id: "four-governments-with-no-employees",
        statement: "Eight of Allen County's forty-five governments have no full-time employee and \
                    four have no employee at all: the Jackson Township Park District, the Allen \
                    County Schools Health Benefit Plan, the Auglaize Township Park District and the \
                    Allen County Transportation Improvement District.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-governments-and-their-employees-2022.yml", "**Eight of the forty-five have no full-time employee and four have nobody at all.** Cairo, Lafayette, Jackson Township, the Jackson Township Park District, the Allen Water District, the Allen County Schools Health Benefit Plan, the Auglaize Township Park District and the Allen County Transportation Improvement District; the last four report no part-time employee either. [verified] \u{2014} the same file. A government is a legal body with a levy and a board, and four of this county's have no payroll to run."),
        ],
        answers: &["cannot say what any of these people are paid"],
        figures: &[],
    },
    Assertion {
        id: "eleven-special-districts",
        statement: "Eleven special districts operate in Allen County. The largest is the Allen \
                    County Regional Transit Authority at 30 full-time employees, then the \
                    Metropolitan Housing Authority at 20 and the Johnny Appleseed Metropolitan Park \
                    District at 19.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-governments-and-their-employees-2022.yml", "**Eleven special districts operate here and the corpus had named none of them.** The Allen County Regional Transit Authority is the largest at 30 full time, then the Allen County Metropolitan Housing Authority at 20 and the Johnny Appleseed Metropolitan Park District at 19; the others are the Allen Soil and Water Conservation District, the Allen Water District, the Allen County Regional Airport Authority, the North Central Ohio Solid Waste Management District, the Allen County Schools Health Benefit Plan, the Allen County Transportation Improvement District and two township park districts. [verified] \u{2014} the same file, unit type 4."),
        ],
        answers: &["cannot say what any of these people are paid"],
        figures: &[
            Figure { label: "transit authority", value: 30.0, literal: "30" },
            Figure { label: "housing authority", value: 20.0, literal: "20" },
            Figure { label: "park district", value: 19.0, literal: "19" },
        ],
    },
    Assertion {
        id: "what-the-county-government-does",
        statement: "Allen County government's largest named function is judicial and legal at 125 \
                    full-time employees, then health at 124, public welfare at 118, corrections at \
                    111 and sworn police at 86. The City of Lima's largest are sworn police at 73 \
                    and firefighters at 65.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-governments-and-their-employees-2022.yml", "**What the county government does with 969 people.** The largest named function is judicial and legal at 125, then health at 124, public welfare at 118, corrections at 111 and sworn police at 86; 170 sit in the file's residual category. [verified] \u{2014} the same file, by item code. The City of Lima's largest are sworn police at 73 and firefighters at 65."),
        ],
        answers: &["cannot say what any of these people are paid"],
        figures: &[
            Figure { label: "judicial and legal", value: 125.0, literal: "125" },
            Figure { label: "health", value: 124.0, literal: "124" },
            Figure { label: "corrections", value: 111.0, literal: "111" },
            Figure { label: "Lima police, sworn", value: 73.0, literal: "73" },
        ],
    },
    Assertion {
        id: "one-jailer-for-every-one-and-a-half-held",
        statement: "Allen County's local governments employ 123 people in corrections \u{2014} 111 \
                    for the county and 12 for the City of Lima \u{2014} against an average daily \
                    jail population of 186.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-governments-and-their-employees-2022.yml", "**The county's local governments employ 123 people in corrections and hold 186.** 111 of the 123 work for the county and 12 for the City of Lima, against an average daily jail population of 186 \u{2014} one worker for every one and a half people held. [inference] \u{2014} the same file against [the jail](allen-county-jail-1970-2023.yml). The two state prisons north of Lima are not in this file at all; see [the government payroll](allen-county-government-employment-2014-2024.yml)."),
        ],
        answers: &["cannot say what any of these people are paid"],
        figures: &[
            Figure { label: "corrections employees", value: 123.0, literal: "123" },
            Figure { label: "average daily jail population", value: 186.0, literal: "186" },
        ],
    },
    Assertion {
        id: "seven-governments-did-not-report",
        statement: "Twenty-eight of Allen County's forty-five governments reported their full-time \
                    employment directly for 2022, eight reported it with an analyst correction, two \
                    had a unit total pro-rated, and seven did not report at all \u{2014} among them \
                    Lima City School District, whose 657 employees are the previous year's figure \
                    grown by a rate.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-governments-and-their-employees-2022.yml", "**Seven of the forty-five did not report, and one of the seven is the second largest.** Twenty-eight governments reported their full-time employment directly, eight reported it with an analyst correction, two reported a unit total the Bureau pro-rated across functions, and seven \u{2014} Elida, Jackson Township, Richland Township, the Metropolitan Housing Authority, the Auglaize Township Park District, **Lima City School District** and Apollo Career Center \u{2014} carry the previous year's figure grown by a rate. [verified] \u{2014} the same file, the data flag at position 32. 789 of the 3,806 full-time employees are in those seven units, and 657 of the 789 are the one school district."),
        ],
        answers: &["cannot say what any of these people are paid"],
        figures: &[
            Figure { label: "did not report", value: 7.0, literal: "7" },
            Figure { label: "full-time employees in those seven", value: 789.0, literal: "789" },
        ],
    },
    Assertion {
        id: "one-job-in-nine-is-a-government-job",
        statement: "One job in nine in Allen County is a government job and has been for eleven \
                    years. Over those eleven years the county's total covered employment fell by 124 \
                    and its private employment rose by 85.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**One job in nine in Allen County is a government job, and it has been for eleven years.** [verified] \u{2014} [the Quarterly Census of Employment and Wages](../../catalog/bls-qcew.md), county files 2014 to 2024, aggregation level 71, ownership codes 1 federal, 2 state, 3 local and 5 private."),
            support!("measure/allen-county-government-employment-2014-2024.yml", "**The federal payroll here has not moved at all.** 321 people in 2014 and 321 in 2024, and in no year of the eleven fewer than 302 or more than 331. [verified] \u{2014} the same source. Over the same years the county's total covered employment fell by 124 and its private employment rose by 85."),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "fall in covered employment", value: 124.0, literal: "124" },
            Figure { label: "rise in private employment", value: 85.0, literal: "85" },
        ],
    },
    Assertion {
        id: "three-hundred-and-twenty-one-federal-jobs-twice",
        statement: "Allen County had 321 federal jobs in 2014 and 321 in 2024, and in no year of the \
                    eleven fewer than 302 or more than 331.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**The federal payroll here has not moved at all.** 321 people in 2014 and 321 in 2024, and in no year of the eleven fewer than 302 or more than 331. [verified] \u{2014} the same source. Over the same years the county's total covered employment fell by 124 and its private employment rose by 85."),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "federal jobs", value: 321.0, literal: "321" },
            Figure { label: "lowest year", value: 302.0, literal: "302" },
            Figure { label: "highest year", value: 331.0, literal: "331" },
        ],
    },
    Assertion {
        id: "the-post-office-is-the-federal-government-here",
        statement: "The largest federal presence in Allen County is the postal service: 155 \
                    employees across seven establishments at $74,566 a year. After it come 70 in \
                    national security at one establishment, 38 in offices of physicians at one, and \
                    eight in public finance.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**The largest federal presence in this county is the post office.** 155 employees across seven establishments in 2023, at $74,566 a year. [verified] \u{2014} the same file at NAICS 491110. After it: 70 people in national security at one establishment, 38 in offices of physicians at one, and eight in public finance. [verified] \u{2014} the same file, NAICS 928110, 621111 and 921130. The national security establishment is the federal cadre at [the tank plant](../site/lima-army-tank-plant.yml), which is government-owned and contractor-operated, so the people who build vehicles there are private employment counted under manufacturing. [inference]"),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "postal employees", value: 155.0, literal: "155" },
            Figure { label: "national security", value: 70.0, literal: "70" },
            Figure { label: "offices of physicians", value: 38.0, literal: "38" },
        ],
    },
    Assertion {
        id: "seventy-people-on-the-federal-payroll-at-the-plant",
        statement: "Allen County has one federal establishment classified under national security, \
                    employing 70 people at $87,612 a year. It is the tank plant's federal cadre and \
                    not its workforce: the plant is government-owned and contractor-operated, and \
                    the people who build vehicles there are private employment counted under \
                    manufacturing.",
        topic: "work",
        supports: &[
            support!("site/lima-army-tank-plant.yml", "**Seventy people here are on the federal payroll.** Allen County has one federal establishment classified under national security, employing 70 at $87,612 a year in 2023. [verified] \u{2014} [the wage file](../../catalog/bls-qcew.md), NAICS 928110; see [the government payroll](../measure/allen-county-government-employment-2014-2024.yml). This is the plant's federal cadre and not its workforce: a government-owned, contractor-operated plant pays the people who build vehicles through the operating contractor, and they are private employment counted under manufacturing. [inference]"),
        ],
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[
            Figure { label: "federal employees", value: 70.0, literal: "70" },
        ],
    },
    Assertion {
        id: "four-hundred-and-fifty-eight-in-the-prisons",
        statement: "Four hundred and fifty-eight people work in Allen County's two state \
                    correctional institutions, at $82,817 a year \u{2014} half of all state \
                    government employment in the county. The census counted 1,513 people held in \
                    them.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**Four hundred and fifty-eight people work in the county's two state correctional institutions**, at $82,817 a year \u{2014} half of all state government employment here. [verified] \u{2014} the same file, NAICS 922140, two establishments. The census counted 1,513 people in adult correctional facilities in this county, so the two prisons hold about three and a third people for each person they employ. [inference] \u{2014} see [the group quarters](allen-county-group-quarters-2020.yml)."),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "average annual pay, dollars", value: 82817.0, literal: "82,817" },
            Figure { label: "people held", value: 1513.0, literal: "1,513" },
        ],
    },
    Assertion {
        id: "a-road-office-a-college-and-a-courthouse",
        statement: "The rest of the state's presence in Allen County is 156 people in transportation \
                    administration at one establishment, 130 in colleges and universities at one, 98 \
                    in human resource programme administration, 24 in courts at $107,506 and 20 in \
                    state police at $97,879.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**The rest of the state's presence is a road office, a college and a courthouse.** 156 in transportation administration at one establishment, 130 in colleges and universities at one, 98 in human resource programme administration, 24 in courts at $107,506 and 20 in state police at $97,879. [verified] \u{2014} the same file, NAICS 926120, 611310, 923130, 922110 and 922120."),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "transportation administration", value: 156.0, literal: "156" },
            Figure { label: "colleges", value: 130.0, literal: "130" },
            Figure { label: "courts", value: 24.0, literal: "24" },
        ],
    },
    Assertion {
        id: "local-government-pay-fell-behind",
        statement: "A local government job in Allen County paid $39,535 in 2014 against $40,528 for \
                    all covered employment \u{2014} 2.4 per cent below. In 2024 it paid $53,722 \
                    against $58,790, or 8.6 per cent below. Local government pay rose 35.9 per cent \
                    over the eleven years and private pay rose 45.9.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**A local government job in this county has been falling behind the county's own average pay for eleven years.** In 2014 it paid $39,535 against $40,528 for all covered employment \u{2014} 2.4 per cent below. In 2024 it paid $53,722 against $58,790 \u{2014} 8.6 per cent below. Local government pay rose 35.9 per cent over the eleven years and private pay rose 45.9. [verified] \u{2014} the same source, `avg_annual_pay`; the shares are computed here."),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "local pay, 2014", value: 39535.0, literal: "39,535" },
            Figure { label: "local pay, 2024", value: 53722.0, literal: "53,722" },
            Figure { label: "local growth, per cent", value: 35.9, literal: "35.9" },
            Figure { label: "private growth, per cent", value: 45.9, literal: "45.9" },
        ],
    },
    Assertion {
        id: "state-pay-overtook-federal",
        statement: "State government pay in Allen County overtook federal pay in 2024 for the first \
                    time in the series \u{2014} $84,761 against $83,773. Federal pay here went from \
                    154 per cent of the county average to 142; state went from 135 to 144.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**State government pay overtook federal in 2024 for the first time in the series.** $84,761 against $83,773. [verified] \u{2014} the same source. Federal pay in this county went from 154 per cent of the county average to 142; state went from 135 to 144."),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "state pay", value: 84761.0, literal: "84,761" },
            Figure { label: "federal pay", value: 83773.0, literal: "83,773" },
        ],
    },
    Assertion {
        id: "government-takes-a-ninth-of-the-wages",
        statement: "Government took 11.56 per cent of Allen County's jobs and 11.84 per cent of its \
                    wages in 2024 \u{2014} $345,876,208 of $2,921,271,371, across 136 establishments \
                    of 2,586. In 2014 it was 12.74 per cent of the wages.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**Government takes 11.56 per cent of the jobs and 11.84 per cent of the wages.** $345,876,208 of $2,921,271,371 in 2024, across 136 establishments of 2,586. [verified] \u{2014} the same file, `total_annual_wages`. In 2014 it was 12.74 per cent of the wages, and the fall is local government pay rather than local government headcount. [inference]"),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "government wages, dollars", value: 345876208.0, literal: "345,876,208" },
            Figure { label: "share of wages, per cent", value: 11.84, literal: "11.84" },
            Figure { label: "establishments", value: 136.0, literal: "136" },
        ],
    },
    Assertion {
        id: "two-agencies-within-five-per-cent",
        statement: "Two federal agencies counted Allen County's local government payroll in 2022 \
                    from opposite ends and came within five per cent of each other: 4,534 jobs \
                    covered by unemployment insurance, and 3,806 full-time plus 984 part-time \
                    employees of forty-five named governments.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-government-employment-2014-2024.yml", "**Two agencies counted the same local payroll in 2022 and came within five per cent.** This file gives 4,534 local government jobs; the Census Bureau's enumeration of all forty-five governments gives 3,806 full-time and 984 part-time employees, or 4,790. [verified] \u{2014} [the government employment file](../../catalog/census-public-employment-and-payroll.md); see [the forty-five governments](allen-county-governments-and-their-employees-2022.yml). One counts jobs covered by unemployment insurance at an establishment and the other counts the employees of a named government in March, so elected officials, casual part-timers and some elected boards fall on different sides of the two rules. [inference]"),
        ],
        answers: &["cannot say how many people who work for a government here also live here"],
        figures: &[
            Figure { label: "jobs covered by insurance", value: 4534.0, literal: "4,534" },
            Figure { label: "employees of named governments", value: 4790.0, literal: "4,790" },
        ],
    },
    Assertion {
        id: "what-a-business-register-leaves-out",
        statement: "Government employed 5,780 people in Allen County in 2023 against the 44,251 \
                    private employees on the federal business register, so what a register of \
                    businesses leaves out of this county's employment is about one job in nine.",
        topic: "work",
        supports: &[
            support!("measure/allen-county-private-employers-1986-2023.yml", "**The register's complement now has a size.** Government employed 5,780 people in this county in 2023 against the 44,251 private employees in the table above, so what a business register leaves out here is about one job in nine. [verified] \u{2014} [the wage file](../../catalog/bls-qcew.md); see [the government payroll](allen-county-government-employment-2014-2024.yml). The private series above is a private series and was never the county's employment. [inference]"),
        ],
        answers: &["cannot say whether that flatness is wages standing still or hours doing so"],
        figures: &[
            Figure { label: "government employees", value: 5780.0, literal: "5,780" },
            Figure { label: "private employees on the register", value: 44251.0, literal: "44,251" },
        ],
    },
    Assertion {
        id: "nineteen-years-nineteen-losses",
        statement: "71,324 exemptions left Allen County on tax returns between 2004\u{2013}05 and \
                    2022\u{2013}23 and 62,153 arrived \u{2014} a net loss of 9,171 people, with no \
                    year of gain in nineteen.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-flows-2004-2023.yml", "**Nineteen years of tax returns changing address, and Allen County lost people in all nineteen.** 71,324 exemptions left and 62,153 arrived \u{2014} a net loss of 9,171. [verified] \u{2014} [the IRS migration files](../../catalog/irs-county-migration.md), outflow and inflow totals for county 39003, summed here."),
        ],
        answers: &["cannot say why anybody moved"],
        figures: &[
            Figure { label: "exemptions out", value: 71324.0, literal: "71,324" },
            Figure { label: "exemptions in", value: 62153.0, literal: "62,153" },
            Figure { label: "net loss", value: 9171.0, literal: "9,171" },
        ],
    },
    Assertion {
        id: "three-hundred-and-fifty-five-million-left",
        statement: "$1,685,961,000 of adjusted gross income left Allen County with the people who \
                    moved away between 2004\u{2013}05 and 2022\u{2013}23, and $1,330,573,000 arrived \
                    with the people who moved in \u{2014} a net outflow of $355 million.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-flows-2004-2023.yml", "**They took $355 million more out than the arrivals brought in.** $1,685,961,000 of adjusted gross income left with the leavers over nineteen years and $1,330,573,000 arrived with the newcomers. [verified] \u{2014} the same files, aggregate AGI summed across all nineteen pairs."),
        ],
        answers: &["cannot say why anybody moved"],
        figures: &[
            Figure { label: "AGI out, dollars", value: 1685961000.0, literal: "1,685,961,000" },
            Figure { label: "AGI in, dollars", value: 1330573000.0, literal: "1,330,573,000" },
        ],
    },
    Assertion {
        id: "the-leavers-out-earned-the-arrivers",
        statement: "The people who left Allen County out-earned the people who arrived in seventeen \
                    of nineteen years: $23,638 of adjusted gross income per exemption on the way out \
                    against $21,408 on the way in, a gap of 10.4 per cent.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-flows-2004-2023.yml", "**The people who left out-earned the people who arrived in seventeen of the nineteen years.** $23,638 of adjusted gross income per exemption on the way out against $21,408 on the way in \u{2014} a gap of 10.4 per cent over the whole span. Only 2012\u{2013}13 and 2011\u{2013}12 run the other way. [verified] \u{2014} the same source; the per-exemption figures are computed here."),
        ],
        answers: &["cannot say why anybody moved"],
        figures: &[
            Figure { label: "per exemption out, dollars", value: 23638.0, literal: "23,638" },
            Figure { label: "per exemption in, dollars", value: 21408.0, literal: "21,408" },
            Figure { label: "gap, per cent", value: 10.4, literal: "10.4" },
        ],
    },
    Assertion {
        id: "the-income-gap-is-closing",
        statement: "In 2004\u{2013}05 someone leaving Allen County carried $18,626 and someone \
                    arriving $16,469, 13.1 per cent apart. In 2022\u{2013}23 it was $31,862 against \
                    $30,149, 5.7 per cent apart.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-flows-2004-2023.yml", "**The gap has been closing.** In 2004\u{2013}05 a leaver carried $18,626 and an arriver $16,469, 13.1 per cent apart; in 2022\u{2013}23 it was $31,862 against $30,149, 5.7 per cent apart. [verified] \u{2014} the same files. Both figures are nominal and neither is deflated."),
        ],
        answers: &["cannot say why anybody moved"],
        figures: &[
            Figure { label: "out, 2004-05", value: 18626.0, literal: "18,626" },
            Figure { label: "in, 2004-05", value: 16469.0, literal: "16,469" },
            Figure { label: "out, 2022-23", value: 31862.0, literal: "31,862" },
            Figure { label: "in, 2022-23", value: 30149.0, literal: "30,149" },
        ],
    },
    Assertion {
        id: "more-than-half-who-leave-ohio-go-south",
        statement: "Of the 1,221 exemptions that left Allen County for another state in \
                    2022\u{2013}23 without a county large enough to name, 630 went to the South, 327 \
                    to the Midwest, 159 to the West and 105 to the Northeast.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-flows-2004-2023.yml", "**More than half of everyone who leaves Ohio from this county goes South.** Of the 1,221 exemptions in the different-state residue in 2022\u{2013}23, 630 went to the South, 327 to the Midwest, 159 to the West and 105 to the Northeast. [verified] \u{2014} the same file, rows coded 59."),
        ],
        answers: &["cannot say why anybody moved"],
        figures: &[
            Figure { label: "to the South", value: 630.0, literal: "630" },
            Figure { label: "to the Midwest", value: 327.0, literal: "327" },
            Figure { label: "to the West", value: 159.0, literal: "159" },
        ],
    },
    Assertion {
        id: "two-years-that-are-the-file",
        statement: "Migration out of Allen County is a third below its median in 2014\u{2013}15 and \
                    a third above it in 2016\u{2013}17, and Auglaize and Hancock counties move the \
                    same way in the same two years. The movement is in the instrument.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-flows-2004-2023.yml", "**Two of the nineteen years are the file and not the county.** 2014\u{2013}15 is a third below the county's median and 2016\u{2013}17 a third above it, and Auglaize and Hancock counties move the same way in the same two years. [verified] \u{2014} the same files read for three counties; see [the control can be the county next door](../../decisions/the-control-can-be-the-county-next-door.yml). No rate here is computed across those two years alone."),
        ],
        answers: &["cannot say why anybody moved"],
        figures: &[],
    },
    Assertion {
        id: "the-tax-file-finds-a-smaller-loss",
        statement: "Over the three years both cover, the tax files give Allen County a net domestic \
                    migration loss of 686 people and the census population estimates give 1,060. The \
                    difference is people who do not file a return.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-flows-2004-2023.yml", "**A second instrument finds a bigger loss, and the difference is people who do not file.** Over the three years both cover, the tax files give a net domestic loss of 686 and the population estimates give 1,060. [verified] \u{2014} [the population estimates](../../catalog/census-popest-2024.md); see [the net migration](allen-county-net-migration-2021-2024.yml). A tax file sees filers, and the very poor, much of the old and anyone who moved without filing are outside it. [inference]"),
            support!("measure/allen-county-net-migration-2021-2024.yml", "**The domestic half now has an address.** Nineteen years of tax returns changing address name the counties: Franklin County took a net 2,036 people from Allen County between 2004\u{2013}05 and 2022\u{2013}23, nine metropolitan counties took 3,981 between them, and the five counties that touch this one came to a net of eighty-two. [verified] \u{2014} [the IRS migration files](../../catalog/irs-county-migration.md); see [where they went](allen-county-migration-by-county-2004-2023.yml). Over the three years the two sources both cover, the tax file finds a domestic net loss of 686 against these estimates' 1,060, and the difference is people who do not file. [inference]"),
        ],
        answers: &["cannot say why anybody moved"],
        figures: &[
            Figure { label: "tax file loss", value: 686.0, literal: "686" },
            Figure { label: "estimates loss", value: 1060.0, literal: "1,060" },
        ],
    },
    Assertion {
        id: "foreign-migration-stops-being-published",
        statement: "The IRS migration file publishes Allen County's foreign migration for six years \
                    \u{2014} 13, 23, 30, 12, 18 and 23 exemptions out \u{2014} and suppresses it in \
                    every year since 2009\u{2013}10, while the census estimates put international \
                    migration at 424 over 2021 to 2024.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-flows-2004-2023.yml", "**Foreign migration disappears from this file after 2009\u{2013}10.** The rows coded 98 carry figures through the first six years \u{2014} 13, 23, 30, 12, 18 and 23 exemptions out \u{2014} and `-1` in every year since. [verified] \u{2014} the same files. The population estimates put international migration at 424 over 2021\u{2013}24, so the quantity exists and the tax file has stopped saying it."),
        ],
        answers: &["cannot say why anybody moved"],
        figures: &[
            Figure { label: "2006-07 exemptions out", value: 30.0, literal: "30" },
            Figure { label: "international migration, 2021-24", value: 424.0, literal: "424" },
        ],
    },
    Assertion {
        id: "forty-five-counties-and-six-hundred-and-ninety-seven-flows",
        statement: "Forty-five counties are named in nineteen years of Allen County's migration \
                    record, in 697 published flows. Everything smaller than ten returns a year is \
                    suppressed, so the named counties account for 5,329 of a net loss of 9,171 and \
                    the other 3,842 have a region and no county.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-by-county-2004-2023.yml", "**Forty-five counties are named in nineteen years of Allen County's migration record, in 697 published flows.** Everything smaller than ten returns a year is suppressed into a regional residue, so the named counties account for 5,329 of the county's net loss of 9,171 exemptions and the other 3,842 have a region and no county. [verified] \u{2014} [the IRS migration files](../../catalog/irs-county-migration.md), all nineteen pairs."),
        ],
        answers: &["cannot say who any of these people are"],
        figures: &[
            Figure { label: "published flows", value: 697.0, literal: "697" },
            Figure { label: "net loss to named counties", value: 5329.0, literal: "5,329" },
            Figure { label: "net loss with no county", value: 3842.0, literal: "3,842" },
        ],
    },
    Assertion {
        id: "seventeen-thousand-crossings-of-one-county-line",
        statement: "8,987 exemptions left Allen County for Auglaize County over nineteen years and \
                    8,866 arrived from it \u{2014} a churn of 17,853 people across one county line \
                    for a net of 121.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-by-county-2004-2023.yml", "**The largest exchange is with the county next door and it is very nearly even.** 8,987 exemptions left for Auglaize County over the nineteen years and 8,866 arrived from it \u{2014} a churn of 17,853 people across one county line for a net of 121. [verified] \u{2014} the same files."),
        ],
        answers: &["cannot say who any of these people are"],
        figures: &[
            Figure { label: "out", value: 8987.0, literal: "8,987" },
            Figure { label: "in", value: 8866.0, literal: "8,866" },
            Figure { label: "churn", value: 17853.0, literal: "17,853" },
        ],
    },
    Assertion {
        id: "the-border-comes-to-eighty-two",
        statement: "Auglaize, Van Wert, Putnam, Hardin and Hancock \u{2014} the five counties that \
                    touch Allen County \u{2014} come to a net of eighty-two people and a net gain of \
                    $3,581,000 across nineteen years.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-by-county-2004-2023.yml", "**The five counties that touch Allen County take as many people as they give.** Auglaize, Van Wert, Putnam, Hardin and Hancock come to a net of 82 people and a net gain of $3,581,000 across nineteen years. [verified] \u{2014} the same files, the five contiguous counties summed here."),
        ],
        answers: &["cannot say who any of these people are"],
        figures: &[
            Figure { label: "net AGI gain, dollars", value: 3581000.0, literal: "3,581,000" },
        ],
    },
    Assertion {
        id: "the-loss-is-to-the-cities",
        statement: "Franklin, Cuyahoga, Hamilton, Montgomery, Lucas, Summit, Delaware, Butler and \
                    Warren counties take a net 3,981 people and $113,815,000 out of Allen County \
                    over nineteen years \u{2014} three quarters of the named part of its migration \
                    loss.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-by-county-2004-2023.yml", "**The loss is to the cities, and three quarters of the named part of it is nine counties.** Franklin, Cuyahoga, Hamilton, Montgomery, Lucas, Summit, Delaware, Butler and Warren take a net 3,981 people and $113,815,000 out of this county over nineteen years. [verified] \u{2014} the same files, those nine counties summed here. Allen County draws from the countryside and loses to the metropolis. [inference]"),
        ],
        answers: &["cannot say who any of these people are"],
        figures: &[
            Figure { label: "net people", value: 3981.0, literal: "3,981" },
            Figure { label: "net AGI, dollars", value: 113815000.0, literal: "113,815,000" },
        ],
    },
    Assertion {
        id: "columbus-is-a-fifth-of-everything",
        statement: "Franklin County took 4,907 exemptions from Allen County over nineteen years and \
                    sent back 2,871 \u{2014} a net 2,036 people and $48,411,000, which is 22.2 per \
                    cent of the county's whole net migration loss.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-by-county-2004-2023.yml", "**Columbus alone is a fifth of everything.** Franklin County took 4,907 exemptions and sent back 2,871 \u{2014} a net 2,036 people, 22.2 per cent of the county's whole net migration loss, and $48,411,000. [verified] \u{2014} the same files. No other county is within a quarter of it."),
        ],
        answers: &["cannot say who any of these people are"],
        figures: &[
            Figure { label: "out", value: 4907.0, literal: "4,907" },
            Figure { label: "in", value: 2871.0, literal: "2,871" },
            Figure { label: "net", value: 2036.0, literal: "2,036" },
            Figure { label: "share of the loss, per cent", value: 22.2, literal: "22.2" },
        ],
    },
    Assertion {
        id: "the-richest-leavers-go-to-delaware-county",
        statement: "The people who leave Allen County for Delaware County \u{2014} Columbus's \
                    northern suburb and the highest-income county in Ohio \u{2014} carry $38,579 of \
                    adjusted gross income per exemption, against $23,638 for the average departure.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-by-county-2004-2023.yml", "**The people who leave for Delaware County are the richest leavers in the record.** $38,579 of adjusted gross income per exemption, against $23,638 for the average departure from this county. [verified] \u{2014} the same files, the fourteen years in which that flow was large enough to publish. Delaware County is Columbus's northern suburb and is the highest-income county in Ohio."),
        ],
        answers: &["cannot say who any of these people are"],
        figures: &[
            Figure { label: "per exemption, dollars", value: 38579.0, literal: "38,579" },
            Figure { label: "county average, dollars", value: 23638.0, literal: "23,638" },
        ],
    },
    Assertion {
        id: "two-allen-counties-and-the-ohio-one-loses",
        statement: "961 exemptions left Allen County, Ohio for Allen County, Indiana over nineteen \
                    years and 635 came back \u{2014} a net 326 people and $6,414,000 to Fort Wayne.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-by-county-2004-2023.yml", "**Two Allen Counties, and the Ohio one loses.** 961 exemptions left for Allen County, Indiana \u{2014} Fort Wayne \u{2014} and 635 came back, a net 326 people and $6,414,000. [verified] \u{2014} the same files."),
        ],
        answers: &["cannot say who any of these people are"],
        figures: &[
            Figure { label: "out", value: 961.0, literal: "961" },
            Figure { label: "in", value: 635.0, literal: "635" },
            Figure { label: "net", value: 326.0, literal: "326" },
        ],
    },
    Assertion {
        id: "five-rural-counties-give-more-than-they-take",
        statement: "Five counties give Allen County more people than they take, and all five are \
                    rural: Hardin +175, Putnam +145, Shelby +132, Mercer +101 and Defiance +50.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-migration-by-county-2004-2023.yml", "**Five counties give this one more than they take, and all five are rural.** Hardin +175, Putnam +145, Shelby +132, Mercer +101 and Defiance +50. [verified] \u{2014} the same files. Every county with a city of a hundred thousand people in it runs the other way. [inference]"),
        ],
        answers: &["cannot say who any of these people are"],
        figures: &[
            Figure { label: "Hardin", value: 175.0, literal: "175" },
            Figure { label: "Putnam", value: 145.0, literal: "145" },
            Figure { label: "Shelby", value: 132.0, literal: "132" },
        ],
    },
    Assertion {
        id: "where-the-missing-people-went",
        statement: "Allen County's population estimate fell 1,351 between 2020 and 2024. Over \
                    nineteen years its net migration loss on tax returns was 9,171 exemptions, and \
                    the counties that took them are named.",
        topic: "population",
        supports: &[
            support!("measure/allen-county-population-2024.yml", "**Where the missing people went is now partly answerable.** Between 2004\u{2013}05 and 2022\u{2013}23 the county had a net migration loss of 9,171 exemptions on tax returns, and the counties that took them are named: Franklin County a net 2,036, nine metropolitan counties 3,981 between them, and the five counties on this one's own border a net eighty-two across nineteen years. [verified] \u{2014} [the IRS migration files](../../catalog/irs-county-migration.md); see [where they went](allen-county-migration-by-county-2004-2023.yml)."),
        ],
        answers: &[],
        figures: &[
            Figure { label: "net migration loss", value: 9171.0, literal: "9,171" },
            Figure { label: "net to Franklin County", value: 2036.0, literal: "2,036" },
        ],
    },
    Assertion {
        id: "the-year-this-site-called-empty",
        statement: "The 2020 assessment file is not empty. It carries a proficiency figure for every \
                    one of Allen County's twelve districts with the two band columns beside it left \
                    null, and 909 of Ohio's 924 district rows have one.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**The 2020 file is not empty, and this node said it was.** It carries a proficiency figure for every one of the twelve districts with the two band columns beside it left null, and the earlier reading here took the null bands for missing data and reported the year as blank. 923 of Ohio's 924 district rows have a positive count of valid tests and 909 have a reading figure. [verified] \u{2014} [the same file](../../catalog/edfacts-outcomes.md), re-read on 4 September 2026; see [a zero is not a blank](../../decisions/a-zero-is-not-a-blank.yml)."),
        ],
        answers: &["cannot say whether that gap widened"],
        figures: &[
            Figure { label: "Ohio rows with a figure", value: 909.0, literal: "909" },
            Figure { label: "Ohio district rows", value: 924.0, literal: "924" },
        ],
    },
    Assertion {
        id: "the-missing-year-is-the-cancelled-one",
        statement: "The file's year is the fall of the school year, so the 2019 that is missing is \
                    the 2019\u{2013}20 year whose spring tests were cancelled and the 2020 that is \
                    present is 2020\u{2013}21. The graduation file, counting a cohort rather than a \
                    test day, runs through 2019 with no gap.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**The `year` field is the fall of the school year, and that is why 2019 is the year that is absent.** 2019 is the 2019\u{2013}20 school year, whose spring tests were not given; 2020 is 2020\u{2013}21, whose spring tests were. The graduation file, which counts a cohort rather than a test day, runs through 2019 with no gap in it. [inference] \u{2014} the collection's own coverage, read against the convention its publisher uses for the school directory beside it."),
        ],
        answers: &["cannot say whether that gap widened"],
        figures: &[],
    },
    Assertion {
        id: "eleven-of-the-twelve-districts-fell",
        statement: "Reading fell in eleven of Allen County's twelve districts between 2018 and 2020 \
                    and mathematics in eleven. The county's weighted reading went from 68.8 per cent \
                    proficient to 63.1 and its mathematics from 69.6 to 60.2.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**Reading fell in eleven of the twelve districts between 2018 and 2020 and mathematics in eleven.** Pandora-Gilboa is the one district whose reading rose, by a point, and Columbus Grove the one whose mathematics held. The county's weighted reading figure went from 68.8 per cent to 63.1 and its mathematics from 69.6 to 60.2. [verified] \u{2014} the same file, weighted here by tests taken."),
        ],
        answers: &["cannot say whether that gap widened"],
        figures: &[
            Figure { label: "reading, 2018", value: 68.8, literal: "68.8" },
            Figure { label: "reading, 2020", value: 63.1, literal: "63.1" },
            Figure { label: "mathematics, 2020", value: 60.2, literal: "60.2" },
        ],
    },
    Assertion {
        id: "limas-mathematics-fell-to-twenty-nine",
        statement: "Lima City went from 42 per cent proficient in mathematics to 29 between 2018 and \
                    2020, against 18 points lost in Waynesfield-Goshen and 15 in Perry. It is the \
                    county's lowest reading district in all eleven years of the collection.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**Mathematics fell furthest where it was already lowest.** Lima went from 42 per cent proficient in mathematics to 29 \u{2014} thirteen points off a figure that had been the lowest in the county for eleven straight years \u{2014} against 18 points in Waynesfield-Goshen and 15 in Perry. [verified] \u{2014} the same file. In eleven years of this collection Lima is the county's lowest reading district in every one."),
        ],
        answers: &["cannot say whether that gap widened"],
        figures: &[
            Figure { label: "Lima, 2018", value: 42.0, literal: "42" },
            Figure { label: "Lima, 2020", value: 29.0, literal: "29" },
        ],
    },
    Assertion {
        id: "the-county-pulled-further-from-ohio",
        statement: "Ohio's weighted reading proficiency fell from 64.4 per cent to 57.2 and its \
                    mathematics from 62.5 to 49.7, so a county 4.4 points clear of the state in \
                    reading and 7.1 in mathematics was 5.9 and 10.5 clear of it in 2020.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-test-proficiency-2009-2020.yml", "**The state fell further than the county, again.** Ohio's weighted reading went from 64.4 per cent to 57.2 and its mathematics from 62.5 to 49.7, so a county 4.4 points clear of the state in reading and 7.1 in mathematics was 5.9 and 10.5 clear of it in 2020. [verified] \u{2014} the same file over every reporting Ohio district, 909 of them in reading and 904 in mathematics. Two crises, one instrument change and one closure, and after each the county's distance from Ohio grew. [inference]"),
        ],
        answers: &["cannot say whether that gap widened"],
        figures: &[
            Figure { label: "reading gap, 2020", value: 5.9, literal: "5.9" },
            Figure { label: "mathematics gap, 2020", value: 10.5, literal: "10.5" },
        ],
    },
    Assertion {
        id: "exact-by-district-and-banded-by-school",
        statement: "All 264 district cells in this collection are numbers. The same measure over the \
                    same children asked by building gives 71 numbers, 91 bands and 4 cells withheld \
                    outright, out of 166.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-school-2018-2020.yml", "**The measure that has no suppression at district level is more than half suppressed at school level.** All 264 district cells in this collection are numbers. The same measure over the same children by building gives 71 numbers, 91 bands and 4 cells withheld outright, out of 166. [verified] \u{2014} the same file at both levels, counted here. Nothing about the schooling changed between the two questions; the denominator did."),
        ],
        answers: &["cannot say what any of these schools does differently"],
        figures: &[
            Figure { label: "numbers", value: 71.0, literal: "71" },
            Figure { label: "bands", value: 91.0, literal: "91" },
        ],
    },
    Assertion {
        id: "the-four-lowest-schools-are-in-lima",
        statement: "The four lowest legible schools in Allen County are all in Lima \u{2014} West \
                    and North Middle at 31 per cent proficient in reading, Lima Senior at 39 and \
                    Liberty Arts Magnet at 56 \u{2014} against Perry Elementary's 62 and a county \
                    top of 87 at Bluffton.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-school-2018-2020.yml", "**The four lowest legible schools in the county are all in Lima and the gap to the fifth is six points.** Lima West and Lima North read 31 per cent proficient, Lima Senior 39 and Liberty Arts 56, against Perry Elementary's 62 and a top of 87 at Bluffton. [verified] \u{2014} the same file, sorted here. Every one of those eighteen figures rests on between 234 and 816 tests."),
        ],
        answers: &["cannot say what any of these schools does differently"],
        figures: &[
            Figure { label: "Lima West and North", value: 31.0, literal: "31" },
            Figure { label: "Lima Senior", value: 39.0, literal: "39" },
            Figure { label: "Bluffton Elementary", value: 87.0, literal: "87" },
        ],
    },
    Assertion {
        id: "one-school-tests-mathematics-at-seventeen",
        statement: "Lima Senior High School is published at 17 per cent proficient in mathematics in \
                    2018 and 16 in 2020, against 39 and 42 in reading. Reading and mathematics part \
                    company at that one building in a way they do at no other in the county.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-school-2018-2020.yml", "**One school in the county tests mathematics at seventeen per cent.** Lima Senior High School, 249 tests in 2018 and 251 in 2020, published as 17 and then 16 \u{2014} against 39 and 42 in reading in the same two years. [verified] \u{2014} the same file. Reading and mathematics part company at this one building in a way they do at no other in the county."),
        ],
        answers: &["cannot say what any of these schools does differently"],
        figures: &[
            Figure { label: "mathematics, 2018", value: 17.0, literal: "17" },
            Figure { label: "mathematics, 2020", value: 16.0, literal: "16" },
        ],
    },
    Assertion {
        id: "one-high-school-a-reader-may-read",
        statement: "Of the 48 high-school cells across two years and two subjects, five are numbers \
                    and four of the five are Lima Senior's. Every other high-school figure in the \
                    county is a band, because Ohio tests one or two grades at that level and a \
                    county high school has between 38 and 199 of them.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-school-2018-2020.yml", "**It is also the only high school here a reader may read.** Of the 48 high-school cells across the two years and two subjects, five are numbers and four of the five are Lima Senior's; the fifth is Elida High School's 2020 mathematics, on 224 tests. Every other high-school figure in the county is a band, because Ohio tests one or two grades at that level and a county high school has between 38 and 199 of them. [verified] \u{2014} the same file; see [a suppressed range is not a margin](../../decisions/a-suppressed-range-is-not-a-margin.yml)."),
        ],
        answers: &["cannot say what any of these schools does differently"],
        figures: &[
            Figure { label: "high-school cells", value: 48.0, literal: "48" },
            Figure { label: "largest tested high school", value: 199.0, literal: "199" },
        ],
    },
    Assertion {
        id: "two-lima-schools-read-higher-in-2020",
        statement: "Two of Lima's nine schools read higher in 2020 than in 2018 \u{2014} Lima \
                    Senior, from 39 to 42, and the South Science-Technology Magnet K-8, from a band \
                    of 65-69 to one of 70-74. Unity Elementary went from 35-39 to 11-19.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-school-2018-2020.yml", "**Two of Lima's nine schools read higher in 2020 than in 2018 and the rest fell.** Lima Senior went from 39 to 42 and the South Science-Technology Magnet K-8 from a band of `65-69` to one of `70-74`; Unity Elementary went from `35-39` to `11-19`, Independence from `50-54` to `25-29`, and Lima North Middle's mathematics from 22 per cent to 13. [verified] \u{2014} the same file, both years. The two that held are a high school and a magnet school."),
        ],
        answers: &["cannot say what any of these schools does differently"],
        figures: &[
            Figure { label: "Lima Senior, 2018", value: 39.0, literal: "39" },
            Figure { label: "Lima Senior, 2020", value: 42.0, literal: "42" },
        ],
    },
    Assertion {
        id: "nine-of-eleven-legible-rows-are-white",
        statement: "Eleven race rows a year clear the disclosure threshold across Allen County's \
                    twelve districts, and nine of them are white children. Perry, Pandora-Gilboa and \
                    Waynesfield-Goshen have no legible race row at all.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-subgroup-2013-2020.yml", "**Eleven rows a year survive the disclosure threshold and nine of them are white children.** A subgroup is published as a number at 301 tested and as a band at 300, so in 2018 the legible race rows are the white children of eight districts, and Lima's white, Black and two-or-more children. Perry, Pandora-Gilboa and Waynesfield-Goshen have no legible race row at all. [verified] \u{2014} the same files, all three years, counted here; see [a suppressed range is not a margin](../../decisions/a-suppressed-range-is-not-a-margin.yml)."),
        ],
        answers: &["cannot say how many children are counted in more than one of these groups"],
        figures: &[
            Figure { label: "number at", value: 301.0, literal: "301" },
            Figure { label: "band at", value: 300.0, literal: "300" },
        ],
    },
    Assertion {
        id: "only-lima-publishes-its-black-children",
        statement: "Lima City is the only district in Allen County where Black children's results \
                    are published as a number, and the only one where they could be: no other \
                    district tests more than three hundred of them in any year.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-subgroup-2013-2020.yml", "**Lima is the only district in this county where Black children's results are published as a number, and it is the only one where they could be.** No other district tests more than three hundred of them in any year. [verified] \u{2014} the same files."),
        ],
        answers: &["cannot say how many children are counted in more than one of these groups"],
        figures: &[],
    },
    Assertion {
        id: "the-gap-inside-lima-outlived-the-test",
        statement: "Inside Lima the reading gap between white and Black pupils is fourteen or \
                    fifteen points in every year and the mathematics gap eighteen to twenty-two. It \
                    survived the change of test that moved both figures thirty points, and it \
                    survived the closed year.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-subgroup-2013-2020.yml", "**The reading gap inside Lima is fourteen or fifteen points in every year and the mathematics gap is eighteen to twenty-two.** It survived the change of test that moved both figures thirty points, and it survived the closed year. [verified] \u{2014} the same files, differenced here."),
        ],
        answers: &["cannot say how many children are counted in more than one of these groups"],
        figures: &[],
    },
    Assertion {
        id: "limas-white-children-are-lowest-too",
        statement: "Lima's white children read at 48 per cent proficient in 2018, against a range of \
                    72 in Elida to 84 in Bluffton and Delphos across the eight other legible \
                    districts \u{2014} twenty-four points below the lowest of them.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-subgroup-2013-2020.yml", "**Lima's white children read below every other district's white children by twenty-four points.** 48 per cent proficient in 2018 against a range of 72 in Elida to 84 in Bluffton and Delphos across the eight other legible districts. [verified] \u{2014} the same file, 2018. Whatever separates this district from the others is not carried by the race of the children in it. [inference]"),
        ],
        answers: &["cannot say how many children are counted in more than one of these groups"],
        figures: &[
            Figure { label: "Lima", value: 48.0, literal: "48" },
            Figure { label: "Elida", value: 72.0, literal: "72" },
            Figure { label: "Bluffton and Delphos", value: 84.0, literal: "84" },
        ],
    },
    Assertion {
        id: "held-to-poor-children-the-spread-holds",
        statement: "Five districts test more than three hundred economically disadvantaged pupils \
                    and are published as numbers. In 2018 they read Bath 71 per cent proficient, \
                    Shawnee 68, Perry 62, Elida 58 and Lima 42; in 2020, 66, 63, 57, 49 and 36.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-subgroup-2013-2020.yml", "**Held to poor children only, the county's spread widened from twenty points to thirty.** Five districts test more than three hundred economically disadvantaged pupils and are published as numbers: in 2013 Bath 86 per cent proficient in reading, Shawnee 84, Elida 82, Perry 79 and Lima 66; in 2018 Bath 71, Shawnee 68, Perry 62, Elida 58 and Lima 42; in 2020 Bath 66, Shawnee 63, Perry 57, Elida 49 and Lima 36. [verified] \u{2014} the same files, all three years. The three spreads \u{2014} 20, 29 and 30 points \u{2014} straddle a change of test that moved every level in the table, so the widening is read here as a difference between districts and not as a measured trend. [inference]"),
        ],
        answers: &["cannot say how many children are counted in more than one of these groups"],
        figures: &[
            Figure { label: "Bath, 2018", value: 71.0, literal: "71" },
            Figure { label: "Lima, 2018", value: 42.0, literal: "42" },
        ],
    },
    Assertion {
        id: "in-two-districts-the-poor-are-the-district",
        statement: "Lima tests 1,895 economically disadvantaged pupils of 1,904 in 2018 and 1,775 of \
                    1,775 in 2020; Perry tests 425 of 425 and 415 of 415. A breakout that returns \
                    the whole is a breakout in name.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-subgroup-2013-2020.yml", "**In two of those five the subgroup is the district.** Lima tests 1,895 economically disadvantaged pupils of 1,904 in 2018 and 1,775 of 1,775 in 2020; Perry tests 425 of 425 and 415 of 415. [verified] \u{2014} the same files against the all-students rows. A breakout that returns the whole is a breakout in name, and the figure it produces is the district's own figure a second time. [inference] See [the district poverty](allen-county-school-district-poverty-2023.yml)."),
        ],
        answers: &["cannot say how many children are counted in more than one of these groups"],
        figures: &[
            Figure { label: "Lima, disadvantaged", value: 1895.0, literal: "1,895" },
            Figure { label: "Perry, disadvantaged", value: 425.0, literal: "425" },
        ],
    },
    Assertion {
        id: "disability-is-legible-in-one-district",
        statement: "Children with disabilities are published as a number in one of Allen County's \
                    twelve districts. Lima tests 357, 367 and 364 of them across the three years and \
                    reads 39, 12 and 7 per cent proficient.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-subgroup-2013-2020.yml", "**Children with disabilities are legible in one district out of twelve, and the figure has fallen from 39 per cent to 7.** Lima tests 357, 367 and 364 of them in the three years and reads 39, 12 and 7 per cent proficient; mathematics runs 30, 14 and 7. No other district in the county reaches three hundred. [verified] \u{2014} the same files."),
        ],
        answers: &["cannot say how many children are counted in more than one of these groups"],
        figures: &[
            Figure { label: "Lima, 2013", value: 39.0, literal: "39" },
            Figure { label: "Lima, 2020", value: 7.0, literal: "7" },
        ],
    },
    Assertion {
        id: "english-learners-are-never-published",
        statement: "Six of Allen County's districts return an English-learner row in 2013, eight in \
                    2018 and seven in 2020, and not one of those twenty-one rows carries a number in \
                    either subject.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-proficiency-by-subgroup-2013-2020.yml", "**English learners are counted in this county and never published.** Six districts return a row in 2013, eight in 2018 and seven in 2020, and not one of those twenty-one rows carries a number in either subject. [verified] \u{2014} the same files. The kind of child the county has more districts reporting in 2018 than in 2013 is the one kind whose results the threshold withholds entirely. [inference]"),
        ],
        answers: &["cannot say how many children are counted in more than one of these groups"],
        figures: &[],
    },
    Assertion {
        id: "a-subgroup-costs-a-hundred-children",
        statement: "A whole cohort is published as a number at 201 and a subgroup row at 301, a \
                    hundred higher, with no exception in 37,632 graduation rows. Being counted as a \
                    kind of child costs a hundred children of legibility.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-graduation-rates-2010-2019.yml", "**The sentence above about cohort size holds for the all-student rows and not for the file.** A whole cohort is published as a number at 201 and a subgroup row at 301, a hundred higher, with no exception in 37,632 graduation rows. [verified] \u{2014} [the same collection](../../catalog/edfacts-outcomes.md), Ohio-wide, counted here; see [a suppressed range is not a margin](../../decisions/a-suppressed-range-is-not-a-margin.yml). The rule that leaves this county one legible district leaves it fewer legible kinds of child than that."),
        ],
        answers: &["cannot say what any other district's graduation rate for poor children is"],
        figures: &[
            Figure { label: "whole cohort", value: 201.0, literal: "201" },
            Figure { label: "subgroup row", value: 301.0, literal: "301" },
        ],
    },
    Assertion {
        id: "a-school-closed-and-the-count-held",
        statement: "Landeck Elementary in Delphos reports 84 pupils and an open status in the fall \
                    of 2018, a closed status in 2019, and is gone from the directory in 2020. \
                    Delphos falls from four schools to three and returns to four in 2021 with a \
                    preschool in the fourth place.",
        topic: "schools",
        supports: &[
            support!("measure/allen-county-school-buildings-1986-2024.yml", "**A building left the count in Delphos and the count did not move.** Landeck Elementary reports 84 pupils and an open status in the fall of 2018, a closed status and no enrolment in 2019, and is absent from the directory in 2020; the district falls from four schools to three and returns to four in 2021 with a preschool standing in the fourth place. [verified] \u{2014} [the same collection](../../catalog/nces-common-core-of-data.md), its school directory, 2018 to 2024; see [proficiency by school](allen-county-proficiency-by-school-2018-2020.yml). The table above gives Delphos 4 in 1986, 4 in 2024 and a change of zero, which is what a count of open schools reports when one closes and another opens."),
        ],
        answers: &["cannot say where any of these buildings stood"],
        figures: &[
            Figure { label: "Landeck pupils, 2018", value: 84.0, literal: "84" },
        ],
    },
    Assertion {
        id: "the-plant-was-begun-in-may-1942",
        statement: "The Army's installation south of Lima was begun in May 1942, on 170 acres of \
                    farmland, by the Ohio Steel Foundry Company, to make centrifugally cast gun \
                    tubes.",
        topic: "history",
        supports: &[
            support!("event/the-army-builds-at-lima-1942.yml", "**In May 1942 the Ohio Steel Foundry Company began building a government-owned, contractor-operated plant about five miles south of the centre of Lima, on open land previously used for agriculture, to produce centrifugally cast gun tubes.** [verified] \u{2014} [the historic properties report](../../catalog/dtic-lima-army-tank-plant-reports.md), *World War II to 1950*. The site covered 170 acres."),
        ],
        answers: &["cannot say who owned the 170 acres before the Army did"],
        figures: &[
            Figure { label: "acres at the start", value: 170.0, literal: "170" },
        ],
    },
    Assertion {
        id: "the-mission-was-cancelled-before-the-plant-was-finished",
        statement: "The plant's mission was cancelled inside the year and before the building was \
                    finished: a new way of piercing seamless steel tubing made a gun-tube casting \
                    plant unnecessary, and the Ordnance Corps turned the site into a depot for \
                    modifying combat vehicles instead.",
        topic: "history",
        supports: &[
            support!("event/the-army-builds-at-lima-1942.yml", "**The mission it was built for was cancelled before it was finished.** A new process for piercing seamless steel tubing to form light artillery pieces made the casting plant unnecessary, and only a few months after construction began the Ordnance Corps decided to use the facility as a depot for modifying and processing tanks and other combat vehicles instead. [verified] \u{2014} the same source. The installation that stands here was therefore never built for the thing it has done for eighty years, nor for the thing it was begun for."),
        ],
        answers: &["cannot say who owned the 170 acres before the Army did"],
        figures: &[],
    },
    Assertion {
        id: "a-hundred-thousand-vehicles-passed-through-lima",
        statement: "More than a hundred thousand combat vehicles passed through the Lima \
                    installation before the war ended \u{2014} which corroborates a headline of \
                    August 1944 this corpus had read as 75,000 through damaged scanning and declined \
                    to write down.",
        topic: "history",
        supports: &[
            support!("event/the-army-builds-at-lima-1942.yml", "**More than a hundred thousand combat vehicles passed through before the war was over.** [verified] \u{2014} the same source. That corroborates the figure this corpus refused to write down: a headline of August 1944 gave `75,01)0` through damaged scanning, and seventy-five thousand by that August is what a hundred thousand by 1945 implies. [inference] See [two scans of one book](../../decisions/two-scans-of-one-book.yml)."),
        ],
        answers: &["cannot say who owned the 170 acres before the Army did"],
        figures: &[],
    },
    Assertion {
        id: "building-147-is-the-war-still-standing",
        statement: "Building 147, the plant's primary factory building, opened in 1943 with 582,000 \
                    square feet of floor space behind curtain walls glazed from ground line to roof \
                    in single-pane glass. About fifty numbered buildings stood at Lima during the \
                    war and eight of them still form the core of the plant.",
        topic: "history",
        supports: &[
            support!("event/the-army-builds-at-lima-1942.yml", "**Building 147 is the war still standing.** The primary factory building, irregular in plan, originally provided 582,000 square feet of floor space \u{2014} a steel-frame, high-bay structure whose curtain walls were glazed from near ground line to roof line with single-pane glass in steel sash. About fifty numbered buildings or structures stood at Lima during the war, and Building 147 and seven others named in the report still form the core of the plant. [verified] \u{2014} the same source. The Detroit architectural engineering firm of Shreve, Anderson and Walker planned and designed the installation from March to September 1942, and the work was completed by another Detroit firm headed by William Edward Lapp."),
        ],
        answers: &["cannot say who owned the 170 acres before the Army did"],
        figures: &[
            Figure { label: "square feet, 1943", value: 582000.0, literal: "582,000" },
        ],
    },
    Assertion {
        id: "the-ground-stayed-at-163-acres",
        statement: "A seven-acre sale in 1943 took the installation from 170 acres to 163, and it \
                    stayed that size for eight years.",
        topic: "history",
        supports: &[
            support!("event/the-army-builds-at-lima-1942.yml", "**The ground stayed at 163 acres for eight years.** A seven-acre sale in 1943 reduced the installation from 170 acres to 163 contiguous ones, and it remained that size until 1951. [verified] \u{2014} the same source; see [the 1951 purchase](the-united-states-buys-the-tank-plant-ground-1951.yml), which is an expansion of this and not a founding."),
        ],
        answers: &["cannot say who owned the 170 acres before the Army did"],
        figures: &[
            Figure { label: "acres, 1943 to 1951", value: 163.0, literal: "163" },
        ],
    },
    Assertion {
        id: "the-deed-was-an-expansion-not-a-founding",
        statement: "The five warranty deeds of June 1951 bought 295 contiguous acres south-east of a \
                    base built in 1942, taking the installation from 163 acres to 458 and making \
                    room for a tank test track. It is an expansion, and this site had recorded it as \
                    a founding.",
        topic: "history",
        supports: &[
            support!("event/the-united-states-buys-the-tank-plant-ground-1951.yml", "**It is an expansion and this node called it a founding.** The Army bought 295 contiguous acres just south-east of a base built in May 1942, taking the installation from 163 acres to 458, to lay out a tank test track and hold room for later building. [verified] \u{2014} [the historic properties report](../../catalog/dtic-lima-army-tank-plant-reports.md), *Korean War to 1975*. The quarter section abstracted here is the south-eastern end of a nine-year-old installation."),
        ],
        answers: &["does not establish when anything was built on the ground it describes"],
        figures: &[
            Figure { label: "acres bought", value: 295.0, literal: "295" },
            Figure { label: "acres after", value: 458.0, literal: "458" },
        ],
    },
    Assertion {
        id: "four-farmhouses-are-numbered-buildings",
        statement: "Four privately-owned houses came with the 1951 land and were still numbered \
                    buildings on the installation in 1984, one of them built between about 1900 and \
                    1925. A farm bought whole leaves its house standing.",
        topic: "history",
        supports: &[
            support!("event/the-united-states-buys-the-tank-plant-ground-1951.yml", "**The dwellings came with the land.** The Army's report lists four privately-owned houses acquired in this purchase and still numbered as buildings on the installation in 1984, one of them built between about 1900 and 1925. [verified] \u{2014} [the historic properties report](../../catalog/dtic-lima-army-tank-plant-reports.md). A farm bought whole leaves its house standing, and four of them are on this ground."),
        ],
        answers: &["does not establish when anything was built on the ground it describes"],
        figures: &[],
    },
    Assertion {
        id: "the-coordinate-sits-on-the-1951-land",
        statement: "No building stands within a hundred and fifty metres of the tank plant's \
                    coordinate of record and the nearest large footprint is 632 metres off \u{2014} \
                    which is what a coordinate on land bought for a test track looks like when the \
                    works stand on ground bought nine years earlier.",
        topic: "history",
        supports: &[
            support!("event/the-united-states-buys-the-tank-plant-ground-1951.yml", "**And the corpus's own coordinate finding was pointing at this all along.** No building stands within a hundred and fifty metres of the installation's coordinate of record, and the nearest large footprint is 632 metres away. [verified] \u{2014} [USA Structures](../../catalog/fema-usa-structures.md); see [the buildings](../measure/allen-county-buildings-2019.yml). That is what a coordinate on land bought for a test track looks like when the works stand on the ground bought nine years earlier. [inference] See [a parcel is not an installation](../../decisions/a-parcel-is-not-an-installation.yml)."),
        ],
        answers: &["does not establish when anything was built on the ground it describes"],
        figures: &[
            Figure { label: "metres to the nearest large footprint", value: 632.0, literal: "632" },
        ],
    },
    Assertion {
        id: "the-depot-is-the-plant-under-its-first-name",
        statement: "The wartime Lima Tank Depot and the tank plant standing south of Lima today are \
                    one installation. Construction began in May 1942, United Motors Service took it \
                    over that November, and more than a hundred thousand vehicles passed through \
                    before the war ended.",
        topic: "history",
        supports: &[
            support!("site/lima-army-tank-plant.yml", "**This installation is the other one, and it is the Lima Tank Depot.** Construction began here in May 1942; the Ordnance Corps turned it from a gun-tube plant into a depot for modifying and processing combat vehicles before it was finished; United Motors Service, a General Motors subsidiary, took it over under contract in November 1942; and more than a hundred thousand vehicles passed through before the war ended. [verified] \u{2014} [the historic properties report](../../catalog/dtic-lima-army-tank-plant-reports.md), *World War II to 1950*; see [the Army's first shovel](../event/the-army-builds-at-lima-1942.yml). Every one of those facts was already in this corpus, attached to a node the corpus had argued was somewhere else."),
        ],
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[],
    },
    Assertion {
        id: "four-names-for-one-installation",
        statement: "One set of buildings has carried four names: Lima Tank Depot through the war, \
                    Lima Ordnance Depot from late 1945, Lima Army Modification Center through the \
                    1970s, and Lima Army Tank Plant on the M-1 award. A sewer easement of 1972 found \
                    in the Recorder's books uses the third.",
        topic: "history",
        supports: &[
            support!("site/lima-army-tank-plant.yml", "**The names are a chronology.** Lima Tank Depot through the war; Lima Ordnance Depot from late 1945, holding mothballed vehicles; Lima Army Modification Center through the 1970s, when it received and processed 12,400 new M880 trucks; and Lima Army Tank Plant on the M-1 award. [verified] \u{2014} [the historic properties report](../../catalog/dtic-lima-army-tank-plant-reports.md). The 1972 sewer easement running from \"(Lima Army Modification Center) Sec'y of the Army\" that this corpus found in the Recorder's books is the third of those names in use, and it is the same place. [inference]"),
        ],
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[
            Figure { label: "M880 trucks processed", value: 12400.0, literal: "12,400" },
        ],
    },
    Assertion {
        id: "general-dynamics-has-operated-it-since-1982",
        statement: "General Dynamics has been the plant's contractor-operator since March 1982, when \
                    Chrysler sold Chrysler Defense to it for $348.5 million. Chrysler had begun M-1 \
                    production at Lima in May 1979 and delivered the first two production tanks in \
                    February 1980.",
        topic: "history",
        supports: &[
            support!("site/lima-army-tank-plant.yml", "**It operates it, and the document that says so is the Army's.** General Dynamics took over M-1 production at Lima and at Detroit in March 1982 as contractor-operator, succeeding the Chrysler Corporation and then Chrysler Defense, which had begun production of M-1s at Lima in May 1979 and delivered the first two production tanks in February 1980. [verified] \u{2014} the same source. The question this node carried for eighteen years of contract awards is answered by a forty-two-year-old report on its buildings."),
            support!("site/lima-army-tank-plant.yml", "What changed in 1994 is who filed and not who operated. General Dynamics has been the contractor-operator of this installation continuously since March 1982, when Chrysler sold Chrysler Defense to it for $348.5 million. [verified] \u{2014} [the historic properties report](../../catalog/dtic-lima-army-tank-plant-reports.md), *M-1 Tank Era*. The reporting entity at a government-owned plant is a compliance arrangement rather than a contract award, and the two moved independently."),
        ],
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[
            Figure { label: "million dollars", value: 348.5, literal: "348.5" },
        ],
    },
    Assertion {
        id: "the-plant-was-nearly-idle-through-vietnam",
        statement: "The installation was nearly idle from 1954 to 1975, through the whole of the \
                    Vietnam War, with parts of it leased out for commercial use; it shrank from 458 \
                    acres to 373 when land went to an Army Reserve training centre and to Johnny \
                    Appleseed Park.",
        topic: "history",
        supports: &[
            support!("site/lima-army-tank-plant.yml", "**The installation's size is a chronology of its use.** 170 acres in May 1942; 163 after a seven-acre sale in 1943 and unchanged for eight years; 458 by June 1951, when the Army added 295 contiguous acres to the south-east for a tank test track; and 373 by the mid-1970s, after land was released to an Army Reserve training centre and to Johnny Appleseed Park. [verified] \u{2014} [the historic properties report](../../catalog/dtic-lima-army-tank-plant-reports.md). It was nearly idle from 1954 to 1975, through the whole of the Vietnam War, and parts of it were leased out for commercial use."),
        ],
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[
            Figure { label: "acres by the mid-1970s", value: 373.0, literal: "373" },
        ],
    },
    Assertion {
        id: "no-archeologist-has-looked-at-this-ground",
        statement: "No archeological investigation has ever been conducted on the tank plant's land, \
                    and the Army's own overview says so while reporting that no site is known there. \
                    An absence of sites in a file that records no survey is an absence of looking.",
        topic: "history",
        supports: &[
            support!("site/lima-army-tank-plant.yml", "**No archeologist has ever looked at this ground.** The Army's companion overview reports the lack of any known archeological site on the installation and states that no archeological investigation has ever been conducted on its land, recommending a reconnaissance survey of a 53-acre parcel then scheduled for construction. [verified] \u{2014} [the archeological overview](../../catalog/dtic-lima-army-tank-plant-reports.md); see [a zero is not a blank](../../decisions/a-zero-is-not-a-blank.yml). An absence of sites in a file that records no survey is an absence of looking. [inference]"),
        ],
        answers: &["cannot say what the plant has been since 1984"],
        figures: &[
            Figure { label: "acre parcel recommended for survey", value: 53.0, literal: "53" },
        ],
    },
    Assertion {
        id: "limas-mayor-came-back-in-1933",
        statement: "Lima changed its city government from the manager-commission form back to mayor \
                    and council in 1933, eleven years after the commission charter took effect and \
                    six years before the next mayor this site could find.",
        topic: "history",
        supports: &[
            support!("event/lima-returns-to-mayor-and-council-1933.yml", "**\"In 1933 Lima changed its city government from the Manager-Commission form to Mayor-Council.\"** [verified] \u{2014} [History of Lima, Ohio](../../catalog/hackman-history-of-lima-1951.md), the booklet's chronology of the 1930s."),
        ],
        answers: &["cannot say how the change was made"],
        figures: &[],
    },
    Assertion {
        id: "the-commission-form-lasted-eleven-years",
        statement: "The seventeen years the commission charter node could say nothing about are now \
                    eleven and six, with a dated change between them: the charter took effect on 1 \
                    January 1922 and the office of mayor returned in 1933.",
        topic: "history",
        supports: &[
            support!("event/lima-adopts-commission-government-1922.yml", "**The office returned in 1933, and the commission form lasted eleven years.** [verified] \u{2014} [History of Lima, Ohio](../../catalog/hackman-history-of-lima-1951.md); see [the return](lima-returns-to-mayor-and-council-1933.yml). The mayor the corpus found cutting twelve men from the street department in November 1939 was serving under a form of government six years old, and the seventeen years this node could say nothing about are now eleven and six with a dated change between them."),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-buses-began-in-1938",
        statement: "Motor buses began replacing Lima's electric street cars in 1938, cars that had \
                    served the city for over fifty years. It is the end of a street railway this \
                    site could follow under four company names from June 1878 to 1921 and no \
                    further.",
        topic: "history",
        supports: &[
            support!("event/the-buses-replace-the-streetcars-1938.yml", "**\"It was in 1938 that motor busses began replacing the electric street cars that had served the city for over fifty years.\"** [verified] \u{2014} [History of Lima, Ohio](../../catalog/hackman-history-of-lima-1951.md), the booklet's chronology of the 1930s."),
        ],
        answers: &["cannot say when the last electric car ran in Lima"],
        figures: &[],
    },
    Assertion {
        id: "two-histories-and-one-fourth-of-july",
        statement: "Two Lima histories a generation apart put the first electric car on the Fourth \
                    of July and differ by a year: the 1921 county history has 1886, and a booklet of \
                    1952 has the street railway converted on July 4th, 1887.",
        topic: "history",
        supports: &[
            support!("event/the-first-electric-car-in-lima-1886.yml", "**A second local historian puts it on the same day of a different year.** \"In 1887 an electric generating plant was built and on July 4th, the street railway was converted to an electric line, among the first of its kind\". [verified] \u{2014} [History of Lima, Ohio](../../catalog/hackman-history-of-lima-1951.md). Two Lima histories a generation apart agree on the Fourth of July and differ by one year, and neither cites anything. The `occurred` date stays at 1886 because the earlier book is nearer the event, and the disagreement is recorded rather than resolved. [inference] See [three witnesses and three dates](../../decisions/three-witnesses-and-three-dates.yml)."),
        ],
        answers: &[],
        figures: &[],
    },
    Assertion {
        id: "the-locomotive-works-ended-as-baldwin-lima-hamilton",
        statement: "The Lima Locomotive Works was \"now known as the Baldwin-Lima-Hamilton Corp.\" \
                    by 1951, and descends from Carnes, Harper & Co., organized in 1869 to make \
                    sawmill machinery. Both ends of the chronology are one sentence each and neither \
                    transition is dated.",
        topic: "history",
        supports: &[
            support!("organization/lima-locomotive-works.yml", "**The end of the name is dated to within a year and the beginning to within one company.** By 1951 the Lima Locomotive Works was \"now known as the Baldwin-Lima-Hamilton Corp.\", and the works descend from Carnes, Harper & Co., organized in 1869 for the manufacture of sawmill machinery. [verified] \u{2014} [History of Lima, Ohio](../../catalog/hackman-history-of-lima-1951.md). That is a merger history in two words and an origin in one sentence, from a chamber of commerce booklet that names no date for either transition."),
        ],
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
/// The sources a block rests on, resolving this corpus's "same source" back-reference.
///
/// A node's prose says "[verified] — [County Business Patterns](../../catalog/cbp.md)" once and
/// "[verified] — same source" in every paragraph after it. Inside the node that is exact and
/// readable. Publication takes a block out of its node, and a block-scoped reader of the second
/// paragraph finds nothing — so twenty-six of this site's citations went out with a provenance
/// badge and no provenance in it, from the first build.
///
/// The fix belongs here and not in the prose. Naming the catalog entry in every paragraph would
/// make the nodes worse to read to make one extractor simpler, and the back-reference is a real
/// feature of how the corpus writes: it means *the source named above*, and above is in the node.
/// So the search widens to the node, in order, and a block with no citation of its own inherits
/// the sources of the nearest block before it that has one.
fn block_sources(node: &Node, block: &crate::claim::Block) -> Vec<String> {
    let own = sources(&block.text);
    if !own.is_empty() {
        return own;
    }
    // Only inherit for an explicit back-reference. A block that cites nothing and says nothing
    // about a source is resting on nothing, and this function must keep saying so.
    if !back_reference(&block.text) {
        return Vec::new();
    }
    let mut inherited = Vec::new();
    for b in &node.blocks {
        if std::ptr::eq(b, block) {
            break;
        }
        let s = sources(&b.text);
        if !s.is_empty() {
            inherited = s;
        }
    }
    inherited
}

/// Nouns a block may point back with, after "[verified] — same ".
///
/// Closed, and for the same reason [`crate::claim::REFUSALS`] is: a pattern loose enough to
/// match "the same reasoning applies" would inherit provenance into blocks that are arguing
/// rather than citing, and inheriting a source is exactly as consequential as naming one.
///
/// It was one entry — `source` — for eleven phases, which was right while every source was a
/// dataset. Then the corpus took a newspaper, and blocks began saying "same page", "same
/// archive" and "same dispatch" because those are what a reader of a newspaper node needs.
/// Forty-three blocks were carrying a back-reference this function did not recognise, every
/// one of them one citation away from publishing a provenance badge with nothing in it.
const BACK_REFERENCES: [&str; 12] = [
    "same source",
    "same page",
    "same archive",
    "same file",
    "same roster",
    "same volume",
    "same table",
    "same dispatch",
    "same series",
    "same register",
    "same dataset",
    "same book",
];

/// Whether `text` points back at the source named in an earlier block of its node.
pub fn back_reference(text: &str) -> bool {
    BACK_REFERENCES.iter().any(|r| text.contains(r))
}

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
                sources: block_sources(node, block),
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
    fn a_newspaper_back_reference_inherits_like_a_dataset_one() {
        // "same page" and "same archive" are what a newspaper node says. Before the closed
        // list they inherited nothing, so a cited block ending "[verified] — same page" went
        // out with a provenance badge and no provenance in it.
        for phrase in ["same source", "same page", "same archive", "same dispatch"] {
            assert!(
                back_reference(&format!("A fact. [verified] — {phrase}.")),
                "{phrase}"
            );
        }
    }

    #[test]
    fn an_argument_that_merely_says_same_inherits_nothing() {
        assert!(!back_reference(
            "The same reasoning applies to the townships."
        ));
        assert!(!back_reference("Two scans of the same edition."));
    }

    #[test]
    fn a_block_naming_one_source_twice_names_it_once() {
        let text = "[a](../../catalog/x.md) and [b](../../catalog/x.md)";
        assert_eq!(sources(text), vec!["catalog/x.md"]);
    }
}
