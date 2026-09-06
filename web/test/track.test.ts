import { describe, expect, it } from 'vitest'
import { nodes } from '../src/lib/feeds'
import {
  DASH_METRES,
  GAP_METRES,
  bearing,
  dashes,
  frame,
  metres,
  parseTrack,
  project,
  straightMiles,
} from '../src/lib/track'

// The track figure's arithmetic, away from the page.
//
// The corpus half is the part worth guarding: two events carry a track today, both out of the
// same federal file, and each states a track length of its own that this file's straight-line
// distance is checked against. Those two numbers agreeing to within a fifth of a mile is what
// identifies the coordinates as belonging to the storm — the check the nodes' own prose makes in
// words, made again here in numbers.

const tracked = nodes.filter((n) => n.class === 'event' && n.properties.track)

/** The track length a node states in its own prose, which is not the straight line between ends. */
const statedMiles = (text: string): number | null => {
  const found = /([\d.]+)-mile track/.exec(text)
  return found ? Number(found[1]) : null
}

describe('parsing', () => {
  it('reads a two-point track', () => {
    expect(parseTrack('40.80, -84.20; 40.87, -83.87')).toEqual([
      { lat: 40.8, lon: -84.2 },
      { lat: 40.87, lon: -83.87 },
    ])
  })

  it('tolerates loose spacing', () => {
    expect(parseTrack('40.80,-84.20;40.87,-83.87')).toHaveLength(2)
    expect(parseTrack('  40.80 ,  -84.20 ;  40.87 , -83.87 ')).toHaveLength(2)
  })

  it('rejects the whole value when one pair is bad, rather than reading half of it', () => {
    // A partial read is the dangerous failure: it would draw a one-point track and silently
    // lose half the claim, and the figure would look finished.
    expect(parseTrack('40.80, -84.20; nonsense')).toEqual([])
    expect(parseTrack('40.80, -84.20; 40.87')).toEqual([])
    expect(parseTrack('40.80, -84.20, 1; 40.87, -83.87')).toEqual([])
  })

  it('rejects the Unicode minus the prose is written with', () => {
    // `location_description` writes U+2212 throughout. Accepting it here would let one back
    // into a property that has to stay machine-readable.
    expect(parseTrack('40.80, −84.20; 40.87, −83.87')).toEqual([])
  })

  it('rejects positions off the globe', () => {
    expect(parseTrack('91.0, -84.20; 40.87, -83.87')).toEqual([])
    expect(parseTrack('40.80, -184.20; 40.87, -83.87')).toEqual([])
  })

  it('refuses a single point, which is not a track', () => {
    expect(parseTrack('40.80, -84.20')).toEqual([])
  })

  it('is empty on an absent property', () => {
    expect(parseTrack(undefined)).toEqual([])
    expect(parseTrack('')).toEqual([])
  })
})

describe('projection', () => {
  it('grows y northward, which the frame then flips', () => {
    expect(project({ lat: 41, lon: -84 }).y).toBeGreaterThan(project({ lat: 40, lon: -84 }).y)
  })

  it('puts north at the top of the box', () => {
    const box = frame(
      [
        { lat: 40.7, lon: -84.4 },
        { lat: 41.0, lon: -83.8 },
      ],
      100,
    )
    expect(box.place({ lat: 41.0, lon: -84.1 }).y).toBeLessThan(box.place({ lat: 40.7, lon: -84.1 }).y)
  })

  it("keeps the county's shape rather than filling a fixed box", () => {
    const box = frame(
      [
        { lat: 40.7, lon: -84.4 },
        { lat: 41.0, lon: -83.8 },
      ],
      520,
    )
    expect(box.width).toBe(520)
    // Allen County is wider than it is tall; a figure that returned a square would be stretching it.
    expect(box.height).toBeLessThan(box.width)
    expect(box.height).toBeGreaterThan(0)
  })

  it('leaves room for a mark sitting on the boundary', () => {
    const extent = [
      { lat: 40.7, lon: -84.4 },
      { lat: 41.0, lon: -83.8 },
    ]
    const box = frame(extent, 520, 6)
    expect(box.place(extent[0]).x).toBeCloseTo(6, 5)
    expect(box.place(extent[1]).x).toBeCloseTo(514, 5)
  })
})

describe('distance', () => {
  it('is zero between a point and itself', () => {
    expect(metres({ lat: 40.8, lon: -84.2 }, { lat: 40.8, lon: -84.2 })).toBeCloseTo(0, 6)
  })

  it('measures a degree of latitude at about 111 km', () => {
    expect(metres({ lat: 40, lon: -84 }, { lat: 41, lon: -84 })).toBeGreaterThan(111_000)
    expect(metres({ lat: 40, lon: -84 }, { lat: 41, lon: -84 })).toBeLessThan(111_400)
  })

  it('sums a multi-leg track rather than measuring end to end', () => {
    const legs = straightMiles([
      { lat: 40.7, lon: -84.2 },
      { lat: 40.8, lon: -84.2 },
      { lat: 40.9, lon: -84.2 },
    ])
    const ends = straightMiles([
      { lat: 40.7, lon: -84.2 },
      { lat: 40.9, lon: -84.2 },
    ])
    expect(legs).toBeCloseTo(ends, 2)
  })
})

describe('the corpus', () => {
  it('has at least one event stating a track', () => {
    expect(tracked.length).toBeGreaterThan(0)
  })

  it('parses every track the corpus publishes', () => {
    // The gate that matters. A track the figure cannot read renders nothing and says nothing,
    // and the entry page would look merely sparse rather than broken.
    for (const node of tracked) {
      expect(parseTrack(node.properties.track), node.id).not.toEqual([])
    }
  })

  it("agrees with each event's own stated track length to within a fifth of a mile", () => {
    // Both nodes make this comparison in prose — 17.92 against a stated 17.8, and 9.53 against
    // a stated 9.7 — and read the near-agreement as identifying the storm. If either number
    // moves, one of the two is now describing something else.
    for (const node of tracked) {
      const said = statedMiles(node.properties.location_description ?? '')
      expect(said, `${node.id} states no track length`).not.toBeNull()
      const measured = straightMiles(parseTrack(node.properties.track))
      expect(Math.abs(measured - (said as number)), node.id).toBeLessThan(0.2)
    }
  })

  it('states no track that runs outside a plausible box for this county', () => {
    for (const node of tracked) {
      for (const p of parseTrack(node.properties.track)) {
        expect(p.lat, node.id).toBeGreaterThan(40.5)
        expect(p.lat, node.id).toBeLessThan(41.1)
        expect(p.lon, node.id).toBeLessThan(-83.7)
        expect(p.lon, node.id).toBeGreaterThan(-84.6)
      }
    }
  })
})

describe('bearing', () => {
  it('reads as English rather than as a signed pair', () => {
    expect(bearing({ lat: 40.87, lon: -83.87 })).toBe('40.87° N, 83.87° W')
  })
})

/** The straight-line length of one drawn piece, in miles. */
const pieceMiles = (pieces: { lat: number; lon: number }[][]): number => straightMiles(pieces[0])

describe('the connector is broken, and the break is the claim', () => {
  const storm = [
    { lat: 40.8, lon: -84.2 },
    { lat: 40.87, lon: -83.87 },
  ]

  it('never returns the whole track as one unbroken piece', () => {
    // The one thing the connector may not be. Nothing about the ground between two recorded
    // positions is on the record, and a solid line would assert it in the one encoding that
    // carries no tag and affords no tooltip.
    for (const node of tracked) {
      const pieces = dashes(parseTrack(node.properties.track))
      expect(pieces.length, node.id).toBeGreaterThan(1)
    }
  })

  it('begins at the first recorded position and ends at the last', () => {
    const pieces = dashes(storm)
    expect(pieces[0][0]).toEqual(storm[0])
    const last = pieces[pieces.length - 1]
    expect(last[last.length - 1]).toEqual(storm[1])
  })

  it('draws less than it leaves out, so the line reads as a reading', () => {
    const pieces = dashes(storm)
    const drawn = pieces.reduce((sum, piece) => sum + straightMiles(piece), 0)
    expect(drawn).toBeLessThan(straightMiles(storm))
    expect(drawn / straightMiles(storm)).toBeCloseTo(DASH_METRES / (DASH_METRES + GAP_METRES), 1)
  })

  it('dashes two tracks at one rhythm, so their lengths compare by eye', () => {
    // The reason the lengths are metres and not a fraction of the track: a nine-mile storm and
    // an eighteen-mile storm drawn at one rhythm can be told apart on the map.
    const short = dashes(storm)
    const long = dashes([storm[0], { lat: 40.94, lon: -83.54 }])
    expect(pieceMiles(long)).toBeCloseTo(pieceMiles(short), 3)
    expect(long.length).toBeGreaterThan(short.length)
  })

  it('keeps a recorded position that falls inside a drawn piece', () => {
    // No track in the corpus has a middle. The ontology admits one, and a dash spanning a bend
    // would cut the corner off a track that did.
    const bent = [
      { lat: 40.7, lon: -84.2 },
      { lat: 40.7, lon: -84.0 },
      { lat: 40.9, lon: -84.0 },
    ]
    const pieces = dashes(bent, 200_000, 1)
    expect(pieces[0]).toContainEqual(bent[1])
  })

  it('draws nothing rather than a line it cannot vouch for', () => {
    expect(dashes([])).toEqual([])
    expect(dashes([{ lat: 40.8, lon: -84.2 }])).toEqual([])
    expect(dashes([{ lat: 40.8, lon: -84.2 }, { lat: 40.8, lon: -84.2 }])).toEqual([])
    expect(dashes(storm, 0, 100)).toEqual([])
    expect(dashes(storm, 100, 0)).toEqual([])
  })

  it('scales the rhythm rather than drawing a short track solid', () => {
    // A track shorter than two cycles would come back as one unbroken piece. Nothing in the
    // corpus is near it — the shorter storm is 15km against a 750m cycle — and the guard is
    // here because solid is the failure, not a degraded case.
    const tiny = [
      { lat: 40.8, lon: -84.2 },
      { lat: 40.8005, lon: -84.2 },
    ]
    expect(metres(tiny[0], tiny[1])).toBeLessThan(DASH_METRES)
    expect(dashes(tiny).length).toBeGreaterThan(1)
  })
})
