import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { parts } from './astro'
import { ARTICLES, unresolvedDeclarations } from '../src/lib/articles'
import { assertions, atlas, nodes } from '../src/lib/feeds'
import { spine, standing } from '../src/lib/eras'
import {
  SECTIONS,
  SECTION_KEYS,
  TOPIC_SECTION,
  assertionsForSection,
  topicsForSection,
} from '../src/lib/sections'

const PAGES = join(import.meta.dirname, '../src/pages')

/**
 * Everything the feed publishes reaches a reader, and everything an article declares resolves.
 *
 * The class of failure this catches is the one the restructure actually surfaced:
 * `/government` had an unbalanced `<section>`, so the block describing Lima's government sat
 * outside any section and its assertion would have been dropped in the move without a word.
 * An assertion that passes the publication gate and then reaches no page is work the corpus
 * did and the site threw away.
 */

function astroFiles(dir: string, prefix = ''): { file: string; source: string }[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory()
      ? astroFiles(join(dir, e.name), `${prefix}${e.name}/`)
      : e.name.endsWith('.astro')
        ? [{ file: prefix + e.name, source: readFileSync(join(dir, e.name), 'utf8') }]
        : [],
  )
}

const pages = astroFiles(PAGES)

/** Assertion id → the pages that render it in an `<AssertionCard>`. */
const rendered = new Map<string, string[]>()
for (const { file, source } of pages) {
  const { frontmatter, body, fenced } = parts(source)
  if (!fenced) continue
  const binding = new Map(
    [...frontmatter.matchAll(/const\s+(\w+)\s*=\s*assertion\(\s*'([^']+)'/g)].map((m) => [
      m[1],
      m[2],
    ]),
  )
  for (const m of body.matchAll(/assertion=\{(\w+)\}/g)) {
    const id = binding.get(m[1])
    if (!id) continue
    rendered.set(id, [...(rendered.get(id) ?? []), file])
  }
}

/**
 * The one assertion this site deliberately makes twice.
 *
 * *An address is not a municipality* is one of the three things the home page says are worth
 * knowing, and it is separately the argument `/ground` makes for drawing the county rather
 * than describing it. Both are the right place for it. Anything else appearing twice is a
 * section that was moved and not deleted.
 */
const REPEATED = new Set(['an-address-is-not-a-municipality'])

describe('every assertion reaches a reader', () => {
  it('leaves none orphaned', () => {
    const orphans = assertions.map((a) => a.id).filter((id) => !rendered.has(id))
    expect(orphans).toEqual([])
  })

  it('renders each on one page, bar the declared repeat', () => {
    const twice = [...rendered.entries()]
      .filter(([id, files]) => files.length > 1 && !REPEATED.has(id))
      .map(([id, files]) => `${id} on ${files.join(', ')}`)
    expect(twice).toEqual([])
  })

  it('does not let the repeat list over-state itself', () => {
    for (const id of REPEATED) {
      expect((rendered.get(id) ?? []).length, id).toBeGreaterThan(1)
    }
  })
})

describe('an article declares what it rests on', () => {
  it('names only ids the feed resolves', () => {
    expect(unresolvedDeclarations()).toEqual([])
  })

  it('has a file for every registry entry and an entry for every file', () => {
    const files = readdirSync(join(PAGES, 'read'))
      .filter((f) => f.endsWith('.astro') && f !== 'index.astro')
      .map((f) => f.replace(/\.astro$/, ''))
      .toSorted()
    expect(ARTICLES.map((a) => a.slug).toSorted()).toEqual(files)
  })

  it('renders the article whose slug it is named for', () => {
    for (const a of ARTICLES) {
      const source = readFileSync(join(PAGES, 'read', `${a.slug}.astro`), 'utf8')
      expect(source, a.slug).toContain(`<Article slug="${a.slug}">`)
    }
  })

  it('files every article under a section that exists', () => {
    for (const a of ARTICLES) {
      expect(SECTION_KEYS, a.slug).toContain(a.section)
    }
  })

  it('gives every section at least one piece', () => {
    for (const key of SECTION_KEYS) {
      expect(ARTICLES.filter((a) => a.section === key).length, key).toBeGreaterThan(0)
    }
  })

  it('orders eras forwards', () => {
    for (const a of ARTICLES) {
      expect(a.era[0], a.slug).toBeLessThanOrEqual(a.era[1])
    }
  })

  it('names entries that are corpus nodes', () => {
    const ids = new Set(nodes.map((n) => n.id))
    for (const a of ARTICLES) {
      for (const id of a.entries) expect(ids.has(id), `${a.slug} → ${id}`).toBe(true)
    }
  })
})

describe('the feed taxonomy maps onto the site arrangement', () => {
  it('maps every topic the feed uses', () => {
    // The corpus keeps its own taxonomy and the site keeps its own arrangement; this is the
    // one file where they meet, and an unmapped topic means an assertion with nowhere to go.
    const topics = new Set(assertions.map((a) => a.topic))
    for (const topic of topics) {
      expect(TOPIC_SECTION[topic], `feed topic "${topic}" maps to no section`).toBeDefined()
    }
  })

  it('maps only onto sections that exist', () => {
    const hrefs = new Set(SECTIONS.map((s) => s.href.slice(1)))
    for (const [topic, section] of Object.entries(TOPIC_SECTION)) {
      expect(hrefs.has(section), `${topic} → ${section}`).toBe(true)
    }
  })
})

describe('the mapping is rendered, not only declared', () => {
  /**
   * `assertionsFor(topic)` sat in `lib/feeds.ts` from the day the feed contract was written and
   * was called by nothing, which is how ten of the seventeen assertions on the old `/land` came
   * to carry the topic `geography` with nobody noticing. `TOPIC_SECTION` replaced it and spent
   * one commit in exactly the same condition: declared, gated by this file, read by no page.
   *
   * So the mapping is rendered on `/sources`, whose subject is what this site rests on, and
   * this is the tripwire on that — a mapping only a test reads is a mapping nobody checks
   * against the world.
   */
  it('is read by a page and not only by this file', () => {
    const consumers = pages.filter(
      ({ file, source }) => file !== 'read/index.astro' && /assertionsForSection/.test(source),
    )
    expect(consumers.map((c) => c.file)).toContain('sources.astro')
  })

  it('places every assertion the feed publishes', () => {
    const placed = SECTION_KEYS.reduce((n, key) => n + assertionsForSection(key).length, 0)
    expect(placed).toBe(assertions.length)
  })

  it('gives every section at least one topic', () => {
    for (const key of SECTION_KEYS) {
      expect(topicsForSection(key).length, key).toBeGreaterThan(0)
    }
  })
})

describe('the reading pages point at each other', () => {
  it('gives every reading page outbound links', () => {
    // Before the restructure there were two editorial cross-links between twelve topic pages.
    // A section that names nothing beyond itself is a page, not part of an argument.
    for (const s of SECTIONS) {
      const file = s.href === '/' ? 'index.astro' : `${s.href.slice(1)}.astro`
      const source = readFileSync(join(PAGES, file), 'utf8')
      const outbound = [...source.matchAll(/href="(\/[a-z/-]*)"/g)]
        .map((m) => m[1])
        .filter((href) => href !== s.href)
      expect(new Set(outbound).size, file).toBeGreaterThanOrEqual(2)
    }
  })
})

describe('nothing links at a page that was retired', () => {
  /**
   * The redirects in `astro.config.mjs` are for links somebody else kept — a bookmark, another
   * site, a printout. They are not a licence for this site to go on pointing at its own retired
   * pages: a redirect costs the reader a hop, and the link text goes stale in a way no redirect
   * can fix. One survived the restructure reading *the same sorting <a href="/housing">the
   * housing page</a> measures* — a page that no longer exists, named as though it did.
   *
   * The list is read out of the config rather than repeated here, so retiring the next page
   * extends this check without anybody remembering to.
   */
  const config = readFileSync(join(import.meta.dirname, '../astro.config.mjs'), 'utf8')
  const block = config.match(/redirects:\s*\{([\s\S]*?)\n\s*\}/)?.[1] ?? ''
  const retired = [...block.matchAll(/'(\/[a-z-]+)':/g)].map((m) => m[1])

  it('found the retired paths to check against', () => {
    // A regex that stops matching turns this whole describe into a no-op.
    expect(retired.length).toBeGreaterThan(0)
  })

  it.each(retired.map((path) => ({ path })))('nothing links to $path', ({ path }) => {
    const offenders = pages
      .filter(({ source }) => new RegExp(`href="${path}/?"`).test(source))
      .map(({ file }) => file)
    expect(offenders).toEqual([])
  })
})


/** Every `<OnTheMap …>` in a source, with the two attributes the gates care about. */
function mapLinks(source: string): { at?: string; year?: string }[] {
  return [...source.matchAll(/<OnTheMap([^>]*)>/g)].map((m) => ({
    at: /\bat="([^"]+)"/.exec(m[1])?.[1],
    year: /\byear=\{(-?\d+)\}/.exec(m[1])?.[1],
  }))
}

describe('the prose points into the map', () => {
  /**
   * The link direction this site never had.
   *
   * Everything pointed *out* of the map — a mark opened an entry, the page handed off to
   * `/ground` — and nothing pointed in, so the map was somewhere a reader arrived rather than
   * somewhere the prose could send them. Promoting it to `/` does not fix that on its own: a
   * front door nobody is sent back to from inside the house is still only a front door.
   *
   * So every reading page carries at least one, and every one of them has to name a node the feed
   * publishes and a year the axis covers. A link into the map that lands on nothing is worse than
   * no link, because it reads as the site having lost the thing it was pointing at.
   */
  const readingPages = SECTIONS.map((section) => {
    const file = `${section.href.slice(1)}.astro`
    return { file, source: readFileSync(join(PAGES, file), 'utf8') }
  })

  const all: { file: string; at?: string; year?: string }[] = []
  for (const { file, source } of readingPages) {
    for (const link of mapLinks(source)) all.push({ file, at: link.at, year: link.year })
  }

  it('gives every reading page a way into the map', () => {
    const silent = readingPages.filter(({ source }) => mapLinks(source).length === 0).map((p) => p.file)
    expect(silent, 'reading pages with no link into the map').toEqual([])
  })

  it('names only nodes the feed publishes', () => {
    const published = new Set(atlas.map((r) => r.node))
    const missing = all.filter((l) => l.at && !published.has(l.at)).map((l) => `${l.file}: ${l.at}`)
    expect(missing).toEqual([])
  })

  it('names only years the axis can stand in', () => {
    // The map clamps rather than throwing, so a year outside the spine would land the reader
    // somewhere they did not ask for and nothing would say so.
    const eras = spine(atlas)
    const from = eras[0].from
    const to = eras[eras.length - 1].to
    const outside = all
      .filter((l) => l.year !== undefined)
      .filter((l) => Number(l.year) < from || Number(l.year) > to)
      .map((l) => `${l.file}: ${l.year}`)
    expect(outside, `years outside ${from}–${to}`).toEqual([])
  })

  it('sends the reader to a year the node is actually standing in', () => {
    // A link naming both a node and a year is a claim that the two go together. If the node's
    // span does not admit the year, the panel opens on a mark the map is not drawing.
    const byNode = new Map(atlas.map((r) => [r.node, r]))
    const wrong = all
      .filter((l) => l.at && l.year !== undefined)
      .filter((l) => {
        const record = byNode.get(l.at as string)
        return record ? !standing(record, { from: Number(l.year), to: Number(l.year), generous: true, hops: 3 }) : false
      })
      .map((l) => `${l.file}: ${l.at} is not standing in ${l.year}`)
    expect(wrong).toEqual([])
  })
})
