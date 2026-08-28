# Actions — organization

**Queries**
- Every `site` an organization `operated-by`; every place it is `seated-in`.
- The `succeeded-by` chain through renames, mergers and acquisitions.
- Every person `affiliated-with` it, and every `measure` that `describes` it.

**Transitions**
- Rename: if the legal identity persists, add to `also_known_as`. If a new entity took over,
  write a new node and link `succeeded-by`. Getting this wrong collapses two companies into
  one or splits one into two.
- Dissolution: set `dissolved`; keep the node and its edges.

**Cautions**
- A corporate successor is not always the operator of the same site, and the operator of a
  site is not always its owner. Do not infer one from the other.
