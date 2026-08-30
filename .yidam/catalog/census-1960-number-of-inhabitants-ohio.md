---
name: 1960 Census of Population, Volume I — Number of Inhabitants, Ohio (Part 37)
description: >-
  The Census Bureau's own place-level history for Ohio, published 1961. Its Table 5 carries every
  incorporated place of 10,000 or more from its earliest census to 1960 — for Lima, twelve counts
  from 1850 — and each row prints the increase over the preceding census beside the count, so the
  table checks its own arithmetic.
type: dataset
obtained: true
retrieved: 2026-08-29
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1960/population-volume-1/37749282v1p37_ch02.pdf
    description: >-
      9.9 MB PDF, 44 pages, the "Number of Inhabitants" chapter of Volume I Part 37. No text layer:
      every page is a 300-dpi bitonal scan, and everything this corpus takes from it was read from
      rendered images. Table 5 begins on printed page 37-10; Lima is on 37-12.
used-by:
  - ../corpus/measure/lima-population-1850-1960.yml
  - ../corpus/measure/lima-population-1880-1920.yml
  - ../corpus/measure/lima-population-1970-1990.yml
  - ../corpus/period/deindustrialization.yml
  - ../corpus/question/pre-1970-population-series.yml
---

**What it settles.** [`pre-1970-population-series`](../corpus/question/pre-1970-population-series.yml)
was opened to ask two things, and after three phases one was still standing: *when did Lima peak?*
The corpus had 1970 as the earliest of three federal counts for the city and said so — a lower bound
and not a peak. This volume supplies 1850 through 1960 and closes it. **Lima peaked at the 1970
census**, and every decennial count before it rises.

**The table checks its own arithmetic, and that is why one scan was enough.** Table 5 prints, for
each census year, the population *and* the increase over the preceding census in both number and
percent. Lima's block holds twelve counts, eleven differences and eleven percentages, and every one
of them is consistent with its neighbours:

    1960   51,037     791    1.6        1900   21,723   5,742    35.9
    1950   50,246   5,535   12.4        1890   15,981   8,414   111.2
    1940   44,711   2,424    5.7        1880    7,567   3,067    68.2
    1930   42,287     961    2.3        1870    4,500   2,511   126.2
    1920   41,326  10,818   35.5        1860    1,989   1,232   162.7
    1910   30,508   8,785   40.4        1850      757     ...     ...

A misread digit anywhere in the left column breaks the chain above it and below it. The
[two-scan rule](../decisions/two-scans-of-one-book.yml) was written for a source that
offers no such check; this one offers twenty-two, and they are stronger than a second scan of the
same page would have been.

**It contradicts a county history and proves it.** [Rusler's 1921 volume](rusler-allen-county-1921.md)
gives Lima 41,306 in 1920, twice, in two chapters. This table gives **41,326**, and both of its
neighbouring rows require that value: 30,508 + 10,818 = 41,326, and 41,326 + 961 = 42,287. The 1921
figure fails both. The same Rusler passage subtracts its own list of incorporated places from the
county total and lands a thousand out, so the arithmetic there was not close-read by its author.

**What it does not carry — for this scan, and stated narrowly.** Table 7, *Population of Counties,
by Minor Civil Divisions: 1940 to 1960*, holds exactly what this corpus wants next: Allen County's
twelve townships at three mid-century censuses, with Allen first in the volume because the counties
run alphabetically. **It cannot be read from this scan.** The page is the same 300-dpi bitonal
rendering as Table 5, but the type is smaller and more tightly leaded, and the thresholding has
closed the counters — the county-total row, set in bold, is the least legible line on the page.
Legibility here is a property of the table, not of the source, and no amount of re-rendering
recovers it because the stored image is already at its native resolution. A second digitization
would settle it.

Also absent: Allen County's own 1930 count, which is the last gap in the county series. Table 6
gives counties for 1960 and 1950 only, and Table 7's county totals are unreadable, so this volume
does not reach 1930 for the county even though it reaches 1850 for the city.
