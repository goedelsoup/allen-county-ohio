// Plotly, driven from a semantic spec in the DOM.
//
// The page emits what a chart *means* — a line with named series, a bar with categories —
// and this module resolves it against the live theme. Colours therefore come from the CSS
// custom properties at render time rather than being baked into the build, which is what
// lets one build serve both modes and follow a viewer who changes theme mid-visit.

import Plotly from 'plotly.js-basic-dist-min'
import type { BarSpec, ChartSpec, LineSpec } from '../lib/chart-spec'

export type { BarSpec, ChartSpec, LineSpec }

interface Theme {
  surface: string
  ink: string
  secondary: string
  muted: string
  grid: string
  axis: string
  series: string[]
}

function theme(): Theme {
  const s = getComputedStyle(document.documentElement)
  const v = (name: string) => s.getPropertyValue(name).trim()
  return {
    surface: v('--surface-card'),
    ink: v('--text-strong'),
    secondary: v('--text-body'),
    muted: v('--text-muted'),
    grid: v('--rule-hairline'),
    axis: v('--rule-strong'),
    // Fixed order, never cycled. Two series is the most any chart here carries.
    series: [v('--series-1'), v('--series-2')],
  }
}

const FONT = 'system-ui, -apple-system, "Segoe UI", sans-serif'

function baseLayout(t: Theme): Record<string, unknown> {
  return {
    paper_bgcolor: t.surface,
    plot_bgcolor: t.surface,
    font: { family: FONT, size: 13, color: t.secondary },
    margin: { l: 64, r: 24, t: 8, b: 48 },
    hovermode: 'closest',
    hoverlabel: {
      bgcolor: t.surface,
      bordercolor: t.axis,
      font: { family: FONT, size: 13, color: t.ink },
    },
    showlegend: false,
  }
}

function axis(t: Theme, title?: string, tickSuffix?: string) {
  return {
    title: title ? { text: title, font: { size: 12, color: t.muted } } : undefined,
    // Solid hairlines, one shade off the surface. Never dashed.
    gridcolor: t.grid,
    griddash: 'solid',
    zeroline: false,
    linecolor: t.axis,
    ticks: 'outside',
    tickcolor: t.axis,
    ticklen: 4,
    tickfont: { size: 12, color: t.muted },
    ticksuffix: tickSuffix ?? '',
    automargin: true,
  }
}

function lineChart(spec: LineSpec, t: Theme) {
  const multi = spec.series.length > 1

  const traces = spec.series.map((s, i) => ({
    type: 'scatter',
    mode: 'lines+markers',
    name: s.name,
    x: spec.x,
    y: s.y,
    line: { color: t.series[i], width: 2, shape: 'linear' },
    marker: {
      color: t.series[i],
      size: 8,
      // A 2px surface ring rather than a border, so overlapping points stay separable.
      line: { color: t.surface, width: 2 },
    },
    hovertemplate: `%{y}${spec.tickSuffix ?? ''} · %{x}<extra>${s.name}</extra>`,
  }))

  // Selective direct labels: the last point of each series and nothing else. With more than
  // one series the label is the series name, so identity never rests on hue alone; with one
  // series the legend is suppressed and the label carries the closing value instead.
  const annotations = spec.labelLast
    ? spec.series.flatMap((s) => {
        const last = s.y.reduce<number>((acc, y, j) => (y === null ? acc : j), -1)
        if (last < 0) return []
        const value = s.y[last] as number
        return [
          {
            // The category *index*, not its label. On a category axis Plotly coerces a
            // string `x` to a number, so anchoring to "2024" silently sets the axis range
            // to 2024 categories and squeezes every point into the first pixel of the plot.
            x: last,
            y: value,
            text: multi ? s.name : `${value.toLocaleString('en-US')}${spec.tickSuffix ?? ''}`,
            showarrow: false,
            xanchor: 'left',
            xshift: 10,
            font: { size: 12, color: t.secondary, family: FONT },
          },
        ]
      })
    : []

  return {
    traces,
    layout: {
      ...baseLayout(t),
      // Room on the right for the direct labels rather than clipping them.
      margin: { l: 64, r: spec.labelLast ? 108 : 24, t: 8, b: 48 },
      annotations,
      xaxis: { ...axis(t, spec.xTitle), type: 'category' },
      yaxis: {
        ...axis(t, spec.yTitle, spec.tickSuffix),
        rangemode: spec.zeroBased ? 'tozero' : 'normal',
      },
      showlegend: multi,
      legend: {
        orientation: 'h',
        y: -0.22,
        x: 0,
        font: { size: 12, color: t.secondary },
      },
      hovermode: 'x unified',
    },
  }
}

function barChart(spec: BarSpec, t: Theme) {
  return {
    traces: [
      {
        type: 'bar',
        x: spec.categories,
        y: spec.values,
        // One series, one colour. A value-ramp across nominal categories would double-encode
        // bar length as hue and buy nothing.
        marker: { color: t.series[0] },
        width: 0.62,
        text: spec.labelValues
          ? spec.values.map((v) => `${v.toLocaleString('en-US')}${spec.tickSuffix ?? ''}`)
          : undefined,
        textposition: 'outside',
        textfont: { color: t.secondary, size: 12, family: FONT },
        cliponaxis: false,
        hovertemplate: `%{x}: %{y}${spec.tickSuffix ?? ''}<extra></extra>`,
      },
    ],
    layout: {
      ...baseLayout(t),
      margin: { l: 64, r: 24, t: 24, b: 56 },
      bargap: 0.38,
      xaxis: { ...axis(t), type: 'category' },
      yaxis: { ...axis(t, spec.yTitle, spec.tickSuffix), rangemode: 'tozero' },
    },
  }
}

function build(spec: ChartSpec, t: Theme) {
  return spec.kind === 'line' ? lineChart(spec, t) : barChart(spec, t)
}

const CONFIG = { displayModeBar: false, responsive: true, doubleClick: false as const }

/** Render every chart on the page, and keep them following the viewer's theme. */
export function renderCharts(): void {
  const specs = new Map<HTMLElement, ChartSpec>()

  for (const holder of document.querySelectorAll<HTMLScriptElement>('script[data-chart-spec]')) {
    const id = holder.dataset.chartSpec
    if (!id) continue
    const el = document.getElementById(id)
    if (!el || !holder.textContent) continue
    specs.set(el, JSON.parse(holder.textContent) as ChartSpec)
  }

  const draw = () => {
    const t = theme()
    for (const [el, spec] of specs) {
      const { traces, layout } = build(spec, t)
      void Plotly.newPlot(el, traces, layout, CONFIG)
    }
  }

  draw()

  const scheme = window.matchMedia('(prefers-color-scheme: dark)')
  scheme.addEventListener('change', draw)
  new MutationObserver(draw).observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['data-theme'],
  })
}
