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
used-by:
  - ../corpus/division/ohio-congressional-district-4-2020.yml
  - ../corpus/division/ohio-house-district-4-2020.yml
  - ../corpus/division/ohio-senate-district-12-2020.yml
  - ../corpus/jurisdiction/allen-east-local-school-district.yml
  - ../corpus/jurisdiction/bath-local-school-district.yml
  - ../corpus/jurisdiction/bluffton-exempted-village-school-district.yml
  - ../corpus/jurisdiction/columbus-grove-local-school-district.yml
  - ../corpus/jurisdiction/delphos-city-school-district.yml
  - ../corpus/jurisdiction/elida-local-school-district.yml
  - ../corpus/jurisdiction/lima-city-school-district.yml
  - ../corpus/jurisdiction/pandora-gilboa-local-school-district.yml
  - ../corpus/jurisdiction/perry-local-school-district.yml
  - ../corpus/jurisdiction/shawnee-local-school-district.yml
  - ../corpus/jurisdiction/spencerville-local-school-district.yml
  - ../corpus/jurisdiction/waynesfield-goshen-local-school-district.yml
  - ../corpus/measure/allen-county-voting-districts-2020.yml
  - ../corpus/place/gomer.yml
  - ../corpus/place/westminster.yml
  - ../corpus/question/allen-county-current-congressional-district.yml
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
among **88 voting districts** and 12 unified school districts.

**Seven of those twelve cross a county line, and the crossings run both ways.** The first reading
of this file recorded only that three districts are centred in neighbouring counties and reach in
by 2, 7 and 44 blocks. That was the district-level tally and it understated the case. Read block
by block: Delphos City is 57 per cent Allen and 43 per cent Van Wert; Spencerville Local spans
Allen, Auglaize and Van Wert; Bluffton Exempted Village reaches out into Hancock and Shawnee
Local into Auglaize; and Columbus Grove, Pandora-Gilboa and Waynesfield-Goshen reach in from
Putnam, Putnam and Auglaize. Only five of the twelve lie wholly inside Allen County. The school
map of this county is not organised by the county boundary in either direction.

**Vintage is the trap.** These are the districts as of the 2020 census. Ohio redistricted after
it, so the congressional and legislative assignments here describe the map that was in force
through the 2022 election and not the one in force now — see
[which congressional district Allen County is in now](../corpus/question/allen-county-current-congressional-district.yml).
There is no BAF for a later year; `baf2022`, `baf2024`, `baf2025` and `baf2026` all return 404.

**What else it holds that nobody has looked at.** The SDUNI and INCPLACE_CDP files are now read
out block by block, and every school district touching the county is a corpus node. What remains
is the VTD file — all 88 voting districts, whose codes (`002AFJ` and the like) carry no names and
so cannot be turned into nodes anyone could read without the Secretary of State's precinct list.
