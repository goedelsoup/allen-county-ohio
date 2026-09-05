---
name: USA Counties (U.S. Census Bureau, 2011 edition)
description: >-
  The Census Bureau's discontinued county compendium: 6,659 items for every county in the country,
  distributed as zipped spreadsheets by subject. It is taken here for one of its fifty-two
  subject tables — the presidential vote, 1980 through 2008 — and that table turns out to be the only one
  in the whole file the federal government did not collect.
type: dataset
obtained: true
retrieved: 2026-09-04
location:
  - kind: url
    value: https://www2.census.gov/library/publications/2011/compendia/usa-counties/zip/ELE.zip
    description: >-
      3.4 MB zipped, one file inside: `ELE01.xls`, 14.5 MB, BIFF format, **nine worksheets**. Each
      sheet carries `Areaname`, `STCOU` and ten items in four columns apiece — flag, data,
      footnote 1, footnote 2 — for 3,198 areas: the United States, the fifty states and the
      District, and 3,146 counties. A converter that reads only the first sheet returns ten of the
      eighty-four items and looks complete.
  - kind: url
    value: https://www2.census.gov/library/publications/2011/compendia/usa-counties/zip/Ref.zip
    description: >-
      The dictionary, and the reason this entry exists. `Mastdata.xls` gives every item's code,
      description, unit, decimal places, national total **and source**; `Source.xls` expands the
      thirty-two source codes. Without it `ELE010180D` is unreadable and the copyright note below
      is invisible.
  - kind: url
    value: https://www2.census.gov/library/publications/2011/compendia/usa-counties/zip/
    description: >-
      The directory listing: fifty-three files, fifty-two subject tables and the dictionary.
      Sibling directories `excel/`, `moe/` and `img/` hold the same data unzipped, the American
      Community Survey margins, and the maps.
used-by:
  - ../corpus/measure/allen-county-presidential-vote-1980-1996.yml
  - ../corpus/measure/allen-county-against-ohio-1980-2008.yml
  - ../corpus/measure/allen-county-presidential-vote-2000-2016.yml
  - ../corpus/measure/allen-county-households-1970-2023.yml
---

**Eighty-four items of six thousand six hundred and fifty-nine carry a copyright notice, and they
are all the same table.** `Mastdata.xls` writes `***Subject to copyright***` into the description
of every election item and of nothing else — `ELE010180D`, "Vote cast for president - total 1980",
through `ELE220208D`, the leading-party code for 2008. Their source field reads `CQ Press`, which
`Source.xls` expands to *SAGE Publications*. [verified] — the two reference tables, counted here.

**Thirty of the thirty-two sources are federal and the vote is not one of them.** The list runs
Census Bureau divisions, the Bureau of Economic Analysis, the FBI, the FDIC, Social Security, the
IRS, the USGS, the agriculture and education statistics agencies. Two are not government: the
American Medical Association, which supplies twenty-one items about physicians, and CQ Press,
which supplies the eighty-four about the vote. Only the second is stamped. [verified] — same file.

**How to read an item code.** `ELE` names the table, the next three digits the measure, the three
after that the **year less 1800**, and `D` the data column beside `F`, `N1` and `N2`. So 1980 is
`180`, 2000 is `200`, 2008 is `208`, and the item that looks like it belongs to a different family
because its prefix reads `ELE0102` rather than `ELE0101` is the same measure four years later.
[verified] — read off the header rows against `Mastdata.xls`.

**The measures held.** Total vote, Democratic, Republican and other, each as a count and as a
percent; the percent for the leading party and a leading-party code, 1 for Democratic and 2 for
Republican; and, for the three elections where it mattered, the third-party line named — Perot in
1992 and 1996, Nader in 2000 — with a residual "except" column beside it.

**What is not here.** Nothing before 1980 and nothing after 2008: the compendium was discontinued.
The county grain stops at the county, so there is no city, township or precinct in this file, and
nothing that would let a reader see Lima apart from Allen County.

**The households table is 188 items and it runs from 1970, which is earlier than anything else
here.** `HSD.zip` opens to two workbooks of nine worksheets each — eighteen, not two, and the trap
that hid seventy-four election items in this same compendium is the same trap. It carries, for
every county in the country: households, family households, married-couple families, male- and
female-householder families with no spouse present, nonfamily households, one-person households,
persons in households and persons per household, at each census from 1970 or 1980 through 2010,
with a parallel set from the 1980, 1990 and 2000 long forms and from the 2005–2009 survey.
[verified] — `Mastdata.xls` filtered to the `HSD` prefix, and the two workbooks read here.

**One item in it is published in thousands and nothing in the data says so.** `HSD110170D`, family
households in 1970, has unit code `THO` and one decimal place: Allen County's cell reads `27.5` and
means 27,500. Every other item in the table is `ABS` and reads as a count. A reader who takes the
`Mastdata` unit column as decoration will publish a county with twenty-eight family households.
[verified] — the unit reference against the cell.

**The same census year is published twice, from two universes, and the two do not agree.** Every
year from 1980 to 2000 appears once as *complete count* and once as *sample* — the short form
against the long form. Allen County's households in 1980 are 39,168 and 39,212; in 1990, 39,408 and
39,303; in 2000, 40,646 and 40,625. The largest gap is 105 households, which is 0.27 per cent, and
the sign is not consistent. [verified] — items `HSD010180D` and `HSD410180D` and their siblings.
The complete count is the one to use for a series; the sample is the one that carries the detail
the short form never asked for.

**It copies its sources exactly, checked on four figures at once.** Allen County's 2000 households,
family households, married-couple families and one-person households — 40,646, 28,213, 21,562 and
10,696 — are the four printed in Table 7 of
[the 2000 Ohio volume](census-general-population-characteristics-1970-2010.md), digit for digit,
and the 2010 set matches the same table of the 2010 volume. The 1990 volume prints percentages
rather than counts and those agree to the tenth. [verified] — the three volumes against this file.
Two of the compendium's decades can be verified against print, and the 1970 row cannot: nothing
else in this corpus carries it. See
[a single witness read once](../decisions/a-single-witness-read-once.yml).
