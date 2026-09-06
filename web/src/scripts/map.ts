// The county, drawn, carrying a year.
//
// Three kinds of thing are on this map and they are kept apart by what they are made of rather
// than by hue, because hue is spent on the era.
//
//   **The ground** — county, townships, municipalities, voting districts, tracts — is vendored
//   2020 Census geography, held in `public/geo/` with its provenance beside it, drawn in
//   neutrals. It is not corpus content; it is what the corpus is drawn against. It fades as the
//   reader travels away from 2020, because it is a 2020 statement and it does not travel.
//
//   **The corpus, placed** — every node `crates/placement` can put on ground, filtered to the
//   window the reader is looking at. Marks take the ink of the era they are being viewed in, so
//   travelling changes the colour of the map as well as its contents.
//
//   **The corpus, undated** — 183 nodes the corpus holds and cannot date, on their own toggle
//   and in their own ink. They stand at no year, so folding them into one would be a lie of a
//   kind this repository has a decision about.
//
// There is no basemap and no tile server. Everything is local, so the map works offline and
// nothing about a visitor reaches a third party.
//
// ---- What is encoded, and in what ----
//
//   era            hue, one tint at a time. Never six at once — the ramp measures as neither a
//                  categorical palette nor a monotonic scale, and `dark.css` records why.
//   how much       radius. A stack of ninety figures about Lima is one mark, not ninety pins.
//   warrant        opacity, in four concentric steps. The solid core is positions somebody
//                  stated; the pale rim is three edges of inference. Nothing is hidden by it —
//                  the reader can also shrink the map by refusing the derived placements.
//   selection      rubric, the system's one emphatic ink, on one mark at a time.
//   undated        the missing-status ink, which belongs to no era on purpose.

import { Deck, WebMercatorViewport } from '@deck.gl/core'
import { ArcLayer, GeoJsonLayer, ScatterplotLayer, TextLayer } from '@deck.gl/layers'
import {
  anchored,
  drawn,
  eraAt,
  groundFidelity,
  shaded,
  spine,
  undrawn,
  view,
  WATER_VINTAGE,
  type Anchored,
  type Era,
  type Grain,
  type View,
} from '../lib/eras'
import { lines, type Line } from '../lib/edges'
import { courses, type Course, type End } from '../lib/water'
import { entryPath } from '../lib/entry'
import { column, tableFor, type AtlasRecord } from '../lib/feeds'

type RGBA = [number, number, number, number]

interface FeatureProps {
  GEOID: string
  /** Stamped on at load from the file the feature came out of — see `LEVELS`. */
  LEVEL: string
  NAME: string
  BASENAME?: string
  POP100?: string
  HU100?: string
  AREALAND?: string
}

type Collection = { type: 'FeatureCollection'; features: { properties: FeatureProps }[] }

/**
 * How a vendored shape is addressed: its summary level, then its key.
 *
 * A GEOID is unique only inside its level, and this county proves it — `3904752` is Beaverdam
 * village and the Upper Scioto Valley Local School District. Every index over `public/geo/` is
 * keyed through here. See `a-geoid-is-not-an-address`.
 */
const shapeKey = (f: { properties: FeatureProps }) => `${f.properties.LEVEL}:${f.properties.GEOID}`

/**
 * The ground layers that own every pixel they cover.
 *
 * One at a time, always. A choropleth is a fill encoding over the whole county, and two of them
 * stacked is one drawn on top of the other for no gain — and the era tint laid over either is
 * two magnitudes in one hue. Showing them singly is also what makes the pair of mortgage layers
 * a comparison rather than a smear: you switch, and the little that moves is the finding.
 */
const CHOROPLETHS = ['population', 'tracts', 'denial', 'priced'] as const

const GEO = {
  county: '/geo/county.geojson',
  subdivisions: '/geo/county-subdivisions.geojson',
  places: '/geo/places.geojson',
  cdps: '/geo/census-designated-places.geojson',
  districts: '/geo/voting-districts.geojson',
  tracts: '/geo/census-tracts.geojson',
  schools: '/geo/school-districts.geojson',
  water: '/geo/linear-water.geojson',
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
  const n = (name: string) => Number(v(name))
  return {
    ink: v('--text-strong'),
    muted: v('--text-muted'),
    faint: v('--text-faint'),
    surface: v('--surface-card'),
    selected: v('--map-selected'),
    undated: v('--map-undated'),
    water: v('--map-water'),
    dug: v('--map-dug'),
    /** Opacity by hop depth. The index is the number of edges followed. */
    warrant: [n('--warrant-0'), n('--warrant-1'), n('--warrant-2'), n('--warrant-3')],
    era: (era: Era) => v(era.ink),
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

/**
 * The radius of a stack of `n` records, in metres.
 *
 * Area is proportional to the count, which is the only encoding of quantity by circle that a
 * reader estimates correctly — radius-proportional circles overstate a large stack by the square
 * of everything. The floor keeps a single record from vanishing at county zoom.
 */
export function stackRadius(n: number): number {
  return 260 + Math.sqrt(Math.max(n, 0)) * 340
}

async function collection(url: string): Promise<Collection> {
  const res = await fetch(url)
  if (!res.ok) throw new Error(`${url}: HTTP ${res.status}`)
  return (await res.json()) as Collection
}

/** What the reader has asked the map for. */
interface State {
  year: number
  grain: Grain
  generous: boolean
  hops: number
  undated: boolean
  ground: Set<string>
  selected: string | null
}

interface Hover {
  x: number
  y: number
  title: string
  rows: [string, string][]
  kind: 'ground' | 'corpus'
}

const esc = (s: string): string =>
  s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c] as string)

/** `place/lima.yml` → `Lima`, for a heading where the label is not to hand. */
const stem = (id: string): string => id.split('/').pop()?.replace(/\.yml$/, '') ?? id

export async function renderMap(container: HTMLElement, records: AtlasRecord[]): Promise<void> {
  const tooltip = container.querySelector<HTMLElement>('[data-map-tooltip]')
  const legend = container.querySelector<HTMLElement>('[data-map-legend]')
  const canvasHost = container.querySelector<HTMLElement>('[data-map-canvas]')
  const panel = container.querySelector<HTMLElement>('[data-map-panel]')
  const scrub = container.querySelector<HTMLInputElement>('[data-scrub]')
  if (!canvasHost) throw new Error('the map has no canvas host')

  const [county, subdivisions, places, cdps, districts, tracts, schools, water] = await Promise.all([
    collection(GEO.county),
    collection(GEO.subdivisions),
    collection(GEO.places),
    collection(GEO.cdps),
    collection(GEO.districts),
    collection(GEO.tracts),
    collection(GEO.schools),
    collection(GEO.water),
  ])

  const eras = spine(records)

  /**
   * Every Census shape the site holds, by key.
   *
   * A polygon whose key is not in here is not a defect in the corpus — it is a shape this
   * repository has not vendored, and the panel says so rather than dropping it. There are none
   * today; the school districts were the last twelve and arrived with layer 12.
   */
  /**
   * The summary level each file holds. Places and CDPs share one, because the Census gives them
   * one code space; school districts have their own, which is what makes this table necessary
   * rather than decorative — `3904752` is Beaverdam village *and* Upper Scioto Valley Local.
   */
  const LEVELS: [Collection, string][] = [
    [county, 'county'],
    [subdivisions, 'county-subdivision'],
    [places, 'place'],
    [cdps, 'place'],
    [schools, 'school-district'],
  ]

  /** The level is stamped onto the feature once, here, so nothing downstream has to remember it. */
  for (const [c, level] of LEVELS) {
    for (const f of c.features) f.properties.LEVEL = level
  }

  const shapes = new Map<string, { properties: FeatureProps }>()
  for (const [c] of LEVELS) {
    for (const f of c.features) shapes.set(shapeKey(f), f)
  }
  /**
   * The county's own key. Its shape is the whole frame, so it is stroked and never filled —
   * tinting every pixel of the county drowns the twelve townships and ten municipalities drawn
   * inside it, and says nothing the outline was not already saying. Same argument the labels
   * make about the county's centroid, one encoding along.
   */
  const frameKeys = new Set(county.features.map(shapeKey))

  const shapeSource = {
    type: 'FeatureCollection',
    features: LEVELS.flatMap(([c]) => c.features),
  }

  const populations = districts.features.map((f) => number(f.properties.POP100))
  const breaks = quantileBreaks(populations, 5)

  // Tracts are their own choropleth, with their own breaks. Sharing the precincts' classing
  // would put every tract in the top class — a tract holds about eight precincts' worth of
  // people, and a scale built for one grain says nothing at the other.
  const tractPopulations = tracts.features.map((f) => number(f.properties.POP100))
  const tractBreaks = quantileBreaks(tractPopulations, 5)

  /**
   * The corpus's two maps of one ground.
   *
   * `allen-county-mortgage-access-by-tract-2018-2024` states that a tract's denial rate and its
   * share of higher-priced loans correlate at 0.828 — *the tracts where a mortgage is hardest to
   * get are the tracts where it is dearest to have*. That was a coefficient standing on ten
   * printed rows until the measure published all thirty-five, and these two layers are the claim
   * drawn: switch between them and watch how little moves.
   *
   * **Each keeps its own quantile breaks, and they are not put on a shared scale.** They are
   * different quantities — a share of applications refused against a share of loans priced above
   * a federal threshold — and a common ramp would assert they are one. Quantile classing is the
   * right encoding for the claim actually made, which is about co-*ranking*; the legend prints
   * each variable's own break values so the levels stay visible under the pattern.
   */
  const ACCESS = 'measure/allen-county-mortgage-access-by-tract-2018-2024.yml'
  const accessTable = tableFor(ACCESS)
  const MEASURES = {
    denial: {
      column: 'denial_pct',
      /** The denominator the rate is computed over, and what the measure's floor applies to. */
      over: 'decisions',
      title: 'Denied applications, per cent, 2018–2024',
      row: 'Denial rate',
    },
    priced: {
      column: 'higher_priced_pct',
      over: 'priced',
      title: 'Higher-priced loans, per cent, 2018–2024',
      row: 'Higher-priced share',
    },
  } as const
  type Measure = keyof typeof MEASURES

  /**
   * The measure's own floor, applied here rather than assumed away.
   *
   * `allen-county-mortgage-access-by-tract-2018-2024` rates a tract only where its denominator
   * reaches thirty, and computes the correlation over the 34 that clear it on both counts. Tract
   * 39003013700 has 71 decisions and 28 priced originations, so its denial rate is rated and its
   * higher-priced share is not — and a share of 28 loans drawn in the darkest class would put the
   * most emphatic-looking tract on the map on the thinnest evidence in the table.
   */
  const RATED = 30

  const tractValues = new Map<Measure, Map<string, number>>(
    (Object.keys(MEASURES) as Measure[]).map((k) => {
      const values = column(accessTable, MEASURES[k].column)
      for (const [key, n] of column(accessTable, MEASURES[k].over)) {
        if (n < RATED) values.delete(key)
      }
      return [k, values]
    }),
  )
  const tractMeasureBreaks = new Map<Measure, number[]>(
    (Object.keys(MEASURES) as Measure[]).map((k) => [
      k,
      quantileBreaks([...(tractValues.get(k) as Map<string, number>).values()], 5),
    ]),
  )

  /** The corpus's containment and location claims, both ends stated. Computed once. */
  const claims = lines()

  /**
   * The nine watercourses, and the courses this site can draw for them.
   *
   * A river is not at its mouth — the ends are what the corpus states, not where the river is.
   * See `.yidam/decisions/a-river-is-not-at-its-mouth.yml`.
   */
  const watercourses = courses()
  const named = new Map<string, { properties: FeatureProps }[]>()
  for (const f of water.features) {
    const key = f.properties.NAME
    if (!key) continue
    named.set(key, [...(named.get(key) ?? []), f])
  }
  /** One end of a watercourse, as a mark with the note the corpus attached to it. */
  interface CourseEnd extends End {
    label: string
    which: 'rises' | 'ends'
  }
  const courseEnds: CourseEnd[] = []
  for (const c of watercourses as Course[]) {
    if (c.source) courseEnds.push({ ...c.source, label: c.label, which: 'rises' })
    if (c.mouth) courseEnds.push({ ...c.mouth, label: c.label, which: 'ends' })
  }

  /**
   * The view is in the URL, so a year is a thing you can link to.
   *
   * `/map?year=1885&grain=year&at=place/lima.yml` is Lima in the year the oil came in, with the
   * panel already open on it, and a reading page can point at exactly that. Without this the map
   * is an instrument you can only arrive at from the start — which is the shape the whole site is
   * trying to get out of.
   */
  const params = new URLSearchParams(globalThis.location.search)
  const clamp = (n: number) =>
    Math.min(Math.max(n, eras[0].from), eras[eras.length - 1].to)
  const asked = Number(params.get('year'))

  const state: State = {
    year: Number.isFinite(asked) && asked !== 0 ? clamp(asked) : eras[eras.length - 1].to,
    grain: params.get('grain') === 'year' ? 'year' : 'era',
    generous: params.get('generous') === '1',
    // `params.has` rather than a truthiness check: `Number(null)` is 0, which is a valid depth,
    // so the obvious spelling silently shipped every reader the stated-positions-only view.
    hops: params.has('hops') && [0, 1, 2, 3].includes(Number(params.get('hops')))
      ? Number(params.get('hops'))
      : 3,
    undated: params.get('undated') === '1',
    ground: new Set(
      (params.get('layers') ?? 'subdivisions,municipalities,water').split(',').filter(Boolean),
    ),
    selected: params.get('at'),
  }

  /** Write the view back, without adding a history entry per drag of the scrub. */
  const address = () => {
    const q = new URLSearchParams()
    q.set('year', String(state.year))
    if (state.grain !== 'era') q.set('grain', state.grain)
    if (state.hops !== 3) q.set('hops', String(state.hops))
    if (state.generous) q.set('generous', '1')
    if (state.undated) q.set('undated', '1')
    if (state.selected) q.set('at', state.selected)
    q.set('layers', [...state.ground].toSorted().join(','))
    globalThis.history.replaceState(null, '', `?${q}`)
  }

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
    tooltip.innerHTML = `<strong>${esc(hover.title)}</strong>${hover.rows
      .map(([k, v]) => `<span><em>${esc(k)}</em>${esc(v)}</span>`)
      .join('')}`
  }

  const groundHover = (rows: (f: FeatureProps) => [string, string][]) =>
    ({ object, x, y }: { object?: unknown; x: number; y: number }) => {
      const f = object as { properties: FeatureProps } | undefined
      hover = f ? { x, y, kind: 'ground', title: f.properties.NAME, rows: rows(f.properties) } : null
      setTooltip()
    }

  // -------------------------------------------------------------------------
  // The scene
  // -------------------------------------------------------------------------

  const layers = () => {
    const p = palette()
    const look = view(state.year, state.grain, eras, {
      generous: state.generous,
      hops: state.hops,
    })
    const era = eraAt(state.year, eras)
    const ink = p.era(era)

    // The ground is a 2020 statement, so it fades with the distance travelled from 2020.
    const fidelity = groundFidelity(state.year)
    const ghost = (alpha: number) => Math.round(alpha * fidelity)

    const present = drawn(records, look)
    const dateless = state.undated ? undrawn(records, look) : []
    const stacks = anchored(present)
    const keyed = shaded(present)
    const keyedUndated = shaded(dateless)

    // A shape may be claimed by more than one record; the map draws each shape once.
    const litKeys = new Set(keyed.map((s) => s.key).filter((k) => shapes.has(k)))

    // One fill encoding at a time. A choropleth already owns every pixel of the county, and the
    // era tint laid over it is two magnitudes in one hue — worst of all at 1940, whose era ink and
    // the sequential ramp are both blue. When a choropleth is drawn the corpus keeps the outline
    // and gives up the fill.
    const choropleth = CHOROPLETHS.some((k) => state.ground.has(k))
    const datelessKeys = new Set(
      keyedUndated.map((s) => s.key).filter((k) => shapes.has(k) && !litKeys.has(k)),
    )

    // The county's own node is excluded from labelling: its extent is the whole frame, so a
    // label at its centroid names Lima's neighbourhood rather than the county, and collides
    // with Lima doing it.
    // Places only, and only the largest stacks. `stacks` is already sorted by size, and past
    // about nine names the Lima corner overprints into a smear no zoom separates — deck.gl's
    // text layer does no collision avoidance, so the thinning has to happen here.
    const labelled = stacks
      .filter((s) => s.node.startsWith('place/') && s.node !== 'place/allen-county.yml')
      .slice(0, 8)

    /** One ring of a stack: everything reachable in `depth` edges or fewer. */
    const ring = (depth: number) =>
      new ScatterplotLayer<Anchored>({
        id: `stack-${depth}`,
        data: stacks.filter((s) => s.records.some((r) => (r.hops ?? 0) <= depth)),
        getPosition: (d) => [d.lon, d.lat],
        getRadius: (d) => stackRadius(d.records.filter((r) => (r.hops ?? 0) <= depth).length),
        radiusMinPixels: 3,
        radiusMaxPixels: 90,
        getFillColor: rgb(ink, Math.round(255 * 0.3 * p.warrant[depth])),
        stroked: false,
        // Not pickable. The hit target is the anchor dot on top, which is also what a click
        // selects — two overlapping pick targets for one mark is two answers to one gesture.
        pickable: false,
      })

    // The water is not a 2020 statement, so it fades from the year it was fetched instead.
    const waterGhost = (alpha: number) =>
      Math.round(alpha * groundFidelity(state.year, WATER_VINTAGE))

    return [
      // ---- the ground -------------------------------------------------------
      state.ground.has('tracts') &&
        new GeoJsonLayer({
          id: 'tracts',
          data: tracts as unknown as object,
          filled: true,
          stroked: true,
          getFillColor: (f: { properties: FeatureProps }) =>
            rgb(p.ramp[classOf(number(f.properties.POP100), tractBreaks)], ghost(205)),
          getLineColor: rgb(p.surface, ghost(150)),
          getLineWidth: 14,
          lineWidthMinPixels: 0.5,
          pickable: true,
          onHover: groundHover((f) => [
            ['Population', number(f.POP100).toLocaleString('en-US')],
            ['Housing units', number(f.HU100).toLocaleString('en-US')],
            ['Census tract', '2020 Census geography'],
          ]),
        }),

      // The corpus's two maps, drawn over the same thirty-five tracts. See `MEASURES`.
      ...(Object.keys(MEASURES) as Measure[]).map(
        (m) =>
          state.ground.has(m) &&
          new GeoJsonLayer({
            id: `tract-${m}`,
            data: tracts as unknown as object,
            filled: true,
            stroked: true,
            getFillColor: (f: { properties: FeatureProps }) => {
              const v = tractValues.get(m)?.get(f.properties.GEOID)
              // A tract the measure has no figure for is left as ground rather than classed.
              // Drawing it in the lightest step would say it had the lowest rate.
              if (v === undefined) return [0, 0, 0, 0]
              return rgb(p.ramp[classOf(v, tractMeasureBreaks.get(m) as number[])], ghost(205))
            },
            getLineColor: rgb(p.surface, ghost(150)),
            getLineWidth: 14,
            lineWidthMinPixels: 0.5,
            updateTriggers: { getFillColor: [state.year] },
            pickable: true,
            onHover: groundHover((f) => {
              const v = tractValues.get(m)?.get(f.GEOID)
              return [
                [
                  MEASURES[m].row,
                  v === undefined
                    ? `under ${RATED} ${MEASURES[m].over} — not rated`
                    : `${v.toFixed(1)} per cent`,
                ],
                ['Census tract', '2020 Census geography'],
              ]
            }),
          }),
      ),

      state.ground.has('population') &&
        new GeoJsonLayer({
          id: 'voting-districts',
          data: districts as unknown as object,
          filled: true,
          stroked: true,
          getFillColor: (f: { properties: FeatureProps }) =>
            rgb(p.ramp[classOf(number(f.properties.POP100), breaks)], ghost(205)),
          getLineColor: rgb(p.surface, ghost(150)),
          getLineWidth: 12,
          lineWidthMinPixels: 0.5,
          pickable: true,
          onHover: groundHover((f) => [
            ['Population', number(f.POP100).toLocaleString('en-US')],
            ['Housing units', number(f.HU100).toLocaleString('en-US')],
            ['Voting district', '2020 Census geography'],
          ]),
        }),

      state.ground.has('subdivisions') &&
        new GeoJsonLayer({
          id: 'subdivisions',
          data: subdivisions as unknown as object,
          filled: false,
          stroked: true,
          getLineColor: rgb(p.muted, ghost(190)),
          getLineWidth: 30,
          lineWidthMinPixels: 1,
          pickable: true,
          onHover: groundHover(() => [['Civil subdivision', '2020 Census geography']]),
        }),

      // The school districts, drawn whole and allowed to leave the frame.
      //
      // Twelve of the seventeen have territory in another county, so a good part of this layer
      // is outside the county outline — which is the point. A district clipped at the line would
      // draw a shape that does not exist and let the reader believe the county contains it.
      // Stroke only and no fill: they tile the county, so filling them is a second choropleth
      // over whatever is already there. See `a-district-is-not-cut-at-the-county-line`.
      state.ground.has('schools') &&
        new GeoJsonLayer({
          id: 'school-districts',
          data: schools as unknown as object,
          filled: false,
          stroked: true,
          // Lighter than the township lines and much lighter than the county outline, which is
          // drawn after this and has to read first: a district running off the frame only means
          // *past the county* if the reader can see where the county is.
          getLineColor: rgb(p.faint, ghost(150)),
          getLineWidth: 45,
          lineWidthMinPixels: 1,
          pickable: true,
          onHover: groundHover((f) => [
            ['Population', number(f.POP100).toLocaleString('en-US')],
            ['Unified school district', '2020 Census geography'],
          ]),
        }),

      state.ground.has('municipalities') &&
        new GeoJsonLayer({
          id: 'municipalities',
          data: {
            type: 'FeatureCollection',
            features: [...places.features, ...cdps.features],
          } as unknown as object,
          filled: true,
          stroked: true,
          getFillColor: rgb(p.ink, ghost(22)),
          getLineColor: rgb(p.ink, ghost(140)),
          getLineWidth: 22,
          lineWidthMinPixels: 1,
          pickable: true,
          onHover: groundHover(() => [['Municipality or CDP', '2020 Census geography']]),
        }),

      // ---- the water --------------------------------------------------------
      //
      // Two feature classes, drawn apart. H3010 is a stream or river; H3020 is a canal, ditch or
      // aqueduct — and in this county that difference is the Great Black Swamp becoming cropland.
      //
      // **Only the dug lines fade.** Everything else on this map is a claim about a year and gets
      // ghosted as the reader travels away from the year it was surveyed. A creek is not: the
      // Ottawa ran where it runs before any boundary on this plate existed, and fading it at 1832
      // would say the opposite. A ditch is a thing somebody dug, most of them after 1859, and
      // drawing one at full strength in 1832 is an anachronism — so the ditches carry the ghost
      // and the streams do not.
      state.ground.has('water') &&
        new GeoJsonLayer({
          id: 'water',
          data: water as unknown as object,
          filled: false,
          stroked: true,
          getLineColor: (f: { properties: FeatureProps }) =>
            f.properties.MTFCC === 'H3020'
              ? rgb(p.dug, waterGhost(235))
              : rgb(p.water, 190),
          getLineWidth: (f: { properties: FeatureProps }) =>
            f.properties.MTFCC === 'H3020' ? 16 : 26,
          lineWidthMinPixels: 0.9,
          lineWidthMaxPixels: 3.5,
          updateTriggers: { getLineColor: [state.year] },
          pickable: true,
          onHover: ({ object, x, y }) => {
            const f = object as { properties: FeatureProps } | undefined
            hover = f
              ? {
                  x,
                  y,
                  kind: 'ground',
                  title: f.properties.NAME ?? 'Unnamed watercourse',
                  rows: [
                    [
                      f.properties.MTFCC === 'H3020' ? 'Dug' : 'Stream',
                      f.properties.MTFCC === 'H3020' ? 'canal, ditch or aqueduct' : 'natural channel',
                    ],
                    ['TIGER hydrography', 'no vintage — current'],
                  ],
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
        getLineColor: rgb(p.ink, ghost(255)),
        getLineWidth: 60,
        lineWidthMinPixels: 1.75,
      }),

      // ---- the corpus, as shapes -------------------------------------------
      //
      // Only a node that states its own Census key. A tract reaching the county government in
      // one edge would otherwise be shaded as the whole county, which is not where the tract is.
      new GeoJsonLayer({
        id: 'corpus-shapes',
        data: shapeSource as unknown as object,
        // deck.gl keeps one dataset and decides per feature, so a shape leaving the window is a
        // colour change rather than a reload of the geometry.
        getFillColor: (f: { properties: FeatureProps }) =>
          frameKeys.has(shapeKey(f)) || choropleth
            ? [0, 0, 0, 0]
            : litKeys.has(shapeKey(f))
              ? rgb(ink, 34)
              : datelessKeys.has(shapeKey(f))
                ? rgb(p.undated, 46)
                : [0, 0, 0, 0],
        getLineColor: (f: { properties: FeatureProps }) =>
          litKeys.has(shapeKey(f))
            ? rgb(ink, 240)
            : datelessKeys.has(shapeKey(f))
              ? rgb(p.undated, 190)
              : [0, 0, 0, 0],
        getLineWidth: 44,
        lineWidthMinPixels: 1.25,
        filled: true,
        stroked: true,
        pickable: true,
        updateTriggers: {
          getFillColor: [state.year, state.grain, state.generous, state.hops, state.undated, choropleth],
          getLineColor: [state.year, state.grain, state.generous, state.hops, state.undated],
        },
        onHover: groundHover((f) => [
          ['In force', litKeys.has(`${f.LEVEL}:${f.GEOID}`) ? 'in this window' : 'the corpus gives no date'],
          ['Boundary', '2020 Census geography'],
        ]),
      }),

      // ---- the corpus, as stacks -------------------------------------------
      ring(3),
      ring(2),
      ring(1),
      ring(0),

      new ScatterplotLayer<Anchored>({
        id: 'anchors',
        data: stacks,
        getPosition: (d) => [d.lon, d.lat],
        // The selected mark grows as well as changing ink. Rubric and the 1860 era ink are both
        // reds, so on that one tile a hue change alone says nothing.
        getRadius: (d) => (d.node === state.selected ? 360 : 190),
        radiusMinPixels: 3.5,
        radiusMaxPixels: 13,
        // The strongest ink available, not the era's. The halo under it and the shapes around
        // it already carry the era; a mark in the same hue as the wash it sits on is a mark
        // nobody can find, which is what the first draft of this layer was.
        getFillColor: (d) => (d.node === state.selected ? rgb(p.selected, 255) : rgb(p.ink, 250)),
        // A 2px surface ring, not a border: overlapping marks stay separable.
        stroked: true,
        getLineColor: rgb(p.surface, 255),
        getLineWidth: (d) => (d.node === state.selected ? 130 : 70),
        lineWidthMinPixels: 2,
        pickable: true,
        updateTriggers: {
          getFillColor: [state.selected, p.ink],
          getRadius: [state.selected],
          getLineWidth: [state.selected],
        },
        onHover: ({ object, x, y }) => {
          const a = object as Anchored | undefined
          hover = a
            ? {
                x,
                y,
                kind: 'corpus',
                title: anchorLabel(a),
                rows: [
                  ['Records here', String(a.records.length)],
                  ['Stated position', a.records.some((r) => r.hops === 0) ? 'yes' : 'all derived'],
                  ['Click', 'to list them'],
                ],
              }
            : null
          setTooltip()
        },
        onClick: ({ object }) => {
          const a = object as Anchored | undefined
          state.selected = a && a.node !== state.selected ? a.node : null
          refresh()
          renderPanel()
        },
      }),

      state.undated &&
        new ScatterplotLayer<Anchored>({
          id: 'undated-anchors',
          data: anchored(dateless),
          getPosition: (d) => [d.lon, d.lat],
          getRadius: (d) => stackRadius(d.records.length),
          radiusMinPixels: 3,
          radiusMaxPixels: 60,
          getFillColor: rgb(p.undated, 44),
          stroked: true,
          getLineColor: rgb(p.undated, 190),
          lineWidthMinPixels: 1,
          pickable: true,
          onClick: ({ object }) => {
            const a = object as Anchored | undefined
            state.selected = a && a.node !== state.selected ? a.node : null
            refresh()
            renderPanel()
          },
        }),

      // ---- the corpus's own claims, as lines ---------------------------------
      //
      // Twelve of them, and every one runs between two positions the corpus states rather than
      // two it was routed to. A line between derived marks would draw a relationship between two
      // guesses, and nothing on this map can say a line is three inferences long.
      //
      // This is the corpus-correctness case the map exists for. The Lima refinery's line does not
      // end in Lima; it crosses into Shawnee Township, which is where the refinery is and is not
      // what its address said for eleven phases.
      state.ground.has('claims') &&
        new ArcLayer<Line>({
          id: 'claims',
          data: claims,
          getSourcePosition: (d) => [d.from.lon, d.from.lat],
          getTargetPosition: (d) => [d.to.lon, d.to.lat],
          getSourceColor: rgb(p.selected, 210),
          getTargetColor: rgb(p.selected, 90),
          getWidth: 1.6,
          // Low, so it reads as a claim joining two marks rather than as a flight path.
          getHeight: 0.22,
          greatCircle: false,
          pickable: true,
          onHover: ({ object, x, y }) => {
            const d = object as Line | undefined
            hover = d
              ? {
                  x,
                  y,
                  kind: 'corpus',
                  title: d.fromLabel,
                  rows: [
                    [d.relationship, d.toLabel],
                    ['Claim tag', d.tier],
                    ['Both ends', 'stated, not derived'],
                  ],
                }
              : null
            setTooltip()
          },
        }),

      // ---- where the corpus says a watercourse begins and ends ---------------
      //
      // Not a placement. These are the ends of a line, and most of them are in another county —
      // which is exactly why `crates/placement` does not read them and why the note travels with
      // the mark.
      state.ground.has('water') &&
        new ScatterplotLayer<CourseEnd>({
          id: 'course-ends',
          data: courseEnds,
          getPosition: (d) => [d.lon, d.lat],
          getRadius: 170,
          radiusMinPixels: 3,
          radiusMaxPixels: 7,
          filled: false,
          stroked: true,
          getLineColor: rgb(p.water, 235),
          lineWidthMinPixels: 1.5,
          pickable: true,
          onHover: ({ object, x, y }) => {
            const d = object as CourseEnd | undefined
            hover = d
              ? {
                  x,
                  y,
                  kind: 'corpus',
                  title: d.label,
                  rows: [
                    ['The corpus says it', d.which === 'rises' ? 'rises here' : 'ends here'],
                    ...(d.note ? ([['', d.note]] as [string, string][]) : []),
                    ['Claim tag', d.tier ?? 'untagged'],
                  ],
                }
              : null
            setTooltip()
          },
        }),

      new TextLayer<Anchored>({
        id: 'anchor-labels',
        // Places only. Five sites and the courthouse sit within two miles of each other in
        // Lima, and labelling every anchor overprints that corner into an unreadable smear.
        data: labelled,
        getPosition: (d) => [d.lon, d.lat],
        getText: (d) => anchorLabel(d),
        getSize: 12,
        sizeUnits: 'pixels',
        getColor: rgb(p.ink, 235),
        getPixelOffset: [0, -18],
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

  /** What the corpus calls an anchor, taken from its own record where it has one. */
  const labels = new Map(records.map((r) => [r.node, r.label]))
  const anchorLabel = (a: Anchored): string => labels.get(a.node) ?? stem(a.node)

  // -------------------------------------------------------------------------
  // The readouts
  // -------------------------------------------------------------------------

  const renderPanel = () => {
    if (!panel) return
    const look = view(state.year, state.grain, eras, {
      generous: state.generous,
      hops: state.hops,
    })
    const pool = [...drawn(records, look), ...(state.undated ? undrawn(records, look) : [])]
    const here = anchored(pool).find((a) => a.node === state.selected)

    if (!here) {
      panel.innerHTML =
        '<p class="empty">Select a mark to see what the corpus puts there, and by what route it got there.</p>'
      return
    }

    const rows = here.records
      .toSorted((a, b) => (a.hops ?? 0) - (b.hops ?? 0) || a.label.localeCompare(b.label))
      .map((r) => {
        const route = r.anchors
          .find((an) => an.node === here.node)
          ?.via.map((s) => s.relationship)
          .join(' → ')
        const when =
          r.from === null
            ? 'undated'
            : r.to === null
              ? `${r.from}–`
              : r.from === r.to
                ? String(r.from)
                : `${r.from}–${r.to}`
        return `<li>
          <a href="${entryPath(r.node)}">${esc(r.label)}</a>
          <span class="meta">
            <em>${esc(r.class)}</em>
            <em>${esc(when)}</em>
            <em data-hops="${r.hops ?? ''}">${
              r.hops === 0 ? 'stated here' : `${route ?? 'derived'}`
            }</em>
          </span>
        </li>`
      })

    panel.innerHTML = `<h3>${esc(anchorLabel(here))}</h3>
      <p class="count">${here.records.length} record${here.records.length === 1 ? '' : 's'} placed here${
        state.grain === 'era' ? ` in ${esc(eraAt(state.year, eras).label)}` : ` in ${state.year}`
      }. A route means the corpus did not state this position — it was reached across those edges.</p>
      <ol>${rows.join('')}</ol>`
  }

  const readouts = () => {
    const look = view(state.year, state.grain, eras, {
      generous: state.generous,
      hops: state.hops,
    })
    const era = eraAt(state.year, eras)
    const present = drawn(records, look)

    const write = (key: string, value: string) => {
      for (const el of container.querySelectorAll<HTMLElement>(`[data-readout="${key}"]`)) {
        el.textContent = value
      }
    }
    write('year', String(state.year))
    write('era', era.label)
    write('window', state.grain === 'era' ? `${era.from}–${era.to}` : String(state.year))
    write('present', present.length.toLocaleString('en-US'))
    write('anchors', String(anchored(present).length))
    write('vintage', String(Math.abs(state.year - 2020)))

    // The era's own ink, so the readout is coloured by where the reader is standing.
    container.style.setProperty('--era-current', `var(${era.ink})`)
    for (const tile of container.querySelectorAll<HTMLElement>('[data-era]')) {
      tile.setAttribute('aria-pressed', String(tile.dataset.era === era.key))
    }
    for (const bar of container.querySelectorAll<HTMLElement>('[data-bar-from]')) {
      const from = Number(bar.dataset.barFrom)
      const to = Number(bar.dataset.barTo)
      bar.toggleAttribute('data-in-window', from <= look.to && look.from <= to)
    }
    if (scrub) {
      scrub.min = String(era.from)
      scrub.max = String(era.to)
      scrub.value = String(state.year)
    }
  }

  // -------------------------------------------------------------------------
  // Deck
  // -------------------------------------------------------------------------

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

  const update = () => {
    address()
    showScale()
    readouts()
    refresh()
    renderPanel()
  }

  /**
   * The choropleth legend, for whichever grain is drawn.
   *
   * A continuous colour encoding without a legend is unreadable, and a legend for a layer nobody
   * asked for is furniture — so it appears with the checkbox it explains and prints that
   * checkbox's own breaks. The two grains carry separate breaks on purpose: a tract holds about
   * eight precincts' worth of people, and a scale built for one grain puts every feature of the
   * other in its top class.
   */
  const scale = container.querySelector<HTMLElement>('[data-map-scale]')
  const scaleTitle = container.querySelector<HTMLElement>('[data-map-scale-title]')

  const showScale = () => {
    const shown = (Object.keys(MEASURES) as Measure[]).find((m) => state.ground.has(m))
    const grain = shown
      ? ({
          label: MEASURES[shown].title,
          breaks: tractMeasureBreaks.get(shown) as number[],
          values: [...(tractValues.get(shown) as Map<string, number>).values()],
        } as const)
      : state.ground.has('tracts')
        ? ({ label: 'People per census tract', breaks: tractBreaks, values: tractPopulations } as const)
        : state.ground.has('population')
          ? ({ label: 'People per voting district', breaks, values: populations } as const)
          : null

    if (scale) scale.hidden = grain === null
    if (!grain || !legend) return
    if (scaleTitle) scaleTitle.textContent = grain.label

    const p = palette()
    // The floor is the smallest value present, not zero. Quantile classes are ranges of the data,
    // and a first class printed as `0–16` on a denial rate names a rate no tract in this county
    // has. The two population layers gain the same correction: the lightest precinct is not empty.
    const edges = [Math.min(...grain.values), ...grain.breaks, Math.max(...grain.values)]
    legend.innerHTML = p.ramp
      .map(
        (c, i) =>
          `<span class="swatch"><i style="background:${c}"></i>${Math.round(
            edges[i],
          ).toLocaleString('en-US')}–${Math.round(edges[i + 1]).toLocaleString('en-US')}</span>`,
      )
      .join('')
  }

  // -------------------------------------------------------------------------
  // The controls
  //
  // Every one of them is real markup rendered by the page, so the spine, the ribbon and the
  // period names are in the document before this script runs and are readable without it.
  // -------------------------------------------------------------------------

  for (const tile of container.querySelectorAll<HTMLElement>('[data-era]')) {
    tile.addEventListener('click', () => {
      const era = eras.find((e) => e.key === tile.dataset.era)
      if (!era) return
      // Land at the tile's own end rather than its start: the closing year of an era is where
      // the most of it is standing, and arriving at the first year of "Peak" shows 1940.
      state.year = Math.min(era.to, eras[eras.length - 1].to)
      update()
    })
  }

  for (const chip of container.querySelectorAll<HTMLElement>('[data-period-from]')) {
    chip.addEventListener('click', () => {
      state.year = Number(chip.dataset.periodFrom)
      update()
    })
  }

  scrub?.addEventListener('input', () => {
    state.year = Number(scrub.value)
    update()
  })

  for (const input of container.querySelectorAll<HTMLInputElement>('[data-grain]')) {
    input.addEventListener('change', () => {
      if (input.checked) state.grain = input.value as Grain
      update()
    })
  }

  const warrant = container.querySelector<HTMLSelectElement>('[data-warrant]')
  warrant?.addEventListener('change', () => {
    state.hops = Number(warrant.value)
    update()
  })

  for (const input of container.querySelectorAll<HTMLInputElement>('[data-toggle]')) {
    input.addEventListener('change', () => {
      if (input.dataset.toggle === 'generous') state.generous = input.checked
      if (input.dataset.toggle === 'undated') state.undated = input.checked
      update()
    })
  }

  for (const input of container.querySelectorAll<HTMLInputElement>('[data-layer]')) {
    input.addEventListener('change', () => {
      const key = input.dataset.layer
      if (!key) return
      if (input.checked) state.ground.add(key)
      else state.ground.delete(key)

      // One choropleth at a time — see `CHOROPLETHS`. Turning one on turns the others off, in
      // the state and on the control together, so the widgets never say something the map does
      // not.
      if (input.checked && (CHOROPLETHS as readonly string[]).includes(key)) {
        for (const other of CHOROPLETHS) {
          if (other === key) continue
          state.ground.delete(other)
          const twin = container.querySelector<HTMLInputElement>(`[data-layer="${other}"]`)
          if (twin) twin.checked = false
        }
      }

      address()
      showScale()
      refresh()
    })
  }

  // The controls are rendered by the page with the defaults in them, so a view that arrived in
  // the URL has to be written back onto them or the reader sees one thing and the widgets say
  // another.
  for (const input of container.querySelectorAll<HTMLInputElement>('[data-grain]')) {
    input.checked = input.value === state.grain
  }
  if (warrant) warrant.value = String(state.hops)
  for (const input of container.querySelectorAll<HTMLInputElement>('[data-layer]')) {
    input.checked = state.ground.has(input.dataset.layer ?? '')
  }
  for (const input of container.querySelectorAll<HTMLInputElement>('[data-toggle]')) {
    if (input.dataset.toggle === 'generous') input.checked = state.generous
    if (input.dataset.toggle === 'undated') input.checked = state.undated
  }

  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', refresh)
  new MutationObserver(refresh).observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['data-theme'],
  })

  update()
}

/** Exported for the page, which prints the spine and the ribbon at build time. */
export type { View }
