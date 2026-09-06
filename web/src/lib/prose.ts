// Rendering corpus prose as HTML.
//
// A feed block is the corpus's own text, markdown and all. Three transformations, and no
// more: what is rendered has to stay recognisable as the sentence in the node file, because
// that is the whole claim the citation apparatus makes.

import type { Tier } from './feeds'

const MARKERS = /\[(verified|inference|open)\]/g

/** Escape before any markup is introduced. */
function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
}

/**
 * Strip claim markers from prose.
 *
 * The tag is not deleted — it is promoted to a badge beside the text. Leaving `[verified]`
 * inline as well would say it twice, and a reader who does not know the vocabulary reads it
 * as a typo.
 */
export function stripMarkers(text: string): string {
  return text.replace(MARKERS, '').replace(/\s{2,}/g, ' ').trim()
}

/** Every claim tag in a piece of prose, in order. */
export function markers(text: string): Tier[] {
  return [...text.matchAll(MARKERS)].map((m) => m[1] as Tier)
}

/** Markdown a corpus block may carry, in the order it has to be read. */
const LINK = /\[([^\]]+)\]\(([^)]+)\)/g
const CODE = /`([^`]+)`/g
const BOLD = /\*\*([\s\S]+?)\*\*/g
// Emphasis, not arithmetic: markdown's own rule is that the opener is followed by non-space
// and the closer preceded by it, which is what keeps `2 * 3 * 4` a sum. The corpus writes no
// spaced bare asterisk today and this costs nothing to hold to.
const ITALIC = /(?<!\*)\*(?!\*)(?!\s)([^*\n]*?[^*\s])\*(?!\*)/g

/**
 * ASCII punctuation the corpus escapes to mean the character itself.
 *
 * Deliberately not markdown's whole set. `&`, `<`, `>` and `"` are gone by the time this runs —
 * `escapeHtml` has turned them into entities — so a rule matching a backslash before one of them
 * would capture the ampersand and leave `lt;` standing. The corpus escapes `*` and `$` and
 * nothing else; the rest of the class is here because they are the ones markdown would.
 */
const ESCAPE = /\\([*_`~$#[\]()!.+-])/g

/**
 * Everything no emphasis rule may look inside, lifted out and put back at the end.
 *
 * Two kinds, and both were leaking. **Code spans**, because the corpus writes
 * `` `MUNI` **blank** `` and also `` `a**b` `` — with the span still in the string, a bold rule
 * scanning past it pairs the `**` *inside* the backticks with the next one outside and
 * emphasises the closing tag along with every word between. And **escapes**, because `\*` is how
 * the corpus writes a footnote marker, and it printed as a backslash and a star on four pages.
 *
 * Code is lifted first: a backslash inside a code span is a backslash, not an escape.
 *
 * The sentinel is a NUL, which `escapeHtml` cannot produce and no corpus file contains.
 */
function protect(text: string, code: (span: string) => string): { text: string; spans: string[] } {
  const spans: string[] = []
  const keep = (value: string) => {
    spans.push(value)
    return `\u0000${spans.length - 1}\u0000`
  }
  const lifted = text
    .replace(CODE, (_match, span: string) => keep(code(span)))
    .replace(ESCAPE, (_match, char: string) => keep(char))
  return { text: lifted, spans }
}

// The sentinel is a control character on purpose — see `protect`. A NUL is the one thing in
// this file that cannot appear in corpus prose or come out of `escapeHtml`, which is the whole
// property being relied on, so the rule against matching one is answered rather than obeyed.
// eslint-disable-next-line no-control-regex
const SENTINEL = /\u0000(\d+)\u0000/g

const restore = (text: string, spans: string[]) =>
  text.replace(SENTINEL, (_match, i: string) => spans[Number(i)])

/**
 * Corpus prose as inline HTML.
 *
 * Markdown links become their link text, not anchors: a corpus link points at a `.yml` file
 * that does not exist on this site, and an anchor to nowhere is worse than plain words.
 * Where the target is a node this site publishes, `linkTo` may resolve it to a real page.
 *
 * **Bold and italic, and bold *containing* italic.** This used to render bold alone, on a rule
 * that forbade a `*` inside the run — and that is worse than not supporting italic, because a
 * nested one made the whole bold fail to match and the sentence printed every marker it had:
 * *The first boat through Delphos was the* Marshall*, on 4 July 1845.* 117 nodes use single-star
 * emphasis, almost all of it newspaper and ship titles, which is what italic is for.
 */
export function inline(text: string, linkTo?: (target: string) => string | undefined): string {
  const { text: lifted, spans } = protect(escapeHtml(stripMarkers(text)), (c) => `<code>${c}</code>`)

  let html = lifted.replace(LINK, (_match, label: string, target: string) => {
    const href = linkTo?.(target)
    return href ? `<a href="${href}">${label}</a>` : label
  })

  // Bold before italic, and bold is non-greedy so `**a** and **b**` is two runs rather than one
  // swallowing the words between. Italic then runs inside what bold produced.
  html = html.replace(BOLD, '<strong>$1</strong>')
  html = html.replace(ITALIC, '<em>$1</em>')

  return restore(html, spans)
}

/**
 * Corpus prose as words, for somewhere markup cannot go.
 *
 * A `<meta name="description">` is the case that made this necessary: 192 built pages offered a
 * search result a sentence with `**` in it, because the lede was passed into an attribute
 * verbatim. `inline` is the wrong tool there — HTML in an attribute is not markup, it is
 * characters — so this keeps the words and drops every marker: the tag, the link syntax around
 * a label, the emphasis stars and the code backticks.
 */
export function plain(text: string): string {
  // Same protection as `inline`, and for the same reason: with `` `a**b` `` still in the string,
  // the bold rule pairs that `**` with the next one outside the span and eats the words between.
  const { text: lifted, spans } = protect(stripMarkers(text), (c) => c)
  return restore(lifted.replace(LINK, '$1').replace(BOLD, '$1').replace(ITALIC, '$1'), spans)
    .replace(/\s{2,}/g, ' ')
    .trim()
}

/**
 * A block reduced to a description: words, then a sensible length.
 *
 * The order is the whole of it. `lede.text.slice(0, 180)` cut the markup and not the markdown,
 * so a bold run longer than the slice lost its closing `**` and the opening one went into a
 * `<meta name="description">` — seven pages offered a search result a sentence starting with two
 * asterisks. Strip first, cut second.
 *
 * Cut at a word boundary and say so with an ellipsis, because a description broken mid-word
 * reads as a truncated string rather than as a summary.
 */
export function summary(text: string, max: number): string {
  const words = plain(text)
  if (words.length <= max) return words
  const cut = words.slice(0, max)
  const edge = cut.lastIndexOf(' ')
  return `${(edge > max * 0.6 ? cut.slice(0, edge) : cut).replace(/[,;:—–-]$/, '')}…`
}

/** A catalog path as a readable source name: `catalog/openelections-ohio.md` → the slug. */
export function sourceName(path: string): string {
  return (
    path
      .replace(/^.*catalog\//, '')
      .replace(/\.md$/, '')
      .replace(/-/g, ' ') || path
  )
}

/** `place/allen-county.yml` → `Allen County`-ish, for when no label is to hand. */
export function nodeName(id: string): string {
  return id.replace(/^.*\//, '').replace(/\.yml$/, '')
}
