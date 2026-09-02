// @ts-check
import { defineConfig } from 'astro/config'

// A static site. There is no server and no runtime data access: every page is rendered at
// build time from `src/feeds/`, which `crates/publish` wrote and the publication gate
// checked. Nothing here can reach the corpus, which is the point.
export default defineConfig({
  site: 'https://example.invalid/allen-county',
  output: 'static',

  /*
   * The seven retired paths. Six reading pages replaced twelve topic pages, and a
   * civic reference that returns 404 for a link somebody kept is not much of a
   * reference. Astro emits these as meta-refresh documents on a static build,
   * so they cost one file each and no server.
   *
   * See `.yidam/decisions/a-page-is-an-argument-not-an-inbox.yml`.
   */
  redirects: {
    '/population': '/people',
    '/housing': '/people',
    '/health': '/people',
    '/schools': '/people',
    '/land': '/ground',
    '/elections': '/government',
    '/corpus': '/sources',
  },
  build: { format: 'directory' },
  vite: {
    // Plotly is large and pulled in only by the chart island; keeping it in its own chunk
    // means the map page does not pay for it.
    build: { chunkSizeWarningLimit: 4096 },
  },
})
