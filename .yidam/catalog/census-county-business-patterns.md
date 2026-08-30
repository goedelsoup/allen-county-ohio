---
name: County Business Patterns (Census Bureau)
description: >-
  Annual counts of establishments, employment and payroll by county and industry, from the Census
  Bureau's business register. It gives this corpus the measurement its central historical claim has
  wanted since genesis: manufacturing employment in Allen County, year by year, inside the period
  named for its decline.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/cbp/datasets/
    description: >-
      County files by year as zipped CSV — cbp86co.zip through cbp22co.zip. Allen County is
      fipstate 39, fipscty 003. Nine years were taken: 1986, 1990, 1995, 2000, 2005, 2010, 2015,
      2020 and 2022.
---

**It measures what this corpus named a period after and could not see.**
[Deindustrialization](../corpus/period/deindustrialization.yml) has carried a refusal since genesis
saying the corpus holds no manufacturing employment inside it. This source holds thirty-six years of
it. See [the series](../corpus/measure/allen-county-manufacturing-employment-1986-2022.yml).

**Its industry codes change in the middle, and the break is real.** Files through 1997 classify by
SIC, where the manufacturing division is coded `20--`; from 1998 they classify by NAICS, where
manufacturing is `31----`. The two are close and not identical — publishing left manufacturing when
NAICS replaced SIC — so a change across 1997/1998 is partly a change of definition. [verified] Every
figure this corpus takes is labelled with which scheme produced it.

**Two arithmetic closures were run rather than a second scan.** In 1986 the ten SIC divisions sum to
45,917, byte-equal to the county total row; in 2022 the twenty NAICS sectors sum to 42,814, likewise.
[verified] Intervening years leave residuals of 2 to 239 against totals near 46,000, which is
consistent with the Bureau suppressing employment in its smallest sectors and reporting them as
zero — every year with a residual has at least one sector showing zero employment against a nonzero
establishment count. [inference]

**What it excludes, which matters here.** County Business Patterns counts private employers on the
business register. It omits government, the self-employed, most agricultural production and railroad
employees. So its county total is not the county's workforce, and a comparison against
[the 1909 manufactures figures](../corpus/measure/lima-manufactures-1899-1909.yml) is a comparison
of two different programs a century apart, not a series. [verified]

**A second source corroborates the recent end of it.** For 2023 the
[Quarterly Census of Employment and Wages](bls-qcew.md) reports 8,661 manufacturing employees in
Allen County against this source's 8,573 for 2022 — about one per cent apart, from a different
programme with a different frame. [verified]

**What else is in it, unread.** Establishment size distributions for every sector and year, payroll,
and industry detail below the sector — this corpus has taken the total and one sector from nine
years of a file that runs from 1986 to the present annually.
