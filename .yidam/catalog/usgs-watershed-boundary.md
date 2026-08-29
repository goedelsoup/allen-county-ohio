---
name: USGS Watershed Boundary Dataset, via The National Map
description: >-
  The federal hydrologic unit hierarchy as a keyless ArcGIS REST service — every scale from
  two-digit region to sixteen-digit unit. It is the source that says which ocean a piece of
  ground drains to, which this corpus asserted from genesis without one.
type: api
obtained: true
retrieved: 2026-08-29
ttl_days: 1825
location:
  - kind: url
    value: https://hydro.nationalmap.gov/arcgis/rest/services/wbd/MapServer
    description: layers 1–8 are HUC 2, 4, 6, 8, 10, 12, 14 and 16; layer 0 is WBDLine
used-by:
  - ../corpus/natural-feature/auglaize-river.yml
  - ../corpus/natural-feature/maumee-river-basin.yml
  - ../corpus/natural-feature/ottawa-river.yml
  - ../corpus/natural-feature/scioto-river-basin.yml
  - ../corpus/place/auglaize-township.yml
---

Queried exactly like [TIGERweb](tigerweb-census2020.md) and for the same reason: a point or a
polygon goes in and the unit containing it comes back. No key, no registration, no rate limit
encountered across the forty-odd queries this phase made.

**What it settled.** [`maumee-river-basin`](../corpus/natural-feature/maumee-river-basin.yml)
has carried this since genesis:

    [open] ... whether any part of Allen County drains instead to the Ohio River. Western Ohio
    carries the divide between the two, and the corpus asserts the Lake Erie side for this
    county without having checked the southern townships.

It does. Four census blocks in [Auglaize Township](../corpus/place/auglaize-township.yml), 99
people and 43 housing units, sit in HUC `05060001` — the Upper Scioto subbasin of the **Ohio
Region**. The other 3,548 blocks and 102,107 people are in the Great Lakes Region. The county
straddles a continental divide and 0.1 per cent of it is on the far side.

**The method is block composition, again.** Intersecting the county polygon with the subbasin
layer returns **four** subbasins — Auglaize, Blanchard, Upper Scioto and St. Marys — and
classifying all 3,552 block internal points returns **three**: St. Marys touches the county
boundary and contains no block of it. That is the same over-count `esriSpatialRelIntersects`
produced against the county-subdivision layer and is recorded here so the next reader does not
have to learn it twice. All 3,552 blocks land in exactly one subbasin and their populations sum
to 102,206, which is the county's own enumeration.

**It also verifies a chain of edges this corpus had only inferred.** The hierarchy is
containment, so a stream's unit says what its water reaches:

    Hog Creek          HUC12 041000070301 Upper Hog Creek     inside
    Little Hog Creek   HUC12 041000070303 Little Hog Creek    inside
                       HUC10 0410000703   Upper Ottawa River  inside
                       HUC8  04100007     Auglaize            inside
                       HUC4  0410         Western Lake Erie

**What it cannot support.** The WBD gives drainage areas, not the flow network. It says a
stream's water reaches a basin's outlet; it does not say which stream it joins first, and it
names no confluence. Directness still rests on GNIS coordinate identity, and the NHD flowline
half of [the `nhd` connector](../../crates/nhd/README.md) is still unwritten and still the only
thing that would give `comid` and downstream `comid`.
