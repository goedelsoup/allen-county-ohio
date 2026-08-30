---
name: 1950 Census of Population, Volume I — Number of Inhabitants, Ohio (Part 35)
description: >-
  The 1950 volume of the same series as the 1960 one, and a better artifact in every way this
  corpus cares about. It has a text layer, its scan is legible, and its Table 6 gives every Ohio
  county's minor civil divisions for 1930, 1940 and 1950 — which for Allen County is twelve
  townships and their villages at three censuses, and the county's 1930 count.
type: dataset
obtained: true
retrieved: 2026-08-30
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1950/population-volume-1/vol-01-38.pdf
    description: >-
      5.3 MB PDF, 58 pages, the Ohio part of Volume I. `pdftotext -layout` works, but the layer is
      itself OCR and its column association across a three-up table is unreliable — the figures
      here were read from rendered page images and the text layer used as the second reader.
      Allen County is on printed page 35-13; Table 4 (cities from earliest census) runs 35-8 to
      35-11; Table 5 (counties, 1930 to 1950) is 35-11.
used-by:
  - ../corpus/measure/allen-county-population-1940-1990.yml
  - ../corpus/measure/allen-county-townships-1930-1950.yml
  - ../corpus/place/amanda-township.yml
  - ../corpus/place/american-township.yml
  - ../corpus/place/auglaize-township.yml
  - ../corpus/place/bath-township.yml
  - ../corpus/place/beaverdam.yml
  - ../corpus/place/jackson-township.yml
  - ../corpus/place/lima.yml
  - ../corpus/place/marion-township.yml
  - ../corpus/place/monroe-township.yml
  - ../corpus/place/perry-township.yml
  - ../corpus/place/richland-township.yml
  - ../corpus/place/shawnee-township.yml
  - ../corpus/place/spencer-township.yml
  - ../corpus/place/sugar-creek-township.yml
  - ../corpus/question/what-became-of-german-and-ottawa-townships.yml
---

**Three arithmetic closures, and every Allen County figure sits inside one.** This is the same
property that made [the 1960 volume](census-1960-number-of-inhabitants-ohio.md) safe to read from a
single scan, arriving here in a different form — not printed differences, but a table that has to
add up:

    1950   the twelve townships sum to 88,183, the county total, with Lima's four
           parts (19,270 + 17,469 + 10,895 + 2,612) summing to 50,246, Lima's own total
    1940   the twelve sum to 28,592, and 28,592 + 44,711 = 73,303, the county total
    1930   the twelve sum to 27,132, and 27,132 + 42,287 = 69,419, the county total

The 1940 and 1930 columns exclude Lima entirely — the table says so in its head, "figures for 1940
and 1930 do not necessarily add to county totals" — and what makes that legible rather than
confusing is that they miss by exactly Lima. The 28,592 of 1940 is also, to the person, a figure
this corpus had already computed by subtraction a phase earlier and published as an inference.

**It confirms the 1960 volume's Lima series, figure for figure.** Table 4 runs Lima from 757 in 1850
to 50,246 in 1950, and every count and every printed increase matches the 1960 volume's Table 5. Two
editions, ten years apart, independently typeset and independently digitized. That is the
[two-scan standard](../decisions/two-scans-of-one-book.yml) met properly rather than substituted
for, and it settles the twenty-person disagreement with
[the 1921 county history](rusler-allen-county-1921.md) at 1920: **41,326**, twice, federally.

**Its Allen County footnote is the answer to a question this corpus has carried since genesis**,
and it is worth quoting whole because every clause of it does work:

> ¹ ALLEN.—Lima city in Bath, Perry, and Shawnee townships returned in 1940 as coextensive with
> nonexistent Ottawa township. Beaverdam village returned in 1940 as Beaver Dam. Part of Shawnee
> township annexed to Lima city in 1944.

Three facts in three sentences: what the Census thought of Ottawa Township in 1952, a name form for
[Beaverdam](../corpus/place/beaverdam.yml) that the 1885 history also uses, and an annexation forty
years earlier than the earliest one this corpus held.

**What it does not carry.** No county figure before 1930 — Table 5 begins there and there is no
counties-from-earliest-census table in this volume — so Allen County in 1910 is untouched by it and
remains the one hole in a series otherwise running 1830 to 2024. Nothing about employment. Its
place-level table stops at 1950, which is why the 1960 volume is still the citation for 1960.
