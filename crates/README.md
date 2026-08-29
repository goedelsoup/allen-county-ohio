# crates

Rust crates implementing the domain computer — the retrieval, calculation, and feature
engineering capabilities that agents use to work with the corpus without loading it
wholesale into context.

See [crates conventions](../.yidam/.vendor/prelude/guidelines/directories.md#crates) for the three
capability types (connectors, calculators, feature engineering) and the index layer.

## Shape of the domain computer

Four crates exist: [`succession`](succession/), [`covering`](covering/),
[`proximity`](proximity/) and [`provenance`](provenance/), described below. The rest of this
section names the shape the corpus's structure calls for, so the next one written is not
invented from scratch.

**Connectors.** Four external sources feed distinct classes. Decennial census and ACS
tables populate `measure` nodes against `place` and `jurisdiction`. Ohio Secretary of State
returns populate `measure` against `division`, which is the grain elections are actually
reported at. County auditor parcel and GIS exports anchor `site` nodes to ground. The
National Hydrography Dataset supplies the `flows-into` topology among `natural-feature`
nodes rather than leaving it hand-asserted.

**Calculators.** Four computations are specific to this domain and are not graph traversal
in disguise. A **succession audit** reads `office` and its `tenure` nodes and reports gaps
and overlaps in a line of holders — a defect no graph check can see, because each node is
well-formed and each edge resolves. A **covering query** reads the jurisdiction and division
edges around a place and reports what lay over it and what the corpus can date, which is the
same distinction one layer out. A **proximity ranking** orders nodes by great-circle distance and exists as much to refuse the
containment question as to answer the distance one. A **boundary-comparability check** reads a
`measure`,
the `place` it describes, and the annexation `event` nodes between two dates, and reports
whether the two figures describe the same ground. Comparing a city's population across
census years without it is the most ordinary way this corpus could publish a false number.

**Checks.** Not anticipated at genesis, and now a third capability type alongside connectors
and calculators. [`provenance`](provenance/) fails the build on a corpus defect rather than
answering a question about the county. It exists because the rule it enforces — that an edge
must say what kind of claim it is — does not exist in the vendored `graph-lint`, and
`.yidam/.vendor/` is read-only. The domain computer carries the gate until upstream does.

**Index.** Not yet. A corpus this size is cheaper to read than to embed, and an index over it
buys nothing but a staleness surface. Add one when the corpus outgrows
direct reading — the marker to watch is a phase that spends more context locating nodes
than reasoning about them.

**First crate — written.** [`succession`](succession/) implements the succession audit. It was
first for the reasons given when it was proposed — no network, no credentials, no fixtures, so
it is testable the day it is written — and it was written once the corpus held a real roster to
test against rather than when it was proposed.

The audit itself is pure and takes terms and a seat count; reading `.yidam/corpus/` lives in a
separate `load` module so the deciding logic is testable without a corpus on disk. Its expected
result was known before it ran: the sheriff line had been checked by hand at extraction and
found continuous, and `tests/corpus.rs` pins that so a later edit breaking the line fails here.

Two things it exists to get right, both about the roster's year precision:

- A shared boundary year is **contiguity, not overlap**. Terms are half-open intervals
  `[began, ended)`, so O'Neill 1889–1893 and Fisher 1893–1898 do not collide. Under closed
  intervals all 38 adjacent pairs in the line would report as overlaps and every one would be
  an artifact of the source rather than a fact about the office.
- The same precision means a point query is genuinely ambiguous, so `holders_in` uses **closed**
  intervals and returns both claimants for 1893. Two models on purpose, for two questions.

Writing it found one defect that reading could not have: `Option`'s ordering puts `None` first,
so the current holder's open-ended term sorted ahead of a single-year 2017 predecessor. The
corpus test caught it.

**Fourth crate — written.** [`provenance`](provenance/) is the first thing here that checks the
corpus instead of querying it. Its subject is an asymmetry the conventions created: `CLAUDE.md`
says an edge is a claim, prose claims must be tagged and cited with `verified-unsourced` as a
lint error, and links carried `target` and `relationship` and nothing else. The graph a
calculator walks was the only part of the corpus exempt from its own discipline, and every
location error of six phases lived there.

It exits 1 on findings, deliberately unlike `succession-audit`, which exits 0 because a gap in a
roster is a fact about the record. An untagged edge is not a fact about anything.

Its shape table is the readable output: every `class --relationship->` with its tag counts, which
is a map of where the corpus is guessing. Every edge the `event` and `period` classes own is an
inference, which is not a defect and is the clearest statement available of which half of this
corpus rests on a source.

**Third crate — written.** [`proximity`](proximity/) ranks corpus nodes by distance, using the
vendored `geodesics` haversine rather than a local copy, and is the first crate here to depend on
`.yidam/.vendor/` by path.

Its subject is the refusal. Ranked from the tank plant's address point, the candidate places come
out Fort Shawnee 2.20, Lima 2.37, Shawnee Township 2.52 miles — and the plant is in Shawnee
Township, last of three, by a spread of 0.32 miles. Nothing about that ranking looks unsafe, and
this corpus was wrong about that plant from genesis through three corrections before a boundary
source settled it. So the crate reports scale, reports distance, and says in its output that
neither is containment.

The corpus test proves the point in both directions in one query: the refinery reads as inside
Lima's scale and is not in Lima, and as outside Shawnee Township's scale, which is the township
it is in.

Writing the tests corrected the crate's own story twice — the right answer ranks third rather
than second, and the plant's two coordinates, 0.81 miles apart and describing one installation,
give opposite scale readings while the containment answer never moves.

**Second crate — written.** [`covering`](covering/) implements the covering query, run as
`jurisdiction-at`. Its subject is not the traversal, which is short, but the fact that almost
every edge it walks is undated: `place governed-by jurisdiction` says the City of Lima governs
Lima and never says since when. Members the corpus dates and members it merely asserts come
back in separate lists and never merge, so asking what governed Lima in 1900 returns one dated
answer, two undated ones, and three 2020 districts set aside rather than silently dropped.

It is the crate where the two calculators disagree on purpose. `succession` reads a missing
`ended` as running to the present; `covering` reads a missing `effective_to` as recording
nothing at all, because the corpus wrote down why its 2020 districts have no end date and it is
not that they still stand. Absent fields do not have one meaning across a corpus, and pretending
they do is how a calculator publishes a false statement while every test passes.

Writing it found a contradiction in the corpus that forty-one commits had not:
`place/delphos.yml` and `place/bluffton.yml` each said in verified prose that they cross a
county line, and each linked to the county with an unqualified `within`. The edge and the
description in the same file disagreed. Both are `partially-within` now, and the query reports
county-scale coverage of either place as partial — including the state and congressional
districts, which inherit the split.

## Crates

<!-- REGEN: yidam crates-index
Regenerated by: `yidam crates-index`
Fields per crate: name, capability type (connector/calculator/feature-engineering/index),
                  description, key external dependencies, test coverage.
-->
| Crate | Description |
|---|---|
| [—](crates/) | — |
| [covering](covering/) | Every jurisdiction and division covering a place, and what the corpus dates |
| [proximity](proximity/) | Corpus nodes within a radius of a point, ordered by distance |
| [succession](succession/) | Gaps and overlaps in an office's line of holders, from its tenure nodes |
<!-- /REGEN -->

## Index status

<!-- REGEN: yidam index-status
Regenerated by: `yidam index-status`
Fields: index backend, embedding model, indexed node count, freshness (HEAD vs last
        indexed commit), stale node count, retrieval latency (p50/p95 last benchmark).
-->
_Index not initialized. Run `yidam index-build` to build._
<!-- /REGEN -->
