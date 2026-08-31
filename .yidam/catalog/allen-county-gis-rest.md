---
name: Allen County GIS — ArcGIS REST map service
description: >-
  The county's live spatial service, sixty-nine layers. Two of them matter here: layer 55, the
  survey sections, which is the first thing this corpus has held that says what *ground* a point
  stands on; and layer 51, the parcels, which carries no owner and is the independent witness
  against layer 55.
type: api
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AllenCountyGIS/MapServer?f=json
    description: Service root. Sixty-nine layers; `maxRecordCount` is 1000 per query.
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AllenCountyGIS/MapServer/55/query?where=1%3D1&outFields=*&returnGeometry=true&outSR=3734&f=json
    description: >-
      Layer 55, Section Numbers — 407 polygons with `Township`, `Range`, `T_R_S` and the section
      number in `TEXTSTRING`. The whole layer returns in one request under the record cap.
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AllenCountyGIS/MapServer/51/query
    description: >-
      Layer 51, Parcels — `PARCEL_NO`, `Acres`, geometry, and **nothing else**. Queried here a
      point at a time with a geometry filter rather than in bulk.
---

**Why this and not the download page.** [The bulk downloads](allen-county-gis-downloads.md) carry
political subdivisions and address points, and this corpus took those. They do not carry the
survey. The REST service does, and it also answers a point-in-polygon question directly, which
means a single fact can be retrieved without taking a dataset.

**What layer 55 is, and what it is not.** It is called *Section Numbers* and it is the map
annotation layer for the rectangular survey — one polygon per section, carrying the number that
gets drawn on it. It is therefore the survey grid **as the county labels it**, which is nearly but
not exactly the survey itself. See the defect below.

**Neither layer used here carries a person.** Layer 51's fields are `PARCEL_NO`, `Acres` and
geometry: no owner, no deed reference, no dates. That is what makes it usable at all under
[the access-terms decision](../decisions/auditor-parcels-access-terms.yml), which named the
county's two *owner* files and recorded that they were never fetched. They still have not been.

## The grid closes against the county

| | |
|---|---|
| polygons returned | 407 |
| of those, zero-area duplicates | 3 — `454`, `3729`, `4413`, each paired with a real one |
| real sections | **404** |
| township-range cells | 15 — T1S–T4S by R4E–R8E |
| section numbers | 1 through 36 |
| total area | **406.805 sq mi** |
| the county, from its own gazetteer record | 402.545 land + 4.306 water = **406.851** |

Forty-six thousandths of a square mile apart, on two sources that share no method: the Census
Bureau computed the county's area from its own boundary file and the county computed each
section's from its own survey. That is the arithmetic self-closure this corpus uses in place of a
second opinion, and it is what licenses treating layer 55 as the survey rather than as decoration.

Mean section: **639.7 acres**, against a nominal 640.

## One polygon is two sections

T3S R8E §5 is **1,282.2 acres on a footprint 1.04 by 2.01 miles**, where its thirty-five
neighbours in that township average 641 and none of them is missing. It is one section's number
drawn over two sections' ground. Nine further polygons run 700–740 acres; the other 394 fall
between 560 and 700.

[Beaverdam](../corpus/place/beaverdam.yml) stands inside the bad one, so the corpus does not
assert Beaverdam's section. `crates/ground` reports the acreage with every answer and says
outright when a polygon is too large to be a section.

## The code is written two ways and they are different strings

Layer 55's `T_R_S` is township, range and the section **as written** — `461` for T4S R6E §1.
A parcel number's first four digits pad it — `4601`. Ninety-six of the county's 404 sections have
a single-digit number, so a string join between the two forms drops 24 per cent of the county and
reports nothing. `ground::Ground` parses and prints both and refuses to compare them as strings.

## Two witnesses, and where they disagree

Every one of this corpus's 29 nodes that carries a point was located twice: by polygon against
layer 55, and by the first four digits of whatever parcel in layer 51 contains the same point.
**Twenty-seven agree.** The two that do not are the interesting ones:

    Beaverdam    layer 55 says T3S R8E §5, and no parcel contains the point at all —
                 it is the oversized polygon, and there is no second witness to break it
    Bluffton     layer 55 says T2S R8E §1; the parcel there is 2812, which is §12,
                 directly south. The village straddles the Allen–Hancock line and its
                 internal point is the whole village's, so it sits near that section
                 line rather than inside either section with any margin. [open]

## used-by

- `crates/ground` — the projection, the section lookup, and the Recorder book mapping
- [`ground-at`](../skills/ground-at.md)
- [Allen County survey sections](../corpus/measure/allen-county-survey-sections.yml)
- [Fort Amanda](../corpus/site/fort-amanda.yml) — the point-in-polygon that returns nothing, and
  the binary search along -84.27 that puts the county line at 40.685776
- [Amanda Township](../corpus/place/amanda-township.yml)
