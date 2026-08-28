---
name: Census TIGERweb REST service, 2020 vintage
description: >-
  The Census Bureau's boundary geometry as a keyless ArcGIS REST service. This is the first
  source in this corpus that can answer "what is inside what" rather than "where is it, roughly".
type: api
obtained: true
retrieved: 2026-08-28
ttl_days: 1825
location:
  - kind: url
    value: https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb/tigerWMS_Census2020/MapServer
    description: 2020 vintage — layers 6 Census Tracts, 10 Census Blocks, 20 County Subdivisions, 26 Incorporated Places, 28 Census Designated Places, 82 Counties
used-by:
  - ../corpus/division/census-tract-39003010300.yml
  - ../corpus/natural-feature/auglaize-river.yml
  - ../corpus/place/american-township.yml
  - ../corpus/place/fort-shawnee.yml
  - ../corpus/question/where-the-auglaize-rises.yml
  - ../corpus/site/lima-army-tank-plant.yml
---

Three retrievals in this corpus — the 2020 Gazetteer, the Block Assignment Files and GNIS —
have each said carefully that they carry no boundaries. This one does. A point and a layer go
in, the feature containing that point comes back, and four claims that had been open across
five phases closed against it.

**The layer numbers are not stable across vintages, and a wrong one does not error.** The
`tigerWMS_Current` and `tigerWMS_Census2020` services publish different layer orders. Counties
is 82 in both, which is how the mistake survives: a query built against the current service's
numbering returned Allen County correctly from the 2020 service and looked right, while the
same run's county-subdivision and place queries returned *no features* — indistinguishable from
"no subdivision contains this point" — and its census-tract query returned a **Block Group**
with a `NAME` of "Block Group 3". Nothing in any response says the layer was not the one asked
for. Introspect `MapServer?f=json` for the exact service being queried and never carry layer
numbers between them.

**`esriSpatialRelIntersects` includes touching, which is not covering.** Intersecting census
tract 39003010300 with the county-subdivision layer returns seven subdivisions, one of them
`3913775206` — a Sugar Creek township in **Putnam** County. A census tract cannot cross a county
line, so that hit is a shared boundary line and no shared area at all. Four of the other six are
the same kind of hit. Read as an answer, "seven townships" is wrong by five.

**The exact method is block composition.** Blocks nest inside both tracts and county
subdivisions, so the areal breakdown of any tract is: fetch its blocks with `INTPTLAT`/`INTPTLON`
and `AREALAND`, classify each internal point into a subdivision polygon, and sum. For tract
39003010300 that is 107 blocks, every one landing in exactly one subdivision, and their land
areas summing to `74,841,137` — the tract's own `AREALAND`, exactly. That the parts sum to the
whole is the check that the method worked.

**It disagrees with the Gazetteer about the county's size.** TIGERweb gives Allen County
`AREALAND` `1042587394` and `AREAWATER` `11152063`; the 2020 Gazetteer file gives `1042587389`
and `11152061` for the same county and the same vintage. Five square metres of land and two of
water, between two products of one bureau. Both round to 402.545 square miles and nothing in
this corpus changes, which is the point: that difference is the floor on how precisely any area
here can be believed, and it is not zero.
