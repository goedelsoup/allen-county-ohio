---
name: Census of Governments — individual government unit lists, 2012 / 2017 / 2022
description: >-
  The Census Bureau's roster of every government that exists in the United States: its name, its
  presiding officer's title, its mailing address and its county. Not a geography product — a
  survey of governments — which is why it can answer a question no map can, namely whether a
  municipal corporation is still there.
type: dataset
obtained: true
retrieved: 2026-08-31
ttl_days: 1825
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/gus/datasets/2012/govt_units_2012.zip
    description: >-
      8.5 MB, one XLSX with four sheets — general purpose, special district, school district,
      dependent school district. The general-purpose sheet is the one that lists municipalities
      and townships.
  - kind: url
    value: https://www2.census.gov/programs-surveys/gus/datasets/2017/govt_units_2017.ZIP
    description: 10.0 MB. Same shape; the column set gains `UNIT_TYPE`.
  - kind: url
    value: https://www2.census.gov/programs-surveys/gus/datasets/2022/govt_units_2022.ZIP
    description: >-
      11.5 MB. Adds `IS_ACTIVE`, which is the field that would have said outright what the 2012
      and 2017 files say only by a name's presence and absence.
used-by:
  - ../corpus/place/fort-shawnee.yml
  - ../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml
---

**Why a corpus about ground wants a list of governments.** Everything else this corpus holds about
municipal existence comes from geography files, and a geography file cannot distinguish a
corporation that dissolved from a tabulation category that changed. This one can, because its unit
of observation is the government and not the polygon: a row here has a mayor and a street address.

**What it says about Allen County.** The 2012 list carries `VILLAGE OF FORT SHAWNEE`, census ID
`36200250100000`, presiding officer *Mayor*, 2050 W Breese Rd, Lima OH 45806, population 3,726 as
of 2010. The 2017 list does not carry it. Neither does 2022. `TOWNSHIP OF SHAWNEE` is in all three.

That is the difference between the two readings the corpus had been holding open, and it points at
one of them. It does not date the event, and it does not name the instrument.

**What it does not carry.** No dissolution dates, no legal citations, no history — an absent row is
the only evidence of an ending, and a roster taken every five years cannot say which of five years
it happened in. The `IS_ACTIVE` flag introduced in 2022 marks units active in that file; it does
not reach back.
