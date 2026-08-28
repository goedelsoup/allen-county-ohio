# Actions — event

**Queries**
- Every event `situated-in` a period; every event that `affected` a place.
- Every event a person or organization was `involved` in.
- The event that `erected` a jurisdiction, or `began-with`/`ended-with` a tenure.

**Transitions**
- An event's date is refined from a better source — a year becomes a full date. Rewrite the
  field and say in the body what superseded what. `date` accepts `YYYY`, `YYYY-MM` and
  `YYYY-MM-DD`, so record the precision actually known rather than padding it.

**Cautions**
- `occurred-in` is where it happened; `affected` is what it changed. The Treaty of St. Marys
  was signed in another county and made this one. Collapsing the two would put the county's
  founding somewhere it did not happen, or lose the connection entirely.
- An event with a duration uses `occurred_through`. A period is not a long event — if the
  node is an interpretation of a span rather than a happening, it belongs in
  [period](../period/).
