---
name: Small Area Income and Poverty Estimates — school districts
description: >-
  The Census Bureau's modelled estimate of how many children of school age are in poverty in every
  school district in the country, produced annually for the federal formula that distributes Title I
  money. It is a model rather than a survey or a count, which makes it the third kind of number this
  corpus holds.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/saipe/datasets/2023/2023-school-districts/ussd23.xls
    description: >-
      2023 estimates for every school district: total population, population aged 5 to 17, and the
      number of those aged 5 to 17 in poverty who are related to the householder.
---

**Three fields and a modelled one.** The poverty count is not tabulated from a survey and not
counted from a census: it is estimated by a model that borrows strength from tax returns, food
assistance records and the American Community Survey, because no annual instrument samples a
district of five hundred children well enough to publish a rate from it directly. [verified] The
corpus has held a count, and it has held a survey with a margin —
[a survey is not a count](../decisions/a-survey-is-not-a-count.yml). This is neither.

**It publishes no interval at this geography.** The state and county SAIPE files carry 90 per cent
confidence bounds; the school district file gives three integers and nothing beside them.
[verified] So a district-level poverty rate from this source cannot be ranked against another
district the way [the survey's](census-acs-summary-file.md) figures can be, and this corpus states
differences here only where they are large.

**Its denominator is the wrong one for a school.** "Population aged 5 to 17" is everyone that age
living in the district's territory — including children in private, parochial, charter and home
schooling, and excluding the district's own eighteen-year-olds. It is not enrolment and cannot be
divided into an enrolment figure to yield anything meaningful. [verified]

**It joins on the same district identifier as everything else**, given as a five-digit LEA code
that the corpus prefixes with the state FIPS to match `fips_code`. [verified]

**What else is in it, unread.** Every district in the country, and the same file back to the
mid-1990s. This corpus reads twelve districts in one year.
