// The document outline, checked over the built site.
//
//   mise run site-outline        (after site-build; `ci` runs both)
//
// ---- Why this is not a vitest check ----
//
// `41383d3` fixed forty-six skipped heading ranks across thirty-six pages and left two checks
// behind to stop them coming back. Both are in `test/register.test.ts` and both are
// source-level: no heading inside a `<figure>`, and every heading in an article at `h2`. They
// cover the two ways the outline was actually broken. They do not cover the general case, and
// a reading page is not covered at all.
//
// The negative control from the issue: insert `<h4>A rank nothing checks</h4>` after the first
// `<h2>` of `src/pages/history.astro` and the suite reports 289 passed. The build is green, the
// page ships, and the outline jumps two ranks.
//
// The check that found the original forty-six read `dist/**/*.html`, walked the headings in
// document order, and reported every `h{n} → h{m>n+1}` transition. That is the real invariant
// and it is cheap. It needs the built site, which is why it is here and not in `vitest run` —
// `mise run ci` builds after it tests, so this runs at the end where the artifact exists.
//
// The two source-level checks stay. They are subsumed by this one, but they fail faster and
// name the component rather than the symptom, which is worth a few milliseconds.
//
// ---- What it asserts ----
//
//   1. Exactly one <h1> per page.
//   2. No skipped rank, in document order.
//   3. No two <nav> landmarks on a page share an accessible name.
//
// All three are structural, not stylistic: a screen reader's heading list is the table of
// contents for a page that has no other, a rank jumped is a level of that outline that does not
// exist, and two landmarks called the same thing are a rotor entry a reader cannot choose
// between.
//
// The third is here rather than in `vitest` for the reason the other two are, and the reason is
// concrete: `shape.test.ts` asserts the layout emits exactly `Sections` and `Instruments`, and
// that was true while `/read/` added a third `<nav aria-label="Sections">` of its own. A check
// over one source file cannot see what a page composes.

import { existsSync } from 'node:fs'
import { readFile, readdir } from 'node:fs/promises'
import { join, relative } from 'node:path'

const DIST = join(import.meta.dirname, '../dist')

if (!existsSync(DIST)) {
  console.error(`Document outline: ${DIST} does not exist — run the build first.`)
  process.exit(1)
}

/**
 * The redirect stubs carry no `h1` by design.
 *
 * Seven paths were retired when six reading pages replaced twelve topic pages, and a civic
 * reference that 404s a link somebody kept is not much of a reference. Astro emits each as a
 * meta-refresh document — a `<title>`, a `<link rel=canonical>` and nothing to head. They are
 * identified by that markup rather than by a hardcoded list, so an eighth needs no edit here
 * and a page that stops being a redirect stops being exempt.
 */
const isRedirectStub = (html) => /<meta\s+http-equiv=["']refresh["']/i.test(html)

async function* htmlFiles(dir) {
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name)
    if (entry.isDirectory()) yield* htmlFiles(path)
    else if (entry.name.endsWith('.html')) yield path
  }
}

/** Markup a reader's assistive technology actually sees. */
const rendered = (html) =>
  html
    .replace(/<!--[\s\S]*?-->/g, '')
    .replace(/<script[\s\S]*?<\/script>/gi, '')
    .replace(/<template[\s\S]*?<\/template>/gi, '')

/** Heading ranks in document order. */
const headings = (html) => [...rendered(html).matchAll(/<h([1-6])[\s>]/gi)].map((m) => Number(m[1]))

/** The accessible name of every `<nav>`, in document order. */
const navNames = (html) =>
  [...rendered(html).matchAll(/<nav\b[^>]*\saria-label=["']([^"']+)["']/gi)].map((m) => m[1])

const findings = []
let checked = 0

for await (const file of htmlFiles(DIST)) {
  const html = await readFile(file, 'utf8')
  if (isRedirectStub(html)) continue

  const page = relative(DIST, file)
  const ranks = headings(html)
  checked++

  const h1s = ranks.filter((r) => r === 1).length
  if (h1s !== 1) findings.push(`${page}: ${h1s} <h1> (expected exactly one)`)

  for (let i = 1; i < ranks.length; i++) {
    if (ranks[i] > ranks[i - 1] + 1) {
      findings.push(`${page}: h${ranks[i - 1]} → h${ranks[i]} skips a rank`)
    }
  }

  const names = navNames(html)
  for (const name of new Set(names)) {
    const n = names.filter((other) => other === name).length
    if (n > 1) findings.push(`${page}: ${n} <nav> landmarks both named "${name}"`)
  }
}

// A check that reports success over nothing is worse than no check: `dist/` is gitignored and
// is cleared at the start of every build, so "0 pages, no skipped rank" is what this prints
// when it runs too early, runs concurrently with the build, or runs where no build happened.
// Silence about an empty room reads exactly like a clean one.
if (checked === 0) {
  console.error(`\nDocument outline: no pages found under ${DIST} — nothing was checked.\n`)
  process.exit(1)
}

if (findings.length > 0) {
  console.error(`\nDocument outline: ${findings.length} finding(s) across ${checked} pages\n`)
  for (const finding of findings) console.error(`  ${finding}`)
  console.error('')
  process.exit(1)
}

console.log(
  `Document structure: ${checked} pages — one h1 each, no skipped rank, no duplicate landmark name.`,
)
