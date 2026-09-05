// Reading a token sheet, for the checks that hold the palette to something.
//
// Two tests need the same three operations — split a stylesheet into its blocks, collect the
// custom properties each one declares, and follow a `var()` alias down to the literal
// underneath. `contrast.test.ts` had them inline and `port.test.ts` would have had a second
// copy, which is one copy too many for a parser: the two would drift and the drift would show
// up as one gate passing while the other failed on the same file.
//
// It is scope-aware, and that is not decoration. A flat regex over a sheet that declares
// `--text-muted` in `:root` and again under `[data-theme='dark']` keeps whichever comes last,
// so a night palette would silently become the thing the light-ground contrast check measures.

/** A rule block, with whatever at-rules it sits inside. */
export interface Block {
  /** The selector, e.g. `:root` or `:root[data-theme='dark']`. */
  selector: string
  /** At-rule preludes, outermost first: `['@media (prefers-color-scheme: dark)']`. */
  conditions: string[]
  /** The custom properties this block declares, in source order. */
  declarations: Map<string, string>
}

function stripComments(css: string): string {
  return css.replace(/\/\*[\s\S]*?\*\//g, '')
}

/** The index of the `}` closing the `{` at `open`. */
function matchBrace(source: string, open: number): number {
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}' && --depth === 0) return i
  }
  return source.length
}

/**
 * The custom properties in a block body.
 *
 * The token sheets are flat — no CSS nesting — so anything between braces inside a body is
 * not ours and is dropped before the scan rather than parsed.
 */
function parseDeclarations(body: string): Map<string, string> {
  const flat = body.replace(/\{[^{}]*\}/g, '')
  return new Map(
    [...flat.matchAll(/(--[a-z0-9-]+)\s*:\s*([^;]+);/g)].map(([, name, value]) => [
      name,
      value.trim(),
    ]),
  )
}

/**
 * At-rules whose body is declarations rather than nested rules.
 *
 * The distinction is not cosmetic. `@media` wraps rules and has to be recursed into;
 * `@font-face` *is* a rule and has to be collected. Treating them alike made every
 * `@font-face` block vanish from the parse — which was silent, because a file of nothing but
 * `@font-face` then compared equal to any other such file, and the fonts sheet is exactly that.
 */
const DECLARATION_AT_RULES = /^@(font-face|page|property|counter-style|font-palette-values)\b/

function walk(source: string, conditions: string[], out: Block[]): void {
  let i = 0
  while (i < source.length) {
    const open = source.indexOf('{', i)
    if (open === -1) return
    // A statement at-rule — `@charset "utf-8";`, `@import url(…);`, `@layer a;` — ends at a
    // semicolon and has no block. Everything between the previous `}` and the next `{` would
    // otherwise glue it onto the following rule's prelude, which then fails to look like a
    // selector, gets treated as a wrapper, and recursed into a body with no braces: the rule
    // vanishes. Top-level CSS puts nothing else between rules, so taking the text after the
    // last `;` discards exactly the statement at-rules and nothing else.
    const prelude = source.slice(i, open).split(';').pop()?.trim() ?? ''
    const close = matchBrace(source, open)
    const body = source.slice(open + 1, close)
    // An at-rule that wraps rules is a condition: recurse. One that holds declarations is a
    // block in its own right, and its prelude is its identity.
    if (prelude.startsWith('@') && !DECLARATION_AT_RULES.test(prelude)) {
      walk(body, [...conditions, prelude], out)
    } else {
      out.push({ selector: prelude, conditions, declarations: parseDeclarations(body) })
    }
    i = close + 1
  }
}

/** Every rule block in a sheet, in source order. */
export function blocks(css: string): Block[] {
  const out: Block[] = []
  walk(stripComments(css), [], out)
  return out
}

/**
 * The custom properties a sheet declares for one scope.
 *
 * `selector` and `condition` are matched exactly. The default is the unconditional `:root`,
 * which is the palette a page gets before any preference or attribute applies.
 */
export function scope(
  css: string,
  selector = ':root',
  condition?: string,
): Map<string, string> {
  const merged = new Map<string, string>()
  for (const block of blocks(css)) {
    if (block.selector !== selector) continue
    const inside = condition === undefined ? block.conditions.length === 0 : block.conditions.includes(condition)
    if (!inside) continue
    for (const [name, value] of block.declarations) merged.set(name, value)
  }
  return merged
}

/**
 * A token's literal value, following `var()` aliases down to the ramp underneath.
 *
 * Returns the unresolved value when the chain leaves the map it was given, which is what a
 * caller wants to see in a failure message: an alias pointing at nothing reads as itself.
 */
export function resolve(token: string, declared: Map<string, string>, depth = 0): string {
  const value = (declared.get(token) ?? '').split('/*')[0].trim()
  const alias = /^var\((--[a-z0-9-]+)\)$/.exec(value)
  if (alias && depth < 10 && declared.has(alias[1])) return resolve(alias[1], declared, depth + 1)
  return value
}

/**
 * A colour function in one canonical form, so two spellings of one colour compare equal.
 *
 * `rgba(30,25,19,.55)` and `rgb(30 25 19 / 55%)` are the same ink. The design system writes the
 * legacy form; the port rewrote it to the modern one, consistently, across twelve tokens in two
 * sheets. Reporting those as departures would fill `design/departures.md` with twelve entries
 * that say "we wrote it differently" — and a file whose entries are mostly noise stops being
 * read, which costs more than the syntax rewrite ever did.
 *
 * Alpha lands as a 0–1 decimal either way, so `.020` and `2%` meet.
 */
/**
 * One RGB channel as a 0–255 number.
 *
 * `%` is CONVERTED, never dropped. Parsing `30%` with `parseFloat` and keeping the 30 makes
 * `rgb(30% 25% 19%)` — which is rgb(77, 64, 48) — canonicalise identically to
 * `rgb(30 25 19)`, a near-black. That is a 2.55× error in every channel presented as a
 * spelling difference, and it is exactly what this normaliser promises never to do.
 */
const channel = (c: string): string => {
  const n = Number.parseFloat(c)
  return String(c.trim().endsWith('%') ? Math.round((n * 255) / 100) : n)
}

function canonicalColour(value: string): string {
  return value.replace(/\brgba?\(([^()]*)\)/g, (whole, args: string) => {
    // Split on comma, slash AND whitespace: the legacy form separates channels with commas and
    // the modern one with spaces, so a separator set that misses either silently declines to
    // canonicalise half the values it was written for.
    const parts = args.split(/[\s,/]+/).map((s) => s.trim()).filter(Boolean)
    if (parts.length < 3 || parts.length > 4) return whole
    const raw = parts[3] ?? '1'
    const alpha = raw.endsWith('%') ? Number.parseFloat(raw) / 100 : Number.parseFloat(raw)
    if (parts.slice(0, 3).some((c) => Number.isNaN(Number.parseFloat(c))) || Number.isNaN(alpha)) {
      return whole
    }
    return `rgb(${parts.slice(0, 3).map(channel).join(' ')}/${Number(alpha.toFixed(4))})`
  })
}

/**
 * A declaration value reduced to what it means, so that presentation is not a departure.
 *
 * What is normalised is only ever spelling: case (CSS keywords, hex digits and font family
 * names are all case-insensitive, and every custom property in this system is lowercase
 * already), quote style, whitespace including around `/` and `,`, a zero written with a unit,
 * and the two colour-function syntaxes. What is NOT normalised is any number, any token name
 * and any keyword — a value that means something different still compares different.
 */
export function normalise(value: string): string {
  return canonicalColour(
    value
      .replace(/\/\*[\s\S]*?\*\//g, ' ')
      .toLowerCase()
      // Quotes around a font family are optional in CSS and the two sides disagree about them
      // per-name — upstream writes `"Optima"`, the port writes `optima`. Dropped rather than
      // unified, because which names are quoted carries no meaning at all.
      .replace(/['"]/g, '')
      .replace(/\s+/g, ' ')
      // A leading zero is optional in CSS and the two sides disagree: `cubic-bezier(.22,.61…)`
      // upstream against `cubic-bezier(0.22, 0.61…)` here. Same curve.
      .replace(/(^|[\s,(/])\.(\d)/g, '$10.$2')
      .trim(),
  )
    .replace(/\s*([,/])\s*/g, '$1')
    // Padding inside parentheses: the port broke long gradients across lines, so a collapsed
    // `linear-gradient(\n  to bottom,` reads as `linear-gradient( to bottom,`.
    .replace(/\(\s+/g, '(')
    .replace(/\s+\)/g, ')')
    // A zero LENGTH may drop its unit; a zero angle or percentage may not. `0` and `0%` are
    // different arguments to `hsl()`, legacy `rgb()` and `color-mix()`, and
    // `repeating-linear-gradient(0deg, …)` is not `repeating-linear-gradient(0, …)` — that one
    // is invalid. Only the length units fold, which is the case the port actually created when
    // it rewrote `--paper-grain`'s `0px` stops as `0`.
    .replace(/(^|[\s,(/])0(px|em|rem)\b/g, '$10')
    .trim()
}

/**
 * A selector or at-rule prelude reduced the same way.
 *
 * Whitespace, comma spacing and the space after a colon are not identity —
 * `@media (prefers-reduced-motion: reduce)` and `@media (prefers-reduced-motion:reduce)` are
 * one condition, and CSS never requires a space around a `:` in either position.
 */
export function normaliseSelector(selector: string): string {
  return selector
    .replace(/\s+/g, ' ')
    .replace(/\s*([,:])\s*/g, '$1')
    .trim()
}
