# Actions — office

**Queries**
- The full line of holders, by following `tenure` nodes that point at this office.
- Every office `established-within` a jurisdiction or organization.
- The `succeeded-by` chain where an office was abolished and its duties transferred.

**Transitions**
- A change in term length or selection method is a change to the office, dated in the body
  and usually tied to an `event` — a charter amendment, a statute.
- Abolition: set `abolished` and link `succeeded-by` to whatever took the duties.

**Skills and calculators**
- `succession-audit` — gaps and overlaps in this office's line of holders. The gate cannot
  see these: every tenure node is individually well-formed and every edge resolves.

**Cautions**
- `seats` matters. A three-seat board with staggered terms produces overlapping tenures that
  are correct, and a succession audit that does not read `seats` will report them as defects.
