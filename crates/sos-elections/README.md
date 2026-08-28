# sos-elections — connector (stub)

**Source.** Ohio Secretary of State, official county-by-precinct election results.

**Feeds.** `measure` nodes at `division` grain — the only source that publishes at the grain
the [division](../../.yidam/corpus/division/) class exists for.

**Why it is a stub and will stay one for a while.** It has nothing to attach to. The corpus
holds one division node, a census tract, and no ward or precinct at all — see
[`seed-scope.yml`](../../.yidam/decisions/seed-scope.yml). Two of the eight implied edges
approved at genesis (`measure describes division`, `division covers place`) are unwritten for
the same reason. Writing this connector before precinct boundaries exist would produce
figures with nowhere to land.

**Order of work.** Precinct boundaries first, then returns. A return keyed to a precinct name
with no boundary node is an orphan figure.

**Retrieval interface.**
```
fn results(county: &str, election: &Date) -> Result<Vec<PrecinctResult>>
struct PrecinctResult { precinct: String, contest: String, choice: String, votes: u32 }
```

**Caution.** Precincts are redrawn between elections and names are reused. A `PrecinctResult`
is meaningless without the boundary in force at that election, which is why `division`
declares `effective_from` and `effective_to`.
