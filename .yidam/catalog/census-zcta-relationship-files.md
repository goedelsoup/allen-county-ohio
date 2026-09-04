---
name: 2020 ZCTA relationship files (U.S. Census Bureau)
description: >-
  Which census blocks and which counties each ZIP Code Tabulation Area overlaps, and how much land
  is in each piece. Joined to block population it turns a mailing ZIP into a county with a weight,
  which is the only way this corpus can read a file that records where a letter goes rather than
  where a person is.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 3650
location:
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/rel2020/zcta520/tab20_zcta520_county20_natl.txt
    description: >-
      ZCTA against county, 433 KB. It answers which counties a ZIP touches and how its land divides,
      and it cannot answer how its people divide.
  - kind: url
    value: https://www2.census.gov/geo/docs/maps-data/data/rel2020/zcta520/tab20_zcta520_tabblock20_natl.txt
    description: >-
      ZCTA against census block, 1.0 GB, every tabulation block in the country. Blocks nest inside
      counties and carry a published population, so this file is what makes a population weight
      possible. Streamed and filtered to twenty ZCTAs rather than stored.
used-by:
  - ../corpus/measure/allen-county-zip-codes-2020.yml
---

**It publishes land and not people.** Every relationship file carries `AREALAND_PART` — the area of
the overlap — and no population column anywhere. A reader who wants to know what share of a ZIP's
*residents* are in a county has to supply that from somewhere else. [verified] — the file headers.

**The block file supplies it, and the join closes exactly.** Joining ZCTA-to-block against the 2020
redistricting file's block populations gives 102,206 people in the Allen County parts of twenty
ZCTAs, which is the county's census population to the person. [verified] — this file against
[the redistricting file](census-2020-redistricting-file.md), computed here. A relationship file
that partitions the country and a population file that partitions the state agree on a county
neither of them names.

**The 2020 vintage's identifiers are five digits and its geoids are not the ACS's.** A ZCTA is
`45801`, a county `39003`, a block the fifteen-digit `390030101001002`; the summary file this corpus
reads for the survey writes the same county as `0500000US39003`. Nothing here carries the survey's
prefixes. [verified] — the same files.

**A ZCTA is not a ZIP code and the Postal Service does not publish one.** ZCTAs are built from
blocks to approximate the delivery areas the Postal Service uses, which are routes rather than
areas and are not published as geography at all. A ZIP that is a single building or a post-office
box has no ZCTA. [verified] — the Bureau's own definition, and the twenty codes here are all
area codes with tabulation areas. Every weight in this corpus is therefore a weight on the
approximation and not on the thing approximated. [inference]
