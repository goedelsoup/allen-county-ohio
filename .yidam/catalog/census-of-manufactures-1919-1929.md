---
name: Census of Manufactures, Ohio state chapters, 1919 and 1929
description: >-
  The two state volumes that carry Lima and Allen County between the 1910 supplement this corpus
  already reads and the federal employment series that begins in 1969. The 1919 volume prints 1914
  beside 1919 for every Ohio city; the 1929 volume is the first that reports Allen County at all.
type: publication
obtained: true
retrieved: 2026-09-01
ttl_days: 3650
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1920/volume-9/06229683v9ch09.pdf
    description: >-
      Fourteenth Census, Volume IX — Manufactures 1919, Reports for States. Chapter 9 of thirteen,
      131 pages, 26 MB. Ohio's section runs from about book page 1140. Table 14, *Character of
      ownership, for selected industries and for cities: 1919 and 1914*, has Lima on book page 1161,
      which is PDF page 36. Book page = PDF page + 1125. The chapters are physical splits and no
      state is named in any filename; chapter 9 was found by rendering a running head.
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1930/manufactures-volume-3/03450419v3ch5.pdf
    description: >-
      Fifteenth Census, Manufactures 1929, Volume III — Reports by States. Chapter 5 of seven, 151
      pages, 28 MB. Ohio's section runs from book page 396. Table 2, *Summary for industrial areas
      and for counties and cities: 1929*, gives every Ohio county on book pages 397–398 and every
      city of 10,000 or more on 399; Allen County is on PDF page 46 and Lima on PDF page 47. Book
      page = PDF page + 352.
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1930/manufactures-volume-3/03450419v3ch1.pdf
    description: >-
      The same volume's General Explanations, 12 pages, no text layer. Sections 12 and 13, on book
      page 3, are what make the series above readable: they state which classes of establishment
      were dropped after 1899 and what the minimum size was at each census.
used-by:
  - ../corpus/measure/lima-manufactures-1914-1929.yml
  - ../corpus/measure/allen-county-manufactures-1929.yml
---

**Neither volume has a text layer and every figure here was read off a page image at 300 dpi.**
Each row was added before it was written down. The three ownership columns of the 1919 table sum
to its printed total in every row taken; in the 1929 table the cost of materials and containers
plus the cost of fuel and purchased energy equals the printed total cost, and the value of products
less that total equals the printed value added. Allen County's population column in the 1929 table
prints 69,419, which is the figure this corpus already holds for 1930 from three other volumes.
[verified] — the checks run on every quoted figure, 1 September 2026.

**The scan reverses 0 and 9 and arithmetic is what catches it.** Allen County's value of products
renders as `47,130,403`; the printed value added of 18,544,201 plus the printed cost of 28,595,202
requires 47,139,403, and every other row on the page closes the same identity exactly. Ashland's
cost of materials renders as `6,757,908` where its own two components sum to 6,757,968. [verified]

**Section 13 of the General Explanations is the reason the establishment counts in these two
volumes cannot be differenced.** The quinquennial censuses — 1899, 1904, 1909, 1914 and 1919 —
collected from every establishment with products of $500 or more. The biennial censuses from 1921
on, which is what 1929 is, collected only from those with $5,000 or more. The Bureau states the
effect at the 1921 census: a **21.6 per cent reduction in the number of establishments**, with
99.4 per cent of wage earners and 99.7 per cent of the value of products still reported.
[verified] — the volume's own words.

**Section 12 records an earlier break, before this corpus's first manufacturing figure.** "The
censuses for 1899 and for prior years covered hand trades as well as industries operated on a
factory basis." From 1904 the neighborhood, household and hand industries were dropped, along with
custom tailors, dressmakers, repair shops, building trades, cotton ginneries, custom gristmills and
manufacturing done inside penal and eleemosynary institutions. Any comparison this corpus makes
between 1899 and a later year crosses that change. [verified] — same page.

**What is not here.** No volume between 1929 and the federal employment series that starts in 1969
was found on this host. The economic-census tree at `www2.census.gov/library/publications/economic-census/`
begins at 1963 and holds no manufactures volume; the County Business Patterns datasets begin at
1986. The forty years from 1929 to 1969 remain unmeasured for this county. [verified] — the host's
trees walked, 1 September 2026.
