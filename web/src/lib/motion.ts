// The clock the axis travels on.
//
// Everything here is a pure function or a pure reducer. Nothing touches the canvas, reads the
// DOM, or calls `requestAnimationFrame` — the same posture `eras.ts` takes, and for the same
// reason: the interesting decisions about a transport are about *when it is allowed to move*,
// and those should be checkable without a browser.
//
// ---- What motion is permitted to mean ----
//
// `an-animation-asserts-continuity` is the rule this module implements, and it has three parts.
//
//   **Every frame is a state the corpus states.** The transport sets `state.year` to values the
//   scrub could already be dragged to. It computes no intermediate state and asserts nothing
//   the URL for that year did not already assert.
//
//   **Transitions are stepped, not tweened.** There is no interpolation anywhere in this file
//   and there must not be one. A tween between two dated states draws positions and magnitudes
//   no source recorded, in the one encoding a reader cannot interrogate — a frame lasts 40ms,
//   carries no tooltip, and has nowhere to put a claim tag.
//
//   **Precision floors the grain.** {@link mayStep} refuses a step finer than the source gave.
//   An event dated to `1820` gets no sub-year motion.
//
// ---- Why the pacing is not one year per tick ----
//
// `eras.ts` builds a six-tile axis on the finding that an evenly spaced control over 257 years
// "gives 99.6 per cent of its travel to the 96 per cent of the timespan holding half the
// record". A transport advancing one year per tick reintroduces exactly that defect in the one
// mode where the reader cannot compensate by dragging faster: they watch two centuries of empty
// county, and either conclude the map is broken or stop watching before the record arrives.
//
// So a stop's *hold* is a function of how much record stands in it. That is pacing and not a
// claim — it changes how long the reader looks at a state, never which states exist.

import { span, type Era } from './eras'
import type { AtlasRecord } from './feeds'

/**
 * How long the transport holds a year, in milliseconds.
 *
 * Read from the token layer rather than written here — `tokens/site.css` declares
 * `--replay-step` and `--replay-hold`, and `map.ts` resolves them the way `palette()` resolves
 * ink. A duration baked into a script is a duration the design layer cannot see.
 */
export interface Cadence {
  /** The floor: how long the emptiest year on the axis is held. */
  step: number
  /** The ceiling: how long the fullest year is held. */
  hold: number
}

/** One stop on the route: the year the map stands at, and how long it stands there. */
export interface Stop {
  year: number
  /** Milliseconds. Never zero — a frame nobody can see is not a frame. */
  hold: number
}

/**
 * The cadence a reader who has asked for no motion gets.
 *
 * **Not zero, and not "no playback".** `motion.css` collapses every duration to `0ms` under
 * `prefers-reduced-motion`, which is right for a hover transition and wrong for a transport: a
 * play control resolving to zero duration does not degrade, it jumps to the end and shows the
 * reader nothing. So the holds become uniform and long enough to read, and the variable dwell
 * — which is the part that reads as *movement* rather than as a sequence of stills — is dropped.
 *
 * Cuts are not what `prefers-reduced-motion` is about. The setting exists for smooth, sustained
 * or parallax movement; a hard cut between two states every second is a slideshow, which is the
 * accessible form of this control rather than a concession on it.
 */
export const STILL: Cadence = { step: 900, hold: 900 }

/**
 * How much record stands in each year of a window.
 *
 * The generous reading of an open end, the same one the ribbon uses, and for the same reason:
 * the question this answers is *is there anything here to look at*, and a reader deciding
 * whether a decade is worth dwelling on should not be talked out of it by a distinction about
 * vouching that they have not met yet. The strict reading governs what is *drawn*; this governs
 * only how long the drawing is held.
 */
export function density(records: AtlasRecord[], from: number, to: number): number[] {
  const counts: number[] = Array.from({ length: Math.max(to - from + 1, 0) }, () => 0)
  for (const record of records) {
    const s = span(record, true)
    if (s === null) continue
    const first = Math.max(s[0], from)
    const last = Math.min(s[1], to)
    for (let year = first; year <= last; year++) counts[year - from]++
  }
  return counts
}

/**
 * The route the transport walks: every year on the axis, paced by what stands in it.
 *
 * **Every year is visited.** The pacing changes how long a year is held and never whether it is
 * held at all — a route that skipped the empty decades would be a route that hid the shape of
 * the archive, which is the one thing the ribbon under the axis exists to show.
 *
 * The hold is linear in the year's share of the fullest year rather than in its raw count. A
 * proportional-to-count pacing on this corpus gives the 2020s a hold two orders of magnitude
 * longer than the 1770s, which is not dwelling — it is stalling.
 */
export function route(records: AtlasRecord[], eras: Era[], cadence: Cadence): Stop[] {
  const from = eras[0].from
  const to = eras[eras.length - 1].to
  const counts = density(records, from, to)
  const fullest = Math.max(...counts, 1)
  const range = Math.max(cadence.hold - cadence.step, 0)
  return counts.map((count, i) => ({
    year: from + i,
    hold: Math.round(cadence.step + range * (count / fullest)),
  }))
}

/**
 * Whether a record may be animated at a grain finer than its whole span.
 *
 * The rule the transport asks before drawing a replay, and the third part of
 * `an-animation-asserts-continuity`. A record the corpus dates only to the year is drawn entire
 * or not at all: sweeping through it frame by frame would draw a specific state at a specific
 * moment inside an interval the source resolved no further, and a smooth sweep reads as
 * knowledge of the progress.
 *
 * `chronology` keeps only the year and records how much more the source gave, so a step can
 * never be finer than a year in any case. What this refuses is the other direction — reading a
 * coarse date as though it named a moment.
 */
export function mayStep(record: { precision: string | null; from: number | null; to: number | null }): boolean {
  if (record.from === null) return false
  if (record.precision !== 'day' && record.precision !== 'month') return false
  return (record.to ?? record.from) > record.from
}

// ---------------------------------------------------------------------------
// The playhead
// ---------------------------------------------------------------------------

/** Where the transport has got to. Immutable — {@link advance} returns a new one. */
export interface Playhead {
  /** Index into the route. */
  index: number
  /** Milliseconds accumulated against the current stop's hold. */
  elapsed: number
  /** True once the route has been walked to its end. */
  done: boolean
}

export const START: Playhead = { index: 0, elapsed: 0, done: false }

/**
 * Move the playhead by a slice of wall time.
 *
 * A pure reducer, which is the whole reason the transport is testable. `rate` multiplies elapsed
 * time rather than dividing holds, so a fast rate cannot round a short hold to zero and skip a
 * year — at 8× the 1770s still get a frame each.
 *
 * **A large delta advances several stops rather than one.** A backgrounded tab, a slow frame or
 * a breakpoint hands back a delta of hundreds of milliseconds, and consuming one stop per call
 * would silently slow the route to the frame rate. The loop is bounded by the route's own
 * length, so a pathological delta ends the route rather than spinning.
 */
export function advance(head: Playhead, stops: Stop[], deltaMs: number, rate = 1): Playhead {
  if (head.done || stops.length === 0) return head
  let { index } = head
  let elapsed = head.elapsed + Math.max(deltaMs, 0) * Math.max(rate, 0)
  while (index < stops.length && elapsed >= stops[index].hold) {
    elapsed -= stops[index].hold
    index++
  }
  if (index >= stops.length) return { index: stops.length - 1, elapsed: 0, done: true }
  return { index, elapsed, done: false }
}

/** Put the playhead on a year, wherever the reader dragged the scrub to. */
export function seek(stops: Stop[], year: number): Playhead {
  const index = stops.findIndex((s) => s.year >= year)
  return { index: index === -1 ? Math.max(stops.length - 1, 0) : index, elapsed: 0, done: false }
}

/**
 * The whole route's wall-clock length at a rate, in milliseconds.
 *
 * For the control, which should be able to say how long a play will take before the reader
 * commits to watching it.
 */
export function duration(stops: Stop[], rate = 1): number {
  const total = stops.reduce((sum, s) => sum + s.hold, 0)
  return rate > 0 ? Math.round(total / rate) : 0
}
