/**
 * The part of the map that can be carried by a link.
 *
 * This module deliberately has no browser dependencies. The map renderer owns the history
 * policy; this file only validates and serializes the state that policy puts in the address bar.
 */

export type MapGrain = 'era' | 'year'

export interface MapCamera {
  longitude: number
  latitude: number
  zoom: number
}

export interface MapState {
  year: number
  grain: MapGrain
  generous: boolean
  hops: number
  undated: boolean
  ground: Set<string>
  selected: string | null
  camera: MapCamera | null
}

export interface MapExtent {
  from: number
  to: number
}

/** Keys rendered by the map's Ground controls. */
export const MAP_LAYERS = [
  'subdivisions',
  'municipalities',
  'water',
  'schools',
  'claims',
  'population',
  'tracts',
  'denial',
  'priced',
] as const

export const CHOROPLETH_LAYERS = ['population', 'tracts', 'denial', 'priced'] as const

const DEFAULT_LAYERS = ['subdivisions', 'municipalities', 'water'] as const
const KNOWN_LAYERS = new Set<string>(MAP_LAYERS)
const CHOROPLETH_SET = new Set<string>(CHOROPLETH_LAYERS)

const LATITUDE_LIMIT = 85.051129

function finiteNumber(raw: string | null): number | null {
  if (raw === null || raw.trim() === '') return null
  const value = Number(raw)
  return Number.isFinite(value) ? value : null
}

function validYear(raw: string | null): number | null {
  const value = finiteNumber(raw)
  // year=0 was the old map's missing-value sentinel. Keep links generated with that value
  // opening at the end of the record, while rejecting fractional years.
  return value !== null && value !== 0 && Number.isInteger(value) ? value : null
}

function extentBounds(extent: MapExtent): { from: number; to: number } {
  const from = Number.isFinite(extent.from) ? extent.from : 0
  const to = Number.isFinite(extent.to) ? extent.to : from
  return from <= to ? { from, to } : { from: to, to: from }
}

function clampYear(value: number | null, extent: MapExtent): number {
  const { from, to } = extentBounds(extent)
  const year = value ?? to
  return Math.min(Math.max(year, from), to)
}

function parseLayers(params: URLSearchParams): Set<string> {
  // Presence matters: `layers=` is an explicit request for no ground, whereas omission keeps
  // the map's useful default layers.
  const raw = params.has('layers') ? params.get('layers') ?? '' : DEFAULT_LAYERS.join(',')
  const layers: string[] = []
  for (const layer of raw.split(',')) {
    if (!KNOWN_LAYERS.has(layer) || layers.includes(layer)) continue
    // A URL has no interaction order, so the first thematic layer is the deterministic winner
    // when an old or hand-written link names contradictory choropleths.
    if (CHOROPLETH_SET.has(layer) && layers.some((item) => CHOROPLETH_SET.has(item))) continue
    layers.push(layer)
  }
  return new Set(layers)
}

function validCamera(camera: MapCamera | null | undefined): camera is MapCamera {
  return (
    camera !== null &&
    camera !== undefined &&
    Number.isFinite(camera.longitude) &&
    camera.longitude >= -180 &&
    camera.longitude <= 180 &&
    Number.isFinite(camera.latitude) &&
    camera.latitude >= -LATITUDE_LIMIT &&
    camera.latitude <= LATITUDE_LIMIT &&
    Number.isFinite(camera.zoom) &&
    camera.zoom >= 0 &&
    camera.zoom <= 14
  )
}

function parseCamera(params: URLSearchParams): MapCamera | null {
  const longitude = finiteNumber(params.get('lon'))
  const latitude = finiteNumber(params.get('lat'))
  const zoom = finiteNumber(params.get('zoom'))
  if (longitude === null || latitude === null || zoom === null) return null
  const camera = { longitude, latitude, zoom }
  return validCamera(camera) ? camera : null
}

/** Parse a browser search string (with or without its leading `?`) into validated map state. */
export function parseMapState(search: string, extent: MapExtent): MapState {
  const params = new URLSearchParams(search)
  const year = clampYear(validYear(params.get('year')), extent)
  const askedHops = finiteNumber(params.get('hops'))
  const hops = params.has('hops') && askedHops !== null && Number.isInteger(askedHops) && askedHops >= 0 && askedHops <= 3
    ? askedHops
    : 3
  const selected = params.get('at')

  return {
    year,
    grain: params.get('grain') === 'year' ? 'year' : 'era',
    generous: params.get('generous') === '1',
    hops,
    undated: params.get('undated') === '1',
    ground: parseLayers(params),
    selected: selected === null || selected === '' ? null : selected,
    camera: parseCamera(params),
  }
}

function canonicalLayers(ground: Set<string>): string[] {
  const layers: string[] = []
  for (const layer of ground) {
    if (!KNOWN_LAYERS.has(layer) || layers.includes(layer)) continue
    if (CHOROPLETH_SET.has(layer) && layers.some((item) => CHOROPLETH_SET.has(item))) continue
    layers.push(layer)
  }
  return layers.toSorted()
}

function precision(value: number, places: number): string {
  return value.toFixed(places)
}

/** Serialize map state to a stable query string without a leading `?`. */
export function serializeMapState(state: MapState): string {
  const params = new URLSearchParams()
  if (Number.isFinite(state.year) && Number.isInteger(state.year)) params.set('year', String(state.year))
  if (state.grain === 'year') params.set('grain', 'year')
  if (Number.isInteger(state.hops) && state.hops >= 0 && state.hops <= 3 && state.hops !== 3) {
    params.set('hops', String(state.hops))
  }
  if (state.generous) params.set('generous', '1')
  if (state.undated) params.set('undated', '1')
  if (state.selected) params.set('at', state.selected)

  // Always write layers. Besides making state round-trips lossless, this distinguishes an
  // explicit empty layer set from an omitted `layers` parameter with the old defaults.
  params.set('layers', canonicalLayers(state.ground).join(','))

  // Camera state is atomic. A partial or out-of-range camera cannot be restored reliably, so it
  // is omitted as a whole rather than producing a URL that looks partially valid.
  if (validCamera(state.camera)) {
    params.set('lon', precision(state.camera.longitude, 6))
    params.set('lat', precision(state.camera.latitude, 6))
    params.set('zoom', precision(state.camera.zoom, 3))
  }

  return params.toString()
}

