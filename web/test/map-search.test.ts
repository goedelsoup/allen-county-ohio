import { describe, expect, it } from 'vitest'
import type { Article } from '../src/lib/articles'
import type { AtlasRecord } from '../src/lib/feeds'
import {
  buildAtlasSearchIndex,
  inlineJson,
  readingByNode,
  searchAtlas,
  type AtlasSearchItem,
} from '../src/lib/map-search'
import { putText } from '../src/scripts/map-search'

const item = (over: Partial<AtlasSearchItem> = {}): AtlasSearchItem => ({
  kind: 'record',
  id: 'place/lima.yml',
  label: 'Lima',
  context: 'place entry',
  when: '1831\u2013',
  href: '/entry/place/lima',
  node: 'place/lima.yml',
  ...over,
})

const record = (over: Partial<AtlasRecord> = {}): AtlasRecord => ({
  node: 'place/lima.yml',
  class: 'place',
  label: 'Lima',
  tier: 'verified',
  from: 1831,
  to: null,
  precision: 'year',
  open_end: 'running',
  undated: null,
  treatment: 'mark',
  hops: 0,
  route_tier: 'verified',
  anchors: [
    {
      node: 'place/lima.yml',
      lat: 40.7426,
      lon: -84.1052,
      points: [],
      geoid: null,
      level: null,
      via: [],
    },
  ],
  ...over,
})

const article = (over: Partial<Article> = {}): Article => ({
  slug: 'lima-before-the-series-begins',
  title: 'Lima before the series begins',
  dek: 'Not indexed.',
  section: 'people',
  era: [1820, 1900],
  assertions: [],
  entries: ['place/lima.yml'],
  ...over,
})

describe('atlas search ranking', () => {
  const index = [
    item({ id: 'event/lima-flood.yml', label: 'Lima flood' }),
    item({ id: 'place/lima.yml', label: 'Lima' }),
    item({ id: 'office/lima-mayor.yml', label: 'Mayor of Lima' }),
    item({ id: 'site/north-lima.yml', label: 'North Lima station' }),
  ]

  it('orders exact, label-prefix, exact-token and token-prefix matches deterministically', () => {
    expect(searchAtlas(index, 'lima').map((result) => result.label)).toEqual([
      'Lima',
      'Lima flood',
      'Mayor of Lima',
      'North Lima station',
    ])
    expect(searchAtlas(index, 'lim').map((result) => result.label)).toEqual([
      'Lima',
      'Lima flood',
      'Mayor of Lima',
      'North Lima station',
    ])
  })

  it('returns no suggestions for an empty query and respects its limit', () => {
    expect(searchAtlas(index, '   ')).toEqual([])
    expect(searchAtlas(index, 'lima', 2)).toHaveLength(2)
  })
})

describe('the build-time index', () => {
  it('matches article titles and routes them to ordinary reading links', () => {
    const index = buildAtlasSearchIndex([record()], [article()])
    expect(searchAtlas(index, 'series begins')[0]).toMatchObject({
      kind: 'article',
      label: 'Lima before the series begins',
      href: '/read/lima-before-the-series-begins',
    })
  })

  it('keeps an unplaced record discoverable without offering a map selection', () => {
    const index = buildAtlasSearchIndex(
      [
        record({
          node: 'event/treaty.yml',
          class: 'event',
          label: 'Treaty of St. Marys',
          treatment: 'unplaced',
          anchors: [],
        }),
      ],
      [],
    )
    expect(index[0]).toMatchObject({
      href: '/entry/event/treaty',
      context: 'Event entry \u00b7 no map position',
    })
    expect(index[0]).not.toHaveProperty('node')
  })

  it('reduces article declarations to a deterministic node-to-reading map', () => {
    expect(
      readingByNode([
        article({ slug: 'z', title: 'Zulu' }),
        article({ slug: 'a', title: 'Alpha' }),
        article({ slug: 'a', title: 'Alpha' }),
      ]),
    ).toEqual({
      'place/lima.yml': [
        { slug: 'a', title: 'Alpha' },
        { slug: 'z', title: 'Zulu' },
      ],
    })
  })
})

describe('result text', () => {
  it('writes feed strings through textContent, leaving markup inert', () => {
    const target: { textContent: string | null } = { textContent: null }
    const hostile = '<img src=x onerror=alert(1)>'
    putText(target, hostile)
    expect(target.textContent).toBe(hostile)
  })

  it('cannot close the inline JSON script element', () => {
    const hostile = [{ label: '</script><script>alert(1)</script>\u2028' }]
    const encoded = inlineJson(hostile)
    expect(encoded).not.toContain('<')
    expect(JSON.parse(encoded)).toEqual(hostile)
  })
})
