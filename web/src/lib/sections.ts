// The site's own arrangement, and the one place it differs from the corpus's.
//
// The feed tags each assertion with a `topic` — ten of them, named for where the data came
// from. Those names are the corpus's and are not negotiable from this side: changing one
// means editing a `&'static str` in `crates/publish/src/derived.rs`, regenerating the feeds
// and committing the bytes, which is a corpus-side change made to serve a presentation
// decision.
//
// So the mapping lives here instead. The corpus keeps its taxonomy, the site keeps its
// arrangement, and `sections.test.ts` holds the two together: every topic in the feed must
// map to a section, and every assertion must reach exactly one page.
//
// See `.yidam/decisions/a-page-is-an-argument-not-an-inbox.yml`.

import { assertions, type Assertion } from './feeds'

/** A reading page. The six of these are the site's argument; everything else is apparatus. */
export interface Section {
  href: string
  /** What the nav calls it. */
  label: string
  /** The question the page answers. One per page, and the page ends when it has. */
  question: string
}

/**
 * The way in.
 *
 * Not a tab. `/` is the map, and the map is reached from the wordmark — a masthead is where a
 * reader looks for the front door, and a front door listed among five destinations is one of
 * six things rather than the way in.
 *
 * It is kept out of `SECTIONS` deliberately and not as a technicality. The reading pages answer
 * *what does it mean*; the map answers *what is here and when*. Putting the map in the reading
 * row would restage the competition
 * [`a-page-is-an-argument-not-an-inbox`](../../.yidam/decisions/a-page-is-an-argument-not-an-inbox.yml)
 * diagnosed and this arrangement is careful not to repeat.
 */
export const MAP: Section = {
  href: '/',
  label: 'The map',
  question: 'What is here, and when was it?',
}

/**
 * The reading pages, in the order the nav shows them.
 *
 * Six is the cap, and `shape.test.ts` enforces it. The number is not sacred; what it protects
 * is: a tab list that grew to thirteen did so one reasonable addition at a time, and nothing
 * in the codebase was in a position to object.
 *
 * Five now. The county's own page was the sixth, and its furniture moved onto the map rather
 * than into a seventh place — the cap is relieved by one rather than breached.
 */
export const SECTIONS: Section[] = [
  {
    href: '/ground',
    label: 'Ground',
    question: 'Two thirds of the county is farms — so what happens to the lines drawn on it?',
  },
  {
    href: '/people',
    label: 'People',
    question: 'The county is shrinking. Is it also sorting?',
  },
  {
    href: '/work',
    label: 'Work',
    question: 'What happened to the factory county, and what stands in its place?',
  },
  {
    href: '/government',
    label: 'Government',
    question: 'Who governs this county, how are they chosen, and what does it cost?',
  },
  {
    href: '/history',
    label: 'History',
    question: 'What can this corpus date, and what does the shape of the gaps mean?',
  },
]

/**
 * The instruments.
 *
 * A reader reaches for these; they do not read them front to back. Keeping them out of the
 * reading row is the cut this site most needed — the map competed with the history page for the
 * same eye, and neither is what the other is for.
 *
 * Two now. The map left this row upward rather than sideways: it is `MAP`, at `/`, reached from
 * the wordmark. The finding that put it here was right and is not being overturned — what it
 * diagnosed was two things both trying to be an argument, and the map is now the index instead.
 */
export const INSTRUMENTS: Section[] = [
  { href: '/entry', label: 'Entries', question: 'What does the corpus hold, node by node?' },
  { href: '/sources', label: 'Sources', question: 'What is this site standing on?' },
]

/**
 * A view of the map, as a path.
 *
 * This is the link direction the site never had. Everything pointed *out* of the map — a mark
 * opened an entry, the page handed off to `/ground` — and nothing pointed in, so the map was
 * somewhere a reader arrived rather than somewhere the prose could send them. A sentence about
 * Lima in 1885 can now put its reader on Lima in 1885.
 *
 * The parameter names are the map's own and are read by `scripts/map.ts`; `coverage.test.ts`
 * holds every link built here to naming a node the feed publishes and a year the axis covers.
 */
export interface MapView {
  /** The year to stand in. Omitted, the map opens at the end of the record. */
  year?: number
  /** `year` narrows to the year itself; `era` shows the tile containing it. */
  grain?: 'year' | 'era'
  /** A node to open the panel on, as `class/name.yml`. */
  at?: string
  /** Ground layers, replacing the default set. */
  layers?: string[]
}

export function mapPath(view: MapView): string {
  const q = new URLSearchParams()
  if (view.year !== undefined) q.set('year', String(view.year))
  if (view.grain) q.set('grain', view.grain)
  if (view.at) q.set('at', view.at)
  if (view.layers) q.set('layers', view.layers.join(','))
  // `URLSearchParams` percent-encodes `/` and `,`, and neither needs it in a query value.
  // These paths are quoted in prose, in the README and in the decision that put the map at `/`,
  // and a reader who copies one should get back what they were shown — `at=place/lima.yml`
  // rather than `at=place%2Flima.yml`. Both forms decode identically on the way in.
  const query = q.toString().replaceAll('%2F', '/').replaceAll('%2C', ',')
  return query ? `${MAP.href}?${query}` : MAP.href
}

/** Every section key an article may declare. `/` takes no articles: it is the way in. */
export type SectionKey = 'ground' | 'people' | 'work' | 'government' | 'history'

export const SECTION_KEYS: SectionKey[] = ['ground', 'people', 'work', 'government', 'history']

/**
 * The feed's topic, mapped onto the site's section.
 *
 * `geography` is the interesting one: it is the corpus's name for claims about where things
 * are and where the lines around them run, which this site reads as ground. Ten of the
 * seventeen assertions on the old `/land` carried it, and nothing had noticed, because
 * `assertionsFor()` existed and no page called it.
 */
export const TOPIC_SECTION: Record<string, SectionKey> = {
  geography: 'ground',
  land: 'ground',
  population: 'people',
  housing: 'people',
  health: 'people',
  schools: 'people',
  work: 'work',
  government: 'government',
  elections: 'government',
  history: 'history',
}

/**
 * Every assertion the feed files under a topic this section takes.
 *
 * This is the mapping doing work rather than sitting in a test. `/sources` renders it, so the
 * arrangement the site imposes on the corpus's taxonomy is visible on the page whose subject
 * is what this site rests on — and a topic that maps nowhere shows up as a hole a reader can
 * see, not only as a red test.
 */
export function assertionsForSection(key: SectionKey): Assertion[] {
  return assertions.filter((a) => TOPIC_SECTION[a.topic] === key)
}

/** The feed topics this section takes, in the order the feed uses them. */
export function topicsForSection(key: SectionKey): string[] {
  return Object.entries(TOPIC_SECTION)
    .filter(([, section]) => section === key)
    .map(([topic]) => topic)
}

/** What the nav calls a section key. */
export function sectionLabel(key: SectionKey): string {
  const found = SECTIONS.find((s) => s.href === `/${key}`)
  if (!found) throw new Error(`no section "${key}"`)
  return found.label
}

/** Where a section key lives. */
export function sectionPath(key: SectionKey): string {
  return `/${key}`
}
