---
name: boundary-comparability
description: Decide whether two measures describe the same ground, given the annexations between them
---

# boundary-comparability (stub)

**Computes.** Whether two figures for the same subject describe the same ground.

**Reads.** Two [`measure`](../corpus/measure/) nodes, the [`place`](../corpus/place/) or
[`jurisdiction`](../corpus/jurisdiction/) each `describes`, and every annexation, erection or
boundary-change [`event`](../corpus/event/) between their `as_of` dates.

**Returns.** Comparable, or not comparable, naming the intervening events — and a third
answer that matters as much as either: *no intervening events found, which is not evidence
that none occurred.*

**Why it cannot be run meaningfully yet.** The corpus holds six measures and **no annexation
events at all**. Every comparison would return the third answer, which is honest but
uninformative. [`lima-land-area-2020`](../corpus/measure/lima-land-area-2020.yml) says this in
its own body.

**Why it matters here specifically.** Census figures for Lima are tabulated at the corporate
boundary of the date, so a population series for the city is a series about changing ground.
Comparing 1970 to 2020 without accounting for annexation measures the boundary as much as the
population — and the direction of the error flatters the decline narrative, which is the
direction to be most careful in.
