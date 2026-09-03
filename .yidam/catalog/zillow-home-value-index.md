---
name: Zillow Home Value Index, county file
description: >-
  What a typical Allen County house is worth in dollars, monthly since January 2000. It is the
  level that [the federal index](fhfa-house-price-index.md) refuses to give, from a company rather
  than an agency, and the two are used here for different jobs on purpose.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 180
location:
  - kind: url
    value: https://files.zillowstatic.com/research/public_csvs/zhvi/County_zhvi_uc_sfrcondo_tier_0.33_0.67_sm_sa_month.csv
    description: >-
      3,071 counties by 319 monthly columns, 13.5 MB, no key and no user-agent filter. Allen County
      is the row with `StateCodeFIPS` 39 and `MunicipalCodeFIPS` 003, and it is complete from
      2000-01-31 to 2026-07-31 with no missing month. The filename is the specification: `uc` all
      homes, `sfrcondo` single-family and condominium together, `tier_0.33_0.67` the middle third
      of the value distribution, `sm` smoothed, `sa` seasonally adjusted.
used-by:
  - ../corpus/measure/allen-county-house-prices-1975-2025.yml
---

**What one number is.** The typical value of a home in the 33rd to 67th percentile of the local
distribution, estimated for every home in the region whether or not it sold, then smoothed and
seasonally adjusted. [verified] — the file's own naming and Zillow's published specification. It is
not a median sale price: no transaction has to occur for a county to have a reading, which is why
the series has no gaps and why it is not a count of anything.

**It is a private company's model of a public market, and this corpus says so at every use.** The
inputs, the model and the revisions are Zillow's, are not published in a form that permits
reproduction here, and are restated backwards through the whole history at each release. [verified]
— the file is republished monthly in full. A figure taken from it is stated with its retrieval
date and never as an observation; compare
[a modelled estimate is not an observation](../decisions/a-modelled-estimate-is-not-an-observation.yml).

**Two independent series agree on the shape and differ on the size.** Over 2000 to 2024 this file
has Allen County's typical home rising 104.8 per cent and the federal index has 90.5 per cent —
seven and a half points of divergence over twenty-four years, with the same turning points in the
same years. [verified] — this file and [FHFA](fhfa-house-price-index.md), computed here. Neither is
the other's check; they share the housing market and nothing else.

**It agrees with the survey too, which is the more useful test.** Zillow's December 2023 reading
for the county is $168,156 and the American Community Survey's 2023 five-year median owner-reported
value is $158,400 — 6.2 per cent apart, on estimates that measure different things over different
windows. [verified] — this file and
[the survey](census-acs-summary-file.md). The corpus prints both and neither as *the* value.

**Its reach is not the county's reach.** The county file covers 3,071 counties, all 88 in Ohio; the
place and ZIP files it sits beside do not reach every Allen County village. Nothing here is taken
below county grain.
