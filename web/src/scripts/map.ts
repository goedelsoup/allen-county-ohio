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
import type { FeatureCollection, Geometry } from 'geojson'
// `PathLayer` and nothing from `@deck.gl/geo-layers`. `TripsLayer` lives there, its whole
// purpose is to animate a position along a path over time, and that is exactly the tween
// `an-animation-asserts-continuity` refuses — it would draw the storm at a position, at a
// moment, that no source recorded. `hops-counts-edges-not-coordinates` records the refusal and
// the reason the package stays out of `package.json`: a dependency that puts the wrong thing one
// import away is an invitation.
import {
  ArcLayer,
  GeoJsonLayer,
  PathLayer,
  ScatterplotLayer,
  TextLayer,
} from '@deck.gl/layers'
import {
  anchored,
  drawn,
  eraAt,
  groundFidelity,
  shaded,
  spine,
  tracked,
  undrawn,
  view,
  WATER_VINTAGE,
  type Anchored,
  type Era,
  type Grain,
  type Tracked,
  type View,
} from '../lib/eras'
import { bearing, dashes, straightMiles } from '../lib/track'
import {
  START,
  STILL,
  advance,
  route as playbackRoute,
  seek,
  type Cadence,
  type Stop,
} from '../lib/motion'
import { lines, type Line } from '../lib/edges'
import { courses, type Course, type End } from '../lib/water'
import { entryPath } from '../lib/entry'
import { parseMapState, serializeMapState, type MapCamera } from '../lib/map-state'
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
  MTFCC?: string
}

type Collection = FeatureCollection<Geometry, FeatureProps>

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
  arealWater: '/geo/areal-water.geojson',
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
 * The playback cadence, resolved from the token layer.
 *
 * **This is also how `prefers-reduced-motion` is read.** `tokens/site.css` redeclares both
 * tokens inside the media query, so a reader who has asked for no motion resolves a single long
 * uniform hold here without this file testing for it — one source for the preference rather
 * than a token and a script that can disagree.
 *
 * `STILL` is the fallback rather than the fast cadence, on the same argument: if the sheet has
 * not resolved, the conservative answer is the one that moves least.
 */
function cadence(): Cadence {
  const s = getComputedStyle(document.documentElement)
  const ms = (name: string, fallback: number): number => {
    const raw = s.getPropertyValue(name).trim()
    const n = Number.parseFloat(raw)
    if (!Number.isFinite(n) || n <= 0) return fallback
    return raw.endsWith('ms') ? n : raw.endsWith('s') ? n * 1000 : n
  }
  return { step: ms('--replay-step', STILL.step), hold: ms('--replay-hold', STILL.hold) }
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

/**
 * One drawn piece of a track's connector.
 *
 * The pieces are computed rather than styled, because the gap between them is the claim: nothing
 * about the ground between two recorded positions is on the record. See `dashes` in `lib/track`.
 */
interface TrackRun {
  track: Tracked
  path: [number, number][]
}

/** One position a source recorded for a moving happening, and what it recorded it as. */
interface TrackEnd {
  track: Tracked
  lat: number
  lon: number
  role: 'began' | 'passed' | 'ended'
}

export async function renderMap(container: HTMLElement, records: AtlasRecord[]): Promise<void> {
  const tooltip = container.querySelector<HTMLElement>('[data-map-tooltip]')
  const legend = container.querySelector<HTMLElement>('[data-map-legend]')
  const canvasHost = container.querySelector<HTMLDivElement>('[data-map-canvas]')
  const panel = container.querySelector<HTMLElement>('[data-map-panel]')
  const scrub = container.querySelector<HTMLInputElement>('[data-scrub]')
  if (!canvasHost) throw new Error('the map has no canvas host')

  // Search is ready before geometry finishes loading. Carry an early choice into the scene.
  let pendingSelection: string | null = null
  let handleSelection: ((id: string) => void) | undefined
  container.addEventListener('atlas:select', (event) => {
    const id = (event as CustomEvent<{ node: string }>).detail?.node
    if (typeof id !== 'string') return
    if (handleSelection) handleSelection(id)
    else pendingSelection = id
  })

  const [county, subdivisions, places, cdps, districts, tracts, schools, water, arealWater] =
    await Promise.all([
      collection(GEO.county),
      collection(GEO.subdivisions),
      collection(GEO.places),
      collection(GEO.cdps),
      collection(GEO.districts),
      collection(GEO.tracts),
      collection(GEO.schools),
      collection(GEO.water),
      collection(GEO.arealWater),
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

  const shapes = new Map<string, Collection['features'][number]>()
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

  const shapeSource: Collection = {
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
  const extent = { from: eras[0].from, to: eras[eras.length - 1].to }
  let state = parseMapState(globalThis.location.search, extent)
  const byNode = new Map(records.map((record) => [record.node, record]))
  const readingData = document.querySelector('[data-atlas-reading]')?.textContent
  const reading: Record<string, { slug: string; title: string }[]> = readingData
    ? JSON.parse(readingData)
    : {}

  /** Continuous gestures replace their frame; deliberate choices add a navigable stop. */
  const address = (mode: 'push' | 'replace' = 'replace') => {
    const search = `?${serializeMapState(state)}`
    if (mode === 'push' && search === globalThis.location.search) return
    globalThis.history[mode === 'push' ? 'pushState' : 'replaceState'](null, '', search)
  }

  const setPanel = (mode: 'closed' | 'preview' | 'expanded') => {
    container.dataset.panelState = mode
    const shell = container.querySelector<HTMLElement>('[data-map-panel-shell]')
    if (shell) shell.inert = mode === 'closed'
    for (const trigger of container.querySelectorAll('[data-panel-action="preview"]')) {
      trigger.setAttribute('aria-expanded', String(mode !== 'closed'))
    }
  }

  const selectedAnchor = (anchor: Anchored) =>
    anchor.node === state.selected || anchor.records.some((record) => record.node === state.selected)

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

    // The two storms. Each is drawn as the positions a source recorded and the broken line
    // between them, and the two are deliberately different objects: the marks are the claim and
    // the connector is this site's reading of two points and a stated length. See `dashes`.
    const tracks = tracked(present)
    const runs: TrackRun[] = tracks.flatMap((track) =>
      dashes(track.points).map((piece) => ({
        track,
        path: piece.map((q) => [q.lon, q.lat] as [number, number]),
      })),
    )
    const trackEnds: TrackEnd[] = tracks.flatMap((track) =>
      track.points.map((at, i) => ({
        track,
        lat: at.lat,
        lon: at.lon,
        role: i === 0 ? 'began' : i === track.points.length - 1 ? 'ended' : 'passed',
      })),
    )
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
          data: tracts,
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
            data: tracts,
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
          data: districts,
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
          data: subdivisions,
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
          data: schools,
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
          },
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
      // **The water drawn as area, under the water drawn as line.** Same toggle, same two
      // classes, same colours — because it is the same water, and the only thing that differs is
      // how wide the Bureau found it. Filled rather than stroked: a polygon here is a channel with
      // two banks, and outlining it would draw the banks as though they were two streams.
      //
      // This layer is why the Ottawa River is on the map at all. It is named nowhere in the linear
      // file and four times in this one; see `UNDRAWN` in `lib/water.ts`.
      state.ground.has('water') &&
        new GeoJsonLayer({
          id: 'areal-water',
          data: arealWater,
          filled: true,
          stroked: false,
          getFillColor: (f: { properties: FeatureProps }) =>
            f.properties.MTFCC === 'H3020' ? rgb(p.dug, waterGhost(200)) : rgb(p.water, 150),
          updateTriggers: { getFillColor: [state.year] },
          pickable: true,
          onHover: ({ object, x, y }) => {
            const f = object as { properties: FeatureProps } | undefined
            hover = f
              ? {
                  x,
                  y,
                  kind: 'ground',
                  title: f.properties.NAME ?? 'Unnamed water',
                  rows: [
                    [
                      f.properties.MTFCC === 'H3020' ? 'Dug' : 'Stream',
                      'wide enough that TIGER draws two banks',
                    ],
                    ['TIGER hydrography', 'areal — no vintage, current'],
                  ],
                }
              : null
            setTooltip()
          },
        }),

      state.ground.has('water') &&
        new GeoJsonLayer({
          id: 'water',
          data: water,
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
        data: county,
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
        data: shapeSource,
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
        getRadius: (d) => (selectedAnchor(d) ? 360 : 190),
        radiusMinPixels: 3.5,
        radiusMaxPixels: 13,
        // The strongest ink available, not the era's. The halo under it and the shapes around
        // it already carry the era; a mark in the same hue as the wash it sits on is a mark
        // nobody can find, which is what the first draft of this layer was.
        getFillColor: (d) => (selectedAnchor(d) ? rgb(p.selected, 255) : rgb(p.ink, 250)),
        // A 2px surface ring, not a border: overlapping marks stay separable.
        stroked: true,
        getLineColor: rgb(p.surface, 255),
        getLineWidth: (d) => (selectedAnchor(d) ? 130 : 70),
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
          selectRecord(a && a.node !== state.selected ? a.node : null)
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
            selectRecord(a && a.node !== state.selected ? a.node : null)
          },
        }),

      // ---- the corpus, as tracks ---------------------------------------------
      //
      // Two events state the ground they crossed rather than a spot they stood on, and before
      // this they were listed beside the map as things placed no finer than the county while the
      // corpus held four surveyed coordinates for them.
      //
      // **Nothing sweeps along the connector, and nothing may.** A mark travelling between the
      // endpoints is the tween `an-animation-asserts-continuity` refuses: it would draw the storm
      // at a position, at a moment, that no source recorded, in the one encoding that carries no
      // tag and affords no tooltip. The line is broken for the same reason it is not animated.
      runs.length > 0 &&
        new PathLayer<TrackRun>({
          id: 'tracks',
          data: runs,
          getPath: (d) => d.path,
          // The era's ink, like every other placed thing, so travelling recolours the storm with
          // the rest of the map. Thin and pale against the endpoints, which are solid: the
          // difference in weight is the difference between a reading and an observation.
          getColor: (d) =>
            d.track.node === state.selected ? rgb(p.selected, 210) : rgb(ink, 150),
          getWidth: 2,
          widthUnits: 'pixels',
          widthMinPixels: 1.5,
          capRounded: true,
          jointRounded: true,
          pickable: true,
          updateTriggers: { getColor: [state.selected, ink] },
          onHover: ({ object, x, y }) => {
            const d = object as TrackRun | undefined
            hover = d
              ? {
                  x,
                  y,
                  kind: 'corpus',
                  title: labels.get(d.track.node) ?? stem(d.track.node),
                  rows: [
                    ['Positions recorded', String(d.track.points.length)],
                    ['Between them', `${straightMiles(d.track.points).toFixed(1)} miles straight`],
                    ['This line', 'a reading, not a record'],
                  ],
                }
              : null
            setTooltip()
          },
          onClick: ({ object }) => {
            const d = object as TrackRun | undefined
            selectRecord(d && d.track.node !== state.selected ? d.track.node : null)
          },
        }),

      trackEnds.length > 0 &&
        new ScatterplotLayer<TrackEnd>({
          id: 'track-ends',
          data: trackEnds,
          getPosition: (d) => [d.lon, d.lat],
          getRadius: (d) => (d.track.node === state.selected ? 300 : 170),
          radiusMinPixels: 3,
          radiusMaxPixels: 11,
          // Filled at the end and hollow at the beginning, which is the encoding
          // `EventTrack.astro` uses on the entry page for the same two storms. A reader who has
          // met one figure should not have to learn the other.
          filled: true,
          getFillColor: (d) =>
            d.role === 'ended'
              ? d.track.node === state.selected
                ? rgb(p.selected, 255)
                : rgb(ink, 250)
              : rgb(p.surface, 255),
          stroked: true,
          getLineColor: (d) =>
            d.track.node === state.selected ? rgb(p.selected, 255) : rgb(ink, 250),
          getLineWidth: 90,
          lineWidthMinPixels: 2,
          pickable: true,
          updateTriggers: {
            getFillColor: [state.selected, ink],
            getLineColor: [state.selected, ink],
            getRadius: [state.selected],
          },
          onHover: ({ object, x, y }) => {
            const d = object as TrackEnd | undefined
            hover = d
              ? {
                  x,
                  y,
                  kind: 'corpus',
                  title: labels.get(d.track.node) ?? stem(d.track.node),
                  rows: [
                    ['The source records it', `${d.role} here`],
                    ['Position', bearing({ lat: d.lat, lon: d.lon })],
                    ['Click', 'to read the track'],
                  ],
                }
              : null
            setTooltip()
          },
          onClick: ({ object }) => {
            const d = object as TrackEnd | undefined
            selectRecord(d && d.track.node !== state.selected ? d.track.node : null)
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

  const empty =
    '<p class="empty">Select a mark to see what the corpus puts there, and by what route it got there.</p>'

  const renderPanel = () => {
    if (!panel) return

    // Nothing selected is the common case, and it was reached by filtering the whole corpus and
    // grouping it onto anchors first — work whose only use was to find no match in it. It costs
    // little and it is on the path the transport runs every frame, which is reason enough for
    // the question to be asked before the work rather than after it.
    if (!state.selected) {
      panel.innerHTML = empty
      return
    }

    const look = view(state.year, state.grain, eras, {
      generous: state.generous,
      hops: state.hops,
    })
    const pool = [...drawn(records, look), ...(state.undated ? undrawn(records, look) : [])]
    const selected = byNode.get(state.selected)
    const here = anchored(pool).find((a) => a.node === state.selected)
      ?? anchored(pool).find((a) => a.records.some((r) => r.node === state.selected))
    const related = reading[state.selected] ?? []
    const relatedHtml = related.length
      ? `<section><h3>Related reading</h3><ul>${related.map((article) =>
          `<li><a href="/read/${esc(article.slug)}">${esc(article.title)}</a></li>`).join('')}</ul></section>`
      : ''
    const selectedWhen = selected?.from === null ? 'Undated' : selected
      ? selected.to === null ? `From ${selected.from}`
        : selected.from === selected.to ? String(selected.from) : `${selected.from}–${selected.to}`
      : ''
    let selectionHtml = selected
      ? `<h3 tabindex="-1" data-selected-heading>${esc(selected.label)}</h3>
        <p class="count">${esc(selected.class)} · ${esc(selectedWhen)} · ${esc(selected.tier)}</p>
        <p><a href="${entryPath(selected.node)}">Read the entry</a></p>${relatedHtml}`
      : ''

    if (selected && !pool.some((r) => r.node === selected.node)) {
      const placed = selected.hops !== null && selected.anchors.length > 0
      const reasons: string[] = []
      if (!placed) reasons.push('This record has no map position.')
      if (selected.from === null && !state.undated) reasons.push('Undated records are hidden in this view.')
      if (placed && selected.hops! > state.hops) reasons.push('Its placement is beyond the selected placement depth.')
      if (selected.from !== null && !drawn([selected], { ...look, hops: 3 }).length && placed) {
        reasons.push('This record is outside the selected time window.')
      }
      selectionHtml += `<p class="count">${reasons.join(' ')}</p>${placed
        ? '<button type="button" data-show-selected>Show this record on the map</button>' : ''}`
      if (!here) {
        panel.innerHTML = selectionHtml
        return
      }
    }

    if (!here) {
      // A track is not in `anchored` and cannot be: its anchor carries no lat and no lon,
      // because the midpoint of a seventeen-mile tornado is a position no source recorded. So it
      // is read out as what it is — the positions the corpus holds, and the distance between
      // them, which is not the track length the source states.
      const track = tracked(pool).find((t) => t.node === state.selected)
      if (track) {
        const r = track.record
        const when = r.from === null ? 'undated' : r.to === null || r.from === r.to ? String(r.from) : `${r.from}–${r.to}`
        const positions = track.points
          .map(
            (at, i) => `<li>
              <span class="role">${i === 0 ? 'Began' : i === track.points.length - 1 ? 'Ended' : 'Passed'}</span>
              <span class="at">${esc(bearing(at))}</span>
            </li>`,
          )
          .join('')
        panel.innerHTML = `${selectionHtml}
          <p class="count">${track.points.length} positions, stated by the source in this order, in ${esc(when)}.
          The corpus places this one itself — no edge was followed to reach it.</p>
          <ol class="positions">${positions}</ol>
          <p class="count">${straightMiles(track.points).toFixed(2)} miles between them in a straight line, which is
          <strong>not</strong> the track length the source states. Nothing about the ground between is on the record,
          so the connector is drawn broken and nothing travels along it.</p>
          <p><a href="${entryPath(r.node)}">Read the entry</a></p>`
        return
      }
      panel.innerHTML = selectionHtml || '<p class="empty">This selection is unavailable. Search for a published record or select a mark.</p>'
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

    panel.innerHTML = `${selectionHtml || `<h3>${esc(anchorLabel(here))}</h3>`}
      <h3>At ${esc(anchorLabel(here))}</h3>
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

  const defaultCamera: MapCamera = {
    longitude: fitted.longitude, latitude: fitted.latitude, zoom: fitted.zoom,
  }
  state.camera ??= defaultCamera
  const distance = container.querySelector<HTMLElement>('[data-map-distance]')
  const showDistance = () => {
    if (!distance || !state.camera) return
    const size = canvasHost.getBoundingClientRect()
    if (!size.width || !size.height) return
    const viewport = new WebMercatorViewport({ width: size.width, height: size.height, ...state.camera })
    const left = viewport.unproject([size.width / 2 - 40, size.height / 2])
    const right = viewport.unproject([size.width / 2 + 40, size.height / 2])
    const miles = straightMiles([{ lon: left[0], lat: left[1] }, { lon: right[0], lat: right[1] }])
    distance.style.width = '80px'
    distance.style.borderBottom = '2px solid var(--text-muted)'
    distance.textContent = `${miles < 1 ? miles.toFixed(2) : miles.toFixed(1)} mi`
  }
  let cameraTimer: ReturnType<typeof setTimeout> | undefined
  const cameraProps = () => ({ ...state.camera!, minZoom: 0, maxZoom: 14, bearing: 0, pitch: 0 })
  const deck = new Deck({
    parent: canvasHost,
    viewState: cameraProps(),
    controller: { dragRotate: false, touchRotate: false },
    onViewStateChange: ({ viewState }) => {
      const camera = viewState as MapCamera
      state.camera = { longitude: ((camera.longitude + 180) % 360 + 360) % 360 - 180,
        latitude: camera.latitude, zoom: camera.zoom }
      deck.setProps({ viewState: cameraProps() })
      showDistance()
      clearTimeout(cameraTimer)
      cameraTimer = setTimeout(() => address(), 250)
    },
    layers: layers(),
    getCursor: ({ isHovering }) => (isHovering ? 'pointer' : 'grab'),
    style: { position: 'absolute', inset: '0' },
  })

  const moveCamera = (camera: MapCamera) => {
    clearTimeout(cameraTimer)
    state.camera = camera
    deck.setProps({ viewState: cameraProps() })
    showDistance()
  }

  const focusRecord = (record: AtlasRecord) => {
    const positions = record.anchors.flatMap((anchor) => {
      if (anchor.points.length) return anchor.points.map((p) => [p.lon, p.lat] as [number, number])
      const shape = anchor.level && anchor.geoid ? shapes.get(`${anchor.level}:${anchor.geoid}`) : undefined
      if (shape) {
        const shapeBounds = bounds(shape.geometry)
        return [[shapeBounds[0], shapeBounds[1]], [shapeBounds[2], shapeBounds[3]]] as [number, number][]
      }
      return anchor.lon !== null && anchor.lat !== null ? [[anchor.lon, anchor.lat] as [number, number]] : []
    })
    if (!positions.length) return
    const frameRect = canvasHost.getBoundingClientRect()
    if (positions.length === 1) {
      moveCamera({ longitude: positions[0][0], latitude: positions[0][1], zoom: Math.max(state.camera!.zoom, 10) })
    } else {
      const focused = new WebMercatorViewport({ width: Math.max(frameRect.width, 1), height: Math.max(frameRect.height, 1) })
        .fitBounds([
          [Math.min(...positions.map((p) => p[0])), Math.min(...positions.map((p) => p[1]))],
          [Math.max(...positions.map((p) => p[0])), Math.max(...positions.map((p) => p[1]))],
        ], { padding: 64, maxZoom: 12 })
      moveCamera({ longitude: focused.longitude, latitude: focused.latitude, zoom: focused.zoom })
    }
  }

  const selectRecord = (id: string | null, focus = false) => {
    halt()
    state.selected = id
    setPanel(id ? 'preview' : 'closed')
    if (focus && id) {
      const record = byNode.get(id)
      if (record) focusRecord(record)
    }
    update('push')
    if (focus) panel?.querySelector<HTMLElement>('[data-selected-heading]')?.focus({ preventScroll: true })
  }

  const refresh = () => deck.setProps({ layers: layers() })

  /** Everything the year touches, without writing the URL. */
  const paint = () => {
    showScale()
    readouts()
    refresh()
    renderPanel()
  }

  const update = (mode: 'push' | 'replace' = 'push') => {
    clearTimeout(cameraTimer)
    address(mode)
    paint()
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
  // The transport
  //
  // The axis, played. Every frame is `state.year` set to a value the scrub could already have
  // been dragged to — the transport computes no state and asserts nothing the URL for that year
  // did not already assert. Nothing is interpolated between two frames and nothing may be: see
  // `an-animation-asserts-continuity`, and `lib/motion.ts`, which holds the rules as functions.
  // -------------------------------------------------------------------------

  const playButton = container.querySelector<HTMLButtonElement>('[data-play]')
  const playLabel = container.querySelector<HTMLElement>('[data-play-label]')
  const playNote = container.querySelector<HTMLElement>('[data-play-note]')

  let stops: Stop[] = []
  let head = START
  let frame = 0
  let last = 0

  /**
   * The longest slice of wall time one tick may consume.
   *
   * Two cases reach this, and the cap is right for both. `requestAnimationFrame` does not fire
   * in a background tab, so a reader who switches away for a minute comes back holding a delta
   * of sixty seconds; and a slow renderer hands back a frame every few hundred milliseconds all
   * the way through. `advance` would honour either and cross most of the county's history in
   * one frame — correct arithmetic, useless playback.
   *
   * So the transport degrades by **running slower, never by skipping years**. That direction is
   * the one the route argues for: every year gets a frame, because a playback that dropped the
   * thin decades would hide the shape of the archive exactly as a linear axis does.
   *
   * **This is not a hot path, and it was measured rather than assumed.** A CPU profile of four
   * seconds of playback under software WebGL came back 98.5 per cent idle: the whole of this
   * file's per-frame work — three passes over 677 records, the layer rebuild, the readouts —
   * is a few milliseconds, and the frame interval is the GPU rasterising. Anyone reaching for a
   * memo or a shared computation here should profile first and find the same thing.
   */
  const MAX_DELTA = 200

  const label = (playing: boolean) => {
    playButton?.setAttribute('aria-pressed', String(playing))
    if (playLabel) playLabel.textContent = playing ? 'Pause' : 'Play'
  }

  /**
   * Stop the transport and write the year the reader stopped on.
   *
   * The URL is written **here** rather than on every frame. `address()` is a `replaceState`,
   * and a play at the fastest cadence calls it eleven times a second — past what Safari
   * permits before it starts dropping them. It is also the wrong shape: `address()` writes a
   * view somebody could be sent, and a moving view is not a view. So playback carries no
   * playback state in the query string, and the URL becomes true again the moment it stops.
   */
  const halt = () => {
    if (!frame) return
    cancelAnimationFrame(frame)
    frame = 0
    last = 0
    label(false)
    address()
  }

  const tick = (now: number) => {
    const delta = last === 0 ? 0 : Math.min(now - last, MAX_DELTA)
    last = now
    head = advance(head, stops, delta)
    const year = stops[head.index]?.year
    if (year !== undefined && year !== state.year) {
      state.year = year
      paint()
    }
    if (head.done) {
      halt()
      return
    }
    frame = requestAnimationFrame(tick)
  }

  const start = () => {
    // Rebuilt on every play rather than cached: the reader may have changed theme, or their
    // motion preference, since the last one — and the cadence comes out of the stylesheet.
    stops = playbackRoute(records, eras, cadence())
    if (stops.length === 0) return
    // Standing at the end means the reader has already watched it, or has just arrived at the
    // default year, which is the last one. Either way the useful play is from the beginning.
    const ended = state.year >= stops[stops.length - 1].year
    head = seek(stops, ended ? stops[0].year : state.year)
    state.year = stops[head.index].year
    address('push')
    paint()
    label(true)
    frame = requestAnimationFrame(tick)
  }

  // The control is hidden in the markup and claimed here, so a reader who never runs this
  // script is not offered a dead button. Everything else on the axis works without JavaScript
  // or at least says what it means; a play button does neither.
  if (playButton) {
    playButton.hidden = false
    if (playNote) playNote.hidden = false
    playButton.addEventListener('click', () => (frame ? halt() : start()))
  }

  // -------------------------------------------------------------------------
  // The controls
  //
  // Every one of them is real markup rendered by the page, so the spine, the ribbon and the
  // period names are in the document before this script runs and are readable without it.
  //
  // Each of the three that moves the year halts the transport first: a reader reaching for the
  // scrub is taking the wheel, and a control that fights the playhead for the same integer is
  // a control that appears broken.
  // -------------------------------------------------------------------------

  for (const tile of container.querySelectorAll<HTMLElement>('[data-era]')) {
    tile.addEventListener('click', () => {
      halt()
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
      halt()
      state.year = Number(chip.dataset.periodFrom)
      update()
    })
  }

  let scrubbing = false
  scrub?.addEventListener('change', () => { scrubbing = false })
  scrub?.addEventListener('input', () => {
    halt()
    state.year = Number(scrub.value)
    update(scrubbing ? 'replace' : 'push')
    scrubbing = true
  })

  for (const input of container.querySelectorAll<HTMLInputElement>('[data-grain]')) {
    input.addEventListener('change', () => {
      halt()
      if (input.checked) state.grain = input.value as Grain
      update()
    })
  }

  const warrant = container.querySelector<HTMLSelectElement>('[data-warrant]')
  warrant?.addEventListener('change', () => {
    halt()
    state.hops = Number(warrant.value)
    update()
  })

  for (const input of container.querySelectorAll<HTMLInputElement>('[data-toggle]')) {
    input.addEventListener('change', () => {
      halt()
      if (input.dataset.toggle === 'generous') state.generous = input.checked
      if (input.dataset.toggle === 'undated') state.undated = input.checked
      update()
    })
  }

  for (const input of container.querySelectorAll<HTMLInputElement>('[data-layer]')) {
    input.addEventListener('change', () => {
      halt()
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

      update()
    })
  }

  const measure = container.querySelector<HTMLSelectElement>('[data-map-measure]')
  measure?.addEventListener('change', () => {
    halt()
    for (const key of CHOROPLETHS) state.ground.delete(key)
    if ((CHOROPLETHS as readonly string[]).includes(measure.value)) state.ground.add(measure.value)
    update()
  })

  /** The URL, canvas and controls agree, including after Back. */
  const syncControls = () => {
    for (const input of container.querySelectorAll<HTMLInputElement>('[data-grain]')) {
      input.checked = input.value === state.grain
    }
    if (warrant) warrant.value = String(state.hops)
    if (measure) measure.value = CHOROPLETHS.find((key) => state.ground.has(key)) ?? ''
    for (const input of container.querySelectorAll<HTMLInputElement>('[data-layer]')) {
      input.checked = state.ground.has(input.dataset.layer ?? '')
    }
    for (const input of container.querySelectorAll<HTMLInputElement>('[data-toggle]')) {
      if (input.dataset.toggle === 'generous') input.checked = state.generous
      if (input.dataset.toggle === 'undated') input.checked = state.undated
    }
  }

  handleSelection = (id) => {
    if (!byNode.has(id)) return
    selectRecord(id, true)
    for (const drawer of container.querySelectorAll<HTMLDetailsElement>('.search-drawer')) drawer.open = false
  }

  container.addEventListener('click', async (event) => {
    const target = (event.target as Element).closest<HTMLElement>('[data-map-action], [data-panel-action], [data-show-selected]')
    if (!target) return
    const panelAction = target.dataset.panelAction
    if (panelAction === 'close' || panelAction === 'preview' || panelAction === 'expand') {
      setPanel(panelAction === 'expand' ? 'expanded' : panelAction === 'close' ? 'closed' : 'preview')
      if (panelAction === 'close') container.querySelector<HTMLElement>('.inspector-trigger')?.focus({ preventScroll: true })
      else panel?.focus({ preventScroll: true })
      return
    }
    if (target.hasAttribute('data-show-selected')) {
      const record = state.selected ? byNode.get(state.selected) : undefined
      if (!record) return
      halt()
      if (record.from === null) state.undated = true
      else {
        state.year = Math.max(extent.from, Math.min(extent.to, record.from))
        state.grain = 'year'
      }
      if (record.hops !== null) state.hops = Math.max(state.hops, record.hops)
      focusRecord(record)
      syncControls()
      update()
      panel?.querySelector<HTMLElement>('[data-selected-heading]')?.focus({ preventScroll: true })
      return
    }

    halt()
    const action = target.dataset.mapAction
    const camera = state.camera ?? defaultCamera
    if (action === 'share') {
      address()
      const status = container.querySelector<HTMLElement>('[data-map-share-status]')
      if (!status) return
      const url = globalThis.location.href
      try {
        await navigator.clipboard.writeText(url)
        status.textContent = 'Link copied. It includes this map view.'
      } catch {
        status.replaceChildren()
        const labelElement = document.createElement('label')
        labelElement.textContent = 'Copy this view: '
        const input = document.createElement('input')
        input.readOnly = true
        input.value = url
        labelElement.append(input)
        status.append(labelElement)
        input.focus()
        input.select()
      }
      return
    }
    if (action === 'zoom-in' || action === 'zoom-out') {
      moveCamera({ ...camera, zoom: Math.min(14, Math.max(0, camera.zoom + (action === 'zoom-in' ? 1 : -1))) })
    } else if (action === 'reset') {
      const size = canvasHost.getBoundingClientRect()
      const countyView = new WebMercatorViewport({ width: Math.max(size.width, 1), height: Math.max(size.height, 1) })
        .fitBounds([[box[0], box[1]], [box[2], box[3]]], { padding: 32 })
      moveCamera({ longitude: countyView.longitude, latitude: countyView.latitude, zoom: countyView.zoom })
    } else if (action && ['north', 'south', 'east', 'west'].includes(action)) {
      const size = canvasHost.getBoundingClientRect()
      const viewport = new WebMercatorViewport({ width: size.width, height: size.height, ...camera })
      const [longitude, latitude] = viewport.unproject([
        size.width / 2 + (action === 'east' ? 100 : action === 'west' ? -100 : 0),
        size.height / 2 + (action === 'south' ? 100 : action === 'north' ? -100 : 0),
      ])
      moveCamera({ ...camera, longitude: ((longitude + 180) % 360 + 360) % 360 - 180,
        latitude: Math.max(-85, Math.min(85, latitude)) })
    } else return
    address('push')
  })

  globalThis.addEventListener('popstate', () => {
    clearTimeout(cameraTimer)
    if (frame) {
      cancelAnimationFrame(frame)
      frame = 0
      label(false)
    }
    state = parseMapState(globalThis.location.search, extent)
    moveCamera(state.camera ?? defaultCamera)
    syncControls()
    setPanel(state.selected ? 'preview' : 'closed')
    paint()
  })

  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', refresh)
  new MutationObserver(refresh).observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['data-theme'],
  })

  syncControls()
  setPanel(state.selected ? 'preview' : 'closed')
  update('replace')
  showDistance()
  new ResizeObserver(showDistance).observe(canvasHost)
  canvasHost.querySelector('[data-map-loading]')?.remove()
  container.dataset.mapReady = 'true'
  if (pendingSelection) handleSelection(pendingSelection)
}

/** Exported for the page, which prints the spine and the ribbon at build time. */
export type { View }
