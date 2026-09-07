---
name: SPC severe weather database — tornadoes
description: >-
  The Storm Prediction Center's national tornado file, 1950 to the present, one row per tornado
  rather than one row per county. It carries the same events as the Storm Events Database and
  arranges them the other way round: a track that crosses a county line is one record here and two
  there, and the counties it crossed are four columns on that record rather than four rows. It is
  the file to ask what a whole storm did, and the only one that says in its own documentation that
  its old damage column is a category.
type: dataset
obtained: true
retrieved: 2026-09-06
ttl_days: 365
location:
  - kind: url
    value: https://www.spc.noaa.gov/wcm/data/1950-2025_actual_tornadoes.csv
    description: >-
      The whole file, 73,458 rows and 8.6 MB, no key and no query interface. `om` is the tornado's
      number within its state and year; `st`, `stf` and `stn` give the state; `mag`, `inj`, `fat`,
      `loss`, `slat`, `slon`, `elat`, `elon`, `len` and `wid` describe the tornado; `f1` through
      `f4` are the county FIPS codes it crossed. There is a companion `1950-2025_all_tornadoes.csv`
      that includes rows the SPC does not count.
  - kind: url
    value: https://www.spc.noaa.gov/wcm/data/SPC_severe_database_description.pdf
    description: >-
      The field description, four pages, image-only. It is the document that defines the `ns`, `sn`
      and `sg` triple and the pre-1996 damage categories, and both are quoted below.
used-by:
  - ../corpus/event/the-tornado-of-11-april-1965.yml
  - ../corpus/event/the-tornado-of-19-july-1950.yml
  - ../corpus/measure/allen-county-tornadoes-1950-2026.yml
  - ../corpus/measure/ohio-tornadoes-of-11-april-1965.yml
---

**A row is a tornado, and three columns say which kind of row it is.** The description reads:
"22- ns=Number of States affected by this tornado: 1, 2, or 3. / 23- sn=State Number: 1 or 0
(1=entire track info in this state). / 24- sg=Tornado SeGment number: 1, 2, or -9 (1=entire track
info)", and then enumerates the combinations — "1,1,1 = Entire record for the track of the tornado
(unless all 4 fips codes are non-zero)", "2,0,1 = A two-state tornado (st=state of touchdown, other
fields summarize entire track)", "1,0,-9 = Continuing county fips code information only from 1,1,1
record, above (same om)". It also warns what happens if you ignore them: "When summing for state
total use sn=1, not sg=1."

**Which makes it the file that can tell a county's share from a storm.** The Palm Sunday tornado
that crossed Allen County is one row here — `om` 166, ns/sn/sg all 1, so by the file's own rule the
entire record for the track — reading F4, 32.5 miles, 400 yards, 13 killed, 104 injured, counties
`f1`=3 and `f2`=63. The Storm Events Database holds the same tornado as an Allen County row of 17.8
miles and eleven dead and a Hancock County row of 14.7 miles and two, and says nowhere that they are
one storm. See [a county row is a share of a storm](../decisions/a-county-row-is-a-share-of-a-storm.yml).

**Its damage column is a category and says so.** "Estimated property loss information - Prior to
1996 this is a categorization of tornado damage by dollar amount (0 or blank-unknown; 1<$50,
2=$50-$500, 3=$500-$5,000, 4=$5,000-$50,000; 5=$50,000-$500,000, 6=$500,000-$5,000,000,
7=$5,000,000-$50,000,000, 8=$50,000,000-$500,000,000, 9=$5000,000,000.) ... From 1996, this is
tornado property damage in millions of dollars." **This file keeps the category**: the Palm Sunday
tornado's `loss` is `6.0`, the band the 1965 publication printed. The Storm Events Database converts
the same category to `2.5M` and leaves no mark that it was ever a band. See
[a damage figure is a category](../decisions/a-damage-figure-is-a-category.yml) and
[the publication](noaa-storm-data-publication.md), whose own footnote prints the same nine steps.

**It disagrees with the Storm Events Database about which state a tornado belongs to.** The twelve
Ohio tornadoes the publication itemises for 11–12 April 1965 appear here as eleven records with
`st` = OH; the twelfth, which crossed from Berne, Indiana and ended in Van Wert County, is `om` 157
with `st` = IN — a `2,0,1` row, so by the file's rule its 52.5 miles and four deaths summarise both
states and belong to neither alone. The Storm Events Database keeps that tornado in Ohio, as a
Mercer County row and a Van Wert County row. **The two files then agree on Ohio's death toll by
accident**: the Toledo tornado is a `2,0,1` row here too and carries Michigan's two deaths in its
eighteen, and the Willshire tornado's two are missing from Ohio for the opposite reason, so both
files reach 60 by errors of two in opposite directions. That is what the description means by "When
summing for state total use sn=1, not sg=1."
