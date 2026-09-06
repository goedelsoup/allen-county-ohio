import { describe, expect, it } from 'vitest'
import { atlas, type AtlasRecord } from '../src/lib/feeds'
import { spine } from '../src/lib/eras'
import {
  START,
  STILL,
  advance,
  density,
  duration,
  mayStep,
  route,
  seek,
  type Cadence,
} from '../src/lib/motion'

// The transport, tested away from the clock.
//
// Same posture as `eras.test.ts`: synthetic records for the rules, the real feed for the
// invariants the design rests on, and **no pinned counts** — a test asserting the corpus holds
// 677 records fails on the next phase that catalogues anything.
//
// The rules under test are `an-animation-asserts-continuity`'s. Two of them are properties of
// this module and are checked here; the third — that every frame is a state the corpus states —
// is structural rather than testable, because the transport only ever hands `map.ts` a year the
// scrub could already be dragged to.

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
  anchors: [],
  ...over,
})

const cadence: Cadence = { step: 100, hold: 500 }
const eras = spine(atlas)

describe('density', () => {
  it('counts a record in every year it stands in', () => {
    const counts = density([record({ from: 1902, to: 1904 })], 1900, 1905)
    expect(counts).toEqual([0, 0, 1, 1, 1, 0])
  })

  it('reads an open end generously, so a running record reaches the last year', () => {
    const counts = density([record({ from: 1903, to: null, open_end: 'running' })], 1900, 1905)
    expect(counts).toEqual([0, 0, 0, 1, 1, 1])
  })

  it('ignores an undated record rather than placing it at the start', () => {
    expect(density([record({ from: null, to: null })], 1900, 1905)).toEqual([0, 0, 0, 0, 0, 0])
  })

  it('clips a record that starts before the window', () => {
    expect(density([record({ from: 1800, to: 1901 })], 1900, 1902)).toEqual([1, 1, 0])
  })
})

describe('the route', () => {
  const stops = route(atlas, eras, cadence)

  it('visits every year on the axis, including the empty ones', () => {
    // The pacing changes how long a year is held, never whether it is held. A route that
    // skipped the thin decades would hide the shape of the archive, which is the one thing
    // the ribbon under the axis exists to show.
    expect(stops[0].year).toBe(eras[0].from)
    expect(stops[stops.length - 1].year).toBe(eras[eras.length - 1].to)
    expect(stops).toHaveLength(eras[eras.length - 1].to - eras[0].from + 1)
    for (const [i, stop] of stops.entries()) expect(stop.year).toBe(eras[0].from + i)
  })

  it('never holds a year for zero, however empty it is', () => {
    for (const stop of stops) expect(stop.hold).toBeGreaterThanOrEqual(cadence.step)
  })

  it('holds no year longer than the ceiling', () => {
    for (const stop of stops) expect(stop.hold).toBeLessThanOrEqual(cadence.hold)
  })

  it('dwells on the full years and skims the thin ones', () => {
    // The invariant, not the numbers: whichever decade the corpus is thickest in must be held
    // longer than whichever it is thinnest in. Asserting *which* decade would pin a count.
    const holds = stops.map((s) => s.hold)
    expect(Math.max(...holds)).toBeGreaterThan(Math.min(...holds))
  })

  it('collapses to one uniform hold when the reader has asked for no motion', () => {
    const still = route(atlas, eras, STILL)
    expect(new Set(still.map((s) => s.hold))).toEqual(new Set([STILL.hold]))
    expect(still).toHaveLength(stops.length)
  })
})

describe('the playhead', () => {
  const stops = [
    { year: 1900, hold: 100 },
    { year: 1901, hold: 100 },
    { year: 1902, hold: 100 },
  ]

  it('holds a stop until its hold is spent', () => {
    expect(advance(START, stops, 40)).toEqual({ index: 0, elapsed: 40, done: false })
    expect(advance({ index: 0, elapsed: 40, done: false }, stops, 40).index).toBe(0)
  })

  it('steps once the hold is spent, carrying the remainder', () => {
    expect(advance({ index: 0, elapsed: 80, done: false }, stops, 30)).toEqual({
      index: 1,
      elapsed: 10,
      done: false,
    })
  })

  it('crosses several stops on one long delta rather than losing the time', () => {
    // A backgrounded tab hands back hundreds of milliseconds. Consuming one stop per call
    // would silently slow the route to whatever the frame rate happened to be.
    expect(advance(START, stops, 250)).toEqual({ index: 2, elapsed: 50, done: false })
  })

  it('ends on the last stop rather than past it', () => {
    const end = advance(START, stops, 10_000)
    expect(end).toEqual({ index: stops.length - 1, elapsed: 0, done: true })
    expect(advance(end, stops, 100)).toBe(end)
  })

  it('scales elapsed time by the rate rather than shrinking the holds', () => {
    // Dividing holds by the rate rounds a short hold to zero and skips the year. At 8x the
    // thinnest decades must still get a frame each.
    expect(advance(START, stops, 50, 2)).toEqual({ index: 1, elapsed: 0, done: false })
    const fast = route(atlas, eras, cadence)
    expect(Math.min(...fast.map((s) => s.hold))).toBeGreaterThan(0)
  })

  it('refuses to move on a negative delta', () => {
    expect(advance({ index: 1, elapsed: 20, done: false }, stops, -500)).toEqual({
      index: 1,
      elapsed: 20,
      done: false,
    })
  })

  it('does nothing on an empty route', () => {
    expect(advance(START, [], 100)).toBe(START)
  })
})

describe('seek', () => {
  const stops = route(atlas, eras, cadence)

  it('lands on the year asked for', () => {
    expect(stops[seek(stops, 1885).index].year).toBe(1885)
  })

  it('clamps past the end rather than running off the route', () => {
    const head = seek(stops, 9999)
    expect(head.index).toBe(stops.length - 1)
    expect(head.done).toBe(false)
  })

  it('clamps before the start to the first stop', () => {
    expect(seek(stops, 1).index).toBe(0)
  })
})

describe('duration', () => {
  it('is the sum of the holds, divided by the rate', () => {
    const stops = [
      { year: 1900, hold: 100 },
      { year: 1901, hold: 300 },
    ]
    expect(duration(stops)).toBe(400)
    expect(duration(stops, 4)).toBe(100)
  })

  it('is long enough to be worth a control saying so, and short enough to watch', () => {
    // Not a pinned number: a bound. A play that takes under ten seconds is a flash, and one
    // that takes over five minutes is not a control anybody uses.
    const whole = duration(route(atlas, eras, cadence))
    expect(whole).toBeGreaterThan(10_000)
    expect(whole).toBeLessThan(300_000)
  })
})

describe('precision floors the grain', () => {
  // The third rule of `an-animation-asserts-continuity`. A record dated only to the year is
  // drawn entire or not at all — stepping through it would draw a specific state at a specific
  // moment inside an interval the source resolved no further.

  it('refuses a year-precision record', () => {
    expect(mayStep(record({ from: 1820, to: 1848, precision: 'year' }))).toBe(false)
  })

  it('allows a day-precision record that spans more than one year', () => {
    expect(mayStep(record({ from: 1974, to: 1980, precision: 'day' }))).toBe(true)
  })

  it('allows a month-precision record over several years', () => {
    expect(mayStep(record({ from: 2004, to: 2005, precision: 'month' }))).toBe(true)
  })

  it('refuses an instant, whatever its precision', () => {
    expect(mayStep(record({ from: 1965, to: 1965, precision: 'day' }))).toBe(false)
  })

  it('refuses an undated record', () => {
    expect(mayStep(record({ from: null, to: null, precision: null }))).toBe(false)
  })

  it('refuses most of the corpus, which is the point', () => {
    // The rule is a refusal, so the invariant worth holding is that it refuses far more than
    // it admits. A change that made this permissive would be the defect the rule exists for.
    const steppable = atlas.filter(mayStep)
    expect(steppable.length).toBeGreaterThan(0)
    expect(steppable.length).toBeLessThan(atlas.length / 2)
  })
})
