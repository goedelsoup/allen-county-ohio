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

import { readFileSync } from 'node:fs'
import { describe, expect, it } from 'vitest'

const css = readFileSync(new URL('../src/styles/tokens/colors.css', import.meta.url), 'utf8')

/** Every custom property in the sheet, unresolved. */
const declared = new Map(
  [...css.matchAll(/(--[a-z0-9-]+):\s*([^;]+);/g)].map(([, name, value]) => [name, value]),
)

/** A token's literal colour, following `var()` aliases to the ramp underneath. */
function resolve(token: string, depth = 0): string {
  const value = (declared.get(token) ?? '').split('/*')[0].trim()
  const alias = /^var\((--[a-z0-9-]+)\)$/.exec(value)
  if (alias && depth < 10) return resolve(alias[1], depth + 1)
  return value
}

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
  '--text-annotation',
  '--status-verified-text',
  '--status-disputed-text',
  '--status-open-text',
]

/**
 * Roles that are marks: a dot, a chip, a dashed edge, an absent value.
 *
 * Measured on the page ground these run 2.14:1 (`--status-missing`) to 4.48:1
 * (`--status-verified`), and they are deliberately not held to the 3:1 bar for graphical
 * objects. That bar is for objects "required to understand content", and none of these is:
 * every badge, relation and status row in this site prints its tier as a word beside the ink.
 * The rule is stated in `Badge.astro`, `RelationsList.astro` and `bridge.css`, and it is what
 * lets the marks stay the colours the design composed.
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
