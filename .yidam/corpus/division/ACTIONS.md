# Actions — division

**Queries**
- Every division `nested-in` a jurisdiction, filtered to those effective on a given date.
- Every `measure` that `describes` a division — the figures published at this grain.
- Which division `covers` a given place, as of a date.

**Transitions**
- Redistricting: do not edit a division's boundary. Close the existing node with
  `effective_to` and write a new node for the new boundary. A boundary that changes in place
  destroys the only thing that made the old figures interpretable.

**Skills and calculators**
- `boundary-comparability` — whether two figures at this grain describe the same ground.
- `jurisdiction-at` — returns divisions alongside jurisdictions.

**Cautions**
- This class is nearly empty at genesis. See
  [`../../decisions/seed-scope.yml`](../../decisions/seed-scope.yml) — ward and precinct
  boundaries were not sourced, so the two implied edges that land here have almost nothing
  to connect.
