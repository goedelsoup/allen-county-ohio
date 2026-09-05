// The contrast pass, as a check rather than as a memory.
//
// The design system shipped with a caveat — "colour contrast was composed by eye against the
// parchment surfaces; a formal WCAG pass has not been run" — and composing by eye is exactly
// what a warm light ground defeats. Yellow at a given lightness carries far more luminance
// than red or green at the same lightness, so an ochre that looks as dark as its rubric
// sibling is nowhere near as readable, and no amount of looking at it says so.
//
// This reads the token file itself, resolves the aliases, and holds every text role to
// 4.5:1 on every surface it might travel to. It is over the tokens and not over the rendered
// pages, because a role is a promise made once and spent in seventeen components: the place
// to keep the promise is where it is made.
//
// It runs twice, over two palettes. The parchment one is a port and inherits a caveat. The
// night one is a derivation this repository composed, and it has no caveat to inherit and
// nobody upstream to report a defect to — which is an argument for measuring it harder rather
// than for trusting it more.

import { readFileSync } from 'node:fs'
import { describe, expect, it } from 'vitest'
import { resolve as follow, scope } from './tokens'

const sheet = (name: string): string =>
  readFileSync(new URL(`../src/styles/tokens/${name}.css`, import.meta.url), 'utf8')

const css = sheet('colors')
const night = sheet('dark')

/**
 * The palette a page gets before any preference applies.
 *
 * Read by scope rather than by a flat sweep of the files, and it has to be. `dark.css`
 * redeclares two dozen of these names under its own selectors; a regex over both files keeps
 * whichever came last, which would quietly turn every ratio below into a measurement of the
 * night palette against the day grounds — and pass.
 */
const declared = scope(css)

/**
 * And the palette a page gets on a dark OS.
 *
 * The ramps come from `colors.css` and only the semantic roles move, so the night map is the
 * day map with the dark block laid over it. That is the same resolution order the browser
 * performs, and doing it any other way here would check a palette no reader ever sees.
 */
const DARK_SELECTOR = ":root[data-theme='dark']"
const declaredDark = new Map([...declared, ...scope(night, DARK_SELECTOR)])

/** A token's literal colour, following `var()` aliases to the ramp underneath. */
const resolve = (token: string): string => follow(token, declared)
const resolveDark = (token: string): string => follow(token, declaredDark)

/** WCAG 2.1 relative luminance. */
function luminance(hex: string): number {
  const channels = [1, 3, 5].map((i) => Number.parseInt(hex.slice(i, i + 2), 16) / 255)
  const linear = channels.map((c) => (c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4))
  return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]
}

function contrast(a: string, b: string): number {
  const [x, y] = [luminance(a), luminance(b)]
  return (Math.max(x, y) + 0.05) / (Math.min(x, y) + 0.05)
}

/**
 * A declaration set, comparable across two blocks written at different indents.
 *
 * The night palette's at-rule block sits one level deeper than its attribute twin, so every
 * multi-line gradient differs by two spaces per line and by nothing else. Indentation is not a
 * value.
 */
const collapsed = (declarations: Map<string, string>): [string, string][] =>
  [...declarations].map(([name, value]): [string, string] => [name, value.replace(/\s+/g, ' ')]).toSorted()

/** The AA bar for normal text. Everything here is set at 11–20px, so it is the bar. */
const AA = 4.5

/** The four parchment grounds a page can put text on. */
const GROUNDS = ['--surface-page', '--surface-card', '--surface-raised', '--surface-sunken']

/**
 * Roles that carry words.
 *
 * A role is not tied to one ground — `--text-faint` is card metadata and scale labels and
 * relation provenance — so each is held to the darkest parchment it could land on rather than
 * to the one it happens to sit on today.
 */
const TEXT_ROLES = [
  '--text-strong',
  '--text-body',
  '--text-muted',
  '--text-faint',
  '--text-link',
  '--text-link-hover',
  '--text-link-active',
  '--text-annotation',
  '--status-verified-text',
  '--status-disputed-text',
  '--status-open-text',
]

/**
 * Roles that are marks: a dot, a chip, a dashed edge, an absent value.
 *
 * Measured across the four grounds these run 2.14:1 (`--status-missing` on `--surface-sunken`,
 * 2.41:1 on the page) to 4.48:1 (`--status-verified`), and they are deliberately not held to
 * the 3:1 bar for graphical
 * objects. That bar is for objects "required to understand content", and none of these is:
 * every badge, relation and status row in this site prints its tier as a word beside the ink.
 * The rule is stated in `Badge.astro` and `RelationsList.astro`, checked in `register.test.ts`,
 * and it is what lets the marks stay the colours the design composed.
 *
 * What is checked is that they stay marks — that nothing quietly points a text role at one.
 */
const MARK_ROLES = ['--status-verified', '--status-disputed', '--status-missing', '--status-error']

describe('text on parchment', () => {
  it.each(TEXT_ROLES)('%s reads on every ground', (role) => {
    const ink = resolve(role)
    expect(ink, `${role} does not resolve to a literal colour`).toMatch(/^#[0-9a-f]{6}$/)

    for (const ground of GROUNDS) {
      const ratio = contrast(ink, resolve(ground))
      expect(
        Number(ratio.toFixed(2)),
        `${role} (${ink}) on ${ground} (${resolve(ground)}) is ${ratio.toFixed(2)}:1`,
      ).toBeGreaterThanOrEqual(AA)
    }
  })

  it('reads inverted on the ink ground too', () => {
    // The masthead rule and the event date chip set parchment on ink.
    expect(contrast(resolve('--text-inverse'), resolve('--surface-ink'))).toBeGreaterThanOrEqual(AA)
  })

  it('keeps three distinguishable weights below the strong ink', () => {
    // --text-faint had to come down to --ink-500, which is the floor: nothing lighter reads on
    // --surface-sunken. --text-muted moved with it so the two roles did not collapse into one
    // colour. If a later edit lands them on the same ink, the hierarchy is gone and the page
    // still passes every ratio above — so it is checked separately.
    const [body, muted, faint] = ['--text-body', '--text-muted', '--text-faint'].map(resolve)
    expect(new Set([body, muted, faint]).size).toBe(3)

    const ground = resolve('--surface-page')
    expect(contrast(body, ground)).toBeGreaterThan(contrast(muted, ground))
    expect(contrast(muted, ground)).toBeGreaterThan(contrast(faint, ground))
  })
})

describe('the accent ramp', () => {
  it('has a -700 step that reads, in every hue', () => {
    // The ported --ochre-700 reached 4.25:1 on the page ground where rubric, verdigris and
    // indigo reached 7.25, 7.70 and 10.15. It was the odd step in the ramp rather than a
    // missing one, and this is what says so if it is ever put back.
    for (const accent of ['--rubric-700', '--verdigris-700', '--indigo-700', '--ochre-700']) {
      for (const ground of GROUNDS) {
        const ratio = contrast(resolve(accent), resolve(ground))
        expect(
          Number(ratio.toFixed(2)),
          `${accent} (${resolve(accent)}) on ${ground} is ${ratio.toFixed(2)}:1`,
        ).toBeGreaterThanOrEqual(AA)
      }
    }
  })

  it('keeps the status marks separate from the status words', () => {
    // The marks are exempt from the bar (see MARK_ROLES). What that exemption cannot survive
    // is a text role pointing at one, so the two sets must not meet.
    const marks = new Set(MARK_ROLES.map(resolve))
    for (const role of TEXT_ROLES) {
      expect(marks.has(resolve(role)), `${role} resolves to a mark ink`).toBe(false)
    }
  })
})

describe('text on ink', () => {
  /**
   * The same pass, on the night palette.
   *
   * The design system ships no dark palette; `tokens/dark.css` derives one from the ink and
   * parchment ramps it does ship. A derivation gets *more* scrutiny than a port, not less —
   * upstream cannot have measured what upstream never composed, so there is no caveat to
   * inherit and no author to report to. Whatever these numbers are, this repository chose them.
   *
   * The binding ground is `--surface-raised`, which at night is the shallowest ink rather than
   * the lightest parchment. It is the reason the accents come from the -100 end of each ramp:
   * at -300 the ochre word reached 4.74:1 there and the rubric one did not reach the bar at all
   * (2.77:1).
   */
  it.each(TEXT_ROLES)('%s reads on every night ground', (role) => {
    const ink = resolveDark(role)
    expect(ink, `${role} does not resolve to a literal colour in the dark scope`).toMatch(
      /^#[0-9a-f]{6}$/,
    )

    for (const ground of GROUNDS) {
      const ratio = contrast(ink, resolveDark(ground))
      expect(
        Number(ratio.toFixed(2)),
        `${role} (${ink}) on ${ground} (${resolveDark(ground)}) is ${ratio.toFixed(2)}:1 at night`,
      ).toBeGreaterThanOrEqual(AA)
    }
  })

  it('keeps three distinguishable weights below the strong ink at night too', () => {
    const [body, muted, faint] = ['--text-body', '--text-muted', '--text-faint'].map(resolveDark)
    expect(new Set([body, muted, faint]).size).toBe(3)

    const ground = resolveDark('--surface-page')
    expect(contrast(body, ground)).toBeGreaterThan(contrast(muted, ground))
    expect(contrast(muted, ground)).toBeGreaterThan(contrast(faint, ground))
  })

  it('keeps the status marks separate from the status words at night too', () => {
    const marks = new Set(MARK_ROLES.map(resolveDark))
    for (const role of TEXT_ROLES) {
      expect(marks.has(resolveDark(role)), `${role} resolves to a mark ink at night`).toBe(false)
    }
  })

  it('reads inverted on the inverted ground, which has swapped ends', () => {
    // In daylight `--surface-ink` is the darkest ink and `--text-inverse` is parchment. At
    // night both turn over: the inverted surface is the one that is *light*. The pair has to
    // keep working through the swap, which is the sort of thing an inversion gets wrong.
    expect(
      contrast(resolveDark('--text-inverse'), resolveDark('--surface-ink')),
    ).toBeGreaterThanOrEqual(AA)
  })

  it('leaves no role undefined in the dark scope', () => {
    // A role the night block forgets silently keeps its daylight value, which on an ink ground
    // is the failure this whole file exists to catch — and it would pass every ratio above if
    // the forgotten role happened to be a light one.
    const dark = scope(night, DARK_SELECTOR)
    const missing = [...TEXT_ROLES, ...MARK_ROLES, ...GROUNDS].filter((role) => !dark.has(role))
    expect(missing).toEqual([])
  })

  it('says the same thing to the OS and to the attribute', () => {
    // Plain CSS cannot write one set of values into both an at-rule and a top-level selector,
    // so the two blocks are duplicated. Duplication that nothing checks is duplication that
    // drifts — and the drift here would be a reader whose OS is dark seeing a different site
    // from a reader who asked for dark, with no error anywhere.
    const byPreference = scope(
      night,
      ":root:where(:not([data-theme='light']))",
      '@media (prefers-color-scheme: dark)',
    )
    const byAttribute = scope(night, DARK_SELECTOR)

    expect(byPreference.size).toBeGreaterThan(0)
    expect(collapsed(byPreference)).toEqual(collapsed(byAttribute))
  })
})

describe('large text is held to the large-text bar, on both grounds', () => {
  /**
   * `--text-drop-cap` is the 66px rubric capital an entry opens with, and WCAG's bar for text
   * that size is 3:1 rather than 4.5:1. It is checked separately for exactly that reason: put it
   * in `TEXT_ROLES` and it fails at night for being a colour rather than for being unreadable;
   * leave it out entirely and it goes back to being what it was — a raw `--rubric-500` at 2.44:1
   * on ink, which is under even this bar.
   *
   * Only `--surface-page` — the drop cap sits in `.claim`, which sets no ground of its own.
   */
  const LARGE = 3

  it.each([
    ['day', resolve],
    ['night', resolveDark],
  ])('%s: the drop cap reads on the page', (_when, read) => {
    const ratio = contrast(read('--text-drop-cap'), read('--surface-page'))
    expect(
      Number(ratio.toFixed(2)),
      `--text-drop-cap (${read('--text-drop-cap')}) on ${read('--surface-page')} is ${ratio.toFixed(2)}:1`,
    ).toBeGreaterThanOrEqual(LARGE)
  })
})
