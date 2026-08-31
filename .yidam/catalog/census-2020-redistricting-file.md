---
name: 2020 Census Redistricting Data Summary File (P.L. 94-171), Ohio
description: >-
  The block-level file every state gets for redistricting, and the only free source of the 2020
  census's group-quarters and occupancy tables at block grain. It answered two questions this
  corpus had left open and named the source for: what holds 1,360 people in a Lima block with no
  housing units, and how many of the county's 44,563 housing units are lived in.
type: dataset
obtained: true
retrieved: 2026-08-31
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/decennial/2020/data/01-Redistricting_File--PL_94-171/Ohio/oh2020.pl.zip
    description: >-
      49 MB zip, 484 MB unpacked, four pipe-delimited files with no header row. `ohgeo2020.pl` is
      the geographic header, 97 fields, one record per summary level; the three numbered files
      carry the tables. Join on LOGRECNO, which is field 8 of the geographic header and field 5 of
      every data file.
  - kind: url
    value: https://www2.census.gov/programs-surveys/decennial/2020/technical-documentation/complete-tech-docs/summary-file/2020Census_PL94_171Redistricting_NationalTechDoc.pdf
    description: >-
      The national technical documentation, 1.2 MB. Chapter 6 is the data dictionary that gives
      each table its file segment; Appendix B defines every group-quarters type by code.
used-by:
  - ../corpus/measure/allen-county-group-quarters-2020.yml
  - ../corpus/measure/allen-county-housing-units-2020.yml
  - ../corpus/measure/allen-county-occupancy-2020.yml
  - ../corpus/place/allen-county.yml
  - ../corpus/place/lima.yml
  - ../corpus/question/who-lives-in-the-county-without-housing.yml
  - ../corpus/site/allen-correctional-institution.yml
  - ../corpus/site/allen-county-courthouse.yml
  - ../corpus/site/allen-county-justice-center.yml
  - ../corpus/site/lima-state-hospital.yml
  - ../corpus/site/oakwood-correctional-facility.yml
---

**What it is.** The tabulation the Census Bureau is required by 13 U.S.C. 141(c) to deliver to
every state within a year of census day, so that legislatures can redistrict. It is small — six
tables — and it is published at every geography down to the block, which is what makes it worth
more here than files ten times its size.

    P1   race                                    file 01
    P2   Hispanic or Latino origin by race       file 01
    P3   race, 18 and over                       file 02
    P4   Hispanic origin by race, 18 and over    file 02
    H1   occupancy status                        file 02, last three fields
    P5   group quarters by major type            file 03

**Why the flat file and not the API.** The same tables are served from the Demographic and Housing
Characteristics file at `api.census.gov`, which answers a keyless request with `302` to
`missing_key.html` and an `X-DataWebAPI-KeyError` header — no error text, no data. The flat file
needs no key, and this is the second time this corpus has taken that route; the
[ACS summary file](census-acs-summary-file.md) was the first. A gate on a public API is a reason to
find the file the API is built over, not a reason to stop.

**Two field offsets worth writing down.** Both cost a wrong answer before they were found. In the
geographic header, **LOGRECNO is field 8 and GEOID is field 9** — keying on field 5, which is what
the data files use, silently returns almost nothing. And **field 18 is COUSUB, not PLACE**; PLACE is
field 30. For Lima the two are identical, because Lima is the only place in this county that the
Census treats as its own county subdivision, so the mistake is invisible on exactly the row most
likely to be checked.

**P5 is the group-quarters table, and it closes on itself.** Ten cells: a total, an institutional
and a noninstitutional subtotal, and four and three types under them. Ohio's own record —
299,628 total, of which 67,080 in correctional facilities and 111,646 in college housing — has both
subtotals summing exactly from their parts and the total from the two subtotals. Allen County's
3,552 block records sum to the file's own county record in all ten cells with no residual.

**Appendix B is where the categories get their meaning**, and one of them is not what its name
suggests. "Other noninstitutional facilities" (codes 701, 702, 704, 706, 801, 802, 900, 901, 902,
903, 904) is the census's category for **emergency and transitional shelters for people
experiencing homelessness**, soup kitchens and targeted outdoor locations, group homes intended for
adults, residential treatment centres, workers' group living quarters and Job Corps centres,
maritime vessels, and quarters for victims of natural disasters. A county reading its own group
quarters table without Appendix B would not know it had been given a count of its shelter
population.

**Its figures carry disclosure avoidance.** Every 2020 block figure is the output of the Bureau's
differential-privacy system rather than a raw count, and small cells move. County totals are held
invariant; block figures are not. Where this corpus quotes a block figure it says so, and the two
it leans on hardest are corroborated by the geometry the block sits on. See
[a block can be a fence](../decisions/a-block-can-be-a-fence.yml).

**What else is in it, unread.** The race and Hispanic-origin tables at block level, for 2020 and
for the population 18 and over — which is voting-age population by block, the input every
redistricting argument in this county is actually made from, and which this corpus has at tract
level from the American Community Survey and not at block level from a count.
