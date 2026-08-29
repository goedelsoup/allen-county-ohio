import { getViteConfig } from 'astro/config'

// Vitest through Astro's own Vite config, so a test resolves `src/` and imports the feed
// JSON exactly as a page does. A test suite with a second resolver is a test suite that can
// pass while the site fails to build.
export default getViteConfig({
  test: {
    globals: true,
    environment: 'node',
    include: ['test/**/*.test.ts'],
  },
})
