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

This domain adds none of its own yet. `mise run ci` is the whole gate.

The first one this corpus is likely to need is a **succession check** over `tenure` nodes:
an office whose holders leave a gap, or whose intervals overlap, is a defect no graph check
can see, because each node is individually well-formed and each edge resolves. Name it here
with the command that runs it the day it exists.
