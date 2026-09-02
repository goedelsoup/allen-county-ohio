---
name: Local Area Unemployment Statistics (BLS)
description: >-
  The Bureau of Labor Statistics' monthly estimate of how many people in a county are working,
  how many are looking, and what share of the two is out of work. It is the household side of a
  labour market this corpus had only ever counted from the employer's side, and it is the first
  source here that reaches the county monthly rather than annually.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 90
location:
  - kind: url
    value: https://download.bls.gov/pub/time.series/la/la.data.64.County
    description: >-
      Every county in the United States, every month, all four measures — 336 MB. Allen County is
      area code `CN3900300000000` and its four series are `LAUCN390030000000003` (unemployment
      rate), `004` (unemployed), `005` (employed) and `006` (labour force). Period `M13` is the
      annual average and sits in the same file as the months. The county's rows were extracted
      and the file deleted; 1,901 lines is what this corpus keeps of it.
  - kind: url
    value: https://download.bls.gov/pub/time.series/la/la.data.2.AllStatesU
    description: >-
      The same measures for the states, not seasonally adjusted, 13 MB. Ohio is
      `LAUST390000000000003` and runs from 1976 — twelve years further back than the counties do.
      The unadjusted state series is the one that matches a county: county estimates are published
      unadjusted, and comparing them against a seasonally adjusted state figure would put the
      county's January against a state's smoothed January.
  - kind: url
    value: https://api.bls.gov/publicAPI/v1/timeseries/data/
    description: >-
      The national rate is not in this programme at all. `LNU04000000` — the Current Population
      Survey's own unadjusted rate for the United States — was taken from the public API in four
      ten-year requests, which is what v1 allows without a key.
  - kind: url
    value: https://download.bls.gov/pub/time.series/la/la.footnote
    description: >-
      Nine footnote codes. Three of them are the agency saying a month is missing and why, and two
      of those three are used on Allen County's rows.
used-by:
  - ../corpus/measure/allen-county-unemployment-1990-2026.yml
  - ../corpus/measure/allen-county-commuting-2022.yml
---

**It reaches this county monthly, and nothing else here does.** Every other measure of work in this
corpus is annual at best — [BEA](bea-county-employment.md) once a year,
[QCEW](bls-qcew.md) once a year, [County Business Patterns](census-county-business-patterns.md) once
a year, [the census of manufactures](census-of-manufactures-area-statistics-1939-1967.md) once in
five. This one publishes 439 separate months for Allen County between January 1990 and July 2026,
which is why it can say what a recession did to the county in the month it did it. [verified]

**The server refuses a client that does not name itself.** `download.bls.gov` returns 403 to a
request with no `User-Agent` header *and* to one carrying a tool's default string, and 200 to the
same request under a name with a contact address in it. Everything here was retrieved under
`allen-county-corpus/1.0`, which is the agency's stated condition of use rather than a way around a
block. [verified] — all three tried against `la.footnote`.

**It is an estimate, and the agency says so in its own structure.** County figures are modelled —
built from unemployment insurance claims, the payroll survey and the household survey, then
controlled to a state total that is itself controlled to the national. So a county's month is not a
count of that county's households; it is a state figure apportioned on county evidence. This matters
for what may be asked of it: the level is reliable and the turn is reliable, and a difference of a
tenth of a point between two counties in one month is not. [verified] — the programme's own
methodology.

**Two of its footnotes are about a month that does not exist.** Code `X` reads *"Data unavailable
due to the 2025 lapse in appropriations"* and code `G` reads *"Annual estimates for 2025 are
11-month averages that exclude October. Data for October 2025 were not collected due to the federal
government shutdown."* Both are on Allen County's rows, on Ohio's, and on every county in the file.
[verified] — `la.footnote`, and the county's own 2025 rows.

**What it will not tell you.** Nothing below the county — no township, no city, no Lima. Nothing
about who the unemployed are: no age, no race, no sex, no industry, no duration. And nothing before
1990 at county grain, which is the wall this corpus meets again: the state series reaches 1976 and
the county series does not.
