---
name: Small Area Income and Poverty Estimates — states and counties
description: >-
  Thirty-one annual estimates of how many people in every American county are below the poverty
  line and what the median household earns, 1989 through 2024, each with a ninety per cent
  confidence interval on every figure. It is the only source in this corpus that gives Allen County
  a poverty series at all, and the only one that publishes an interval wide enough to refuse most
  of its own year-to-year movement.
type: dataset
obtained: true
retrieved: 2026-09-06
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/saipe/datasets/2024/2024-state-and-county/est24all.txt
    description: >-
      The 2024 file: one fixed-width row per county and per state, 3,196 of them, with twenty-one
      numeric fields and no header. Substitute the year twice in the path and the two-digit year in
      the file name. Files from 2004 onward are `.txt`; 1989 through 2003 are `.dat` with the same
      layout under a different extension.
  - kind: url
    value: https://www2.census.gov/programs-surveys/saipe/datasets/1989/1989-state-and-county/est89all.dat
    description: >-
      The earliest county file. It carries one extra trailing column the later years do not — a
      four-person-family median income the programme later dropped — so a parser that counts fields
      from the right reads 1989 wrong and every other year correctly.
  - kind: url
    value: https://www2.census.gov/programs-surveys/saipe/datasets/
    description: >-
      The directory listing, which is how the produced years are established rather than guessed:
      1989, 1993, 1995, 1996, 1997, 1998 and then annually from 1999. There is no 1990, 1991, 1992
      or 1994, and the gaps are the programme's rather than the retrieval's.
---

**Twenty-one numbers a row, and seven of them are the estimates.** Poverty count and poverty rate
for all ages, for people under 18, and for related children aged 5 to 17; and median household
income. Each of the seven is followed by its **ninety per cent lower and upper bound**, which is
what makes the file three times as wide as the estimates it publishes. [verified]

**It is a model, not a survey and not a count.** The estimates borrow strength from federal income
tax returns, food assistance participation, the previous census and the American Community Survey,
because no annual instrument samples a county of a hundred thousand people well enough to publish
a rate from it directly. [verified] The corpus has held a count and a survey with a margin; see
[a survey is not a count](../decisions/a-survey-is-not-a-count.yml) and
[a modelled estimate is not an observation](../decisions/a-modelled-estimate-is-not-an-observation.yml).

**It is the same production as [the school-district file](census-saipe-school-districts.md) and is
not a second witness to it.** One annual model run produces states, counties and school districts
from shared inputs; the district file simply publishes three integers where this one publishes
twenty-one. Agreement between them is not corroboration. [verified] What this file has and that one
does not is the interval, which is the whole of why it was worth fetching separately.

**The interval is wider than almost everything anyone would want to say with it.** Allen County's
2024 rate is 14.8 per cent with bounds of 12.6 and 17.0 — a span of 4.4 points on a figure that has
moved 9.5 points across the whole series. [verified] Across thirty-one years and thirty adjacent
pairs, **not one year-to-year change in this county separates from the one before it**; see
[a series that cannot see a year](../decisions/a-series-that-cannot-see-a-year.yml).

**The layout is fixed-width and the safe parse is from the left.** State FIPS, county FIPS, then
twenty-one numeric fields, then a county name that contains spaces, then a postal abbreviation, and
in recent years a trailing file name and a production date. Splitting on whitespace and taking
fields two through twenty-two works on every year including 1989; counting back from the end does
not. [verified] A suppressed cell is a bare `.`, which parses as neither a number nor an empty
string.

**One year of the archive is broken and it is not this corpus's doing.** Every path under
`/programs-surveys/saipe/datasets/1996/` — the file, the subdirectory, the year's own index —
answers **HTTP 520**, repeatedly and over several minutes, while all thirty other produced years
answer 200 on the first request. The parent listing names `1996/` as a directory. So the year
exists, the programme produced it, and the server will not serve it. [verified] — requested four
ways and recorded; see [a live URL is not a live file](../decisions/a-live-url-is-not-a-live-file.yml).
