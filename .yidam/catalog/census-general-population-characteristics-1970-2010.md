---
name: Decennial general population characteristics, Ohio, 1970–2010
description: >-
  Five state volumes, one per census, each of which prints the population of every Ohio county by
  race. Together with the 1930–1960 volumes and the 2020 redistricting file they close the county's
  race series from 1920 to 2020 with no gap, and they carry the moment the series stops being one
  series: 2000 is the first of them at which a person could be counted as more than one race.
type: dataset
obtained: true
retrieved: 2026-09-01
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1970/population-volume-1/1970a_oh1-03.pdf
    description: >-
      1970, PC(1)-B37, section 1 part 3 of nine. **Table 34, Race by Sex, for Counties: 1970**, PDF
      page 55 = book page 37-249. No text layer in any of the nine parts; every figure was read from
      a 300-dpi render. Lima is over 50,000 and so is not in the county-subdivision tables: its race
      row is in **Table 23, Race by Sex, for Areas and Places**, at part 2 PDF page 27 = book page
      37-96, under the Lima SMSA as its central city.
  - kind: url
    value: https://archive.org/download/1980censusofpop80137un/1980censusofpop80137un_bw.pdf
    description: >-
      1980, PC80-1-B37. 28 MB, 488 pages. **Table 44, General Characteristics for Counties and
      County Subdivisions: 1980**, PDF page 330 = book page 37-317, which is the one table in this
      set that gives race for the county and all thirteen of its subdivisions on the same row of
      the same page. Its race panel is White, Black and Spanish origin only. Census.gov serves the
      same volume split into eighteen files under `decennial/1980/volume-1/ohio/`.
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1990/cp-1/cp-1-37.pdf
    description: >-
      1990, CP-1-37. 12 MB, 698 pages, with a text layer: `pdftotext -layout` renders it exactly.
      **Table 76, General Characteristics of Persons, Households, and Families: 1990**, extracted
      line 62699, which is the Allen County row.
  - kind: url
    value: https://www2.census.gov/library/publications/2002/dec/phc-1-37.pdf
    description: >-
      2000, PHC-1-37. 8.6 MB, 570 pages, born digital. **Table 3, Race and Hispanic or Latino:
      2000**, whose Allen County row is extracted line 10736.
  - kind: url
    value: https://www2.census.gov/library/publications/2012/dec/cph-1-37.pdf
    description: >-
      2010, CPH-1-37. 31 MB, 613 pages, born digital. **Table 3, Race and Hispanic or Latino Origin:
      2010**, whose Allen County row is extracted line 12955.
used-by:
  - ../corpus/measure/allen-county-population-by-race-1970-2020.yml
  - ../corpus/measure/allen-county-households-1970-2023.yml
---

**What it is.** The general-characteristics volume is the second book of every decennial census —
the one published from the complete count rather than the sample, a state at a time, with a table
for counties and a table for the subdivisions inside them. This corpus already held the 1930 to
1960 members of the same family under
[a different entry](census-population-volumes-1930-1960-ohio.md); these five are the ones after
them, and the 2020 census publishes no successor as a printed volume at all. Its equivalent is
[the redistricting file](census-2020-redistricting-file.md).

**The race question changes twice inside these five books and the column headings say so.**

    1970    White · Negro · Other races (Indian, Japanese, Chinese, Filipino, all other)
    1980    White · Black · Spanish origin, the last overlapping the first two
    1990    White · Black · American Indian, Eskimo, or Aleut · Asian or Pacific Islander
    2000    the same five, split "one race" against "two or more races", the second new
    2010    the same, with Native Hawaiian separated from Asian

The 1980 volume's Table 44 prints no residual: White plus Black is 960 short of Allen County's
total and the book does not say what the 960 are. The 1990 volume prints four races and no
residual either — its four sum to 491 short of the county. The 2000 and 2010 volumes print every
category and close exactly. A reader taking "White plus Black" for "the whole county" out of the
first two books and out of the last two is taking two different things.

**Where the checks were made.** Every figure this corpus takes from these volumes was added before
it was written down, and each census closed against something it did not have to:

- 1970 — Allen's four sex rows and its race rows both sum to the printed 111,144, and its five
  named other races sum to the printed 271.
- 1980 — the thirteen county subdivisions sum to the county in all three printed columns at once:
  112,241 total, 100,306 White, 10,975 Black.
- 1990 — the same thirteen sum to 109,755, 96,177 and 12,313.
- 2000 and 2010 — the seven race categories sum to the printed county total in both books.
- and 1970's county total, 111,144, is the figure this corpus already held from
  [the 1970s estimates file](census-county-estimates-1970s.md) and from
  [the 1990 historical volume](census-cph-2-37-ohio.md), read from neither of these books.

**Two figures in these books disagree with figures this corpus holds, and both disagreements are
already on the record.** The 1980 volume prints Lima at 47,381 where
[the 1990 historical volume](census-cph-2-37-ohio.md) restates it as 47,827 with a revision mark,
and the 2000 volume prints Lima at 40,081 where the intercensal base is 40,307 and
[the 2010 historical volume](census-cph-2-37-ohio-2010.md) restates it as 41,581. The race figures
here are each book's own and are consistent inside their own book; they are not consistent with the
totals a later book assigns to the same city on the same day. See
[two volumes are not one series](../decisions/two-volumes-are-not-one-series.yml).

**A note on the 1970 volume.** It is nine PDFs with no text in any of them, and the tables are not
where the table numbers suggest. Chapter B runs across parts 2 and 3; the counties come after the
metropolitan areas and after the places of 2,500 to 10,000, so **Table 34 sits at the back of part
3 and Table 23 in the middle of part 2**. Both were found by rendering the top eighth of every
fourth page into a single strip and reading the table titles off it, which is cheaper than reading
866 pages and is written down here so the next reader does not repeat the search.

**What none of them carries.** No county figure before 1970 — that is the earlier entry's work —
and no Black population for Lima at 1960, which is a gap in the 1960 volume's reading rather than
in these books. It is the one gap left in the city's race series between 1920 and 2020.

**Three of the five volumes carry a households table as well as a race table, and the corpus had
read only the race table.** 1990 prints **Table 2, Summary of General Characteristics of Households
and Families: 1990**, whose county panel is at extracted line 2155; 2000 prints **Table 7,
Households and Families: 2000** beginning at line 25931; 2010 prints **Table 7, Households and
Families: 2010**, whose Allen County row is line 31272. [verified] — `pdftotext -layout` over the
three files. Each gives total households, family households, married-couple families, female
householder with no husband present, nonfamily households, householders living alone, householders
living alone aged 65 and over, and average household and family size — and 2000 and 2010 give all
of it for every county subdivision and place as well.

**The 1990 volume prints shares where the later two print counts.** Its columns after the household
total are percentages of all households to one decimal: Allen County reads 73.5, 58.9, 11.7, 26.5,
23.6, 11.3 and then 2.66 and 3.16 for the two average sizes. Multiplying them back out recovers the
counts to within a household, and this corpus takes the counts from
[USA Counties](census-usa-counties.md) instead and uses this volume as the check. [verified] — the
two read against each other.

**The 1980 volume's household columns were not taken and the reason is the scan.** Table 44 of
PC80-1-B37 gives households for the county and all thirteen subdivisions, but the archive.org OCR
of that page renders `112 241` as `112 24)` and drops a leading digit from at least one township
row. The county's 1980 household count in this corpus comes from the compendium, which is born
digital. [verified] — the extracted text, read here.
