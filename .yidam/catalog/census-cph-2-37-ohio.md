---
name: 1990 Census of Population and Housing — Population and Housing Unit Counts, Ohio (CPH-2-37)
description: >-
  The Census Bureau's own historical counts for Ohio, published 1993. It carries county population
  and housing units decade by decade from 1940 to 1990 — the four counts this corpus has been
  missing between the county histories and its own 1970 figure — and the geographic change notes
  that record which townships Lima annexed from.
type: dataset
obtained: true
retrieved: 2026-08-29
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1990/cph-2/cph-2-37.pdf
    description: >-
      3.5 MB PDF, 12,700 lines when extracted with `pdftotext -layout`. Table 3 is the county
      series; the Geographic Change Notes are in the front matter under "User Note 1".
used-by:
  - ../corpus/jurisdiction/city-of-lima.yml
  - ../corpus/measure/allen-county-outside-lima-1940-2020.yml
  - ../corpus/measure/allen-county-population-1940-1990.yml
  - ../corpus/measure/lima-population-1850-1960.yml
  - ../corpus/measure/lima-population-1970-1990.yml
  - ../corpus/period/deindustrialization.yml
  - ../corpus/question/pre-1970-population-series.yml
  - ../corpus/question/what-became-of-german-and-ottawa-townships.yml
---

**What it settles.** [`pre-1970-population-series`](../corpus/question/pre-1970-population-series.yml)
named this document's family in its `would_close_this` from the day it was opened — "the Census
publication *Population of States and Counties of the United States: 1790–1990* (a 14.9MB PDF
located but not read)". This is the Ohio volume of the same programme, a quarter the size, and it
answers the county half outright.

**It is the corpus's first federal historical publication**, and the difference from the county
histories shows immediately. It is not a narrative with figures in it; it is a table with a
`[For information concerning historical counts, see "User Notes."]` header, revision marks on
individual cells, and a note explaining that Ohio's 1970 urban counts were corrected for an
erroneously defined CDP. Where a county history says "the population in 1850 was 12,100" and a
second says 12,116, this says `r47 827` and tells you what the `r` means.

**Its Ohio user note explains something three phases of this corpus could not.** "The Census Bureau,
in agreement with the State of Ohio, does not recognize an Ohio township that is coextensive with a
village or a city, and treats such places as independent of any MCD." That is why
[Ottawa Township](../corpus/question/what-became-of-german-and-ottawa-townships.yml) is absent from
every present-day federal source the corpus holds — not abolition, a reporting rule.

**What it does not carry.** Places get 1970 to 1990 only, so **Lima 1930 to 1960 is not here** and
the city's peak was still bracketed when this entry was written. It is not now:
[the 1960 volume](census-1960-number-of-inhabitants-ohio.md) supplies 1850 to 1960 for places, and
Lima peaked at the 1970 census this table gave. County figures stop at 1940 going back, so
1930 remains the one gap between the 1921 history's 1920 and this table's 1940. No township-level
history, no vital statistics, no employment.

**A note on extraction.** `pdftotext -layout` renders these tables as fixed-width rows whose columns
must be counted off against a header printed pages earlier. Every figure this corpus takes was read
by locating the header row and counting positions, and checked against a value the corpus already
held from an unrelated source: Allen County's 1970 count of 111,144, which matches the
[1970–1979 county estimates file](census-county-estimates-1970s.md) exactly. A column-alignment
error would have shown there first.
