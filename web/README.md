# web

A public reading of this repository's corpus: what can be established about Allen County,
Ohio, and — as plainly as the same page can put it — what cannot.

See [web conventions](../.yidam/.vendor/prelude/guidelines/directories.md#web) for what belongs here.

## Status: a static site over the exported feeds

Six reading pages, thirty-seven shorter articles and three instruments, built by
[Astro](https://astro.build) into `dist/` with no server behind them. [deck.gl](https://deck.gl)
draws the county; [Plotly](https://plotly.com/javascript/) draws its measures. Nothing here
reaches the network at build time or at read time.

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

And the three instruments, set apart from the reading row in the nav because a reader reaches
for them rather than reading them:

| Instrument | What it is for |
|---|---|
| `/map` | The corpus's located nodes on the county's actual 2020 boundaries |
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
subdivisions, its municipalities and CDPs, and all 88 voting districts — as GeoJSON, with
`PROVENANCE.json` recording the service, the query and the date beside it. It comes from
[TIGERweb](../.yidam/catalog/tigerweb-census2020.md), which the corpus already catalogues.

```
mise run boundaries       # re-vendor from TIGERweb. The only task here that uses the network.
```

It is committed so that a build needs no network and a Census Bureau outage cannot turn CI
red. `scripts/fetch-boundaries.mjs` refuses to write a layer whose feature count has changed,
because a silent recount is the failure this connector can actually have — the query still
succeeds and the map quietly loses a township.

**The join is the site's own gate.** `crates/publish` cannot see `public/geo/` and the
connector cannot see the corpus, so the one derivation performed in this directory is matching
a corpus GEOID to a vendored geometry — and `test/geography.test.ts` is what holds it. It also
pins the fact that the county's geography contains one municipality the corpus does not name:
Cridersville crosses in from Auglaize County, and the map shows it as ground.

## What the map is for

It is a corpus-correctness instrument before it is a presentation one, which is what this
file predicted and is the reason it exists. The worked case is the
[Lima refinery](../.yidam/corpus/site/lima-refinery.yml): a Lima postal address, a `located-in`
edge that said Lima from genesis until two authorities were asked, and ground that is in
Shawnee Township. A table of edges cannot show that. A dot outside a boundary can.

The corollary is the one the corpus states about itself, and the map page states it too:
nearness is not containment. A dot inside a shape is a dot inside a shape, and where a
location claim is an inference its badge says so.

## Bundle status

<!-- REGEN: yidam bundle-status
Regenerated by: `yidam bundle-status`
Fields: bundle contract version, feed list, last export timestamp, node counts per feed,
        deployment target, last deploy status.
-->
_No bundle. Run `yidam bundle` to produce one._
<!-- /REGEN -->
