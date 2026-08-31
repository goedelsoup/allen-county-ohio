---
name: Census 2000, Population and Housing Unit Counts — Ohio (PHC-3-37)
description: >-
  The 2000 counterpart of the 1990 volume this corpus already holds. Its geographic change notes
  are the record of what moved in Allen County between the two censuses — including the county
  line itself — and its Table 5 gives population, housing, total area and land area for every
  township and place in the county at 2000.
type: dataset
obtained: true
retrieved: 2026-08-31
location:
  - kind: url
    value: https://www2.census.gov/library/publications/2003/dec/phc-3-37.pdf
    description: >-
      6.8 MB PDF, 11,818 lines under `pdftotext -layout`. Born digital: the extraction is exact,
      not OCR. The Geographic Change Notes are in the User Notes front matter; the Allen County
      block of the county-subdivision table begins at extracted line 2784.
used-by:
  - ../corpus/jurisdiction/city-of-lima.yml
  - ../corpus/measure/allen-county-annexations-1990-2024.yml
  - ../corpus/measure/allen-county-land-area-2000-2024.yml
  - ../corpus/measure/lima-population-2000.yml
  - ../corpus/place/allen-county.yml
  - ../corpus/place/fort-shawnee.yml
  - ../corpus/place/jackson-township.yml
  - ../corpus/place/marion-township.yml
---

**The county line moved, and this is where it says so.** The Allen County entry opens with a
sentence about the county and not about anything inside it:

> Allen County, Ohio . . . gained territory from Putnam County.

and names the mechanism four lines down — *Marion township . . . gained territory from Jennings
township, Putnam County*. This corpus had treated the county's outline as the one boundary that
does not move. It moved, once, in the 1990s, and by a distance small enough that the county's
total area is the same to a hundredth of a square mile either side of it.

**The whole Allen County block, for the record.** Two of these are township-to-township, which no
other source this corpus holds records at all:

    Allen County                  gained territory from Putnam County
      American township           exchanged territory with Lima city
      Bath township               gained territory from Jackson township
      Beaverdam village           gained territory
      Bluffton village            gained territory
      Delphos city                gained territory
      Elida village               gained territory
      Harrod village              gained territory
      Jackson township            lost territory to Bath township
      Lima city                   exchanged territory with American township; gained
                                  territory from Perry and Shawnee townships
      Marion township             gained territory from Jennings township, Putnam County
      Perry township              lost territory to Lima city
      Shawnee township            lost territory to Lima city
      Spencerville village        gained territory

*Exchanged* is the word that matters in Lima's line. It means the city gave ground back as well as
taking it, which is the first detachment this corpus has held for any date after 1903.

**What it does not carry.** No date for any of it — the notes are a decade's worth of change
collapsed into a verb. For dates the corpus goes to
[the Boundary and Annexation Survey](census-bas-boundary-changes-ohio.md), which starts in the same
decade and overlaps it almost exactly.
