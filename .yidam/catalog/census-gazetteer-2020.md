---
name: 2020 Census Gazetteer Files
description: >-
  The Census Bureau's published gazetteer for the 2020 geographies — land and water area,
  internal-point coordinates, GEOID and ANSI codes for counties, county subdivisions,
  incorporated places and census tracts.
type: dataset
obtained: true
retrieved: 2026-08-28
ttl_days: 3650
location:
  - kind: url_template
    value: https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_gaz_{level}_{state}.txt
    description: per-state tab-delimited files; county subdivisions and places used here
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_Gaz_counties_national.zip
    description: national county file, zipped
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_Gaz_tracts_national.zip
    description: national census tract file, zipped
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_Gaz_116CDs_national.zip
    description: congressional districts, 116th Congress
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_Gaz_sldu_national.zip
    description: state legislative districts, upper chamber
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_Gaz_sldl_national.zip
    description: state legislative districts, lower chamber
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_Gaz_unsd_national.zip
    description: unified school districts
  - kind: url_template
    value: https://www2.census.gov/geo/docs/maps-data/data/gazetteer/{year}_Gazetteer/{year}_gaz_{level}_39.txt
    description: >-
      The same files for every vintage from 2012 to 2024 — there is no 2010 or 2011 Ohio place
      file. Retrieved 2026-08-31 for `place`, `cousubs` and `counties`, which is what turns a
      single snapshot into a thirteen-year series of the ground each entity is tabulated at.
used-by:
  - ../corpus/division/census-tract-39003010300.yml
  - ../corpus/division/ohio-congressional-district-4-2020.yml
  - ../corpus/division/ohio-house-district-4-2020.yml
  - ../corpus/division/ohio-senate-district-12-2020.yml
  - ../corpus/division/voting-district-sugar-creek-2020.yml
  - ../corpus/jurisdiction/allen-east-local-school-district.yml
  - ../corpus/jurisdiction/amanda-township.yml
  - ../corpus/jurisdiction/american-township.yml
  - ../corpus/jurisdiction/auglaize-township.yml
  - ../corpus/jurisdiction/bath-local-school-district.yml
  - ../corpus/jurisdiction/bath-township.yml
  - ../corpus/jurisdiction/bluffton-exempted-village-school-district.yml
  - ../corpus/jurisdiction/columbus-grove-local-school-district.yml
  - ../corpus/jurisdiction/delphos-city-school-district.yml
  - ../corpus/jurisdiction/elida-local-school-district.yml
  - ../corpus/jurisdiction/jackson-township.yml
  - ../corpus/jurisdiction/lima-city-school-district.yml
  - ../corpus/jurisdiction/marion-township.yml
  - ../corpus/jurisdiction/monroe-township.yml
  - ../corpus/jurisdiction/pandora-gilboa-local-school-district.yml
  - ../corpus/jurisdiction/perry-local-school-district.yml
  - ../corpus/jurisdiction/perry-township.yml
  - ../corpus/jurisdiction/richland-township.yml
  - ../corpus/jurisdiction/shawnee-local-school-district.yml
  - ../corpus/jurisdiction/shawnee-township.yml
  - ../corpus/jurisdiction/spencer-township.yml
  - ../corpus/jurisdiction/spencerville-local-school-district.yml
  - ../corpus/jurisdiction/sugar-creek-township.yml
  - ../corpus/jurisdiction/waynesfield-goshen-local-school-district.yml
  - ../corpus/measure/allen-county-annexations-1990-2024.yml
  - ../corpus/measure/allen-county-land-area-2000-2024.yml
  - ../corpus/measure/allen-county-land-area-2020.yml
  - ../corpus/measure/allen-county-survey-sections.yml
  - ../corpus/measure/lima-land-area-2020.yml
  - ../corpus/measure/ohio-house-district-4-land-area-2020.yml
  - ../corpus/natural-feature/auglaize-river.yml
  - ../corpus/place/allen-county.yml
  - ../corpus/place/amanda-township.yml
  - ../corpus/place/american-township.yml
  - ../corpus/place/auglaize-township.yml
  - ../corpus/place/bath-township.yml
  - ../corpus/place/beaverdam.yml
  - ../corpus/place/bluffton.yml
  - ../corpus/place/cairo.yml
  - ../corpus/place/delphos.yml
  - ../corpus/place/elida.yml
  - ../corpus/place/fort-shawnee.yml
  - ../corpus/place/gomer.yml
  - ../corpus/place/harrod.yml
  - ../corpus/place/jackson-township.yml
  - ../corpus/place/lafayette.yml
  - ../corpus/place/lima.yml
  - ../corpus/place/marion-township.yml
  - ../corpus/place/monroe-township.yml
  - ../corpus/place/perry-township.yml
  - ../corpus/place/richland-township.yml
  - ../corpus/place/shawnee-township.yml
  - ../corpus/place/spencer-township.yml
  - ../corpus/place/spencerville.yml
  - ../corpus/place/sugar-creek-township.yml
  - ../corpus/place/westminster.yml
  - ../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml
---

Tab-delimited files published by the Census Bureau's Geography Division, one row per
geographic unit, giving `GEOID`, `ANSICODE`, `NAME`, land and water area in both square
metres and square miles, and an internal point (`INTPTLAT`, `INTPTLONG`) in decimal degrees.
The internal point is guaranteed to fall inside the polygon, which is not true of a
mathematical centroid for a concave shape — the distinction matters for any node claiming a
coordinate lies within the thing it describes.

**Retrieved without a key.** This matters and is recorded because the obvious route does
not work: the Census data API at `api.census.gov` now rejects every keyless request with
*"A valid key must be included with each data API request."* The gazetteer files are static
downloads under `www2.census.gov` and require nothing. Anything this corpus needs at
county, county-subdivision, place or tract grain should come from here rather than from the
API, unless and until somebody registers a key.

**What else this holds that nobody has looked at.** The county-subdivision and place files are
now read out: all 13 subdivisions, all 9 municipalities and all 3 census designated places in the
county are corpus nodes. Two of those CDPs were missed on the first pass — Gomer and Westminster
— because the corpus built its list of places from the sub-county *population* file, which
tabulates governmental units only. A place with no government is in this file and not in that
one, and the gap took a school-district retrieval to find. What remains here is the tract file,
which carries 35 rows for this county of which one has been turned into a node. The water-area
columns have not been examined at all, and they are the natural anchor for any future claim about
the county's surface water.

**Districts were added to this entry after the electoral-grain phase.** The same gazetteer
program publishes congressional, state legislative and school district files in the same
format, and four were retrieved. They give a district's area and internal point and **not** the
counties inside it; the county relationship comes from
[the Block Assignment Files](census-block-assignment-2020.md) instead.

One comparison out of those files is worth recording where somebody will find it. Ohio's State
House District 4 reads `ALAND 1042587389`, `AWATER 11152061`, `ALAND_SQMI 402.545`,
`INTPTLAT 40.771627`, `INTPTLONG -84.106103` — every field byte-identical to Allen County's own
record in the county file. The district is coterminous with the county.

**What it cannot support.** The gazetteer publishes geography, not population. Population in
this corpus comes from [Census Population Estimates](census-popest-2024.md) for any date after
2020, and from [TIGERweb](tigerweb-census2020.md) block counts for the 2020 enumeration itself.
