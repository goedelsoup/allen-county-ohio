---
name: FRA Highway-Rail Grade Crossing Inventory
description: >-
  Every place a road, path or private drive meets a railroad in the United States, with the
  railroad that reports it, how many trains a day pass, how fast they run and whether any of them
  carries passengers. It is the only current source this corpus has that names the county's
  railroads as a set.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 180
location:
  - kind: url
    value: https://data.transportation.gov/resource/m2f8-22s6.json?$limit=2000&countycode=39003
    description: >-
      Socrata, keyless. 305 rows for Allen County — 201 open crossings and 104 closed. The county
      filter is `countycode`, which is the five-digit FIPS code and not the two-digit county code
      the field name suggests; there is no `cnty` column despite what the FRA's own documentation
      calls it.
used-by:
  - ../corpus/measure/allen-county-railroads-2026.yml
  - ../corpus/measure/allen-county-rail-crossing-deaths-1980-2005.yml
---

**Six railroads report open crossings in Allen County and two of them are Class I.** CSX and
Norfolk Southern; then three Class III short lines — the Chicago, Fort Wayne & Eastern, the R. J.
Corman Western Ohio Line and the Indiana & Ohio — and one switching company. [verified]

**Not one of the 201 open crossings sees a passenger train.** The `numberpassengertrainperday`
field is `0` at every one of them. [verified] The same field would have carried the ten fast
passenger trains a day that
[the 1906 history](miller-allen-county-1906.md) reports through Lima on the Chicago & Erie alone.

**The closed crossings are a record of who left.** All 24 of Conrail's and all 3 of Grand Trunk
Western's Allen County crossings are marked closed, and the closure revisions run from 1989 to
2025 with clusters in 1995–96, 2003 and 2019. A closed row is not deleted, which is what makes the
file a history as well as an inventory. [verified]

**It carries a route without naming one.** There is no line or subdivision field worth the name —
`branchname` is blank on 162 of the 201 open rows — so a line has to be reconstructed from the
towns its crossings sit in and the timetable station each names. That is enough to tell a
north-south road from an east-west one in this county and it is not enough to trace either beyond
the county line.

**The gate arms are now read, and a different file supplied the accidents.** Ninety-four of the
201 open crossings carry roadway gate arms, and 94 of the 163 open *public* ones do — 57.7 per cent.
[verified] — `countroadwaygatearms`, the same 305 rows. The twenty crossings where somebody died
between 1980 and 2005 were named not by the FRA's own incident file but by
[NHTSA's crash census](nhtsa-fars.md), whose `RAIL` field is this file's `crossingid`; nineteen of
the twenty join. See
[the crossings](../corpus/measure/allen-county-rail-crossing-deaths-1980-2005.yml).

**What is in it and still unread.** Advance signs, annual average daily road traffic, roadway
surface, and the FRA's own incident file on `crossingid` — which is the accident history of every
one of these crossings, is a separate dataset, and would carry the injuries and the property damage
that a fatality census by definition leaves out. The historical Form 71 file (`8uv2-y4is`) holds
superseded revisions of each row and would date the closures — and the gates — more precisely than
the revision year does, which is what a claim about whether gates came before or after a death
would need.
