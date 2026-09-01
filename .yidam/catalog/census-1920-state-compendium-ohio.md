---
name: State Compendium, Ohio (Fourteenth Census, 1920)
description: >-
  The 1920 census's Ohio volume, and the direct successor of the 1910 supplement this corpus already
  reads. It carries county tables the abstracts do not: composition of the population by colour,
  race, nativity and sex for every Ohio county, country of birth of the foreign-born white for every
  county, and the same for every city of 10,000 or more.
type: publication
obtained: true
retrieved: 2026-09-01
ttl_days: 3650
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1920/state-compendium/06229686v32-37ch3.pdf
    description: >-
      31 MB, 148 pages, the Ohio chapter of the volume covering state compendia 32 to 37. Chapter 2
      of the same file group is North Dakota; the chapters are the states in order and nothing in the
      filename says so. Table 9, "Composition and characteristics of the population, for counties",
      begins on page 42, and the PDF page is the book page. Table 12, country of birth of the
      foreign-born white, runs from page 59 and takes three pages because the countries are split
      into two blocks that each restart at Adams.
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1920/abstract/abstract-1920-1-population.pdf
    description: >-
      The 1920 abstract's population chapter, 469 pages. Its Table 34 gives colour or race, nativity
      and parentage for cities of 25,000 to 100,000, which is where Lima is. The abstract has no
      county tables at all, which is why the compendium is the volume that matters here.
used-by:
  - ../corpus/measure/allen-county-population-by-race-1920.yml
  - ../corpus/measure/allen-county-foreign-born-1920.yml
---

**The text layer is good enough to find a page and not good enough to read one.** `pdftotext` finds
"Lima" on the right page of the abstract and renders its row as
`4l, 320 40,0li2 OU. \l 32,572 78.8`. Every figure here was read from an image rendered at 300 dpi,
and the compendium's own arithmetic is the check: 64,064 native white plus 2,753 foreign-born white
plus 1,385 Negro plus 21 others is 68,223, which is the total it prints for Allen County, and the
twenty-two country-of-birth columns sum to 2,753, which is the foreign-born total it prints.
[verified] — both sums computed on the transcribed figures.

**It disagrees with a county history this corpus has been trusting for a census count.** The
compendium gives Allen County 68,223 in 1920. The corpus has carried 68,203 in seven places, and
that figure came from
[A Standard History of Allen County, 1921](rusler-allen-county-1921.md) — a book printed before the
census was published. The 1950 volume of Number of Inhabitants, which this corpus also holds, gives
the county for 1930, 1940 and 1950 and does not print 1920, so nothing federal had ever stood behind
the number. [verified] — that volume's Table 6, read again for this.

**What it does not have.** No county table by colour for 1910, so this volume anchors one census and
not a series; the same table exists in the 1930 and 1940 volumes and neither has been read. The
whole century between this count and the American Community Survey is still unmeasured for the
county's Black population, and this is now one end of it rather than a claim standing alone.
