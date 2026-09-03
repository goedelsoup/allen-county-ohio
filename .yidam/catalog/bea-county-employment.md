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
used-by:
  - ../corpus/measure/allen-county-total-employment-1969-2022.yml
  - ../corpus/measure/allen-county-manufacturing-employment-1969-2022.yml
  - ../corpus/measure/allen-county-proprietors-1969-2022.yml
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

**The line codes above 100 are industries and the five below it are the whole workforce split two
ways.** Line 10 is total employment, 20 wage and salary, 40 proprietors, 50 farm proprietors and 60
nonfarm proprietors, with 40 equal to 50 plus 60 in every Allen County year checked. Those five had
been in this corpus's hands since August and only line 10 was read. [verified] — the two zips,
Ohio files, GeoFIPS 39003.

**The proprietor lines are what makes this source say something no other source here can.** The
occupational survey and the insurance filings both count people on a payroll, so neither can see a
proprietor; this file counts 11,964 of them in Allen County in 2022, nearly one job in five.
[verified] — same source; see
[the proprietors](../corpus/measure/allen-county-proprietors-1969-2022.yml).

**The join between the two tables moves the total, so it is not a reshuffle.** Total employment goes
from 72,973 in the last SIC year to 69,825 in the first NAICS year, wage and salary from 62,741 to
60,335 and proprietors from 10,232 to 9,490. Under
[a-revision-that-moves-a-category](../decisions/a-revision-that-moves-a-category.yml) an aggregate
that moves is the sign that the change is not confined to how the parts are grouped, so the two
tables are read as two series and nothing is differenced across the join. [verified] — same source,
the 2000 and 2001 columns.

**Industry detail is suppressed at small cells.** Forestry and fishing, utilities and mining all
return `(D)` for this county in most years, so the industry lines cannot be summed to the total and
none of them is read here. [verified] — same files.
