import { describe, expect, it } from 'vitest'
import { classOf, quantileBreaks } from '../src/scripts/map'

// The choropleth's classing, tested away from the canvas. Everything else in `map.ts` is
// deck.gl configuration and is exercised by rendering the page.

describe('quantile classing', () => {
  it('splits values so each class holds about the same count', () => {
    const values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    const breaks = quantileBreaks(values, 5)
    expect(breaks).toHaveLength(4)
    expect(breaks).toEqual(breaks.toSorted((a, b) => a - b))
  })

  it('does not collapse a skewed distribution into one class', () => {
    // Equal-interval breaks on this shape put nearly everything in the lightest class and
    // say nothing. That is the failure this function exists to avoid.
    const skewed = [...Array(70).fill(300), ...Array(18).fill(4000)]
    const breaks = quantileBreaks(skewed, 5)
    const counts = [0, 0, 0, 0, 0]
    for (const v of skewed) counts[classOf(v, breaks)]++
    expect(counts.filter((c) => c > 0).length).toBeGreaterThan(1)
  })

  it('puts a value below the first break in the lightest class', () => {
    expect(classOf(0, [10, 20, 30, 40])).toBe(0)
  })

  it('puts the largest value in the darkest class', () => {
    expect(classOf(100, [10, 20, 30, 40])).toBe(4)
  })

  it('puts a value exactly on a break in the lower class', () => {
    expect(classOf(10, [10, 20, 30, 40])).toBe(0)
  })

  it('survives an empty layer rather than throwing inside a render', () => {
    expect(quantileBreaks([], 5)).toEqual([])
    expect(classOf(5, [])).toBe(0)
  })
})
