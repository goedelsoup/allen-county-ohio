import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { atlas, censusKey, mapPoints } from '../src/lib/feeds'
import { FRAME_KEYS, HELD_KEYS, containedInCounty, unheldShapes } from '../src/lib/ground'
import { UNDRAWN, courses, tigerName } from '../src/lib/water'

// The map draws corpus nodes onto vendored Census geometry and joins the two by GEOID. That
// join is the one derivation this site performs, so it is the one thing this site has to
// gate: `crates/publish` cannot see `public/geo/`, and the connector cannot see the corpus.

const GEO = join(import.meta.dirname, '..', 'public', 'geo')

interface Feature {
  properties: { GEOID: string; NAME: string }
  geometry: { type: string; coordinates: unknown }
}

function layer(file: string): Feature[] {
  const raw = readFileSync(join(GEO, `${file}.geojson`), 'utf8')
  return (JSON.parse(raw) as { features: Feature[] }).features
}

const provenance = JSON.parse(readFileSync(join(GEO, 'PROVENANCE.json'), 'utf8')) as {
  vintage: string
  catalog: string
  layers: { file: string; features: number }[]
}

// Keyed by filename, so a provenance entry and a layer are the same string.
const layers: Record<string, Feature[]> = {
  'county.geojson': layer('county'),
  'county-subdivisions.geojson': layer('county-subdivisions'),
  'places.geojson': layer('places'),
  'census-designated-places.geojson': layer('census-designated-places'),
  'voting-districts.geojson': layer('voting-districts'),
  'census-tracts.geojson': layer('census-tracts'),
  'school-districts.geojson': layer('school-districts'),
  'linear-water.geojson': layer('linear-water'),
}

/**
 * The layers whose features are Census *entities*: a GEOID, a name, an area.
 *
 * The water is not one. It has no GEOID because a stream is not a geographic entity in the
 * Census's sense, and 224 of its 313 segments have no name at all — which is a fact about how
 * the federal file is built and not a defect in the fetch.
 */
const ENTITY = Object.keys(layers).filter((f) => f !== 'linear-water.geojson')

const known = new Set(ENTITY.flatMap((f) => layers[f]).map((f) => f.properties.GEOID))

/**
 * The summary level each shape file holds — the site's own table, restated.
 *
 * Not shared with `ground.ts` on purpose: a check that imports the table it is checking cannot
 * fail when the table is wrong. Places and CDPs share a level because the Census gives them one
 * code space; school districts have their own, which is what makes a bare GEOID ambiguous here.
 */
const LEVELS: Record<string, string> = {
  county: 'county',
  'county-subdivisions': 'county-subdivision',
  places: 'place',
  'census-designated-places': 'place',
  'school-districts': 'school-district',
}

describe('the vendored geography', () => {
  it('is the 2020 vintage the corpus cites', () => {
    expect(provenance.vintage).toBe('2020')
    expect(provenance.catalog).toContain('tigerweb-census2020')
  })

  it('holds the county, its thirteen civil subdivisions and all 88 precincts', () => {
    expect(layers['county.geojson']).toHaveLength(1)
    expect(layers['county-subdivisions.geojson']).toHaveLength(13)
    // The corpus states this count as a measure. If TIGER and the corpus ever disagree, the
    // map is drawing a different county than the pages describe.
    expect(layers['voting-districts.geojson']).toHaveLength(88)
  })

  it('holds the 35 tracts the lending record is reported at', () => {
    // The corpus reports mortgage denial and pricing over 34 tracts; TIGER holds 35. The
    // extra one takes no first-lien loans because it holds no one-to-four-family houses, so
    // it appears in no such table. Pinned at TIGER's number, which is the ground: a table
    // that covers 34 of 35 is a fact about the table.
    expect(layers['census-tracts.geojson']).toHaveLength(35)
  })

  it('records what it holds in its provenance file', () => {
    expect(Object.keys(layers).toSorted()).toEqual(
      provenance.layers.map((l) => l.file).toSorted(),
    )
    for (const entry of provenance.layers) {
      expect(layers[entry.file], entry.file).toHaveLength(entry.features)
    }
  })

  it('gives every entity a geometry and a name', () => {
    for (const f of ENTITY.flatMap((file) => layers[file])) {
      expect(f.geometry?.coordinates, f.properties.GEOID).toBeTruthy()
      expect(f.properties.NAME).toBeTruthy()
    }
  })
})

describe('the corpus-to-geometry join', () => {
  it('resolves every GEOID the corpus carries', () => {
    const unresolved = mapPoints
      .filter((p) => p.geoid)
      .filter((p) => !known.has(p.geoid as string))
      .map((p) => `${p.label} (${p.geoid})`)

    expect(unresolved, 'corpus GEOIDs with no vendored geometry').toEqual([])
  })

  it('puts every located node inside the county bounding box', () => {
    // A transposed coordinate pair lands in Asia and renders as an empty map with no error.
    const outside = mapPoints
      .filter((p) => p.lat < 40.5 || p.lat > 41.0 || p.lon < -84.6 || p.lon > -83.7)
      .map((p) => `${p.label} at ${p.lat}, ${p.lon}`)

    expect(outside, 'located nodes outside Allen County').toEqual([])
  })

  it('holds geometry the corpus does not name, and does not pretend otherwise', () => {
    // Cridersville village crosses in from Auglaize County. The corpus has no node for it.
    // This is a fact about the county, not a defect — and the map shows it as ground.
    const named = new Set(mapPoints.map((p) => p.geoid).filter(Boolean))
    const unnamed = layers['places.geojson'].filter((f) => !named.has(f.properties.GEOID))
    expect(unnamed.map((f) => f.properties.NAME)).toEqual(['Cridersville village'])
  })
})


describe('the shapes the travelling map can draw', () => {
  /**
   * The map shades a corpus node's own Census key, and says on the page how many keys it cannot
   * shade. Both halves have to be checked or the page keeps a number that used to be true.
   *
   * The unheld keys were the twelve school districts until layer 12 was vendored, and the
   * count is zero now. That is the reason the checks below are written as *nothing is unheld*
   * and *nothing is silently dropped* rather than as a list: a rule that names its exceptions
   * has to be rewritten every time the exceptions change, and gets switched off instead.
   */
  it('holds every key it says it holds', () => {
    expect(HELD_KEYS.size).toBe(
      layers['county.geojson'].length +
        layers['county-subdivisions.geojson'].length +
        layers['places.geojson'].length +
        layers['census-designated-places.geojson'].length +
        layers['school-districts.geojson'].length,
    )
  })

  it('names the county as the frame rather than as a place on it', () => {
    // Filling the frame tints every pixel inside it. The map strokes this key and never fills it.
    expect([...FRAME_KEYS]).toEqual(
      layers['county.geojson'].map((f) => `county:${f.properties.GEOID}`),
    )
  })

  it('can draw every shape the corpus keys', () => {
    // Was twelve school districts; is nothing. The failure message carries the labels rather
    // than a count, because the useful question on the day this breaks is *which*.
    const unheld = unheldShapes(atlas).map((r) => `${r.label} (${r.node})`)
    expect(unheld, 'corpus polygons with no vendored shape').toEqual([])

    const drawable = atlas.filter((r) => r.treatment === 'polygon' && r.hops === 0)
    for (const r of drawable) {
      for (const a of r.anchors) {
        if (a.geoid === null) continue
        const key = censusKey(a.level, a.geoid)
        expect(key, `${r.node} keys ${a.geoid} at no summary level`).not.toBeNull()
        expect(HELD_KEYS.has(key as string), `${r.node} keys ${key}`).toBe(true)
      }
    }
  })

  it('resolves every key the atlas states, or files it as unheld', () => {
    // The anti-drift half: a key that is neither drawable nor counted as unheld is a shape the
    // page silently drops, which is the failure the count on the page exists to prevent.
    const unheld = new Set(unheldShapes(atlas).map((r) => r.node))
    for (const r of atlas) {
      if (r.treatment !== 'polygon' || r.hops !== 0) continue
      const keys = r.anchors
        .map((a) => censusKey(a.level, a.geoid))
        .filter((k): k is string => k !== null)
      if (keys.length === 0) continue
      expect(keys.every((k) => HELD_KEYS.has(k)) || unheld.has(r.node), r.node).toBe(true)
    }
  })

  it('gives every key it draws a summary level, because a bare GEOID is not an address', () => {
    // The defect vendoring the districts exposed: `3904752` is Beaverdam village and the Upper
    // Scioto Valley Local School District, and an index over bare GEOIDs resolves the village to
    // whichever layer loaded last. Both halves are checked — every key carries a level, and no
    // two features share a levelled key.
    const unlevelled = atlas
      .filter((r) => r.treatment === 'polygon' && r.hops === 0)
      .flatMap((r) => r.anchors.filter((a) => a.geoid !== null && a.level === null).map(() => r.node))
    expect(unlevelled, 'atlas anchors with a key and no level').toEqual([])

    expect(HELD_KEYS.size).toBe(
      Object.keys(LEVELS).reduce((n, file) => n + layers[`${file}.geojson`].length, 0),
    )
  })
})


describe('the school districts, which are mostly not this county\'s', () => {
  const districts = layers['school-districts.geojson']
  const within = new Set(containedInCounty('school-districts'))
  const catalogued = new Set(
    atlas
      .filter((r) => r.node.includes('school-district'))
      .flatMap((r) => r.anchors.map((a) => a.geoid))
      .filter((g): g is string => g !== null),
  )

  it('holds every district with territory in the county', () => {
    expect(districts).toHaveLength(17)
  })

  it('draws them whole rather than cutting them at the county line', () => {
    // The decision this layer carries, stated as the thing a clip would break. Clipped, every
    // district would be contained by the county and this would read 17 — so the check is not
    // that the number is five but that it is *not all of them*.
    expect(within.size).toBeLessThan(districts.length)
    expect(within.size).toBe(5)
  })

  it('names the five that do not leave the county', () => {
    // Pinned, because the count above is only meaningful if it is the right five. Allen East
    // and Elida are here on a tolerance: their northern edges are the county line, stated to
    // five decimal places in two files that rounded it independently.
    const names = districts
      .filter((f) => within.has(f.properties.GEOID))
      .map((f) => f.properties.NAME.replace(' School District', ''))
      .toSorted()
    expect(names).toEqual(['Allen East Local', 'Bath Local', 'Elida Local', 'Lima City', 'Perry Local'])
  })

  it('holds a shape for every district the corpus catalogues', () => {
    const keys = new Set(districts.map((f) => f.properties.GEOID))
    expect(catalogued.size).toBe(12)
    expect([...catalogued].filter((k) => !keys.has(k)), 'catalogued with no shape').toEqual([])
  })

  it('holds five districts the corpus does not name, and does not pretend otherwise', () => {
    // The other half of the Cridersville rule one describe up: geometry the corpus has no node
    // for is a fact about the county, not a defect — and pinning the list is what makes the day
    // one of them gets catalogued a day somebody notices.
    const unnamed = districts
      .filter((f) => !catalogued.has(f.properties.GEOID))
      .map((f) => f.properties.NAME.replace(' School District', ''))
      .toSorted()
    expect(unnamed).toEqual([
      'Ada Exempted Village',
      'Cory-Rawson Local',
      'Jennings Local',
      'Upper Scioto Valley Local',
      'Wapakoneta City',
    ])
  })
})

describe('the water', () => {
  const water = layers['linear-water.geojson'] as unknown as {
    properties: { OID: string; NAME?: string; MTFCC: string }
    geometry: { coordinates: unknown }
  }[]

  const entry = provenance.layers.find((l) => l.file === 'linear-water.geojson') as unknown as {
    vintage?: string
    generalized_degrees?: number
  }

  it('is dated by its own service rather than by the file', () => {
    // TIGERweb keeps hydrography out of the decennial services: there is one Hydro MapServer
    // with no vintage of its own. Letting it inherit the file's 2020 would date the water to a
    // year nobody published it in.
    expect(provenance.vintage).toBe('2020')
    expect(entry.vintage).toBe('current')
  })

  it('says it was generalized, and by how much', () => {
    // 313 polylines came back at 847 KB before this. The service does the generalizing so the
    // vendored file stays a mirror; what the file owes the reader is the number.
    expect(entry.generalized_degrees).toBeGreaterThan(0)
  })

  it('draws every segment, named or not', () => {
    for (const f of water) {
      expect(f.geometry?.coordinates, f.properties.OID).toBeTruthy()
      expect(f.properties.OID).toBeTruthy()
    }
  })

  it('keeps the dug lines apart from the streams', () => {
    // H3010 is a stream or river; H3020 is a canal, ditch or aqueduct. In this county that is
    // not a taxonomy detail — it is the Miami & Erie Canal and the ditches that turned the
    // Great Black Swamp into cropland, and the map draws them apart.
    const classes = new Set(water.map((f) => f.properties.MTFCC))
    expect([...classes].toSorted()).toEqual(['H3010', 'H3020'])
    expect(water.filter((f) => f.properties.MTFCC === 'H3020').length).toBeGreaterThan(0)
  })

  it('holds the canal under both of TIGER\u2019s spellings', () => {
    const canal = water.filter((f) => /Cnl/.test(f.properties.NAME ?? ''))
    expect(canal.length).toBeGreaterThan(0)
    // `Miami & Erie Cnl` and `Miami-Erie Cnl` are the same waterway. Nothing here resolves
    // them, and the map draws both — a mirror that tidied its source would stop being one.
    expect(new Set(canal.map((f) => f.properties.NAME)).size).toBeGreaterThan(1)
  })
})

describe('the corpus\u2019s watercourses against the file that draws them', () => {
  const drawn = new Set(
    (layers['linear-water.geojson'] as unknown as { properties: { NAME?: string } }[])
      .map((f) => f.properties.NAME)
      .filter((n): n is string => Boolean(n)),
  )

  it('abbreviates a name the way TIGER does', () => {
    expect(tigerName('Little Ottawa River')).toBe('Little Ottawa Riv')
    expect(tigerName('Riley Creek')).toBe('Riley Crk')
    // Only the generic half moves. `Little` is part of the name.
    expect(tigerName('Great Black Swamp')).toBe('Great Black Swamp')
  })

  it('finds a course for every watercourse it does not declare undrawn', () => {
    const missing = courses()
      .filter((c) => !drawn.has(c.tigerName) && !(c.tigerName in UNDRAWN))
      .map((c) => `${c.label} (${c.tigerName})`)
    expect(missing, 'watercourses with no vendored course and no declared reason').toEqual([])
  })

  it('declares nothing undrawn that the file now draws', () => {
    // The anti-drift half. If TIGER starts naming the Ottawa inside the county, the entry here
    // becomes a claim about the world that stopped being true, and the map would keep refusing
    // to draw a course it now has.
    const resolved = Object.keys(UNDRAWN).filter((name) => drawn.has(name))
    expect(resolved, 'declared undrawn and present in the file').toEqual([])
  })

  it('cannot draw the river the city was built on', () => {
    // Stated as its own check because it is the finding rather than a housekeeping detail.
    expect(Object.keys(UNDRAWN)).toContain('Ottawa Riv')
    expect(drawn.has('Ottawa Riv')).toBe(false)
    // And the corpus does know where it runs — both ends, at real coordinates.
    const ottawa = courses().find((c) => c.label === 'Ottawa River')
    expect(ottawa?.source).not.toBeNull()
    expect(ottawa?.mouth).not.toBeNull()
  })

  it('states both ends of a watercourse, or neither', () => {
    for (const c of courses()) {
      expect(Boolean(c.source) === Boolean(c.mouth), c.label).toBe(true)
    }
  })

  it('puts most of those ends outside the county, which is why they are not placements', () => {
    // The argument for `a river is not at its mouth`, checked. Reading `mouth` as a position
    // would put the Ottawa in Putnam County and the Auglaize in Defiance.
    const outside = courses()
      .map((c) => c.mouth)
      .filter((m) => m !== null)
      .filter((m) => m.lat < 40.64 || m.lat > 40.93 || m.lon < -84.4 || m.lon > -83.87)
    expect(outside.length).toBeGreaterThan(0)
  })
})
