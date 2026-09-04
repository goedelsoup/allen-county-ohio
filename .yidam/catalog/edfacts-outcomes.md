---
name: EDFacts district outcomes — graduation rates and assessment results
description: >-
  What the children in a school district actually did: the share of a ninth-grade cohort that
  graduated in four years, and the share of tested pupils scoring proficient in reading and
  mathematics. Districts report to their state, states report to the U.S. Department of Education,
  and the department publishes by local education agency. It is the first source in this corpus
  that describes school outcomes rather than school size or school money.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://educationdata.urban.org/api/v1/school-districts/edfacts/grad-rates/2019/?fips=39
    description: >-
      Adjusted cohort graduation rate by district, one request per year, 2010 to 2019. Ohio is
      6,441 rows in 2019 — every district crossed with every reported subgroup. `cohort_num` is the
      denominator, and `grad_rate_low`, `grad_rate_high` and `grad_rate_midpt` are the answer.
  - kind: url
    value: https://educationdata.urban.org/api/v1/school-districts/edfacts/assessments/2018/grade-99/?fips=39
    description: >-
      Reading and mathematics proficiency by district, one request per year, 2009 to 2018 and 2020.
      The `{grade}` segment takes 3 to 8, 9 for high school, or 99 for all grades together, which is
      the only one that gives a district one comparable figure a year.
  - kind: url
    value: https://educationdata.urban.org/api/v1/schools/edfacts/assessments/2020/grade-99/?fips=39
    description: >-
      The same measure one level down, by school rather than by district — 3,259 Ohio rows in 2018
      and 3,187 in 2020, of which 42 and 41 belong to this county's twelve districts. It is the
      level at which a county school is a single building with a name.
  - kind: url
    value: https://educationdata.urban.org/api/v1/school-districts/edfacts/assessments/2018/grade-99/race/?fips=39
    description: >-
      Proficiency by district and race, and beside it `/special-populations/` for economic
      disadvantage, disability, English-learner status, homelessness, foster care, migrant status
      and military connection. About 5,400 Ohio rows a year for race and 4,900 for the special
      populations, each row a district crossed with one category.
  - kind: url
    value: https://educationdata.urban.org/api/v1/schools/edfacts/grad-rates/2018/?fips=39
    description: >-
      Graduation rates by school. This is the one endpoint in the collection that paginates —
      10,512 Ohio rows in 2018 arriving 10,000 at a time — and the only one where a `next` link
      must be followed.
  - kind: url
    value: https://educationdata.urban.org/api/v1/api-endpoints/
    description: >-
      The endpoint index, which is where the year coverage of each collection is stated. Graduation
      rates stop at 2019 and assessments skip 2019 entirely.
used-by:
  - ../corpus/measure/allen-county-graduation-rates-2010-2019.yml
  - ../corpus/measure/allen-county-test-proficiency-2009-2020.yml
  - ../corpus/measure/allen-county-proficiency-by-school-2018-2020.yml
  - ../corpus/measure/allen-county-proficiency-by-subgroup-2013-2020.yml
---

**Almost every graduation rate in it is a range, and the range is a disclosure rule rather than an
uncertainty.** Of the 120 all-student district-years this county has, 109 are published as a band —
`90-94`, `95-100`, `50-100` — and 11 as an exact figure. [verified] — the graduation files, counted
here. The true value is inside the band with certainty and nothing in the file says where. This is
not the interval an American Community Survey publishes, and see
[a suppressed range is not a margin](../decisions/a-suppressed-range-is-not-a-margin.yml).

**The width of the band is set by two things and this entry had found one of them.** A whole
cohort is published as a number at 201 and as a band at 200; a *subgroup* row is published as a
number at 301 and as a band at 300. [verified] — the same files, Ohio-wide: 37,632 graduation rows
and 64,235 assessment cells, and not one exception in either. Both thresholds are exact and the
distance between them is exactly one hundred children.

**So a group of 250 is a number when it is the district and a range when it is a kind of child.**
The sentence that stood here — that width is a function of cohort size and of nothing else — was
reading only the whole-cohort rows, where it holds. Of the 30,192 graduation rows that name a
subgroup, 29,152 are bands. [verified] — the same files, counted here. The districts a reader can
read are the large ones, and the children a reader can read are the ones not counted as a kind.

**`grad_rate_midpt` is a convenience column and it invents precision.** 173 of this county's 1,231
graduation rows are published as `50` to `100`, and every one of those 173 carries a midpoint of 75.
[verified] — the same files. A pipeline that reads the midpoint column gets a plausible number for a
cell that says nothing at all.

**The assessment file carries no suppression at district level and a great deal at school
level.** All 240 of this county's district-year reading and mathematics cells at `grade-99` are
exact: about 9,300 tests a year against about 1,300 graduates. The same measure over the same
children by school gives 71 numbers, 91 bands and 4 suppressed cells out of 166. [verified] — the
same collection at both levels. One source, one measure, and the difference in legibility is
entirely arithmetic.

**Missing codes are negative and must be excluded rather than summed.** `-1`, `-2` and `-3`
appear where a figure is missing, suppressed or not applicable. [verified] — the same files.

**The 2020 assessment year is not empty, and this entry said it was.** Its rows carry
`read_test_pct_prof_low` and `read_test_pct_prof_high` as null with the proficiency figure beside
them populated: 923 of Ohio's 924 district rows have a positive count of valid tests and 909 have a
reading figure, and only 242 carry a band at all. [verified] — the same endpoint, re-read on
4 September 2026. A reader that took the band columns for the data found an empty year.

**The `year` field is the fall of the school year, which is why 2019 is the year that is missing.**
2019 is the 2019–20 school year, whose spring tests were not given; 2020 is 2020–21, whose spring
tests were. The graduation file, which measures a cohort rather than a test day, runs through 2019
with no gap in it. [inference] — the collection's own coverage, and the same convention
[the Common Core of Data](nces-common-core-of-data.md) uses for its district directory.

**Subgroup rows are the same shape and are mostly unreadable.** Every district-year is crossed with
race, disability, economic disadvantage, English learner status, homelessness and foster care.
Outside Lima the economically disadvantaged cohorts run from 3 to 50 pupils, so their rates read
`50-100` in almost every district and year. [verified] — the same files.

**The assessment file's subgroups are readable where the graduation file's are not.** Five of this
county's twelve districts test more than 300 economically disadvantaged pupils and so are published
as numbers — Bath 71 per cent proficient in reading in 2018, Shawnee 68, Perry 62, Elida 58, Lima
42. The same five districts' economically disadvantaged *graduating cohorts* run from 3 to 50.
[verified] — the same collection, both endpoints. Whether a kind of child can be seen at all depends
on which of two measures is being asked for.
