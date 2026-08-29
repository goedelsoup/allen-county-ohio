// The county, drawn.
//
// Two kinds of thing are on this map and they are coloured to stay apart. **The ground** —
// county, townships, municipalities, voting districts — is vendored 2020 Census geography,
// held in `public/geo/` with its provenance beside it; it is drawn in neutrals and a single
// sequential blue. **The corpus** — the places, sites and features this repository actually
// makes claims about — is drawn in one accent colour on top. A reader should never have to
// wonder which of the two they are looking at.
//
// There is no basemap and no tile server. Everything is local, so the map works offline and
// nothing about a visitor reaches a third party.

import { Deck, WebMercatorViewport } from '@deck.gl/core'
import { GeoJsonLayer, ScatterplotLayer, TextLayer } from '@deck.gl/layers'

type RGBA = [number, number, number, number]

interface CorpusPoint {
  id: string
  class: string
  label: string
  tier: string
  lat: number
  lon: number
  geoid: string | null
  kind: string | null
  area_sq_mi: string | null
}

interface FeatureProps {
  GEOID: string
  NAME: string
  BASENAME?: string
  POP100?: string
  HU100?: string
  AREALAND?: string
}

type Collection = { type: 'FeatureCollection'; features: { properties: FeatureProps }[] }

const GEO = {
  county: '/geo/county.geojson',
  subdivisions: '/geo/county-subdivisions.geojson',
  places: '/geo/places.geojson',
  cdps: '/geo/census-designated-places.geojson',
  districts: '/geo/voting-districts.geojson',
} as const

/** `#2a78d6` → `[42, 120, 214, alpha]`. */
function rgb(hex: string, alpha = 255): RGBA {
  const h = hex.trim().replace('#', '')
  const full = h.length === 3 ? [...h].map((c) => c + c).join('') : h
  return [
    parseInt(full.slice(0, 2), 16),
    parseInt(full.slice(2, 4), 16),
    parseInt(full.slice(4, 6), 16),
    alpha,
  ]
}

function palette() {
  const s = getComputedStyle(document.documentElement)
  const v = (name: string) => s.getPropertyValue(name).trim()
  return {
    ink: v('--ink'),
    muted: v('--ink-muted'),
    rule: v('--rule'),
    ruleStrong: v('--rule-strong'),
    accent: v('--series-2'),
    surface: v('--surface'),
    // One hue, light to dark. Five steps, because past about seven bins adjacent classes blur.
    ramp: [v('--seq-100'), v('--seq-250'), v('--seq-400'), v('--seq-550'), v('--seq-700')],
  }
}

/**
 * Quantile breaks, so each class holds about the same number of precincts.
 *
 * Equal-interval breaks on this distribution put seventy of the eighty-eight precincts in
 * the lightest class and say nothing.
 */
export function quantileBreaks(values: number[], classes: number): number[] {
  const sorted = values.filter((v) => Number.isFinite(v)).toSorted((a, b) => a - b)
  if (sorted.length === 0) return []
  const breaks: number[] = []
  for (let i = 1; i < classes; i++) {
    const at = (i / classes) * (sorted.length - 1)
    const lo = Math.floor(at)
    const hi = Math.ceil(at)
    breaks.push(sorted[lo] + (sorted[hi] - sorted[lo]) * (at - lo))
  }
  return breaks
}

/** Which class a value falls in, given ascending breaks. */
export function classOf(value: number, breaks: number[]): number {
  let i = 0
  while (i < breaks.length && value > breaks[i]) i++
  return i
}

function number(raw: string | undefined): number {
  const n = Number(raw)
  return Number.isFinite(n) ? n : 0
}

/** The bounding box of every coordinate in a GeoJSON geometry. */
export function bounds(geometry: unknown): [number, number, number, number] {
  let minLng = Infinity
  let minLat = Infinity
  let maxLng = -Infinity
  let maxLat = -Infinity

  const walk = (node: unknown): void => {
    if (!Array.isArray(node)) return
    if (typeof node[0] === 'number' && typeof node[1] === 'number') {
      const [lng, lat] = node as [number, number]
      minLng = Math.min(minLng, lng)
      minLat = Math.min(minLat, lat)
      maxLng = Math.max(maxLng, lng)
      maxLat = Math.max(maxLat, lat)
      return
    }
    for (const child of node) walk(child)
  }

  walk((geometry as { coordinates?: unknown }).coordinates)
  return [minLng, minLat, maxLng, maxLat]
}

async function collection(url: string): Promise<Collection> {
  const res = await fetch(url)
  if (!res.ok) throw new Error(`${url}: HTTP ${res.status}`)
  return (await res.json()) as Collection
}

interface Hover {
  x: number
  y: number
  title: string
  rows: [string, string][]
  kind: 'ground' | 'corpus'
}

export async function renderMap(container: HTMLElement, points: CorpusPoint[]): Promise<void> {
  const tooltip = container.querySelector<HTMLElement>('[data-map-tooltip]')
  const legend = container.querySelector<HTMLElement>('[data-map-legend]')
  const canvasHost = container.querySelector<HTMLElement>('[data-map-canvas]')
  if (!canvasHost) throw new Error('the map has no canvas host')

  const [county, subdivisions, places, cdps, districts] = await Promise.all([
    collection(GEO.county),
    collection(GEO.subdivisions),
    collection(GEO.places),
    collection(GEO.cdps),
    collection(GEO.districts),
  ])

  // The county's own node is excluded: its extent is the whole frame, so a label at its
  // centroid names Lima's neighbourhood rather than the county, and collides with Lima doing it.
  const labelled = points.filter((p) => p.class === 'place' && p.kind !== 'county')
  const populations = districts.features.map((f) => number(f.properties.POP100))
  const breaks = quantileBreaks(populations, 5)

  const shown = new Set(['districts', 'subdivisions', 'municipalities', 'corpus'])
  let hover: Hover | null = null

  const setTooltip = () => {
    if (!tooltip) return
    if (!hover) {
      tooltip.hidden = true
      return
    }
    tooltip.hidden = false
    tooltip.style.left = `${hover.x}px`
    tooltip.style.top = `${hover.y}px`
    tooltip.dataset.kind = hover.kind
    tooltip.innerHTML = `<strong>${hover.title}</strong>${hover.rows
      .map(([k, v]) => `<span><em>${k}</em>${v}</span>`)
      .join('')}`
  }

  const layers = () => {
    const p = palette()
    const rampRgb = p.ramp.map((c) => rgb(c, 205))

    return [
      shown.has('districts') &&
        new GeoJsonLayer({
          id: 'voting-districts',
          data: districts as unknown as object,
          filled: true,
          stroked: true,
          getFillColor: (f: { properties: FeatureProps }) =>
            rampRgb[classOf(number(f.properties.POP100), breaks)],
          getLineColor: rgb(p.surface, 150),
          getLineWidth: 12,
          lineWidthMinPixels: 0.5,
          pickable: true,
          onHover: ({ object, x, y }) => {
            hover = object
              ? {
                  x,
                  y,
                  kind: 'ground',
                  title: (object as { properties: FeatureProps }).properties.NAME,
                  rows: [
                    ['Population', number((object as { properties: FeatureProps }).properties.POP100).toLocaleString('en-US')],
                    ['Housing units', number((object as { properties: FeatureProps }).properties.HU100).toLocaleString('en-US')],
                    ['Voting district', '2020 Census geography'],
                  ],
                }
              : null
            setTooltip()
          },
        }),

      shown.has('subdivisions') &&
        new GeoJsonLayer({
          id: 'subdivisions',
          data: subdivisions as unknown as object,
          filled: false,
          stroked: true,
          getLineColor: rgb(p.muted, 190),
          getLineWidth: 30,
          lineWidthMinPixels: 1,
          pickable: true,
          onHover: ({ object, x, y }) => {
            hover = object
              ? {
                  x,
                  y,
                  kind: 'ground',
                  title: (object as { properties: FeatureProps }).properties.NAME,
                  rows: [['Civil subdivision', '2020 Census geography']],
                }
              : null
            setTooltip()
          },
        }),

      shown.has('municipalities') &&
        new GeoJsonLayer({
          id: 'municipalities',
          data: {
            type: 'FeatureCollection',
            features: [...places.features, ...cdps.features],
          } as unknown as object,
          filled: true,
          stroked: true,
          getFillColor: rgb(p.ink, 26),
          getLineColor: rgb(p.ink, 150),
          getLineWidth: 22,
          lineWidthMinPixels: 1,
          pickable: true,
          onHover: ({ object, x, y }) => {
            hover = object
              ? {
                  x,
                  y,
                  kind: 'ground',
                  title: (object as { properties: FeatureProps }).properties.NAME,
                  rows: [['Municipality or CDP', '2020 Census geography']],
                }
              : null
            setTooltip()
          },
        }),

      new GeoJsonLayer({
        id: 'county',
        data: county as unknown as object,
        filled: false,
        stroked: true,
        getLineColor: rgb(p.ink, 235),
        getLineWidth: 60,
        lineWidthMinPixels: 1.75,
      }),

      shown.has('corpus') &&
        new ScatterplotLayer<CorpusPoint>({
          id: 'corpus-points',
          data: points,
          getPosition: (d) => [d.lon, d.lat],
          // Size separates a named place from a single site; hue is already carrying the
          // corpus-versus-ground distinction and cannot carry a second thing as well.
          getRadius: (d) => (d.class === 'place' ? 320 : 190),
          radiusMinPixels: 4,
          radiusMaxPixels: 11,
          getFillColor: rgb(p.accent, 245),
          // A 2px surface ring, not a border: overlapping points stay separable.
          stroked: true,
          getLineColor: rgb(p.surface, 255),
          lineWidthMinPixels: 2,
          pickable: true,
          // The hit area is bigger than the mark, so a 10px dot is not a pinpoint target.
          radiusScale: 1,
          onHover: ({ object, x, y }) => {
            hover = object
              ? {
                  x,
                  y,
                  kind: 'corpus',
                  title: object.label,
                  rows: [
                    ['In the corpus as', object.class],
                    ...(object.kind ? ([['Type', object.kind]] as [string, string][]) : []),
                    ...(object.area_sq_mi
                      ? ([['Area', object.area_sq_mi]] as [string, string][])
                      : []),
                    ['Claim tag', object.tier],
                  ],
                }
              : null
            setTooltip()
          },
        }),

      shown.has('corpus') &&
        new TextLayer<CorpusPoint>({
          id: 'corpus-labels',
          // Places only. Five sites and the courthouse sit within two miles of each other in
          // Lima, and labelling every node overprints that corner into an unreadable smear.
          // The places name the ground, which is what a label is for; everything else is a
          // mark with a tooltip, which is what a hit target is for.
          data: labelled,
          getPosition: (d) => [d.lon, d.lat],
          getText: (d) => d.label,
          getSize: 12,
          sizeUnits: 'pixels',
          getColor: rgb(p.ink, 235),
          getPixelOffset: [0, -14],
          fontFamily: 'system-ui, -apple-system, "Segoe UI", sans-serif',
          fontWeight: 500,
          outlineWidth: 3,
          outlineColor: rgb(p.surface, 255),
          fontSettings: { sdf: true },
          getTextAnchor: 'middle',
          getAlignmentBaseline: 'bottom',
        }),
    ].filter(Boolean)
  }

  // Fit the county rather than hard-coding a zoom: the frame is whatever shape the reader's
  // window is, and a fixed zoom leaves the county small on a wide screen and cropped on a
  // narrow one.
  const box = bounds((county.features[0] as unknown as { geometry: unknown }).geometry)
  const rect = canvasHost.getBoundingClientRect()
  const fitted = new WebMercatorViewport({
    width: Math.max(rect.width, 1),
    height: Math.max(rect.height, 1),
  }).fitBounds(
    [
      [box[0], box[1]],
      [box[2], box[3]],
    ],
    { padding: 32 },
  )

  const deck = new Deck({
    parent: canvasHost,
    initialViewState: {
      longitude: fitted.longitude,
      latitude: fitted.latitude,
      zoom: fitted.zoom,
      minZoom: fitted.zoom - 1,
      maxZoom: 14,
      bearing: 0,
      pitch: 0,
    },
    controller: { dragRotate: false },
    layers: layers(),
    getCursor: ({ isHovering }) => (isHovering ? 'pointer' : 'grab'),
    style: { position: 'absolute', inset: '0' },
  })

  const refresh = () => deck.setProps({ layers: layers() })

  // Legend for the sequential scale. A continuous colour encoding without one is unreadable.
  if (legend) {
    const p = palette()
    const edges = [0, ...breaks, Math.max(...populations)]
    legend.innerHTML = p.ramp
      .map(
        (c, i) =>
          `<span class="swatch"><i style="background:${c}"></i>${Math.round(
            edges[i],
          ).toLocaleString('en-US')}–${Math.round(edges[i + 1]).toLocaleString('en-US')}</span>`,
      )
      .join('')
  }

  for (const input of container.querySelectorAll<HTMLInputElement>('[data-layer]')) {
    input.addEventListener('change', () => {
      const key = input.dataset.layer
      if (!key) return
      if (input.checked) shown.add(key)
      else shown.delete(key)
      refresh()
    })
  }

  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', refresh)
  new MutationObserver(refresh).observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['data-theme'],
  })
}
