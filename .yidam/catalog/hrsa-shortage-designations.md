---
name: HRSA health professional shortage area designations
description: >-
  Every Health Professional Shortage Area the federal government has ever designated for primary
  care, dental health or mental health, live and withdrawn, with the population it covers, the
  provider hours it was found to have, the ratio that made it short and the dates it began and
  ended. It is a register of a designation rather than a measure of a county, and it keeps what it
  has withdrawn.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 180
location:
  - kind: url
    value: https://data.hrsa.gov/DataDownload/DD_Files/BCD_HPSA_FCT_DET_PC.csv
    description: >-
      Primary care, 48.3 MB. Dental health is `BCD_HPSA_FCT_DET_DH.csv` at 28.2 MB and mental health
      `BCD_HPSA_FCT_DET_MH.csv` at 24.7 MB, same layout. One row per designation crossed with each
      piece of geography it covers, so a designation over three census tracts is three rows with one
      identifier.
used-by:
  - ../corpus/measure/allen-county-shortage-designations-1985-2026.yml
  - ../corpus/measure/allen-county-coverage-types-2023.yml
---

**A designation is not a place, and the file's own columns are what say so.** Each row carries a
designation type — geographic, population, correctional facility, federally qualified health centre
— and a population type. A *geographic* designation is drawn on a map. A *population* designation
is drawn on a class of people who live anywhere inside a boundary: the low-income population of a
county, everywhere in it. A *facility* designation is drawn on a building and its inmates or its
patients. These are not degrees of the same thing, and a county code that flattens them to "part of
the county" loses the distinction entirely. See
[a designation is not a county](../decisions/a-designation-is-not-a-county.yml).

**It keeps what it has withdrawn, and that is what makes it a history rather than a snapshot.**
`HPSA Status` is Designated or Withdrawn; a withdrawn row keeps its designation date, its withdrawal
date and every figure that stood at the end. Allen County has fifteen designations across the three
files, ten of them live and five withdrawn, and the two oldest were designated on 4 April 1985.

**Four numbers describe a shortage and they are not interchangeable.** `HPSA FTE` is the
full-time-equivalent providers the designation was found to have; `HPSA Designation Population` is
the people it covers; `HPSA Formal Ratio` is the second divided by the first; `HPSA Provider Ratio
Goal` is the threshold — 3,000:1 for primary care, 4,000:1 for dental, 20,000:1 for mental health —
and `HPSA Shortage` is the additional providers it would take to reach the goal. A ratio can be
enormous because the population is large or because the provider count is a fraction, and only the
FTE column separates the two.

**One field name will mislead a join.** `County or County Equivalent Federal Information Processing
Standard Code` holds the three-digit county part alone — `003` — and matches every third county in
the country. The full five-digit code is in `State and County Federal Information Processing
Standard Code`. A filter written against the first field and a county name is how a reader finds
Allen County's designations in the Marshall Islands.

**Score is a rank order, not a rate.** `HPSA Score` runs 0 to 25 for primary care and 0 to 26 for
dental and mental health, and is what the National Health Service Corps places clinicians by. It
folds the ratio together with poverty, infant health and travel time to the nearest source of care,
and two designations with the same score are not two places with the same shortage.
