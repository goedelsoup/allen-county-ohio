// Re-vendor the Atlas webfonts into web/public/fonts/, and write the @font-face sheet.
//
// Run by hand, like `fetch-boundaries.mjs` and for the same reason: it reaches the network and
// its output is committed, so no build and no CI run ever needs fonts.gstatic.com to be
// reachable. That is the whole point of the exercise —
//
//   web/README.md says "Nothing here reaches the network at build time or at read time."
//
// which was false for as long as `Base.astro` carried a Google Fonts <link>. Every reader's
// browser announced itself to a third party on every page load, and the fonts were the only
// thing that had to be fetched from elsewhere for a page to render as designed: the feeds are
// committed, the geometry is vendored, the build is static.
//
//   mise run fonts
//
// ---- These faces are substitutions ----
//
// The design system's CAVEATS say so plainly: no font binaries were supplied with the brief,
// and the five families were chosen for a letterpress character rather than specified. When
// licensed faces arrive, this script is what gets replaced — the token file it writes names
// the families and nothing else in the site does.
//
// ---- What is fetched, and what is not ----
//
// Latin and latin-ext only. Google serves each family cut into seven unicode-range subsets;
// the county's material is English with the occasional diacritic in a surname, and shipping
// Cyrillic, Greek and Vietnamese would triple the directory to serve nobody. A reader who
// needs them gets the fallback stack, which is what a fallback stack is for.
//
// Styles are the ones the site can actually render, audited rather than assumed:
//
//   IM Fell English   400, and italic — the display face has only these two
//   EB Garamond       400, 500, 600, 700 and 400 italic. 700 is not in any token: it is what
//                     `<strong>` resolves to, and the corpus writes `**bold**` in its prose.
//                     Italic is `<em>`, which the reading pages use throughout.
//   Alegreya Sans     400, 500 — interface text, `--type-ui` is the 500
//   Alegreya Sans SC  400, 500 — labels and eyebrows
//   IBM Plex Mono     400, 500 — data
//
// Anything outside that list synthesises, which for a weight is a slur and for an italic is a
// slant. Both are visible on a serif at reading size, so the list is worth keeping honest.

import { mkdir, readdir, rm, writeFile } from 'node:fs/promises'
import { join } from 'node:path'

const ROOT = join(import.meta.dirname, '..')
const OUT_DIR = join(ROOT, 'public/fonts')
const SHEET = join(ROOT, 'src/styles/tokens/fonts.css')

const SUBSETS = new Set(['latin', 'latin-ext'])

const REQUEST = [
  'IM+Fell+English:ital@0;1',
  'EB+Garamond:ital,wght@0,400;0,500;0,600;0,700;1,400',
  'Alegreya+Sans:wght@400;500',
  'Alegreya+Sans+SC:wght@400;500',
  'IBM+Plex+Mono:wght@400;500',
]

// Google serves woff2 only to a browser that says it can take it. With curl's own agent the
// API answers in truetype, which is three times the bytes for the same glyphs.
const UA =
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'

const slug = (family) => family.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '')

/** Split the API's stylesheet into `@font-face` blocks, each tagged with its subset comment. */
function parse(css) {
  const faces = []
  const pattern = /\/\*\s*([a-z-]+)\s*\*\/\s*@font-face\s*\{([^}]*)\}/g
  for (const [, subset, body] of css.matchAll(pattern)) {
    const field = (name) => new RegExp(`${name}:\\s*([^;]+);`).exec(body)?.[1].trim()
    faces.push({
      subset,
      family: field('font-family')?.replace(/^['"]|['"]$/g, ''),
      style: field('font-style'),
      weight: field('font-weight'),
      range: field('unicode-range'),
      url: /url\(([^)]+)\)/.exec(body)?.[1],
    })
  }
  return faces
}

const css = await fetch(
  `https://fonts.googleapis.com/css2?${REQUEST.map((f) => `family=${f}`).join('&')}&display=swap`,
  { headers: { 'User-Agent': UA } },
).then((r) => {
  if (!r.ok) throw new Error(`Google Fonts answered ${r.status}`)
  return r.text()
})

const faces = parse(css).filter((f) => SUBSETS.has(f.subset))
if (faces.length === 0) throw new Error('no latin faces in the response — the API shape changed')

// Rewritten wholesale rather than merged, so a family dropped from REQUEST leaves no orphan
// behind. An unreferenced binary in a public directory is a file nobody can tell is dead.
await rm(OUT_DIR, { recursive: true, force: true })
await mkdir(OUT_DIR, { recursive: true })

const rules = []
for (const face of faces) {
  // Every file this writes is named `.woff2` and declared `format('woff2')`, so it has to BE
  // woff2. Google decides the format from the User-Agent, and a stale one gets a mixed sheet
  // back — seven `.woff` URLs among sixty-eight, in the case that surfaced this — which the
  // script would then write under a woff2 name with a `wOFF` signature inside. Browsers sniff
  // the bytes and it renders, so nothing visibly breaks; the filename, the declared format and
  // the compression this whole exercise is for are all quietly wrong.
  if (!face.url.endsWith('.woff2')) {
    throw new Error(
      `${face.family} ${face.weight}${face.style === 'italic' ? 'i' : ''} ${face.subset}: ` +
        `the API returned ${face.url} — not woff2. Update UA above.`,
    )
  }
  const name = `${slug(face.family)}-${face.weight}${face.style === 'italic' ? 'i' : ''}-${face.subset}.woff2`
  const bytes = await fetch(face.url).then((r) => {
    if (!r.ok) throw new Error(`${face.url} answered ${r.status}`)
    return r.arrayBuffer()
  })
  // Belt and braces: the URL can end .woff2 and the bytes still not be. wOF2 is the signature.
  if (Buffer.from(bytes.slice(0, 4)).toString('latin1') !== 'wOF2') {
    throw new Error(`${name}: downloaded bytes are not woff2`)
  }
  await writeFile(join(OUT_DIR, name), Buffer.from(bytes))
  rules.push(
    [
      '@font-face {',
      `  font-family: '${face.family}';`,
      `  font-style: ${face.style};`,
      `  font-weight: ${face.weight};`,
      // swap, not optional: the fallback stack is a different serif at a different width, and
      // a reader who has already started reading should not have the line re-flow under them.
      '  font-display: swap;',
      `  src: url('/fonts/${name}') format('woff2');`,
      `  unicode-range: ${face.range};`,
      '}',
    ].join('\n'),
  )
}

const header = `/*
 * Allen County Atlas — the webfonts themselves.
 *
 * GENERATED by \`mise run fonts\` (web/scripts/fetch-fonts.mjs). Do not edit: the next run
 * rewrites it. To change which faces or weights ship, change the audit in that script.
 *
 * NOT PORTED. The design system ships a \`tokens/fonts.css\` of its own, and it was not taken
 * during the port because a Google Fonts <link> covered more weights for less work. That link
 * is gone — it made every reader's browser announce itself to a third party on every page
 * load, on a public civic reference, and it was the only thing the site had to fetch from
 * elsewhere to render as designed. See \`design/departures.md\`.
 *
 * The families are substitutions. The brief supplied no binaries, and swapping these for
 * licensed faces touches this file's generator and \`tokens/typography.css\`, nothing else.
 */

`

await writeFile(SHEET, header + rules.join('\n\n') + '\n')

const files = await readdir(OUT_DIR)
console.log(`${files.length} woff2 files in public/fonts/, ${rules.length} @font-face rules`)
for (const family of new Set(faces.map((f) => f.family))) {
  const cuts = faces.filter((f) => f.family === family)
  const styles = [...new Set(cuts.map((f) => `${f.weight}${f.style === 'italic' ? 'i' : ''}`))]
  console.log(`  ${family.padEnd(18)} ${styles.join(' ')}`)
}
