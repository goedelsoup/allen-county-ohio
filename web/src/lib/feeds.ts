// The only data this site has.
//
// Every page reads these four files and nothing else. They are written by `crates/publish`
// from `.yidam/corpus/`, and they carry a `feed_version` so a change to how a page renders
// and a change to the ontology stay separable — which is the coupling `web/README.md` asked
// to be settled before any page existed.

import atlasFeed from '../feeds/atlas.json'
import graphFeed from '../feeds/graph.json'
import manifestFeed from '../feeds/manifest.json'
import mapFeed from '../feeds/map.json'
import seriesFeed from '../feeds/series.json'

/** The three claim tags, strongest first. */
export type Tier = 'verified' | 'inference' | 'open'

export interface Block {
  text: string
  tier: Tier
}

export interface Node {
  id: string
  class: string
  label: string
  tier: Tier
  properties: Record<string, string>
  blocks: Block[]
  refusals: string[]
  withheld: number
}

export interface Edge {
  from: string
  to: string
  relationship: string
  tier: Tier
  source: string | null
}

export interface Point {
  node: string
  label: string
  as_of: string
  value: number
  published: string
  tier: Tier
  source: string | null
  method: string | null
}

export interface Series {
  id: string
  subject: string
  subject_label: string
  parameter: string
  unit: string | null
  tier: Tier
  points: Point[]
}

/** One figure set beside another, with the judgement the corpus recorded about the pair. */
export interface ComparabilityRow {
  node: string
  label: string
  as_of: string
  /** The figure as the corpus published it, unrounded and underived. */
  published: string
  unit: string | null
  /** True on the row for the entry's own figure. */
  this_figure: boolean
  /** `null` on the entry's own row, which is not judged against itself. */
  comparable: boolean | null
  /** Why. Never null where `comparable` is set — `edge-audit` fails on a judgement with no reason. */
  because: string | null
  /** The **judgement's** tag, not the figure's. */
  tier: Tier | null
  source: string | null
}

/**
 * A measure's own rows, where it has many and they are not a time series.
 *
 * Cells are the strings the corpus published, in the order it published them — nothing in the
 * feed parses them, so a consumer that wants numbers does the reading and owns it. `NA` is how
 * the corpus writes an absent cell.
 */
export interface MeasureTable {
  node: string
  label: string
  /** Column names in order. The first names the key column. */
  columns: string[]
  /** One row per key. Every row has as many cells as there are columns. */
  rows: string[][]
  tier: Tier
}

/**
 * A measure's comparability table.
 *
 * Not a series. A series groups measures that share a `parameter` string and a subject, which is
 * a mechanical join; this is the set of figures somebody wrote down a judgement about, and said
 * why. A measure nobody judged has no entry here at all.
 */
export interface Comparability {
  node: string
  rows: ComparabilityRow[]
}

export interface Citation {
  node: string
  node_label: string
  span: string
  tier: Tier
  sources: string[]
}

export interface PlottedFigure {
  label: string
  value: number
  literal: string
}

export interface Assertion {
  id: string
  statement: string
  topic: string
  tier: Tier
  citations: Citation[]
  /** Refusals the cited nodes make, carried verbatim rather than routed around. */
  caveats: string[]
  figures: PlottedFigure[]
}

export interface MapPoint {
  id: string
  class: string
  label: string
  tier: Tier
  lat: number
  lon: number
  geoid: string | null
  kind: string | null
  area_sq_mi: string | null
}

/** How much of a date the source gave. */
export type Precision = 'year' | 'month' | 'day'

/**
 * What an absent end date means for a node's class.
 *
 * Four readings, because the corpus has four kinds of silence — see
 * `.yidam/decisions/an-absent-end-date-means-four-things.yml`. Absent on a record whose span
 * is bounded at both ends, and on one that has no span at all.
 */
export type OpenEnd = 'instantaneous' | 'running' | 'unvouched' | 'unknown'

/**
 * What the map should do with a node.
 *
 * `count` is drawn on its subject at whatever grain that subject is; `register` is a node the
 * corpus places no finer than the county, which is listed beside the map rather than pinned in
 * a field. See `.yidam/decisions/a-derived-placement-is-a-claim.yml`.
 */
export type Treatment = 'mark' | 'polygon' | 'count' | 'register' | 'unplaced' | 'not-spatial'

/** One edge followed to reach ground. */
export interface AtlasStep {
  relationship: string
  to: string
  /** `null` on a structural edge, which carries no tag by rule. */
  tier: Tier | null
}

/** One position a node reached, and the route that reached it. */
export interface AtlasAnchor {
  node: string
  lat: number | null
  lon: number | null
  /** A Census key where the anchor is a shape. Joined to `public/geo/` by the site. */
  geoid: string | null
  /** The summary level the key belongs to, without which it identifies nothing — see {@link censusKey}. */
  level: string | null
  /** Empty where the node states its own position. */
  via: AtlasStep[]
}

/**
 * How a Census shape is addressed on this site: the summary level, then the key.
 *
 * **A GEOID is unique only within its summary level.** This county holds the proof — `3904752`
 * is Beaverdam village and also the Upper Scioto Valley Local School District, both seven
 * digits, both vendored in `public/geo/`. Indexing shapes by bare GEOID draws a village of 319
 * people as a school district spanning two counties, silently and with no error anywhere.
 *
 * So every index over vendored geometry is keyed through here, and a feature that cannot name
 * its level is not addressable rather than addressable-by-luck.
 */
export function censusKey(level: string | null | undefined, geoid: string | null | undefined): string | null {
  return level && geoid ? `${level}:${geoid}` : null
}

/**
 * A node reduced to when it was and where it lands.
 *
 * Every node appears, including the undated and the unplaced: the shape of what this corpus
 * cannot date or cannot place is a subject in its own right, and a feed that dropped those
 * rows would make it undiscoverable.
 */
export interface AtlasRecord {
  node: string
  class: string
  label: string
  tier: Tier
  /** First year the corpus places it in. */
  from: number | null
  /**
   * Last year, where there is one. **Null on every open end**, including the ones read as
   * running to the present — "now" is not a date any source recorded.
   */
  to: number | null
  precision: Precision | null
  open_end: OpenEnd | null
  /** Why the node carries no span, where it carries none. */
  undated: string | null
  treatment: Treatment
  /** Edges followed to reach ground. `0` is a stated position. */
  hops: number | null
  /** The weakest tag on the placement route. */
  route_tier: Tier | null
  /**
   * Every anchor reached at `hops`. More than one is not a defect: a district serving five
   * townships is at all five, and averaging them would invent a centroid.
   */
  anchors: AtlasAnchor[]
}

/**
 * What a class declares itself to be.
 *
 * Read from `manifest.json`, which carries `.yidam/corpus/<class>.ont.yml` across the feed
 * boundary. Nothing here is derived from the instances: it is the corpus's own statement of
 * its ontology, and this site renders it rather than keeping a second copy.
 */
export interface ClassSchema {
  label: string
  /** The ontology the foundational type is drawn from — `ufo` throughout this corpus. */
  ontology: string
  /** `kind`, `role`, `relator`, `event`, `quality` or `situation`. */
  foundational_type: string
  edge_policy: string
  /** Properties a node of this class may not omit, in declaration order. */
  required: string[]
  /** The class's own account of why it exists. */
  description: string
}

export interface Manifest {
  feed_version: number
  policy: { ceiling: Tier; rationale: string }
  corpus: {
    nodes: number
    nodes_published: number
    blocks: number
    blocks_published: number
    blocks_untagged: number
    blocks_withheld: number
    properties_withheld: number
    edges: number
    edges_published: number
    assertions: number
    by_class: Record<string, number>
    edge_tags: Record<string, Record<string, number>>
  }
  classes: Record<string, ClassSchema>
}

export const manifest = manifestFeed as Manifest

/** The declared ontology, keyed by class. */
export const classes = manifest.classes
export const nodes = graphFeed.nodes as Node[]
export const edges = graphFeed.edges as Edge[]
export const series = seriesFeed.series as Series[]
export const tables = (seriesFeed.tables ?? []) as MeasureTable[]
export const comparability = seriesFeed.comparability as Comparability[]
export const assertions = seriesFeed.assertions as Assertion[]
export const mapPoints = mapFeed.points as MapPoint[]
export const atlas = atlasFeed.records as AtlasRecord[]

/** The feed contract every page here was written against. */
export const FEED_VERSION = manifest.feed_version

/** One assertion by id. Throws rather than rendering a page with a hole in it. */
export function assertion(id: string): Assertion {
  const found = assertions.find((a) => a.id === id)
  if (!found) {
    throw new Error(
      `no assertion "${id}" in the feed — it was withdrawn, or it failed the publication gate`,
    )
  }
  return found
}

/** Every assertion supporting one view. */
export function assertionsFor(topic: string): Assertion[] {
  return assertions.filter((a) => a.topic === topic)
}

/** One series by id. */
export function seriesById(id: string): Series {
  const found = series.find((s) => s.id === id)
  if (!found) throw new Error(`no series "${id}" in the feed`)
  return found
}

/**
 * The comparability table for one measure, or undefined where the corpus has judged nothing.
 *
 * Undefined is the common case and is not a gap to fill in with a series. It says the corpus
 * has not decided whether this figure may be set beside any other, which is a different thing
 * from deciding that it may.
 */
export function comparabilityFor(id: string): Comparability | undefined {
  return comparability.find((c) => c.node === id)
}

/** A measure's declared table, or undefined where it declares none. */
export function tableFor(id: string): MeasureTable | undefined {
  return tables.find((t) => t.node === id)
}

/**
 * One column of a table, keyed by the first column and parsed as a number.
 *
 * Cells the corpus wrote as `NA` are absent from the map rather than present as zero — a tract
 * with no priced originations has no higher-priced share, and drawing it in the lightest class
 * would say it had the lowest one.
 */
export function column(table: MeasureTable | undefined, name: string): Map<string, number> {
  const out = new Map<string, number>()
  if (!table) return out
  const i = table.columns.indexOf(name)
  if (i < 1) return out
  for (const row of table.rows) {
    const value = Number(row[i])
    if (Number.isFinite(value)) out.set(row[0], value)
  }
  return out
}

/** What one class declares itself to be, or undefined if the corpus does not declare it. */
export function classSchema(cls: string): ClassSchema | undefined {
  return classes[cls]
}

/** One atlas record by `class/name.yml`. */
export function atlasRecord(id: string): AtlasRecord | undefined {
  return atlas.find((r) => r.node === id)
}

/**
 * Every record the corpus places in `year`.
 *
 * `vouched` is the strict reading: a record whose span admits the year but whose class
 * declines to carry it forward is excluded. Passing `false` gives the generous one. The two
 * are separate questions and separate lists — a division effective from 2020 admits 2024 and
 * vouches for nothing there — and merging them invents a fact.
 */
export function atlasAt(year: number, vouched = true): AtlasRecord[] {
  return atlas.filter((r) => {
    if (r.from === null) return false
    if (year < r.from) return false
    if (r.to !== null) return year <= r.to
    // Open-ended. `running` carries forward; the other two readings support their start year
    // and nothing after it.
    if (r.open_end === 'running') return true
    if (r.open_end === 'instantaneous') return false
    return vouched ? year === r.from : true
  })
}

/** One node by `class/name.yml`. */
export function node(id: string): Node | undefined {
  return nodes.find((n) => n.id === id)
}
