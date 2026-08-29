// @ts-check
import { defineConfig } from 'astro/config'

// A static site. There is no server and no runtime data access: every page is rendered at
// build time from `src/feeds/`, which `crates/publish` wrote and the publication gate
// checked. Nothing here can reach the corpus, which is the point.
export default defineConfig({
  site: 'https://example.invalid/allen-county',
  output: 'static',
  build: { format: 'directory' },
  vite: {
    // Plotly is large and pulled in only by the chart island; keeping it in its own chunk
    // means the map page does not pay for it.
    build: { chunkSizeWarningLimit: 4096 },
  },
})
