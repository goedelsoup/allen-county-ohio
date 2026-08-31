// Entry pages over the published graph.
//
// One route renders every class. What varies by class is furniture, not structure: a tenure
// wants its interval drawn, a question wants what would close it, a measure wants its value
// and method. The shared parts — properties, relations, sources, record status — are the
// same apparatus everywhere because the corpus gives them the same shape everywhere.
//
// Only the feeds are read here, which is the site's whole contract. The class table below is
// deliberately presentation and nothing else: a glyph and a plural. A class's foundational
// type and edge policy are corpus facts, and they reach the page the only way they may — the
// feed carries them from `.yidam/corpus/*.ont.yml`, and `SchemaCard` renders what it finds.
// Restating them here would be exactly the drift the publication gate exists to prevent.

import { classSchema, edges, nodes, type Edge, type Node, type Tier } from './feeds'

/** Glyph and plural for each class. Presentation only — see the note above. */
export const CLASS_PRESENTATION: Record<string, { glyph: string; plural: string }> = {
  division: { glyph: 'layers', plural: 'Divisions' },
  event: { glyph: 'calendar', plural: 'Events' },
  jurisdiction: { glyph: 'landmark', plural: 'Jurisdictions' },
  measure: { glyph: 'ruler', plural: 'Measures' },
  'natural-feature': { glyph: 'compass', plural: 'Natural features' },
  office: { glyph: 'scroll-text', plural: 'Offices' },
  organization: { glyph: 'archive', plural: 'Organizations' },
  period: { glyph: 'book-open', plural: 'Periods' },
  person: { glyph: 'users', plural: 'People' },
  place: { glyph: 'map-pin', plural: 'Places' },
  question: { glyph: 'pen-line', plural: 'Questions' },
  site: { glyph: 'house', plural: 'Sites' },
  tenure: { glyph: 'clock', plural: 'Tenures' },
}

export function glyphFor(cls: string): string {
  return CLASS_PRESENTATION[cls]?.glyph ?? 'scroll-text'
}

export function pluralFor(cls: string): string {
  return CLASS_PRESENTATION[cls]?.plural ?? cls
}

export interface SchemaRow {
  label: string
  value: string
  /** Whether this row is the one the card sets in rubric. At most one row per class. */
  accent: boolean
}

/**
 * The Schema card's rows for one class, or none where the corpus declares no such class.
 *
 * Four rows, of which the fourth is whichever declaration is specific to the class: the
 * properties a node of it may not omit, or — where it requires none — its edge policy.
 *
 * At most one row is accented, and it is the one that departs from the default. `kind` and
 * `characteristic` are what most of the thirteen classes say, so most cards accent nothing.
 * Where a class both requires properties and is not a kind, the requirement takes the accent:
 * the card makes one point, and the sharper one is what a node of this class may not omit.
 */
export function schemaRows(cls: string): SchemaRow[] {
  const schema = classSchema(cls)
  if (!schema) return []

  return [
    { label: 'Class', value: schema.label, accent: false },
    { label: 'Ontology', value: schema.ontology.toUpperCase(), accent: false },
    {
      label: 'Foundational',
      value: schema.foundational_type,
      accent: schema.required.length === 0 && schema.foundational_type !== 'kind',
    },
    schema.required.length > 0
      ? { label: 'Required', value: schema.required.join(' · '), accent: true }
      : { label: 'Edge policy', value: schema.edge_policy, accent: false },
  ]
}

/** A class name as it reads in a sentence: `natural-feature` → `Natural feature`. */
export function className(cls: string): string {
  const word = cls.replace(/-/g, ' ')
  return word.charAt(0).toUpperCase() + word.slice(1)
}

/** `tenure/auditor-2023.yml` → `/entry/tenure/auditor-2023`. */
export function entryPath(id: string): string {
  return `/entry/${id.replace(/\.yml$/, '')}`
}

/** The route parameter for a node, which is its id without the extension. */
export function entrySlug(id: string): string {
  return id.replace(/\.yml$/, '')
}

/**
 * A corpus link target as a site path, or undefined where this site has no such page.
 *
 * Node files are linked relatively from inside the corpus (`../division/x.yml`). Catalog
 * sources are linked the same way but are not published as pages here, so they resolve to
 * nothing and `prose.inline` leaves them as plain words rather than as a link to nowhere.
 */
export function resolveCorpusLink(target: string): string | undefined {
  if (target.includes('/catalog/') || target.startsWith('catalog/')) return undefined

  const match = /(?:\.\.\/)*([a-z-]+)\/([^/]+)\.yml$/.exec(target)
  if (!match) return undefined

  const [, cls, name] = match
  if (!(cls in CLASS_PRESENTATION)) return undefined
  if (!nodes.some((n) => n.id === `${cls}/${name}.yml`)) return undefined

  return `/entry/${cls}/${name}`
}

export interface Relation {
  edge: Edge
  /** Which way the edge points relative to the node being read. */
  direction: 'out' | 'in'
  /** The node at the other end, when this site publishes it. */
  other: Node | undefined
  otherId: string
}

/** Every edge touching a node, outgoing first, each with the node at its far end. */
export function relationsFor(id: string): Relation[] {
  const out: Relation[] = edges
    .filter((e) => e.from === id)
    .map((e) => ({
      edge: e,
      direction: 'out' as const,
      other: nodes.find((n) => n.id === e.to),
      otherId: e.to,
    }))

  const incoming: Relation[] = edges
    .filter((e) => e.to === id)
    .map((e) => ({
      edge: e,
      direction: 'in' as const,
      other: nodes.find((n) => n.id === e.from),
      otherId: e.from,
    }))

  return [...out, ...incoming]
}

/**
 * The catalog sources a node's prose cites, in first-appearance order.
 *
 * The corpus writes its citations inline as markdown links into `catalog/`. Collecting them
 * gives the entry a source list without a second feed: the sources are already in the prose,
 * and a list assembled from anywhere else could disagree with what the paragraphs say.
 */
export function sourcesFor(node: Node): string[] {
  const seen: string[] = []
  for (const block of node.blocks) {
    for (const match of block.text.matchAll(/\[([^\]]+)\]\(([^)]*catalog\/[^)]+)\)/g)) {
      const path = match[2].replace(/^.*catalog\//, 'catalog/')
      if (!seen.includes(path)) seen.push(path)
    }
  }
  return seen
}

/** Nodes of one class, by label. */
export function nodesOfClass(cls: string): Node[] {
  return nodes.filter((n) => n.class === cls).toSorted((a, b) => a.label.localeCompare(b.label))
}

/** The badge a tier earns: its word, its tone and its glyph. */
export function tierBadge(tier: Tier): { label: string; glyph: string } {
  if (tier === 'verified') return { label: 'Verified', glyph: 'circle-check' }
  if (tier === 'inference') return { label: 'Inference', glyph: 'info' }
  return { label: 'Open', glyph: 'triangle-alert' }
}

/**
 * A property key as a label: `how_began` → `How began`.
 *
 * The corpus writes keys in snake case and the design sets them as small-caps labels, so the
 * underscore has to go before the tracking is applied or the label reads as code.
 */
export function propertyLabel(key: string): string {
  const word = key.replace(/_/g, ' ')
  return word.charAt(0).toUpperCase() + word.slice(1)
}

/* ------------------------------------------------------------------------- *
 * Holdings: the tenure / office / person triangle.
 *
 * Three of the boards draw the same relation from different corners. A tenure
 * shows every holding of its office; an office shows its holders; a person
 * shows the offices they held. One derivation, three viewpoints — and the
 * tenure is always the row, because the tenure is the thing that has dates.
 * ------------------------------------------------------------------------- */

export interface Holding {
  tenure: Node
  person: Node | undefined
  office: Node | undefined
  began: string
  ended: string
  howBegan: string
  howEnded: string
}

function holdingOf(tenure: Node): Holding {
  const heldBy = edges.find((e) => e.from === tenure.id && e.relationship === 'held-by')
  const ofOffice = edges.find((e) => e.from === tenure.id && e.relationship === 'of-office')
  return {
    tenure,
    person: nodes.find((n) => n.id === heldBy?.to),
    office: nodes.find((n) => n.id === ofOffice?.to),
    began: tenure.properties.began ?? '',
    ended: tenure.properties.ended ?? '',
    howBegan: tenure.properties.how_began ?? '',
    howEnded: tenure.properties.how_ended ?? '',
  }
}

/**
 * Every holding relevant to a node, oldest first.
 *
 * `seat` — the other holdings of this tenure's office, this one among them.
 * `holders` — the holdings of this office.
 * `offices` — the holdings by this person.
 */
export function holdingsFor(node: Node, view: 'seat' | 'holders' | 'offices'): Holding[] {
  let tenureIds: string[] = []

  if (view === 'offices') {
    tenureIds = edges
      .filter((e) => e.relationship === 'held-by' && e.to === node.id)
      .map((e) => e.from)
  } else {
    const officeId =
      view === 'holders'
        ? node.id
        : edges.find((e) => e.from === node.id && e.relationship === 'of-office')?.to
    if (!officeId) return []
    tenureIds = edges
      .filter((e) => e.relationship === 'of-office' && e.to === officeId)
      .map((e) => e.from)
  }

  return tenureIds
    .map((id) => nodes.find((n) => n.id === id))
    .filter((n): n is Node => Boolean(n))
    .map(holdingOf)
    .toSorted((a, b) => a.began.localeCompare(b.began))
}

/** The four-digit year at the head of a corpus date, if it carries one. */
export function year(date: string): number | null {
  const match = /(\d{4})/.exec(date)
  return match ? Number(match[1]) : null
}

/**
 * A coordinate pair out of a corpus property.
 *
 * Some of these fields are a bare pair and some carry a sentence after it —
 * `natural-feature.mouth` gives the coordinates, then the county, then a claim
 * tag — so the numbers are taken from the head of the string and the rest is
 * left for the properties grid to print as written.
 */
export function parseCoords(value: string | undefined): { lat: number; lon: number } | undefined {
  if (!value) return undefined
  const match = /^\s*(-?\d+(?:\.\d+)?)\s*,\s*(-?\d+(?:\.\d+)?)/.exec(value)
  if (!match) return undefined
  return { lat: Number(match[1]), lon: Number(match[2]) }
}

/**
 * External identifiers, which the design keeps in their own card.
 *
 * They are kept apart from the other properties because they are keys into
 * somebody else's system, and because two of them can disagree about what they
 * name: a GNIS id names the ground, a GEOID keys the incorporated place.
 */
const KEY_FIELDS: Record<string, string> = {
  gnis_id: 'GNIS',
  geoid: 'GEOID',
  fips_code: 'FIPS',
  huc_code: 'HUC',
  nrhp_id: 'NRHP',
  identifier: 'ID',
}

export function keysFor(node: Node): Array<{ label: string; value: string }> {
  return Object.entries(KEY_FIELDS)
    .filter(([field]) => node.properties[field])
    .map(([field, label]) => ({ label, value: node.properties[field] }))
}

/** Alias strings a node carries, however its class spells the field. */
export function aliasesFor(node: Node): string[] {
  const raw = node.properties.also_known_as ?? node.properties.former_names
  if (!raw) return []
  return raw
    .split(',')
    .map((s) => s.trim())
    .filter(Boolean)
}
