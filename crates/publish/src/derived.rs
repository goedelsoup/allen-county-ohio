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
            "**578 (1830), 9,079 (1840), 12,100 (1850), 19,185 (1860), 23,623 (1870), 31,314 (1880), 40,644 (1890), 47,976 (1900), 56,580 (1910), 68,203 (1920), 69,419 (1930), 73,303 (1940), 88,183 (1950), 103,691 (1960), 111,144 (1970), 112,241 (1980), 109,755 (1990), 108,464 (2000), 106,331 (2010), 102,217 (2020) and 100,866 (2024).**"
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
            Figure { label: "1920", value: 68_203.0, literal: "68,203" },
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
                "**the county outside Lima held 24,663 people in 1890, 26,253 in 1900, 26,072 in 1910, 26,877 in 1920, 27,132 in 1930, 28,592 in 1940, 37,937 in 1950, 52,654 in 1960, 57,410 in 1970, 64,414 in 1980, 64,206 in 1990, 68,157 in 2000, 67,560 in 2010 and 66,627 in 2020.**"
            ),
        ],
        answers: &[],
        figures: &[
            Figure { label: "1890", value: 24_663.0, literal: "24,663" },
            Figure { label: "1900", value: 26_253.0, literal: "26,253" },
            Figure { label: "1910", value: 26_072.0, literal: "26,072" },
            Figure { label: "1920", value: 26_877.0, literal: "26,877" },
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
                    and these are the only measurements of work this corpus holds, all of them \
                    sixty years before the decline it describes.",
        topic: "population",
        // Three points, sixty years before the period they are shown beside. The chart is small
        // and the sentence beneath it is the point: this is a baseline, not a mechanism.
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
                    from 17,985 to 16,113 between 2010 and 2023 — 10.4 per cent, where the county's \
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
            Figure { label: "2010", value: 17_985.0, literal: "17,985" },
            Figure { label: "2011", value: 17_830.0, literal: "17,830" },
            Figure { label: "2012", value: 17_766.0, literal: "17,766" },
            Figure { label: "2013", value: 17_565.0, literal: "17,565" },
            Figure { label: "2014", value: 17_634.0, literal: "17,634" },
            Figure { label: "2015", value: 17_656.0, literal: "17,656" },
            Figure { label: "2016", value: 17_518.0, literal: "17,518" },
            Figure { label: "2017", value: 17_333.0, literal: "17,333" },
            Figure { label: "2018", value: 17_107.0, literal: "17,107" },
            Figure { label: "2019", value: 16_921.0, literal: "16,921" },
            Figure { label: "2020", value: 16_774.0, literal: "16,774" },
            Figure { label: "2021", value: 16_124.0, literal: "16,124" },
            Figure { label: "2022", value: 16_127.0, literal: "16,127" },
            Figure { label: "2023", value: 16_113.0, literal: "16,113" },
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
        answers: &[
            "this corpus does not know",
            "cannot say whether either of them stands on this ground",
        ],
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
        statement: "Allen County had two tank installations in the Second World War, two years and \
                    a category apart: the Lima Locomotive Works, which built medium tanks from \
                    1941, and the Lima Tank Depot, which finished and forwarded vehicles built \
                    elsewhere.",
        topic: "history",
        supports: &[
            support!(
                "site/lima-locomotive-works-plant.yml",
                "In February 1941 the works had \"under construction a new $290,000 factory building\", which the local press read as confirmation that it was switching part of its activity to defense production; by that August medium tank production was scheduled to begin \"this fall\"."
            ),
            support!(
                "site/lima-tank-depot.yml",
                "Vehicles arrived \"in a 'raw' state of completion to Lima from the tank arsenals throughout the United States\", and it was at the Depot that the modifications for a particular theatre of operations were installed, the vehicle given a final run on the Depot's proving ground, sealed, and put on a road train for a shipping port."
            ),
        ],
        answers: &[
            "does not establish where the Depot stood",
            "does not establish whether this is the ground the",
        ],
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
        answers: &["so this corpus knows that the farmland went and cannot say when"],
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
        answers: &["so this corpus knows that the farmland went and cannot say when"],
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
        statement: "Allen County's 364 highway bridges are in poor condition or not according to \
                    who owns them, and the state's hundred and three are all sound.",
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
            "The corpus does not know why, and the two obvious readings — that the state maintains better, or that the state owns newer and larger structures on the interstate — are not separated by anything in this file",
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
                    for eight incidents, and only twice did the money reach households.",
        topic: "history",
        supports: &[
            support!(
                "measure/allen-county-disaster-declarations-1965-2020.yml",
                "**Ten declarations in fifty-five years, for eight distinct incidents.**"
            ),
            support!(
                "measure/allen-county-disaster-declarations-1965-2020.yml",
                "**Two of the ten brought money to households.**"
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
