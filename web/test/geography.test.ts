import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { mapPoints } from '../src/lib/feeds'

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
