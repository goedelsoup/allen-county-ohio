import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { body as markup } from './astro'
import { INSTRUMENTS, MAP, SECTIONS } from '../src/lib/sections'

const PAGES = join(import.meta.dirname, '../src/pages')

/**
 * A reading page is an argument: one question, movements that turn the answer, and an ending.
 *
 * This file is the structural budget. Nothing in it is a law of nature — the numbers are a
 * judgement, and the reason they live in a test rather than in a comment is that arguing with
 * them should be cheap and changing them should be deliberate. What they protect is not
 * arbitrary: a nav that reached thirteen items and a page that reached 7,628 words both got
 * there one reasonable addition at a time, with nothing in the codebase in a position to
 * object.
 *
 * See `.yidam/decisions/a-page-is-an-argument-not-an-inbox.yml`.
 */

/** Prose only: paragraph text, excluding tables, chart captions and figure data. */
export function proseWords(source: string): number {
  const body = markup(source).replace(/<style>[\s\S]*?<\/style>/g, '')
  const paragraphs = [...body.matchAll(/<p[^>]*>([\s\S]*?)<\/p>/g)].map((m) => m[1])
  const text = paragraphs
    .join(' ')
    .replace(/<[^>]+>/g, ' ')
    .replace(/\{[^{}]*\}/g, ' X ')
  return text.split(/\s+/).filter(Boolean).length
}

/** Top-level `<section>`s, excluding the hand-off at the foot. */
export function movements(source: string): number {
  const sections = (source.match(/<section[\s>]/g) ?? []).length
  return sections - (source.includes('class="onward"') ? 1 : 0)
}

/**
 * Every reading page, which no longer includes the index.
 *
 * `/` is the map now, and the map is an instrument: it answers *what is here and when*, and a
 * word budget written for an argument does not apply to it. What governs the index instead is
 * `register.test.ts`, which reads every page in the directory.
 */
const readingPages = SECTIONS.map((section) => {
  const file = `${section.href.slice(1)}.astro`
  return { section, file, source: readFileSync(join(PAGES, file), 'utf8') }
})

/**
 * 1,800 words of prose, and prose is the main line only.
 *
 * `<Gloss>` content is outside `<p>` and so outside this count, which is deliberate: the
 * margin is apparatus and does not compete for the reader's attention the way a paragraph
 * does. It is not a loophole — a page cannot get under the budget by moving its argument into
 * the margin, because `register.test.ts` governs what may go there.
 *
 * `/people` was expected to need a raised budget: it carries four former pages, and the plan
 * that produced this restructure said so. It came in at 1,764 after the register pass, so the
 * exception was never taken. Add one here if a page earns it, with the argument beside it.
 */
const BUDGET: Record<string, number> = {}
const DEFAULT_BUDGET = 1800

describe('a reading page is an argument', () => {
  it.each(readingPages)('$file stays inside its word budget', ({ section, source }) => {
    const budget = BUDGET[section.href] ?? DEFAULT_BUDGET
    expect(proseWords(source)).toBeLessThanOrEqual(budget)
  })

  it.each(readingPages)('$file answers in at most eight movements', ({ source }) => {
    expect(movements(source)).toBeLessThanOrEqual(8)
  })

  it.each(readingPages)('$file hands off rather than stopping', ({ source }) => {
    // A page that runs out of sections has not ended; it has stopped. Every reading page
    // names what it did not cover and where that went.
    expect(source).toContain('class="onward"')
  })

  it('every section declares the question it answers', () => {
    for (const s of [...SECTIONS, ...INSTRUMENTS, MAP]) {
      expect(s.question, s.label).toMatch(/\?$/)
    }
  })
})

describe('the nav cannot grow back', () => {
  it('holds at most six reading tabs', () => {
    // It was thirteen. Six is the cap; the instruments are counted separately because they
    // are a different kind of thing and were the half that made the row unreadable.
    expect(SECTIONS.length).toBeLessThanOrEqual(6)
  })

  it('keeps the instruments out of the reading row', () => {
    expect(INSTRUMENTS.map((i) => i.href)).toEqual(['/entry', '/sources'])
    for (const i of INSTRUMENTS) {
      expect(SECTIONS.map((s) => s.href)).not.toContain(i.href)
    }
  })

  it('leaves the county no page of its own to grow back into', () => {
    // The index was a sixth reading tab and its furniture moved onto the map. If `/` ever
    // reappears in the reading row, the map has been demoted back to an instrument and the
    // arrangement this test guards has quietly come apart.
    expect(SECTIONS.map((s) => s.href)).not.toContain('/')
    expect(SECTIONS.length).toBe(5)
  })

  it('makes the map the way in rather than a tab', () => {
    // The whole of the rearrangement, checked in the data and in the markup together — which is
    // the lesson the instruments row already taught this file once.
    expect(MAP.href).toBe('/')
    expect(SECTIONS.map((s) => s.href)).not.toContain(MAP.href)
    expect(INSTRUMENTS.map((i) => i.href)).not.toContain(MAP.href)

    const layout = readFileSync(join(PAGES, '../layouts/Base.astro'), 'utf8')
    // The wordmark carries it, and carries `aria-current` — there is no tab to highlight, so
    // that attribute is the only thing telling a screen reader the reader is already there.
    expect(layout).toMatch(/class="wordmark"[\s\S]*?href=\{MAP\.href\}/)
    expect(layout).toMatch(/class="wordmark"[\s\S]*?aria-current=\{here === MAP\.href/)
  })

  it('makes the same cut in the accessibility tree', () => {
    // The check above holds the two groups apart in the data. For a while that was the only
    // place it held: the layout put `aria-label="Instruments"` on a bare <ul>, and an
    // accessible name on a list is neither a landmark nor announced on entry — so Map, Entries
    // and Sources were read out inside the landmark named Sections, on every built page — 583 of
    // them when it was found, 773 by the time it was fixed.
    //
    // A separation made in the data, in the styling and not in the accessibility tree is a
    // separation made for everybody except the readers who most need the nav to be legible.
    const layout = readFileSync(join(PAGES, '../layouts/Base.astro'), 'utf8')
    const landmarks = [...layout.matchAll(/<nav[^>]*aria-label="([^"]+)"/g)].map((m) => m[1])
    expect(landmarks).toEqual(['Sections', 'Instruments'])

    // And the name never goes back onto a list. `<ul aria-label>` is the exact shape of the
    // defect, and it is not obviously wrong on the page — which is why it lasted.
    expect(/<ul[^>]*aria-label/.test(layout)).toBe(false)
  })

  it('names a page that exists for every tab', () => {
    const pages = new Set(readdirSync(PAGES))
    for (const item of [...SECTIONS, ...INSTRUMENTS, MAP]) {
      const file = item.href === '/' ? 'index.astro' : `${item.href.slice(1)}.astro`
      const asDirectory = pages.has(item.href.slice(1))
      expect(pages.has(file) || asDirectory, item.href).toBe(true)
    }
  })
})
