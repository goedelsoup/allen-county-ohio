---
name: Census Building Permits Survey
description: >-
  The Census Bureau's monthly and annual count of residential building permits issued by every
  permit-issuing jurisdiction in the United States, by structure size, with the reported
  construction value. It is the only annual series this corpus holds that measures what the county
  *built* rather than what it had, and the only one published at both county and place grain for
  every year since 1990.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/econ/bps/County/co<yyyy>a.txt
    description: >-
      One annual file per year, all counties, comma-separated with a three-line header. Allen County
      is the row whose state FIPS is 39 and county FIPS 003. 1990 is the earliest year present;
      2025 is the latest. The `a` suffix is the annual file.
  - kind: url
    value: https://www2.census.gov/econ/bps/Place/Midwest%20Region/mw<yyyy>a.txt
    description: >-
      The same year at place grain, one file per census region. Allen County's permit-issuing places
      are its two cities, seven villages and twelve townships, plus an "Allen County Part" row for
      unincorporated ground not inside a reporting township.
used-by:
  - ../corpus/measure/allen-county-building-permits-1990-2025.yml
  - ../corpus/measure/allen-county-new-houses-by-place-1990-2025.yml
---

**The column layout is not stable across years and the place files are the worse of the two.** The
1990 county file separates fields with " , " and the 2024 one with ","; the place files gained
columns between 1990 and 2025, so the place-name field sits at a different index depending on the
year. A parser that hard-codes a column number reads the wrong figure and does not fail. The read
here locates the name field as the first field after index five containing a letter, and takes the
numeric fields relative to it. [verified] — both layouts inspected.

**Place names carry footnote markers that must be stripped before names are matched.** `American
township@4`, `Lima #`, `Cairo village@4 (N)`. Left in, one place becomes three across a 36-year
series. [verified] — the county's rows across all 36 years.

**A permit is not a house.** It is authorization to build, issued before construction, and the
survey says nothing about whether the work was done, when it finished, or whether anything was
demolished to make room. The corpus reports permits as permits and never as net change in the
housing stock. [verified] — the survey's own definitions.

**The value is the applicant's estimate at the time of the permit, in that year's dollars.** It
excludes land. Nothing here is inflation-adjusted and no comparison of value across years in this
corpus is offered without saying so. [verified] — same.

**The place rows and the county row disagree by eight houses over thirty-six years.** The county
file gives 5,429 single-family units for 1990–2025 and the places sum to 5,437. That is 0.15 per
cent and the corpus does not reconcile it: the two are separately edited tabulations of the same
returns, and annexation moves ground between places within a year. Each figure is reported against
its own file and the two are never differenced. [verified] — computed from both.
