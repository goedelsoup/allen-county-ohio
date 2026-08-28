# Actions — natural-feature

**Queries**
- The downstream chain from any feature, by following `flows-into` to an outlet.
- Every place a feature `traverses`; every `site` sited `on` one.

**Transitions**
- A feature is renamed or a former name is recovered from the record: add it to the body and
  keep the file name. Streams in this county have carried several names.
- A wetland is drained or a channel straightened: that is an `event`, and the feature node
  gains an edge to it rather than being rewritten.

**Skills and calculators**
- `watershed-trace` — the ordered chain to the outlet and the places traversed.

**Cautions**
- `flows-into` is a claim about hydrology and should rest on a source, not on a map read at
  a glance. Where the corpus has not verified it, say so — the National Hydrography Dataset
  is the connector proposed to settle it.
