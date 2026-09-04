---
name: Incarceration Trends (Vera Institute of Justice)
description: >-
  Jail and prison figures for every county in the United States — jail population, capacity,
  admissions, pretrial and sentenced custody from 1970, and the number of people a county's courts
  have sent to state prison from 1983. It is the first source in this catalog that counts the
  people this county locks up, as against
  [the people locked up inside it](census-2020-redistricting-file.md).
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://raw.githubusercontent.com/vera-institute/incarceration-trends/main/incarceration_trends_county.csv
    description: >-
      The county file, 61.7 MB, 164 columns, one row per county-year. Allen County has **43 rows**,
      1970 to 2023. Filter on `county_fips = 39003`; there is no state-and-county pair to join.
  - kind: url
    value: https://raw.githubusercontent.com/vera-institute/incarceration-trends/main/incarceration_trends_state.csv
    description: >-
      The state file on the same columns, which is what makes a county figure comparable to
      anything. Ohio runs 1970 to 2024.
  - kind: url
    value: https://raw.githubusercontent.com/vera-institute/incarceration-trends/main/Incarceration%20Trends%20Codebook%2003-2026.pdf
    description: >-
      The March 2026 codebook, and the only place the definitions are written down — what a jail
      population is measured on, why the pretrial count is not directly comparable to it, and that
      the prison figures are counted by county of commitment.
used-by:
  - ../corpus/measure/allen-county-jail-1970-2023.yml
  - ../corpus/measure/allen-county-in-state-prison-1983-2019.yml
---

**Its two families of numbers are about two different populations, and the county line means
opposite things in each.** The jail figures count people held in Allen County's jail, wherever they
are from. The prison figures count people sentenced from Allen County, wherever they are held — the
underlying collection is the National Corrections Reporting Program, which records a county of
commitment. [verified] — the codebook, its prison section. A reader who adds the two has added a
place to a jurisdiction; see
[located here is not of here](../decisions/located-here-is-not-of-here.yml).

**The jail total and the pretrial count are not measured the same way.** The total is BJS's average
daily population; the pretrial and race breakdowns are single-day counts taken at the end of June,
and the codebook says in terms that they "do not necessarily sum to" the total. In this county's
rows they nevertheless sum to it exactly, which means Vera has apportioned them — so a pretrial
*share* computed here is a share of an apportionment and not two independent measurements.
[verified] — the codebook against the file.

**Rates are per 100,000 residents aged 15 to 64**, not per 100,000 people. The denominator is in
the file as `total_pop_15to64`, which is what makes the published rates checkable and what makes
them incomparable with any rate computed over a whole population. [verified] — the codebook.

**The jail series is a sample, not a census, in most years.** BJS runs a full Census of Jails every
five to eight years and an Annual Survey of about a third of jails in between. Allen County has
jail figures for 1970, 1978, 1983, 1988, 1993 and then every year from 1999; the gaps are years
the survey did not reach it, not years without a jail. [verified] — the codebook's project history
against this county's rows.

**Two of this county's rows should not be used and the file gives no sign of it.** The year 2000
records a jail population of 23, of whom 0 are men and 23 are women, against 224 in 1999 and 229 in
2001. The year 2020 records 3,614 jail admissions against 1,268 the year before and 585 the year
after, in the twelve months the population fell by half. [verified] — the file. Neither is flagged,
neither is null, and both would pass any range check written against the national distribution.
