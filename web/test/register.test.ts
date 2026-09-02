import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'

const PAGES = join(import.meta.dirname, '../src/pages')

/**
 * The main line says what is true of Allen County; the margin says how this site came to know
 * it.
 *
 * This corpus's best instinct is to record what a claim cost to get, and for eighty-three
 * phases that instinct arrived in the main line: 129 of 340 paragraphs opened *for nine phases
 * this corpus described…*, *this site had never read it*, *publishing found it; reading had
 * not*. A reader who wanted to know about Allen County was handed the minutes of the meeting.
 *
 * The prose is not deleted — it is the reason to trust the page. It moves into `<Gloss>`,
 * which this check exempts.
 *
 * **What this bans is narrower than it first looks.** It is acquisition narration: phase
 * numbers, what this corpus used to hold, dates measured from the build. It is *not* the
 * site's epistemic voice — *this page will not tell you which district*, *nothing here says
 * why the building went where it did*, *the two are not joined here*. Those are claims about
 * the state of the evidence, which is what the site is for, and an earlier draft of this check
 * flagged forty of them.
 */
const BANNED: [RegExp, string][] = [
  [/\bphases?\b/i, 'phase talk'],
  [/\bthis (corpus|site) (had|has|held|spent|described|could|now)\b/i, 'acquisition narration'],
  [/\bhad never (read|held|measured|been)\b/i, 'acquisition narration'],
  [/\bpublishing found\b/i, 'acquisition narration'],
  [/\b(two days ago|last week|yesterday)\b/i, 'a date measured from the build'],
  [/\bsince the \w+ page was written\b/i, 'acquisition narration'],
]

/**
 * `/sources` is exempt, and is the only page that is.
 *
 * It is the audit: how much of this is sourced, how much inferred, what was withheld. Its
 * subject *is* the corpus's own conduct, so a rule that keeps the corpus out of the main line
 * would leave it with nothing to say.
 */
const EXEMPT = new Set(['sources.astro'])

function mainLine(source: string): string[] {
  const parts = source.split('---')
  let body = parts.length > 2 ? parts.slice(2).join('---') : source
  body = body.replace(/<style>[\s\S]*?<\/style>/g, '')
  body = body.replace(/<Gloss[\s\S]*?<\/Gloss>/g, '')
  return [...body.matchAll(/<p[^>]*>([\s\S]*?)<\/p>/g)]
    .map((m) => m[1].replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim())
    .filter(Boolean)
}

function offenders(source: string): string[] {
  return mainLine(source).flatMap((p) => {
    const hit = BANNED.find(([pattern]) => pattern.test(p))
    return hit ? [`${hit[1]}: ${p.slice(0, 90)}…`] : []
  })
}

const readingPages = readdirSync(PAGES)
  .filter((f) => f.endsWith('.astro') && !EXEMPT.has(f))
  .map((file) => ({ file, source: readFileSync(join(PAGES, file), 'utf8') }))

const articles = readdirSync(join(PAGES, 'read'))
  .filter((f) => f.endsWith('.astro') && f !== 'index.astro')
  .map((file) => ({ file, source: readFileSync(join(PAGES, 'read', file), 'utf8') }))

describe('the main line is about the county', () => {
  it.each(readingPages)('$file narrates no acquisition', ({ source }) => {
    expect(offenders(source)).toEqual([])
  })

  /**
   * The articles are baselined, not exempted.
   *
   * Thirty-five paragraphs across thirty-seven pieces still narrate their own acquisition.
   * They are gated against this number rather than against zero for the reason
   * `.yidam/lint-baseline.yml` gives: a check that fails on inherited debt gets switched off
   * and stays off. Lower it as pieces are revised; it must never rise.
   */
  const ARTICLE_BASELINE = 35

  it('does not let the articles get worse', () => {
    const found = articles.flatMap((a) => offenders(a.source).map((o) => `${a.file}: ${o}`))
    expect(found.length).toBeLessThanOrEqual(ARTICLE_BASELINE)
  })

  it('has a baseline that is not over-stated', () => {
    // The other half of the same argument: a baseline permitted to be wrong drifts, and one
    // that over-lists silently re-permits whatever it over-lists.
    const found = articles.flatMap((a) => offenders(a.source))
    expect(found.length, `baseline is ${ARTICLE_BASELINE}, actual is ${found.length} — lower it`)
      .toBeGreaterThan(ARTICLE_BASELINE - 5)
  })
})

describe('headings are claims, not continuations', () => {
  /**
   * A heading that opens with a conjunction is the visible seam of accretion: it was written
   * to be stitched onto whatever happened to be last. Five of them were on the old population
   * page alone — *And what happened to it*, *And the one that grew*, *But not because it is
   * old*.
   *
   * Advisory: h3s inside a merged movement legitimately continue the h2 above them.
   */
  const CONTINUATION = /<h2[^>]*>\s*(And|But|Or|So|Also|Then)\b/i

  it.each(readingPages)('$file opens no movement with a conjunction', ({ source }) => {
    expect(CONTINUATION.test(source)).toBe(false)
  })
})
