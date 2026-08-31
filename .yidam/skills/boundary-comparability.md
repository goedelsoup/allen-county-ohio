---
name: boundary-comparability
description: Decide whether two measures describe the same ground, given the annexations between them
---

# boundary-comparability

**Computes.** Whether two figures for the same subject describe the same ground.

**Reads.** Two [`measure`](../corpus/measure/) nodes, the [`place`](../corpus/place/) or
[`jurisdiction`](../corpus/jurisdiction/) each `describes`, the
[annexation record](../corpus/measure/allen-county-annexations-1990-2024.yml), and every erection
or boundary-change [`event`](../corpus/event/) between their `as_of` dates.

**Returns.** Comparable, or not comparable, naming the intervening annexations — and a third
answer that matters as much as either: *no intervening changes found, which is not evidence
that none occurred.*

**What changed, and what did not.** This was written as a stub because the corpus held no
annexation events. It now holds the annexations — fifty-seven of them for the county between 1990
and 2024, dated, with instrument numbers — but as **one measure and not as fifty-seven event
nodes**, on the rule in
[a-line-of-holders-is-a-table](../decisions/a-line-of-holders-is-a-table.yml): a member earns a
node when something about *it* does not fit the table, and an annexation's date, acreage and
ordinance number all fit. So this skill reads a table where it was drafted to read a graph. That
is the second phase running to pay this cost, and it is recorded rather than engineered around.

**And the answer it returns is now usually *not comparable*, for a reason the drafter did not
anticipate.** Subtracting annexations from an area change does not recover the ground, because the
Census Bureau's area figures move for two causes and print as one number: Allen County's own land
area fell 1,235 acres between the 2000 and 2010 volumes without losing any land. Any run of this
skill must apply
[the control is the figure that cannot move](../decisions/the-control-is-the-figure-that-cannot-move.yml)
before it reports a difference.

**Why it matters here specifically.** Census figures for Lima are tabulated at the corporate
boundary of the date, so a population series for the city is a series about changing ground.
Comparing 1970 to 2020 without accounting for annexation measures the boundary as much as the
population — and the direction of the error flatters the decline narrative, which is the
direction to be most careful in.
