# design

The Allen County Atlas design system, and this repository's relationship to it.

The site's token layer was ported by hand in
[`a2ef5ab`](../web/src/styles/tokens/) and corrected under measurement in `2737dd7`. For five
days that port had no pin, no mirror, and no way to send a correction back — which is the
same failure [`upstream-findings`](../.yidam/decisions/upstream-findings.yml) records against
the yidam template, in a repository that had already paid for it once.

This directory is the return address.

```
pin.toml        which project, which pull, and the hash of every file in the mirror
upstream/       a verbatim copy of the design system's own files — imported, never edited
departures.md   every declaration where the site's tokens differ from the mirror, and why
outbound/       what has been sent back, kept so the record of what left survives
```

## What it is for

Three things the port could not do without it:

**Say what upstream actually holds.** `upstream/` is the design system's own bytes. Nothing
here paraphrases them, and nothing here edits them — a defect in the mirror is reported
upstream, never repaired in place, for the reason the directory conventions give for
`.yidam/.vendor/`: *editing those to satisfy a linter falsifies the record the directory
exists to keep.* It is declared as `imported` in
[`.yidam/authorship.yml`](../.yidam/authorship.yml) so the prose gates address findings there
to whoever can act on them.

**Say where this repository departed, and on what argument.** `departures.md` is the whole
list. `web/test/port.test.ts` fails on a difference that is not in it — and on an entry that
no longer describes a difference, which is the same discipline
[`.yidam/lint-baseline.yml`](../.yidam/lint-baseline.yml) and the publication gate's `answers`
list are held to: a list permitted to be wrong drifts, and one that over-lists silently
re-permits whatever it over-lists.

**Carry a finding back.** The corrections this repository made are not local preferences. A
`var()` that does not resolve invalidates the whole `font` shorthand for every consumer of the
system, not only this one; an ochre step at 4.25:1 is unreadable on anybody's parchment. Those
belong upstream, and
[`the-design-system-is-an-upstream`](../.yidam/decisions/the-design-system-is-an-upstream.yml)
is where each one is written down until it gets there.

## What it does not get to relax

Nothing. This is not corpus material and makes no claim about Allen County, so the claim-tag
vocabulary does not reach it — but the two rules that do reach it are unchanged:

- **`upstream/` is read-only in the same sense `.yidam/.vendor/` is.** An edit there is
  discarded by the next pull and is invisible until then.
- **A push is an outward-facing act and waits on the elector.** Per
  [`what-may-leave-the-repository`](../.yidam/decisions/what-may-leave-the-repository.yml),
  nothing here authorises an agent to write to the design system unprompted.
  [`design-push`](../.yidam/skills/design-push.md) halts at the plan boundary by construction.

## Running it

Both directions are agent skills rather than `mise` tasks, because the `DesignSync` tool has no
CLI to wrap.

| | |
|---|---|
| [`design-pull`](../.yidam/skills/design-pull.md) | refresh `upstream/`, rewrite the pin, report drift |
| [`design-push`](../.yidam/skills/design-push.md) | stage a finding, finalize the plan, **stop**, then write |

The gate over the result is ordinary and hermetic:

```
mise run site-test        # includes web/test/port.test.ts
```
