---
name: County and City Data Book, 1967 (U.S. Census Bureau)
description: >-
  A Statistical Abstract supplement printing fifteen items for every county in the country, of
  which three are the presidential vote of 1964 and 1960. It is the only volume of this series the
  corpus could reach that carries an election at all, and it is how two elections entered a
  ninety-year hole in this county's returns.
type: document
obtained: true
retrieved: 2026-09-04
location:
  - kind: url
    value: https://archive.org/download/CountyAndCityDataBook/CountyAndCityDataBook_djvu.txt
    description: >-
      The OCR, 5.3 MB, and unusually good for a statistical table: the numbers come out row by row
      rather than column by column, which is what makes the volume readable at all. See the caution
      below on where that breaks.
  - kind: url
    value: https://archive.org/details/CountyAndCityDataBook
    description: >-
      The item, unrestricted. The scan itself is a 212 MB PDF. Four tables: 1, regions, divisions
      and States, from book page 2; 2, counties, from page 12, with Ohio's eighty-eight beginning
      at page 282; 3, standard metropolitan statistical areas, from page 432; 4, cities, from
      page 464.
used-by:
  - ../corpus/measure/allen-county-presidential-vote-1960-1964.yml
---

**The three items, and what they are not.** Items 13, 14 and 15 are *total vote cast for
President, 1964*, and *percent for the leading party* in 1964 and in 1960 — printed as `R-51.2` or
`D-57.5`, party letter and share in one cell. So the volume says who won a county and by what
share of the whole vote, and never says what the loser got. No margin can be computed from it, and
neither can a two-party share. [verified] — the volume's column headings and its note to items
13–15.

**Its source is not the government.** The note reads: "Source: Governmental Affairs Institute,
*America at the Polls*, University of Pittsburgh Press, 1965. (Copyright.)" Forty-four years later
[the Bureau's last county compendium](census-usa-counties.md) licensed the same table from a
different private publisher and stamped every item of it. [verified] — the note, quoted.

**The cities table has no vote.** Items 201–216, the city list, are population, area and
population characteristics only, so this volume cannot separate Lima from Allen County. [verified]
— the column headings for cities.

**The volume checks itself on this county, because in 1960 the Lima metropolitan area *was* Allen
County.** Table 3 carries `Lima, Ohio` with a 1960 population of 103,691 — the county's own figure
— and the same three election values that Table 2 gives the county. Two tables, one number, and
the second is what confirmed the first through the defect described next. [verified] — Tables 2
and 3, read against
[the county's own 1960 count](../corpus/measure/allen-county-population-by-race-1930-1960.yml).

**The caution: in the county table the last three columns lag a row.** The OCR emits a county's
name and leading columns, then the *previous* county's vote triple. Adams's `8 707 D-57.5` appears
under Allen's code; Allen's `38 887 R-51.2 R-65.5` appears under Ashland's name; Ashland's
`15 801 D-53.7 R-69.7` appears under Ashtabula's. A reader who takes the numbers under a name as
that county's gets the whole state shifted by one. This is why nothing county-by-county is
published here from Table 2 alone, and why only the row Table 3 independently confirms was taken.
[verified] — the county block for Ohio, read line by line.

**Earlier volumes of this series do not have the vote.** The 1949 and 1952 editions were retrieved
and searched: neither prints a presidential item anywhere. [verified] — archive.org items
`countycitydatabo1956unit` and `aah3435.1952.001.umich.edu`, searched in full. The 1962, 1983 and
1994 editions were located and are either access-restricted or carry only a year this corpus holds
better elsewhere; the 1972 and 1977 editions, which would carry 1968 and 1976, are not digitized
anywhere reachable.
