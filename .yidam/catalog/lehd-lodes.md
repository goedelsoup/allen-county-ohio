---
name: LEHD Origin-Destination Employment Statistics (LODES8)
description: >-
  Where every job in the country is, and where the person who holds it lives, block by block, from
  the Census Bureau's Longitudinal Employer-Household Dynamics programme. It is the only source in
  this corpus that says anything about the journey to work.
type: dataset
obtained: true
retrieved: 2026-08-31
ttl_days: 365
location:
  - kind: url
    value: https://lehd.ces.census.gov/data/lodes/LODES8/oh/od/oh_od_main_JT00_2022.csv.gz
    description: >-
      Ohio origin-destination pairs where both ends are in Ohio, all job types, 2022. 27 MB
      gzipped; 61,765 pairs have a block in Allen County at one end or the other. Filter on the
      first five characters of `w_geocode` and `h_geocode`, which are the county FIPS.
  - kind: url
    value: https://lehd.ces.census.gov/data/lodes/LODES8/oh/od/oh_od_aux_JT00_2022.csv.gz
    description: >-
      The same for workers living outside Ohio, 1.6 MB. Omitting it undercounts in-commuters, and
      1,394 Allen County pairs are in it.
  - kind: url
    value: https://lehd.ces.census.gov/data/lodes/LODES8/LODESTechDoc8.1.pdf
    description: >-
      The technical document. `LODESTechDoc8.2.pdf` returns 404 and serves a 32 KB error page that
      `curl` will happily write to disk; 8.1 is the one that exists.
used-by:
  - ../corpus/measure/allen-county-commuting-2022.yml
---

**It is disclosure-protected and says so on its first page:**

> The U.S. Census Bureau reviewed this data product for unauthorized disclosure of confidential
> information and approved the disclosure avoidance practices applied to this release.

A block-to-block table of where people work cannot be published as collected, so the file this
corpus reads is not the file the Bureau holds. Sums over thousands of pairs survive that; single
cells do not. See
[the flow is real and the cell is not](../decisions/the-flow-is-real-and-the-cell-is-not.yml).

**Its county total checks out against a source that is not protected.** 48,730 jobs located in
Allen County in 2022, against 49,690 covered jobs in the
[Quarterly Census of Employment and Wages](bls-qcew.md) for 2024 — different years, different
programmes, different definitions of a job, 960 apart. [verified]

**Its most interesting row is one this corpus will not read.** Franklin County — Columbus, a
hundred miles away — is the second-largest recorded destination for Allen County residents, above
two counties that share a border with this one. The 1,723 are spread across many Columbus tracts
with a maximum of 79 in any one, so it is not a headquarters address standing in for worksites. It
is what a protected file looks like at small grain, and the rank order at the top of a list is the
place that shows it.

**What is in it and unread.** The residence and workplace area characteristic files, `rac` and
`wac`, which give each block's jobs by industry, age and earnings without the origin-destination
pairing and are therefore cheaper to use for anything that does not need a flow. The `JT01` through
`JT05` job types — primary jobs, private, private primary — where this corpus has read only `JT00`,
all jobs. And every year from 2002 to 2022, of which this corpus has read one.
