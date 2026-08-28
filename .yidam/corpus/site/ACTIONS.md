# Actions — site

**Queries**
- Every site `located-in` a place; every site `part-of` a larger work.
- Every site `operated-by` an organization or a jurisdiction, and the succession of operators.
- Every `event` that `occurred-at` a site.

**Transitions**
- Change of operator: add a new `operated-by` edge; keep the old one. A site's operator
  history is the substance of its industrial biography.
- Closure or demolition: set `ceased` and update `status`. The node stays.

**Skills and calculators**
- `proximity` — what else is near this site.

**Cautions**
- `built` is the completion or opening date, not the date construction started, and for a
  work rebuilt after fire or expansion the single field cannot carry the whole story. Put
  that in the body.
