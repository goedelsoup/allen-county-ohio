---
name: Civil Rights Data Collection (U.S. Department of Education, Office for Civil Rights)
description: >-
  What happens to children once they are inside a school building: who is suspended, who is
  expelled, who is referred to the police, who is chronically absent, and who takes the advanced
  courses — school by school and by race, every second year from 2011. It is the first source in
  this corpus that describes what a school does to its pupils rather than how many of them there
  are or what they scored. It is also the only one that records what a school **does not offer**,
  and records it as an answer rather than as a hole.
type: dataset
obtained: true
retrieved: 2026-09-05
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
    value: https://educationdata.urban.org/api/v1/schools/crdc/offerings/2021/?fips=39
    description: >-
      The course list: whether a school offers Advanced Placement, whether it has a gifted
      programme, whether it runs dual enrolment, and how many classes of algebra I, algebra II,
      geometry, advanced mathematics, calculus, biology, chemistry and physics it holds. Through
      2017 it also carries how many of those classes are taught by a teacher certified in the
      subject; the 2020 and 2021 vintages return `-1` in every one of those eight columns.
  - kind: url
    value: https://educationdata.urban.org/api/v1/schools/crdc/math-and-science/2021/race/sex/?fips=39&sex=99
    description: >-
      Pupils enrolled in each of seven mathematics and science courses, by school and race. Its
      sibling endpoints `ap-ib-enrollment`, `dual-enrollment`, `algebra1` and
      `sat-act-participation` have the same shape and the same race dimension.
  - kind: url
    value: https://educationdata.urban.org/api/v1/schools/crdc/teachers-staff/2021/?fips=39
    description: >-
      One row per school: teachers, certified and uncertified, first- and second-year teachers,
      teachers absent more than ten days, and the counsellors, nurses, psychologists, social
      workers, security guards and sworn law-enforcement officers assigned to the building. The
      four teacher-experience and absence columns are collected through 2017 and return `-1`
      afterwards.
  - kind: url
    value: https://educationdata.urban.org/api/v1/api-endpoints/
    description: >-
      The endpoint index, which is the only place the collection's fifty-two tables and the years
      each was collected are written down in a machine-readable form.
  - kind: url
    value: https://civilrightsdata.ed.gov/
    description: >-
      The Office for Civil Rights' own site, which renders its data in JavaScript and serves a
      plain client an empty shell. Not a gate — nothing here is withheld — but nothing is reachable
      from it either, which is why the Urban Institute's mirror of the same files is the route.
used-by:
  - ../corpus/measure/allen-county-school-discipline-2011-2021.yml
  - ../corpus/measure/allen-county-chronic-absenteeism-2013-2022.yml
  - ../corpus/measure/allen-county-course-offerings-2013-2021.yml
  - ../corpus/measure/allen-county-advanced-coursework-2021.yml
  - ../corpus/measure/allen-county-dual-enrollment-2017-2021.yml
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

**In the coursework files `-2` is the school's own answer and it is the most informative cell in
them.** The questionnaire gates: a school that answers *no* to *do you offer Advanced Placement* is
never asked how many pupils are enrolled in one, and the enrolment column is filled with `-2` rather
than with zero. Across Allen County's ten high schools in 2021 the Advanced Placement enrolment
column is `-2` eight times and a number twice. [verified] — the 2021 offerings and AP files, read
together. A reader who drops the reserved codes as missing data loses the finding; a reader who
reads them as zeros invents ten schools that offer the course and enrol nobody. See
[a course that is not offered is not a course refused](../decisions/a-course-that-is-not-offered-is-not-a-course-refused.yml).

**The class-count question changed between 2015 and 2017 and nothing in the file says so.** Lima
Senior High School reports 1 physics class in 2015 and 25 in 2017; the same jump appears at Elida
(2 to 15), Shawnee (5 to 14) and Bluffton (2 to 7), and all ten of this county's high schools move
in the same direction across that one boundary and across no other. [verified] — the
2013, 2015, 2017, 2020 and 2021 offerings files for this county's ten high schools. The earlier
vintages are counting something else — most plausibly distinct courses where the later ones count
sections — so this corpus reads the class counts as a series from 2017 and reads the yes-or-no
indicators, which did not change, from 2013. [inference]

**Two of its columns were retired without notice.** `num_taught_certified_*`, which is the only
federal measure of whether the person teaching calculus is certified to teach it, returns a figure
for 2011 through 2017 and `-1` for every school in 2020 and 2021. The teacher-experience and
teacher-absence columns in `teachers-staff` do the same. [verified] — the 2017 and 2021 files,
compared column by column. Anything this corpus says about who is in front of a mathematics class
is therefore about 2017 and stops there.

**Three of this county's cells are a zero where the file had a code for the alternative.** Bath High
School reports 199 pupils in dual enrolment in 2017 and 0 in 2021, on a row whose own indicator says
the school offers it; Allen East and Apollo Career Center each report 0 pupils sitting the SAT or
ACT in 2021, from schools of 326 and 834 with twelfth grades. [verified] — the 2017 and 2021 dual
enrolment and SAT files. None of the three is coded `-1` or `-2`, so the file is asserting them.
See [a zero is not a blank](../decisions/a-zero-is-not-a-blank.yml); this corpus reports them and
uses none of them in a rate.

**One district reports twenty-five school psychologists in an elementary school.** Shawnee Local
gives 25.0 psychologist FTE at Maplewood Elementary, a school of 328 children, and 25.0 again at
Shawnee High School, against 0.2 at its other two buildings — 50.4 of the county's 58.8 reported
psychologist FTE, in a county whose psychologists are employed by the educational service center. [verified] — the 2021 `teachers-staff` file against
[the staffing series](../corpus/measure/allen-county-school-staffing-1992-2024.yml). It is a
decimal-point slip of the kind
[an impossible value is not an outlier](../decisions/an-impossible-value-is-not-an-outlier.yml)
governs, and no support-staff figure in this corpus is computed across it.

**Its high-school enrolment is 1.9 per cent above the district files' for the same ten buildings.**
The collection counts 4,836 pupils in Allen County's ten high schools in 2021–22 where the Common
Core counts 4,747, and the whole of the difference is at Apollo Career Center — 834 against 668 —
which enrols pupils who are also counted by the district that sends them. [verified] — the 2021
enrolment file against [the Common Core of Data](nces-common-core-of-data.md). Every course-taking
rate in this corpus uses the collection's own denominator, which is the rule the paragraph above
sets and the reason it exists.
