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

  /*
   * `/map` is not in that list, and the reason is worth stating.
   *
   * It was an instrument for the life of the site and is now the index — nothing about it was
   * withdrawn, it moved. But the view lives in the query string, so `/map?year=1885&at=…` is a
   * link somebody may have kept, and Astro's static redirect emits a fixed
   * `<meta http-equiv="refresh" content="0;url=/">` that drops everything after the `?`. A
   * reader who kept a year would land on the present with no error and no year, which is the
   * quietest possible way to lose a link.
   *
   * So `pages/map.astro` is a hand-written stub that carries the query across. See the argument
   * in that file.
   */
  build: { format: 'directory' },
  vite: {
    // Plotly is large and pulled in only by the chart island; keeping it in its own chunk
    // means the map page does not pay for it.
    build: { chunkSizeWarningLimit: 4096 },
  },
})
