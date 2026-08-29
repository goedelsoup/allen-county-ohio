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

/**
 * Corpus prose as inline HTML.
 *
 * Markdown links become their link text, not anchors: a corpus link points at a `.yml` file
 * that does not exist on this site, and an anchor to nowhere is worse than plain words.
 * Where the target is a node this site publishes, `linkTo` may resolve it to a real page.
 */
export function inline(text: string, linkTo?: (target: string) => string | undefined): string {
  let html = escapeHtml(stripMarkers(text))

  html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, (_match, label: string, target: string) => {
    const href = linkTo?.(target)
    return href ? `<a href="${href}">${label}</a>` : label
  })

  // Bold and inline code, in that order: a `**` inside a code span is not emphasis, and the
  // corpus writes `MUNI` **blank** with both on one line.
  html = html.replace(/`([^`]+)`/g, '<code>$1</code>')
  html = html.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')

  return html
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
