---
name: American Community Survey, table-based summary file
description: >-
  The Census Bureau's rolling survey of who Americans are — income, poverty, age, race, education,
  housing — published as one pipe-delimited file per table covering every geography in the country.
  It is the first source in this corpus that describes the county's people rather than counting
  them, and the first that publishes a margin of error beside every figure.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/acs/summary_file/2023/table-based-SF/data/5YRData/acsdt5y2023-b19013.dat
    description: >-
      The 2023 five-year file for one table, median household income. Every table follows the same
      path with its own identifier. Four were taken — B19013 income, B17001 poverty, B02001 race,
      B01001 sex by age — plus B01002 median age and B03003 Hispanic origin. Each is 15 to 90 MB.
  - kind: url
    value: https://api.census.gov/data/2023/acs/acs5
    description: >-
      The API for the same data, which now requires a key and returns a Missing Key page without
      one. Not used. The bulk files need no key and are the route this corpus already takes for
      every other Census product.
---

**Its geographies are the ones this corpus already has.** `GEO_ID` is summary level plus FIPS:
`0500000US39003` for Allen County, `1600000US3943554` for Lima, `0600000US39003NNNNN` for each
township, `0400000US39` and `0100000US` for the state and the nation. [verified] The corpus joins on
the same `geoid` property its place nodes carry for the map.

**Ohio's twelve townships and Lima partition the county exactly, and three tables prove it.**
Population 66,381 in the townships plus 35,304 in Lima is 101,685, which is the county row.
Black residents 2,515 plus 8,290 is 10,805, the county row. People below poverty 4,601 plus 8,214
is 12,815, the county row. [verified] Three independent closures, so nothing here is a subset
mistaken for a whole.

**Every figure carries a margin of error and this corpus had never held one before.** Allen County's
median household income is $62,001 ± $2,572; Lima's is $43,370 ± $2,298; Amanda Township's is
$99,148 ± $20,903, an interval of plus or minus twenty-one per cent. [verified] Small geographies
have wide intervals, and a five-year survey of 1,398 people cannot say much narrowly. See
[a survey is not a count](../decisions/a-survey-is-not-a-count.yml).

The consequence is not decorative. Lima's median household income is lower than Perry Township's by
$9,902 against a combined margin of $11,142, so this source **does not establish** that Lima has the
lowest household income in the county. It establishes a gap against Shawnee Township — $47,764
against a combined margin of $6,359 — and that is a different claim.

**Two codes to know.** `-555555555` in a margin column means the estimate is a controlled total with
no sampling error, which is why the county's population carries no interval and its income does.
Suppressed and unavailable values use other negative sentinels; no figure in this corpus is taken
from a negative cell. [verified]

**Six more tables were taken for housing.** B25002 occupancy, B25003 tenure, B25034 year structure
built, B25035 median year built, B25077 median value and B25064 median gross rent. The same thirteen
subdivisions close against the county row again — units 44,697, occupied 40,928, vacant 3,769,
owner-occupied 27,960, and every one of the eleven year-built bands — so the housing tables
partition this county fifteen more times over. [verified]

**Two of its sentinels turn up in the housing tables where they had not before.** Median gross rent
returns `-666666666` for Amanda and Perry townships — the value is unavailable, not zero — and the
margin column beside it reads `-222222222`. [verified] No figure in this corpus is taken from a
negative cell, and the two townships are recorded as having no published median rent rather than a
low one.

**A 2024 file is published and this corpus reads 2023.** Every table has an `acsdt5y2024-` sibling of
almost the same size. It is not a later observation of the same thing: the 2023 file covers
2019–2023 and the 2024 file covers 2020–2024, so they share four of five years and a difference
between them is mostly the two years that changed places. [inference] The corpus stays on 2023 for
now because its income, race, age and housing tables are all that vintage and they close against
each other. Moving means moving all of them.

**Six more tables were taken for the county's people.** B15002 and B15003 educational attainment,
B18101 disability by sex and age, B21001 veteran status by sex and age, and B01001 and B17001 read
again for their universes rather than their contents. Eighteen tables from one year now. [verified]

**Its tables do not all describe the same population, and the file never says so in the data.** The
universe is printed in the table shells and nowhere else: B15003 is taken over the population 25
years and over, B21001 over the civilian population 18 years and over, B18101 over the civilian
noninstitutionalized population, and B17001 over the population for whom poverty status is
determined. In this county those four universes differ by as much as 3,899 people. [verified] — the
shells file against the data; see
[the gap between two universes is a measurement](../decisions/the-gap-between-two-universes-is-a-measurement.yml).

**What else is in it, unread.** Commuting time, language, health insurance, housing cost burden,
mortgage status, place of birth, and the year-by-year five-year files between 2010 and 2022 — those
before 2021 in a different format, since the table-based summary file exists only from then. The
first of them, 2005–2009, has now been taken and is
[catalogued separately](census-acs-summary-file-2009.md).
