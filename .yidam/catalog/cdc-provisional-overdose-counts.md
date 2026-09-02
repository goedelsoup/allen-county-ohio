---
name: CDC VSRR provisional drug overdose death counts
description: >-
  CDC's Vital Statistics Rapid Release counts of drug overdose deaths, published monthly as
  twelve-month-ending totals from a live flow of death certificates rather than from the final
  annual file. Two datasets are read: one at county grain and one at state and national grain, the
  second serving as the control the first cannot supply.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 180
location:
  - kind: url
    value: https://data.cdc.gov/resource/gb4e-yj24.json
    description: >-
      County-level provisional counts, Socrata JSON. Filtered on `fips='39003'` it returns **72
      rows**, one for each month from January 2020 to December 2025, each a count for the twelve
      months ending that month. Data as of 2026-07-05. `percentage_of_records_pending` is 0 for
      every window after December 2020.
  - kind: url
    value: https://data.cdc.gov/resource/xkb8-kh2a.json
    description: >-
      State and national provisional counts, monthly twelve-month-ending, from 2015. Filtered to
      `indicator='Number of Drug Overdose Deaths'` it gives Ohio and the United States on the same
      basis as the county file, and runs three months further — to March 2026 against the county
      file's December 2025.
used-by:
  - ../corpus/measure/allen-county-drug-overdose-deaths-2020-2025.yml
---

**Every figure is a twelve-month-ending total, not a month.** Consecutive rows overlap by eleven
months, so the series is a rolling count and a rise from one row to the next is one month replacing
one month a year earlier. Nothing here is a monthly death count and no row should be added to
another.

**The county rows cannot be summed into a state, and the trap is shaped to deceive.** Counts below
ten are withheld. Ohio's counties reporting a figure fall from **73** in the twelve months ending
December 2020 to **45** in the twelve months ending December 2025, as county after county drops
under the threshold — so a sum of the published county rows falls by more than the state did, and
would manufacture a decline out of the suppression rule. The state file is the control, and this
corpus uses it. [verified] — the two datasets, counted here.

**Allen County is never suppressed.** Its seventy-two windows run from 16 to 54, so no row is
withheld and the series has no holes. [verified] — same file.

**Provisional means the number can still move.** CDC's own note says counts "may not include all
deaths that occurred during a given time period" and that numbers "may differ from published reports
using final data". Where the final file can be checked against this one it agrees:
[County Health Rankings](county-health-rankings.md) gives Allen County 116 overdose deaths for
2020–2022 from NCHS's final mortality files, and this file's December windows for those three years
sum to 116. [verified] — the two sources.

**The two files are of different vintages and are compared at the earlier endpoint.** The state and
national file runs to March 2026 and the county file stops at December 2025; every comparison this
corpus makes between them is made at December 2025.
