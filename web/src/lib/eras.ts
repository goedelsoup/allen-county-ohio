// The axis the map travels along, and what stands on it at a given year.
//
// Everything here is a pure function over `atlas.json`. None of it touches the canvas, for the
// reason `crates/chronology` keeps its normalizer apart from its loader: the interesting
// decisions should be checkable without a browser.
//
// ---- Why the axis is not a slider ----
//
// 484 of the corpus's 667 nodes carry a span, and they run from 1769 to 2026. Half of that mass
// sits in one decade. An evenly spaced control over 257 years gives 99.6 per cent of its travel
// to the 96 per cent of the timespan holding half the record — the reader drags across two
// centuries, watches nothing move, reaches the end, and the county appears all at once. They
// conclude the map is broken. It is not: it is showing them the shape of the archive, and
// nothing on the screen says so.
//
// So the axis is **six tiles of equal width and unequal duration**, and the density ribbon under
// them says what each holds. Time is deliberately not linear here, and the tiles print their own
// year ranges so a reader can see that it is not.
//
// ---- Why the six tiles are not the corpus's seven periods ----
//
// The obvious build is one button per `period` node. The seven nodes forbid it: they do not tile
// and were never meant to. Nothing covers 1769–1845, 1910–17, 1918–41 or 1946–70. The canal era
// wholly contains both the Civil War and the oil boom. Two of the seven are wars a few years long
// sitting beside a fifty-five-year era. They are interpretations of spans, which is what the class
// says they are, and an interpretation is not a partition.
//
// The design system's era ramp *is* a partition. It tiles the whole range exactly and it balances
// far better than decades. So the ramp is the spine and the periods ride on top as named overlays
// — jump targets that highlight their own span, overlap freely, and say in the corpus's own words
// why that stretch is a unit. The reader gets an axis that is always somewhere and a set of names
// that mean something, and neither has to pretend to be the other.

import type { AtlasRecord } from './feeds'

/**
 * One step of the design system's era ramp.
 *
 * The ink names are `colors.css`'s and the boundaries are the ink names' own — `--era-1860`
 * begins in 1860. The labels are this site's; the period nodes keep theirs.
 */
export interface EraStep {
  key: string
  label: string
  /** The custom property holding this step's ink. Resolved at render, never copied. */
  ink: string
  /** First year of the tile, or `null` for the opening tile, which starts where the record does. */
  start: number | null
}

export const ERA_STEPS: readonly EraStep[] = [
  { key: 'pre1820', label: 'Settlement', ink: '--era-pre1820', start: null },
  { key: '1820', label: 'Erection', ink: '--era-1820', start: 1820 },
  { key: '1860', label: 'Rail and oil', ink: '--era-1860', start: 1860 },
  { key: '1900', label: 'Industry', ink: '--era-1900', start: 1900 },
  { key: '1940', label: 'Peak', ink: '--era-1940', start: 1940 },
  { key: '1980', label: 'After', ink: '--era-1980', start: 1980 },
]

/** A tile of the spine, with both ends resolved against the record. */
export interface Era extends EraStep {
  from: number
  /** Inclusive. */
  to: number
}

/** The first and last year the corpus places anything in. */
export function extent(records: AtlasRecord[]): { from: number; to: number } {
  let from = Infinity
  let to = -Infinity
  for (const r of records) {
    if (r.from === null) continue
    from = Math.min(from, r.from)
    to = Math.max(to, r.to ?? r.from)
  }
  // An open end reads as running to the present, and the present is later than any date a
  // source recorded. Without this the spine stops at the last figure anyone published.
  if (records.some((r) => r.to === null && r.open_end === 'running')) {
    to = Math.max(to, new Date().getUTCFullYear())
  }
  return Number.isFinite(from) ? { from, to } : { from: 0, to: 0 }
}

/**
 * The six tiles, with the outer ends taken from the record rather than written down.
 *
 * The inner boundaries are fixed — they are the design system's, and moving one would mean the
 * ink no longer names the years it covers. The two outer ends are not: the opening tile begins
 * wherever the corpus's earliest span begins, and the closing one runs to wherever it runs. A
 * spine with its ends hard-coded goes stale the first time somebody catalogues something older.
 */
export function spine(records: AtlasRecord[]): Era[] {
  const { from, to } = extent(records)
  const opens = Math.min(from, (ERA_STEPS[1].start as number) - 1)
  const tiles: Era[] = []
  for (const [i, step] of ERA_STEPS.entries()) {
    const next = ERA_STEPS[i + 1]?.start
    tiles.push({
      key: step.key,
      label: step.label,
      ink: step.ink,
      start: step.start,
      from: step.start ?? opens,
      to: next === undefined || next === null ? Math.max(to, from) : next - 1,
    })
  }
  return tiles
}

/** The tile a year falls in. Years outside the spine clamp to its ends rather than throwing. */
export function eraAt(year: number, eras: Era[]): Era {
  return eras.find((e) => year >= e.from && year <= e.to) ?? (year < eras[0].from ? eras[0] : eras[eras.length - 1])
}

// ---------------------------------------------------------------------------
// What is standing at a year
// ---------------------------------------------------------------------------

/** How wide a window the reader's year stands for. */
export type Grain = 'year' | 'era'

/**
 * A window on the axis, and the two questions the reader may ask inside it.
 *
 * `from`/`to` are inclusive and resolved: a `year`-grain view is one year wide, an `era`-grain
 * view is its whole tile. Both are on the page because they answer genuinely different questions
 * and the strict year is much the sparser of the two — 36 records stand in 1885 against 67 in
 * the tile that contains it. A map that showed only the first would read as an empty county; one
 * that showed only the second could never say what a single year held.
 */
export interface View {
  from: number
  to: number
  /**
   * Carry a record past an open end its class declines to vouch for.
   *
   * `false` is the strict reading and the default. A division effective from 2020 admits 2024
   * and vouches for nothing there; the two are separate questions and separate lists, and
   * merging them invents a fact. The control exists so a reader can ask the other question, not
   * so the map can answer both at once.
   */
  generous: boolean
  /**
   * The deepest derivation a placement may have and still be drawn.
   *
   * `0` is stated positions only — the 72 nodes carrying a coordinate or a Census key. Every step
   * above that is an inference this map is making, and the control is what lets a reader watch
   * the map lose most of itself.
   */
  hops: number
}

/** The window a year and a grain resolve to. */
export function view(
  year: number,
  grain: Grain,
  eras: Era[],
  options: { generous: boolean; hops: number },
): View {
  const era = eraAt(year, eras)
  return {
    from: grain === 'era' ? era.from : year,
    to: grain === 'era' ? era.to : year,
    ...options,
  }
}

/**
 * The years a record stands in, as an interval.
 *
 * `Infinity` is the deliberate spelling of an end that runs to the present: "now" is not a date
 * any source recorded, and writing this year's number here would put a date in the corpus's
 * mouth. The three other readings of an absent end are the reason this takes `generous` —
 * `unvouched` and `unknown` support their start year and, strictly, nothing after it.
 */
export function span(record: AtlasRecord, generous: boolean): [number, number] | null {
  if (record.from === null) return null
  if (record.to !== null) return [record.from, record.to]
  if (record.open_end === 'running') return [record.from, Infinity]
  if (record.open_end === 'instantaneous') return [record.from, record.from]
  return generous ? [record.from, Infinity] : [record.from, record.from]
}

/** Whether the corpus places a record in `year`. */
export function admits(record: AtlasRecord, year: number, generous: boolean): boolean {
  const s = span(record, generous)
  return s !== null && year >= s[0] && year <= s[1]
}

/** Whether a record stands anywhere in a window. */
export function standing(record: AtlasRecord, look: View): boolean {
  const s = span(record, look.generous)
  return s !== null && s[0] <= look.to && look.from <= s[1]
}

/** Every record the view draws: placed, within the warrant, and standing in the window. */
export function drawn(records: AtlasRecord[], look: View): AtlasRecord[] {
  return records.filter(
    (r) => r.hops !== null && r.hops <= look.hops && r.anchors.length > 0 && standing(r, look),
  )
}

/**
 * Every record the corpus cannot date, and can place.
 *
 * Kept separate from `drawn` on purpose: these stand at no year and belong to no window, so
 * folding them into a year's list would be the merge the whole feed is composed to avoid. They
 * are drawn on their own toggle, in their own encoding, and three of them are shapes — Bath
 * Township, Fort Shawnee and Harrod are holes in the county that are holes in the record.
 */
export function undrawn(records: AtlasRecord[], look: View): AtlasRecord[] {
  return records.filter(
    (r) => r.from === null && r.hops !== null && r.hops <= look.hops && r.anchors.length > 0,
  )
}

/**
 * What the window holds, in the states the map has to keep apart.
 *
 * `undated` is the one worth the control. A node the corpus cannot date is not a gap in the
 * county's history; it is a gap in the record, and a map that silently omits it says the first
 * when it means the second.
 */
export interface Census {
  /** Dated, placed, within the warrant, and standing in the window. */
  present: number
  /** Dated and placed, and standing outside it. */
  elsewhen: number
  /** Standing in the window, and refused by the warrant control. */
  beyondWarrant: number
  /** Never dated. Not absent from the county — absent from the record. */
  undated: number
  /** Dated, and reaching no ground by any licensed route. */
  unplaced: number
}

export function census(records: AtlasRecord[], look: View): Census {
  const c: Census = { present: 0, elsewhen: 0, beyondWarrant: 0, undated: 0, unplaced: 0 }
  for (const r of records) {
    if (r.from === null) {
      c.undated++
      continue
    }
    if (r.hops === null || r.anchors.length === 0) {
      c.unplaced++
      continue
    }
    if (!standing(r, look)) {
      c.elsewhen++
      continue
    }
    if (r.hops > look.hops) c.beyondWarrant++
    else c.present++
  }
  return c
}

// ---------------------------------------------------------------------------
// Where the drawn records go
// ---------------------------------------------------------------------------

/** A position on the ground, and everything the year puts there. */
export interface Anchored {
  /** The anchor node — one of the 72 that state their own position. */
  node: string
  lat: number
  lon: number
  records: AtlasRecord[]
}

/**
 * The drawn records gathered onto the positions they reached.
 *
 * A record reaching several anchors at the same depth appears at **all** of them. That is the
 * decision's rule and not a defect: a district serving five townships is at all five, and
 * averaging them would invent a centroid no source stated.
 */
export function anchored(records: AtlasRecord[]): Anchored[] {
  const at = new Map<string, Anchored>()
  for (const record of records) {
    for (const a of record.anchors) {
      if (a.lat === null || a.lon === null) continue
      const found = at.get(a.node)
      if (found) found.records.push(record)
      else at.set(a.node, { node: a.node, lat: a.lat, lon: a.lon, records: [record] })
    }
  }
  return [...at.values()].toSorted((x, y) => y.records.length - x.records.length)
}

/**
 * A Census shape the map may draw, and the record that claims it.
 *
 * **Only a node that states its own key gets a shape.** A tract placed one edge away at the
 * county government would otherwise be drawn as the whole county, which is not where the tract
 * is — it is where the corpus could reach from it. Everything derived stays a mark on the
 * ground it actually reached, and says so.
 */
export interface Shaded {
  geoid: string
  record: AtlasRecord
}

export function shaded(records: AtlasRecord[]): Shaded[] {
  return records
    .filter((r) => r.treatment === 'polygon' && r.hops === 0)
    .flatMap((r) => r.anchors.filter((a) => a.geoid).map((a) => ({ geoid: a.geoid as string, record: r })))
}

// ---------------------------------------------------------------------------
// The ribbon
// ---------------------------------------------------------------------------

/** One bar: a decade, the tile it sits in, and how much record it holds. */
export interface Bar {
  era: string
  from: number
  to: number
  count: number
}

/**
 * How much record each decade holds, in spine space.
 *
 * Drawn under the tiles and aligned to them, so the ribbon inherits the spine's deliberate
 * non-linearity rather than arguing with it. A decade holds a record if the corpus places that
 * record in any year of it — the generous reading, because the question the ribbon answers is
 * *is there anything here to see*, and a reader deciding where to travel should not be talked
 * out of a decade by a distinction they have not met yet.
 */
export function ribbon(eras: Era[], records: AtlasRecord[]): Bar[] {
  const bars: Bar[] = []
  for (const era of eras) {
    for (let start = Math.floor(era.from / 10) * 10; start <= era.to; start += 10) {
      const from = Math.max(start, era.from)
      const to = Math.min(start + 9, era.to)
      let count = 0
      for (const r of records) {
        const s = span(r, true)
        if (s !== null && s[0] <= to && from <= s[1]) count++
      }
      bars.push({ era: era.key, from, to, count })
    }
  }
  return bars
}

// ---------------------------------------------------------------------------
// The periods, riding on top
// ---------------------------------------------------------------------------

/** A period node as a jump target. */
export interface Overlay {
  node: string
  label: string
  from: number
  /** `null` where the period is still running — the depopulation is the one that is. */
  to: number | null
}

export function overlays(records: AtlasRecord[]): Overlay[] {
  return records
    .filter((r) => r.class === 'period' && r.from !== null)
    .map((r) => ({ node: r.node, label: r.label, from: r.from as number, to: r.to }))
    .toSorted((a, b) => a.from - b.from)
}

// ---------------------------------------------------------------------------
// The ground, and how far the reader has travelled from it
// ---------------------------------------------------------------------------

/** The vintage of the boundaries in `public/geo/`. */
export const GROUND_VINTAGE = 2020

/**
 * The water's vintage, which is not the boundaries'.
 *
 * TIGERweb keeps hydrography out of the decennial services — one Hydro MapServer, republished
 * with the rest of TIGER and carrying no vintage of its own. `PROVENANCE.json` records it as
 * *current*, so the water is measured from the year it was fetched rather than from 2020. A layer
 * fading away from a date nobody published it in would be a fade that lied.
 */
export const WATER_VINTAGE = 2026

/**
 * Below this the county outline stops being an outline.
 *
 * 0.3 was the first value and it was too low: at 1832 the county's own edge had faded past the
 * township lines drawn inside it, and a reader had nothing to orient against. The fade is meant
 * to say *this ground is not this year's ground*, not to take the frame away.
 */
const FIDELITY_FLOOR = 0.45

/** How far the ghost can fade over. Two centuries of travel reaches the floor. */
const FIDELITY_SPAN = 200

/**
 * How much of the basemap's strength survives at `year`.
 *
 * **The geometry does not travel and must not pretend to.** Every boundary this site holds is a
 * 2020 Census statement, and drawing 1885 on it at full strength asserts that Lima's limits and
 * the townships' lines were where they are now. They were not: Spencer Township was erected in
 * 1848 out of ground the map already shows as Spencer's, and six of the villages had not been
 * incorporated at all.
 *
 * So the ground fades as the reader travels away from its own year. The fade is the caption
 * made visible — it says *this outline is a 2020 claim and you are 135 years from it* without
 * asking anyone to read a footnote. It floors rather than vanishing, because a county with no
 * outline is not an honest map either; it is an empty one.
 */
export function groundFidelity(year: number, vintage: number = GROUND_VINTAGE): number {
  const distance = Math.abs(year - vintage)
  return Math.max(FIDELITY_FLOOR, 1 - distance / FIDELITY_SPAN)
}
