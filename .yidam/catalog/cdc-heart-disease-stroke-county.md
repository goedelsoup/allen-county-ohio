---
name: Heart disease and stroke mortality by county (CDC, Division for Heart Disease and Stroke Prevention)
description: >-
  Cause-specific death rates for the county, from the one CDC programme that publishes below the
  state line. It is how this corpus finally learned what its residents die of, after the agency
  that holds the death certificates refused the county to a machine.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://data.cdc.gov/resource/7b9s-s8ck.json?locationid=39003
    description: >-
      *Rates and Trends in Heart Disease and Stroke Mortality Among US Adults (35+) by County, Age
      Group, Race/Ethnicity, and Sex – 2000-2019.* 1,840 rows for Allen County: five topics — all
      heart disease, all stroke, cardiovascular disease, coronary heart disease, heart failure —
      across two age groups, six race categories, three sex categories and twenty-one years. Two
      further "years" are not years: `1999 - 2010` and `2010 - 2019` carry the programme's own
      fitted total percent change, which is not the endpoint-to-endpoint arithmetic and is quoted
      here as published.
  - kind: url
    value: https://data.cdc.gov/resource/mri7-5jtw.json?locationid=39003
    description: >-
      *Heart Disease Mortality … 2022-2024*, the newest of four vintages on one design — a
      three-year average for ages 35 and over, age-adjusted and spatially smoothed. The three
      before it are `jiwm-ppbh` (2018–2020), `55yu-xksw` (2019–2021) and `th8y-thx5` (2021–2023).
      Together they are the only recent cause-specific series this corpus can put beside a state
      figure, because unlike the twenty-year file these carry State and County rows in one table.
  - kind: url
    value: https://data.cdc.gov/resource/y5ii-knwc.json?locationid=39003
    description: >-
      *Stroke Mortality … 2022-2024*, the same design for the other cause; `kgsi-35re` is its
      2013–2015 predecessor.
used-by:
  - ../corpus/measure/allen-county-heart-disease-and-stroke-1999-2024.yml
---

**It exists because the other door is shut.** CDC WONDER holds the death certificates and
[serves the nation to a machine and refuses the county](../decisions/the-api-serves-the-nation-and-refuses-the-county.yml).
This programme publishes the same underlying deaths at county grain, and it can do so because it
does not publish counts. [verified]

**Everything here is a model, and the file says so in a column.** Every value carries a
`data_value_type` reading *Age-Standardized, Spatiotemporally Smoothed Rate* or *Age-adjusted,
Spatially Smoothed, 3-year Average Rate*. These are small-area estimates that borrow strength from
neighbouring counties and adjacent years; they are not this county's deaths divided by this
county's people. A rate here is reliable as a level and as a direction, and a difference of a point
between two counties in one year is not a finding. [verified] — the programme's own methodology.
See [a modelled estimate is not an observation](../decisions/a-modelled-estimate-is-not-an-observation.yml).

**Two designs, and they must not be subtracted from each other.** The twenty-year file splits ages
35–64 from 65-and-over and smooths across space *and* time; the recent vintages combine everyone
over 35 and smooth across space only. Different ages, different models, different years. This
corpus reports both and differences neither against the other. [inference]

**And the vintages overlap.** 2018–2020, 2019–2021, 2021–2023 and 2022–2024 share years with their
neighbours, so four vintages are not four independent observations of anything. A move across the
four is a move in a smoothed overlapping average, which is the only recent thing on offer.
[verified] — the four files' own date ranges.

**A trap in the county names.** The twenty-year file calls this county `Allen`; the recent vintages
call it `Allen County`. A ranking script written against one and run against the other returns
eighty-eight counties with this one missing and no error. [verified] — it did.

**What it does not carry.** Only these two causes. No cancer, no accidents, no suicide, no
overdose, no diabetes, no respiratory disease — so this file names two of the county's killers and
is silent on every other. Nothing below the county, and no counts anywhere.
