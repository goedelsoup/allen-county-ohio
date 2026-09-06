// The corpus's edges, put on the ground.
//
// `crates/placement` answers *where is this node*. This answers a different question: **what does
// this edge draw**, given that both of its ends have been placed. They are separate questions and
// the second one has a lot more refusals in it.
//
// See `.yidam/decisions/a-river-is-not-at-its-mouth.yml`. The short version:
//
//   Line     a point-like thing pointing at the ground it names. Drawn only when the subject
//            states its own position — a line from a derived mark to a derived mark is a
//            relationship between two guesses.
//   Nesting  one extent inside another. Drawn by shape, never by a line between two centroids.
//   Course   a line the corpus does not hold and must not invent.
//   Nothing  not spatial, or already drawn by the mark itself.

import { atlas, edges as published, type AtlasRecord, type Edge, type Tier } from './feeds'

export type Drawn = 'line' | 'nesting' | 'course' | 'nothing'

export interface Drawing {
  relationship: string
  drawn: Drawn
  /** Why. The refusals are the half worth reading. */
  because: string
}

/**
 * Every relationship the published graph carries, and what it draws.
 *
 * `edges.test.ts` fails on a relationship the feed uses and this table omits, **and** on an entry
 * naming a relationship the feed no longer carries. The second half is what stops it becoming a
 * list of permissions nobody re-reads — the same argument `ROUTES`, the lint baseline and the
 * publication gate's `answers` list are all built on.
 *
 * Structural edges are not here because they are not published: `instance-of`, `concerns` and
 * `subject-of` are dropped from `graph.json` by rule. None of them is spatial.
 */
export const DRAWINGS: readonly Drawing[] = [
  // ---- a line ------------------------------------------------------------
  {
    relationship: 'located-in',
    drawn: 'line',
    because:
      'A site or a work naming the ground it stands on. This is the corpus-correctness case the map was built for: the Lima refinery carried a Lima address for eleven phases and stands in Shawnee Township, and a line from the dot to the place it names crosses a boundary it should not.',
  },
  {
    relationship: 'situated-in',
    drawn: 'line',
    because:
      'The same claim as `located-in`, made by a node that chose the other word. Six of them, and nothing distinguishes the two in the ontology — which is itself worth knowing.',
  },
  {
    relationship: 'occurred-at',
    drawn: 'line',
    because:
      'An event naming the site it happened at, rather than the place. The site has a coordinate of its own, so the line joins two stated positions.',
  },
  {
    relationship: 'formerly-in',
    drawn: 'line',
    because:
      'The one edge in the corpus that exists to carry a date `located-in` cannot. Fort Amanda was in Allen County from the erection until 1848 and is now a fifth of a mile outside it, so this line crosses the county boundary — which is the whole reason the edge was coined.',
  },

  // ---- nesting -----------------------------------------------------------
  {
    relationship: 'within',
    drawn: 'nesting',
    because:
      'Containment. A line from Lima’s centroid to the county’s centroid says nothing the two shapes do not already say, and says it as though it were a journey.',
  },
  {
    relationship: 'territory-within',
    drawn: 'nesting',
    because:
      'Containment between jurisdictions, which is the edge that fills the county in: twelve townships nested in one county, each from the year it was erected.',
  },
  {
    relationship: 'partially-within',
    drawn: 'nesting',
    because:
      'Partial containment, which is a fact about two shapes overlapping. Drawn by lighting both, never by a line — a line would pick one point in the overlap and imply the corpus had chosen it.',
  },
  {
    relationship: 'covers',
    drawn: 'nesting',
    because:
      'Containment stated from the containing side. Same shapes, same drawing — the direction is the corpus’s business and does not change what a reader sees.',
  },
  {
    relationship: 'partially-covers',
    drawn: 'nesting',
    because:
      'Partial containment from the containing side: a tract that reaches into two townships, a district that takes part of a city. Both shapes light and neither is a line.',
  },
  {
    relationship: 'nested-in',
    drawn: 'nesting',
    because:
      'Containment between divisions — a tract inside the county, a precinct inside a township. The word differs from `within` because the class does; the drawing does not.',
  },

  // ---- a course this corpus does not hold --------------------------------
  {
    relationship: 'flows-into',
    drawn: 'course',
    because:
      'Both ends are watercourses and both resolve to the county centroid, so an arc would be eight lines from the middle of the county to the middle of the county — a picture of the resolver’s ignorance drawn as though it were hydrography. The drainage is real and its geometry is TIGER’s.',
  },
  {
    relationship: 'traverses',
    drawn: 'course',
    because:
      'A watercourse crossing a place. Its extent is a line and the corpus holds two of its points; drawing the chord between them would look like a course, and no river runs straight.',
  },

  // ---- nothing -----------------------------------------------------------
  //
  // The four below are the entries worth arguing with. They are the placement routes — how a
  // person, an event or an organization reaches ground at all — and the mark is already drawn
  // where they lead. A line as well would run from a mark to the mark it stands on.
  {
    relationship: 'occurred-in',
    drawn: 'nothing',
    because:
      'A placement route: this is how an event reaches ground at all, and the mark is already standing on the place it names. A line as well would run from a mark to itself.',
  },
  {
    relationship: 'resided-in',
    drawn: 'nothing',
    because:
      'A placement route, and the coarsest one: 56 of the corpus’s 99 people are recorded as having resided in Allen County and nowhere finer.',
  },
  {
    relationship: 'seated-in',
    drawn: 'nothing',
    because:
      'A placement route. An organization is already drawn where it sits, and 33 lines from Lima to Lima would say nothing the stack does not.',
  },
  {
    relationship: 'established-within',
    drawn: 'nothing',
    because:
      'A placement route whose interesting half is temporal: it says a thing began inside a place, and the axis is what carries the beginning.',
  },
  {
    relationship: 'concentrated-in',
    drawn: 'nothing',
    because:
      'A placement route for a figure whose subject is one part of the county. The count is drawn on that part, which is what the treatment rules already do with it.',
  },

  {
    relationship: 'describes',
    drawn: 'nothing',
    because:
      'A subject edge, and the corpus’s commonest: what a figure is about, not where it is. It is a placement route, so the figure is already stacked on its subject.',
  },
  {
    relationship: 'relates-to',
    drawn: 'nothing',
    because:
      'The associative edge — 378 of them, refused as a placement route for the same reason: it would drag every figure onto every subject it was ever compared with.',
  },
  {
    relationship: 'evidenced-by',
    drawn: 'nothing',
    because:
      'Provenance: what a claim rests on. A catalog source has no position, and if it had one it would be the archive holding the paper rather than the ground the claim is about.',
  },
  {
    relationship: 'affected',
    drawn: 'nothing',
    because:
      'What an event changed, not where it was. Drawing it would run a line from the county to the Treaty of St. Marys, which was signed in another county and made this one.',
  },
  {
    relationship: 'held-by',
    drawn: 'nothing',
    because:
      'A term of office and the person holding it. A term is not at its holder’s residence, which is why the placement rules refuse it too.',
  },
  {
    relationship: 'of-office',
    drawn: 'nothing',
    because:
      'A term of office and the office it is a term of. This is the route the term reaches ground by, in two more hops, so the mark is already there.',
  },
  {
    relationship: 'governed-by',
    drawn: 'nothing',
    because:
      'A jurisdiction and the office that governs it. Both are drawn — one as a shape, one as a mark on the seat — and the relation between them is not a distance.',
  },
  {
    relationship: 'serves',
    drawn: 'nothing',
    because:
      'An office and the body it serves. A line between two seats in the same courthouse is a line nobody can see and nothing anybody wanted to know.',
  },
  {
    relationship: 'operated-by',
    drawn: 'nothing',
    because:
      'A site and whoever runs it. The operator may be headquartered on another continent, and a line saying so would leave the frame and tell the reader nothing about the county.',
  },
  {
    relationship: 'erected-by',
    drawn: 'nothing',
    because:
      'A work and whoever put it up. The builder is a person or a body, not a place, and the work is already drawn where it stands.',
  },
  {
    relationship: 'leased-to',
    drawn: 'nothing',
    because:
      'A tenancy. Who holds the ground rather than where the ground is, and the ground is drawn either way.',
  },
  {
    relationship: 'affiliated-with',
    drawn: 'nothing',
    because:
      'An association between organizations — a congregation and its denomination, a hospital and its system. Membership is not geography.',
  },
  {
    relationship: 'involved',
    drawn: 'nothing',
    because:
      'An event and who took part in it. Most of the people have no position finer than the county, so the line would run from the event to the middle of the frame.',
  },
  {
    relationship: 'precedes',
    drawn: 'nothing',
    because:
      'Order in time. The axis under the map is what carries it, and drawing it on the ground would make a sequence look like a journey.',
  },
  {
    relationship: 'named-for',
    drawn: 'nothing',
    because:
      'Amanda Township is named for a fort that now stands in the next county. Drawing a line to a namesake would suggest the township goes there, which is exactly what the placement rules refuse.',
  },
  {
    relationship: 'comparable-to',
    drawn: 'nothing',
    because:
      'Two figures that may be set beside each other. Both are usually about the same ground, so the line would run from a place to itself — and where they are not, the difference is a definition or a column heading rather than a distance. The claim is temporal and evidentiary, and the entry carries it as a table.',
  },
  {
    relationship: 'not-comparable-to',
    drawn: 'nothing',
    because:
      'A break in series. This is the one refusal here that is tempting to draw, because a break is often *caused* by ground moving — a corporation line that grew, a city entering a township table. But the edge names two figures, not the annexation between them, and a line from the county to the county would put the corpus’s sharpest warning on the map as a mark that says nothing.',
  },
]

const BY_RELATIONSHIP = new Map(DRAWINGS.map((d) => [d.relationship, d]))

export function drawingFor(relationship: string): Drawing | undefined {
  return BY_RELATIONSHIP.get(relationship)
}

/** An edge with both ends resolved against the atlas. */
export interface Spatial {
  edge: Edge
  drawn: Drawn
  from: AtlasRecord
  to: AtlasRecord
  tier: Tier
}

const BY_NODE = new Map(atlas.map((r) => [r.node, r]))

/** Every published edge that draws something, with both ends resolved. */
export function spatial(): Spatial[] {
  const out: Spatial[] = []
  for (const edge of published) {
    const drawing = BY_RELATIONSHIP.get(edge.relationship)
    if (!drawing || drawing.drawn === 'nothing') continue
    const from = BY_NODE.get(edge.from)
    const to = BY_NODE.get(edge.to)
    if (!from || !to) continue
    out.push({ edge, drawn: drawing.drawn, from, to, tier: edge.tier })
  }
  return out
}

/** A position a node states for itself, rather than one it was routed to. */
export function stated(record: AtlasRecord): { lat: number; lon: number } | null {
  if (record.hops !== 0) return null
  const anchor = record.anchors.find((a) => a.lat !== null && a.lon !== null)
  return anchor ? { lat: anchor.lat as number, lon: anchor.lon as number } : null
}

/** A key a node states for itself, which is how its shape is found. */
export function statedKey(record: AtlasRecord): string | null {
  if (record.hops !== 0) return null
  return record.anchors.find((a) => a.geoid !== null)?.geoid ?? null
}

/** One line the map can draw: two stated positions and the claim between them. */
export interface Line {
  from: { lat: number; lon: number }
  to: { lat: number; lon: number }
  relationship: string
  tier: Tier
  fromLabel: string
  toLabel: string
}

/**
 * The `line` edges both of whose ends state a position.
 *
 * A line is drawn only from a stated position to a stated position. Running one between derived
 * marks would draw a relationship between two guesses, and the map has no way to say that a line
 * is three inferences long.
 */
export function lines(all = spatial()): Line[] {
  return all.flatMap((s) => {
    if (s.drawn !== 'line') return []
    const from = stated(s.from)
    const to = stated(s.to)
    if (!from || !to) return []
    return [
      {
        from,
        to,
        relationship: s.edge.relationship,
        tier: s.tier,
        fromLabel: s.from.label,
        toLabel: s.to.label,
      },
    ]
  })
}

/** A containment claim between two shapes the site can draw. */
export interface Nest {
  inner: string
  outer: string
  relationship: string
  tier: Tier
  innerLabel: string
  outerLabel: string
}

/** The `nesting` edges both of whose ends state a Census key. */
export function nests(all = spatial()): Nest[] {
  return all.flatMap((s) => {
    if (s.drawn !== 'nesting') return []
    const inner = statedKey(s.from)
    const outer = statedKey(s.to)
    if (!inner || !outer) return []
    return [
      {
        inner,
        outer,
        relationship: s.edge.relationship,
        tier: s.tier,
        innerLabel: s.from.label,
        outerLabel: s.to.label,
      },
    ]
  })
}

/** What the connection layer draws, and what it declines to. */
export interface Tally {
  /** Edges whose relationship draws a line, and how many have two stated ends. */
  line: { total: number; drawable: number }
  nesting: { total: number; drawable: number }
  /** Courses the corpus states and does not hold the geometry of. */
  course: { total: number }
  /** Relationships that draw nothing, and the edges under them. */
  silent: number
}

export function tally(): Tally {
  const all = spatial()
  const drawn = (kind: Drawn) => all.filter((s) => s.drawn === kind)
  return {
    line: { total: drawn('line').length, drawable: lines(all).length },
    nesting: { total: drawn('nesting').length, drawable: nests(all).length },
    course: { total: drawn('course').length },
    silent: published.length - all.length,
  }
}
