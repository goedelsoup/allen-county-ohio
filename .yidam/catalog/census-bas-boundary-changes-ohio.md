---
name: Boundary and Annexation Survey — Ohio boundary changes, 1971–2025
description: >-
  The dataset this corpus said it did not have. Every boundary change an Ohio municipality
  reported to the Census Bureau, one row each: the entity, the legal instrument and its number,
  the date it took effect, and the acreage. For Allen County it is fifty-seven annexations
  between April 1990 and November 2024.
type: dataset
obtained: true
retrieved: 2026-08-31
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/geo/pvs/bas/annexation/39/
    description: >-
      The Ohio directory. One file per survey year from 2011, one per decade before that, tab
      delimited with an Excel twin. `39OH_bas91-00.txt` and `39OH_bas01-10.txt` carry the two
      decades; `39OH_bas2012` through `39OH_bas2025` carry the years, with no 2014 file.
  - kind: url
    value: https://www2.census.gov/geo/pvs/bas/annexation/bas24_readme.txt
    description: >-
      The record layout. It is the readme that says what `Acreage` is — "estimated area of the
      boundary change in acres" — and that the files after 2016 changed shape "due to a database
      upgrade", which is why `Annexation`/`Ordinance` becomes `A`/`O` partway through the series.
  - kind: url
    value: https://www2.census.gov/geo/pvs/bas/annexation/39/39OH_bas81-90.pdf
    description: >-
      1981–1990, and 1971–1980 beside it. Not read. These are OCR of a line-printer report dated
      14-APR-94, and the OCR breaks the column association within rows — a date on one line and
      its ordinance number on the next. See the defect below.
used-by:
  - ../corpus/jurisdiction/city-of-lima.yml
  - ../corpus/measure/allen-county-annexations-1990-2024.yml
  - ../corpus/measure/allen-county-land-area-2000-2024.yml
  - ../corpus/measure/allen-county-outside-lima-1890-2020.yml
  - ../corpus/measure/lima-land-area-2020.yml
  - ../corpus/measure/lima-population-2000.yml
  - ../corpus/place/elida.yml
  - ../corpus/place/fort-shawnee.yml
  - ../corpus/place/jackson-township.yml
---

**What a row is.** An Ohio annexation is not one act. A petition goes to the board of county
commissioners, which acts by **resolution**; the municipality then accepts by **ordinance**. The
`Type` column in these files holds whichever instrument the reporting government put on the form,
which is why the same series carries both words for the same kind of event. Forty-four of Allen
County's fifty-seven rows name an ordinance and thirteen a resolution.

**It is a survey, not a register.** Governments report to the BAS voluntarily. A year with no row
for a place is a year that place reported nothing, which is not the same as a year in which nothing
happened — and five of the fifty-seven Allen County rows carry no acreage at all. The corpus takes
this as a floor and says so wherever it uses it.

**The acreage is checkable and it checks.** Lima reported 601.0 acres of annexation with effective
dates between January 2000 and January 2010. The Census Bureau's own computation of Lima's total
area, from a polygon and not from a form, grew by 0.91 square miles — **582.4 acres** — between the
2000 and 2010 censuses. [Two documents](census-phc-3-37-ohio.md) that
[share no method](census-cph-2-37-ohio-2010.md) agree to 3.1 per cent.

**What it does not carry.** No township is a reporting entity here, so a row says what a
municipality gained and never what a township lost; for that the corpus uses the decennial
geographic change notes. No map, no legal description, no parcel. Nothing before 1971 in any form,
and nothing before 1991 in a form this corpus is willing to read.
