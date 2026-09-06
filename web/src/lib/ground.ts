// Which Census shapes this site actually holds.
//
// **Build time only.** It reads `public/geo/` from disk, so it must never be imported by
// anything that ships to a browser — `src/scripts/` reads the same files over `fetch()` at run
// time, which is the right way round for a client and the wrong way round for a page that wants
// the answer while it is being rendered.
//
// It exists because of a gap the atlas feed exposes and cannot close. A corpus node may state a
// Census key for a boundary layer this repository has not vendored, and for five days twelve of
// them did: the school districts carry Unified School District keys, and `fetch-boundaries.mjs`
// did not ask for that layer. It does now, so the count is zero — which is exactly why this
// function stays. A page that had printed *twelve* as a literal would still be printing it.

import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { censusKey, type AtlasRecord } from './feeds'

const GEO = join(import.meta.dirname, '../../public/geo')

/** Every polygon file, in the order `map.ts` loads them. Voting districts and tracts are drawn
 * as ground rather than joined to corpus keys, so they are not here. */
const FILES = [
  'county',
  'county-subdivisions',
  'places',
  'census-designated-places',
  'school-districts',
]

type Position = [number, number]

interface Shapes {
  features: {
    properties: { GEOID: string }
    /** Single-ring polygons throughout: TIGER returns one for every layer this file reads. */
    geometry: { coordinates: Position[][] }
  }[]
}

/**
 * The summary level each vendored file holds, which is half of every key.
 *
 * CDPs and incorporated places share one level, because the Census gives them one code space —
 * a CDP's GEOID is a place GEOID. School districts have their own, and that is the whole reason
 * this table exists: `3904752` is a place *and* a school district in this county.
 */
const LEVELS: Record<string, string> = {
  county: 'county',
  'county-subdivisions': 'county-subdivision',
  places: 'place',
  'census-designated-places': 'place',
  'school-districts': 'school-district',
}

function shape(file: string): Shapes {
  return JSON.parse(readFileSync(join(GEO, `${file}.geojson`), 'utf8')) as Shapes
}

function keys(file: string): string[] {
  const level = LEVELS[file]
  return shape(file).features.map((f) => `${level}:${f.properties.GEOID}`)
}

/** How many features a vendored layer holds. */
export function layerSize(file: string): number {
  return shape(file).features.length
}

/** Keys this site can draw a shape for, each as `level:geoid`. */
export const HELD_KEYS = new Set(FILES.flatMap(keys))

/**
 * The county's own key, which is a shape and is not a *place* on the map.
 *
 * Its extent is the whole frame. Filling it tints every pixel of the county and drowns the
 * twelve townships and ten municipalities drawn inside it — the same argument `map.ts` already
 * makes about labelling the county at its centroid, one encoding along.
 */
export const FRAME_KEYS = new Set(keys('county'))

/**
 * The county's outline, as the positions TIGER draws it with.
 *
 * For a figure that needs the frame and not the map: an event's track means nothing without the
 * shape it crossed, and loading deck.gl and nine GeoJSON files onto an entry page to draw one
 * outline would be a map's cost for a diagram's job.
 *
 * **It is a 2020 outline like every other boundary here**, and a figure drawing 1965 on it owes
 * the reader the same caveat `groundFidelity()` makes on the map. The county's own limits have
 * not moved since 1848, which is why this particular anachronism is a small one — but it is the
 * figure's job to say so, not this function's to pretend otherwise.
 */
export function countyRing(): { lat: number; lon: number }[] {
  const feature = shape('county').features[0]
  return feature.geometry.coordinates[0].map(([lon, lat]) => ({ lat, lon }))
}

/**
 * Records that state a Census key for a shape this site does not hold.
 *
 * Empty today, and kept because the day it stops being empty is the day a page needs to say so.
 * A record here draws as a mark at whatever position it reaches instead, which is honest: the
 * corpus knows where the thing is and this site does not hold its outline.
 */
export function unheldShapes(records: AtlasRecord[]): AtlasRecord[] {
  return records.filter(
    (r) =>
      r.treatment === 'polygon' &&
      r.hops === 0 &&
      r.anchors.some((a) => a.geoid !== null && !HELD_KEYS.has(censusKey(a.level, a.geoid) ?? '')),
  )
}

/**
 * Districts that do not leave the county.
 *
 * The school-district layer is the one piece of ground here that ignores the county entirely —
 * Ohio draws a district to hold a population, and seventeen of them reach into Allen. A page
 * saying so needs the number, and the number is small enough to be typed and left to rot. So it
 * is computed from the same two files the map draws.
 *
 * The test is *every vertex inside*, and it needs a tolerance, which is the whole subtlety here.
 * A district boundary that follows the county line is stored twice — once in each file — and
 * `PRECISION` rounds both to five decimal places independently, so a shared vertex lands up to
 * about a metre on either side. Without the tolerance five districts read as three, and the two
 * it loses are Allen East and Elida, whose northern edges *are* the county line.
 *
 * `EDGE` is 2e-5 degrees, a shade over two metres: larger than the rounding can move a point and
 * far smaller than any real excursion, the narrowest of which is Shawnee's at some 400 metres.
 */
export function containedInCounty(file: string): string[] {
  const ring = shape('county').features[0].geometry.coordinates[0]
  return shape(file)
    .features.filter((f) =>
      f.geometry.coordinates.every((r) => r.every((p) => inside(p, ring) || onEdge(p, ring))),
    )
    .map((f) => f.properties.GEOID)
}

/** Rounding slack on a boundary two files state independently. Degrees. */
const EDGE = 2e-5

/** Ray casting, half-open on the upper vertex so a point level with one does not count twice. */
function inside([x, y]: Position, ring: Position[]): boolean {
  let hit = false
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [xi, yi] = ring[i]
    const [xj, yj] = ring[j]
    if (yi > y !== yj > y && x < ((xj - xi) * (y - yi)) / (yj - yi) + xi) hit = !hit
  }
  return hit
}

/** Within `EDGE` of any segment of the ring — a point on the county line, however it rounded. */
function onEdge([x, y]: Position, ring: Position[]): boolean {
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [xi, yi] = ring[i]
    const [xj, yj] = ring[j]
    const dx = xj - xi
    const dy = yj - yi
    const span = dx * dx + dy * dy
    const t = span === 0 ? 0 : Math.max(0, Math.min(1, ((x - xi) * dx + (y - yi) * dy) / span))
    if (Math.hypot(x - (xi + t * dx), y - (yi + t * dy)) < EDGE) return true
  }
  return false
}
