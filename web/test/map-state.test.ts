import { describe, expect, it } from 'vitest'
import { parseMapState, serializeMapState, type MapState } from '../src/lib/map-state'

const extent = { from: 1800, to: 2024 }

const defaults = (): MapState => ({
  year: 2024,
  grain: 'era',
  generous: false,
  hops: 3,
  undated: false,
  ground: new Set(['subdivisions', 'municipalities', 'water']),
  selected: null,
  camera: null,
})

describe('map URL state', () => {
  it('keeps the old empty-link defaults', () => {
    expect(parseMapState('', extent)).toEqual(defaults())
  })

  it('parses and serializes every state field, including hop zero', () => {
    const state = parseMapState(
      '?year=1818&grain=year&generous=1&hops=0&undated=1&at=place%2Flima.yml&layers=water%2Cclaims&lon=-84.1234567&lat=40.7654321&zoom=8.1259',
      extent,
    )
    expect(state).toEqual({
      year: 1818,
      grain: 'year',
      generous: true,
      hops: 0,
      undated: true,
      ground: new Set(['water', 'claims']),
      selected: 'place/lima.yml',
      camera: { longitude: -84.1234567, latitude: 40.7654321, zoom: 8.1259 },
    })

    const encoded = serializeMapState(state)
    expect(encoded).toBe(
      'year=1818&grain=year&hops=0&generous=1&undated=1&at=place%2Flima.yml&layers=claims%2Cwater&lon=-84.123457&lat=40.765432&zoom=8.126',
    )
    expect(parseMapState(encoded, extent)).toEqual({ ...state, camera: { longitude: -84.123457, latitude: 40.765432, zoom: 8.126 } })
  })

  it('preserves an explicit empty layer set', () => {
    const state = parseMapState('layers=', extent)
    expect(state.ground).toEqual(new Set())
    expect(serializeMapState(state)).toBe('year=2024&layers=')
  })

  it('drops unknown layers and keeps only the first thematic layer', () => {
    const state = parseMapState('layers=bogus,tracts,population,denial,schools,tracts', extent)
    expect(state.ground).toEqual(new Set(['tracts', 'schools']))
    expect(serializeMapState(state)).toBe('year=2024&layers=schools%2Ctracts')
  })

  it('clamps valid integer years and falls back on malformed years and hops', () => {
    expect(parseMapState('year=-20&hops=4', extent)).toMatchObject({ year: 1800, hops: 3 })
    expect(parseMapState('year=9999&hops=1.5', extent)).toMatchObject({ year: 2024, hops: 3 })
    expect(parseMapState('year=18.5&hops=nope', extent)).toMatchObject({ year: 2024, hops: 3 })
    // This was the old missing-year sentinel and remains a useful legacy link.
    expect(parseMapState('year=0', extent).year).toBe(2024)
  })

  it('decodes and re-encodes selected node IDs', () => {
    const state = parseMapState('at=jurisdiction%2Ftown%20ship%3Fx.yml', extent)
    expect(state.selected).toBe('jurisdiction/town ship?x.yml')
    expect(new URLSearchParams(serializeMapState(state)).get('at')).toBe('jurisdiction/town ship?x.yml')
  })

  it('rejects a partial or invalid camera as a unit', () => {
    for (const query of [
      'lon=-84&lat=40',
      'lon=oops&lat=40&zoom=8',
      'lon=181&lat=40&zoom=8',
      'lon=-84&lat=90&zoom=8',
      'lon=-84&lat=40&zoom=15',
    ]) {
      expect(parseMapState(query, extent).camera, query).toBeNull()
    }
  })
})

