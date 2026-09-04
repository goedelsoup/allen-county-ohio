---
name: IRS Statistics of Income county-to-county migration
description: >-
  Every county-to-county move in the United States large enough to publish, once a year for nineteen
  years, built by comparing the address on a tax return with the address on the same filer's return
  the year before. It carries the money with the people: returns, exemptions and aggregate adjusted
  gross income for each flow.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://www.irs.gov/pub/irs-soi/countyoutflow2223.csv
    description: >-
      Outflow for 2022–23, 4.5 MB: one row per origin county and destination. `countyinflow2223.csv`
      is the same for arrivals. Nineteen pairs exist, `0405` through `2223`; `0304` and `2324` return
      404. Nine columns in every vintage — two FIPS pairs, a state abbreviation, a name, returns,
      exemptions and aggregate AGI in thousands of dollars.
used-by:
  - ../corpus/measure/allen-county-migration-flows-2004-2023.yml
  - ../corpus/measure/allen-county-migration-by-county-2004-2023.yml
---

**The two files put their columns in opposite orders and neither says so in the header of the
older vintages.** Outflow begins with the origin and inflow begins with the destination — `y1` then
`y2` in one and `y2` then `y1` in the other. Files to 2010–11 name the columns
`State_Code_Origin` and `State_Code_Dest` and swap which pair comes first; files from 2011–12 use
`y1_statefips` and `y2_statefips` and swap them too. A parser that takes columns one and two as the
origin reads the inflow file backwards and gets a plausible, wrong answer. [verified] — the headers
of both files in three vintages.

**A migrant here is a tax return that moved, and a person is an exemption.** The unit of
observation is a filer whose address differs from the address on the previous year's return;
`n1` counts returns, `n2` counts exemptions claimed on them, and `agi` is the money on those returns
in thousands of dollars. People who do not file are not in it — the very poor, many of the old, and
anyone whose return was late enough to miss the matching window.

**A flow smaller than ten returns is not published, and the residue is aggregated by region.** The
smallest named flow into or out of Allen County in nineteen years is exactly ten returns, of 697
named flows. [verified] — the county's own rows. What is suppressed reappears in rows coded 57, 58
and 59 — *Other flows – Same State*, *Different State*, *Northeast*, *Midwest*, *South*, *West* —
so the totals are complete and the geography is not. For Allen County the named counties are 5,329
of a net loss of 9,171 exemptions; the other 3,842 have a region and no county.

**Foreign migration stops being published for this county after 2009–10.** Rows coded 98 carry a
figure through the first six years and `-1` thereafter. [verified] — the county's own rows. A `-1`
in any field means suppressed and never zero; see
[a zero is not a blank](../decisions/a-zero-is-not-a-blank.yml).

**Two of the nineteen years are the file rather than the county.** 2014–15 is low and 2016–17 is
high in Allen County — 2,661 and 5,097 exemptions out against a nineteen-year median near 3,750 —
and Auglaize and Hancock counties move the same way in the same two years. [verified] — the same
files, three counties read together. See
[the control can be the county next door](../decisions/the-control-can-be-the-county-next-door.yml).
