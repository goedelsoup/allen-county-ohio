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
used-by:
  - ../corpus/division/census-tract-39003010300.yml
  - ../corpus/measure/allen-county-land-area-2020.yml
  - ../corpus/measure/lima-land-area-2020.yml
  - ../corpus/natural-feature/auglaize-river.yml
  - ../corpus/place/allen-county.yml
  - ../corpus/place/american-township.yml
  - ../corpus/place/bluffton.yml
  - ../corpus/place/delphos.yml
  - ../corpus/place/lima.yml
  - ../corpus/place/shawnee-township.yml
  - ../corpus/place/sugar-creek-township.yml
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

**What else this holds that nobody has looked at.** The files retrieved cover Ohio county
subdivisions, Ohio places, all US counties and all US census tracts. Only Allen County rows
were read. The tract file alone carries 35 rows for this county, of which one has been
turned into a node. The water-area columns have not been examined at all, and they are the
natural anchor for any future claim about the county's surface water.

**What it cannot support.** The gazetteer publishes geography, not population. Every
population figure in this corpus comes from
[Census Population Estimates](census-popest-2024.md) instead.
