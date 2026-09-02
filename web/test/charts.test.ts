import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { bar, line } from '../src/lib/chart-spec'

const PAGES = join(import.meta.dirname, '../src/pages')

/**
 * Every `.astro` page, walked rather than listed.
 *
 * `readdirSync` on the top level was right when every page was one. It stopped being right the
 * day thirty-seven articles moved into `src/pages/read/`, and it would have stopped silently:
 * the checks below would have kept passing over a directory they no longer saw. That is the
 * same shape as the failure they exist to catch — `fix: forty-four charts on six pages that
 * were never drawing`.
 */
function pageFiles(dir = PAGES, prefix = ''): { name: string; source: string }[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory()
      ? pageFiles(join(dir, e.name), `${prefix}${e.name}/`)
      : e.name.endsWith('.astro')
        ? [{ name: prefix + e.name, source: readFileSync(join(dir, e.name), 'utf8') }]
        : [],
  )
}

describe('building a chart spec', () => {
  it('gives a bar the two fields the renderer reads', () => {
    // The whole reason this module exists. `charts.ts` draws a bar from `categories` and
    // `values`; six charts were written with a line's `x` and `series`, annotated `BarSpec`,
    // and drew a trace over two undefined arrays. Nothing failed — the Astro build does not
    // typecheck frontmatter, and the table under each chart was built from the assertion's
    // figures and stayed correct.
    expect(bar(['1939', '1940'], [14, 51])).toEqual({
      kind: 'bar',
      categories: ['1939', '1940'],
      values: [14, 51],
    })
  })

  it('carries options through without letting them displace the data', () => {
    const spec = bar(['a'], [1], { yTitle: 'Things', tickSuffix: '%', labelValues: true })
    expect(spec.categories).toEqual(['a'])
    expect(spec.values).toEqual([1])
    expect(spec.yTitle).toBe('Things')
  })

  it('gives a line an x and named series', () => {
    const spec = line(['2020'], [{ name: 'County', y: [3] }], { labelLast: true })
    expect(spec.kind).toBe('line')
    expect(spec.x).toEqual(['2020'])
    expect(spec.series[0].name).toBe('County')
  })
})

describe('how pages are allowed to build one', () => {
  // A constructor removes the class of error only while every page uses it. This is the
  // tripwire on that: a hand-rolled object literal is exactly the shape that went wrong, and
  // it is not typechecked anywhere in this build.
  it('no page writes a spec literal by hand', () => {
    const offenders = pageFiles()
      .filter(({ source }) => /kind:\s*'(bar|line)'/.test(source))
      .map(({ name }) => name)
    expect(offenders).toEqual([])
  })

  it('every page that plots imports the constructors from the module without Plotly in it', () => {
    // A value import from `scripts/charts` pulls Plotly into the server build, where it
    // throws on `self is not defined`. That is why the specs were type-imported, and why the
    // constructors had to move somewhere Plotly is not.
    const plotting = pageFiles().filter(({ source }) => /(?<![\w.])(bar|line)\(/.test(source))
    expect(plotting.length).toBeGreaterThan(0)
    for (const { name, source } of plotting) {
      expect(source, name).toMatch(/from '\.{2}(\/\.{2})?\/lib\/chart-spec'/)
      expect(source, name).not.toMatch(
        /import \{[^}]*\b(bar|line)\b[^}]*\} from '\.{2}(\/\.{2})?\/scripts\/charts'/,
      )
    }
  })
})

describe('a page that plots also draws', () => {
  /**
   * A spec with no renderer is an empty div of the right height, and no gate notices. That
   * happened once across six pages and forty-four charts; the layouts import `renderCharts` so
   * an article cannot forget, and this is the tripwire on the layouts.
   */
  it('every page with a chart spec reaches renderCharts', () => {
    const LAYOUTS = join(import.meta.dirname, '../src/layouts')
    const layoutSource = readdirSync(LAYOUTS)
      .filter((f) => f.endsWith('.astro'))
      .map((f) => readFileSync(join(LAYOUTS, f), 'utf8'))

    for (const { name, source } of pageFiles()) {
      if (!/data-chart-spec=/.test(source)) continue
      const ownImport = /renderCharts/.test(source)
      // an article draws through its layout; a section page draws for itself
      const viaLayout = /<Article\b/.test(source) && layoutSource.some((l) => /renderCharts/.test(l))
      expect(ownImport || viaLayout, name).toBe(true)
    }
  })
})
