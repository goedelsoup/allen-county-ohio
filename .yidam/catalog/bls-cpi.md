---
name: Consumer Price Index for All Urban Consumers (BLS)
description: >-
  The price level, monthly and annually, from 1913 to the present. It is the instrument this corpus
  has been missing every time it printed a dollar figure from one year beside a dollar figure from
  another — three nodes said in so many words that they could not deflate, and one of them carried
  an open question about it.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 180
location:
  - kind: url
    value: https://download.bls.gov/pub/time.series/cu/cu.data.1.AllItems
    description: >-
      All-items index values for every geography and adjustment, tab-delimited, 2.7 MB, 63,946 rows.
      The series this corpus uses is `CUUR0000SA0`; annual averages carry `period` = `M13` and the
      twelve months carry `M01` through `M12`.
  - kind: url
    value: https://download.bls.gov/pub/time.series/cu/cu.series
    description: >-
      The series catalogue, 8,105 rows, which is where a series id is decoded into its area, its
      item, its base and the range of periods it covers.
used-by:
  - ../corpus/measure/allen-county-private-employers-1986-2023.yml
  - ../corpus/measure/allen-county-manufactures-1939-1967.yml
  - ../corpus/measure/allen-county-building-permits-1990-2025.yml
  - ../corpus/measure/allen-county-house-prices-1975-2025.yml
  - ../corpus/measure/allen-county-house-prices-by-tract-1986-2025.yml
  - ../corpus/measure/allen-county-personal-income-1969-2024.yml
  - ../corpus/measure/allen-county-transfer-receipts-1969-2024.yml
  - ../corpus/measure/allen-county-federal-contributions-1980-2024.yml
---

**A request without a user agent gets HTTP 403.** BLS asks that automated requests identify the
requester, and a bare `curl` is refused outright rather than served an empty body — which makes this
the friendlier of the two bot filters this corpus has met, because the failure is visible.
[verified] — the retrievals here, with and without.

**The series is `CUUR0000SA0` and its full name is a sentence.** *All items in U.S. city average,
all urban consumers, not seasonally adjusted*, on the 1982–84 = 100 base, monthly from January 1913.
[verified] — `cu.series`. Its annual average for 1986 is 109.6 and for 2023 is 304.702, so a 1986
dollar is 2.78 of a 2023 dollar.

**There is a Midwest index and this corpus does not use it, and the choice is not free.** `CUUR0200SA0`
— *All items in Midwest urban* — begins in December 1966, twenty-two years after the earliest dollar
figure this corpus holds and fifty-three after the national series. Over 1986 to 2023 it rises 2.62
times against the national 2.78, a difference of six per cent on a thirty-seven-year comparison.
[verified] — the same file. That is large enough to change a sign; see
[a deflator is a choice](../decisions/a-deflator-is-a-choice.yml).

**`M13` is not a month.** The period codes run `M01` to `M12` for months, `M13` for the annual
average, and `S01`/`S02` for the semiannual series that some ids carry instead. A reader that filters
on `period` beginning with `M` and averages what it gets counts the year twice. [verified] — the same
file.

**It reaches every dollar figure in this corpus.** The oldest is 1909 and the index begins in 1913,
so the four years before that are the only ones it cannot serve; the 1929, 1939, 1947, 1954, 1958,
1963 and 1967 censuses of manufactures are all inside it, and so is every modern series.
[verified]

**What it is not.** It is a national index of urban consumer prices. It is not a cost-of-living index
for Allen County, not a producer price index, and not the right deflator for construction cost or
for value added — each of which has its own index that this corpus does not hold. A figure deflated
here is stated as *in the dollars of year X by the national consumer price index*, and never as
*what it would cost today*.
