---
name: Quarterly Census of Employment and Wages (BLS)
description: >-
  The Bureau of Labor Statistics' count of employment and wages covered by unemployment insurance,
  by county and industry. The corpus uses it as a second witness on recent employment and for what
  County Business Patterns leaves out — government.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://data.bls.gov/cew/data/api/2023/a/area/39003.csv
    description: >-
      Allen County annual averages, all ownerships and industries, 2023. The same path serves
      2014 onward; earlier years return 404, so this source cannot reach the decline itself.
---

**It corroborates the recent figures and cannot reach the old ones.** Its annual county files are
served from 2014, so it sees the last decade and nothing of the fall this corpus went looking for.
[verified] Where it overlaps
[County Business Patterns](census-county-business-patterns.md) the two agree closely: 8,661
manufacturing employees in 2023 against 8,573 for 2022.

**It counts what County Business Patterns does not.** Its 2023 total for all ownerships is 49,577
employees across 2,564 establishments, against County Business Patterns' 42,814 private employees
for 2022. The gap is roughly government, which QCEW covers and CBP omits. [inference] The corpus has
not decomposed it and does not assert the whole difference is government.

**One figure recorded here and not used elsewhere.** Average annual pay across all Allen County
employment in 2023 was $55,899, and in manufacturing $84,155. [verified] Manufacturing pays about
half again the county average, which is the ordinary reason a county mourns manufacturing jobs
specifically rather than jobs in general — but this corpus holds one year of it and draws no trend.
