---
name: USGS Geographic Names Information System — Domestic Names, Ohio
description: >-
  The federal authority for geographic feature names, identifiers and coordinates. The Ohio
  file carries 17,921 features, 109 of them in Allen County — and 4 more in the other one.
type: dataset
obtained: true
retrieved: 2026-08-28
ttl_days: 3650
location:
  - kind: url
    value: https://prd-tnm.s3.amazonaws.com/StagedProducts/GeographicNames/DomesticNames/DomesticNames_OH_Text.zip
    description: Ohio domestic names, pipe-delimited — Text/DomesticNames_OH.txt
used-by:
  - ../corpus/natural-feature/auglaize-river.yml
  - ../corpus/natural-feature/great-black-swamp.yml
  - ../corpus/natural-feature/hog-creek.yml
  - ../corpus/natural-feature/little-hog-creek.yml
  - ../corpus/natural-feature/maumee-river.yml
  - ../corpus/natural-feature/ottawa-river.yml
  - ../corpus/question/where-the-auglaize-rises.yml
  - ../corpus/site/lima-army-tank-plant.yml
  - ../corpus/site/miami-and-erie-canal.yml
---

Twenty-one pipe-delimited columns, one row per named feature: `feature_id`, `feature_name`,
`feature_class`, `county_name`, a primary coordinate in both DMS and decimal, and a source
coordinate in both. This is the authority behind the `gnis_id` property on `place` and
`natural-feature`, and it is the only reachable source that gives Allen County's watercourses
an identifier at all.

**`county_name` alone will get you the wrong Allen County.** The file is an extract of features
*related to* Ohio, not features in it: 56 rows carry a `state_name` other than Ohio — 31
Indiana, 10 Pennsylvania, 7 Michigan, 4 West Virginia, 3 Kentucky. Four of them are in Allen
County, **Indiana**, whose seat is Fort Wayne, and both Allen Counties carry `county_numeric`
`003`. So the discriminator is `state_numeric`: `39` is Ohio and `18` is Indiana. Filtering
this file on `county_name == "Allen"` returns 113 rows, four of which are 90 miles away, and
two of those four — the St. Joseph and the St. Marys — are the rivers that *form* the Maumee,
so they look exactly like features this corpus should want.

**A stream is filed under the county of its mouth.** This is the trap and it is not a small
one. `county_name` is a single value, and for a linear feature it names where the feature
*ends*, not everywhere it runs. The Ottawa River runs through the middle of Lima and does not
appear in Allen County's 109 rows — it is filed under Putnam, where it discharges. Taking the
county's rows as "the county's named features" would omit the county's principal watercourse,
and would do it silently.

**The primary coordinate means different things by class.** For a `Stream` it is the mouth;
for a point feature such as `Military` it is the location. Nothing in this file is a centroid,
and no field says which reading applies — the class does. A pipeline that wrote `prim_lat_dec`
into a "location" property would place every river in the state at its outlet.

**`source_lat_dec` is `0.0` where there is no source, not empty.** 13,223 of the 17,921 Ohio
rows carry `0.0|0.0` in the source columns, which is every non-stream feature; no stream in
Ohio has a null source. The Miami and Erie Canal's row is one of the 13,223. Read as a
coordinate it puts a canal in the Gulf of Guinea.

**What it settles that nothing else here could.** Coordinate identity between records is
evidence of topology. Hog Creek's mouth, Little Hog Creek's mouth and the Ottawa River's source
are all `40.7708852, -83.9557761` — the same point to seven decimal places, in three separate
records written by one authority. That is how this corpus's first `flows-into` edges stopped
being asserted from general knowledge.

**It does not carry buildings.** There is no `Building` class in the Ohio file, and none of the
34 classes present is a substitute. The courthouse, the refinery and the locomotive works are
not in GNIS and will not be; four of the corpus's five `site` nodes remain without coordinates
after this retrieval, and the county auditor's parcel data is still the only proposed route.

**The 109 Ohio rows bound the county loosely.** Their coordinates span 40.647272–40.904493 and
-84.388008 to -83.879939: about 17.75 miles tall by 26.55 miles wide, a 471-square-mile box
around a 402.5-square-mile county. Every feature filed under the county is inside the county,
so the box contains it; the box is not the county, and nothing here is a boundary.

**Name collisions are real here.** Ohio has two Ottawa Rivers — one in Lucas County by Toledo,
one in Putnam draining Allen — and two Hog Creeks, in Allen and Champaign. Every lookup in this
file must be constrained by county or resolved to a `feature_id`, and the `feature_id` is what
this corpus records.
