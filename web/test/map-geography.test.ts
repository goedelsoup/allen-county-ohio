import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import {
  createGeographyLoader,
  GEO,
  type GeographyFetcher,
  type GeographyResponse,
} from '../src/lib/map-geography'

const geometry = { type: 'Point' as const, coordinates: [-84, 40] as [number, number] }

function response(body: unknown, ok = true): GeographyResponse {
  return { ok, status: ok ? 200 : 503, statusText: ok ? 'OK' : 'Unavailable', json: async () => body }
}

function collection(properties: Record<string, unknown> = { GEOID: '39003', NAME: 'Allen County' }) {
  return { type: 'FeatureCollection', features: [{ type: 'Feature', geometry, properties }] }
}

describe('the geography loader', () => {
  it('aborts a stalled browser fetch and permits an explicit retry', async () => {
    const controller = new AbortController()
    vi.spyOn(AbortSignal, 'timeout').mockReturnValue(controller.signal)
    vi.stubGlobal('fetch', vi.fn()
      .mockImplementationOnce((_url: string, init: RequestInit) => new Promise((_resolve, reject) => {
        init.signal?.addEventListener('abort', () => reject(new Error('Request timed out')))
      }))
      .mockResolvedValue(response(collection())))
    try {
      const loader = createGeographyLoader()
      const pending = loader.load('county')
      await Promise.resolve()
      controller.abort()
      await expect(pending).rejects.toThrow('Request timed out')
      expect(loader.status('county')).toBe('error')
      await expect(loader.load('county', { retry: true })).resolves.toHaveProperty('type', 'FeatureCollection')
    } finally {
      vi.restoreAllMocks()
      vi.unstubAllGlobals()
    }
  })

  it('exports the nine map URLs and stamps the source level into each feature', async () => {
    expect(GEO).toEqual({
      county: '/geo/county.geojson',
      subdivisions: '/geo/county-subdivisions.geojson',
      places: '/geo/places.geojson',
      cdps: '/geo/census-designated-places.geojson',
      districts: '/geo/voting-districts.geojson',
      tracts: '/geo/census-tracts.geojson',
      schools: '/geo/school-districts.geojson',
      water: '/geo/linear-water.geojson',
      arealWater: '/geo/areal-water.geojson',
    })

    const fetcher: GeographyFetcher = async () =>
      response(collection({ GEOID: '3904752', NAME: 'Beaverdam village' }))
    const loader = createGeographyLoader(fetcher)
    const levels = [
      ['county', 'county'],
      ['subdivisions', 'county-subdivision'],
      ['places', 'place'],
      ['cdps', 'place'],
      ['districts', 'district'],
      ['tracts', 'tract'],
      ['schools', 'school-district'],
      ['water', 'water'],
      ['arealWater', 'water'],
    ] as const

    const loaded = await Promise.all(levels.map(async ([name]) => loader.load(name)))
    loaded.forEach((result, index) => {
      const feature = result.features[0]
      const level = levels[index][1]
      expect(`${feature.properties.LEVEL}:${feature.properties.GEOID}`).toBe(`${level}:3904752`)
    })
  })

  it('accepts every committed geography collection and normalizes its source properties', async () => {
    const directory = join(import.meta.dirname, '..', 'public', 'geo')
    const fetcher: GeographyFetcher = async (url) => {
      const file = url.replace('/geo/', '')
      return response(JSON.parse(readFileSync(join(directory, file), 'utf8')))
    }
    const loader = createGeographyLoader(fetcher)
    const names = Object.keys(GEO) as (keyof typeof GEO)[]
    const loaded = await Promise.all(names.map((name) => loader.load(name)))

    expect(loaded.every((result) => result.type === 'FeatureCollection')).toBe(true)
    expect(loaded.every((result) => result.features.length > 0)).toBe(true)
    const linearWater = loaded[names.indexOf('water')].features[0].properties
    expect(linearWater.LEVEL).toBe('water')
    expect(linearWater.GEOID).toBe('1102205002059')
    expect(linearWater.NAME).toBe('Unnamed watercourse')
    const county = loaded[names.indexOf('county')].features[0].properties
    expect(county.AREALAND).toBe('1042587393')
    expect(county.LEVEL).toBe('county')
  })

  it('deduplicates an in-flight request and caches a successful response', async () => {
    let calls = 0
    let resolve!: (value: GeographyResponse) => void
    const pending = new Promise<GeographyResponse>((r) => { resolve = r })
    const loader = createGeographyLoader(async () => { calls += 1; return pending })

    const first = loader.load('county')
    const second = loader.load('county')
    expect(first).toBe(second)
    expect(loader.status('county')).toBe('loading')
    resolve(response(collection()))
    const loaded = await first
    expect(calls).toBe(1)
    expect(loader.status('county')).toBe('ready')
    expect(loader.get('county')).toBe(loaded)

    await loader.load('county')
    expect(calls).toBe(1)
  })

  it('isolates failures and only retries after an explicit request', async () => {
    let countyCalls = 0
    const fetcher: GeographyFetcher = async (url) => {
      if (url === GEO.county) {
        countyCalls += 1
        return countyCalls === 1 ? response({}, false) : response(collection())
      }
      return response(collection({ GEOID: '3904752', NAME: 'Beaverdam village' }))
    }
    const loader = createGeographyLoader(fetcher)

    await expect(loader.load('county')).rejects.toThrow('Failed to load geography county')
    expect(loader.status('county')).toBe('error')
    expect(loader.get('county')).toBeUndefined()
    await expect(loader.load('county')).rejects.toThrow('Failed to load geography county')
    expect(countyCalls).toBe(1)

    await expect(loader.load('places')).resolves.toMatchObject({ type: 'FeatureCollection' })
    expect(loader.status('places')).toBe('ready')
    await expect(loader.load('county', { retry: true })).resolves.toMatchObject({ type: 'FeatureCollection' })
    expect(countyCalls).toBe(2)
  })

  it('rejects malformed collections and malformed features', async () => {
    const loader = createGeographyLoader(async (url) =>
      url === GEO.county
        ? response({ type: 'FeatureCollection', features: [{ type: 'Feature', properties: {}, geometry }] })
        : response({ type: 'GeometryCollection', geometries: [] }),
    )

    await expect(loader.load('county')).rejects.toThrow('missing GEOID or NAME')
    expect(loader.status('county')).toBe('error')
    await expect(loader.load('places')).rejects.toThrow('expected a FeatureCollection')
    expect(loader.status('places')).toBe('error')
  })

  it.each(['county', 'districts', 'tracts'] as const)('rejects empty %s geography needed for the frame or classification', async (name) => {
    const loader = createGeographyLoader(async () => response({ type: 'FeatureCollection', features: [] }))
    await expect(loader.load(name)).rejects.toThrow('boundary collection is empty')
    expect(loader.status(name)).toBe('error')
  })
})
