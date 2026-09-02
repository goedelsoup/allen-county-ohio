import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'

const PAGES = join(import.meta.dirname, '../src/pages')
const COMPONENTS = join(import.meta.dirname, '../src/components')

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

function figureSpans(source: string): string[] {
  return [...source.matchAll(/<figure[\s\S]*?<\/figure>/g)].map((m) => m[0])
}

// Walked, not listed. `entry/class/` is two directories down, and a check that stops at one
// is a check that quietly stops covering whatever moves.
function componentFiles(dir = COMPONENTS, prefix = ''): { file: string; source: string }[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory()
      ? componentFiles(join(dir, e.name), `${prefix}${e.name}/`)
      : e.name.endsWith('.astro')
        ? [{ file: prefix + e.name, source: readFileSync(join(dir, e.name), 'utf8') }]
        : [],
  )
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

  it.each(articles)('$file narrates no acquisition either', ({ source }) => {
    // The articles were baselined at 35 when this check was written, in the manner of
    // `.yidam/lint-baseline.yml`, and the baseline was worked off rather than lived with.
    // There is nothing left to grandfather, so the check is the same one the reading pages get.
    expect(offenders(source)).toEqual([])
  })
})

describe('headings are claims, not continuations', () => {
  /**
   * A heading that opens with a conjunction is the visible seam of accretion: it was written
   * to be stitched onto whatever happened to be last. Five of them were on the old population
   * page alone — *And what happened to it*, *And the one that grew*, *But not because it is
   * old*.
   *
   * This was written to cover `h2` only, on the argument that an `h3` folded into a movement
   * legitimately continues the `h2` above it. That turned out to describe one heading of
   * eighteen, and it was a redundancy rather than a continuation — an `h2` reading *Not age,
   * and not suburbanization* over an `h3` reading *Nor is it suburbanization*. Both were
   * rewritten as claims, so the check covers both ranks and there is nothing to exempt.
   */
  const CONTINUATION = /<h[23][^>]*>\s*(And|But|Or|So|Also|Then|Nor)\b/i

  it.each(readingPages)('$file opens no movement with a conjunction', ({ source }) => {
    expect(CONTINUATION.test(source)).toBe(false)
  })

  it.each(articles)('$file opens no heading with a conjunction', ({ source }) => {
    expect(CONTINUATION.test(source)).toBe(false)
  })
})

describe('a figure is labelled, not headed', () => {
  /**
   * A heading inside a `<figure>` is a rank in the document outline that leads nowhere.
   *
   * `ChartFigure` and `DataTable` put the figure's title in an `h3` inside its own
   * `<figcaption>`, and `AssertionCard` labelled its caveat block `h4` in a card whose
   * statement is a paragraph. Under an article's `h1` that is a skipped rank, and it was one
   * on thirty-six pages — forty-six skips, every one of them a label a reader would never
   * navigate to. The type is unchanged; only the element is.
   *
   * The entry blocks keep their `h2`: those are sections of the entry page and sit under its
   * `h1`, which is what a heading is for.
   */
  const components = componentFiles()

  it.each(components)('$file puts no heading inside a figure', ({ source }) => {
    const headed = figureSpans(source).filter((span) => /<h[1-6][\s>]/.test(span))
    expect(headed).toEqual([])
  })
})

describe('an article heads its movements at the rank the layout styles', () => {
  /**
   * `Article.astro` styles `.piece-body h2` and nothing below it, so a piece that opens a
   * movement with an `h3` gets a heading the layout never sized and a rank skipped from the
   * `h1` above it. Two pieces did — both lifted out of a topic page where the `h3` sat under
   * that page's `h2`, and the rank came along with the prose.
   *
   * If a piece ever earns a sub-heading, the fix is to style `h3` in the layout and relax this
   * check. What it forbids is inheriting a rank from wherever the paragraphs used to live.
   */
  it.each(articles)('$file heads its movements at h2', ({ source }) => {
    const parts = source.split('---')
    const body = parts.length > 2 ? parts.slice(2).join('---') : source
    const ranks = [...body.matchAll(/<h([1-6])[\s>]/g)].map((m) => m[1])
    expect([...new Set(ranks)].toSorted()).toEqual(ranks.length > 0 ? ['2'] : [])
  })
})
