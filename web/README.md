# web

A public reading of this repository's corpus: what can be established about Allen County,
Ohio, and — as plainly as the same page can put it — what cannot.

See [web conventions](../.yidam/.vendor/prelude/guidelines/directories.md#web) for what belongs here.

## Status: a static site over the exported feeds

Six reading pages, thirty-seven shorter articles and three instruments, built by
[Astro](https://astro.build) into `dist/` with no server behind them. [deck.gl](https://deck.gl)
draws the county; [Plotly](https://plotly.com/javascript/) draws its measures. Nothing here
reaches the network at build time or at read time — and that sentence is now checked rather
than asserted. It was false for the life of the design port, which carried the five Atlas faces
in as a Google Fonts `<link>`: a public civic reference telling a third party about every
reader on every page load, and the one thing the site still had to fetch from elsewhere to
render as designed. The faces are self-hosted in `public/fonts/`, written by `mise run fonts`,
and `test/register.test.ts` fails the build on a link to anywhere.

**A reading page is an argument**: one question, movements that turn the answer, and an ending.
It holds at most 1,800 words of prose and eight movements, and `test/shape.test.ts` is what
holds it to that. Anything that would not serve the question is an article at `/read/<slug>` —
which is the content type this directory did not have, and whose absence is the whole of the
problem recorded in
[a-page-is-an-argument-not-an-inbox](../.yidam/decisions/a-page-is-an-argument-not-an-inbox.yml).

| Page | The question it answers |
|---|---|
| `/` | What is this place, and what does this site claim to know about it? |
| `/ground` | Two thirds of the county is farms — so what happens to the lines drawn on it? |
| `/people` | The county is shrinking. Is it also sorting? |
| `/work` | What happened to the factory county, and what stands in its place? |
| `/government` | Who governs this county, how are they chosen, and what does it cost? |
| `/history` | What can this corpus date, and what does the shape of the gaps mean? |
| `/read` | The reading room: every shorter piece, by section and era |

`/` is not in that list. **It is the map**, reached from the wordmark rather than from a tab — a
front door listed among five destinations is one of six things rather than the way in. The county's
own summary sits under it: four tiles, one opening claim, three assertions. See
[`the-map-is-the-index`](../.yidam/decisions/the-map-is-the-index.yml).

And the two instruments, set apart from the reading row in the nav because a reader reaches
for them rather than reading them:

| Instrument | What it is for |
|---|---|
| `/entry` | Every node the corpus publishes, by class |
| `/sources` | The audit: what is sourced, what is inferred, what was withheld |

**The margin, not the main line.** The site records what a claim cost to get — and that belongs
beside the claim rather than in front of it. `<Gloss>` is where it goes, and
`test/register.test.ts` keeps acquisition narration out of the reading pages' prose. What it
bans is phase numbers and what this corpus used to hold; the site's own epistemic voice — *this
page will not tell you which district* — is the point of the site and stays.

```
mise run site-install     # once
mise run site-dev         # localhost:4321
mise run site             # lint, test, build — what CI runs
```

## The two things this file said to settle first

They were settled before a page existed, which is why this section is longer than the one
above. The decision is recorded in
[what-may-leave-the-repository](../.yidam/decisions/what-may-leave-the-repository.yml).

**The data source is a bundled feed with a contract version, not the corpus read live.**
Nothing under `web/` parses a node file. [`crates/publish`](../crates/publish/) reads
`.yidam/corpus/` and writes four JSON feeds into `src/feeds/`, each carrying a
`feed_version`; the site reads those and only those. The feeds are a pure function of the
corpus — no timestamp, no commit hash — so `cargo test` can compare the committed bytes
against a fresh derivation and fail when they differ. **The derivation is the gate**, which is
what the directory conventions ask of anything outside the corpus that derives from it.

The manifest carries the ontology across the same boundary: what each class declares itself
to be, read from `.yidam/corpus/<class>.ont.yml`. An entry page can then say what licenses
the structure it is rendering — that a tenure is a relator is *why* its dates sit on the
holding and not on the person — without this side of the line holding a second copy of it.

```
mise run publish          # rewrite src/feeds/ from the corpus
mise run publish-check    # report whether what is committed is current
```

**The audience is public, and the publication rules are enforced rather than remembered.**
Under [agent-conduct](../.yidam/.vendor/prelude/guidelines/agent-conduct.md), a derived
assertion travels only as far as the weakest claim beneath it, an external assertion cites a
verbatim span rather than a node, and a refusal in a cited node fails the build. All three are
checks in `crates/publish`, run on every build:

- A claim's tier is **computed** from the corpus, never declared, so a downgrade upstream
  propagates on the next build rather than whenever somebody remembers.
- Every assertion the site makes names the passage it rests on, and the gate asserts that
  passage still appears in that node character for character.
- Where a cited node refuses an inference, the assertion must answer it and the refusal is
  rendered beside what it qualifies.
- Every number plotted on a chart is quoted from a span the assertion already cites.

`[open]` does not leave the repository: 85 prose blocks and three structured fields are
withheld by rule, and a corpus test asserts the string appears in no feed. The site publishes
at the `inference` ceiling — the tier the rule allows *attributed* material — and earns it by
showing the tag, the span and the source under every claim.

## The geography is vendored, not fetched

`public/geo/` holds Allen County's 2020 census geography — the county, its thirteen civil
subdivisions, its municipalities and CDPs, all 88 voting districts, its 35 tracts and the
seventeen school districts that reach it — as GeoJSON, with `PROVENANCE.json` recording the
service, the query and the date beside it. It comes from
[TIGERweb](../.yidam/catalog/tigerweb-census2020.md), which the corpus already catalogues.

```
mise run boundaries       # re-vendor from TIGERweb. The only task here that uses the network.
```

It is committed so that a build needs no network and a Census Bureau outage cannot turn CI
red. `scripts/fetch-boundaries.mjs` refuses to write a layer whose feature count has changed,
because a silent recount is the failure this connector can actually have — the query still
succeeds and the map quietly loses a township.

**The water is the one layer that is not a 2020 statement.** TIGERweb keeps hydrography out of
the decennial services entirely — one Hydro MapServer, no vintage, no STATE or COUNTY column —
so `linear-water.geojson` records its own vintage as *current* in `PROVENANCE.json` rather than
inheriting the file's. It is also the one layer the service generalizes on request: 313
polylines arrive at 847 KB describing meanders a few metres across, in a frame where the whole
county is 1,400 pixels wide, and `maxAllowableOffset` asks for the resolution the map has. The
generalizing is done server-side and the offset is recorded, so the vendored file stays a mirror
rather than becoming a cartographer.

**The school districts are the one layer not cut at the county line.** Ohio draws a district to
hold a population, so twelve of the seventeen that reach Allen County also lie in another one and
only five are wholly this county's. They are vendored and drawn whole — the part of Pandora-Gilboa
inside Allen County is not a school district, it is the corner of one — and the county outline is
drawn over the top saying where the county is. See
[`a-district-is-not-cut-at-the-county-line`](../.yidam/decisions/a-district-is-not-cut-at-the-county-line.yml).

**A shape is addressed by `level:geoid`, never by GEOID alone.** A Census key is unique only
inside its summary level, and this county holds the proof: `3904752` is Beaverdam village *and*
the Upper Scioto Valley Local School District. The atlas feed carries the level beside every key
and `feeds.ts::censusKey` is where the pair is spelled. See
[`a-geoid-is-not-an-address`](../.yidam/decisions/a-geoid-is-not-an-address.yml).

**The join is the site's own gate.** `crates/publish` cannot see `public/geo/` and the
connector cannot see the corpus, so the one derivation performed in this directory is matching
a corpus key to a vendored geometry — and `test/geography.test.ts` is what holds it. It also
pins the fact that the county's geography contains entities the corpus does not name:
Cridersville village crosses in from Auglaize County, and five school districts reach in from
Hardin, Hancock, Putnam and Auglaize. The map shows all of them as ground.

## What the map is for

It is a corpus-correctness instrument before it is a presentation one, which is what this
file predicted and is the reason it exists. The worked case is the
[Lima refinery](../.yidam/corpus/site/lima-refinery.yml): a Lima postal address, a `located-in`
edge that said Lima from genesis until two authorities were asked, and ground that is in
Shawnee Township. A table of edges cannot show that. A dot outside a boundary can.

The corollary is the one the corpus states about itself, and the map page states it too:
nearness is not containment. A dot inside a shape is a dot inside a shape, and where a
location claim is an inference its badge says so.

**It now carries a year**, and the same rule governs the second axis. The six tiles are the
design system's era ramp, which tiles the whole range; the corpus's seven `period` nodes ride on
top as named overlays, because they overlap and leave holes and were never a partition. Time is
deliberately not linear across the axis — half the record sits in one decade — and the density
ribbon under it says so before a reader travels through a thin century and concludes the page is
broken.

Three things are kept apart at every year, because collapsing them would be a claim nobody made:
what the corpus places *here*, what it places *elsewhen*, and what it cannot date at all. The
last is a gap in the record rather than in the county, and it has its own toggle and its own ink.
Every derived placement carries its route, and the *Placement* control shrinks the map to the 72
positions somebody actually stated.

The view is in the URL. `/?year=1885&grain=year&at=place/lima.yml` is Lima in the year the oil came
in, with the panel open on it — and that is what a reading page links to now, through
`<OnTheMap>`. Every such link is held to naming a node the feed publishes, a year the axis covers,
and a node that is actually standing in that year; and every reading page has to carry at least
one, because a front door nobody is sent back to from inside the house is still only a front door.

`/map` still works. It is a hand-written stub rather than an `astro.config` redirect, because that
kind emits a fixed URL and drops the query — and dropping the query on this map means dropping the
year.

`src/lib/eras.ts` holds all of it as pure functions over `feeds/atlas.json`, testable without a
canvas; `src/scripts/map.ts` is the deck.gl scene and nothing else.

**It also carries the graph.** `src/lib/edges.ts` classifies every relationship the feed publishes
by what it draws on the ground — a line, nesting, a course, or nothing — with the argument beside
each and a gate on both halves of the table. Twelve edges are drawn as lines, and only between two
positions the corpus *states*: a line between derived marks would join two guesses, and nothing on
a map can say a line is three inferences long. Two of the twelve leave the shape they name, which
is the corpus-correctness case this file predicted.

The refusals are the bigger half and the page prints the ledger. Containment is drawn by the
shapes themselves; drainage is not drawn at all, because the corpus holds the topology and TIGER
holds the geometry — see
[`a-river-is-not-at-its-mouth`](../.yidam/decisions/a-river-is-not-at-its-mouth.yml).

## Bundle status

<!-- REGEN: yidam bundle-status
Regenerated by: `yidam bundle-status`
Fields: bundle contract version, feed list, last export timestamp, node counts per feed,
        deployment target, last deploy status.
-->
_No bundle. Run `yidam bundle` to produce one._
<!-- /REGEN -->
