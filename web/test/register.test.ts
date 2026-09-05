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

/** Every file under `dir` with `extension`, walked to any depth. */
function walk(dir: string, extension: string, prefix = ''): { file: string; source: string }[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory()
      ? walk(join(dir, e.name), extension, `${prefix}${e.name}/`)
      : e.name.endsWith(extension)
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

describe('a claim tag is carried in words', () => {
  /**
   * The one rule the badge has never been allowed to break, now checked rather than commented.
   *
   * Tier is a status distinction and a reader who cannot separate verdigris from ochre has to
   * be able to read the tier anyway. It is also what licenses the rest of the palette: the
   * status *marks* are deliberately exempt from the 3:1 contrast bar for graphical objects, on
   * the argument that the bar is for objects required to understand content and none of these
   * is — because the word is always there. `contrast.test.ts` holds the ink to its ratios and
   * relies on that argument; this is the half of it that lives in the markup.
   *
   * There were two badge components until the idiom was consolidated, and both followed the
   * rule by habit. A third would have had nothing to read.
   */
  it('renders the tier as text on every path through the badge', () => {
    const badge = readFileSync(join(COMPONENTS, 'entry/Badge.astro'), 'utf8')
    const body = badge.split('---').slice(2).join('---').replace(/<style>[\s\S]*<\/style>/, '')

    // The word is unconditional: `{label ?? badge.label}`, never inside a `&&` or a ternary
    // that can render nothing.
    expect(body).toMatch(/\{\s*label\s*\?\?\s*badge\.label\s*\}/)
    expect(body).not.toMatch(/\{[^}]*&&[^}]*label[^}]*\}/)
  })

  it('is the only badge in the tree', () => {
    const badges = componentFiles().filter(({ file }) => /badge/i.test(file))
    expect(badges.map((b) => b.file)).toEqual(['entry/Badge.astro'])
  })
})

describe('nothing on this page reaches the network', () => {
  /**
   * `web/README.md` states it plainly — *"Nothing here reaches the network at build time or at
   * read time"* — and for the life of the design port it was false. `Base.astro` carried a
   * Google Fonts stylesheet and two `preconnect`s, so every reader's browser announced itself
   * to a third party on every page load of a public civic reference.
   *
   * It was not a lie anyone told. The feeds are committed, the geometry is vendored, the build
   * is static, and the sentence was true of everything its author was thinking about. The
   * fonts arrived as a `<link>` because that is how fonts arrive, and nothing was in a
   * position to notice that the claim had stopped holding.
   *
   * So it is checked. The faces are in `public/fonts/`, written by `mise run fonts`.
   */
  /**
   * Everything a page can carry a URL in: components, layouts, **pages** and **stylesheets**.
   *
   * The first draft read components and layouts only, which is 29 files out of 143 — every one
   * of the 110 pages and every stylesheet went unchecked, and since only `.astro` was ever fed
   * to them, the `@import url(…)` and `url(…woff2)` patterns below were unreachable code
   * pretending to be a rule. Both holes were live: a Google Fonts `<link>` in a page, or an
   * `@import` at the top of `tokens.css`, restored the exact dependency this check was written
   * to remove and left the suite green.
   */
  const sources = [
    ...walk(COMPONENTS, '.astro'),
    ...walk(join(COMPONENTS, '../layouts'), '.astro'),
    ...walk(PAGES, '.astro'),
    ...walk(join(COMPONENTS, '../styles'), '.css'),
  ]

  it('reads the whole site, not a corner of it', () => {
    // A walk that silently stops covering a directory is the failure this check itself had.
    expect(sources.length).toBeGreaterThan(140)
    expect(sources.some(({ file }) => file.endsWith('.css'))).toBe(true)
    expect(sources.some(({ file }) => file === 'index.astro')).toBe(true)
  })

  it.each(sources)('$file fetches nothing from anywhere', ({ source }) => {
    // A stylesheet, a script, a preconnect, a `@import url(…)`, an <img> or a font from a host.
    // `preload` is exempt only for a same-origin path — `/fonts/…`, never `//` or `https://`.
    const reaches = [
      /<link[^>]+href=["']https?:\/\//i,
      /<link[^>]+href=["']\/\//i,
      /<script[^>]+src=["']https?:\/\//i,
      /rel=["'](?:dns-)?preconnect["']/i,
      /@import\s+url\(\s*["']?https?:/i,
      /url\(\s*["']?https?:\/\/[^)]*\.(?:woff2?|ttf|otf)/i,
    ].filter((pattern) => pattern.test(source))

    expect(reaches.map(String)).toEqual([])
  })

  it('serves every face it declares from its own directory', () => {
    const sheet = readFileSync(join(COMPONENTS, '../styles/tokens/fonts.css'), 'utf8')
    const urls = [...sheet.matchAll(/src:\s*url\(['"]?([^'")]+)/g)].map((m) => m[1])
    expect(urls.length).toBeGreaterThan(0)
    expect(urls.filter((url) => !url.startsWith('/fonts/'))).toEqual([])

    const shipped = new Set(readdirSync(join(COMPONENTS, '../../public/fonts')))
    expect(urls.filter((url) => !shipped.has(url.replace('/fonts/', '')))).toEqual([])
  })
})

describe('text colour goes through a role, never into the ramp', () => {
  /**
   * The night palette gives values to *roles*. A component that reaches past them into the base
   * ramp gets a literal, and a literal has no dark value — so it keeps its daylight ink on an
   * ink ground, silently, on every page that renders it.
   *
   * Three shipped that way and were found by measuring rather than by looking:
   *
   *   .date-chip           --parchment-50 on a ground that inverts to parchment   1.18:1
   *   .claim p.drop        --rubric-500 on --surface-page                          2.44:1
   *   a:active             --rubric-700 on --surface-page                          1.66:1
   *
   * `contrast.test.ts` could not see any of them: it sweeps role names, which is its stated
   * scope and the right one. This is the other half — the rule that everything carrying text
   * colour is a role in the first place.
   *
   * **Scoped to `color:` deliberately.** Borders, backgrounds and decoration inks still reach
   * into the ramp in about two dozen places. Those are hairlines and fills rather than text:
   * on an ink ground they lose contrast against it and read as absent, which is a cosmetic
   * regression and not a legibility one. Widening this check to them means giving each a night
   * value and re-judging it in both palettes, which is its own piece of work — see
   * `.yidam/decisions/the-design-system-is-an-upstream.yml`.
   */
  const RAMP = /(^|[\s:])color:\s*var\(--(?:parchment|ink|rubric|verdigris|indigo|ochre|foxing)-/

  const styled = [
    ...walk(COMPONENTS, '.astro'),
    ...walk(join(COMPONENTS, '../layouts'), '.astro'),
    ...walk(PAGES, '.astro'),
    ...walk(join(COMPONENTS, '../styles'), '.css'),
  ].filter(({ file }) => !file.startsWith('tokens/'))

  it.each(styled)('$file sets text colour from a role', ({ source }) => {
    const raw = source
      .split('\n')
      .filter((line) => RAMP.test(line))
      .map((line) => line.trim())
    expect(raw).toEqual([])
  })
})
