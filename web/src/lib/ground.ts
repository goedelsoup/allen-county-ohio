// Which Census shapes this site actually holds.
//
// **Build time only.** It reads `public/geo/` from disk, so it must never be imported by
// anything that ships to a browser — `src/scripts/` reads the same files over `fetch()` at run
// time, which is the right way round for a client and the wrong way round for a page that wants
// the answer while it is being rendered.
//
// It exists because of a gap the atlas feed exposes and cannot close. A corpus node may state a
// Census key for a boundary layer this repository has not vendored, and twelve of them do: the
// school districts carry Unified School District keys, and TIGERweb serves those from a layer
// `fetch-boundaries.mjs` does not ask for. The map cannot draw those shapes and must not
// silently drop them either, so it says how many there are — and this is what counts them,
// rather than a number typed into a page and left to rot.

import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import type { AtlasRecord } from './feeds'

const GEO = join(import.meta.dirname, '../../public/geo')

/** Every polygon file, in the order `map.ts` loads them. Voting districts and tracts are drawn
 * as ground rather than joined to corpus keys, so they are not here. */
const FILES = ['county', 'county-subdivisions', 'places', 'census-designated-places']

function keys(file: string): string[] {
  const text = readFileSync(join(GEO, `${file}.geojson`), 'utf8')
  const collection = JSON.parse(text) as {
    features: { properties: { GEOID: string } }[]
  }
  return collection.features.map((f) => f.properties.GEOID)
}

/** Keys this site can draw a shape for. */
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
 * Records that state a Census key for a shape this site does not hold.
 *
 * They are drawn as marks at whatever position they reach instead, which is honest: the corpus
 * knows where the district is, and this site does not hold its outline.
 */
export function unheldShapes(records: AtlasRecord[]): AtlasRecord[] {
  return records.filter(
    (r) =>
      r.treatment === 'polygon' &&
      r.hops === 0 &&
      r.anchors.some((a) => a.geoid !== null && !HELD_KEYS.has(a.geoid)),
  )
}
