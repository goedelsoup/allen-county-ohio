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
 * The reading pages, in the order the nav shows them.
 *
 * Six is the cap, and `nav.test.ts` enforces it. The number is not sacred; what it protects
 * is: a tab list that grew to thirteen did so one reasonable addition at a time, and nothing
 * in the codebase was in a position to object.
 */
export const SECTIONS: Section[] = [
  {
    href: '/',
    label: 'The county',
    question: 'What is this place, and what does this site claim to know about it?',
  },
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
 * reading row is the cut this site most needed — for thirteen phases the map competed with
 * the history page for the same eye, and neither is what the other is for.
 */
export const INSTRUMENTS: Section[] = [
  { href: '/map', label: 'Map', question: 'Where is any of this?' },
  { href: '/entry', label: 'Entries', question: 'What does the corpus hold, node by node?' },
  { href: '/sources', label: 'Sources', question: 'What is this site standing on?' },
]

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
