// The port is a derivation, so the derivation is the gate.
//
// `web/src/styles/tokens/` was copied out of the Allen County Atlas design system by hand and
// has since been corrected in five places under measurement. For five days nothing recorded
// which values were upstream's and which were this repository's, which meant two failures
// waiting to happen: a later edit could quietly diverge from the system with nobody able to
// tell it from the port, and a re-sync could quietly discard a correction that cost a
// contrast pass to find.
//
// So `design/upstream/` holds what upstream actually says, `design/departures.md` holds every
// place this repository differs and why, and this file holds the two together. A difference
// that is not declared fails. A declaration that no longer describes a difference fails too —
// the same argument `.yidam/lint-baseline.yml` and the publication gate's `answers` list are
// both built on: a list of exceptions permitted to be wrong drifts, and one that over-lists
// silently re-permits whatever a later edit puts in its place.
//
// It compares DECLARATIONS, not bytes. The port rewrote every file header to say where the
// file came from and what it is doing here; a byte diff would be nothing but those comments.

import { createHash } from 'node:crypto'
import { existsSync, readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { describe, expect, it } from 'vitest'
import { blocks, normalise, normaliseSelector } from './tokens'

const DESIGN = join(import.meta.dirname, '../../design')
const MIRROR = join(DESIGN, 'upstream/tokens')
const PORT = join(import.meta.dirname, '../src/styles/tokens')

const pin = readFileSync(join(DESIGN, 'pin.toml'), 'utf8')
const departures = readFileSync(join(DESIGN, 'departures.md'), 'utf8')

const headings = [...departures.matchAll(/^###\s+(.*)$/gm)].map((m) => m[1])
const backticked = (heading: string, pattern: RegExp): string[] =>
  [...heading.matchAll(pattern)].map((m) => m[1])

/** Every token named in a `### …` heading. One heading may name several. */
const declaredTokens = new Set(
  headings.filter((h) => !/^(File|Rule):/.test(h)).flatMap((h) => backticked(h, /`(--[a-z0-9-]+)`/g)),
)

/** Every sheet named in a `### File: …` heading, relative to the tokens directory. */
const declaredFiles = new Set(
  headings.filter((h) => h.startsWith('File:')).flatMap((h) => backticked(h, /`(?:tokens\/)?([\w.-]+\.css)`/g)),
)

/**
 * Rules one side has and the other does not, as `file → selectors`.
 *
 * Keyed by file and matched exactly. Matching a bare selector name as a substring of the scope
 * key exempted far more than it declared: a `code` entry written for `base.css` let through
 * `pre code`, `.code-block`, `code.inline` and a `code` rule appearing in any other sheet.
 */
const declaredRules = new Map<string, Set<string>>()
for (const heading of headings.filter((h) => h.startsWith('Rule:'))) {
  const names = backticked(heading, /`([^`]+)`/g)
  const file = names.find((n) => n.endsWith('.css'))?.replace(/^tokens\//, '')
  if (!file) continue
  declaredRules.set(
    file,
    new Set(names.filter((n) => !n.endsWith('.css')).map((n) => n.replace(/\s+/g, ' ').trim())),
  )
}

const sheets = (dir: string): string[] =>
  existsSync(dir) ? readdirSync(dir).filter((f) => f.endsWith('.css')).toSorted() : []

/** A block's scope, as one comparable string: `@media (…) › :root[data-theme='dark']`. */
const scopeKey = (conditions: string[], selector: string): string =>
  [...conditions, selector].join(' › ')

/**
 * Every custom property a sheet declares, grouped by the scope it is declared in.
 *
 * Not `scope(css)`, which keeps the unconditional `:root` and drops everything else. That
 * default left real surface unguarded: `motion.css` ships five tokens inside
 * `@media (prefers-reduced-motion: reduce) { :root }` on both sides, and a value changed there
 * — a reader who asked for no motion getting 999ms transitions — was never compared to
 * upstream at all. `departures.md` promises name, value **and scope**; this is the scope half.
 */
function byScope(css: string): Map<string, Map<string, string>> {
  const out = new Map<string, Map<string, string>>()
  for (const block of blocks(css)) {
    const key = scopeKey(block.conditions.map(normaliseSelector), normaliseSelector(block.selector))
    const bucket = out.get(key) ?? new Map<string, string>()
    for (const [token, value] of block.declarations) bucket.set(token, value)
    out.set(key, bucket)
  }
  return out
}

/**
 * The mirror-dependent checks skip when there is no mirror, and the honest thing is to say so
 * rather than to pass.
 *
 * It is pulled, never reconstructed. `a2ef5ab` says the six token files were taken "whole" and
 * names where it departed, so a plausible `upstream/` could be inferred from the commit log —
 * but `.yidam/authorship.yml` declares that directory `imported`, which asserts these are
 * upstream's bytes, and bytes nobody read are not that. The first real pull settled the
 * argument: it found three departures the commit log does not record, so a reconstruction would
 * have agreed with the port on precisely the values the port had changed.
 */
const mirrored = sheets(MIRROR)

describe('the design system mirror', () => {
  it('agrees with the pin about whether it has been pulled', () => {
    const pulled = /^\s*pulled\s*=\s*"([^"]*)"/m.exec(pin)?.[1]
    expect(pulled, 'pin.toml declares no `pulled`').toBeDefined()
    expect(
      pulled === 'never',
      `design/upstream/tokens holds ${mirrored.length} sheet(s) and pin.toml says pulled = "${pulled}"`,
    ).toBe(mirrored.length === 0)
  })

  it('is not shadowed by a sheet that sits outside it', () => {
    // The hole this closes: `site.css` and `dataviz.css` are not mirrored, so nothing upstream
    // constrains them — and a token redeclared there would change the value every component
    // resolves while the ported sheet still agreed with upstream, byte for byte. The port gate
    // would be structurally unable to see it.
    //
    // So no two sheets may declare the same token in the same scope. Scope, not name: the
    // night palette redeclares two dozen roles under the dark selectors on purpose, and that
    // is the mechanism working rather than a collision.
    const owner = new Map<string, string>()
    const collisions: string[] = []
    for (const file of sheets(PORT)) {
      for (const block of blocks(readFileSync(join(PORT, file), 'utf8'))) {
        for (const token of block.declarations.keys()) {
          const key = `${scopeKey(block.conditions, block.selector)} ${token}`
          const held = owner.get(key)
          if (held && held !== file) collisions.push(`${token} in both ${held} and ${file}`)
          else owner.set(key, file)
        }
      }
    }
    expect(collisions).toEqual([])
  })

  it.skipIf(mirrored.length === 0)('is the bytes the pin recorded', () => {
    // The mirror is read-only in the same sense `.yidam/.vendor/` is. An edit here is
    // discarded by the next pull and invisible until then, so it is caught at the gate
    // instead — this is the check that makes "never edited" a fact rather than a habit.
    for (const file of mirrored) {
      const actual = createHash('sha256')
        .update(readFileSync(join(MIRROR, file)))
        .digest('hex')
      const recorded = new RegExp(`^\\s*"tokens/${file}"\\s*=\\s*"([0-9a-f]{64})"`, 'm').exec(pin)
      expect(recorded, `pin.toml records no hash for tokens/${file}`).not.toBeNull()
      expect(actual, `tokens/${file} has been edited since the pull`).toBe(recorded?.[1])
    }
  })
})

/**
 * A sheet declared as a wholesale replacement is not diffed.
 *
 * `fonts.css` is the case that forced this. Upstream ships one — an `@import` from the Google
 * Fonts CDN plus five `@font-face` rules pointing at `fonts.gstatic.com` — and this repository
 * replaced it entirely with a generated sheet over self-hosted binaries. Diffing the two would
 * report every rule in both as a departure, which is true and useless: the departure is the
 * *replacement*, and it is declared once with its argument.
 *
 * The gate still asserts such a sheet exists on at least one side, so the entry cannot outlive
 * what it describes.
 */
const replaced = (file: string): boolean => declaredFiles.has(file)

describe.skipIf(mirrored.length === 0)('every departure from the design system is declared', () => {
  /** Token-level differences, in every scope either side declares. */
  const differences = mirrored
    .filter((file) => !replaced(file))
    .flatMap((file) => {
      const before = byScope(readFileSync(join(MIRROR, file), 'utf8'))
      const local = existsSync(join(PORT, file))
        ? byScope(readFileSync(join(PORT, file), 'utf8'))
        : new Map<string, Map<string, string>>()

      const empty = new Map<string, string>()
      return [...new Set([...before.keys(), ...local.keys()])].flatMap((key) => {
        const [a, b] = [before.get(key) ?? empty, local.get(key) ?? empty]
        return [...new Set([...a.keys(), ...b.keys()])]
          .filter((token) => {
            const [x, y] = [a.get(token), b.get(token)]
            return x === undefined || y === undefined ? x !== y : normalise(x) !== normalise(y)
          })
          .map((token) => ({
            file,
            token,
            scope: key,
            upstream: a.get(token) ?? '(absent)',
            here: b.get(token) ?? '(absent)',
          }))
      })
    })

  it.each(differences)('$file $token is in departures.md', (d) => {
    expect(
      declaredTokens.has(d.token),
      `${d.file} { ${d.scope} }: ${d.token} is "${d.upstream}" upstream and "${d.here}" here, and design/departures.md does not say why`,
    ).toBe(true)
  })

  it('declares nothing that is no longer a departure', () => {
    const differing = new Set(differences.map((d) => d.token))
    const stale = [...declaredTokens].filter((token) => !differing.has(token))
    expect(
      stale,
      'design/departures.md explains tokens that now match upstream — re-sync them and delete the entries',
    ).toEqual([])
  })

  it('declares every sheet only one side has', () => {
    // A whole file is a departure too, and in both directions. `dataviz.css` and `dark.css`
    // are palettes the system does not specify, so naming them is how the question of
    // whether they belong upstream gets asked rather than defaulted. The other direction
    // matters as much and is easier to miss: a sheet upstream ships and the port declined to
    // take — `tokens/fonts.css` was skipped because a Google Fonts `<link>` covered more
    // weights — is a decision, and an undeclared one is indistinguishable from an oversight.
    const here = sheets(PORT)
    const onlyOneSide = [
      ...here.filter((file) => !mirrored.includes(file)),
      ...mirrored.filter((file) => !here.includes(file)),
    ]
    expect(onlyOneSide.filter((file) => !declaredFiles.has(file))).toEqual([])
  })

  it('declares every rule only one side has', () => {
    // Declarations are the gate's subject, and on their own they miss a whole rule: `.scroll-x`
    // and the `code` rule came into `base.css` from the deleted bridge and declare no custom
    // property at all, so a token-only diff sees nothing. Selectors are the cheap second axis —
    // stable against reformatting in a way a body diff is not.
    const selectors = (dir: string, file: string): Set<string> =>
      new Set(
        blocks(readFileSync(join(dir, file), 'utf8'))
          .filter((b) => b.selector !== '')
          .map((b) => scopeKey(b.conditions.map(normaliseSelector), normaliseSelector(b.selector))),
      )

    const undeclared = mirrored
      .filter((file) => existsSync(join(PORT, file)) && !replaced(file))
      .flatMap((file) => {
        const [before, here] = [selectors(MIRROR, file), selectors(PORT, file)]
        return [
          ...[...here].filter((s) => !before.has(s)),
          ...[...before].filter((s) => !here.has(s)),
        ].map((selector) => ({ file, selector }))
      })
      // Exact, and scoped to the file the entry was written for. `.includes` over the scope key
      // exempted `pre code` and `.code-block` on the strength of a `code` entry, and exempted
      // it in every sheet rather than the one named.
      .filter(({ file, selector }) => !declaredRules.get(file)?.has(selector))

    expect(undeclared).toEqual([])
  })

  it('declares no sheet that exists on neither side', () => {
    // A `### File:` entry means "this sheet is not a straight port" — either only one side has
    // it, or this repository replaced upstream's wholesale. What it may never mean is nothing:
    // an entry naming a file nobody has is the drift the stale-declaration rule exists against.
    const present = new Set([...sheets(PORT), ...mirrored])
    expect(
      [...declaredFiles].filter((file) => !present.has(file)),
      'design/departures.md explains sheets that exist on neither side — delete the entries',
    ).toEqual([])
  })
})

describe('every token a page asks for is a token something declares', () => {
  /**
   * The failure this exists for, found the hard way.
   *
   * `bridge.css` aliased the site's original names onto the design system's, and shrinking it
   * meant renaming `var(--ink)` to `var(--text-strong)` across eleven files. A search-and-replace
   * over `var(--…)` finds every one of them in CSS and **none** of the ones in TypeScript, where
   * the chart and map palettes read the same tokens as bare strings:
   *
   *     ink: v('--ink'),
   *
   * So the rename passed the build, passed 521 tests and passed the linter, and left every
   * chart and the choropleth reading empty strings for their ink, ground, grid and axis. A
   * missing custom property is not an error anywhere: `getPropertyValue` returns `''`, Plotly
   * takes `''` as "use the default", and the page renders in somebody else's palette.
   *
   * Nothing in a stylesheet can be checked against a string in a script. This is what checks
   * them against each other.
   */
  const declared = new Set<string>()
  for (const file of sheets(PORT)) {
    for (const block of blocks(readFileSync(join(PORT, file), 'utf8'))) {
      for (const token of block.declarations.keys()) declared.add(token)
    }
  }

  const SCRIPTS = join(import.meta.dirname, '../src/scripts')
  const scripts = readdirSync(SCRIPTS)
    .filter((f) => f.endsWith('.ts'))
    .map((file) => ({ file, source: readFileSync(join(SCRIPTS, file), 'utf8') }))

  it.each(scripts)('$file reads only tokens that exist', ({ source }) => {
    const asked = [...source.matchAll(/getPropertyValue\(\s*['"](--[a-z0-9-]+)['"]|v\(\s*['"](--[a-z0-9-]+)['"]/g)]
      .map((m) => m[1] ?? m[2])
      .filter((token, i, all) => all.indexOf(token) === i)

    expect(asked.length, 'found no token reads — the pattern stopped matching').toBeGreaterThan(0)
    expect(asked.filter((token) => !declared.has(token))).toEqual([])
  })
})
