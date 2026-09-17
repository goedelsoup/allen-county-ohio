import type { FeatureCollection, Geometry } from 'geojson'

/** The geometry files served by the map, and their browser URLs. */
export const GEO = {
  county: '/geo/county.geojson',
  subdivisions: '/geo/county-subdivisions.geojson',
  places: '/geo/places.geojson',
  cdps: '/geo/census-designated-places.geojson',
  districts: '/geo/voting-districts.geojson',
  tracts: '/geo/census-tracts.geojson',
  schools: '/geo/school-districts.geojson',
  water: '/geo/linear-water.geojson',
  arealWater: '/geo/areal-water.geojson',
} as const

export type GeometryName = keyof typeof GEO

export interface FeatureProps {
  GEOID: string
  LEVEL: string
  NAME: string
  BASENAME?: string
  POP100?: string
  HU100?: string
  AREALAND?: string
  MTFCC?: string
}

export type Collection = FeatureCollection<Geometry, FeatureProps>

/** The small part of Response used by the loader, which also makes fetch easy to replace in tests. */
export interface GeographyResponse {
  ok: boolean
  status?: number
  statusText?: string
  json(): Promise<unknown>
}

export type GeographyFetcher = (url: string, init?: RequestInit) => Promise<GeographyResponse>

export type GeographyStatus = 'idle' | 'loading' | 'ready' | 'error'

const LEVEL: Record<GeometryName, string> = {
  county: 'county',
  subdivisions: 'county-subdivision',
  places: 'place',
  cdps: 'place',
  districts: 'district',
  tracts: 'tract',
  schools: 'school-district',
  water: 'water',
  arealWater: 'water',
}

const PROPERTY_FIELDS = ['BASENAME', 'POP100', 'HU100', 'AREALAND', 'MTFCC'] as const
const GEOMETRY_TYPES = new Set([
  'Point',
  'MultiPoint',
  'LineString',
  'MultiLineString',
  'Polygon',
  'MultiPolygon',
])

function object(value: unknown): Record<string, unknown> | undefined {
  return value !== null && typeof value === 'object' ? (value as Record<string, unknown>) : undefined
}

function stringValue(value: unknown): string | undefined {
  if (typeof value === 'string') return value
  if (typeof value === 'number' && Number.isFinite(value)) return String(value)
  return undefined
}

function validGeometry(value: unknown): boolean {
  if (value === null) return true
  const geometry = object(value)
  if (!geometry || typeof geometry.type !== 'string') return false
  if (geometry.type === 'GeometryCollection') {
    return Array.isArray(geometry.geometries) && geometry.geometries.every(validGeometry)
  }
  return GEOMETRY_TYPES.has(geometry.type) && Array.isArray(geometry.coordinates)
}

function malformed(name: GeometryName, detail: string): Error {
  return new Error(`Malformed GeoJSON for ${name}: ${detail}`)
}

/** Validate and normalize one server response into the properties the map consumes. */
function parseCollection(name: GeometryName, payload: unknown): Collection {
  const raw = object(payload)
  if (!raw || raw.type !== 'FeatureCollection' || !Array.isArray(raw.features)) {
    throw malformed(name, 'expected a FeatureCollection')
  }

  const features = raw.features.map((featureValue, index) => {
    const rawFeature = object(featureValue)
    const properties = rawFeature && object(rawFeature.properties)
    if (!rawFeature || rawFeature.type !== 'Feature' || !properties || !validGeometry(rawFeature.geometry)) {
      throw malformed(name, `invalid feature at index ${index}`)
    }

    // Hydrography is the one source here that has no Census GEOID and whose linear features are
    // commonly unnamed. OID is stable within these vendored files and gives those features the
    // same typed identity as every boundary; the explicit label keeps hover text meaningful.
    const sourceId = stringValue(properties.GEOID)
      ?? (name === 'water' || name === 'arealWater' ? stringValue(properties.OID) : undefined)
    const geoid = sourceId
    const sourceName = stringValue(properties.NAME)
      ?? (name === 'water'
        ? 'Unnamed watercourse'
        : name === 'arealWater'
          ? 'Unnamed water'
          : undefined)
    if (!geoid || !sourceName) {
      throw malformed(name, `feature ${index} is missing GEOID or NAME`)
    }

    const normalized: FeatureProps = {
      GEOID: geoid,
      LEVEL: LEVEL[name],
      NAME: sourceName,
    }
    for (const field of PROPERTY_FIELDS) {
      const fieldValue = stringValue(properties[field])
      if (fieldValue !== undefined) normalized[field] = fieldValue
    }

    const normalizedFeature: Collection['features'][number] = {
      type: 'Feature' as const,
      geometry: rawFeature.geometry as Collection['features'][number]['geometry'],
      properties: normalized,
    }
    if (rawFeature.id !== undefined) normalizedFeature.id = rawFeature.id as string | number
    return normalizedFeature
  })

  if (['county', 'districts', 'tracts'].includes(name) && features.length === 0) {
    throw malformed(name, 'boundary collection is empty')
  }
  return { type: 'FeatureCollection', features }
}

interface CacheEntry {
  status: GeographyStatus
  data?: Collection
  error?: Error
  promise?: Promise<Collection>
}

/**
 * A per-layer request cache for the map's GeoJSON.
 *
 * Requests already in flight are shared. Successful responses remain cached for the life of the
 * loader. A failed response remains failed until the caller opts into `retry`, so a transient
 * source problem cannot turn every render into an unbounded request loop.
 */
export function createGeographyLoader(fetcher: GeographyFetcher = (url, init) => fetch(url, init)): {
  load(name: GeometryName, options?: { retry?: boolean }): Promise<Collection>
  status(name: GeometryName): GeographyStatus
  get(name: GeometryName): Collection | undefined
} {
  const cache = new Map<GeometryName, CacheEntry>()

  const load = (name: GeometryName, options: { retry?: boolean } = {}): Promise<Collection> => {
    const existing = cache.get(name)
    if (existing?.status === 'ready' && existing.data) return Promise.resolve(existing.data)
    if (existing?.status === 'loading' && existing.promise) return existing.promise
    if (existing?.status === 'error' && existing.promise && !options.retry) return existing.promise

    const entry: CacheEntry = { status: 'loading' }
    cache.set(name, entry)
    let promise: Promise<Collection>
    promise = Promise.resolve()
      .then(() => fetcher(GEO[name], { signal: AbortSignal.timeout(15_000) }))
      .then((response) => {
        if (!response || response.ok !== true) {
          const status = response?.status === undefined ? '' : ` (${response.status})`
          const message = response?.statusText ? `: ${response.statusText}` : ''
          throw new Error(`Failed to load geography ${name}${status}${message}`)
        }
        return response.json()
      })
      .then((value) => parseCollection(name, value))
      .then(
        (data) => {
          if (cache.get(name)?.promise === promise) {
            entry.status = 'ready'
            entry.data = data
          }
          return data
        },
        (reason: unknown) => {
          const error = reason instanceof Error ? reason : new Error(String(reason))
          if (cache.get(name)?.promise === promise) {
            entry.status = 'error'
            entry.error = error
          }
          throw error
        },
      )
    entry.promise = promise
    return promise
  }

  return {
    load,
    status: (name) => cache.get(name)?.status ?? 'idle',
    get: (name) => {
      const entry = cache.get(name)
      return entry?.status === 'ready' ? entry.data : undefined
    },
  }
}
