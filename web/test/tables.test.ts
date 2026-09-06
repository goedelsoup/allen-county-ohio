import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { column, edges, node, nodes, tableFor, tables } from '../src/lib/feeds'

// A measure's declared rows, and the one thing worth gating about them.
//
// The table exists because a coefficient stated over 34 tracts and illustrated by ten of them
// could not be checked or drawn. So the check is that it *can* be checked: the prose states
// 0.828 and two ranges, the property carries the rows, and this recomputes the first from the
// second. Either half edited alone fails here, which is the whole point of publishing both.
//
// Counts of tables are not pinned. A second measure declaring one must not fail this file.

const ACCESS = 'measure/allen-county-mortgage-access-by-tract-2018-2024.yml'

/** The measure's own floor: a tract is rated where its denominator reaches thirty. */
const RATED = 30

const mean = (v: number[]) => v.reduce((a, b) => a + b, 0) / v.length

function pearson(xs: number[], ys: number[]): number {
  const mx = mean(xs)
  const my = mean(ys)
  const cov = xs.reduce((a, x, i) => a + (x - mx) * (ys[i] - my), 0)
  const sx = Math.sqrt(xs.reduce((a, x) => a + (x - mx) ** 2, 0))
  const sy = Math.sqrt(ys.reduce((a, y) => a + (y - my) ** 2, 0))
  return cov / (sx * sy)
}

describe('a declared table', () => {
  it('reaches the site with its columns and rows intact', () => {
    const table = tableFor(ACCESS)
    expect(table, ACCESS).toBeDefined()
    expect(table?.columns[0]).toBe('tract')
    expect(table?.rows).toHaveLength(35)
  })

  it('gives every row as many cells as there are columns', () => {
    // The publisher refuses a ragged table; this is the second half, on the far side of the
    // feed. A shifted row is every number real and every number under the wrong heading.
    for (const table of tables) {
      for (const row of table.rows) {
        expect(row.length, `${table.node} ${row[0]}`).toBe(table.columns.length)
      }
    }
  })

  it('keys every row uniquely', () => {
    for (const table of tables) {
      const keys = table.rows.map((r) => r[0])
      expect(new Set(keys).size, table.node).toBe(keys.length)
    }
  })

  it('is published by a node that exists and carries the same tier', () => {
    for (const table of tables) {
      const owner = node(table.node)
      expect(owner, table.node).toBeDefined()
      expect(table.tier).toBe(owner?.tier)
    }
  })
})

describe('the column reader', () => {
  it('keys on the first column and parses the one asked for', () => {
    const denial = column(tableFor(ACCESS), 'denial_pct')
    expect(denial.get('39003010100')).toBeCloseTo(9.5, 5)
    expect(denial.size).toBe(35)
  })

  it('returns nothing for a column that is not there, and never the key by accident', () => {
    expect(column(tableFor(ACCESS), 'no_such_column').size).toBe(0)
    // Column 0 is the key. Asking for it must not hand back a map of keys parsed as numbers.
    expect(column(tableFor(ACCESS), 'tract').size).toBe(0)
    expect(column(undefined, 'denial_pct').size).toBe(0)
  })
})

describe('the corpus checking itself', () => {
  const table = tableFor(ACCESS)
  const denial = column(table, 'denial_pct')
  const priced = column(table, 'higher_priced_pct')
  const decisions = column(table, 'decisions')
  const pricedN = column(table, 'priced')

  const rated = [...denial.keys()].filter(
    (t) => (decisions.get(t) ?? 0) >= RATED && (pricedN.get(t) ?? 0) >= RATED,
  )

  it('rates 34 of the 35 tracts, on the floor the measure states', () => {
    expect(rated).toHaveLength(34)
    const unrated = [...denial.keys()].filter((t) => !rated.includes(t))
    expect(unrated).toEqual(['39003013700'])
  })

  it('reproduces the correlation its own prose states', () => {
    // The gate this file exists for. `0.828` is read out of the published description rather
    // than typed here, so the two cannot be edited apart: change the rows and this fails,
    // change the sentence and this fails.
    const prose = node(ACCESS)
      ?.blocks.map((b) => b.text)
      .join(' ')
    const stated = /correlation between a tract's denial rate and its share of higher-priced loans is (\d\.\d+)/.exec(
      prose ?? '',
    )
    expect(stated, 'the description no longer states the coefficient').not.toBeNull()

    const r = pearson(
      rated.map((t) => denial.get(t) as number),
      rated.map((t) => priced.get(t) as number),
    )
    expect(r.toFixed(3)).toBe(Number(stated?.[1]).toFixed(3))
  })

  it('reproduces the two ranges its own prose states', () => {
    const prose = node(ACCESS)
      ?.blocks.map((b) => b.text)
      .join(' ')
    const stated =
      /Denial runs from (\d+\.\d) to (\d+\.\d) per cent across those tracts and the higher-priced share from (\d+\.\d) to (\d+\.\d)/.exec(
        prose ?? '',
      )
    expect(stated, 'the description no longer states the ranges').not.toBeNull()

    const d = rated.map((t) => denial.get(t) as number)
    const p = rated.map((t) => priced.get(t) as number)
    expect([Math.min(...d).toFixed(1), Math.max(...d).toFixed(1)]).toEqual([stated?.[1], stated?.[2]])
    expect([Math.min(...p).toFixed(1), Math.max(...p).toFixed(1)]).toEqual([stated?.[3], stated?.[4]])
  })

  it('agrees with the Lima split the same node publishes', () => {
    // Both halves of the split are in the prose and neither is in the table; what the table can
    // check is their total. If a row is ever dropped or duplicated, these stop adding up.
    const houses = column(table, 'houses')
    const loans = column(table, 'loans')
    expect([...houses.values()].reduce((a, b) => a + b, 0)).toBe(14913 + 26595)
    expect([...loans.values()].reduce((a, b) => a + b, 0)).toBe(2958 + 11738)
  })
})

describe('the join to the ground', () => {
  const GEO = join(import.meta.dirname, '..', 'public', 'geo')
  const tractKeys = new Set(
    (
      JSON.parse(readFileSync(join(GEO, 'census-tracts.geojson'), 'utf8')) as {
        features: { properties: { GEOID: string } }[]
      }
    ).features.map((f) => f.properties.GEOID),
  )

  it('keys every row on a tract this site holds the shape of', () => {
    // The map draws this table over `census-tracts.geojson`. A key that is not a vendored tract
    // draws nothing and says nothing, which is the silent half of a bad join.
    const table = tableFor(ACCESS)
    const missing = (table?.rows ?? []).map((r) => r[0]).filter((k) => !tractKeys.has(k))
    expect(missing, 'table rows with no vendored tract').toEqual([])
  })

  it('covers every tract the county has, so the map has no unexplained holes', () => {
    // The other direction. A tract in the ground and not in the table is a gap in the choropleth,
    // and a reader cannot tell one of those from a tract the measure declined to rate.
    const keyed = new Set((tableFor(ACCESS)?.rows ?? []).map((r) => r[0]))
    const uncovered = [...tractKeys].filter((k) => !keyed.has(k))
    expect(uncovered, 'vendored tracts with no row').toEqual([])
  })
})

/** The acts table a jurisdiction's entry draws, found the way `JurisdictionBlock` finds it. */
function actsFor(id: string) {
  return edges
    .filter((e) => e.to === id && e.relationship === 'describes')
    .map((e) => tableFor(e.from))
    .find((tb) => tb && tb.columns[0] !== 'place_fips')
}

describe('the boundary a jurisdiction entry draws', () => {
  // `JurisdictionBlock` joins a municipal corporation to its line in this table on the seven-digit
  // place code, and renders the acts themselves where a second measure holds them. Both halves are
  // gated here, because the failure mode is silence: a municipality whose code stops matching
  // simply loses the section, and nothing else on the page would say so.
  const RECORD = 'measure/allen-county-annexations-1990-2024.yml'

  const perPlace = tableFor(RECORD)
  const municipal = nodes.filter(
    (n) => n.class === 'jurisdiction' && n.properties.jurisdiction_type === 'municipal corporation',
  )

  it('carries a row for every municipal corporation and for nothing else', () => {
    expect(perPlace?.columns[0]).toBe('place_fips')
    const keyed = new Set((perPlace?.rows ?? []).map((r) => r[0]))
    const codes = new Set(municipal.map((n) => n.properties.fips_code))
    expect([...codes].filter((c) => !keyed.has(c)), 'corporations with no row').toEqual([])
    expect([...keyed].filter((k) => !codes.has(k)), 'rows matching no corporation').toEqual([])
    expect(keyed.size).toBe(10)
  })

  it('matches each code to exactly one government, at the level the code belongs to', () => {
    // A seven-digit GEOID is unique only within its summary level — `3904752` is Beaverdam
    // village and also a school district — so the block restricts the join to municipal
    // corporations. This is that restriction being load-bearing rather than decorative.
    for (const row of perPlace?.rows ?? []) {
      const hits = nodes.filter((n) => n.properties.fips_code === row[0])
      expect(hits.length, `${row[0]} matches ${hits.length} nodes`).toBeGreaterThanOrEqual(1)
      const corporations = hits.filter(
        (n) => n.properties.jurisdiction_type === 'municipal corporation',
      )
      expect(corporations, row[0]).toHaveLength(1)
    }
  })

  it('adds up to the count the record publishes', () => {
    const records = column(perPlace, 'records')
    const total = [...records.values()].reduce((a, b) => a + b, 0)
    expect(String(total)).toBe(node(RECORD)?.properties.value)
  })

  it('agrees with the acts, which are a node each', () => {
    // The two grains, cross-checked for every municipality that has both. A summary line says how
    // many acts and how many acres; the acts table has to be that many rows summing to that many
    // acres, and the two are different nodes read from the same files years apart. This is the
    // check that caught nothing and would have caught a mis-keyed row in any of the fifty-five.
    const records = column(perPlace, 'records')
    const acres = column(perPlace, 'acres')
    let covered = 0
    for (const gov of municipal) {
      const table = actsFor(gov.id)
      const code = gov.properties.fips_code
      if (!table) {
        // Only a place with nothing to itemise may be missing one: a single act is its own
        // summary line, and none of the four that reported nothing has anything to hold.
        expect(records.get(code) ?? 0, `${gov.label} has acts and no table`).toBeLessThanOrEqual(1)
        continue
      }
      covered += 1
      expect(table.rows, gov.label).toHaveLength(records.get(code) as number)
      const stated = column(table, 'acres')
      expect([...stated.values()].reduce((a, b) => a + b, 0), gov.label).toBeCloseTo(
        acres.get(code) as number,
        5,
      )
    }
    expect(covered, 'municipalities itemised act by act').toBe(5)
  })

  it('itemises fifty-five of the fifty-seven, and says which two it does not', () => {
    // Cairo and Harrod reported one act each, which their summary line states in full. The other
    // fifty-five are rows in five tables, and this is the arithmetic that says none went missing
    // between the county's count and the nodes that itemise it.
    const itemised = municipal.reduce((n, gov) => n + (actsFor(gov.id)?.rows.length ?? 0), 0)
    const single = municipal.filter((gov) => !actsFor(gov.id) && column(perPlace, 'records').get(gov.properties.fips_code) === 1)
    expect(itemised).toBe(55)
    expect(single.map((g) => g.label).toSorted()).toEqual(['Village of Cairo', 'Village of Harrod'])
    expect(itemised + single.length).toBe(Number(node(RECORD)?.properties.value))
  })

  it('counts the corporations that reported nothing, and says so in its prose', () => {
    // The finding the table made checkable: three of the ten filed no boundary change in
    // thirty-five years. The node said four and named a place that has no corporation at all.
    const at = (name: string) => perPlace!.columns.indexOf(name)
    const silent = (perPlace?.rows ?? []).filter((r) => r[at('records')] === '0')
    expect(silent.map((r) => r[at('place')]).toSorted()).toEqual([
      'Beaverdam',
      'Fort-Shawnee',
      'Lafayette',
    ])
    const prose = node(RECORD)?.blocks.map((b) => b.text).join(' ') ?? ''
    expect(prose).toContain('Three villages that annexed nothing')
    expect(prose).not.toMatch(/Four villages that annexed nothing/)
  })
})
