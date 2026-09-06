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

/**
 * Hydrography is a separate service, and it is not a 2020 statement.
 *
 * TIGERweb keeps water out of the decennial services entirely: there is one Hydro MapServer,
 * republished with the rest of TIGER, carrying no vintage of its own and no STATE or COUNTY
 * column. So the water is fetched from here, filtered by the county polygon like the two
 * place layers, and `PROVENANCE.json` records that its vintage is *current* rather than 2020.
 * A layer whose date the site cannot state is a layer the site must not date.
 *
 * **Two of its layers, not one.** 0 is the water drawn as line and 1 is the water drawn as area,
 * and neither is the county's hydrography by itself — a watercourse moves between them at the
 * width where the Bureau starts drawing two banks instead of one thread. Reading only 0 loses
 * every river wide enough to be worth the name.
 */
const HYDRO =
  'https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb/Hydro/MapServer'

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
  {
    id: 6,
    file: 'census-tracts.geojson',
    label: 'Census Tracts',
    where: `STATE='${STATE}' AND COUNTY='${COUNTY}'`,
    fields: ['GEOID', 'NAME', 'BASENAME', 'POP100', 'HU100', 'AREALAND', 'CENTLAT', 'CENTLON'],
    // Thirty-five, which is one more than the thirty-four the lending record covers: a tract
    // with no one-to-four-family houses in it takes no mortgages and appears in no such table.
    expect: 35,
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
    intersectsCounty: true,
  },
  {
    id: 28,
    file: 'census-designated-places.geojson',
    label: 'Census Designated Places',
    where: `STATE='${STATE}'`,
    fields: ['GEOID', 'NAME', 'BASENAME', 'LSADC', 'AREALAND', 'CENTLAT', 'CENTLON'],
    intersectsCounty: true,
  },
  // Unified school districts, and the one layer here that does not respect the county line.
  //
  // Ohio draws a district to hold a population, not to fit a county. Seventeen of them have
  // territory in Allen County and **twelve of those seventeen also lie in another county.** Only
  // five are wholly this county's — Allen East, Bath, Elida, Lima City and Perry — and
  // `ground.ts::containedInCounty` computes that five from this file rather than asserting it.
  //
  // So they are vendored **whole**, and the map lets them leave the frame. Clipping at the county
  // line would be cheap to look at and false about the object: the part of Pandora-Gilboa inside
  // Allen County is not a school district, it is the corner of one. The county outline is drawn
  // over the top and says where the county is; a shape running past it says something the outline
  // cannot. See `.yidam/decisions/a-district-is-not-cut-at-the-county-line.yml`.
  //
  // No COUNTY column, like the two place layers, so the filter is the county polygon. Of the
  // seventeen the corpus catalogues twelve; the other five reach in from a neighbouring county
  // and have no node, which is a gap in the corpus and not in this file.
  {
    id: 12,
    file: 'school-districts.geojson',
    label: 'Unified School Districts',
    where: `STATE='${STATE}'`,
    fields: ['GEOID', 'NAME', 'BASENAME', 'POP100', 'HU100', 'AREALAND', 'CENTLAT', 'CENTLON'],
    intersectsCounty: true,
    expect: 17,
  },
  // The water. Not a boundary and not a 2020 statement — see HYDRO above.
  //
  // Two feature classes come back and the difference between them is the county's own
  // history: H3010 is a stream or river and H3020 is a canal, ditch or aqueduct. This county
  // was drained to be farmed and dug to be reached, so the dug lines are not incidental
  // hydrography — they are the Miami & Erie Canal and the field ditches that made the Great
  // Black Swamp into cropland. The layer keeps MTFCC so the map can draw them apart.
  //
  // `OID` rather than `GEOID`: a water feature is not a geographic entity in the Census's
  // sense and carries no GEOID. OID is TIGER's permanent id and is what sorts the file.
  {
    id: 0,
    service: HYDRO,
    file: 'linear-water.geojson',
    label: 'Linear Hydrography',
    where: '1=1',
    fields: ['OID', 'NAME', 'BASENAME', 'MTFCC', 'ARTPATH'],
    key: 'OID',
    intersectsCounty: true,
    // 297 streams and 16 dug lines. The dug ones are two segments of the Miami & Erie Canal
    // spelled two ways by TIGER, plus fourteen named and unnamed ditches.
    expect: 313,
    // Generalized by the service, not by this script.
    //
    // TIGER's water is surveyed at a resolution no county-scale map can draw: 313 polylines
    // came back at 847 KB, about 130 vertices each, to describe meanders a few metres across
    // in a frame where the whole county is 1,400 pixels wide. `maxAllowableOffset` asks the
    // service for the resolution the map actually has. It is the same argument PRECISION makes
    // one level up, and it is done server-side on purpose: generalizing the geometry here
    // would make this file a cartographer rather than a mirror.
    offset: 0.0002,
  },
  // The same water, drawn as area rather than as line — and the layer this connector spent its
  // first five days without.
  //
  // A river wide enough to have two banks on the plate is a polygon in TIGER and not a polyline,
  // so it leaves the linear layer at the point where it becomes worth drawing. That is why the
  // Ottawa River — the river Lima was built on — is named nowhere in `linear-water.geojson` and
  // four times here. Layer 0 is not the county's hydrography; layer 0 and layer 1 are.
  //
  // The corpus's own catalogue of the shapefile edition said as much on the day it was written —
  // *the named ones are the rivers, the reservoirs and eight lakes* — and this connector read the
  // lines anyway. See `a-linear-file-is-not-the-hydrography`.
  {
    id: 1,
    service: HYDRO,
    file: 'areal-water.geojson',
    label: 'Areal Hydrography',
    // **Watercourses only.** The layer is 1,127 polygons in this county and 1,089 of them are a
    // lake, a pond or a reservoir — mostly farm ponds under an acre. Standing water is a
    // different subject with a different file behind it, already measured from NHD in
    // `allen-county-standing-water-2026.yml`; vendoring it a second time here would put two
    // federal counts of one ground on one map. What this file is for is the courses: H3010 the
    // natural channel, H3020 the dug one, the same two classes the linear layer carries.
    where: "MTFCC IN ('H3010','H3020')",
    fields: ['OID', 'NAME', 'BASENAME', 'MTFCC'],
    key: 'OID',
    intersectsCounty: true,
    // 41 stream or river polygons and one dug line. Twenty carry a name and there are five of
    // them: the Auglaize six times, Riley Creek six, the Ottawa four, Hog Creek twice and Sugar
    // Creek twice. Four of those twenty are the finding this layer was added for.
    //
    // **This count is filtered by the county polygon at full precision and not by the committed
    // one.** `county.geojson` is rounded to PRECISION and returns 38 for the same query — four
    // polygons graze the boundary and fall on the other side of a fifth decimal place. The
    // number to trust is the one this script computes from the geometry it just fetched.
    expect: 42,
    offset: 0.0002,
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
  if (layer.offset) body.set('maxAllowableOffset', String(layer.offset))
  if (geometry) {
    body.set('geometry', JSON.stringify(geometry))
    body.set('geometryType', 'esriGeometryPolygon')
    body.set('inSR', '4326')
    body.set('spatialRel', 'esriSpatialRelIntersects')
  }

  const res = await fetch(`${layer.service ?? SERVICE}/${layer.id}/query`, {
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
        .toSorted((a, b) =>
          String(a.properties[layer.key ?? 'GEOID']).localeCompare(
            String(b.properties[layer.key ?? 'GEOID']),
          ),
        ),
    })

    await writeFile(join(OUT, layer.file), `${JSON.stringify(out)}\n`)
    provenance.layers.push({
      file: layer.file,
      layer: layer.id,
      name: layer.label,
      features: features.length,
      // Both halves of the filter, where there are two. `areal-water.geojson` is clipped to
      // the county *and* restricted to the watercourse classes, and a provenance line naming
      // only the clip would describe a file that holds 1,127 features when it holds 38.
      filter: [
        layer.intersectsCounty ? 'intersects the county polygon' : null,
        layer.where === '1=1' ? null : layer.where,
      ]
        .filter(Boolean)
        .join(' AND '),
      // Stated per layer rather than once for the file: the water comes from a service with no
      // vintage, and letting it inherit the decennial one would date it to a year nobody
      // published it in.
      ...(layer.service ? { service: layer.service, vintage: 'current' } : {}),
      ...(layer.offset ? { generalized_degrees: layer.offset } : {}),
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
