---
name: CMS skilled nursing facility cost reports, 2011–2023
description: >-
  The annual financial and utilisation report every Medicare-certified nursing home files, thirteen
  years of it. It is where the days come from — how much nursing-home care a county actually
  delivered in a year, and who paid for it.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://data.cms.gov/provider-data/dataset/skilled-nursing-facility-cost-report
    description: >-
      The dataset page. Thirteen annual releases, each a separate distribution; the CSV download
      URLs are in `https://data.cms.gov/data.json` under the *Skilled Nursing Facility Cost Report*
      entry and run `CostReportsnf_Final_11.csv` through `CostReportsnf_Final_23.csv`, about 8.7 MB
      each.
  - kind: url
    value: https://data.cms.gov/sites/default/files/2025-11/34ea98e4-20f4-42f7-b5b2-616d35b0fe93/CostReportsnf_Final_23.csv
    description: >-
      Fiscal 2023, the latest final year, 122 columns. Twelve Allen County rows; the county's whole
      run is 2011 through 2023 and holds 10 to 14 rows a year.
used-by:
  - ../corpus/measure/allen-county-nursing-home-days-2011-2023.yml
---

**The days are sound and the beds are not.** `Total Days Total` — resident days, split into Title
XVIII (Medicare), Title XIX (Medicaid) and other — tracks smoothly across the run and agrees with
an independent count: 270,773 Allen County days in 2020 is 742 residents a day, against the 712.5
[Care Compare](cms-nursing-home-care-compare.md) reports for 2026. `Number of Beds` does not. It
doubles for one facility or another in 2014, 2016, 2017, 2018, 2019 and 2022 while
`Total Bed Days Available` for the same rows stays where it was, and in 2017 both fields double for
two homes at once — Shawnee Manor filing 274 beds and 100,010 bed-days in a year it reports 137 and
50,005 on either side. An occupancy series computed from the published denominator shows a cliff in
2017 that is arithmetic rather than care. [verified] — the thirteen files, compared row by row.

**One certification number, six facility names.** CCN 365202 files as Lima Acres Nursing & Rehab
Center, Golden Living-Lima, Lima Healthcare LLC, The Orchards of Lima Living & Rehab — spelled
three ways — and CareCore at Lima, across thirteen years at one address. The number is the stable
key here and the name is not, which is the opposite of what
[the injury summaries](osha-injury-tracking-application.md) do with the same kind of establishment.
[verified] — the same files.

**A change of ownership splits a year in two.** CCN 365936 files 1 January to 20 April 2017 as
Baton Rouge Medical & Rehab Center and 21 April to 31 December 2017 as Liberty Retirement Community
of Lima. Summing days over rows handles it; counting rows as facilities does not. [verified] — the
2017 file.

**A blank report is still a report.** Willow Ridge of Mennonite Home Communities files in eleven of
thirteen years and reports no beds and no days in eight of them. A county total that divides by the
number of filers understates the care each one gave. [verified] — the same files.

**What it is not.** It is not a census of nursing-home residents, because it counts days billed by
certified facilities rather than people cared for; it is not a source on quality; and its `County`
field is the one the provider entered, which is why it says ALLEN for a Delphos facility whose
[Care Compare](cms-nursing-home-care-compare.md) row says Van Wert. On that disagreement the
address ranges decide, and they agree with the cost report.
