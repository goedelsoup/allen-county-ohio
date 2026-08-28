# jurisdiction-at (stub)

**Computes.** Every jurisdiction and division covering a given place on a given date.

**Reads.** A [`place`](../corpus/place/) node, the [`jurisdiction`](../corpus/jurisdiction/)
nodes it is `governed-by` and their `territory-within` chain, and the
[`division`](../corpus/division/) nodes that `cover` it, filtered by `effective_from` and
`effective_to`.

**Returns.** The covering set at that date — typically a municipality or township, the county,
a school district, and whatever reporting divisions were in force.

**Why it is a stub.** It would run today and return a thin answer. The corpus has the
jurisdiction spine but almost no division layer, and no `division covers place` edge is
written — that edge was approved at genesis and could not be defended for the one division
that exists, whose relationship to Sugar Creek Township is
[explicitly open](../corpus/division/census-tract-39003010300.yml).

**Design note.** The answer is a *set*, not a hierarchy, and overlapping members are normal.
A village, the township around it, the county and a school district can all cover one acre
with four different boundaries. Do not collapse the result into a single containment chain.
