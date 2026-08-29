---
name: Census Population Estimates, Vintage 2024
description: >-
  The Census Bureau's annual population estimates for counties and sub-county units,
  Vintage 2024 — an April 2020 estimates base plus July 1 estimates for 2020 through 2024.
type: dataset
obtained: true
retrieved: 2026-08-28
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/datasets/2020-2024/counties/totals/co-est2024-alldata.csv
    description: county totals, all states
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/datasets/2020-2024/cities/totals/sub-est2024.csv
    description: sub-county totals — places and minor civil divisions
used-by:
  - ../corpus/jurisdiction/auglaize-township.yml
  - ../corpus/jurisdiction/village-of-beaverdam.yml
  - ../corpus/jurisdiction/village-of-bluffton.yml
  - ../corpus/jurisdiction/village-of-cairo.yml
  - ../corpus/jurisdiction/village-of-elida.yml
  - ../corpus/jurisdiction/village-of-harrod.yml
  - ../corpus/jurisdiction/village-of-spencerville.yml
  - ../corpus/measure/allen-county-natural-change-2021-2024.yml
  - ../corpus/measure/allen-county-net-migration-2021-2024.yml
  - ../corpus/measure/allen-county-population-2020.yml
  - ../corpus/measure/allen-county-population-2024.yml
  - ../corpus/measure/lima-population-2020.yml
  - ../corpus/measure/lima-population-2024.yml
  - ../corpus/period/deindustrialization.yml
  - ../corpus/place/allen-county.yml
  - ../corpus/place/amanda-township.yml
  - ../corpus/place/american-township.yml
  - ../corpus/place/auglaize-township.yml
  - ../corpus/place/bath-township.yml
  - ../corpus/place/beaverdam.yml
  - ../corpus/place/bluffton.yml
  - ../corpus/place/cairo.yml
  - ../corpus/place/delphos.yml
  - ../corpus/place/elida.yml
  - ../corpus/place/harrod.yml
  - ../corpus/place/jackson-township.yml
  - ../corpus/place/lafayette.yml
  - ../corpus/place/marion-township.yml
  - ../corpus/place/monroe-township.yml
  - ../corpus/place/perry-township.yml
  - ../corpus/place/richland-township.yml
  - ../corpus/place/shawnee-township.yml
  - ../corpus/place/spencer-township.yml
  - ../corpus/place/spencerville.yml
  - ../corpus/place/sugar-creek-township.yml
  - ../corpus/question/pre-1970-population-series.yml
---

Comma-delimited files from the Census Bureau's Population Estimates Program. The county file
carries `ESTIMATESBASE2020` and `POPESTIMATE2020` through `POPESTIMATE2024`; the sub-county
file carries the same series for incorporated places and minor civil divisions, keyed by
`SUMLEV` — `061` for a minor civil division, `071` for a place-within-MCD part, `157` for a
place total.

**An estimate is not a count, and the two are in this file together.** `ESTIMATESBASE2020` is
the April 1, 2020 census count as subsequently revised, and every `POPESTIMATE` column is a
July 1 modeled estimate. A node comparing a 2020 figure to a 2024 figure is comparing a
revised enumeration to a model output. Say which is which; do not describe an estimate as a
census count.

**`(pt.)` is load-bearing.** Bluffton and Delphos appear in the Allen County rows as
`Bluffton village (pt.)` and `Delphos city (pt.)` — both straddle a county line, so the
figures under Allen County are the Allen County portion only, not the municipality. Reading
them as whole-place totals understates both places. This is the file stating a boundary fact
that the gazetteer's per-state place file does not.

**`ttl_days` is short here** — 365 rather than the gazetteer's 3650 — because a new vintage
supersedes this one every year, and each vintage revises the whole series back to 2020
rather than only appending. A figure cited from Vintage 2024 will not necessarily appear in
Vintage 2025.

**The sub-county file is the county's civil geography in one place.** Its 40 Allen County rows
carry four things at once, and it took nine phases to read them: every municipality (summary
level 157), every township (061), **which township each municipality sits in** (071, a place
within a county subdivision), and which municipalities cross a county line — the `(pt.)` mark
on the name. All 21 of the county's civil divisions came from here.

**The components of change are now read out.** The county file's `BIRTHS`, `DEATHS`,
`INTERNATIONALMIG` and `DOMESTICMIG` columns had been catalogued and unread since this corpus's
second phase, while five phases described a population decline they could not explain. They are the
explanation: over 2021–2024 the county lost 506 people to natural decrease and 793 to net
migration. They are published for the **estimate years only**, so they explain four of the
fifty-four years this corpus's decline covers and nothing before 2020.

**What else it holds that nobody has looked at.** Births, deaths, net migration and the
components of change are all in the county file and none were read. They are the mechanism
behind the population decline this corpus describes, and the decline is all it can currently
describe.
