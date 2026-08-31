---
name: Annual Survey of School System Finances (F-33)
description: >-
  Every public school district in the United States, once a year, with its enrolment and every
  dollar it took in and spent — by source, by function, and per pupil. It is the first source in
  this corpus that says what a unit of local government costs, and the first that measures the
  county's schools as anything but a boundary.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/school-finances/tables/2023/secondary-education-finance/elsec23t.xlsx
    description: >-
      Fiscal 2023, every district in the country, 71 columns. Enrolment, revenue split federal,
      state and local, property tax, current spending, and per-pupil totals already computed.
  - kind: url
    value: https://www2.census.gov/programs-surveys/school-finances/tables/2010/secondary-education-finance/elsec10t.xls
    description: >-
      The same for fiscal 2010, and for every year between. All fourteen were taken; 2010 through
      2022 are `.xls` and 2023 is `.xlsx`, and each is about eight megabytes.
---

**Fourteen years, read annually rather than sampled.** The corpus adopted that rule after reading
County Business Patterns at five-year steps and broke it on the next annual source it met; see
[a rule is not a habit](../decisions/a-rule-is-not-a-habit.yml). This source was read whole on
first contact — 2010 through 2023, no gaps — and the enrolment series it produces has a shape that
five-year steps would have flattened. [verified]

**It joins on `NCESID`, which is the identifier this corpus's school district nodes already carry.**
`fips_code: "3904579"` on Shawnee Local is the same seven digits. [verified] Twelve of twelve
districts matched in all fourteen years.

**The column set changed between 2021 and 2022** — 66 columns become 72, then 71 — and the names
this corpus uses are in all three layouts: `ENROLL`, `TOTALREV`, `TFEDREV`, `TSTREV`, `TLOCREV`,
`LOCRPROP`, `TCURSPND`, `PPCSTOT`, and the three percentage shares. [verified] Reading by column
name rather than position is what makes fourteen files one series.

**Dollars are nominal and the file deflates nothing.** Per-pupil spending in this county roughly
doubles between 2010 and 2023, and about half of that is price. [inference] The corpus reports
*ratios between districts in one year* where it wants a comparison, because a ratio needs no
deflator, and says "nominal" wherever it quotes a level across years.

**What it does not carry.** Nothing about pupils but their number: no attainment, no attendance, no
graduation, no staffing detail beyond salaries and benefits in aggregate, and no building. It is a
finance survey, and a district that spends more per pupil is not thereby doing better or worse by
one.

**What else is in it, unread.** Instruction against support spending, salaries against benefits,
capital outlay, debt outstanding, long-term debt issued and retired, and the same for every state
and district in the country. This corpus reads about eleven of seventy-one columns for twelve
districts.
