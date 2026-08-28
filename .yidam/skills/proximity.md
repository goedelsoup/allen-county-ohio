---
name: proximity
description: Return corpus nodes within a radius of a point, ordered by distance
---

# proximity (stub)

**Computes.** Corpus nodes within a radius of a point, ordered by distance.

**Reads.** `centroid` on [`place`](../corpus/place/) and `coordinates` on
[`site`](../corpus/site/), both in decimal degrees.

**Returns.** Nodes within the radius, with distances.

**Calls.** `haversine_km` from the vendored
[`geodesics`](../.vendor/prelude/domains/geodesics/) domain library — the only prelude domain
this repository vendored, and the reason it was vendored. Do not reimplement the formula; the
library is pinned across Rust, TypeScript and Python by shared parity fixtures.

**Why it is a stub.** No `site` node carries coordinates yet. Every place node has a verified
internal point, so the place-to-place case would work today; the site case, which is the
interesting one, waits on the [`auditor-parcels`](../../crates/auditor-parcels/) or
[`nrhp`](../../crates/nrhp/) connector.

**Design note.** The gazetteer publishes an *internal point*, guaranteed to fall inside the
polygon — not a centroid, which for a concave township may fall outside it. The distinction
does not matter for ranking nearby nodes and does matter for any claim that a coordinate lies
within the thing it names. This calculator does the first and must not be read as doing the
second.

**Worked precedent.** The comparison in
[`census-tract-39003010300`](../corpus/division/census-tract-39003010300.yml) — roughly three
quarters of a mile between the tract's internal point and Sugar Creek Township's — was done by
hand at genesis and is exactly what this calculator automates.
