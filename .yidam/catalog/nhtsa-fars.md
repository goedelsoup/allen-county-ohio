---
name: Fatality Analysis Reporting System (NHTSA)
description: >-
  Every crash on a public road in the United States in which someone died within thirty days,
  since 1975, one record per crash with the county it happened in, the road, the hour, the weather
  and the number killed. It is a census of a kind of death rather than a sample, and it reaches a
  county of a hundred thousand people fifty years deep.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://static.nhtsa.gov/nhtsa/downloads/FARS/2024/National/FARS2024NationalCSV.zip
    description: >-
      The 2024 national file, 32.7 MB zipped. The path is a template —
      `FARS/<year>/National/FARS<year>NationalCSV.zip` — and answers for every year from 1975,
      growing from 4.7 MB in 1975 to 34.2 MB in 2023.
  - kind: url
    value: https://static.nhtsa.gov/nhtsa/downloads/FARS/1975/National/FARS1975NationalCSV.zip
    description: >-
      The first year. Its zip has the tables at the root as `ACCIDENT.CSV`; from about 2010 they sit
      under a `FARS<year>NationalCSV/` folder, and from about 2015 the names are lower case. A
      reader that matches the path rather than the basename finds nothing for half the run.
used-by:
  - ../corpus/measure/allen-county-road-deaths-1975-2024.yml
  - ../corpus/measure/allen-county-rail-crossing-deaths-1980-2005.yml
---

**Fifty archives were opened and none was downloaded whole.** The one table this corpus needs is
`accident.csv`, and it compresses to between 0.7 and 2.6 MB inside archives of 4.7 to 34 MB. Reading
it takes three range requests per year — the tail of the file for the central directory, the
directory entry for the member's offset, then the member — and about 60 MB of transfer for the whole
half-century instead of 700. [verified] — the retrievals here. The server honours `Range` on every
year.

**The API is shut and the files are not.** `crashviewer.nhtsa.dot.gov/CrashAPI` answers a scripted
request with an edge-network *Access Denied* page rather than a rate limit or a challenge, on every
endpoint tried. [verified] The static host serves the same data to the same client without
complaint, which is the pattern
[the API serves the nation and refuses the county](../decisions/the-api-serves-the-nation-and-refuses-the-county.yml)
records in a different form: the obstacle is the delivery mechanism, not the data.

**Two years of the fifty carry a byte-order mark and the rest do not.** In the 2021 and 2022 files
the first column is named `﻿STATE`, so a reader keyed on `STATE` matches nothing, returns an
empty result and reports success. [verified] — the 2020 through 2023 files compared. Allen County
lost 36 fatal crashes and 38 deaths to this before it was found, including the worst year in the
county's record. Decoding as `utf-8-sig` fixes it everywhere and costs nothing on the other
forty-eight.

**The header is not one header.** The accident file runs 45 columns in 1975, 47 from 1982, 48 from
1987, 49 from 1991, 51 from 1999, 55 from 2007, back to **47 in 2010**, then 89 in 2015 when every
coded field gained a `…NAME` twin, 91, and 80 from 2021. [verified] — the fifty files. Individual
fields come and go with it:

      DRUNK_DR      1975-2020   count of drinking drivers; gone from this file afterwards
      VE_TOTAL      2005-2024   vehicles; VE_FORMS is the field that runs the whole way
      PEDS          1991-2024   pedestrians and cyclists involved
      PERNOTMVIT    2011-2024   persons not in a motor vehicle in transport
      RUR_URB       2015-2024   rural or urban
      FUNC_SYS      2015-2024   functional class of the road
      LATITUDE      1999-2000, 2008-2024

**`LATITUDE` is a two-year island and then a nine-year hole.** It appears in 1999 and 2000, vanishes
for seven years and returns in 2008. [verified] — the same files. A map of this county's fatal
crashes can be drawn for 1999, 2000 and 2008 onward and for nothing between.

**`RAIL` is filled with asterisks for four years and zeroes thereafter.** Every one of the 84 Allen
County crashes in 1975 through 1978 carries `RAIL = *******`; from 1979 the field is `0000000` where
no railroad was involved and a seven-character USDOT grade-crossing number where one was.
[verified] — the Allen County rows of those years. Treating anything non-zero as a rail crossing
turns four years of ordinary crashes into 84 phantom ones, and the corpus's rail-crossing series
therefore begins in 1979 and not in 1975. See
[a zero is not a blank](../decisions/a-zero-is-not-a-blank.yml).

**The crossing number in `RAIL` is the same key the FRA inventory uses.** Nineteen of the twenty
Allen County crossings that appear in FARS resolve against
[the grade crossing inventory](fra-crossing-inventory.md) on `crossingid`, which supplies the street,
the town, the railroad and the warning devices that FARS does not carry. The twentieth, `53278X` in
2005, is six characters where the key is seven and matches nothing. [verified] — the join, run here.
One 1997 and one 2001 record spell the same crossing `532715R` and `532715r`.

**`TWAY_ID` is free text and the same road has three spellings.** Interstate 75 appears as `I-75`,
`I75` and `75` in the same column in different years; state routes appear as `SR-117` and as `117`
with the class carried only in the separate `ROUTE` code. [verified] — the Allen County rows. Any
count by road name has to normalise first, and the `ROUTE` code is the reliable one.

**A code's label can change without the code changing.** Within 2015–2024 alone, `ROUTE` value 4 is
printed as `County Road` in some years and `County` in others, and 2, 5 and 6 do the same.
[verified] — the Ohio rows of those years. Grouping by the `…NAME` twin rather than the code splits
one class in two.

**The control closes against the publisher.** Summing `FATALS` over every row of the 2024 file gives
39,254 deaths in 36,297 crashes nationally, which is the figure NHTSA publishes for that year.
[verified] — the same file.

**What it is and is not.** It is a census of deaths, so it carries no sampling margin — and a county
of a hundred thousand people supplies about a dozen a year, which is a small enough count that
chance alone moves it by half. See
[a count of tens is a draw](../decisions/a-count-of-tens-is-a-draw.yml). It counts deaths by **where
the crash happened**, not by where the dead lived, which is the opposite convention from the
death-certificate files behind [County Health Rankings](county-health-rankings.md).

**What is in it and unread here.** The vehicle, person, drug and impairment tables that join to this
one on `ST_CASE`, which carry age, sex, restraint use, licence status and blood alcohol; the crash
event sequence; and the two-year window of coordinates that would put the county's fatal crashes on
a map.
