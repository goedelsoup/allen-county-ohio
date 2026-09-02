---
name: Census of Agriculture historical archive, Ohio 1925–1978 (NASS)
description: >-
  The full printed run of the United States Census of Agriculture, scanned county table by county
  table, on the National Agricultural Statistics Service's own host. Nine Ohio volumes read here
  fill every remaining gap in Allen County's farmland record between 1910 and 2002.
type: publication
obtained: true
retrieved: 2026-09-02
ttl_days: 3650
location:
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/
    description: >-
      The index. Twenty-eight census years from 1840 to 2012, each with a page per state part and a
      separate PDF for every table in it. This is the Cornell agricultural census archive: the old
      `agcensus.library.cornell.edu` and `agcensus.mannlib.cornell.edu` addresses **301-redirect
      into it**, which is how it was found.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1925-Ohio-CountyTables-Table-01.pdf
    description: >-
      1925, County Table 1. Rows 29–31 give land in farms for **1925, 1920 and 1910** in one block,
      and rows 1–3 the farm counts for the same three years. Allen is the third numeric column,
      after the state and Adams.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1940-Ohio-COUNTY_TABLES-1265-Table-01.pdf
    description: >-
      1940, County Table 1. Rows 6–8 give land in farms for **1940, 1935 and 1930**, rows 1–3 the
      farm counts. One table, three censuses.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1945-Ohio-COUNTY_TABLES-1171-Table-01.pdf
    description: >-
      1945, County Table 1 part 1. Laid out with two columns per county, Census of 1945 beside
      Census of 1940 — so it reprints 1940 as a check on the volume above.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1959-Ohio-CHAPTER_B_-_STATISTICS_FOR_COUNTIES-866-Table-01.pdf
    description: >-
      1959, County Table 1. Prints 1959 beside 1954, and row 3 gives the **decrease in farms due to
      the change of farm definition between them: 75 for Allen County**, 7,017 for Ohio.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1964-Ohio-COUNTY_TABLES-725-Table-01.pdf
    description: 1964, Table 1. Prints 1964 beside 1959.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1969-Ohio-County-Ohio_countyData_1969_Allen.pdf
    description: >-
      1969 — a **separate PDF for each Ohio county**, unlike every other year here. Table 1 gives
      1969 beside 1964 for Allen County alone.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1974-Ohio-CHAPTER_2._County_Summary_Data-306-Table-01.pdf
    description: 1974, Table 1. Prints 1974 beside 1969, one county to two rows.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1978-Ohio-CHAPTER_2_County_Summary_Data-182-Table-01.pdf
    description: 1978, Table 1. Prints 1978 beside 1974.
  - kind: url
    value: https://www.nass.usda.gov/AgCensus/archive/files/1982-Ohio-CHAPTER_2_County_Data-122-Table-01.pdf
    description: >-
      1982, Table 1. Prints 1982 beside 1978, and both agree to the acre with figures already read
      from two other books — so it is corroboration rather than a new year.
used-by:
  - ../corpus/measure/allen-county-farmland-1910-2022.yml
  - ../corpus/question/when-the-farmland-went.yml
---

**The shelf this corpus said did not exist.** [The farmland question](../corpus/question/when-the-farmland-went.yml)
recorded that "the Cornell historical archive that used to hold the scans now redirects into
nass.usda.gov without them". It redirects into nass.usda.gov **with** them: the whole archive is
mirrored at `/AgCensus/archive/`, table by table, and the old Cornell addresses lead straight to it.
[verified] — the redirect chain, followed 2026-09-02.

**Every volume prints the census before it, which makes the run self-checking.** Ten volumes give
fifteen census years, and ten of those years are printed twice in two different books. Allen
County's land in farms agrees to the acre across every one of those pairs but one. [verified] — the
ten tables above.

**The exception is 1964, and it is the same shape as a divergence this corpus has already recorded.**
The 1964 volume prints **211,196** acres and the 1969 volume prints **211,195** for the same county
in the same census year. One acre, one bureau, five years apart, with nothing on either page to
catch it. By [a reprint is not a second witness](../decisions/a-reprint-is-not-a-second-witness.yml)
the census's own year wins and the divergence is published. [verified] — the two tables.

**A second, smaller divergence is in a derived column rather than a count.** The 1974 volume gives
Allen County 78.5 per cent of its land area in farms for 1974 and the 1978 volume gives 78.4 for the
same year, on an identical acreage — a rounding of the land-area denominator rather than a
disagreement about the county. [verified] — the two tables.

**The text layers are OCR and were used only to find pages.** They mangle digits in the way these
typefaces always do: the 1925 table's text gives Ohio's 1920 land in farms as 23,615,888 where the
page prints 23,515,888, and the 1964 table renders 4 as `ij` and 3 as `)` throughout. Every figure
taken here was read from a 300-dpi render of the page.

**What it will not answer.** The 1950 Ohio part is a table of contents and nothing else, so that
census still comes from [the 1954 volume on archive.org](census-of-agriculture-historical.md). 1920
and 1930 have no Ohio state part in this archive — 1920 has no
parts listed at all and 1930 is bound by region, into "the northern states", which was not read.
Both years are nonetheless in the record, because the 1925 volume prints 1920 and the 1940 volume
prints 1930.
