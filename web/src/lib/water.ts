// The county's water, and what the corpus says about it.
//
// Two different things meet here and the whole file is about keeping them apart.
//
//   **The course** is a line on the ground, and it comes from TIGER's hydrography the way every
//   boundary on this map comes from TIGER: vendored, provenanced, and never derived from the
//   corpus.
//
//   **The topology** is the corpus's: which watercourse flows into which, where each rises and
//   where it ends. Nine nodes state a `source` and a `mouth` as real coordinates, and eight
//   `flows-into` edges say how they join.
//
// ---- Why a river gets a course and not a placement ----
//
// `crates/placement` reads four location properties and `mouth` is not one of them, which looked
// like an omission and is the opposite. **A river is not at its mouth.** The Ottawa's mouth is in
// Putnam County; the Auglaize's is in Defiance County, two counties away; Riley Creek rises in
// Hancock County. A resolver that took those as positions would place three of this county's
// rivers outside the county — the same failure `AGENTS.md` records against reading an event's
// `affected` edge as where it happened, in a different property.
//
// So the nine watercourses stay in the register, placed no finer than the county, and get a
// second thing instead: two stated endpoints, and a drawn course where the federal file names
// one.

import { nodes, type Node, type Tier } from './feeds'

/** One end of a watercourse, as the corpus states it. */
export interface End {
  lat: number
  lon: number
  /** Whatever the corpus wrote after the coordinates — usually the county it is in. */
  note: string
  /** The tag on the property, where it carries one. */
  tier: Tier | null
}

/** A watercourse's two ends, either of which the corpus may leave unstated. */
export interface Course {
  node: string
  label: string
  /** Where it rises. */
  source: End | null
  /** Where it ends, which is generally in another county. */
  mouth: End | null
  /** TIGER's spelling of the name, which is how the vendored file is joined. */
  tigerName: string
}

const TIER = /\[(verified|inference|open)\]/

/**
 * `40.995, -84.241, in Putnam County. Joins the Auglaize River. [verified]`
 *
 * The coordinates come first by convention and the rest is prose. Both halves are kept: the note
 * is where the corpus says *in Putnam County*, and a map drawing that point without it would
 * show a mark outside the frame with nothing to explain it.
 */
export function parseEnd(value: string | undefined): End | null {
  if (!value) return null
  const match = /^\s*(-?\d+(?:\.\d+)?)\s*,\s*(-?\d+(?:\.\d+)?)\s*(.*)$/s.exec(value)
  if (!match) return null
  const rest = match[3].replace(/^[\s,—–-]+/, '').trim()
  return {
    lat: Number(match[1]),
    lon: Number(match[2]),
    note: rest.replace(TIER, '').replace(/\s+/g, ' ').trim(),
    tier: (TIER.exec(rest)?.[1] as Tier) ?? null,
  }
}

/**
 * TIGER's abbreviations for the generic half of a water name.
 *
 * Written as a rule rather than as a table of nine, so a watercourse catalogued next month joins
 * without an edit here. The abbreviations are TIGER's own and apply to the last word only —
 * *Little Ottawa River* becomes *Little Ottawa Riv*, and the *Little* is untouched.
 */
const ABBREVIATION: Record<string, string> = {
  River: 'Riv',
  Creek: 'Crk',
  Canal: 'Cnl',
  Fork: 'Frk',
  Branch: 'Br',
  Brook: 'Brk',
}

export function tigerName(label: string): string {
  const words = label.split(' ')
  const last = words[words.length - 1]
  return [...words.slice(0, -1), ABBREVIATION[last] ?? last].join(' ')
}

/** A node the corpus treats as a watercourse rather than as a wetland or a watershed. */
const WATERCOURSE = new Set(['river', 'creek', 'stream', 'canal'])

export function isWatercourse(node: Node): boolean {
  return node.class === 'natural-feature' && WATERCOURSE.has(node.properties.feature_type ?? '')
}

/** Every watercourse the corpus holds, with whatever geometry it states. */
export function courses(): Course[] {
  return nodes.filter(isWatercourse).map((n) => ({
    node: n.id,
    label: n.label,
    source: parseEnd(n.properties.source),
    mouth: parseEnd(n.properties.mouth),
    tigerName: tigerName(n.label),
  }))
}

/**
 * Watercourses the corpus names and the vendored file does not draw inside the county.
 *
 * Two kinds, and only the first is unsurprising. The Blanchard and the Maumee are downstream
 * rivers this county's water reaches and never touches — the Blanchard is in Hancock and Putnam,
 * the Maumee runs to Toledo — so a file clipped to Allen County correctly holds neither.
 *
 * **The Ottawa is the finding.** It is the river Lima was built on and the one the corpus has
 * most to say about, and TIGER's linear hydrography does not name it anywhere inside the county
 * line. Its named segments begin north of the boundary, in Putnam County, and every segment
 * carrying it through Lima and Allen County is unnamed. So the corpus can say where the Ottawa
 * rises and where it ends, and this site cannot draw a foot of it.
 *
 * `geography.test.ts` fails on a name that is here and now resolves, as well as on one that is
 * absent and not here. The second half is what stops this from becoming a list of excuses.
 */
export const UNDRAWN: Record<string, string> = {
  'Ottawa Riv':
    'TIGER names the Ottawa only downstream of the county line; every segment inside Allen County is unnamed',
  'Blanchard Riv': 'flows in Hancock and Putnam counties — Riley Creek reaches it, this county does not hold it',
  'Maumee Riv': 'the Auglaize joins it in Defiance County, outside this map',
}
