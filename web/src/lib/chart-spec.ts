// What a chart means, and the only sanctioned way to build one.
//
// This is a separate module from `scripts/charts.ts` for a reason that was load-bearing and
// unwritten. `charts.ts` imports Plotly at the top level, so a page can only ever take a
// *type* from it — `import type { BarSpec }` — because a value import drags Plotly into the
// server build and it throws on `self is not defined`. A type import is erased, which means
// nothing at build time ever looked at a spec's shape, which is how six charts came to be
// annotated `BarSpec` and shaped like a line and rendered empty for two phases.
//
// Constructors cannot be type-erased. Putting them here, with no Plotly, is what lets a page
// import a value and get the shape enforced.

export interface LineSpec {
  kind: 'line'
  x: (string | number)[]
  series: { name: string; y: (number | null)[] }[]
  xTitle?: string
  yTitle?: string
  tickSuffix?: string
  /** Label the last point of each series directly, so identity is never colour alone. */
  labelLast?: boolean
  /** Start the y-axis at zero. Off by default: a decline of 9% on a 0-based axis is a flat line. */
  zeroBased?: boolean
}

export interface BarSpec {
  kind: 'bar'
  categories: string[]
  values: number[]
  yTitle?: string
  tickSuffix?: string
  /** Print each bar's value at its end. Only ever used where there are few enough bars. */
  labelValues?: boolean
}

export type ChartSpec = LineSpec | BarSpec

/**
 * Build a bar spec from the two arrays the renderer actually reads.
 *
 * This exists because the type did not save us. Five charts on the housing page and one on
 * the schools page were written as `{ kind: 'bar', x, series }` — a line's shape, annotated
 * `BarSpec` — and drew a Plotly trace over two `undefined` arrays. Nothing failed. The Astro
 * build does not typecheck frontmatter, the tables under each chart were built from the
 * assertion's figures and stayed correct, and every gate passed over six empty plots for two
 * phases.
 *
 * A constructor cannot be called with the wrong field names. That is the whole fix: the class
 * of error is gone rather than detected, and `charts.test.ts` holds the pages to using it.
 */
export function bar(
  categories: string[],
  values: number[],
  opts: Omit<BarSpec, 'kind' | 'categories' | 'values'> = {},
): BarSpec {
  return { kind: 'bar', categories, values, ...opts }
}

/** Build a line spec. The counterpart to {@link bar}, and required for the same reason. */
export function line(
  x: (string | number)[],
  series: { name: string; y: (number | null)[] }[],
  opts: Omit<LineSpec, 'kind' | 'x' | 'series'> = {},
): LineSpec {
  return { kind: 'line', x, series, ...opts }
}
