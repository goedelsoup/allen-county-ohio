---
name: Preliminary Estimates of the Intercensal Population of Counties, 1970–1979
description: >-
  A 1982 Census Bureau publication of county population for the 1970s, whose first column is
  the 1970 decennial census count.
type: dataset
obtained: true
retrieved: 2026-08-28
ttl_days: 3650
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/tables/1900-1980/counties/totals/e7079co.txt
    description: fixed-width text, all US counties
used-by: []
---

Issued April 1982 by the Population Estimates and Population Distribution Branches. The
methodology is described in Current Population Reports, Series P25-957.

**Only the first column is taken.** The file is laid out in two blocks. The first is headed
`Census 1970` followed by estimates for 1971–1974; the second is headed `Estimate` for
1975–1979. Allen County, Ohio (FIPS 39003) appears in both — 111,144 in the first block and
110,800 in the second. **The 111,144 is the census count and the 110,800 is a 1975 estimate**,
and reading the second row as a 1970 figure would be an easy and invisible error, because both
rows begin `39003 Allen Co.` and only the block header distinguishes them. It is written down
here because the file gives no other warning.

There are also Allen Counties in Indiana (18003), Kansas (20001) and Kentucky (21003) in this
file. Match on the FIPS code, never on the name.

**Estimates in this file are rounded** — counties to hundreds, states to thousands — and the
publication says unrounded figures are not available. The 1970 census count is not rounded.

**What it does not hold.** Counties only, no places, so it cannot supply a 1970 figure for
Lima. It also stops at 1970 going backwards: nothing in this corpus reaches the 1900–1960
decades, which is the gap
[the county's pre-1970 population series](../corpus/question/pre-1970-population-series.yml)
names.
