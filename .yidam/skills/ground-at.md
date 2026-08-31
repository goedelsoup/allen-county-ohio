---
name: ground-at
description: The survey section a point stands on, and the Recorder volume that abstracts it
---

# ground-at

**Computes.** The rectangular-survey section a point stands on — township, range, section — and
which of the Recorder's thirty Section Ground volumes abstracts that ground.

**Reads.** A [`place`](../corpus/place/) or [`site`](../corpus/site/) node's `coordinates` or
`centroid`, or a literal point, or a bare section code. Nothing else: this skill does not touch
the graph's edges, because ground is not a jurisdiction question.

**Run it.**

```
cargo run --manifest-path crates/Cargo.toml --bin ground-at -- lima-army-tank-plant
cargo run --manifest-path crates/Cargo.toml --bin ground-at -- 40.6994478,-84.137903
cargo run --manifest-path crates/Cargo.toml --bin ground-at -- 4614
```

Implemented by [`crates/ground`](../../crates/ground/).

```
Joint Systems Manufacturing Center (Lima Army Tank Plant)
  40.699448, -84.137903

  T4S R6E §14
    layer 55 writes it   4614
    a parcel number      4614xxxxxxxxxx
    section area         648.5 acres

  Section Ground volume 34A
```

That last line is the point of the tool. It is the answer to "where would I go to read what has
happened to this ground", and it is one lookup away from a question this corpus has been unable
to close for two phases.

## What it is careful about

**It prints the section's acreage every time.** Layer 55 is a *label* layer, and one of its
polygons — T3S R8E §5 — is 1,282 acres, which is two sections carrying one section's number.
Anything over 700 acres is called out in words: the point is inside that label and its section is
not established. [Beaverdam](../corpus/place/beaverdam.yml) is the case.

**It reports overlaps instead of resolving them.** `sections_at` returns every polygon containing
the point. The single-answer form returns nothing when two claim it, because picking the first
would resolve a defect in the source by iteration order.

**It never compares the two forms of the code as strings.** The county writes T4S R6E §1 as `461`
on layer 55 and `4601` in a parcel number, and 96 of the county's 404 sections are written
differently in the two places. `Ground` parses and prints both.

**Books come back as a list.** The county's own finding aid puts T4S R5E §8 in two volumes and
T4S R5E §18 in none, so a lookup promising one answer would have to invent one.

## What it does not do

It does not read a tract page, and this crate touches no title, no ownership and no person. What
may be taken from the Recorder, and on what terms, is
[written down separately](../decisions/what-crosses-from-the-recorder.yml).

## The fixture

`crates/ground/fixtures/sections.json` holds only the sections some corpus node stands on —
currently 29 — rather than all 404. Rebuild it with `mise run ground-fixture` after giving a node
a coordinate. A point outside the fixture is reported as outside *the fixture*, which is not the
same as outside the county, and the tool says so.
