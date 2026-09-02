// The article: one claim, its own URL, and a declaration of what it rests on.
//
// This is the content type the site did not have, and its absence is the whole of the
// problem `.yidam/decisions/a-page-is-an-argument-not-an-inbox.yml` records. With only the
// page and the entry to choose between, every phase appended a section to whichever topic
// page was nearest; `/history` reached twenty-one of them and thirty-five minutes.
//
// **The registry is separate from the prose on purpose.** An `.astro` page cannot export
// metadata another module can read, so a reading room built over `import.meta.glob` would
// have to parse its own pages. Declaring here instead buys the thing that matters more: the
// `assertions` and `entries` a piece rests on are a *typed list*, so `articles.test.ts` can
// resolve every id against the feed and fail the build on one that does not — the same shape
// as the publication gate in `crates/publish`, one level up.
//
// The same test holds the registry and `src/pages/read/` to a one-to-one correspondence, so
// the two cannot drift.

import { assertions as feedAssertions, nodes } from './feeds'
import type { SectionKey } from './sections'

export interface Article {
  /** The URL is `/read/<slug>`, and the file is `src/pages/read/<slug>.astro`. */
  slug: string
  title: string
  /** One sentence, shown under the title and in the reading room. */
  dek: string
  section: SectionKey
  /**
   * The years the piece is about, for ordering within a section. A piece about a standing
   * condition rather than a span takes the year of the record it reads.
   */
  era: [number, number]
  /** Assertion ids from `series.json`. Every one is checked to resolve. */
  assertions: string[]
  /** Corpus node ids from `graph.json`, as `class/name.yml`. Every one is checked. */
  entries: string[]
}

/**
 * Every article, grouped by the section that links to it.
 *
 * Order within a section is by `era`, computed rather than declared, so adding a piece never
 * means renumbering its neighbours — which is exactly the maintenance the old page-appending
 * habit avoided by never ordering anything at all.
 */
export const ARTICLES: Article[] = [
  // ── Ground ──────────────────────────────────────────────────────────────
  {
    slug: 'a-hog-county',
    title: 'A hog county',
    dek: 'The hogs multiplied while the farms keeping them halved, which is one fact about animals and another about who is left to keep them.',
    section: 'ground',
    era: [2002, 2022],
    assertions: ['the-hogs-multiplied-as-the-keepers-halved'],
    entries: ['place/allen-county.yml'],
  },
  {
    slug: 'where-the-lines-moved',
    title: 'Where the lines moved',
    dek: 'Fifty-seven dated annexations, and a county that lost land it never lost.',
    section: 'ground',
    era: [1990, 2024],
    assertions: ['the-annexations-are-dated-now', 'the-county-lost-land-it-never-lost'],
    entries: [
      'place/lima.yml',
      'place/allen-county.yml',
    ],
  },
  {
    slug: 'what-crosses-the-water',
    title: 'What crosses the water',
    dek: 'Three hundred and sixty-four bridges, and a state that replaced its own while the county kept its.',
    section: 'ground',
    era: [1992, 2025],
    assertions: ['not-one-state-bridge-is-in-poor-condition', 'the-state-replaced-its-bridges'],
    entries: [
      'site/hay-road-bridge.yml',
      'office/allen-county-engineer.yml',
    ],
  },
  {
    slug: 'the-river-measured',
    title: 'The river, measured',
    dek: 'The city drinks from a river nobody gauges, and the gauge everyone cites is somewhere else.',
    section: 'ground',
    era: [1920, 2026],
    assertions: ['the-gauge-was-not-there', 'the-city-drinks-from-the-river-nobody-measures'],
    entries: [
      'natural-feature/ottawa-river.yml',
      'event/the-ottawa-river-flood-of-1959.yml',
      'question/where-the-auglaize-rises.yml',
    ],
  },
  {
    slug: 'when-the-villages-began',
    title: 'When the villages began',
    dek: 'The county’s villages have incorporation dates now, and the order they arrive in is not the order the histories tell.',
    section: 'ground',
    era: [1831, 1912],
    assertions: ['the-villages-have-birthdays-now'],
    entries: [
      'event/the-incorporation-of-delphos-1851.yml',
      'event/the-naming-and-incorporation-of-bluffton-1861.yml',
      'event/the-organization-of-cairo-1875.yml',
      'question/what-happened-to-the-village-of-fort-shawnee.yml',
    ],
  },
  {
    slug: 'counted-two-ways',
    title: 'Counted two ways',
    dek: 'A federal gazetteer and a county map service disagree about how many named things stand on this ground, and both are right.',
    section: 'ground',
    era: [2026, 2026],
    assertions: ['a-county-column-is-a-filing-decision'],
    entries: ['place/allen-county.yml'],
  },
  {
    slug: 'the-weather-the-farms-are-in',
    title: 'The weather the farms are in',
    dek: 'The winters warmed and the summers did not, and there are three more weeks between the frosts than there were.',
    section: 'ground',
    era: [1895, 2025],
    assertions: [
      'the-winters-warmed-and-the-summers-did-not',
      'the-frost-free-season-is-three-weeks-longer',
    ],
    entries: [
      'place/allen-county.yml',
      'question/when-the-farmland-went.yml',
    ],
  },

  // ── People ──────────────────────────────────────────────────────────────
  {
    slug: 'fifth-worst-for-hearts',
    title: 'Fifth worst for hearts',
    dek: 'The county’s heart disease rate climbed while Ohio’s fell, and the gains that stopped in 2010 stopped only for people under 65.',
    section: 'people',
    era: [1999, 2024],
    assertions: [
      'fifth-worst-in-ohio-for-heart-disease',
      'the-improvement-stopped-at-working-age',
      'middling-on-hearts-worst-fifth-on-strokes',
    ],
    entries: ['place/allen-county.yml'],
  },
  {
    slug: 'the-schools-that-are-not-districts',
    title: 'The schools that are not districts',
    dek: 'Ten private schools and two community schools account for a third of the county’s missing children, in a survey whose roster cannot be differenced.',
    section: 'people',
    era: [2013, 2022],
    assertions: ['four-catholic-schools-hold-four-fifths', 'the-private-school-fall-is-in-the-file'],
    entries: ['place/allen-county.yml', 'place/lima.yml'],
  },
  {
    slug: 'lima-before-the-series-begins',
    title: 'Lima before the series begins',
    dek: 'The city grew fivefold after the oil, and the county outside it had three peaks thirty years apart.',
    section: 'people',
    era: [1880, 1970],
    assertions: [
      'lima-grew-fivefold-after-the-oil',
      'three-peaks-thirty-years-apart',
      'every-township-grew',
    ],
    entries: [
      'place/lima.yml',
      'period/lima-oil-boom.yml',
      'question/pre-1970-population-series.yml',
    ],
  },
  {
    slug: 'the-county-in-1920',
    title: 'The county in 1920',
    dek: 'The census counted what the county history had estimated, the migration arrived in the forties, and in 2000 the category itself split.',
    section: 'people',
    era: [1920, 2020],
    assertions: [
      'the-census-counted-what-the-history-estimated',
      'the-migration-arrived-in-the-forties',
      'the-category-split-in-2000',
    ],
    entries: [
      'place/allen-county.yml',
      'place/lima.yml',
    ],
  },
  {
    slug: 'who-is-not-in-a-house-at-all',
    title: 'Who is not in a house at all',
    dek: 'Three thousand people here live outside a household, and one census block was drawn along a prison fence.',
    section: 'people',
    era: [2020, 2020],
    assertions: [
      'three-thousand-people-live-outside-households',
      'the-census-drew-the-block-on-the-fence',
    ],
    entries: [
      'site/allen-correctional-institution.yml',
      'site/oakwood-correctional-facility.yml',
      'question/who-lives-in-the-county-without-housing.yml',
    ],
  },
  {
    slug: 'buying-and-borrowing',
    title: 'Buying, and borrowing against what you own',
    dek: 'Seven years of mortgage applications, and a denial gap that income does not account for.',
    section: 'people',
    era: [2018, 2024],
    assertions: [
      'borrowing-against-a-house-in-lima',
      'the-denial-gap-is-seven-years-old',
      'the-lending-gap-is-not-composition',
      'the-denial-gap-that-does-not-resolve',
    ],
    entries: [
      'question/why-allen-countys-lending-outcomes-differ.yml',
      'place/lima.yml',
    ],
  },
  {
    slug: 'the-overdose-epidemic-ending',
    title: 'The overdose epidemic, ending',
    dek: 'Overdose deaths fell by two thirds from their peak, and the fall is steeper here than in the state.',
    section: 'people',
    era: [2017, 2025],
    assertions: ['overdose-deaths-fell-by-two-thirds'],
    entries: ['place/allen-county.yml'],
  },
  {
    slug: 'where-a-districts-money-comes-from',
    title: 'Where a district’s money comes from',
    dek: 'The district that spends the most per pupil raises the least of it locally.',
    section: 'people',
    era: [2023, 2023],
    assertions: ['lima-schools-spend-most-and-raise-least'],
    entries: ['place/lima.yml'],
  },
  {
    slug: 'who-governs-the-districts',
    title: 'Who governs the districts',
    dek: 'Twelve school boards, elected on a ballot most of the county never sees.',
    section: 'people',
    era: [2026, 2026],
    assertions: [],
    entries: ['question/why-one-child-in-five-is-not-in-these-districts.yml'],
  },
  {
    slug: 'after-the-twelfth-grade',
    title: 'After the twelfth grade',
    dek: 'The county lost a third of its college students, and its second-largest field of study is not a local job.',
    section: 'people',
    era: [2010, 2024],
    assertions: [
      'the-county-lost-a-third-of-its-college-students',
      'the-second-largest-field-of-study-is-not-a-local-one',
      'in-state-tuition-here-runs-eightfold',
    ],
    entries: [
      'organization/james-a-rhodes-state-college.yml',
      'organization/bluffton-university.yml',
      'organization/ohio-state-university-at-lima.yml',
      'organization/university-of-northwestern-ohio.yml',
    ],
  },

  // ── Work ────────────────────────────────────────────────────────────────
  {
    slug: 'the-rate-caught-up-with-ohio',
    title: 'The rate caught up with Ohio',
    dek: 'Twenty-four years above the state’s unemployment rate, and then a convergence made mostly of people leaving the labour force.',
    section: 'work',
    era: [1990, 2026],
    assertions: ['the-rate-converged-and-the-workforce-did-not'],
    entries: ['place/allen-county.yml'],
  },
  {
    slug: 'the-factories-left-lima',
    title: 'The factories left Lima',
    dek: 'They did not leave the county. Lima’s share of the factory workforce fell for forty years while the county’s held.',
    section: 'work',
    era: [1929, 1967],
    assertions: ['the-factories-left-lima-and-not-the-county'],
    entries: [
      'place/lima.yml',
      'period/deindustrialization.yml',
    ],
  },

  // ── Government ──────────────────────────────────────────────────────────
  {
    slug: 'the-democratic-column',
    title: 'The Democratic column',
    dek: 'Five elections added to the modern run, and a third of one column gone in eight years while the other grew.',
    section: 'government',
    era: [2000, 2020],
    assertions: ['the-democratic-column-lost-a-third-of-itself'],
    entries: ['place/allen-county.yml'],
  },
  {
    slug: 'the-government-below-the-county',
    title: 'The government below the county',
    dek: 'Thirteen townships, nine villages and a city — and in two of the villages, appointment is how the government gets filled.',
    section: 'government',
    era: [2026, 2026],
    assertions: [
      'the-townships-are-named',
      'appointment-is-a-form-of-local-government',
      'two-villages-cannot-fill-their-governments',
      'lima-elects-by-ward-and-fills-its-seats',
    ],
    entries: [
      'jurisdiction/allen-county-government.yml',
      'question/why-allen-countys-villages-are-staffed-by-appointment.yml',
    ],
  },
  {
    slug: 'who-polices-the-county',
    title: 'Who polices the county',
    dek: 'A step in the reported numbers that is not a crime wave, and what changed underneath it.',
    section: 'government',
    era: [1985, 2024],
    assertions: ['a-step-that-is-not-a-crime-wave'],
    entries: [
      'office/allen-county-sheriff.yml',
      'measure/allen-county-sheriff-offenses-2015-2024.yml',
    ],
  },
  {
    slug: 'the-mayors-and-where-the-office-went',
    title: 'The mayors, and where the office went',
    dek: 'Three books, four slots, a year with nobody in it, and a line of Lima mayors that reaches 1922.',
    section: 'government',
    era: [1842, 1922],
    assertions: ['the-mayoral-line-reaches-1922', 'the-1906-list-lost-a-year'],
    entries: [
      'office/mayor-of-lima.yml',
      'event/lima-adopts-commission-government-1922.yml',
    ],
  },
  {
    slug: 'twelve-boards-nine-ballots',
    title: 'Twelve boards, nine ballots',
    dek: 'Twelve governing boards hold ground in this county and nine of them are elected — on ballots that do not line up.',
    section: 'government',
    era: [2026, 2026],
    assertions: ['twelve-boards-and-nine-ballots'],
    entries: ['jurisdiction/allen-county-government.yml'],
  },
  {
    slug: 'what-the-bar-register-knows',
    title: 'What the bar register knows',
    dek: 'It dates every admission to the day and cannot say where anybody practised — and it dots a ninety-nine-year hole.',
    section: 'government',
    era: [1850, 2026],
    assertions: [
      'the-register-dates-them-but-cannot-place-them',
      'the-ninety-nine-year-hole-is-dotted',
    ],
    entries: ['place/allen-county.yml'],
  },
  {
    slug: 'three-cents-in-the-dollar',
    title: 'Three cents in the dollar',
    dek: 'The county’s public library raises three per cent of its money locally, and the rest arrives from Columbus.',
    section: 'government',
    era: [2024, 2024],
    assertions: ['three-per-cent-of-the-library-money-is-local'],
    entries: [
      'organization/lima-public-library.yml',
      'organization/bluffton-public-library.yml',
      'organization/delphos-public-library.yml',
    ],
  },
  {
    slug: 'three-rosters-one-checkable',
    title: 'Three rosters, one checkable',
    dek: 'A 1906 book prints a line of holders for every office, and only one of its three rosters can be tested against anything.',
    section: 'government',
    era: [1831, 1906],
    assertions: [
      'a-line-of-holders-for-every-office',
      'three-rosters-and-only-one-can-be-checked',
      'the-board-is-printed-as-a-shift-register',
    ],
    entries: [
      'office/allen-county-board-of-commissioners.yml',
      'office/allen-county-sheriff.yml',
    ],
  },
  {
    slug: 'the-county-changed-sides',
    title: 'The county changed sides',
    dek: 'It was Democratic ground for thirty years, and it did not turn with 1896 — it turned before.',
    section: 'government',
    era: [1856, 1908],
    assertions: ['the-county-changed-sides', 'the-county-did-not-turn-with-1896'],
    entries: ['place/allen-county.yml'],
  },

  // ── History ─────────────────────────────────────────────────────────────
  {
    slug: 'the-fort-across-the-line',
    title: 'The fort across the line',
    dek: 'The county’s first settlement, first post office and the fort a township is named for all stand nine hundred and ninety feet outside it.',
    section: 'history',
    era: [1812, 1848],
    assertions: ['the-county-is-settled-from-outside-itself'],
    entries: [
      'site/fort-amanda.yml',
      'event/the-building-of-fort-amanda-1812.yml',
      'event/settlers-occupy-fort-amanda-1817.yml',
      'place/amanda-township.yml',
    ],
  },
  {
    slug: 'who-the-county-is-named-for',
    title: 'Who the county is named for',
    dek: 'The earliest printed account of Ohio’s counties says a colonel of the War of 1812, and cannot name him.',
    section: 'history',
    era: [1820, 1847],
    assertions: ['a-colonel-of-that-name-in-the-war-of-1812'],
    entries: [
      'question/namesake-of-allen-county.yml',
      'event/erection-of-allen-county.yml',
    ],
  },
  {
    slug: 'the-road-under-four-names',
    title: 'The road under four names',
    dek: 'A county history totals its own railroad columns to the unit, forecasts from them, and the forecast does not follow.',
    section: 'history',
    era: [1854, 2026],
    assertions: [
      'the-page-adds-up-and-the-forecast-does-not',
      'no-passenger-train-calls-in-this-county',
    ],
    entries: [
      'event/the-first-railroad-reaches-allen-county.yml',
      'site/lima-pennsylvania-railroad-depot.yml',
      'organization/ohio-electric-railway.yml',
    ],
  },
  {
    slug: 'one-person-in-ten',
    title: 'One person in ten',
    dek: 'The county put 1,920 men into the Civil War and its roster never added up its own dead.',
    section: 'history',
    era: [1861, 1885],
    assertions: [
      'one-person-in-ten-went-to-war',
      'the-roster-never-totalled-its-own-dead',
      'more-died-at-corinth-than-anywhere',
    ],
    entries: [
      'period/the-civil-war-in-allen-county.yml',
      'event/the-first-lima-company-1861.yml',
    ],
  },
  {
    slug: 'three-times-a-crowd-came-to-the-jail',
    title: 'Three times a crowd came to the jail',
    dek: 'Sixty-one years, three crowds, and an office that passed from father to son because of the last of them.',
    section: 'history',
    era: [1872, 1933],
    assertions: ['three-times-a-mob-came-to-the-jail', 'the-office-passed-from-father-to-son'],
    entries: [
      'event/allen-county-jail-raid-1933.yml',
      'event/the-mob-at-the-allen-county-jail-1916.yml',
      'event/the-execution-of-andrew-brentlinger-1872.yml',
      'office/allen-county-sheriff.yml',
    ],
  },
  {
    slug: 'two-counts-of-one-war',
    title: 'Two counts of one war',
    dek: 'The county counted its World War soldiers twice and got different answers, and the man its Legion post is named for is in only one of them.',
    section: 'history',
    era: [1917, 1921],
    assertions: ['the-two-war-lists-do-not-join'],
    entries: ['period/the-world-war-in-allen-county.yml'],
  },
  {
    slug: 'a-hundred-and-sixty-congregations',
    title: 'A hundred and sixty congregations',
    dek: 'Forty-nine religious bodies report 58,696 adherents here, and a survey a century earlier counted differently on purpose.',
    section: 'history',
    era: [1906, 2020],
    assertions: ['a-hundred-and-sixty-congregations'],
    entries: [
      'organization/market-street-presbyterian-church.yml',
      'organization/trinity-methodist-episcopal-church.yml',
      'question/when-limas-first-congregation-was-organized.yml',
    ],
  },
  {
    slug: 'ninety-nine-papers',
    title: 'Ninety-nine papers',
    dek: 'Ninety-nine newspapers have been printed in Allen County. One is digitized, and forty-three have no library reporting a copy.',
    section: 'history',
    era: [1843, 2026],
    assertions: ['ninety-nine-papers-and-one-of-them-online'],
    entries: [
      'organization/the-lima-news.yml',
      'organization/the-lima-argus.yml',
      'organization/the-lima-citizen.yml',
      'organization/der-lima-courier.yml',
    ],
  },
  {
    slug: 'two-tank-plants',
    title: 'Two tank plants',
    dek: 'Two tank installations, two years and a category apart — and what the war did to a locomotive builder’s books.',
    section: 'history',
    era: [1939, 1947],
    assertions: [
      'the-county-had-two-tank-installations',
      'the-war-turned-the-locomotive-works-books',
    ],
    entries: [
      'site/lima-army-tank-plant.yml',
      'site/lima-tank-depot.yml',
      'organization/lima-locomotive-works.yml',
      'event/the-united-states-buys-the-tank-plant-ground-1951.yml',
    ],
  },
  {
    slug: 'one-hundred-and-seventy-one',
    title: 'One hundred and seventy-one',
    dek: 'The third war is counted here by its dead: one Army casualty for every 429 people the county held.',
    section: 'history',
    era: [1941, 1946],
    assertions: ['the-third-war-is-counted-by-its-dead'],
    entries: ['period/the-second-world-war-in-allen-county.yml'],
  },
  {
    slug: 'what-a-tract-book-knows',
    title: 'What a tract book knows',
    dek: 'The county has abstracted its conveyances by piece of ground since the federal patents, and a conveyance is dated whether or not anybody reported it.',
    section: 'history',
    era: [1831, 2026],
    assertions: [],
    entries: ['office/allen-county-recorder.yml'],
  },
  {
    slug: 'twenty-nine-on-the-register',
    title: 'Twenty-nine on the register',
    dek: 'Sixteen of the county’s National Register properties were signed on one day in 1982, and nothing on the list made anything.',
    section: 'history',
    era: [1970, 2026],
    assertions: ['sixteen-of-twenty-nine-are-one-days-work'],
    entries: [
      'site/allen-county-courthouse.yml',
      'site/lima-memorial-hall.yml',
    ],
  },
  {
    slug: 'what-the-sky-did',
    title: 'What the sky did',
    dek: 'A thirty-fold rise in severe weather reports that is mostly categories arriving — and eighty-four cents in every disaster dollar is one pandemic.',
    section: 'history',
    era: [1950, 2025],
    assertions: ['the-blizzard-is-not-in-the-weather-file', 'eighty-four-cents-in-the-dollar'],
    entries: [
      'event/the-tornadoes-of-april-1965.yml',
      'event/allen-county-declared-for-covid-19-2020.yml',
      'event/allen-county-declared-for-hurricane-katrina-2005.yml',
      'event/the-windstorm-of-22-june-2006.yml',
    ],
  },
]

/** One article by slug. Throws rather than rendering a page with a hole in it. */
export function article(slug: string): Article {
  const found = ARTICLES.find((a) => a.slug === slug)
  if (!found) throw new Error(`no article "${slug}" in the registry`)
  return found
}

/** Every article in one section, earliest era first. */
export function articlesIn(section: SectionKey): Article[] {
  return ARTICLES.filter((a) => a.section === section).toSorted(
    (a, b) => a.era[0] - b.era[0] || a.era[1] - b.era[1],
  )
}

/**
 * Every article that names one corpus node.
 *
 * This is the back-link, and it is the reason `entries` is declared rather than inferred from
 * prose: an entry page can say what has been written about its node without the reading side
 * keeping a second list.
 */
export function articlesAbout(nodeId: string): Article[] {
  return ARTICLES.filter((a) => a.entries.includes(nodeId))
}

/** The era of a piece, as the reading room prints it. */
export function eraLabel(a: Article): string {
  return a.era[0] === a.era[1] ? String(a.era[0]) : `${a.era[0]}–${a.era[1]}`
}

/**
 * Every declared id that does not resolve against the feed.
 *
 * Exported rather than kept in the test so that `read/index.astro` can refuse to build over a
 * broken declaration — the gate runs where the page is rendered, not only where it is tested.
 */
export function unresolvedDeclarations(): string[] {
  const knownAssertions = new Set(feedAssertions.map((a) => a.id))
  const knownNodes = new Set(nodes.map((n) => n.id))
  const bad: string[] = []
  for (const a of ARTICLES) {
    for (const id of a.assertions) {
      if (!knownAssertions.has(id)) bad.push(`${a.slug}: no assertion "${id}" in the feed`)
    }
    for (const id of a.entries) {
      if (!knownNodes.has(id)) bad.push(`${a.slug}: no node "${id}" in the feed`)
    }
  }
  return bad
}
