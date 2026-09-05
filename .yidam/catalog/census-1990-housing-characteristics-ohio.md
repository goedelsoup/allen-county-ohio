---
name: 1990 Census of Population and Housing — General and Detailed Housing Characteristics, Ohio (CH-1-37, CH-2-37)
description: >-
  The two 1990 housing volumes for Ohio, published 1993. They carry, for every county and every
  place of 1,000 or more, the questions the census asked about the building rather than the
  household — plumbing, kitchen, telephone, vehicles, rooms, crowding, structure and year built —
  and they are the only pre-2005 measurement of any of them this corpus holds for Allen County.
type: dataset
obtained: true
retrieved: 2026-09-04
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1990/ch-1/ch-1-37.pdf
    description: >-
      *General Housing Characteristics: Ohio*, 10.2 MB, 57,070 lines under `pdftotext -layout`.
      Table 1 is the summary by county; the Allen County row is at extracted line 1076 and Lima's
      at 1625.
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1990/ch-2/ch-2-37.pdf
    description: >-
      *Detailed Housing Characteristics: Ohio*, 11.1 MB, 56,857 lines. Table 1 is the summary by
      county; Allen County is at extracted line 1207 and Lima at 1587. Table 102 is the percent of
      each answer that was allocated rather than reported.
used-by:
  - ../corpus/measure/allen-county-housing-1990.yml
  - ../corpus/measure/allen-county-housing-age-2023.yml
  - ../corpus/measure/allen-county-tenure-2010-2020.yml
---

**The two volumes are one design.** CH-1 is the complete count and CH-2 the sample; both print a
county summary table with the same row order, so a county is one line in each and the two lines
together give twenty-eight numbers. Allen County's, whole:

    CH-1  109,755 persons · 42,758 units · median 5.7 rooms · 74.1% one-unit · 5.7% in
          buildings of 10 or more · 39,408 occupied · 2.33 persons per unit · 0.45 per room
          · 71.7% owner · 1.7% at 1.01 or more per room · 24.0% householder 65 or over
          · 23.6% one-person · median value $51,800 · median contract rent $262
    CH-2  1.6% condominium · 0.4% lacking complete plumbing · 0.6% lacking complete kitchen
          · 10.6% built 1980 to March 1990 · 27.8% built 1939 or earlier · median year built
          1957 · 16.1% moved in during the last year · 8.4% no vehicle · 5.0% no telephone
          · median owner cost with a mortgage $526, without $180 · median gross rent $347

**Percentages, not counts, and that is a limit worth stating.** The county summary prints shares
against a base it also prints, so a count can be recovered — 0.4 per cent of 42,758 is about 171
units — but only to the precision of one decimal place, which on a base of forty thousand is plus
or minus twenty-one units. This corpus reads these as shares and converts only where it says so.

**Table 102 publishes the allocation rate, which is a modern courtesy.** Of Allen County's 39,408
occupied units, the year the structure was built was allocated for 25.2 per cent and the plumbing
answer for 1.0. [verified] — CH-2, Table 102. A quarter of the county's 1990 year-built figures are
imputed, which is a caution the 2023 survey does not print at all.

**Its questions are not all still asked.** Telephone in unit was dropped after 2000; source of
water and sewage disposal after 1990. A source that measures something no later source measures is
a terminus rather than the start of a series, and this corpus says so where it uses one.

**What it is not.** It is not a series. The 1980 and 1970 volumes exist on the same server under
their own numbering and have not been read, so every figure taken from here is a single year
against 2023 and not a trend. See
[a before and after needs a before](../decisions/a-before-and-after-needs-a-before.yml).
