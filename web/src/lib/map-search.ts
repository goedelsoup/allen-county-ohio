import type { Article } from './articles'
import type { AtlasRecord } from './feeds'

export type AtlasSearchKind = 'record' | 'article'

/**
 * The small, public shape sent to the browser for local atlas search.
 *
 * It is built from the publication feeds and the article registry. It carries labels and
 * routing metadata only: corpus prose never crosses this boundary, so a search index cannot
 * accidentally publish a passage the publication gate withheld.
 */
export interface AtlasSearchItem {
  kind: AtlasSearchKind
  id: string
  label: string
  context: string
  when: string
  href: string
  /** Present only when the record has geometry the map can select. */
  node?: string
}

export interface AtlasReading {
  slug: string
  title: string
}

/** JSON safe to place inside a script element with Astro's `set:html`. */
export function inlineJson(value: unknown): string {
  return JSON.stringify(value)
    .replaceAll('<', '\\u003c')
    .replaceAll('\u2028', '\\u2028')
    .replaceAll('\u2029', '\\u2029')
}

const words = (value: string): string[] => normalizeSearchText(value).split(' ').filter(Boolean)

/** Punctuation and case do not decide whether two public labels match. */
export function normalizeSearchText(value: string): string {
  return value
    .normalize('NFKD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLocaleLowerCase('en-US')
    .replace(/[^a-z0-9]+/g, ' ')
    .trim()
    .replace(/\s+/g, ' ')
}

/** A feed span, without claiming that an open end means "present." */
export function searchDate(from: number | null, to: number | null): string {
  if (from === null) return 'Date unknown'
  if (to === null) return `${from}\u2013`
  return from === to ? String(from) : `${from}\u2013${to}`
}

const entryPath = (node: string): string => `/entry/${node.replace(/\.yml$/, '')}`

const contextLabel = (value: string): string => {
  const plain = value.replaceAll('-', ' ')
  return `${plain.charAt(0).toLocaleUpperCase('en-US')}${plain.slice(1)}`
}

/** Whether selecting this record can result in a mark or shape on the map. */
export function isMapSelectable(record: AtlasRecord): boolean {
  if (record.treatment === 'unplaced' || record.treatment === 'not-spatial') return false
  return record.anchors.some(
    (anchor) =>
      (anchor.lat !== null && anchor.lon !== null) ||
      anchor.points.length >= 2 ||
      (anchor.level !== null && anchor.geoid !== null),
  )
}

/** Build the browser index exclusively from already-published site data. */
export function buildAtlasSearchIndex(
  records: readonly AtlasRecord[],
  articles: readonly Article[],
): AtlasSearchItem[] {
  const recordItems: AtlasSearchItem[] = records.map((record) => ({
    kind: 'record',
    id: record.node,
    label: record.label,
    context:
      record.treatment === 'unplaced' || record.treatment === 'not-spatial'
        ? `${contextLabel(record.class)} entry \u00b7 no map position`
        : `${contextLabel(record.class)} entry`,
    when: searchDate(record.from, record.to),
    href: entryPath(record.node),
    ...(isMapSelectable(record) ? { node: record.node } : {}),
  }))

  const articleItems: AtlasSearchItem[] = articles.map((article) => ({
    kind: 'article',
    id: article.slug,
    label: article.title,
    context: `${contextLabel(article.section)} article`,
    when: searchDate(article.era[0], article.era[1]),
    href: `/read/${article.slug}`,
  }))

  return [...recordItems, ...articleItems]
}

interface Match {
  item: AtlasSearchItem
  rank: number
}

/**
 * Exact labels lead, followed by label prefixes, exact tokens, token prefixes and substrings.
 * Stable textual tie-breaks make the same index and query return the same order everywhere.
 */
export function searchAtlas(
  index: readonly AtlasSearchItem[],
  rawQuery: string,
  limit = 8,
): AtlasSearchItem[] {
  const query = normalizeSearchText(rawQuery)
  const take = Math.max(0, Math.floor(limit))
  if (!query || take === 0) return []

  const queryWords = words(query)
  const matches: Match[] = []

  for (const item of index) {
    const label = normalizeSearchText(item.label)
    const haystack = normalizeSearchText(`${item.label} ${item.id} ${item.context} ${item.when}`)
    const tokens = words(haystack)

    let rank = Number.POSITIVE_INFINITY
    if (label === query) rank = 0
    else if (label.startsWith(query)) rank = 10
    else if (queryWords.length === 1 && tokens.includes(query)) rank = 20
    else if (queryWords.every((part) => tokens.includes(part))) rank = 30
    else if (queryWords.every((part) => tokens.some((token) => token.startsWith(part)))) rank = 40
    else if (haystack.includes(query)) rank = 50
    else if (queryWords.every((part) => tokens.some((token) => token.includes(part)))) rank = 60

    if (Number.isFinite(rank)) matches.push({ item, rank })
  }

  return matches
    .toSorted(
      (a, b) =>
        a.rank - b.rank ||
        a.item.label.localeCompare(b.item.label, 'en-US', { sensitivity: 'base' }) ||
        a.item.kind.localeCompare(b.item.kind) ||
        a.item.id.localeCompare(b.item.id),
    )
    .slice(0, take)
    .map(({ item }) => item)
}

/** The article links a selected entry's map panel may show. */
export function readingByNode(articles: readonly Article[]): Record<string, AtlasReading[]> {
  const linked = new Map<string, Map<string, AtlasReading>>()

  for (const article of articles) {
    for (const node of article.entries) {
      const pieces = linked.get(node) ?? new Map<string, AtlasReading>()
      pieces.set(article.slug, { slug: article.slug, title: article.title })
      linked.set(node, pieces)
    }
  }

  return Object.fromEntries(
    [...linked.entries()]
      .toSorted(([a], [b]) => a.localeCompare(b))
      .map(([node, pieces]) => [
        node,
        [...pieces.values()].toSorted(
          (a, b) => a.title.localeCompare(b.title, 'en-US') || a.slug.localeCompare(b.slug),
        ),
      ]),
  )
}
