// The only data this site has.
//
// Every page reads these four files and nothing else. They are written by `crates/publish`
// from `.yidam/corpus/`, and they carry a `feed_version` so a change to how a page renders
// and a change to the ontology stay separable — which is the coupling `web/README.md` asked
// to be settled before any page existed.

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
export const assertions = seriesFeed.assertions as Assertion[]
export const mapPoints = mapFeed.points as MapPoint[]

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

/** What one class declares itself to be, or undefined if the corpus does not declare it. */
export function classSchema(cls: string): ClassSchema | undefined {
  return classes[cls]
}

/** One node by `class/name.yml`. */
export function node(id: string): Node | undefined {
  return nodes.find((n) => n.id === id)
}
