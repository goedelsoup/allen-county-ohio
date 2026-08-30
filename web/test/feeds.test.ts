import { describe, expect, it } from 'vitest'
import {
  FEED_VERSION,
  assertion,
  assertions,
  edges,
  manifest,
  mapPoints,
  nodes,
  series,
} from '../src/lib/feeds'

// The feeds are written by `crates/publish`, which gates them. These tests are the other
// side of that contract: what this site assumes about the shape it is handed.

describe('the feed contract', () => {
  it('is the version every page here was written against', () => {
    expect(FEED_VERSION).toBe(1)
  })

  it('declares a publication ceiling that is not open', () => {
    expect(manifest.policy.ceiling).not.toBe('open')
  })

  it('carries no open claim anywhere in it', () => {
    // The rule with no exception. Checked over the serialized feed, because the question is
    // what the file contains, not what a struct meant.
    for (const [name, feed] of Object.entries({ nodes, edges, series, assertions, mapPoints })) {
      expect(JSON.stringify(feed), `${name} carries an open marker`).not.toContain('[open]')
    }
  })

  it('tags every published claim at or above the ceiling', () => {
    const allowed = new Set(['verified', 'inference'])
    for (const node of nodes) {
      for (const block of node.blocks) expect(allowed).toContain(block.tier)
    }
    for (const edge of edges) expect(allowed).toContain(edge.tier)
  })

  it('never points an edge at a node the reader cannot open', () => {
    const ids = new Set(nodes.map((n) => n.id))
    for (const edge of edges) {
      expect(ids.has(edge.from), `${edge.from} is missing`).toBe(true)
      expect(ids.has(edge.to), `${edge.to} is missing`).toBe(true)
    }
  })
})

describe('the assertions the pages render', () => {
  // Every id a page asks for by name. A page throws at build time on a missing one, but
  // this says which one and why without reading a stack trace.
  const used = [
    'county-population-decline',
    'decline-concentrated-in-lima',
    'decline-is-not-suburbanization',
    'three-places',
    'republican-share-rising',
    'democratic-ground-moved-fastest',
    'eleven-precincts',
    'an-address-is-not-a-municipality',
  ]

  it.each(used)('%s is in the feed', (id) => {
    expect(() => assertion(id)).not.toThrow()
  })

  it('gives every assertion at least one citation', () => {
    for (const a of assertions) expect(a.citations.length).toBeGreaterThan(0)
  })

  it('plots only numbers that appear in the passage cited beside them', () => {
    // The crate checks this too. Repeating it here is deliberate: this is the assumption the
    // charts are built on, and a chart that quietly plots something else is the failure the
    // whole apparatus exists to prevent.
    for (const a of assertions) {
      for (const figure of a.figures) {
        const quoted = a.citations.some((c) => c.span.includes(figure.literal))
        expect(quoted, `${a.id} plots "${figure.literal}", which it does not cite`).toBe(true)
      }
    }
  })

  // The pinned phrase moved once. It used to be "1970 is the start", the corpus's refusal to
  // read its own chart as showing a 1970 peak when 1970 was merely where its data began. The
  // 1990 Census volume settled that — the county peaked in 1980 — so the refusal was answered
  // by evidence and the assertion now states the peak. The same worry survives one subject
  // down, for Lima, whose series still begins at its own first federal observation.
  it('carries the corpus refusal that qualifies the population series', () => {
    const decline = assertion('county-population-decline')
    expect(decline.caveats.join(' ')).toContain('Lima peaked in 1970 rather than earlier')
  })
})

describe('the series the charts read', () => {
  it('orders every series in time', () => {
    for (const s of series) {
      const dates = s.points.map((p) => p.as_of)
      expect(dates, s.id).toEqual(dates.toSorted())
    }
  })

  it('keeps both 2020 population figures rather than choosing one', () => {
    // An enumeration and an estimates base, eleven people apart. Dropping either would make
    // the corpus's own caution about comparability disappear into a chart.
    const county = series.find((s) => s.id === 'place-allen-county::total-resident-population')
    const in2020 = county?.points.filter((p) => p.as_of.startsWith('2020')) ?? []
    expect(in2020).toHaveLength(2)
  })
})
