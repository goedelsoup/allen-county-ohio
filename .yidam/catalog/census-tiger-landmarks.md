---
name: TIGER/Line Landmark Files, Ohio (2024)
description: >-
  The Census Bureau's own list of the named things on its maps — prisons, campuses, parks,
  cemeteries, airports, shopping centres — as polygons and points with land area and internal
  point. It is what turned a census block holding 1,360 people into a named institution, because
  the block and the landmark are the same polygon and the file proves it.
type: dataset
obtained: true
retrieved: 2026-08-31
location:
  - kind: url
    value: https://www2.census.gov/geo/tiger/TIGER2024/AREALM/tl_2024_39_arealm.zip
    description: >-
      Area landmarks for Ohio, 1.0 MB, 3,460 features. Fields STATEFP, ANSICODE, AREAID, FULLNAME,
      MTFCC, ALAND, AWATER, INTPTLAT, INTPTLON, PARTFLG.
  - kind: url
    value: https://www2.census.gov/geo/tiger/TIGER2024/POINTLM/tl_2024_39_pointlm.zip
    description: >-
      Point landmarks for Ohio, 1.0 MB, 36,985 features — mostly cemeteries, hospitals, airports
      and shopping centres. Fields STATEFP, ANSICODE, POINTID, FULLNAME, MTFCC.
used-by:
  - ../corpus/measure/allen-county-group-quarters-2020.yml
  - ../corpus/measure/allen-county-health-2023.yml
  - ../corpus/measure/allen-county-higher-education-2023.yml
  - ../corpus/organization/james-a-rhodes-state-college.yml
  - ../corpus/organization/ohio-state-beauty-academy.yml
  - ../corpus/organization/ohio-state-university-at-lima.yml
  - ../corpus/organization/university-of-northwestern-ohio.yml
  - ../corpus/place/bath-township.yml
  - ../corpus/question/who-lives-in-the-county-without-housing.yml
  - ../corpus/site/allen-correctional-institution.yml
  - ../corpus/site/allen-county-courthouse.yml
  - ../corpus/site/allen-county-justice-center.yml
  - ../corpus/site/hay-road-bridge.yml
  - ../corpus/site/lima-state-hospital.yml
  - ../corpus/site/oakwood-correctional-facility.yml
---

**What it is for here.** Every other geographic file this corpus holds answers *where is this
boundary*. This one answers *what is that*. It is the Census Bureau's inventory of named
non-governmental features, maintained because its enumerators and its maps need them, and it
carries the two fields that make an exact join to a census block possible: `ALAND` in square
metres and `INTPTLAT`/`INTPTLON` to seven decimal places.

**What it holds in Allen County.** Four confinement landmarks, of which the county's own address
file names a different three:

    K1237  Allen Correctional Instn                  364,210 m²   +40.7751673 -084.0996846
    K1237  Oakwood Corr Faclty                     1,383,460 m²   +40.7812913 -084.1012253
    K1237  Western Ohio Regional Treatment and         74,013 m²   +40.7894993 -084.1022793
           Rehabilitation Ctr
    K1236  Allen County Justice Ctr                     5,598 m²   +40.7437104 -084.1054470

K1237 is a state or federal correctional institution and K1236 is a county or municipal
governmental facility, so the file distinguishes the state's prisons from the county's jail
without being asked. **Lima Correctional Institution is not in it**, which agrees with
[the department's own list](odrc-facilities.md) of facilities closed in 2004 and disagrees with
the county address file, which still carries it.

It also carries the county's colleges as K2540 — *Bluffton Univ* at 435,444 m² and *Univ of
Northwestern Ohio* at 659,172 m² — and the corpus has a node for one of them.

**The join, and its limit.** Two Allen County blocks match a landmark on both land area and
internal point exactly; those are identities. Two more fall *inside* the University of
Northwestern Ohio's polygon without matching it, which is a containment and is reported as one.
Three group-quarters blocks fall inside nothing, and the nearest landmark to each is 300 to 1,400
metres away, which is not evidence of anything. The rule is in
[a block can be a fence](../decisions/a-block-can-be-a-fence.yml).

**Cautions.** `FULLNAME` is abbreviated on the Bureau's own conventions — "Instn", "Corr",
"Faclty", "Cmtry" — so a search for "Institution" or "Cemetery" finds nothing. `ANSICODE` is null
for every feature used here. And the file is a 2024 vintage joined here to 2020 blocks: boundaries
that moved between the two would break the identity, and for these two they did not.

**What else is in it, unread.** Ohio's parks and state parks (K2180, K2184), its cemeteries, its
airports and heliports, and its shopping centres — including, in this county, the Lima Mall, Lima
Plaza, Westgate, the Allen County Fairgrounds, Lima Allen County Airport, St Rita's Medical Center
and Lima Memorial Hospital, none of which the corpus has a coordinate for from any other source.
