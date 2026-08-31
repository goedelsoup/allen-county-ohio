---
name: CDC PLACES — Local Data for Better Health, 2025 release
description: >-
  Forty health measures for every county, city and census tract in the United States, modelled from
  the Behavioral Risk Factor Surveillance System onto small areas. It is the first health source in
  this corpus and the first that reaches the county's tracts, which is the grain the corpus already
  works at for lending and race.
type: dataset
obtained: true
retrieved: 2026-08-31
ttl_days: 365
location:
  - kind: url
    value: https://data.cdc.gov/resource/cwsq-ngmh.json?$limit=5000&stateabbr=OH&countyname=Allen
    description: >-
      Census tract file, Socrata, keyless. 1,400 rows for Allen County — 40 measures across all 35
      of its 2020 tracts, crude prevalence only at this grain, with each tract's 2020 population.
  - kind: url
    value: https://data.cdc.gov/resource/swc5-untb.json?$limit=2000&stateabbr=OH&locationname=Allen
    description: County file. 80 rows — 40 measures at crude and age-adjusted prevalence.
  - kind: url
    value: https://data.cdc.gov/resource/eav7-hnsx.json?$limit=2000&stateabbr=OH&locationname=Lima
    description: >-
      Place file, for the City of Lima, GEOID 3943554. The same 80 rows, on a total population of
      35,579 — which is the corpus's own 2020 census figure for the city, unrounded.
used-by:
  - ../corpus/measure/allen-county-health-2023.yml
---

**It says what it is, in the dataset's own description**, and the corpus takes the caution as
binding:

> This dataset contains model-based census tract estimates. … Because the small area model cannot
> detect effects due to local interventions, users are cautioned against using these estimates for
> program or policy evaluations. Data sources used to generate these model-based estimates are
> Behavioral Risk Factor Surveillance System (BRFSS) 2023 or 2022 data, Census Bureau 2020
> population data, and American Community Survey 2019-2023 or 2018–2022 estimates.

That last sentence is the one that matters for this corpus. The American Community Survey is an
**input to the model**, and it is also the source of this corpus's figures for income, poverty,
tenure and race. So a PLACES estimate for a Lima tract is partly a function of that tract's ACS
profile, and explaining the health estimate by the ACS profile would be circular. See
[a modelled estimate is not an observation](../decisions/a-modelled-estimate-is-not-an-observation.yml).

**Its tract set is the corpus's tract set exactly.** All 35 of Allen County's 2020 tracts appear,
and their populations sum to **102,206** — the county's 2020 census count, which the corpus has
used as a closure check since its block-composition phase. [verified]

**Two vintages in one file.** Thirty-five measures rest on BRFSS 2023 and five on BRFSS 2022 — all
teeth lost, dental visits, mammograms, colorectal screening and short sleep — because the survey
asks those every other year. A row's `year` field carries which, and a table that ignores it
compares a 2022 estimate with a 2023 one.

**Two prevalence types, and only one of them exists at tract grain.** County and place rows carry
both crude and age-adjusted prevalence; tract rows carry crude only. Comparing a crude place figure
with an age-adjusted county figure would be a straightforward mistake and the file makes it easy.

**What is in it and unread.** The confidence limits on every estimate, which this corpus has used
only to check that the Lima–county gaps clear them; the seven health-related social needs measures
at tract grain beyond the four cited; and the ZIP-code tabulation area file, which is a fourth
geography this corpus does not hold.
