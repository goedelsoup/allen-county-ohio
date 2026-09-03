---
name: OSHA Injury Tracking Application — establishment 300A summaries
description: >-
  Every American workplace large enough or hazardous enough to owe OSHA an annual injury summary,
  named, addressed and counted, 2016 through 2024. It is the first source this corpus has held that
  says what work does to the people who do it, and the first that reaches inside a private employer's
  own logbook.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://www.osha.gov/Establishment-Specific-Injury-and-Illness-Data
    description: >-
      The index page. Ten annual releases, 2016 through 2025, plus a data dictionary, a users' guide
      and OSHA's own comparison of these figures against the BLS survey.
  - kind: url
    value: https://www.osha.gov/sites/default/files/ITA%20Data%20CY%202016.zip
    description: >-
      Calendar 2016, 12.1 MB zipped, 214,978 rows. The 2017 through 2019 files follow the same
      naming; 2020 is `ITA-Data-CY-2020.zip`, 2021 and 2022 are `ITA-data-cyYYYY.zip`, and 2023
      and 2024 are `ITA_300A_Summary_Data_YYYY_through_MM-DD-YYYY.zip`. Six naming schemes in nine
      years.
  - kind: url
    value: https://www.osha.gov/sites/default/files/ITA_300A_Summary_Data_2024_through_12-31-2025.zip
    description: >-
      Calendar 2024, 23.9 MB zipped, 398,620 rows, the last complete year. The 2025 release is a
      bare 84.6 MB CSV covering filings through 15 March 2026 and is therefore partial.
used-by:
  - ../corpus/measure/allen-county-workplace-injuries-2016-2024.yml
---

**What a row is.** One establishment's Form 300A summary for one calendar year: its name, its
company, its street address, its NAICS code, its annual average employees, its total hours worked,
and the counts of deaths, days-away cases, job-transfer cases, other recordable cases, and the
lost and restricted days behind them. Nine years hold 2,805,767 rows, and 2,801,240 once duplicate submissions of the same
establishment-year are collapsed. [verified] — the nine files,
counted here.

**Who is in it, and who is not.** An establishment owes a 300A submission if it has 250 or more
employees, or 20 to 249 in one of the industries OSHA designates as high-hazard. Everyone else is
outside the file whatever happens to their workers. In Allen County the file reaches 222
establishments in its widest year against the 2,239 private establishments the business register
counts, and 22,622 employees against 44,251 — **one workplace in ten and half the workforce**. Two
of the 222 are Postal Service facilities and the register counts only private employers, so the
comparison is off by two in the file's favour. [verified] — the 2023 file against
[County Business Patterns](census-county-business-patterns.md).

**The column order changes and the header does not.** Through 2022 the file begins
`id, company_name, establishment_name, ein, street_address…`; from 2023 it begins
`id, establishment_name, establishment_id, ein, company_name…`, and the illness columns are
reordered as well. A reader that takes fields by position rather than by name silently swaps the
establishment's name for its company's, and its skin disorders for its poisonings. [verified] —
the nine headers, compared.

## Two fields that cannot be trusted as published

**`total_hours_worked` is typed by the employer and checked by nobody.** In the 2019 file one
Arkansas establishment with seven employees reports **16,831,620,723,179 hours** — about two billion
years of work. It is enough on its own to drive the national recordable rate computed from that
file to **0.016 per 100 full-time workers** against the 4.02 the same file gives once impossible
rows are removed, a factor of 249. Across the nine years, 27,462 of the 2,801,240 establishment-years — one in a
hundred — report more hours per employee than a year contains. [verified] — the nine files, the
arithmetic here, and see
[an impossible value is not an outlier](../decisions/an-impossible-value-is-not-an-outlier.yml).

**`establishment_id` is not a key.** Ford's Lima Engine plant files in all nine years and carries
id `103091` in seven of them, `103091.00` in 2019 — the whole file is written with float formatting
that year — and `454835` in 2018. Across Allen County the id yields 820 distinct values for 1,597
establishment-years and puts **no establishment in all nine years**, while name and street together
put nine there. A panel built on the published id is a panel of strangers. [verified] — the same
files.

**What it is not.** It is not a census of workplace injury: the reporting duty is a regulation and
the file is what compliance with it produced. It is not the BLS Survey of Occupational Injuries and
Illnesses, whose estimates come from a designed sample and carry intervals. It is not a record of
who was hurt — no worker appears in it — and it is not a count of workplace deaths in any place,
because an establishment below the reporting threshold reports nothing at all.
