---
name: Census of Population, state volumes for Ohio, 1930 to 1960
description: >-
  The Ohio part of the printed population volume at four consecutive censuses. Each carries the
  county race and nativity tables that the "Number of Inhabitants" volumes this corpus already
  holds do not, and together they fill the forty years between the 1920 compendium and the
  county's modern counts.
type: publication
obtained: true
retrieved: 2026-09-01
ttl_days: 3650
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1930/population-volume-3/10612982v3p2ch05.pdf
    description: >-
      1930, Volume III Part 2 (Montana–Wyoming), chapter 5 of eleven, 150 pages, 35 MB. Ohio's
      section runs from about book page 470. Table 13, *Composition of the population, by counties*,
      opens on book page 479, which is PDF page 25; the counties are columns and Allen is the third,
      after the state and Adams. The table prints 1930 and repeats 1920 beneath it. Book page =
      PDF page + 454. Chapters are physical splits, not states: chapter 5 was found by rendering a
      running head.
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1940/population-volume-2/33973538v2p5ch6.pdf
    description: >-
      1940, Volume II Part 5 (Nevada–Oregon), chapter 6 of nine, 151 pages, 46 MB. Table 28,
      *Race and age, by sex, with rural-farm population, for minor civil divisions, by counties*,
      opens on book page 618, which is PDF page 80; Allen County's block is on the same page, and
      its townships and Lima follow. Book page = PDF page + 538.
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1950/population-volume-2/37783896v2p35ch3.pdf
    description: >-
      1950, Volume II Part 35 — Ohio, chapter 3 of five, 147 pages, 18 MB. Table 34 (PDF page 48)
      carries Lima among the urban places of 10,000 or more; Table 42 (PDF page 108) carries the
      counties, Allen second; Table 42a (PDF page 113) gives country of birth by county, Allen
      second. This is the only one of the four with a usable text layer, and it is OCR: the row
      stubs and the column heads are legible and the digits are not.
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1960/population-volume-1/37749282v1p37.zip
    description: >-
      1960, Volume I Part 37 — Ohio, the whole state part as a 187 MB zip of ten chapter PDFs.
      Section 3, *General Population Characteristics*, is `37749282v1p37_ch03.pdf`, 169 pages.
      Table 25 (PDF page 87) gives the twelve townships by race; Table 28 (PDF page 145) gives the
      counties, Allen second. Book page = PDF page + 43. The chapter files are named with an
      underscore, which is why the pattern that works for every other state's part returns 404 here;
      the standalone `37749282v1p37.pdf` at the same path is a nine-page cover.
used-by:
  - ../corpus/measure/allen-county-population-by-race-1930-1960.yml
  - ../corpus/measure/allen-county-foreign-born-1930-1950.yml
---

**Three of the four have no text layer at all and every figure here was read off a page image.**
`pdftotext` returns nothing for the 1930, 1940 and 1960 chapters; the 1950 chapter returns an OCR
layer that renders Adams County's 1950 population as `-33,040` and its under-5 count as `2,l?b`.
Pages were rendered with `pdftoppm` at 300 dpi, cropped to the columns wanted, and read.
[verified] — both routes exercised on all four volumes, 1 September 2026.

**Every figure taken from these volumes closes an arithmetic identity before it is written down.**
The race lines of a county sum to that county's printed total in all four; the twelve townships of
Allen County sum to the county in 1940 and 1960; the twenty-eight country-of-birth columns of 1950
sum to the printed 1,485. Four county totals — 69,419, 73,303, 88,183 and 103,691 — were read
before being compared with the figures this corpus already held from
[the 1950 Number of Inhabitants volume](census-1950-number-of-inhabitants-ohio.md), and all four
agree to the person. [verified] — the checks run on every quoted figure.

**The 1930 volume is a second witness for 1920 and it settles a correction this corpus made a phase
earlier.** Its Table 13 prints the 1920 county figures beneath the 1930 ones: total 68,223, native
white 64,064, foreign-born white 2,753, Negro 1,385. Those are the four figures the corpus took
from [the 1920 compendium](census-1920-state-compendium-ohio.md), including the total that replaced
the 68,203 the 1921 county history had supplied. [verified]

**Not every category survives from one volume to the next.** The 1920 compendium names twenty-one
countries of birth; the 1950 table names twenty-seven and Switzerland is not among them, and Wales
is folded into England. 1930 and 1940 print four race lines, 1950 prints four, 1960 prints seven.
The county's minor civil divisions are reported without Lima in 1930 and 1940 and with it in 1950
and 1960. Nothing here can be differenced without checking the column heading first. [verified]

**What is not here.** The census.gov path for the 1950 and 1940 population volumes rejects a
directory listing outright; the file identifiers were recovered from the library catalogue pages at
`census.gov/library/publications/1953/dec/population-vol-02.html` and `.../1943/dec/population-vol-2.html`.
No volume for 1970 or later is used here. [verified] — 1 September 2026.
