---
name: CDC county-level COVID-19 files
description: >-
  Three federal county series for the pandemic, all on the same open data service and all keyless —
  weekly deaths by county of occurrence from the National Center for Health Statistics, county
  vaccination coverage, and the weekly community levels the agency used to advise mask-wearing.
  The first of the three suppresses more of this county than it publishes.
type: dataset
obtained: true
retrieved: 2026-09-05
ttl_days: 1825
location:
  - kind: url
    value: https://data.cdc.gov/resource/ite7-j2w7.json
    description: >-
      "AH COVID-19 Death Counts by County and Week, 2020-present", data as of 5 April 2023. One row
      per county-week with deaths involving COVID-19 coded to ICD-10 U07.1 and deaths from all
      causes. 170 weeks for Allen County, ending 1 April 2023.
  - kind: url
    value: https://data.cdc.gov/resource/8xkx-amqh.json
    description: >-
      "COVID-19 Vaccinations in the United States, County", 598 daily rows for Allen County from 13
      December 2020 to 10 May 2023, when the series ended with the public health emergency. Carries
      dose-one and completed-series counts and shares by age band, booster doses, and a
      completeness percentage for the county's residence coding.
  - kind: url
    value: https://data.cdc.gov/resource/3nnm-4jni.json
    description: >-
      "United States COVID-19 Community Levels by County", weekly from February 2022 to May 2023,
      with the case rate, the hospital admission rate and the inpatient-bed share the agency
      combined into a low, medium or high level. Its `county_population` column is a vintage
      estimate and is 145 people above the 2020 census count for this county.
  - kind: url
    value: https://data.cdc.gov/resource/anag-cw7u.json
    description: >-
      The facility-level hospital capacity series, which would name this county's two hospitals.
      Answers 404; not used.
---

**The weekly death file suppresses every cell below ten and it is most of this county's record.**
96 of Allen County's 170 county-weeks carry no death figure at all, with the footnote "One or more
data cells have counts between 1-9 and have been suppressed in accordance with NCHS confidentiality
standards." The 74 weeks that survive sum to 601. [verified] — the file, counted here. Published as
it stands, the county's total by occurrence is somewhere between 697 and 1,465.

**Ohio publishes the suppressed weeks and the suppression rule holds in every one of them.** In all
96 weeks the state's own count of deaths in this county is between nought and nine, and in none of
them is it ten or more. [verified] — this file against
[Ohio's dashboard files](ohio-covid-19-dashboard-files.md), week ending Saturday, matched date by
date. The rule is exactly what the footnote says it is, checked on ninety-six cells.

**And where both publish a number, they agree.** Across the 74 unsuppressed weeks the federal file
gives 601 and the state gives 567; **50 of the 74 are equal to the death, 71 of 74 are within two,
and the largest disagreement in any week is three.** [verified] — the same matching. The federal
count runs slightly the higher throughout, which is what the two definitions predict: NCHS counts a
death with U07.1 anywhere on the certificate and Ohio counts a death due to the illness. So the two
instruments differ by a definition of about six per cent and by a disclosure rule of about a
quarter.

**Its all-cause column is not suppressed and it is by occurrence too.** 5,669 deaths from all
causes in Allen County over those 170 weeks — about 1,740 a year, against roughly 1,300 a year for
the county's residents in the Census Bureau's own components of change. [verified] — this file
against [the natural change](../corpus/measure/allen-county-natural-change-2021-2024.yml). The gap
is the same gap the COVID figures show and it is a property of where the hospital is; see
[located here is not of here](../decisions/located-here-is-not-of-here.yml).

**The vaccination file dates its own coverage and warns about its own residence coding.** Allen
County's `completeness_pct` is 98.6, which is the share of doses administered in Ohio for which a
county of residence was recorded. The series stops on 10 May 2023 with the end of the federal
public health emergency and nothing continues it at county grain. [verified] — the file.

**Every one of the three answers a keyless request.** No registration, no token, no throttle
encountered at these volumes — which is worth saying in a corpus that has recorded a long line of
federal interfaces that refuse one. [verified] — the requests made here.
