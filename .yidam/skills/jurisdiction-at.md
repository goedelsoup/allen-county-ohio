---
name: jurisdiction-at
description: Return every jurisdiction and division covering a place on a given date
---

# jurisdiction-at

**Computes.** Every jurisdiction and division covering a given place, with what the corpus
records about *when* each of them covered it.

**Reads.** A [`place`](../corpus/place/) node — or a [`site`](../corpus/site/), which resolves
to the place it stands in — and from there the `governed-by`, `within`, `partially-within`,
`serves`, `covers`, `territory-within` and `nested-in` edges around it.

**Run it.**

```
cargo run --manifest-path crates/Cargo.toml --bin jurisdiction-at -- lima
cargo run --manifest-path crates/Cargo.toml --bin jurisdiction-at -- lima 1900
cargo run --manifest-path crates/Cargo.toml --bin jurisdiction-at -- allen-county-courthouse
```

Implemented by [`crates/covering`](../../crates/covering/). Omitting the year asks the undated
question — what covers this ground at all — and **does not mean "now"**. There is no honest way
for this tool to answer a question about the present: the corpus does not know whether its 2020
districts still stand, and [says so](../corpus/question/allen-county-current-congressional-district.yml).

## The answer is a set, and it is in two halves

Overlapping members are normal and the result is never collapsed into a containment chain. But
the division that matters more is temporal. Almost every edge in this corpus is undated —
`place governed-by jurisdiction` records that the City of Lima governs Lima and not since when
— so members the corpus *dates* and members it merely *asserts* are returned separately and
never merged.

Asking what governed Lima in 1900:

```
covering, and dated to include 1900:
  jurisdiction/allen-county-government.yml   nested · whole · recorded from 1820, no end recorded

covering, but the corpus never dated it — no evidence either way about the year:
  jurisdiction/city-of-lima.yml              asserted · whole · no dates recorded
  jurisdiction/lima-city-school-district.yml asserted · whole · no dates recorded

not covering in 1900 — recorded dates rule it out:
  division/ohio-congressional-district-4-2020.yml   ... recorded from 2020
  division/ohio-house-district-4-2020.yml           ... recorded from 2020
  division/ohio-senate-district-12-2020.yml         ... recorded from 2020
```

One dated answer, two undated ones, three set aside. A single sorted list would have been a
lie with exactly the same contents.

## What each member carries

- **Reach** — `asserted` (an edge to the place itself), `nested` (a chain of authority edges out
  of an asserted member), or `inherited` (derived from a containing place). Inheritance is an
  inference: ground inside a larger place is *normally* under its authorities, and an
  incorporated village inside a township is normally not. A path is only as strong as its
  weakest step, so nesting out of an inherited member stays inherited.
- **Extent** — `whole`, or `PARTIAL` naming the edge that said the ground is split. Delphos and
  Bluffton cross county lines, so nothing at county scale covers either whole — not the county
  government and not the districts drawn over the county.
- **Warrant** — the dates the member's own node records, and whether the year falls inside.
  Intervals are closed at both ends, as in `succession`'s point query and for the same reason:
  these are year-precision dates and a boundary year belongs to both sides.

**A missing end date is not "still in force".** This is where the calculator deliberately parts
company with [`succession-audit`](succession-audit.md), where an open `ended` runs to the
present because the last entry in a sheriff's roster is the sitting sheriff. Here the same
absent field means only that the corpus never recorded an end.

## What it still cannot do

A site's answer is its **place's** answer, and the result says so. It is coarser than the
site's own ground: it cannot tell you which ward an address falls in, only which authorities
lie over the place containing it. That needs boundary geometry, which is what
[`proximity`](proximity.md) is waiting on too.
