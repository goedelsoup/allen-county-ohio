---
name: 2010 Census Summary File 1
description: >-
  The same tables the 2020 characteristics file publishes, taken ten years earlier by an instrument
  that added no noise to any of them. It is what makes a decade out of a census — and the only file
  in this corpus whose geographic header is fixed-width and whose data are comma-separated.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 1825
location:
  - kind: url
    value: https://www2.census.gov/census_2010/04-Summary_File_1/Ohio/oh2010.sf1.zip
    description: >-
      Ohio's copy: 408,847,426 bytes compressed, 4.14 GB open, a geographic header and
      forty-seven data segments. The housing tables H3 occupancy, H4 tenure, H5 vacancy status and
      H14 tenure by race of householder are all in segment 44, at data fields 1, 4, 8 and 79.
  - kind: url
    value: https://www2.census.gov/census_2010/04-Summary_File_1/Ohio/oh2010.sf1.zip#oh2010.sf1.prd.packinglist.txt
    description: >-
      The packing list inside the archive, which is the layout key: one line per table, giving the
      segment and the number of cells, in file order. Accumulating the cell counts within a segment
      gives every table's offset without opening the technical document.
  - kind: url
    value: https://www2.census.gov/programs-surveys/decennial/2010/technical-documentation/complete-tech-docs/summary-file/sf1.pdf
    description: >-
      The technical documentation, 6-1 through 7-9 of which carry the table shells, the geographic
      header layout and the chapter on how the 2010 census protected what it published.
used-by:
  - ../corpus/measure/allen-county-vacancy-status-2010-2020.yml
  - ../corpus/measure/allen-county-tenure-2010-2020.yml
---

**Its geographic header is fixed-width and its data are comma-separated, in the same archive.**
`SUMLEV` at characters 9–11, `LOGRECNO` at 19–25, `COUNTY` at 30–32, `NAME` at 227–316, `POP100` at
319–327 and `HU100` at 328–336; the data segments are ordinary CSV whose first five fields repeat
the file identification and the record number. [verified] — the archive, read here. The 2020 file
replaced both with pipes, so a reader written for one does not read the other.

**No number in it was moved by an algorithm; some households were.** The 2010 census protected its
published tables by data swapping — "a sample of households is selected and matched on a set of
selected key variables with households in neighboring geographic areas … Because the swap often
occurs within a geographic area with a small population, there is no effect on the marginal totals."
[verified] — the technical documentation, chapter 7. That is a different instrument from 2020's
noise infusion, and a 2010 figure and a 2020 figure are not the same kind of number; see
[the total is enumerated and the split is not](../decisions/the-total-is-enumerated-and-the-split-is-not.yml).

**Its race categories are not 2020's.** Households with a householder of two or more races number
517 in this county in 2010 and 1,567 in 2020, and the county's Black-alone population moved by 66
people over the same decade. [verified] — this file and
[the 2020 characteristics file](census-2020-dhc.md), against
[the race series](../corpus/measure/allen-county-population-by-race-1970-2020.yml). The 2020 write-in
coding is the reason and the corpus has established it once already; a rate computed inside a
single census year survives it and a decade change in the White-alone or multiple-race rows does
not.

**Two geographies in this county changed between the files and neither is flagged.** Fort Shawnee
is a village in 2010 with 1,605 housing units and a census designated place in 2020 with 2,736;
Gomer and Westminster have no 2010 place record at all. [verified] — the two archives, on place
code. The thirteen county subdivisions are stable across both and are what this corpus compares.
