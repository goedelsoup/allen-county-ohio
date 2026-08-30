---
name: BEA Regional Economic Accounts — county employment by industry
description: >-
  The Bureau of Economic Analysis's county employment series, CAEMP25S for 1969–2000 under SIC and
  CAEMP25N for 2001–2022 under NAICS. It reaches thirty-one years further back than
  [County Business Patterns](census-county-business-patterns.md) and covers the whole of the span
  this corpus calls deindustrialization.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://apps.bea.gov/regional/zip/CAEMP25S.zip
    description: Employment by SIC industry, all counties, 1969–2000. Allen County is GeoFIPS 39003.
  - kind: url
    value: https://apps.bea.gov/regional/zip/CAEMP25N.zip
    description: Employment by NAICS industry, all counties, 2001–2022.
---

**It closes the gap the last phase named and falsifies the sentence that named it.** That phase
recorded the years 1970 to 1986 as open and described them in the same breath: manufacturing in 1986
was "already falling", and the corpus had "the second half of a fall". Neither is true. Allen County
manufacturing employment peaked at **18,400 in 1973**, stood at 14,349 in the recession year 1982,
and had recovered to **17,163 by 1986**. [verified] See
[the series](../corpus/measure/allen-county-manufacturing-employment-1969-2022.yml) and
[naming a gap is not leaving it empty](../decisions/naming-a-gap-is-not-leaving-it-empty.yml).

**It counts jobs, where County Business Patterns counts employees.** BEA includes proprietors,
part-time work, farm and government; CBP counts employees of private establishments in a March pay
period. So BEA's figures run higher, and the two are not a series. [verified] The gap between them
narrows over time — 1,401 in 1986, 1,834 in 2000, 749 in 2010, 142 in 2022 — which is consistent
with manufacturing proprietorships thinning out and is not established here. [inference]

**Where they can be compared they agree on shape.** Both put the recent trough at the end of the
2000s — BEA in 2009, CBP in 2010 — and both show a recovery of about a fifth since. [verified]

**Its own break is at 2001**, where BEA switches from SIC to NAICS, and the corpus reports the two
segments separately for that reason. Manufacturing reads 13,106 for 2000 under SIC and 11,897 for
2001 under NAICS, a step of 1,209 that is partly definitional. [verified]

**What else is in it, unread.** Twenty-three industry lines per county per year for fifty-four years,
including a farm/nonfarm split, proprietors' employment, and a government breakdown into federal
civilian, military, state and local. This corpus has taken two of the twenty-three lines.
