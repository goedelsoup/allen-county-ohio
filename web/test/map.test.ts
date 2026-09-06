import { readFileSync } from 'node:fs'
import { join } from 'node:path'
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

describe('the layer that may not be reached for', () => {
  /**
   * `TripsLayer` animates a position along a path over time. That is precisely the tween
   * `an-animation-asserts-continuity` refuses — it would draw the storm at a position, at a
   * moment, that no source recorded, in the one encoding that carries no tag and affords no
   * tooltip.
   *
   * The refusal is written down in `hops-counts-edges-not-coordinates`, and a refusal that is
   * merely not implemented gets implemented by the next person who sees the gap. So it is
   * checked where it can actually be broken: the package it lives in stays out of the manifest,
   * and the map does not import it under any other name.
   */
  const manifest = JSON.parse(readFileSync(join(import.meta.dirname, '../package.json'), 'utf8'))
  const source = readFileSync(join(import.meta.dirname, '../src/scripts/map.ts'), 'utf8')

  it('is not a dependency', () => {
    const declared = Object.keys({
      ...(manifest.dependencies ?? {}),
      ...(manifest.devDependencies ?? {}),
    })
    expect(declared).not.toContain('@deck.gl/geo-layers')
  })

  it('is not imported by the map', () => {
    // Read off the import statements rather than the whole file, because the file argues at
    // length about why the layer is refused and a substring search would find the argument.
    const imports = [...source.matchAll(/^import[^;]*?from\s+'([^']+)'/gm)].map((m) => m[1])
    expect(imports).not.toContain('@deck.gl/geo-layers')
    const named = [...source.matchAll(/^import\s*\{([^}]*)\}\s*from/gm)].flatMap((m) =>
      m[1].split(',').map((n) => n.trim().replace(/^type\s+/, '')),
    )
    expect(named).not.toContain('TripsLayer')
    // Proof the reader above sees a multi-line import block at all, so the two refusals are
    // absences it looked for rather than absences it could not have found.
    expect(imports).toContain('@deck.gl/layers')
    expect(named).toContain('PathLayer')
  })

  it('has not been replaced by a transition on a track', () => {
    // deck.gl's own tweening lives on `transitions:`, which interpolates a layer's accessors
    // between renders. On the track layers that is the sweep by another route.
    const tracks = source.slice(source.indexOf("id: 'tracks'"), source.indexOf("id: 'claims'"))
    expect(tracks.length).toBeGreaterThan(0)
    expect(tracks).not.toContain('transitions')
  })
})
