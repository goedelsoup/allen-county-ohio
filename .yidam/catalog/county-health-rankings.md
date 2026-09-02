---
name: County Health Rankings & Roadmaps annual data
description: >-
  The University of Wisconsin Population Health Institute's county-by-county compilation of health
  measures, published every year since 2010 with numerators, denominators and confidence intervals
  for every county in the United States. For mortality its data system is the National Center for
  Health Statistics, and it is the route by which this corpus can hold county death counts at all —
  the publisher of those counts serves them to a browser and refuses them to a machine.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://www.countyhealthrankings.org/sites/default/files/media/document/chr_trends_csv_2025.csv
    description: >-
      The trends file, 60.4 MB, 745,088 rows — fifteen measures against a `yearspan` rather than a
      release year. **Premature death is the only mortality measure in it**, and it runs twenty-four
      overlapping three-year windows from 1997–1999 to 2020–2022 for every county, every state and
      the nation, with `differflag` and `trendbreak` columns.
  - kind: url
    value: https://www.countyhealthrankings.org/sites/default/files/media/document/analytic_data2025_v3.csv
    description: >-
      The 2025 annual release, 13.1 MB, 3,195 county rows by 770 columns. Every measure carries
      `_rawvalue`, `_numerator`, `_denominator`, `_cilow`, `_cihigh` and `_flag`, and a dozen carry
      `_race_black`, `_race_white`, `_race_hispanic` and four more. Allen County is state code 39,
      county code 003.
  - kind: url
    value: https://www.countyhealthrankings.org/sites/default/files/media/document/CHRR%20Technical%20Documentation%202025_2.pdf
    description: >-
      Names the data years behind each measure, which the CSV does not. For the 2025 release:
      premature death, premature age-adjusted mortality, life expectancy and drug overdose deaths
      are **2020–2022**; injury deaths, suicides and firearm fatalities **2018–2022**; homicides,
      motor vehicle crash deaths and infant mortality **2016–2022**; child mortality **2019–2022**.
  - kind: url
    value: https://www.countyhealthrankings.org/sites/default/files/media/document/Trends%20documentation%202025.pdf
    description: >-
      The trends codebook. Defines `trendbreak` as the start of a new trend whose data "should not
      be used with prior years", and flags premature death at **2004–2006** and **2019–2021** for
      changes in how the population denominator was estimated.
used-by:
  - ../corpus/measure/allen-county-premature-death-1997-2022.yml
  - ../corpus/measure/allen-county-early-deaths-by-cause-2020-2022.yml
---

**It is a compilation, and this entry says what that costs and what it was worth.** Nothing here is
the publisher of its own data. For every mortality measure the source is the National Center for
Health Statistics, and the download link the trends codebook names for premature death is
`wonder.cdc.gov/ucd-icd10.html` — CDC WONDER, whose machine interface answers a national query and
refuses a county one. See
[the API serves the nation and refuses the county](../decisions/the-api-serves-the-nation-and-refuses-the-county.yml).

**The compiler was checked against its own source, and it is exact.** At the one grain WONDER will
serve, deaths before age 75 summed from WONDER's own national table come to **4,125,218** for
2018–2020 and **3,813,889** for 2016–2018. The national numerator in this file for those two windows
is 4,125,218 and 3,813,889 — the same digits, twice. [verified] — CDC WONDER dataset D76 grouped by
year and ten-year age group, and the trends file's national rows.

**That check also settles what the numerator column holds, because the documentation is wrong about
it.** The trends codebook says the numerator is "the number of years of potential life lost". It is
not: 4,125,218 is a count of deaths, and years of potential life lost for the United States over
three years runs to tens of millions. The column holds **deaths before age 75**. A reader who
believed the codebook would understate the county's death count by a factor of about twenty.

**A second control, from a different pipeline.** This file gives Allen County 116 drug overdose
deaths for 2020–2022. CDC's provisional county file, which counts from a live flow of certificates
rather than from the final annual file, gives 34, 33 and 49 for the twelve months ending each
December of those years — 116. [verified] — same file and
[the provisional counts](cdc-provisional-overdose-counts.md).

**What it will not answer.** No cause of death beyond the dozen measures it chooses; no sub-county
grain; and no race category whose population is too small to carry one. Allen County publishes Black
and white figures and almost nothing else, and the Hispanic life expectancy of 87.6 years it does
print rests on a population this corpus will not quote a life table for. The premature-death series
has no numerator for Allen County in the 2009–2011 window, and the publisher's own two trend breaks
mean the **rate** is not continuous across 2004–2006 or 2019–2021. A comparison of numerators is not
affected by either, because both breaks are changes to the denominator.
