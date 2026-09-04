---
name: 2020 Census Demographic and Housing Characteristics File
description: >-
  The 2020 census tabulated past the seven tables of the redistricting file — occupancy, tenure,
  vacancy status, household size, and every one of them crossed by the race of the householder,
  down to the block. It is the file the corpus named as the one it had not fetched when it asked
  why 3,628 housing units in this county were empty.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 1825
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/decennial/2020/data/demographic-and-housing-characteristics-file/Ohio/oh2020.dhc.zip
    description: >-
      Ohio's copy: 424,372,600 bytes compressed and 3.72 GB open, one geographic header and
      forty-four pipe-delimited data segments. Housing tables H1 through H12 are all in segment 1,
      which is 209.8 MB and 390,444 records. A record is linked to its geography by `LOGRECNO`.
  - kind: url
    value: https://www2.census.gov/programs-surveys/decennial/2020/technical-documentation/complete-tech-docs/demographic-and-housing-characteristics-file-and-demographic-profile/2020-dhc-table-matrix.xlsx
    description: >-
      The table matrix, which is the only thing that turns a segment into columns: one row per cell,
      in file order, giving the table, the reference name and the segment. Without it segment 1 is
      234 unlabelled integers.
  - kind: url
    value: https://www2.census.gov/programs-surveys/decennial/2020/technical-documentation/complete-tech-docs/demographic-and-housing-characteristics-file-and-demographic-profile/geoheader-2020-dhc-state.xlsx
    description: >-
      The geographic header layout, 97 fields, pipe-delimited. `SUMLEV` and `GEOCOMP` together
      decide what a record is: 050 with component 00 is a county, 01 its urban part, 43 its rural
      part, 060 a county subdivision, 140 a tract, 160 a place, 100 a block.
  - kind: url
    value: https://www2.census.gov/programs-surveys/decennial/2020/technical-documentation/complete-tech-docs/demographic-and-housing-characteristics-file-and-demographic-profile/2020census-demographic-and-housing-characteristics-file-and-demographic-profile-techdoc.pdf
    description: >-
      The technical documentation, and the place the Bureau writes down which figures in this file
      are enumerated and which are protected. Its definitions appendix also gives the formulas for
      the homeowner and rental vacancy rates.
  - kind: url
    value: https://api.census.gov/data/2020/dec/dhc
    description: >-
      The API for the same tables, which answers a keyless request with an HTTP 302 to a
      registration page. Not used; the bulk files need no key.
used-by:
  - ../corpus/measure/allen-county-vacancy-status-2010-2020.yml
  - ../corpus/measure/allen-county-tenure-2010-2020.yml
---

**The Bureau says in writing which numbers in this file were counted and which were computed.**
"The total population for each state is held invariant—used exactly as enumerated and with no noise
added. Similarly, the total number of housing units in each census block and the number and major
type of each occupied group quarters facility in each census block are also held invariant."
[verified] — the technical documentation, chapter 4. Everything else in the file, occupancy status
included, is the output of the TopDown algorithm; see
[the total is enumerated and the split is not](../decisions/the-total-is-enumerated-and-the-split-is-not.yml).

**This file and the redistricting file are not two readings of the same question.** Both publish
occupancy status at block level. Across all 3,552 blocks of Allen County they differ on nothing —
not one block, not on units, occupied or vacant — although their segments carry file dates twenty
months apart, 26 July 2021 and 3 March 2023. [verified] — both archives, compared here on
`GEOCODE`. A reader who treats the agreement as corroboration has counted one answer twice.

**Segment 1 holds the housing tables and nothing announces where a table starts.** H1 housing units
is data field 1, H3 occupancy status is field 6, H4 tenure field 9, H5 vacancy status field 101 and
H10 tenure by race of householder field 143, of 234. The order in the matrix is not the order of
the table numbers, and the documentation says so in its own user notes. [verified] — the table
matrix and chapter 5.

**Its vacancy rates are narrower than its vacancy count, and the definitions appendix says how.**
The homeowner vacancy rate divides *for sale only* by owner-occupied plus for-sale plus
sold-not-occupied; the rental rate divides *for rent* by renter-occupied plus for-rent plus
rented-not-occupied. Seasonal units, units for migrant workers and the residual class the file
calls *Other vacant* appear in neither rate, on either side of the line. [verified] — the same
appendix, B-21.

**`Other vacant` is defined by exclusion and the Bureau gives two examples, not a list.** "If a
vacant unit does not fall into any of the categories specified above, it is classified as 'Other
vacant.' For example, this category includes units held for occupancy by a caretaker or janitor and
units held for personal reasons of the owner." [verified] — the same appendix. It is the file's
remainder term, and in this county it is the largest single kind of empty house.

**What it does not carry.** No value, no rent, no year built, no plumbing, no heating fuel, no
income. Every dollar figure and every condition of a house in this corpus comes from the American
Community Survey or from an administrative file, and none of it is in here.
