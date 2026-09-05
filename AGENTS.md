# Agent Instructions

This repository's domain is **Allen County, Ohio** — the county seated at Lima in the
northwest quarter of the state, from pre-statehood occupation and the 1818 treaty cession
through the present. It is organized around one wide question: *what is true of this county,
and when was it true?* No single thesis about the county is privileged; the corpus is built
to be entered from a place, a moment, or an office, and to answer across all three.

This repository is a living knowledge artifact. Its git history is the knowledge graph:
files are nodes, commits are knowledge events, links are edges.

## Before taking substantive action

Read the vendored prelude. It is the model this repository runs on, and it is not
negotiable from inside the repo:

- [Identity](.yidam/.vendor/prelude/IDENTITY.md) — what this kind of repository is
- [Graph model](.yidam/.vendor/prelude/GRAPH.md) — how git encodes knowledge
- [Agent conduct](.yidam/.vendor/prelude/guidelines/agent-conduct.md) — behavioral norms,
  including the `[verified]` / `[inference]` / `[open]` claim tags
- [Directory conventions](.yidam/.vendor/prelude/guidelines/directories.md) — what belongs where
- [Phases](.yidam/.vendor/prelude/PHASES.md) — how a unit of inquiry is bounded and committed
- [Constitution](.yidam/.vendor/prelude/CONSTITUTION.md) — binding on every resolution event
  *(collective governance only; dormant in a single-elector repository)*

Files under `.yidam/.vendor/` are read-only. A defect in the prelude is fixed by re-vendoring
against a newer yidam release (`mise run yidam-vendor-update`), never by editing in place —
an edit there is silently discarded on the next update.

## Conduct norms

**Commit deliberately.** Every commit is a permanent knowledge event. The message should read
as a legible description of what changed and why — not a diff summary. Keep epistemic commits
(understanding added or revised) distinct from operational commits (extraction, refresh,
regeneration) — and the distinction is carried by the **closed verb vocabulary** in
[GRAPH.md](.yidam/.vendor/prelude/GRAPH.md), not by style alone. Every subject begins
`<verb>: `, the verb stands alone with no `(scope)` suffix, and `yidam lint --commits`
reports anything outside the list. A merge deserves a written subject too; git's default
names the ref and not what joining it meant.

**Link generously.** New nodes must connect to existing ones. Orphan files weaken the graph.

**Stay within scope.** Do not add nodes speculatively. Add a node when an edge needs a target
that does not yet exist.

**Make synthesis explicit.** Adding edges between existing nodes is a first-class
contribution, not housekeeping.

**Preserve provenance.** Do not delete or rewrite committed nodes without a record of why.

**Tag your claims.** Non-obvious claims carry `[verified]`, `[inference]`, or `[open]`.
Untagged inference is the problem the tags exist to prevent.

## The gate

`mise run ci` is what CI runs: `graph-check`, `graph-lint`, and `regen --check`. A commit
that breaks an edge, orphans a node, or leaves a REGEN block stale fails there. Run it
before committing rather than after.

Run the composite rather than its parts. Each catches something the others do not —
`graph-check` reads the graph and is blind to a stale REGEN block; `regen --check` reads
generated content and is blind to a broken edge — and the set is held to CI's by a test
upstream, so it stays right when a gate is added.

`yidam lint` gates against `.yidam/lint-baseline.yml`, not against zero. It asks whether
*this change* made the corpus less clean, because a gate that fails on inherited debt gets
switched off and stays off. Two things fail it: an error-severity violation that is not in
the baseline, and a baseline entry that no longer occurs. The second is not a bug — a
baseline permitted to be wrong drifts, and one that over-lists silently re-permits whatever
it over-lists. Fix the corpus, then `mise run graph-lint-bless` and commit the diff.

Two checks report and never gate. `unauthored-prose-link` covers material this repository
did not author — generated output, and imports copied from elsewhere unmodified — declared in
`.yidam/authorship.yml`. Those findings are real; they are somebody else's. Fix the
generator, or raise it upstream. Do not baseline them, and do not edit an import to satisfy a
linter: that falsifies the record the import exists to keep. `authorship-region-stale` says a
declaration there no longer matches anything on disk.

`mise run graph-lint-explain` prints each check's rationale. Read it before deciding a
check is wrong.

## Validation while you type

```
mise run schema           # emit .yidam/schemas/*.json
yidam schema --settings   # the editor mapping to paste into .vscode/settings.json
```

Read by `yaml-language-server` (the Red Hat YAML extension in VS Code; available to Neovim
and Helix over LSP). Editors using none of these still get the check from `yidam lint`.

## Domain gates

`mise run ci` still runs all of them — it is the composite, and the two domain gates below sit
inside it rather than beside it, so there is one command to remember and one place they can
drift from CI.

**The edge-provenance check.** `crates/provenance`, run as `edge-audit`. Every edge that asserts
something about the world must carry a `claim_tag`; `instance-of`, `concerns` and `subject-of`
are structural and must not. It is a `cargo test` in that crate, so `mise run ci` runs it.

**The publication gate.** `crates/publish`, run as `publish-feeds`. This repository publishes —
[`web/`](web/) is a public site — so the rules in
[agent-conduct](.yidam/.vendor/prelude/guidelines/agent-conduct.md) under *When claims leave the
repository* apply, and this is what enforces them: a tier computed from the corpus rather than
declared, a verbatim span behind every assertion, a refusal answered rather than routed around,
and `[open]` excluded absolutely. It also holds `web/src/feeds/` — derived from the corpus and
committed — to what the corpus currently says, which is the derivation-is-the-gate rule the
directory conventions ask for. Run `mise run publish` after changing a node the site cites, and
commit the regenerated feeds.

The site's own gate is `mise run site-test`, also inside `ci`. Its subject is the one derivation
performed under `web/`: joining a corpus GEOID to the census geometry vendored in
`web/public/geo/`, which neither of the crates above can see.

**The port gate.** [`web/test/port.test.ts`](web/test/port.test.ts), inside `site-test`. The
site's token layer is a hand port of a design system this repository does not own, and
[`design/`](design/) is the return address it lacked for its first five days: a verbatim mirror
under `design/upstream/`, the bytes pinned in `design/pin.toml`, and every local difference
declared in `design/departures.md` with its argument. The gate fails on a difference that is not
declared **and on a declaration that no longer describes one** — the same discipline as the lint
baseline and the publication gate's `answers` list, for the same reason.

`design/upstream/` is read-only in the sense `.yidam/.vendor/` is, and is declared `imported` in
[`.yidam/authorship.yml`](.yidam/authorship.yml). A defect inside it is reported upstream, never
repaired in place. The two directions are agent skills rather than `mise` tasks, because the
tool that reaches the design system has no CLI: [`design-pull`](.yidam/skills/design-pull.md)
refreshes the mirror and reports drift, and [`design-push`](.yidam/skills/design-push.md) carries
a finding back and **halts** at the plan boundary — a push is an outward-facing act and waits on
the elector. The findings themselves live in
[the-design-system-is-an-upstream](.yidam/decisions/the-design-system-is-an-upstream.yml).

A **succession check** over `tenure` nodes is still the gate this corpus is most likely to want
next: an office whose holders leave a gap, or whose intervals overlap, is a defect no graph check
can see, because each node is individually well-formed and each edge resolves. `crates/succession`
computes it and `tests/corpus.rs` pins the sheriff line; what does not exist yet is a rule that
every office be checked. Name it here with the command that runs it the day it does.
