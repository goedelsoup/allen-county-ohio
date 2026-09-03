---
name: Eviction Lab county data (Princeton University)
description: >-
  Eviction filings and judgments by county and year, assembled from court records across the United
  States. It is the first source this corpus holds that describes what happens to a household that
  cannot pay, and the first that reaches inside a courthouse at all.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 730
location:
  - kind: url
    value: https://eviction-lab-data-downloads.s3.amazonaws.com/data-for-analysis/county_court-issued_2000_2018.csv
    description: >-
      The court-issued series, 33,247 county-years. `filings_observed` is eviction cases filed and
      `hh_threat_observed` the households they were filed against, which is the smaller number
      because a household can be filed against more than once in a year. Ohio has 87 of its 88
      counties in every year from 2002 to 2018.
  - kind: url
    value: https://eviction-lab-data-downloads.s3.amazonaws.com/legacy-data/validated/counties/OH_counties.csv
    description: >-
      The original county file for Ohio, 830 rows, 2000 to 2016. It carries judgments as well as
      filings — `evictions` against `eviction.filings` — which the court-issued file does not, and
      a block of demographic columns.
  - kind: url
    value: https://eviction-lab-data-downloads.s3.amazonaws.com/data-for-analysis/county_proprietary_valid_2000_2018.csv
    description: >-
      The third series, built from a commercial records vendor rather than from the courts, 21,232
      rows. It is the lowest of the three in every year this county appears in all of them.
used-by:
  - ../corpus/measure/allen-county-evictions-2001-2018.yml
---

**Three files from one project give three counts of one thing and never agree.** For the thirteen
years all three cover this county, the legacy file, the court-issued file and the proprietary file
differ by between 47 and 116 filings — a mean spread of 78 on counts averaging 865, or 9.0 per cent
— and in none of the thirteen do any two of them match. [verified] — the three files joined here.
The court-issued series is the one this corpus uses, because it is the one whose provenance is a
court; the spread is published beside it under
[three witnesses and three dates](../decisions/three-witnesses-and-three-dates.yml).

**The demographic columns in the legacy file are steps, not a series.** Population, poverty rate,
median rent, median income, median property value and rent burden take four distinct values across
this county's fifteen years — one per census or survey vintage, carried forward. [verified] — the
legacy Ohio file. The renter-household denominator is different: it is interpolated and moves every
year, which is why a rate computed from this file moves smoothly when its numerator does not.

**A filing is not a household and a judgment is not an eviction.** `filings_observed` exceeds
`hh_threat_observed` by 10 to 20 per cent in this county every year, the difference being repeat
filings against one household; and a judgment is an order, not a record that anyone moved.
[verified] — the court-issued file. Three quantities, three names, and a node that says *evictions*
without saying which one has said nothing.

**Coverage is a property of the file and not of the state.** The court-issued file has 87 Ohio
counties in every year from 2002 to 2018 and 88 in 2017. The legacy file has between 27 and 70 Ohio
counties with a filing count depending on the year, rising steadily across the run. [verified] — the
two files counted here. A state total from the legacy file is a total over a set of places that
changes size every year, and it climbs when coverage does; the court-issued file is the only one of
the three that will support a state figure at all.

**The bucket is open and the front door is not.** `evictionlab.org` refuses an automated request
with HTTP 403; the S3 bucket the site downloads from is publicly listable and served the files
without objection. [verified] — the retrievals here. What is published is the bucket.
