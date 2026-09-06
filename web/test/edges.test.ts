import { describe, expect, it } from 'vitest'
import { DRAWINGS, drawingFor, lines, nests, spatial, stated, statedKey, tally } from '../src/lib/edges'
import { atlas, edges } from '../src/lib/feeds'

// What the corpus's edges draw when the graph is put on the ground.
//
// Same posture as the placement gates: what is pinned is **the table's agreement with the feed**
// and the refusals that would be invisible if they broke. Counts are not pinned — a test asserting
// 32 drawable lines fails on the next site the corpus catalogues, and a gate that fails for the
// wrong reason gets switched off. The one exception is a shape check on the distribution, which is
// the finding the design rests on.

describe('the drawing table', () => {
  it('classifies every relationship the feed carries', () => {
    const missing = [...new Set(edges.map((e) => e.relationship))]
      .filter((r) => !drawingFor(r))
      .toSorted()
    expect(
      missing,
      'relationships in graph.json and not in DRAWINGS — decide what each draws, with the argument beside it',
    ).toEqual([])
  })

  it('names no relationship the feed has stopped carrying', () => {
    // The anti-drift half. An entry for a relationship nobody uses is a rule nobody can check,
    // and a table permitted to over-list re-permits whatever a later edit puts there.
    const used = new Set(edges.map((e) => e.relationship))
    const stale = DRAWINGS.filter((d) => !used.has(d.relationship)).map((d) => d.relationship)
    expect(stale, 'DRAWINGS names these and graph.json no longer carries them').toEqual([])
  })

  it('gives every entry an argument', () => {
    for (const d of DRAWINGS) {
      expect(d.because.length, d.relationship).toBeGreaterThan(30)
    }
  })

  it('classifies each relationship exactly once', () => {
    const names = DRAWINGS.map((d) => d.relationship)
    expect(names.length).toBe(new Set(names).size)
  })
})

describe('what a line is allowed to be', () => {
  it('runs only between two stated positions', () => {
    // The rule the whole layer rests on. A line between derived marks draws a relationship
    // between two guesses, and nothing on the map can say a line is three inferences long.
    const byLabel = new Map(atlas.map((r) => [r.label, r]))
    for (const line of lines()) {
      for (const label of [line.fromLabel, line.toLabel]) {
        const record = byLabel.get(label)
        expect(record?.hops, label).toBe(0)
      }
    }
  })

  it('draws the boundary Fort Amanda crossed', () => {
    // The edge `formerly-in` was coined for, and the reason it is a line rather than a refusal:
    // the fort was in this county until 1848 and is now outside it, so this line crosses the
    // county boundary. If it ever stops being drawable, the map has lost the one edge that
    // shows a boundary moving.
    const crossing = lines().filter((l) => l.relationship === 'formerly-in')
    expect(crossing).toHaveLength(1)
    expect(crossing[0].fromLabel).toMatch(/Amanda/)
  })

  it('never draws a placement route as a line as well', () => {
    // A line from a mark to the mark it already stands on. These four are how a person, an event
    // or an organization reaches ground at all, and the stack is already drawn there.
    for (const relationship of ['occurred-in', 'resided-in', 'seated-in', 'established-within']) {
      expect(drawingFor(relationship)?.drawn, relationship).toBe('nothing')
    }
  })

  it('refuses a river as a line, in both directions', () => {
    // Eight lines from the middle of the county to the middle of the county is not a drainage
    // map. See `.yidam/decisions/a-river-is-not-at-its-mouth.yml`.
    expect(drawingFor('flows-into')?.drawn).toBe('course')
    expect(drawingFor('traverses')?.drawn).toBe('course')
    expect(lines().some((l) => l.relationship === 'flows-into')).toBe(false)
  })
})

describe('containment is drawn by shape', () => {
  it('never becomes a line', () => {
    for (const relationship of ['within', 'territory-within', 'covers', 'nested-in']) {
      expect(drawingFor(relationship)?.drawn, relationship).toBe('nesting')
    }
    const nesting = new Set(nests().map((n) => n.relationship))
    expect(lines().filter((l) => nesting.has(l.relationship))).toEqual([])
  })

  it('resolves both ends to a stated key or draws neither', () => {
    for (const n of nests()) {
      expect(n.inner, n.innerLabel).toMatch(/^\d+$/)
      expect(n.outer, n.outerLabel).toMatch(/^\d+$/)
    }
  })
})

describe('a stated position is not a derived one', () => {
  it('returns nothing for a node the resolver had to route', () => {
    const derived = atlas.find((r) => r.hops !== null && r.hops > 0 && r.anchors.length > 0)
    expect(derived).toBeDefined()
    expect(stated(derived as never)).toBeNull()
    expect(statedKey(derived as never)).toBeNull()
  })

  it('returns a position for a node that carries one', () => {
    const own = atlas.find((r) => r.hops === 0 && r.anchors.some((a) => a.lat !== null))
    expect(stated(own as never)).not.toBeNull()
  })
})

describe('the tally the legend prints', () => {
  const t = tally()

  it('counts every published edge exactly once', () => {
    expect(t.line.total + t.nesting.total + t.course.total + t.silent).toBe(edges.length)
  })

  it('never claims to draw more than it has', () => {
    expect(t.line.drawable).toBeLessThanOrEqual(t.line.total)
    expect(t.nesting.drawable).toBeLessThanOrEqual(t.nesting.total)
  })

  it('refuses far more than it draws, which is the finding', () => {
    // The shape the connection layer's legend exists to state. Most of this graph is not
    // spatial, and most of what is spatial reaches ground by inference at one end or both.
    const drawn = t.line.drawable + t.nesting.drawable
    expect(t.silent).toBeGreaterThan(drawn)
  })

  it('has something to draw in each of the two kinds', () => {
    expect(t.line.drawable).toBeGreaterThan(0)
    expect(t.nesting.drawable).toBeGreaterThan(0)
    expect(t.course.total).toBeGreaterThan(0)
  })

  it('agrees with the spatial set it is computed from', () => {
    const all = spatial()
    expect(t.line.total + t.nesting.total + t.course.total).toBe(all.length)
  })
})
