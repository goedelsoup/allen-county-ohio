# Departures

Every declaration where `web/src/styles/tokens/` differs from `upstream/`, and the argument
for each.

`web/test/port.test.ts` reads this file. A difference that is not listed here fails the build;
so does an entry here that no longer describes a difference. The second half is the one that
does the work over time — a list of exceptions permitted to be wrong drifts exactly as a lint
baseline does, and an entry that outlives the departure it describes quietly re-permits
whatever a later edit puts in its place.

**Comments are not departures.** The port rewrote every file header to say where the file came
from and what it is doing here, and a byte diff would be nothing but that. The gate compares
custom-property *declarations* — name, value and the scope they are declared in — plus the set
of selectors each sheet defines, and ignores everything else.

**What that leaves outside the gate**, stated rather than discovered later: the *contents* of a
rule the two sides share. If upstream changes a `padding` inside `body` the check is silent,
because a body diff over hand-formatted CSS reports whitespace as loudly as substance and a gate
nobody can read is a gate somebody switches off. The token contract is the thing under
guarantee here. `design-pull` prints the raw diff for a human on every run, which is where that
class of change is meant to be caught.

**Every entry below is verified against `upstream/`**, pulled 2026-09-05. Nothing here is
testimony from a commit message any more; the "upstream" column is the bytes the design system
actually serves, and the gate compares against them on every run.

The first pull found three departures nobody had written down — `--text-faint`,
`--text-link-hover` and `--text-annotation`, all marked ▲▲ below. All three were made
deliberately in the contrast pass, all three are argued somewhere in `colors.css` — two of them
only in a mid-file comment, under a header whose own count said *two* — and all three were
invisible to anything that could act on them. That is the whole case for this file
in one result: **a departure explained in a comment is not a departure that has been declared**,
and for five days the difference could not be seen from either end.

**Syntax is not listed.** The port also rewrote `rgba(30,25,19,.55)` as `rgb(30 25 19 / 55%)`,
unquoted single-word font families, added leading zeros to `cubic-bezier` arguments and broke
long gradients across lines — consistently, across about twenty tokens in three sheets. None of
it is a departure and none of it appears here, because `port.test.ts` normalises spelling before
it compares: case, quotes, whitespace, a zero written with a unit, and the two colour-function
syntaxes. Twenty entries saying *we wrote it differently* would have made this file unreadable,
and an unread file is not a record.

---

## Corrections

A defect in the system, repaired here because the site had to ship. Each is a finding in
[`the-design-system-is-an-upstream`](../.yidam/decisions/the-design-system-is-an-upstream.yml),
and each **comes out of this file** when the fix lands upstream and the declaration can be
re-synced verbatim.

### `--type-eyebrow` · `tokens/typography.css`

```
upstream   var(--weight-500) var(--size-label)/1.2 var(--font-smallcaps)
here       var(--weight-medium) var(--size-label) / 1.2 var(--font-smallcaps)
```

`--weight-500` is not a token the system defines; the weights are `--weight-regular` through
`--weight-bold`. In the `font` shorthand an unresolved `var()` invalidates the **whole
declaration** at computed-value time, so `--type-eyebrow` resolved to nothing and every element
set with it fell back to inherited type — losing the family, the size and the line height
together.

It fails quietly in the worst way. The eyebrow is small-caps metadata, so the symptom reads as
slightly-off styling rather than as a broken rule, and nothing errors.

Filed as issue #10. Finding 1.

### `--ochre-700` · `tokens/colors.css`

```
upstream   #8a6417
here       #5c420f
```

4.25:1 on `--surface-page`, where `--rubric-700`, `--verdigris-700` and `--indigo-700` reach
7.25, 7.70 and 10.15. Yellow at a given HSL lightness carries far more luminance than red or
green at the same lightness — which is exactly the error an eye makes and a meter does not, and
the design system's own CAVEATS say the contrast was composed by eye.

It is the odd step in the ramp rather than a missing one, so it is corrected in place rather
than worked around with an `--ochre-800`. Finding 2.

### `--text-muted`, `--text-faint` · `tokens/colors.css`

```
upstream   --text-muted: var(--ink-500)     --text-faint: var(--ink-400)   ▲▲
here       --text-muted: var(--ink-600)     --text-faint: var(--ink-500)
```

One move, in two tokens. `--text-faint` measured 3.06:1 on the sunken ground and had to come
down to `--ink-500`, which is the floor — nothing lighter reads there at all. `--text-muted`
moves with it so the two roles do not collapse into a single colour: a palette can pass every
ratio and still have lost its hierarchy, which is why `contrast.test.ts` checks the three text
weights against each other separately from the bar.

Measured 3.06 → 4.60 and 4.60 → 6.36. Finding 2.

### `--text-link-hover`, `--text-annotation` · `tokens/colors.css`   ▲▲

```
upstream   var(--rubric-500)
here       var(--rubric-700)
```

The `-500` rubric is the design's signature and stays wherever it is a mark or set large — the
drop cap, the active nav rule. At label size it reaches 4.36:1 on the sunken ground, and **a
role has to be safe on every surface it might travel to**, not on the one it happens to sit on
today. Both roles take the `-700` step instead.

Measured 4.36 → 6.44 for each. Finding 2.

---

## Additions

Roles and files the system does not specify. These are **not** corrections and do not come out
when upstream moves; they come out if and when upstream adopts them.

### `--status-verified-text`, `--status-disputed-text`, `--status-open-text` · `tokens/colors.css`

The system ships one ink per status. Two are needed, because a status *mark* and a status
*word* want different things from the same hue: the `-500` inks are composed as a dot, a chip
or a dashed edge, and the ochre one reads at 2.47:1 against the page — right for a mark and
unreadable as a word.

So the marks keep the colours the design composed, and the words take the same hues at the step
that can be read on any of the four grounds. The marks are deliberately not held to the 3:1 bar
for graphical objects: that bar is for objects required to understand content, and every badge,
relation and status row here prints its tier as a word beside the ink. Finding 2.

### `--text-link-active`, `--text-drop-cap` · `tokens/colors.css`

Two text roles the system does not name, both of which existed as raw ramp inks reached for at
the point of use — `--rubric-700` in `base.css`'s `a:active`, `--rubric-500` in the entry page's
drop cap.

The night palette is what made that cost visible, and it is worth stating as a general result
rather than as two fixes: **a role gets a dark value; a literal does not.** Rest and hover both
routed through roles and inverted correctly; active did not, so every prose link went to 1.66:1
the moment the ground turned to ink. The drop cap went to 2.44:1, under even the 3:1 large-text
bar. Neither was visible to `contrast.test.ts`, which sweeps role names — correctly, since a
role is the thing that carries a promise.

`register.test.ts` now fails the build on any `color:` in a component that reads the base ramp
instead of a role.

### Rule: `.scroll-x`, `code` · `tokens/base.css`

Neither is a token, and both came here from `bridge.css` when that file was deleted — the
migration seam had accumulated two element rules that had nowhere else to go.

`.scroll-x` is load-bearing rather than cosmetic: it is what keeps a wide table or chart
scrolling inside its own box so the page never scrolls sideways. The system's `base.css` has no
opinion about overflow, which is a reasonable omission in a design brief and not one a shipped
site can keep.

### File: `tokens/site.css`

One token, `--gutter-fluid`. The system's `--gutter-page` is a fixed 36px, which is right at
reading width and too much on a phone; the site's own gutter was fluid and its narrow-screen
behaviour is worth keeping. So the system's value becomes the *ceiling* —
`clamp(18px, 4vw, var(--gutter-page))` — and is referenced rather than copied, so it cannot fall
out of step.

This is deliberately a new name rather than an override of `--gutter-page`. An override would
change the value every component resolves while `spacing.css` still agreed with upstream byte
for byte, which is a departure the gate is structurally unable to see. `port.test.ts` checks
that no unmirrored sheet shadows a mirrored token for exactly this reason.

Not a finding. The system is not wrong to specify a constant; this site needs a second value at
one breakpoint, which is a consumer's business.

### File: `tokens/dataviz.css`

The system specifies no data-visualization palette. The site needs one, and derives it from the
accent inks rather than importing a chart palette composed against a cool ground. See the file's
own header for the argument. Finding 4 — either these become a real upstream addition or they
stay a documented local derivation, and the point of writing them down is that the question gets
asked rather than settled by default.

### File: `tokens/fonts.css`

The design system ships a `tokens/fonts.css` and this is **not** it. The port did not take that
file, because the system supplies no font binaries — its own CAVEATS say so, and say the five
families were chosen for a letterpress character rather than specified — so what it declares are
`@font-face` rules for faces nobody has. A Google Fonts `<link>` covered more weights for less
work, and that is what the port used.

Which turned out to cost more than it saved. A public civic-reference site made every reader's
browser announce itself to a third party on every page load, and the fonts became the only thing
the site had to fetch from elsewhere to render as designed — the feeds are committed, the
geometry is vendored, the build is static, and `web/README.md` said as much while it was not
true.

The file here is generated by `mise run fonts` from `web/scripts/fetch-fonts.mjs`, which is
where the audit of which weights actually ship lives. Finding 5, and a small one: the report
upstream is that a `tokens/fonts.css` assuming binaries it does not have is discoverable only
after a consumer has wired something else around it.

### File: `tokens/dark.css`

The system is emphatic about being composed for paper and ships no dark palette. Dropping night
mode was a functional regression for anyone reading on a dark OS, and deriving one from the
system's own pieces — the ink ramp as ground, the parchment ramp as text — is in keeping rather
than an inversion. Finding 3, and offered upstream as a proposal rather than a correction.

Every value is a step the system already ships, except two interpolated stops in the choropleth
marked ▲ in the file. Measured on the four night grounds by `contrast.test.ts`, the worst text
role reaches 5.42:1 against a bar of 4.5.

Two things about this file are load-bearing and easy to undo by accident:

- **It redeclares tokens `colors.css` owns, and that is the mechanism rather than a collision.**
  The port gate's shadowing check is scoped, so a second declaration under
  `prefers-color-scheme` or `[data-theme]` is allowed where one under `:root` is not.
- **The two blocks inside it are identical and duplicated.** Plain CSS cannot write one set of
  values into both an at-rule and a top-level selector. `contrast.test.ts` asserts they have not
  drifted, because the drift would be a reader whose OS is dark seeing a different site from a
  reader who asked for dark, with no error anywhere.
