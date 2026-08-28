# Actions — period

**Queries**
- Every `event` `situated-in` a period.
- The chain of periods through `precedes`.
- Every `measure` whose `as_of` falls inside a period's bounds.

**Transitions**
- Re-bounding a period is a `revise:` and needs an argument in `boundary_basis`, not just new
  numbers. Moving a boundary silently reclassifies every event inside it.

**Cautions**
- Periods overlap, and that is not an error. An oil boom and a canal era can both be running.
- A period's `character` is the corpus asserting an interpretation. Tag it. `[inference]` is
  the honest tag for most period characterizations, and untagged is the one thing it must
  not be.
