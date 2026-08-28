# Actions — jurisdiction

**Queries**
- Every jurisdiction whose territory is `territory-within` another — the nesting of ground.
- Every `office` `established-within` a jurisdiction, and through those, its officeholders.
- The `event` that `erected` a jurisdiction, and any `succeeded-by` chain after a merger.

**Transitions**
- Erection: a new unit comes into legal existence. Write the `event` first, then the
  jurisdiction node pointing at it with `erected-by`.
- Consolidation: the absorbing unit gains a `succeeded-by` edge from the absorbed one. Both
  nodes stay — the superseded one is provenance, not clutter.
- Abolition: set `abolished`; do not delete.

**Skills and calculators**
- `jurisdiction-at` — the covering set for a place on a date.
- `succession-audit`, applied through this jurisdiction's offices.

**Cautions**
- Territorial nesting is not subordination. Ohio townships are creatures of the state, not
  of the county whose ground contains them; `territory-within` says only where the ground is.
