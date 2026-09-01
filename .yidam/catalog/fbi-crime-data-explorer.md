---
name: FBI Crime Data Explorer API
description: >-
  The Federal Bureau of Investigation's public API for reported crime, agency by agency and month by
  month. It serves offense and clearance counts beside the agency's own population, and the same
  figures as rates for the state and the nation, so a small city can be put against Ohio and the
  United States without the corpus computing either denominator.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 180
location:
  - kind: url
    value: https://api.usa.gov/crime/fbi/cde/agency/byStateAbbr/OH?API_KEY=DEMO_KEY
    description: >-
      Every reporting agency in Ohio, 863 of them, keyed by the county name they serve. Allen County
      has ten. Each row carries `nibrs_start_date`, which is the date the agency began incident-based
      reporting and the most important field in this source.
  - kind: url
    value: https://api.usa.gov/crime/fbi/cde/summarized/agency/OH0020400/violent-crime?from=01-2015&to=12-2024&API_KEY=DEMO_KEY
    description: >-
      Monthly offenses and clearances for one agency, with Ohio and United States rates alongside.
      Dates are `MM-YYYY` and the service rejects anything else. `violent-crime` and
      `property-crime` are the two summaries read here.
used-by:
  - ../corpus/measure/allen-county-law-enforcement-agencies-2026.yml
  - ../corpus/measure/allen-county-sheriff-offenses-2015-2024.yml
  - ../corpus/measure/lima-crime-2015-2024.yml
---

**Access, and the limit that shaped what this corpus holds.** The API is served through api.data.gov
and accepts `DEMO_KEY` without registration. The demonstration key is rate-limited to a few dozen
requests an hour, and the limit was reached after eight. **So the corpus holds two of Allen County's
ten agencies** — the Lima Police Department and the Allen County Sheriff's Office, which between
them cover 58,631 of the county's 100,866 people. The eight township, village and park agencies are
reachable and were not read. A registered key would fix this in one phase.

**`nibrs_start_date` is the field to read first.** It dates the agency's change from summary
reporting to incident-based reporting, which is a change in what counts as an offense and not only
in how it is filed. Allen County's ten agencies converted between June 2004 and January 2021, and
the two that police most of the county converted last. A series read across an agency's own start
date is two series. See
[a reporting change has a date and a control](../decisions/a-reporting-change-has-a-date-and-a-control.yml).

**Its rates and its actuals are both given, and they disagree about what a year is.** The `rates`
block is monthly, per hundred thousand, so an annual rate is the sum of twelve values and not their
mean. The `actuals` block is counts. The corpus computes agency rates from actuals and population
and takes the state and national rates as the file gives them, which is why those two are the only
figures here it did not compute.

**Its population is not the census's.** The file gives Lima 36,452 for 2020 where
[the census](../corpus/measure/lima-population-2020-census.yml) counts 35,579 — 873 more, or 2.5 per
cent. It is an agency's service population on a different vintage, and the corpus uses it only as
the denominator for that agency's own rate.

**What it will not answer.** Anything about arrests, charges, sentences or who was involved; the
county's two prisons and its jail are a different record entirely. Nor anything before an agency's
own reporting history, which for the Lima Police Department in this file begins in 2015.
