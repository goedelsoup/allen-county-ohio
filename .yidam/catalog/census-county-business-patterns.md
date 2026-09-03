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
      County files by year as zipped CSV — `<year>/cbp<yy>co.zip`. Allen County is fipstate 39,
      fipscty 003. **All thirty-eight years from 1986 to 2023 are now taken**, replacing the nine
      that were sampled from them.
used-by:
  - ../corpus/measure/allen-county-private-employers-1986-2023.yml
  - ../corpus/measure/allen-county-manufacturing-employment-1986-2022.yml
  - ../corpus/measure/allen-county-health-care-employment-2010-2022.yml
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

**The header changes case, the archive changes name, and one column changes name.** Files through
2014 have lower-case column names and files from 2015 have upper-case ones; the member inside the
2007 and 2008 archives is `Cbp07co.txt` and `Cbp08co.txt` with a capital C where every other year is
lower case; and the smallest establishment-size column is `n1_4` through 2016 and `n<5` from 2017.
[verified] — the thirty-eight archives. Each of the three returns nothing and raises nothing: a
reader written against one era of the file silently loses the other.

**The column count runs 23, then 26 from 2007.** The three added are `emp_nf`, `qp1_nf` and `ap_nf`,
noise flags on the three figures beside them. [verified] — the same archives.

**The published industry detail halves in 2017.** Allen County has 1,120 industry rows in 2016 and
669 in 2017, falling to 634 by 2022. [verified] — the same archives. That is a change in what the
Bureau discloses and not a change in the county's economy, and it is why an industry series at
detailed NAICS cannot be carried across that year.

**The size columns close against the establishment total.** In 1986 and in 2023 the nine size classes
sum to `est` exactly — 2,763 and 2,239. [verified] — the same archives. That closure is what makes
the size distribution usable where the employment columns are suppressed.

**Payroll is here and is in the dollars of its year.** `ap` is annual payroll in thousands and `qp1`
is first-quarter payroll; neither is adjusted for anything. The deflator this corpus now holds is
[the consumer price index](bls-cpi.md).
