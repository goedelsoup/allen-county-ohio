import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { column, node, tableFor, tables } from '../src/lib/feeds'

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
