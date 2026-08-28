# watershed-trace (stub)

**Computes.** The downstream chain from any natural feature to its outlet.

**Reads.** [`natural-feature`](../corpus/natural-feature/) nodes and their `flows-into` edges,
plus the `traverses` edges to places along the way.

**Returns.** The ordered chain to the outlet, and the places traversed.

**Why it is a stub even though the corpus can already answer it.** The trace exists and is
short: [Ottawa River](../corpus/natural-feature/ottawa-river.yml) →
[Auglaize River](../corpus/natural-feature/auglaize-river.yml) →
[Maumee River Basin](../corpus/natural-feature/maumee-river-basin.yml), discharging to Lake
Erie. Running a calculator by hand and committing the result as `compute:` would put a figure
in the corpus that no code can reproduce, which is worse than no figure — so the chain is
recorded in the node bodies as prose and this stays a stub until there is a crate.

**The claim the trace rests on is not verified.** Every `flows-into` edge above is
`[inference]`. Whether any part of Allen County drains south to the Ohio instead is open. The
[`nhd` connector](../../crates/nhd/) is what would settle both, and it should land before this
calculator is trusted.
