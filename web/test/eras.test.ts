import { describe, expect, it } from 'vitest'
import { atlas, type AtlasRecord } from '../src/lib/feeds'
import {
  ERA_STEPS,
  GROUND_VINTAGE,
  WATER_VINTAGE,
  admits,
  anchored,
  census,
  drawn,
  eraAt,
  extent,
  groundFidelity,
  overlays,
  ribbon,
  shaded,
  spine,
  standing,
  undrawn,
  view,
  type View,
} from '../src/lib/eras'

// The axis, tested away from the canvas.
//
// Two halves, the same posture `crates/chronology` and `crates/placement` take with their own
// corpus gates: synthetic records for the rules, and the real feed for the invariants the design
// rests on. **Counts are not pinned.** A test asserting 484 dated records fails on the next phase
// that catalogues anything, and a gate that fails for the wrong reason gets switched off.

const record = (over: Partial<AtlasRecord> = {}): AtlasRecord => ({
  node: 'measure/x.yml',
  class: 'measure',
  label: 'X',
  tier: 'verified',
  from: 1900,
  to: 1900,
  precision: 'year',
  open_end: 'instantaneous',
  undated: null,
  treatment: 'count',
  hops: 1,
  route_tier: 'verified',
  anchors: [{ node: 'place/lima.yml', lat: 40.74, lon: -84.11, geoid: null, level: null, via: [] }],
  ...over,
})

const eras = spine(atlas)

/** A one-year window, which is the strictest thing the map can be asked for. */
const at = (year: number, over: Partial<View> = {}): View => ({
  from: year,
  to: year,
  generous: false,
  hops: 3,
  ...over,
})

describe('the spine', () => {
  it('is the design system ramp, one tile per step', () => {
    expect(eras.map((e) => e.key)).toEqual(ERA_STEPS.map((s) => s.key))
    expect(eras.map((e) => e.ink)).toEqual(ERA_STEPS.map((s) => s.ink))
  })

  it('tiles the whole range with no gap and no overlap', () => {
    // This is the property the seven period nodes do not have and the reason they are not the
    // control. If it ever fails, the reader can land on a year the axis cannot show them.
    for (let i = 1; i < eras.length; i++) {
      expect(eras[i].from, `${eras[i - 1].key} → ${eras[i].key}`).toBe(eras[i - 1].to + 1)
    }
    for (const e of eras) expect(e.to).toBeGreaterThanOrEqual(e.from)
  })

  it('takes its outer ends from the record rather than from a constant', () => {
    const { from, to } = extent(atlas)
    expect(eras[0].from).toBeLessThanOrEqual(from)
    expect(eras[eras.length - 1].to).toBeGreaterThanOrEqual(to)
  })

  it('runs to the present, because an open end does', () => {
    // The last figure anyone published is not the last year anything is standing. A spine that
    // stopped there would show a county that ends whenever the corpus last measured it.
    expect(extent(atlas).to).toBeGreaterThanOrEqual(new Date().getUTCFullYear())
  })

  it('puts every year of the range in exactly one tile', () => {
    const { from, to } = extent(atlas)
    for (const year of [from, 1819, 1820, 1859, 1860, 1899, 1900, 1939, 1940, 1979, 1980, to]) {
      const hit = eras.filter((e) => year >= e.from && year <= e.to)
      expect(hit, `${year}`).toHaveLength(1)
    }
  })

  it('clamps rather than throwing outside its own ends', () => {
    expect(eraAt(1000, eras).key).toBe('pre1820')
    expect(eraAt(9999, eras).key).toBe('1980')
  })

  it('balances better than the decades it replaces', () => {
    // The whole argument for six tiles. Five of the six hold a comparable share; the closing
    // tile holds the bulk and is a known refinement, not a surprise. What is checked is that
    // no tile is empty — an axis with a dead sixth is an axis a reader learns to avoid.
    const dated = atlas.filter((r) => r.from !== null)
    for (const e of eras) {
      const held = dated.filter((r) => (r.from as number) >= e.from && (r.from as number) <= e.to)
      expect(held.length, e.key).toBeGreaterThan(0)
    }
  })
})

describe('what is standing at a year', () => {
  it('admits a bounded span at both ends and outside neither', () => {
    const r = record({ from: 1861, to: 1865 })
    expect([1860, 1861, 1863, 1865, 1866].map((y) => admits(r, y, false))).toEqual([
      false,
      true,
      true,
      true,
      false,
    ])
  })

  it('carries a running record forward and an instant nowhere', () => {
    const running = record({ from: 1820, to: null, open_end: 'running' })
    const instant = record({ from: 1933, to: null, open_end: 'instantaneous' })
    expect(admits(running, 2026, false)).toBe(true)
    expect(admits(instant, 1934, false)).toBe(false)
  })

  it('keeps the strict and generous readings apart', () => {
    // A division effective from 2020 admits 2024 and vouches for nothing there. The two are
    // separate questions; a map that merged them would invent a fact and never say so.
    const unvouched = record({ from: 2020, to: null, open_end: 'unvouched' })
    expect(admits(unvouched, 2024, false)).toBe(false)
    expect(admits(unvouched, 2024, true)).toBe(true)
    expect(admits(unvouched, 2020, false)).toBe(true)
  })

  it('never draws an undated record at any year', () => {
    const undated = record({ from: null, to: null, open_end: null, undated: 'no start property' })
    for (const y of [1769, 1900, 2026]) expect(admits(undated, y, true)).toBe(false)
  })

  it('drops a placement deeper than the warrant allows', () => {
    const shallow = record({ hops: 0 })
    const deep = record({ hops: 3 })
    expect(drawn([shallow, deep], at(1900, { hops: 0 }))).toEqual([shallow])
    expect(drawn([shallow, deep], at(1900, { hops: 3 }))).toHaveLength(2)
  })

  it('draws nothing that reached no ground', () => {
    expect(drawn([record({ hops: null, anchors: [] })], at(1900))).toEqual([])
  })
})

describe('the three states the map keeps apart', () => {
  const records = [
    record({ from: 1900, to: 1900 }),
    record({ from: 1800, to: 1800 }),
    record({ from: null, to: null, open_end: null }),
    record({ hops: null, anchors: [] }),
    record({ from: 1900, to: 1900, hops: 3 }),
  ]

  it('counts every record exactly once', () => {
    const c = census(records, at(1900, { hops: 1 }))
    expect(c.present + c.elsewhen + c.undated + c.unplaced + c.beyondWarrant).toBe(records.length)
  })

  it('separates absent-from-the-year from absent-from-the-record', () => {
    // The distinction the control exists for. A node the corpus cannot date is not a gap in the
    // county's history; conflating the two says the first and means the second.
    const c = census(records, at(1900, { hops: 1 }))
    expect(c).toEqual({ present: 1, elsewhen: 1, undated: 1, unplaced: 1, beyondWarrant: 1 })
  })

  it('files the undated before the unplaced, so neither is double-counted', () => {
    const both = record({ from: null, open_end: null, hops: null, anchors: [] })
    const c = census([both], at(1900))
    expect([c.undated, c.unplaced]).toEqual([1, 0])
  })

  it('accounts for every record in the corpus at any year', () => {
    for (const year of [1769, 1885, 1943, 2020, 2026]) {
      const c = census(atlas, at(year))
      expect(c.present + c.elsewhen + c.undated + c.unplaced + c.beyondWarrant, `${year}`).toBe(
        atlas.length,
      )
    }
  })

  it('leaves the corpus with something to show in every era', () => {
    // The failure the density ribbon exists to pre-empt, checked rather than trusted: a tile a
    // reader can land on and see an empty county reads as a broken page.
    for (const era of eras) {
      const midpoint = Math.floor((era.from + era.to) / 2)
      expect(census(atlas, at(midpoint)).present, era.key).toBeGreaterThan(0)
    }
  })
})

describe('where the drawn records go', () => {
  it('puts a record at every anchor it reached, and averages none of them', () => {
    // A district serving five townships is at all five. A centroid between them is a position
    // no source stated, and `crates/proximity` already refuses that move in the distance case.
    const many = record({
      anchors: [
        { node: 'place/a.yml', lat: 1, lon: 1, geoid: null, level: null, via: [] },
        { node: 'place/b.yml', lat: 2, lon: 2, geoid: null, level: null, via: [] },
      ],
    })
    const placed = anchored([many])
    expect(placed.map((a) => a.node).toSorted()).toEqual(['place/a.yml', 'place/b.yml'])
    for (const a of placed) expect(a.records).toEqual([many])
  })

  it('gathers rather than scatters, so a stack is one mark', () => {
    const placed = anchored([record(), record(), record()])
    expect(placed).toHaveLength(1)
    expect(placed[0].records).toHaveLength(3)
  })

  it('skips an anchor that is a key rather than a point', () => {
    const keyed = record({
      anchors: [{ node: 'jurisdiction/x.yml', lat: null, lon: null, geoid: '39003', level: 'county', via: [] }],
    })
    expect(anchored([keyed])).toEqual([])
  })

  it('draws a shape only for a node that states its own key', () => {
    // The tract is the case. It carries no key of its own and reaches the county government in
    // one edge, so shading its anchor would draw the whole county and call it a tract — which is
    // not where the tract is, only how far the corpus could reach from it.
    const stated = record({ treatment: 'polygon', hops: 0, anchors: [{ node: 'jurisdiction/x.yml', lat: null, lon: null, geoid: '3943554', level: 'place', via: [] }] })
    const derived = record({ treatment: 'polygon', hops: 1, anchors: [{ node: 'jurisdiction/y.yml', lat: null, lon: null, geoid: '39003', level: 'county', via: [] }] })
    expect(shaded([stated, derived]).map((s) => s.key)).toEqual(['place:3943554'])
  })

  it('refuses a key whose summary level the feed could not name', () => {
    // A bare GEOID identifies nothing — `3904752` is a village and a school district in this
    // county — so an anchor with no level is not addressable and is dropped rather than
    // resolved against whichever layer happens to hold that number.
    const unlevelled = record({
      treatment: 'polygon',
      hops: 0,
      anchors: [{ node: 'jurisdiction/z.yml', lat: null, lon: null, geoid: '3904752', level: null, via: [] }],
    })
    expect(shaded([unlevelled])).toEqual([])
  })

  it('shades a village only from the year it was incorporated', () => {
    // The payoff, checked on the corpus: the townships and the villages carry founding dates,
    // so travelling from 1820 to 1880 is watching the county fill in. If this inverts, the map
    // has stopped travelling and is drawing 2020 at every year.
    const early = shaded(drawn(atlas, at(1825))).length
    const late = shaded(drawn(atlas, at(1900))).length
    expect(late).toBeGreaterThan(early)
  })
})

describe('the ribbon', () => {
  it('covers every year of the spine and stays inside its tile', () => {
    const bars = ribbon(eras, atlas)
    for (const era of eras) {
      const mine = bars.filter((b) => b.era === era.key)
      expect(mine.length, era.key).toBeGreaterThan(0)
      expect(mine[0].from).toBe(era.from)
      expect(mine[mine.length - 1].to).toBe(era.to)
      for (let i = 1; i < mine.length; i++) expect(mine[i].from).toBe(mine[i - 1].to + 1)
    }
  })

  it('holds a bar no wider than a decade', () => {
    for (const bar of ribbon(eras, atlas)) expect(bar.to - bar.from).toBeLessThanOrEqual(9)
  })

  it('counts a span in every decade it crosses, not only the one it opens in', () => {
    const bars = ribbon(
      [{ key: 'x', label: 'X', ink: '--x', start: 1900, from: 1900, to: 1929 }],
      [record({ from: 1905, to: 1925 })],
    )
    expect(bars.map((b) => b.count)).toEqual([1, 1, 1])
  })

  it('carries a running span to the end of the axis', () => {
    const bars = ribbon(
      [{ key: 'x', label: 'X', ink: '--x', start: 1900, from: 1900, to: 1929 }],
      [record({ from: 1905, to: null, open_end: 'running' })],
    )
    expect(bars.map((b) => b.count)).toEqual([1, 1, 1])
  })

  it('shows the thin century before the reader travels through it', () => {
    // The ribbon's whole job. The first two tiles must read as sparse and the last as dense, or
    // an empty map looks like a bug rather than like a documented silence.
    const total = (key: string) =>
      ribbon(eras, atlas)
        .filter((b) => b.era === key)
        .reduce((sum, b) => sum + b.count, 0)
    expect(total('1980')).toBeGreaterThan(total('pre1820'))
  })
})

describe('the periods ride on top', () => {
  const over = overlays(atlas)

  it('offers every period the corpus dates, in order', () => {
    expect(over.length).toBe(atlas.filter((r) => r.class === 'period' && r.from !== null).length)
    expect(over.map((o) => o.from)).toEqual(over.map((o) => o.from).toSorted((a, b) => a - b))
  })

  it('does not tile, which is why it is not the axis', () => {
    // Stated as a check so nobody later "fixes" the spine by replacing it with these. The
    // periods overlap and leave holes; they are interpretations of spans, not a partition.
    const overlapping = over.some((a) =>
      over.some((b) => a !== b && a.from <= (b.to ?? Infinity) && b.from <= (a.to ?? Infinity)),
    )
    expect(overlapping).toBe(true)
  })

  it('keeps a running period open rather than closing it at a guess', () => {
    expect(over.some((o) => o.to === null)).toBe(true)
  })
})

describe('the ground does not travel', () => {
  it('is at full strength only at its own vintage', () => {
    expect(groundFidelity(2020)).toBe(1)
    expect(groundFidelity(2019)).toBeLessThan(1)
    expect(groundFidelity(2021)).toBeLessThan(1)
  })

  it('fades the further the reader gets from it, in both directions', () => {
    expect(groundFidelity(1900)).toBeLessThan(groundFidelity(1980))
    expect(groundFidelity(1885)).toBe(groundFidelity(2155))
  })

  it('floors rather than vanishing', () => {
    // A county with no outline is not an honest map; it is an empty one. The first value here
    // was 0.3 and the frame vanished under the township lines at 1832, which is what the floor
    // exists to prevent.
    expect(groundFidelity(1769)).toBeGreaterThanOrEqual(0.45)
    expect(groundFidelity(-5000)).toBe(groundFidelity(1769))
  })
})

describe('the window a year stands for', () => {
  it('is one year at year grain and the whole tile at era grain', () => {
    const strict = view(1885, 'year', eras, { generous: false, hops: 3 })
    const wide = view(1885, 'era', eras, { generous: false, hops: 3 })
    expect([strict.from, strict.to]).toEqual([1885, 1885])
    expect([wide.from, wide.to]).toEqual([1860, 1899])
  })

  it('carries the reader’s other two questions through untouched', () => {
    const v = view(1885, 'era', eras, { generous: true, hops: 1 })
    expect([v.generous, v.hops]).toEqual([true, 1])
  })

  it('shows more of the corpus at era grain than at year grain, in every tile', () => {
    // The reason both grains are on the page. The strict year is much the sparser reading — a
    // map that offered only it would read as an empty county in five tiles out of six.
    for (const era of eras) {
      const midpoint = Math.floor((era.from + era.to) / 2)
      const options = { generous: false, hops: 3 }
      const wide = drawn(atlas, view(midpoint, 'era', eras, options)).length
      const strict = drawn(atlas, view(midpoint, 'year', eras, options)).length
      expect(wide, era.key).toBeGreaterThanOrEqual(strict)
    }
  })

  it('counts a record standing anywhere in the window, not only at its start', () => {
    const civilWar = record({ from: 1861, to: 1865 })
    expect(standing(civilWar, { from: 1863, to: 1863, generous: false, hops: 3 })).toBe(true)
    expect(standing(civilWar, { from: 1900, to: 1939, generous: false, hops: 3 })).toBe(false)
  })
})

describe('the undated are drawn on their own toggle', () => {
  it('takes only records with no span at all', () => {
    const dated = record({ from: 1900 })
    const undated = record({ from: null, to: null, open_end: null })
    expect(undrawn([dated, undated], at(1900))).toEqual([undated])
  })

  it('is still held to the warrant control', () => {
    const deep = record({ from: null, to: null, open_end: null, hops: 3 })
    expect(undrawn([deep], at(1900, { hops: 1 }))).toEqual([])
  })

  it('leaves holes in the county that are holes in the record', () => {
    // Three shapes the corpus holds and cannot date — a township and two villages. They belong
    // to no year, so they are drawn on their own toggle rather than folded into one.
    const shapes = shaded(undrawn(atlas, at(2020)))
    expect(shapes.length).toBeGreaterThan(0)
    for (const s of shapes) expect(s.record.from).toBeNull()
  })

  it('never appears in a year’s own list', () => {
    const years = new Set(drawn(atlas, at(2020)).map((r) => r.node))
    for (const r of undrawn(atlas, at(2020))) expect(years.has(r.node)).toBe(false)
  })
})

describe('a layer fades from its own vintage', () => {
  it('measures the water from the year it was fetched, not from 2020', () => {
    // The boundaries are a 2020 statement and the water is not: TIGERweb serves hydrography from
    // a service with no vintage, so `PROVENANCE.json` records it as current. Fading it from 2020
    // would be a fade that lied about which year it is honest at.
    expect(groundFidelity(GROUND_VINTAGE)).toBe(1)
    expect(groundFidelity(WATER_VINTAGE, WATER_VINTAGE)).toBe(1)
    expect(groundFidelity(WATER_VINTAGE)).toBeLessThan(1)
  })

  it('keeps the same floor whatever it is measured from', () => {
    expect(groundFidelity(1769, WATER_VINTAGE)).toBe(groundFidelity(1763))
  })
})
