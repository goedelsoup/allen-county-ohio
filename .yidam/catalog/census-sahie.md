---
name: Census Bureau Small Area Health Insurance Estimates
description: >-
  A modelled estimate of how many people in every county in the United States have no health
  insurance, published once a year since 2008 and by a different method for three years before
  that. It is the only annual county-level coverage series that exists, and it is the first source
  in this corpus whose own inputs include the programme whose effect a reader would want to measure.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/sahie/datasets/time-series/estimates-acs/sahie-2024-csv.zip
    description: >-
      The 2024 estimates, 13.7 MB compressed and 111.6 MB open, 402,872 rows. Every year from 2008
      to 2024 sits at the same path with its own year. A row is a county or a state crossed with an
      age, a race, a sex and an income category, and carries the number in the group, the number
      uninsured, the number insured and four percentages, each with a 90 per cent margin of error.
  - kind: url
    value: https://www2.census.gov/programs-surveys/sahie/datasets/time-series/estimates-cps/sahie-2007.csv
    description: >-
      The older series, 2005 to 2007, built from the Current Population Survey rather than the
      American Community Survey. Separate directory, different model, and not a continuation of the
      one above. 2006 and 2007 were taken; 2005 is a fixed-width text file that was not parsed.
  - kind: url
    value: https://www.census.gov/programs-surveys/sahie/technical-documentation/model-input-data.html
    description: >-
      The list of what goes into the model. It is the reason this entry exists in the shape it does.
used-by:
  - ../corpus/measure/allen-county-health-insurance-2008-2024.yml
  - ../corpus/measure/allen-county-coverage-types-2023.yml
---

**The Bureau publishes what the model was told, and the list decides what the file may be used
for.** The inputs are the American Community Survey's direct single-year estimates, the Current
Population Survey's ASEC, County Business Patterns, the Bureau's own population estimates, federal
individual income tax returns, SNAP recipient counts, **Medicaid participation — "Number of
individuals enrolled in Medicaid by age and sex" — and Children's Health Insurance Program
participation**, plus tenure and urban-rural counts from the decennial census. [verified] — the
model input data page. So a reader who uses this file to show that a Medicaid expansion cut the
uninsured rate is reading a covariate back out of a prediction; see
[a modelled estimate is not an observation](../decisions/a-modelled-estimate-is-not-an-observation.yml),
whose second rule this is the second case of, and
[a model fitted to a survey is not a second witness](../decisions/a-model-fitted-to-a-survey-is-not-a-second-witness.yml).

**Its dependent variable is a source this corpus already holds.** The survey the model is fitted to
is the American Community Survey, which is where this corpus's income, poverty, race and housing
figures come from and which publishes health insurance coverage directly in tables B27001 and
B27010. Agreement between the two is not two sources agreeing.

**The file grows underneath the series.** Allen County has 50 published cells in 2008 and 114 in
2024: age categories 3, 5 and 6 and income category 5 appear part-way through, so a cut that exists
in the last year may not exist in the first. [verified] — the county's rows in all seventeen files.
Only *under 65*, *18 to 64*, *40 to 64*, *under 19*, the three sexes and income categories 0 to 4
run the whole span.

**Race is a state figure here and not a county one.** County rows carry `racecat` 0 and nothing
else, by the file's own layout note; the seven race categories exist only at state level.
[verified] — the layout preamble and every county row in it. There is no national row at all: the
file's largest geography is a state.

**2013 was published twice and only the second is in this copy.** The year carries a `version`
column reading `Updated`, for a May 2016 revision; the original is a separate download that was not
taken. Every other year's version column is blank. [verified] — the 2013 file.

**The margins say which series is which.** The ACS-based estimates give Allen County's under-65
uninsured rate to about ±1.1 points in 2008 and ±0.8 by 2024; the CPS-based estimate for 2006 is
12.2 ± 2.0 and for 2007 is 12.5 ± 1.7. The old series' intervals are roughly twice as wide, and its
last year overlaps the new series' first. Overlapping intervals across a change of method are not a
join.
