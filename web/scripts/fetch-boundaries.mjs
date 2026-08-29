// Vendor Allen County's 2020 census geography from TIGERweb into `public/geo/`.
//
// This is a connector, and the only thing here that touches the network. It is run rarely
// and by hand; its output is committed, so a build — and the site — needs no network at all.
//
// It deliberately knows nothing about the corpus. The corpus's own geography is joined to
// these files by GEOID in `crates/publish`, which is where every derivation from the corpus
// lives. A connector that read node files would put half the derivation on this side of the
// line and make the publication gate blind to it.
//
//   node scripts/fetch-boundaries.mjs
//
// Source: https://tigerweb.geo.census.gov — catalogued at .yidam/catalog/tigerweb-census2020.md

import { writeFile, mkdir } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..')
const OUT = join(ROOT, 'public', 'geo')

const SERVICE =
  'https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb/tigerWMS_Census2020/MapServer'

const STATE = '39'
const COUNTY = '003'

// Five decimal places is about 1.1 m at this latitude. TIGER publishes far more, and every
// digit past the fifth is a byte committed to this repository to describe a distance no
// county-scale map can draw.
const PRECISION = 5

/** Layers to vendor, in draw order: the ground first, the finest grain last. */
const LAYERS = [
  {
    id: 82,
    file: 'county.geojson',
    label: 'County',
    where: `STATE='${STATE}' AND COUNTY='${COUNTY}'`,
    fields: ['GEOID', 'NAME', 'BASENAME', 'AREALAND', 'AREAWATER', 'CENTLAT', 'CENTLON'],
    expect: 1,
  },
  {
    id: 20,
    file: 'county-subdivisions.geojson',
    label: 'County Subdivisions',
    where: `STATE='${STATE}' AND COUNTY='${COUNTY}'`,
    fields: ['GEOID', 'NAME', 'BASENAME', 'LSADC', 'AREALAND', 'AREAWATER', 'CENTLAT', 'CENTLON'],
    // Twelve townships and the city of Lima, which in Ohio is its own county subdivision.
    expect: 13,
  },
  {
    id: 58,
    file: 'voting-districts.geojson',
    label: 'Voting Districts',
    where: `STATE='${STATE}' AND COUNTY='${COUNTY}'`,
    fields: ['GEOID', 'NAME', 'BASENAME', 'VTD', 'POP100', 'HU100', 'AREALAND', 'CENTLAT', 'CENTLON'],
    expect: 88,
  },
  // Places and CDPs are state-level layers — a place may cross a county line, which is why
  // two of this county's own municipalities do. Neither carries a COUNTY column, so both are
  // filtered by the county polygon rather than by attribute.
  {
    id: 26,
    file: 'places.geojson',
    label: 'Incorporated Places',
    where: `STATE='${STATE}'`,
    fields: ['GEOID', 'NAME', 'BASENAME', 'LSADC', 'AREALAND', 'CENTLAT', 'CENTLON'],
    clipToCounty: true,
  },
  {
    id: 28,
    file: 'census-designated-places.geojson',
    label: 'Census Designated Places',
    where: `STATE='${STATE}'`,
    fields: ['GEOID', 'NAME', 'BASENAME', 'LSADC', 'AREALAND', 'CENTLAT', 'CENTLON'],
    clipToCounty: true,
  },
]

/** Round every coordinate to `PRECISION`, and drop nulls, which ArcGIS emits freely. */
function trim(value) {
  if (typeof value === 'number') return Number(value.toFixed(PRECISION))
  if (Array.isArray(value)) return value.map(trim)
  if (value && typeof value === 'object') {
    return Object.fromEntries(
      Object.entries(value)
        .filter(([, v]) => v !== null)
        .map(([k, v]) => [k, trim(v)]),
    )
  }
  return value
}

async function query(layer, { geometry } = {}) {
  const body = new URLSearchParams({
    where: layer.where,
    outFields: layer.fields.join(','),
    outSR: '4326',
    f: 'geojson',
    returnGeometry: 'true',
  })
  if (geometry) {
    body.set('geometry', JSON.stringify(geometry))
    body.set('geometryType', 'esriGeometryPolygon')
    body.set('inSR', '4326')
    body.set('spatialRel', 'esriSpatialRelIntersects')
  }

  const res = await fetch(`${SERVICE}/${layer.id}/query`, {
    method: 'POST',
    headers: { 'content-type': 'application/x-www-form-urlencoded' },
    body,
  })
  if (!res.ok) throw new Error(`layer ${layer.id} (${layer.label}): HTTP ${res.status}`)

  const json = await res.json()
  // An ArcGIS error is a 200 with an `error` key, which is the failure mode that silently
  // writes an empty layer if nobody looks for it.
  if (json.error) throw new Error(`layer ${layer.id} (${layer.label}): ${json.error.message}`)
  if (!Array.isArray(json.features)) throw new Error(`layer ${layer.id}: no feature array`)
  return json
}

async function main() {
  await mkdir(OUT, { recursive: true })

  const provenance = {
    source: 'TIGERweb, U.S. Census Bureau',
    catalog: '.yidam/catalog/tigerweb-census2020.md',
    service: SERVICE,
    vintage: '2020',
    fetched: new Date().toISOString().slice(0, 10),
    coordinate_precision: PRECISION,
    crs: 'EPSG:4326',
    layers: [],
  }

  // Fetched first: it is both a layer and the spatial filter for the two that need one.
  const countyLayer = LAYERS[0]
  const county = await query(countyLayer)
  if (county.features.length !== 1) {
    throw new Error(`expected one county polygon, got ${county.features.length}`)
  }
  const countyRings = county.features[0].geometry.coordinates
  const countyGeometry = {
    rings: county.features[0].geometry.type === 'Polygon' ? countyRings : countyRings.flat(),
    spatialReference: { wkid: 4326 },
  }

  for (const layer of LAYERS) {
    const collection =
      layer === countyLayer ? county : await query(layer, { geometry: countyGeometry })

    const features = collection.features
    if (layer.expect !== undefined && features.length !== layer.expect) {
      // A silent change in count is the failure this connector can actually have: the
      // geography is republished, the query still succeeds, and the map quietly loses a
      // township. Refuse rather than write it.
      throw new Error(
        `layer ${layer.id} (${layer.label}): expected ${layer.expect} features, got ${features.length}`,
      )
    }

    const out = trim({
      type: 'FeatureCollection',
      features: features
        .map((f) => ({ type: 'Feature', properties: f.properties, geometry: f.geometry }))
        .toSorted((a, b) => a.properties.GEOID.localeCompare(b.properties.GEOID)),
    })

    await writeFile(join(OUT, layer.file), `${JSON.stringify(out)}\n`)
    provenance.layers.push({
      file: layer.file,
      layer: layer.id,
      name: layer.label,
      features: features.length,
      filter: layer.clipToCounty ? 'intersects the county polygon' : layer.where,
    })
    console.log(`${layer.file.padEnd(34)} ${String(features.length).padStart(3)} feature(s)`)
  }

  await writeFile(join(OUT, 'PROVENANCE.json'), `${JSON.stringify(provenance, null, 2)}\n`)
  console.log(`\nwrote ${provenance.layers.length} layer(s) and PROVENANCE.json to public/geo/`)
}

main().catch((err) => {
  console.error(`fetch-boundaries: ${err.message}`)
  process.exit(1)
})
