---
name: CDC National Environmental Public Health Tracking Network API
description: >-
  County-level birth outcomes, blood lead, asthma and a dozen other health measures, keyless, as
  annual series back to 2000. It is the source that answers what happens when a child is born in
  Allen County — a question this corpus has had a birth count for and no outcome of any kind.
type: api
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://ephtracking.cdc.gov/apigateway/api/v1/getCoreHolder/{measure}/2/2/39003/{years}/0/0
    description: >-
      The data call. Stratification level 2 is *State x County*, geographic type 2 is county, and
      the geographic item is the five-digit FIPS — `39003`. The year list must be enumerated; there
      is no `ALL`. The reply is a large envelope of which only `tableResult` is ever populated for
      these measures.
  - kind: url
    value: https://ephtracking.cdc.gov/apigateway/api/v1/temporal/{measure}/2/2/39003
    description: >-
      Which years exist for a measure in this county, and the only way to build the year list the
      data call needs. Asked at state grain — `temporal/{measure}/2/2/39` — it returns an empty
      array, which is not the same as an error and is easy to mistake for one.
  - kind: url
    value: https://ephtracking.cdc.gov/apigateway/api/v1/indicators/{contentAreaId}
    description: >-
      The catalogue: content area to indicator to measure. `indicators/8` is Reproductive & Birth
      Outcomes and `measures/{indicator}` lists what can be asked. Adding `/json` to any of these
      paths, which several published examples do, returns HTTP 400.
used-by:
  - ../corpus/measure/allen-county-birth-outcomes-2000-2021.yml
---

**It rate-limits anonymous callers and says so in the body.** A burst of requests returns HTTP 429
with `"Server has serviced too many non-token requests"`. The service offers an API token and this
corpus does not hold one; every retrieval here is spaced and retried with a growing delay, which the
service serves. [verified] — the retrievals. That is politeness toward a public endpoint and not a
way around a gate: nothing here required a token to answer.

**A measure that changes definition is published twice, with an overlap.** *Percent of Low
Birthweight Live Singleton Births* is measure 36 through 2020 and measure 1416 from 2018, and the
three years they share are in both. Reading only one of the pair gives a series that stops in 2020
or starts in 2018; reading both without noticing gives a duplicated year. [verified] — the two
measures. What the overlap is worth is
[its own rule](../decisions/an-overlap-names-what-changed.yml).

**County-level race stratification exists in the schema and returns nothing here.** *State x County
x Race/Ethnicity* is stratification level 13 and is offered for these measures; asked for Allen
County it returns zero rows, as small-count suppression would. [verified] — the same endpoint. So
the Black/white gaps this corpus has measured in
[life expectancy](../corpus/measure/allen-county-life-expectancy-2010-2015.yml) and cancer cannot be
followed into birth outcomes from here.

**Ohio reports no childhood blood lead to it at county grain.** The Annual Blood Lead Levels
indicator has three measures, all of them offered for this county, and every value for every year
from 2017 to 2022 is null. [verified] — the same endpoint, six years by three measures. That is an
absence of reporting rather than an absence of lead, and this corpus states it as such.

**Its numbers are NCHS's, not its own.** Births, birthweights and gestational ages come from the
national vital statistics birth file and the population denominators from the Census Bureau's
estimates; the Tracking Network assembles and publishes them. [verified] — the measure definitions.
A figure taken here is a vital-statistics figure with a second publisher's revisions on it.
