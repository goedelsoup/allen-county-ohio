// `plotly.js-basic-dist-min` ships no types. The surface this site uses is three functions.
declare module 'plotly.js-basic-dist-min' {
  const Plotly: {
    newPlot(
      el: HTMLElement,
      data: unknown[],
      layout?: Record<string, unknown>,
      config?: Record<string, unknown>,
    ): Promise<unknown>
    purge(el: HTMLElement): void
    Plots: { resize(el: HTMLElement): void }
  }
  export default Plotly
}
