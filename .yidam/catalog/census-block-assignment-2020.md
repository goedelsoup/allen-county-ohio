---
name: 2020 Census Block Assignment Files, Ohio
description: >-
  The Census Bureau's block-level assignment of every 2020 census block to the congressional,
  state legislative, school and voting districts covering it.
type: dataset
obtained: true
retrieved: 2026-08-28
ttl_days: 3650
location:
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/baf2020/BlockAssign_ST39_OH.zip
    description: Ohio — CD, SLDL, SLDU, SDUNI, VTD and INCPLACE_CDP assignment files
used-by: []
---

Pipe-delimited files, one row per census block, mapping a 15-digit block GEOID to a district
code. The GEOID's first five digits are the state and county FIPS, so filtering on `39003`
yields every block in Allen County and, through it, every district that covers any part of the
county. That is the relationship these files supply and the gazetteer does not: a gazetteer
gives a district's geometry, never the counties inside it.

**Why this source and not the Ohio Secretary of State.** The SoS is the authority for precinct
boundaries and election returns and blocks automated clients, so it could not be retrieved. The
Census Bureau publishes the same voting-district geography — collected from the states through
the Redistricting Data Program — as a keyless static file. This is a substitution of publisher,
not of grain: the districts are the states' own, tabulated federally.

It is nonetheless **not** a substitute for the SoS on the thing the SoS uniquely holds. These
files carry district *codes* and no district *names*, and they carry no election results at
all. A voting district here is `002AFJ`, which is not something anyone can read.

**What Allen County looks like in it.** 3,552 blocks. All 3,552 fall in congressional district
04, all 3,552 in state senate district 012, and all 3,552 in state house district 004 — so no
federal or state legislative boundary splits this county in the 2020 geography. They divide
among **88 voting districts** and 12 unified school districts, three of which are centered in
neighbouring counties and reach in by 2, 7 and 44 blocks respectively.

**Vintage is the trap.** These are the districts as of the 2020 census. Ohio redistricted after
it, so the congressional and legislative assignments here describe the map that was in force
through the 2022 election and not the one in force now — see
[which congressional district Allen County is in now](../corpus/question/allen-county-current-congressional-district.yml).
There is no BAF for a later year; `baf2022`, `baf2024`, `baf2025` and `baf2026` all return 404.

**What else it holds that nobody has looked at.** All 88 voting districts and all 12 school
districts, block by block, plus the INCPLACE_CDP file that assigns blocks to places. Only the
district-level tallies were read.
