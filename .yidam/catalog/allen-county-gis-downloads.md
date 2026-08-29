---
name: Allen County GIS bulk downloads
description: >-
  The county's own parcel, address and political-subdivision layers, published weekly as
  shapefiles. The first local-authority source in this corpus, and the only one that names
  buildings.
type: dataset
obtained: true
retrieved: 2026-08-28
ttl_days: 365
location:
  - kind: url
    value: https://gis.allencountyohio.com/GIS/downloads.html
    description: Index page — 15 downloads. Only Political_Subdivisions.zip and Addresses_ft.zip were retrieved; see the access-terms decision.
used-by:
  - ../corpus/place/american-township.yml
  - ../corpus/place/shawnee-township.yml
  - ../corpus/site/allen-county-courthouse.yml
  - ../corpus/site/lima-army-tank-plant.yml
  - ../corpus/site/lima-refinery.yml
---

**Read [the access-terms decision](../decisions/auditor-parcels-access-terms.yml) before using
this.** The county publishes two files of parcel owner data. They were not retrieved, and the
reason is recorded there rather than here.

**It names buildings, which is what nothing else here does.** GNIS has no `Building` class and
the Census names no structures at all, so three of this corpus's five sites had no coordinate
until this file. `Addresses_ft.dbf` carries 50,710 address points, 4,491 of them with a
`LANDMARK` string naming a non-residential structure — "ALLEN COUNTY COURT HOUSE IN LIMA AT 301
N MAIN ST", "VALERO LIMA REFINERY AT 1150 S METCALF ST". It also carries a `MUNI` column, which
is the county's own answer to which municipality an address is in, and blank where the answer is
none.

**Coordinates are State Plane and have to be projected.** Everything is NAD 1983 State Plane
Ohio North, FIPS 3401, in **US survey feet** — a Lambert Conformal Conic with standard parallels
40°26′ and 41°42′, central meridian −82.5°, latitude of origin 39°40′, false easting 1,968,500
feet, and the foot defined as 1200/3937 metres rather than 0.3048. Getting the foot wrong misses
by about 2 feet per mile, which is under the width of a building and over the width of a lot
line.

The inverse was checked three ways before any number from it was written down: a point at the
false easting returns longitude exactly −82.5°; the projection origin round-trips to
(1968500, 0) exactly; and forward-then-inverse on the three landmarks used here returns to
0.0000 m.

**Two independent authorities agree, which is the real result.** The county's `MUNI` column and
the Census Bureau's [TIGERweb](tigerweb-census2020.md) boundaries were asked the same question
about the same three points and gave the same answer three times: the courthouse in Lima, the
refinery and the tank plant in no municipality at all. These are separate agencies, separate
geometry, separate methods of assignment. Where a corpus has been getting locations wrong for
seven phases, that is worth more than either source alone.

**Its political subdivisions are 27 polygons and 21 subdivisions.** `Political_Subdivisions.dbf`
holds one name column and repeats a township once per polygon, so Bath, Lima, Marion, Perry,
Richland and Shawnee each appear twice — a township split by a municipality is not one shape.
The distinct set is 12 townships and **9 municipalities**: Beaverdam, Bluffton, Cairo, Delphos,
Elida, Harrod, Lafayette, Lima, Spencerville. This corpus holds jurisdiction nodes for two of
the nine.

**What was not taken from it.** No geometry. The polygon and point `.shp` files were downloaded
and not parsed: the attribute tables answered every question asked, and a boundary this corpus
needed was already available from TIGERweb as a query rather than as a file to interpret.
