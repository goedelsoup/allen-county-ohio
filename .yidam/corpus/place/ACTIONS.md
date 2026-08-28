# Actions — place

**Queries**
- Every place `within` a given place — the township roll of the county, the villages of a township.
- Every place a `natural-feature` `traverses` — which settlements a stream passes.
- Every `site` `located-in` a place, and every `measure` that `describes` it.

**Transitions**
- A place is annexed into a municipality: record the `event`, then the boundary change. The
  place node is not rewritten — its `area_sq_mi` gains a new dated figure as a `measure`.
- A place is renamed: add to `former_names`; do not rename the file. Renaming severs edges.

**Skills and calculators**
- `jurisdiction-at` — every jurisdiction and division covering this place on a given date.
- `proximity` — corpus nodes within a radius of this place's `centroid`.
- `boundary-comparability` — whether two figures for this place describe the same ground.

**Cautions**
- A `(pt.)` figure is a county portion, not a municipal total. Check before comparing.
- The gazetteer's internal point is guaranteed inside the polygon; a centroid is not. Do not
  substitute one for the other in a concave township.
