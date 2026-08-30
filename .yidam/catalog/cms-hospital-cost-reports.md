---
name: CMS Hospital Provider Cost Report (HCRIS)
description: >-
  Every Medicare-certified hospital's annual cost report, filed under penalty and published by CMS
  as one row per hospital per fiscal year. It carries staffing, beds, discharges, salaries and a
  full balance sheet — and it is where this corpus first measures a named employer rather than an
  industry.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://data.cms.gov/data.json
    description: >-
      The catalogue. The dataset is titled "Hospital Provider Cost Report"; each fiscal year is a
      separate distribution with its own CSV URL, which changes when CMS republishes.
  - kind: url
    value: https://data.cms.gov/sites/default/files/2026-01/3c39f483-c7e0-4025-8396-4df76942e10f/CostReport_2023_Final.csv
    description: >-
      FY2023, the most recent final file. Thirteen years were taken, FY2011 through FY2023, each
      about 4 MB and 117 columns. Allen County providers are CCNs 360009, 360066, 360263, 361322
      and 362020 — 66 rows across the thirteen files.
---

**It measures employment at a named employer, which no other source in this corpus does.** County
Business Patterns and [BEA](bea-county-employment.md) and [QCEW](bls-qcew.md) all count an industry
in a county; this file counts the hospital. Column `FTE - Employees on Payroll` gives Allen County's
five hospitals year by year, and column `Number of Beds` gives what they were staffed to hold. See
[hospital employment](../corpus/measure/allen-county-hospital-employment-2011-2023.yml) and
[hospital beds](../corpus/measure/allen-county-hospital-beds-2011-2023.yml).

**Full-time equivalents are not employees, and the difference is the whole reason this file cannot
settle a headcount.** An FTE is hours divided by a full-time year, so a hospital's headcount is
always the larger number and by an unknown margin. [verified] Against
[County Business Patterns](census-county-business-patterns.md), which counts people on a March
payroll, the county's five hospitals report 3,677 FTE in 2011 against 4,848 employees, and 2,990
against 3,211 in 2022 — a ratio that moves from 0.76 to 0.93 across the span. [inference] Two
programmes measuring the same five buildings, and the gap between them is not constant, so neither
can be substituted for the other.

**Its year is a fiscal year, and one of the five does not use the calendar.** Lima Memorial,
St. Rita's, Bluffton and the Institute all file 1 January to 31 December. Kindred Hospital Lima
filed on the calendar through FY2013 and has filed 1 September to 31 August ever since. [verified]
The change puts two Kindred rows in the FY2014 file — a stub from 1 January to 31 August 2014 and a
full year from 1 September 2014 — so a county total summed naively over a year column counts that
hospital twice, in the one year where it happens to be wrong.

Every county figure this corpus takes from this file is the four calendar-year filers, with Kindred
stated separately. That is a choice about comparability and it costs between 64 and 94 FTE a year,
which the measure nodes name rather than absorb.

**One line moves for a reason that is not employment.** St. Rita's filed as "St. Ritas Medical
Center" through FY2013 and as "St. Ritas Medical Center LLC" from FY2014, and across that boundary
its reported FTE fell from 2,303 to 1,678 and its salaries from $145.5 million to $94.6 million.
[verified] A renaming that coincides with a quarter of a workforce leaving a cost report is a
change in the reporting entity as readily as a change in the workforce, and this corpus does not
know which. [open]

**What else is in it, unread.** Charity care and bad debt; the wage index components; a complete
balance sheet — land, buildings, equipment, receivables, long-term liabilities and fund balances;
DRG and disproportionate-share payments; Medicaid revenue and charges. A hundred and one columns
this corpus has not touched, for every hospital in the United States for thirteen years.
