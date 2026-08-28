---
name: Census Intercensal Estimates, 2000–2010
description: >-
  The Census Bureau's intercensal series bridging the 2000 and 2010 censuses, at county and
  sub-county grain — carrying the 2000 estimates base and the 2010 census count as anchors.
type: dataset
obtained: true
retrieved: 2026-08-28
ttl_days: 3650
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/datasets/2000-2010/intercensal/county/co-est00int-tot.csv
    description: county totals
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/datasets/2000-2010/intercensal/cities/sub-est00int.csv
    description: sub-county totals — places and minor civil divisions
used-by:
  - ../corpus/measure/allen-county-population-2000.yml
  - ../corpus/measure/allen-county-population-2010.yml
  - ../corpus/measure/lima-population-2000.yml
  - ../corpus/measure/lima-population-2010.yml
  - ../corpus/period/deindustrialization.yml
  - ../corpus/place/fort-shawnee.yml
  - ../corpus/question/pre-1970-population-series.yml
  - ../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml
---

Intercensal estimates are revised *after* the closing census is taken, so the series is fitted
between two known endpoints rather than projected forward from one. That makes this file more
reliable than a postcensal vintage for the decade it covers, and it is why the 2000–2010
figures in this corpus come from here rather than from a contemporary release.

**The two anchors are what this corpus takes.** `ESTIMATESBASE2000` is the April 1, 2000
census count as revised; `CENSUS2010POP` is the April 1, 2010 count. The 96 estimate columns
between them are modeled and none is used here.

**A cross-check that held.** `CENSUS2010POP` for Allen County reads 106,331 in this file and
106,331 in the Vintage 2020 county file, which is a different release built for a different
purpose. Two independent files agreeing on a census count is weak evidence — they draw on the
same enumeration — but a disagreement would have been strong evidence of a transcription
error, and there was none.

**A boundary change it records.** The sub-county file lists `Fort Shawnee village` in Allen
County with 3,866 at the 2000 base and 3,726 at the 2010 census. The 2020 Gazetteer lists
`Fort Shawnee CDP`, and the Vintage 2024 sub-county file does not list it at all. See
[what happened to the village of Fort Shawnee](../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml).

**What else it holds that nobody has looked at.** Ten annual estimate columns per row, plus
the whole of the United States. Only Allen County's rows were read.
