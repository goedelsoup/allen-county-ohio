---
name: Fair Market Rents (HUD)
description: >-
  The rent HUD says a modest unit costs in a given place, published every fiscal year for every
  county and metropolitan area in the country and by ZIP code inside metropolitan areas. It is the
  number a housing voucher is sized against, so it is both a statement about the local rental market
  and an administrative decision about how much the government will pay.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://www.huduser.gov/portal/datasets/fmr/fmr2025/FY25_FMRs.xlsx
    description: >-
      Fiscal 2025, 14 columns, one row per county or county-part. Allen County is `fips`
      3900399999, area `Lima, OH MSA`: $788 for a studio, $793 for one bedroom, $1,040 for two,
      $1,262 for three and $1,431 for four.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/fmr/fmr2026/FY26_FMRs.xlsx
    description: >-
      Fiscal 2026, the same shape with `pop2022` renamed `pop2023`. Allen County: $839, $844,
      $1,108, $1,338, $1,493.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/fmr/fmr2026/fy2026_safmrs_revised.xlsx
    description: >-
      Small Area Fair Market Rents, 51,895 rows of ZIP code by fair-market-rent area, with 90 and
      110 per cent payment standards beside each figure. The column headers contain embedded
      newlines.
  - kind: url
    value: https://www.huduser.gov/portal/datasets/fmr.html
    description: The download page, which carries every fiscal year back to 1983 on varying paths.
used-by:
  - ../corpus/measure/allen-county-housing-cost-burden-2006-2022.yml
---

**The Lima area's two-bedroom rent rose 6.5 per cent in one fiscal year**, $1,040 to $1,108.
[verified] — the fiscal 2025 and 2026 county files. This is not a measurement of the market moving
by that much; it is HUD's estimate moving, and the estimate is rebuilt each year from a different
American Community Survey vintage plus a forecast.

**A ZIP code row is not a place.** In the small-area file the same ZIP appears once for every
fair-market-rent area it touches, with a different rent under each: 45806 is listed under both
`Lima, OH MSA` and `Auglaize County, OH`, and 45817 under both Lima and `Hancock County, OH`.
[verified] — the same file. Summing or averaging the rows for a county's ZIPs double-counts every
straddle.

**Inside the Lima area the small-area rents span $140.** Nine ZIP codes are listed for the metro,
and their two-bedroom figures run from $1,040 in 45817 and 45833 — Bluffton and Delphos, both at a
county edge — to $1,180 in 45807, against a metro-wide figure of $1,108. [verified] — the same file.
Whether any of these are in force for a voucher here is a decision of the housing authority and is
not in this file.

**It is a payment standard before it is a statistic.** The figure is set at a percentile of local
gross rent for recent movers, adjusted, and is then the ceiling a voucher is sized to. A county
whose fair market rent sits above what its median renter actually pays is not necessarily an
expensive county; it may be a county whose voucher-holders can reach most of its stock.
[inference] — the reasoning is this corpus's.

**Same bot filter as the rest of huduser.gov.** Without a browser user-agent the request succeeds
and returns nothing. [verified] — see
[A Picture of Subsidized Households](hud-picture-of-subsidized-households.md).
