import { describe, expect, it } from 'vitest'
import { classOf, quantileBreaks, stackRadius } from '../src/scripts/map'

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

describe('a stack is sized by area', () => {
  /**
   * Ninety figures about Lima is one mark, not ninety pins, and how big that mark is has to be
   * something a reader can estimate. Area proportional to the count is the encoding they get
   * right; radius proportional to the count overstates a big stack by the square of everything,
   * which on this corpus means Lima swallowing the county.
   */
  it('quadruples the count when the radius doubles', () => {
    const floor = stackRadius(0)
    const one = stackRadius(1) - floor
    const four = stackRadius(4) - floor
    expect(four / one).toBeCloseTo(2, 5)
  })

  it('keeps a single record visible at county zoom', () => {
    expect(stackRadius(1)).toBeGreaterThan(400)
  })

  it('does not go imaginary on a count it should never see', () => {
    expect(Number.isFinite(stackRadius(-3))).toBe(true)
  })
})
