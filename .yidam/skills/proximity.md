---
name: proximity
description: Return corpus nodes within a radius of a point, ordered by distance
---

# proximity

**Computes.** Corpus nodes ranked by distance from a point, with bearing.

**Reads.** `centroid` on [`place`](../corpus/place/) and `coordinates` on
[`site`](../corpus/site/), both in decimal degrees. Nothing else — see *What it will not read*.

**Run it.**

```
cargo run --manifest-path crates/Cargo.toml --bin proximity -- lima-refinery
cargo run --manifest-path crates/Cargo.toml --bin proximity -- lima --radius-mi 5
cargo run --manifest-path crates/Cargo.toml --bin proximity -- 40.7085,-84.1280 --limit 5
```

Implemented by [`crates/proximity`](../../crates/proximity/). Calls `haversine_km` from the
vendored [`geodesics`](../.vendor/prelude/domains/geodesics/) domain — the only prelude domain
this repository vendored, and the reason it was vendored. **Do not reimplement the formula**;
it is pinned across Rust, TypeScript and Python by shared parity fixtures.

## It ranks. It does not tell you what a point is inside.

This is the whole discipline of the tool, and the corpus paid for it. Ranked by distance from
the tank plant's address point:

```
   2.20 mi   SW  place/fort-shawnee.yml       [internal point — 9.5 sq mi, scale 1.74 mi]
   2.37 mi    N  place/lima.yml               [internal point — 13.6 sq mi, scale 2.08 mi]
   2.52 mi    W  place/shawnee-township.yml   [internal point — 29.0 sq mi, scale 3.04 mi]
```

**The plant is in Shawnee Township** — last of the three, by 0.32 miles. Nothing about that
ranking looks unsafe, which is why this corpus was wrong about that plant from genesis through
three separate corrections. See
[a-postal-address-is-not-a-municipality](../decisions/a-postal-address-is-not-a-municipality.yml).

The reason is what the coordinate means. The gazetteer publishes an **internal point**,
guaranteed to fall inside the polygon and otherwise unconstrained. It is not a centroid, and no
bound relates distance-from-it to membership. A township is thirty square miles of arbitrary
shape.

## The scale annotation is not a hint

Each area node is annotated with the radius of a circle of the same land area — its scale in one
number — and flagged when the distance is smaller than that. **That flag is not a containment
signal in either direction**, and the refinery proves both directions in one query: it reads as
*inside Lima's scale* and is not in Lima, and as *outside Shawnee Township's* scale, which is the
township it is in. `tests/corpus.rs` pins exactly that.

For containment, ask a boundary source. This corpus uses
[TIGERweb](../catalog/tigerweb-census2020.md), and the county's own address file carries a `MUNI`
column that agrees with it.

## What it will not read

`natural-feature` is excluded. A stream's `mouth` is where it *ends*, not where it is — the
Ottawa River runs through the middle of Lima and discharges in Putnam County, so ranking against
that coordinate would report the county's principal watercourse as far from the city it flows
through. The corpus made the same refusal for the
[Miami and Erie Canal](../corpus/site/miami-and-erie-canal.yml), whose single GNIS point sits 34
miles from the reach this corpus cares about. **One point does not locate a line.**

`division` carries no coordinate property; the tract's internal point lives in its prose.

## Worked precedent

The comparison in
[`census-tract-39003010300`](../corpus/division/census-tract-39003010300.yml) — roughly three
quarters of a mile between the tract's internal point and Sugar Creek Township's — was done by
hand at genesis. `tests/corpus.rs` reproduces it, which is the check that this tool automates the
same arithmetic rather than a different one.
