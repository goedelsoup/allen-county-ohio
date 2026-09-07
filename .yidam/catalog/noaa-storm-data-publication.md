---
name: Storm Data and Unusual Weather Phenomena
description: >-
  The monthly book the Weather Bureau printed about the weather that broke things, and the document
  every row of the Storm Events Database before the 1990s was typed out of. It is the same record
  the database holds and it is not the same shape: it files a storm under one place name rather than
  under every county the storm crossed, it writes a paragraph about what the storm did, and it
  reports damage as a category on a nine-step ladder rather than as an amount. Scanned page images
  with no text layer, so nothing in it is searchable and nothing in it has an identifier.
type: publication
obtained: true
retrieved: 2026-09-06
ttl_days: 3650
location:
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/swdi/stormevents/pub-pdf/storm_1965_04.pdf
    description: >-
      *Storm Data and Unusual Weather Phenomena*, April 1965 — Volume 7 No. 4, U.S. Department of
      Commerce, Weather Bureau, Robert M. White, Chief; Asheville, 1965; printed 28 May 1965 in an
      edition of 1,100. Fourteen pages. Its Ohio section runs from page 28 to page 29 and carries
      the twelve tornadoes of Palm Sunday under the heading the Bureau gave them.
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/swdi/stormevents/pub-pdf/
    description: >-
      The whole run, one PDF per month, `storm_<year>_<month>.pdf`, **beginning at January 1959** —
      so the county's tornado of 19 July 1950 is in the databases and not in this book. There is no
      index, no OCR and no per-storm identifier: a citation into this source is a volume, a page and
      a place name. The files are image-only — `pdftotext` returns nothing — so they must be
      rendered and read.
used-by:
  - ../corpus/event/the-tornado-of-11-april-1965.yml
  - ../corpus/event/the-tornadoes-of-april-1965.yml
  - ../corpus/measure/allen-county-tornadoes-1950-2026.yml
  - ../corpus/measure/ohio-tornadoes-of-11-april-1965.yml
---

**Its damage column is a category and not an amount.** The footnote at the end of every table reads:
"Storm damages are placed in categories varying from 1 to 9 as follows: 1 Less than $50 / 2 $50 to
$500 / 3 $500 to $5,000 / 4 $5,000 to $50,000 / 5 $50,000 to $500,000 / 6 $500,000 to $5,000,000 /
7 $5,000,000 to $50,000,000 / 8 $50,000,000 to $500,000,000 / 9 $500,000,000 to $5,000,000,000."
**The Storm Events Database prints those categories as dollars** — one fixed value each, $30 and
then half of every upper bound — and gives no sign that it has done so. See
[a damage figure is a category](../decisions/a-damage-figure-is-a-category.yml).

**It files a storm under a place, and the database files it under counties.** The Palm Sunday
tornado that crossed this county appears here once, as **Bluffton**, with one track and one death
toll. The database holds it as an Allen County row and a Hancock County row that share a coordinate.
Some of this publication's Ohio entries are place names, some are county names — "Shelby County",
"Lorain & Cuyahoga Counties" — and the choice is the writer's. See
[a county row is a share of a storm](../decisions/a-county-row-is-a-share-of-a-storm.yml).

**Its clock is the local clock.** The column is headed TIME — LOCAL STANDARD, and Allen County keeps
Eastern. The database stamps the same events `CST` and moves them an hour earlier, which is why the
Ashville tornado of 12:30 a.m. on 12 April is dated to the 11th in the file and to the 12th in the
book. The corpus had inferred that offset from the timezone field alone; this publication is the
thing that confirms it.

**Its own numbers do not always close.** The Ohio header for April 1965 says "at least 12 separate
tornadoes struck within the state" and the twelve itemised rows kill exactly the 57 the header
claims — but the last of them opens "This comparatively mild tornado was the last of 10 major storms
which visited the northern half of Ohio during Palm Sunday evening and night". Two counts of one
evening, four pages apart, in one issue.

**It says in a footnote that it is provisional, and it names where the corrections go.** "This
publication contains our best information on storms but, due to the difficulties inherent in
collection of this type of data, it is not all-inclusive. Delayed data and corrections will be
carried in the June and December issues of this publication." So a figure read here has a stated
place to be checked against — and for April 1965 there is nothing there. The June issue's DELAYED
DATA section carries an April entry for Indiana and an April correction for Iowa ("The one injury
should be changed to the death column") and no Ohio entry at all; December's begins at July. The
Storm Events Database's nineteen Ohio county rows for 11–12 April 1965 sum to 60 deaths against
this issue's 57; every one of the twelve entries here matches its county rows to the person except
Rockaway, where the book says one died and the database says four; and **that revision of three
lives is not in either issue the book says it would be in.**
