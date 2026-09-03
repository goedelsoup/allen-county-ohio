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
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/datasets/2020-2024/counties/asrh/cc-est2024-agesex-39.csv
    description: >-
      County population by age and sex, Ohio, this vintage. Carries `UNDER5_TOT`, `AGE513_TOT`,
      `AGE1417_TOT` and the rest as counts; it does not publish an under-18 column, which is the
      sum of those three.
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/datasets/2010-2020/counties/asrh/CC-EST2020-AGESEX-39.csv
    description: >-
      The same file for the 2010s, and **its name is upper-case** — the lower-case form the newer
      vintage uses returns a 404 page named `.csv`. It is a different series, not more years of
      this one.
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/datasets/2010-2019/counties/totals/co-est2019-alldata.csv
    description: >-
      Components of change for the 2010s — `BIRTHS2010` through `BIRTHS2019` and the same for
      deaths and migration. The 2020s equivalent is the `co-est2024-alldata.csv` already listed.
  - kind: url
    value: https://www2.census.gov/programs-surveys/popest/technical-documentation/file-layouts/2010-2020/cc-est2020-agesex.pdf
    description: >-
      The file layout, which is the only place the `YEAR` codes are defined and the only place the
      2010s series says it was built without the 2020 census.
used-by:
  - ../corpus/jurisdiction/auglaize-township.yml
  - ../corpus/jurisdiction/village-of-beaverdam.yml
  - ../corpus/jurisdiction/village-of-bluffton.yml
  - ../corpus/jurisdiction/village-of-cairo.yml
  - ../corpus/jurisdiction/village-of-elida.yml
  - ../corpus/jurisdiction/village-of-harrod.yml
  - ../corpus/jurisdiction/village-of-spencerville.yml
  - ../corpus/measure/allen-county-children-2010-2024.yml
  - ../corpus/measure/allen-county-elected-seats-2026.yml
  - ../corpus/measure/allen-county-natural-change-2021-2024.yml
  - ../corpus/measure/allen-county-net-migration-2021-2024.yml
  - ../corpus/measure/allen-county-population-2020.yml
  - ../corpus/measure/allen-county-population-2024.yml
  - ../corpus/measure/allen-county-townships-1930-1950.yml
  - ../corpus/measure/lima-population-2020.yml
  - ../corpus/measure/lima-population-2024.yml
  - ../corpus/period/depopulation.yml
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
  - ../corpus/question/why-allen-countys-villages-are-staffed-by-appointment.yml
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

**`YEAR` is a code and not a year, and the codes differ between vintages.** In the 2010s age file
1 is the 2010 census, 2 the estimates base, 3 through 12 the July estimates for 2010 to 2019, 13 the
1 April 2020 estimate and 14 the 1 July 2020 estimate. In the 2020s file 1 is the 2020 base and 2
through 6 are July 2020 to 2024. [verified] — the Bureau's file layout. Reading `YEAR` as a year
puts every figure a decade out and raises nothing.

**The first `BIRTHS` and `DEATHS` column of each vintage is a three-month stub.** `BIRTHS2010` is
339 for Allen County and `BIRTHS2020` is 293, against about 1,200 in every full year, because each
covers only the census date to 30 June. [verified] — the two components files. Plotted in a column
of full years it is a collapse and a recovery that did not happen.

**The two vintages disagree about this county's children by 3.5 per cent at the date they share.**
1 July 2020: 101,980 people against 102,137, and 23,452 under eighteen against 24,263. [verified] —
the two age files. The total agrees to a seventh of a per cent and the parts do not, which is why
[an estimate is anchored to a census](../decisions/an-estimate-is-anchored-to-a-census.yml) says to
test a seam on the figure being used.

**The components files reach back further than the age files were taken.** `co-est2019-alldata.csv`
carries births, deaths and both migration components for every county for 2010 to 2019, and the
1990s and 2000s have their own vintages on paths this corpus has not walked. What is here is
fifteen years, and the enrolment series it is used against runs thirty-seven. [verified]
