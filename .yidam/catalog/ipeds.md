---
name: Integrated Postsecondary Education Data System (IPEDS)
description: >-
  The federal census of American higher education — every institution that takes Title IV student
  aid, surveyed annually on who it is, who enrols, what it awards, what it charges and where its
  students come from. It gave this corpus the five post-secondary institutions in Allen County it
  did not have and the thirteen-year enrolment series behind them.
type: dataset
obtained: true
retrieved: 2026-08-31
location:
  - kind: url
    value: https://nces.ed.gov/ipeds/datacenter/data/HD2023.zip
    description: >-
      Directory, 2023-24. One row per institution, 74 fields — name, address, coordinates, county
      FIPS, control, level, highest degree offered, system parent, web address. Filtering
      `COUNTYCD` to 39003 returns the six institutions in Allen County.
  - kind: url
    value: https://nces.ed.gov/ipeds/datacenter/data/HD2023_Dict.zip
    description: >-
      The directory's data dictionary as an xlsx. Sheet 4 is the code-value table for every coded
      field, which is the only way to read `CONTROL`, `ICLEVEL`, `SECTOR`, `HDEGOFR1` and `LOCALE`
      without guessing.
  - kind: url
    value: https://nces.ed.gov/ipeds/datacenter/data/EFFY2023.zip
    description: >-
      Twelve-month enrolment. Level 1 is all students. Available back to at least 2010 by
      substituting the year, which is how the series in this corpus was built.
  - kind: url
    value: https://nces.ed.gov/ipeds/datacenter/data/DRVEF2023.zip
    description: Derived enrolment — autumn headcount, full-time equivalent, full and part time, undergraduate and graduate.
  - kind: url
    value: https://nces.ed.gov/ipeds/datacenter/data/C2023_A.zip
    description: >-
      Completions by field and award level, 62 MB unpacked. One row per institution, CIP code,
      award level and major number. `CIPCODE` 99 rows are institution totals and must be excluded
      before summing, or every figure doubles.
  - kind: url
    value: https://nces.ed.gov/ipeds/datacenter/data/IC2023.zip
    description: Institutional characteristics, including `ROOM` and `ROOMCAP` — whether the institution provides housing, and how many beds.
  - kind: url
    value: https://nces.ed.gov/ipeds/datacenter/data/IC2023_AY.zip
    description: Academic-year charges — tuition and fees by residency, room and board.
  - kind: url
    value: https://nces.ed.gov/ipeds/datacenter/data/EF2023C.zip
    description: >-
      Residence of first-time students, by state. `EFCSTATE` 99 is the total and 39 is Ohio; 58 is
      a sub-total line and not a state, which is easy to read as one.
used-by:
  - ../corpus/measure/allen-county-college-enrollment-2010-2023.yml
  - ../corpus/measure/allen-county-group-quarters-2020.yml
  - ../corpus/measure/allen-county-higher-education-2023.yml
  - ../corpus/organization/apollo-career-center.yml
  - ../corpus/organization/bluffton-university.yml
  - ../corpus/organization/james-a-rhodes-state-college.yml
  - ../corpus/organization/ohio-state-beauty-academy.yml
  - ../corpus/organization/ohio-state-university-at-lima.yml
  - ../corpus/organization/university-of-northwestern-ohio.yml
  - ../corpus/place/bath-township.yml
  - ../corpus/question/who-lives-in-the-county-without-housing.yml
---

**What it is.** A collection of about a dozen annual surveys that every institution participating in
federal student aid is required to complete, published as flat CSVs with no key and no rate limit.
It is to higher education what QCEW is to employment: a mandatory census rather than a sample, with
the same consequence — the universe is defined by a programme's eligibility rule, so an institution
outside Title IV is outside the file.

**What it gave this corpus.** Six institutions in Allen County, of which the corpus had one:

    UNITID  institution                         control          highest degree
    203678  James A. Rhodes State College       public           bachelor's
    204486  University of Northwestern Ohio     private NFP      master's
    204671  Ohio State University-Lima Campus   public           bachelor's
    201371  Bluffton University                 private NFP      master's
    201034  Apollo Career Center                public, <2 yr    non-degree
    204714  Ohio State Beauty Academy           private FP       non-degree

Two of those addresses are the same — Rhodes State and Ohio State-Lima are both at 4240 Campus
Drive — and two more fall in the same census block, the Ohio State Beauty Academy standing where
the University of Northwestern Ohio does and recorded in the file as a member of its system.

**Four traps, all met.**

  - **`CIPCODE` 99 is the institution total**, carried in the same file as the per-field rows. Sum
    without excluding it and every field figure is exactly doubled, which is invisible because the
    proportions stay right.
  - **The award-level code set is wider than it looks.** Alongside the familiar 1 to 8, recent
    files use **20** for certificates of under twelve weeks and **21** for twelve weeks to a year.
    Those two are 38 per cent of everything Allen County awards, and a tally that filters on the
    old codes silently drops two of the county's six institutions almost entirely.
  - **The twelve-month enrolment file renamed its level column.** Before 2020 it is `EFFYLEV`; from
    2020 it is `EFFYALEV`, with `EFFYLEV` retained beside it carrying different codes. Reading the
    old name across the whole period returns zero for every institution in the later years, which
    looks exactly like a set of closures.
  - **Values are space-padded.** `' 1'` is not `'1'`, and the failure is silent.

**Autumn headcount and twelve-month headcount are different questions.** Apollo Career Center is
527 students in the autumn and 1,439 across the year, because an adult career centre enrols in
cohorts rather than in terms. Either figure is right; a table that mixes them is not.

**What else is in it, unread.** The staff and salary surveys, which would say who teaches in this
county and what they are paid; the finance survey, which would give each institution's revenue and
its sources; graduation and retention rates; net price by family income band; and the whole of the
admissions survey. The College Scorecard, a separate federal publication built partly on this one,
carries post-enrolment earnings by institution and by field of study, which is the natural next
question after "what does this county award".
