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
retrieved: 2026-09-03
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
    value: https://educationdata.urban.org/api/v1/api-endpoints/
    description: >-
      The endpoint index, which is where the year coverage of each collection is stated. Graduation
      rates stop at 2019 and assessments skip 2019 entirely.
used-by:
  - ../corpus/measure/allen-county-graduation-rates-2010-2019.yml
  - ../corpus/measure/allen-county-test-proficiency-2009-2018.yml
---

**Almost every graduation rate in it is a range, and the range is a disclosure rule rather than an
uncertainty.** Of the 120 all-student district-years this county has, 109 are published as a band —
`90-94`, `95-100`, `50-100` — and 11 as an exact figure. [verified] — the graduation files, counted
here. The true value is inside the band with certainty and nothing in the file says where. This is
not the interval an American Community Survey publishes, and see
[a suppressed range is not a margin](../decisions/a-suppressed-range-is-not-a-margin.yml).

**The width of the band is a function of the size of the cohort, and of nothing else.** Across
Ohio's 7,440 all-student district-years, no district with a graduating cohort under 150 has ever had
an exact rate published; above 300, 99.0 per cent are exact, and the smallest exact cohort in ten
years is 201. [verified] — the same files, Ohio-wide. So the districts a reader can read are the
large ones, and in this county that is one district and sometimes two.

**`grad_rate_midpt` is a convenience column and it invents precision.** 173 of this county's 1,231
graduation rows are published as `50` to `100`, and every one of those 173 carries a midpoint of 75.
[verified] — the same files. A pipeline that reads the midpoint column gets a plausible number for a
cell that says nothing at all.

**The assessment files carry no suppression here at all, because the denominator is seven times
larger.** All 240 of this county's district-year reading and mathematics cells at `grade-99` are
exact: about 9,300 tests a year against about 1,300 graduates. [verified] — the same collection, the
assessment endpoint. One source, two measures, twelve districts, and the difference in legibility is
entirely arithmetic.

**Missing codes are negative and one absence is real.** `-1`, `-2` and `-3` appear where a figure is
missing, suppressed or not applicable and must be excluded rather than summed. [verified] — the same
files. The 2020 assessment year returns rows for every district with no scores in them, because the
tests were not given; that is an absence and not a defect, and 2019 has no assessment file at all.

**Subgroup rows are the same shape and are mostly unreadable.** Every district-year is crossed with
race, disability, economic disadvantage, English learner status, homelessness and foster care.
Outside Lima the economically disadvantaged cohorts run from 3 to 50 pupils, so their rates read
`50-100` in almost every district and year. [verified] — the same files.
