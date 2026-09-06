import { describe, expect, it } from 'vitest'
import {
  FEED_VERSION,
  assertion,
  assertions,
  comparability,
  comparabilityFor,
  comparableWith,
  edges,
  judgedApart,
  manifest,
  mapPoints,
  nodes,
  series,
  seriesById,
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
    for (const [name, feed] of Object.entries({
      nodes,
      edges,
      series,
      comparability,
      assertions,
      mapPoints,
    })) {
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

const judgedTable = (id: string) => comparability.find((c) => c.node === id)

describe('the comparability tables', () => {
  it('is what the entry renders, and every row opens', () => {
    const ids = new Set(nodes.map((n) => n.id))
    for (const table of comparability) {
      expect(ids.has(table.node), `${table.node} is not published`).toBe(true)
      for (const row of table.rows) {
        expect(ids.has(row.node), `${table.node} → ${row.node} is not published`).toBe(true)
      }
    }
  })

  it('says why, on every judged row', () => {
    // The rule `edge-audit` gates in the corpus, asserted again on the shape the site is
    // handed: a table can never print *Not comparable* and stop there.
    for (const table of comparability) {
      for (const row of table.rows.filter((r) => !r.this_figure)) {
        expect(typeof row.comparable, `${table.node} → ${row.node}`).toBe('boolean')
        expect(row.because?.trim(), `${table.node} → ${row.node}`).toBeTruthy()
        expect(row.tier, `${table.node} → ${row.node}`).toBeTruthy()
      }
    }
  })

  it('gives each table exactly one row for its own figure, and at least one other', () => {
    // A table of one row reads as a series of one, which is a claim about continuity nobody
    // made. It is the state this feed is built to leave empty rather than to fill in.
    for (const table of comparability) {
      expect(table.rows.filter((r) => r.this_figure)).toHaveLength(1)
      expect(table.rows.length).toBeGreaterThan(1)
      const self = table.rows.find((r) => r.this_figure)!
      expect(self.node).toBe(table.node)
      expect(self.comparable).toBeNull()
    }
  })

  it('is written once and reaches both ends, agreeing at each', () => {
    // The corpus writes the judgement on the later figure only. The earlier node needs the
    // warning more, being the one that cannot know its successor moved the definition.
    for (const table of comparability) {
      for (const row of table.rows.filter((r) => !r.this_figure)) {
        const other = judgedTable(row.node)
        expect(other, `${row.node} has no table but is judged from ${table.node}`).toBeTruthy()
        const back = other!.rows.find((r) => r.node === table.node)
        expect(back, `${row.node} does not carry ${table.node} back`).toBeTruthy()
        expect(back!.comparable, `${table.node} and ${row.node} disagree`).toBe(row.comparable)
      }
    }
  })

  it('is ordered in time, so a break reads where it happened', () => {
    for (const table of comparability) {
      const dates = table.rows.map((r) => r.as_of)
      expect(dates, table.node).toEqual(dates.toSorted())
    }
  })

  it('is a judgement and not the parameter-string join a series is', () => {
    // The whole reason this feed exists. `series` groups by matching `parameter` against a
    // shared subject, which is mechanical; the corpus's own minor-civil-division series puts
    // 56,580 in 1910 beside 27,132 in 1930, and the whole of that fall is Lima leaving the
    // table. The grouping says nothing about it; the judgement does.
    const grouped = series.find(
      (s) => s.id === 'place-allen-county::total-resident-population-by-minor-civil-division,-decennial',
    )
    expect(grouped?.points.map((p) => p.published)).toEqual(['56580', '27132'])

    const table = comparabilityFor('measure/allen-county-townships-1890-1910.yml')
    const verdict = table?.rows.find((r) => !r.this_figure)
    expect(verdict?.comparable).toBe(false)
    expect(verdict?.because).toContain('Lima')
  })

  it('does not claim a judgement where the corpus made none', () => {
    // Most measures have no table, and that is the honest state rather than a gap.
    const measures = nodes.filter((n) => n.class === 'measure')
    expect(comparability.length).toBeLessThan(measures.length)
    expect(comparabilityFor('measure/allen-county-population-1970.yml')).toBeUndefined()
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

  // The pinned phrase has moved twice, and both retired ones are recorded here so the next
  // reader knows what this test was for:
  //
  //   "1970 is the start"                        retired by the 1990 Census volume
  //   "Lima peaked in 1970 rather than earlier"  retired by the 1960 Census volume
  //
  // Both were refusals about where the corpus's data began, and retrieval answered both — the
  // county peaked in 1980, Lima in 1970 with twelve rising counts behind it. The pin is now
  // the refusal no retrieval answers: the period is named for manufacturing employment and the
  // corpus has no measurement of it inside the period.
  //
  // The phrase narrowed once, without the refusal being answered. Lima's wage earners for 1899,
  // 1904 and 1909 arrived and falsified the old wording — "does not measure it at all" — while
  // leaving the claim it protects untouched. The pin was the narrow phrase on purpose.
  //
  //   "*manufacturing employment* inside this period"  ANSWERED by County Business Patterns
  //
  // Thirty-six years of Allen County manufacturing employment, all of it inside the period:
  // 15,762 in 1986 down to 7,127 in 2010. Three retirements and an answer, and the decision
  // that set the narrow pin said a fourth event was the moment to ask whether the caveat still
  // had a subject. It does, and a better one. What the chart invites is a causal reading, and
  // the corpus now declines it not for want of a measurement but because the two series
  // diverge: 2010 to 2022, private employment fell 3,709 while manufacturing rose 1,446.
  // Causation stays something retrieval cannot answer.
  it('carries the corpus refusal that qualifies the population series', () => {
    const decline = assertion('county-population-decline')
    expect(decline.caveats.join(' ')).toContain('does not establish that')
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

describe('drawing a line through a series', () => {
  const county = seriesById('place-allen-county::total-resident-population')
  const lima = seriesById('place-lima::total-resident-population')

  it('keeps the enumeration off the estimates line, because the corpus says so', () => {
    // The pair the site had been separating by matching `-census` in a filename. The judgement
    // it stands on is `allen-county-population-2020 --not-comparable-to-> the census count`.
    const line = comparableWith(county.points, 'measure/allen-county-population-2024.yml')
    expect(line.map((p) => p.node)).toEqual([
      'measure/allen-county-population-1970.yml',
      'measure/allen-county-population-2000.yml',
      'measure/allen-county-population-2010.yml',
      'measure/allen-county-population-2020.yml',
      'measure/allen-county-population-2024.yml',
    ])
    expect(comparableWith(lima.points, 'measure/lima-population-2024.yml').map((p) => p.node)).toEqual([
      'measure/lima-population-2000.yml',
      'measure/lima-population-2010.yml',
      'measure/lima-population-2020.yml',
      'measure/lima-population-2024.yml',
    ])
  })

  it('draws the other line when the other figure is the anchor', () => {
    // The judgement says the two may not share a line. It does not say which one to keep, and
    // this is not computed from the graph: a page that wants the enumeration asks for it and
    // gets the estimates dropped instead.
    const line = comparableWith(county.points, 'measure/allen-county-population-2020-census.yml')
    expect(line.map((p) => p.node)).not.toContain('measure/allen-county-population-2020.yml')
    expect(line.map((p) => p.node)).not.toContain('measure/allen-county-population-2024.yml')
    expect(line.map((p) => p.node)).toContain('measure/allen-county-population-2010.yml')
  })

  it('keeps a figure the corpus has said nothing about', () => {
    // Silence is not a judgement of not-comparable. 2000 and 2010 are unjudged against
    // anything and stay on the line; if that ever becomes a judgement, this test is where the
    // consequence shows up.
    expect(judgedApart('measure/allen-county-population-2010.yml', 'measure/allen-county-population-2000.yml')).toBe(false)
    expect(comparableWith(county.points, 'measure/allen-county-population-2024.yml').map((p) => p.as_of)).toContain('2010-04-01')
  })

  it('refuses an anchor that is not in the series', () => {
    expect(() => comparableWith(county.points, 'measure/lima-population-2024.yml')).toThrow()
  })

  it('reads a judgement in either direction', () => {
    const [a, b] = ['measure/lima-population-2020.yml', 'measure/lima-population-2020-census.yml']
    expect(judgedApart(a, b)).toBe(true)
    expect(judgedApart(b, a)).toBe(true)
  })
})
