import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { atlas, mapPoints } from '../src/lib/feeds'
import { FRAME_KEYS, HELD_KEYS, unheldShapes } from '../src/lib/ground'

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
}

const known = new Set(
  Object.values(layers)
    .flat()
    .map((f) => f.properties.GEOID),
)

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

  it('gives every feature a geometry and a name', () => {
    for (const f of Object.values(layers).flat()) {
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
   * `/map` shades a corpus node's own Census key, and says on the page how many keys it cannot
   * shade. Both halves have to be checked or the page keeps a number that used to be true.
   *
   * The unheld keys are the twelve school districts. TIGERweb serves Unified School Districts
   * from a layer `fetch-boundaries.mjs` does not ask for, so the corpus knows where those
   * districts are and this site does not hold their outlines. That is a gap worth stating rather
   * than a defect worth hiding, and vendoring the layer is what closes it.
   */
  it('holds every key it says it holds', () => {
    expect(HELD_KEYS.size).toBe(
      layers['county.geojson'].length +
        layers['county-subdivisions.geojson'].length +
        layers['places.geojson'].length +
        layers['census-designated-places.geojson'].length,
    )
  })

  it('names the county as the frame rather than as a place on it', () => {
    // Filling the frame tints every pixel inside it. The map strokes this key and never fills it.
    expect([...FRAME_KEYS]).toEqual(layers['county.geojson'].map((f) => f.properties.GEOID))
  })

  it('cannot draw exactly the districts, and can draw everything else', () => {
    const unheld = unheldShapes(atlas)
    expect(unheld.length).toBeGreaterThan(0)
    for (const r of unheld) expect(r.node).toMatch(/school-district/)

    const drawable = atlas.filter(
      (r) => r.treatment === 'polygon' && r.hops === 0 && !unheld.includes(r),
    )
    for (const r of drawable) {
      for (const a of r.anchors) {
        if (a.geoid === null) continue
        expect(HELD_KEYS.has(a.geoid), `${r.node} keys ${a.geoid}`).toBe(true)
      }
    }
  })

  it('resolves every key the atlas states, or files it as unheld', () => {
    // The anti-drift half: a key that is neither drawable nor counted as unheld is a shape the
    // page silently drops, which is the failure the count on the page exists to prevent.
    const unheld = new Set(unheldShapes(atlas).map((r) => r.node))
    for (const r of atlas) {
      if (r.treatment !== 'polygon' || r.hops !== 0) continue
      const keys = r.anchors.map((a) => a.geoid).filter((g): g is string => g !== null)
      if (keys.length === 0) continue
      expect(keys.every((k) => HELD_KEYS.has(k)) || unheld.has(r.node), r.node).toBe(true)
    }
  })
})
