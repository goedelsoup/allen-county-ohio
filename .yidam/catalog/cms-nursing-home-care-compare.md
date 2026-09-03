---
name: CMS Care Compare — nursing home provider data
description: >-
  Every Medicare- or Medicaid-certified nursing home in the United States, with its beds, its
  residents, its owner, its staffing hours, its staff turnover, its five-star ratings and every
  deficiency an inspector has cited at it since 2019. It is the first source this corpus has held
  that describes where the county's oldest people live.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 180
location:
  - kind: url
    value: https://data.cms.gov/provider-data/dataset/4pq5-n9py
    description: >-
      *Provider Information* — one row per nursing home, 14,690 of them, 99 columns. Queryable at
      `https://data.cms.gov/provider-data/api/1/datastore/query/4pq5-n9py/0` with
      `conditions[n][property|value|operator]`, or downloadable whole as
      `NH_ProviderInfo_Aug2026.csv`, 9.2 MB.
  - kind: url
    value: https://data.cms.gov/provider-data/dataset/r5ix-sfxw
    description: >-
      *Health Deficiencies* — one row per citation, with the survey date, the F-tag, the scope and
      severity letter, and whether it came from a standard survey or a complaint. 287 rows for
      Allen County across the twelve homes, 21 February 2019 to 20 May 2026.
used-by:
  - ../corpus/measure/allen-county-nursing-homes-2026.yml
---

**The snapshot has a date and it is not the date of the thing.** `processing_date` on every row of
the August 2026 file is `2026-08-01`, but its staffing figures come from a quarter of payroll data,
its turnover from a rolling year, and its inspection ratings from up to three survey cycles reaching
back to 2019. A row is a composite of six windows wearing one date. [verified] — the file and its
own data-collection-intervals table.

**The county field is not a boundary source, and here it is wrong.** `countyparish` derives from the
provider's enrollment record rather than from where the building stands. For The Meadows of Delphos
it reads *Van Wert*; the Census Bureau's address ranges put 800 Ambrose Drive in **Allen County**
on an exact match, and CMS's own cost report for the same certification number reads *ALLEN* in
every one of its six Delphos years. The provider file also misspells the street as `AMBOSE`, which
is why a geocoder has to be asked twice. [verified] — the file, the
[Census geocoder](https://geocoding.geo.census.gov/), and
[the cost reports](cms-snf-cost-reports.md); and see
[a postal address is not a municipality](../decisions/a-postal-address-is-not-a-municipality.yml).

**The published latitude and longitude are rounded to about a hundred metres.** Three or four
decimal places, which is enough to place a building in a county in the middle of one and not enough
near a line. The corpus uses them to find candidates and the address ranges to decide. [verified] —
the file.

**Scope and severity is a letter and it carries the whole weight.** A through L on a grid of how
many residents were affected against how much harm was done: A to C is no actual harm, D to F is
harm that is not immediate jeopardy, G to I is actual harm, and J to L is immediate jeopardy. A
count of deficiencies without the letters treats a missing signature and a resident in danger as one
thing each. [verified] — the file's own codes.

**What it is not.** It is not a register of every place old people are cared for in a county: it
holds only Medicare- and Medicaid-certified nursing facilities, and neither assisted living, nor
residential care, nor a home where somebody is looked after by their family. The 2020 census counted
966 people in nursing and skilled-nursing group quarters in Allen County against the 742 a day the
certified homes billed for that year, and the difference is the part this source cannot see.
