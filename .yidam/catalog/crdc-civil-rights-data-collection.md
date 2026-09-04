---
name: Civil Rights Data Collection (U.S. Department of Education, Office for Civil Rights)
description: >-
  What happens to children once they are inside a school building: who is suspended, who is
  expelled, who is referred to the police, who is chronically absent, and who takes the advanced
  courses — school by school and by race, every second year from 2011. It is the first source in
  this corpus that describes what a school does to its pupils rather than how many of them there
  are or what they scored.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://educationdata.urban.org/api/v1/schools/crdc/discipline/2017/disability/race/sex/?fips=39&disability=99&sex=99
    description: >-
      Suspensions, expulsions, law-enforcement referrals and arrests by school and race. The
      `disability=99&sex=99` filter takes the totals rather than the full cross-product and turns
      100,000 rows into 28,744. Collected for 2011, 2013, 2015, 2017, 2020 and 2021.
  - kind: url
    value: https://educationdata.urban.org/api/v1/schools/crdc/chronic-absenteeism/2022/race/sex/?fips=39&sex=99
    description: >-
      Pupils absent fifteen days or more, by school and race. This one collection runs a year
      further than the others, to 2022, and the enrolment collection beside it does not.
  - kind: url
    value: https://educationdata.urban.org/api/v1/schools/crdc/enrollment/2021/race/sex/?fips=39&sex=99
    description: >-
      The collection's own enrolment, which is the only denominator its counts may be divided by.
  - kind: url
    value: https://civilrightsdata.ed.gov/
    description: >-
      The Office for Civil Rights' own site, which renders its data in JavaScript and serves a
      plain client an empty shell. Not a gate — nothing here is withheld — but nothing is reachable
      from it either, which is why the Urban Institute's mirror of the same files is the route.
used-by:
  - ../corpus/measure/allen-county-school-discipline-2011-2021.yml
  - ../corpus/measure/allen-county-chronic-absenteeism-2013-2022.yml
---

**Its 2020 vintage publishes a blank district identifier on every row.** A query filtered on
`leaid` returns nothing at all for this county in 2020 and returns 36 schools in 2021, with no
error and no warning. The school's own identifier, `ncessch`, is populated in every year, and
filtering on it recovers 36 schools and 14,839 pupils for 2020. [verified] — the 2017, 2020 and
2021 enrolment files. See
[a total is checked against one it did not come from](../decisions/a-total-is-checked-against-one-it-did-not-come-from.yml).

**Its enrolment is not the Common Core's enrolment, and the gap is four to five per cent.** For
Allen County the collection counts 16,786 pupils in 2013 where the district files count 17,634,
16,474 in 2017 against 17,107, and 15,461 in 2021 against 16,127. [verified] — this collection
against [the Common Core of Data](nces-common-core-of-data.md). Every rate in this corpus computed
from these counts uses this collection's own enrolment, because a numerator from one collection
over a denominator from another is a fifth of a suspension point wrong before anything is measured.
[inference]

**Negative cells are codes and there are almost none here.** Across 7,168 discipline cells for this
county the only reserved value is `-1`, appearing 576 times and always in the corporal-punishment
column for 2013 and 2015 — a practice Ohio schools do not use, recorded as not applicable rather
than as zero. [verified] — the discipline files, counted here. Everything else is a count.

**The race dimension carries its own total.** Race code 99 is the all-pupils row and sits beside
codes 1 through 9 in the same response, so a sum over the race dimension double-counts exactly.
[verified] — the same files.

**Two of its years are the pandemic and it does not say so.** The 2020 vintage is the 2020–21 school
year and the 2021 vintage is 2021–22; in the first of those most of this county's children were not
in a building for part of the year. A suspension count and an absence count both mean something
different when the school is remote, and nothing in the file marks the change. [inference]

**The districts report it themselves.** Every figure is a district's own answer to a federal
questionnaire, not an audit, and the Office for Civil Rights has published corrections to earlier
collections. The `revised_flag` column marks records the department reissued. [verified] — the
schema.
