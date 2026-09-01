---
name: EPA Safe Drinking Water Information System (Envirofacts)
description: >-
  The Environmental Protection Agency's register of every public water system in the country — who
  runs it, how many people it serves, where its water comes from, and every violation of a drinking
  water rule recorded against it. Served over Envirofacts as plain REST with no key.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 180
location:
  - kind: url
    value: https://data.epa.gov/efservice/WATER_SYSTEM/PWSID/BEGINNING/OH02/JSON
    description: >-
      Every water system whose identifier begins `OH02`, which is Ohio's prefix for Allen County —
      279 rows, of which 41 are active. `BEGINNING` is the path segment that turns an exact match
      into a prefix match and is the only reliable way to select a county here.
  - kind: url
    value: https://data.epa.gov/efservice/WATER_SYSTEM_FACILITY/PWSID/OH0200811/JSON
    description: >-
      One system's intakes, wells, reservoirs, treatment plants and purchased connections. This is
      where a system's water actually comes from; the `gw_sw_code` on the system record is a summary
      and can be misleading, as it is for Bluffton.
  - kind: url
    value: https://data.epa.gov/efservice/VIOLATION/PWSID/BEGINNING/OH02/JSON
    description: >-
      197 rows for the county, 1984 to 2026, with the measured value, the standard it exceeded, the
      compliance period, and the date the system returned to compliance.
used-by:
  - ../corpus/measure/allen-county-water-systems-2026.yml
  - ../corpus/measure/allen-county-water-violations-1984-2026.yml
---

**Its three system types are not three sizes.** `CWS` is a community water system — year-round
service to residents. `NTNCWS` is non-transient non-community: a school or a workplace with its own
well serving the same people daily. `TNCWS` is transient non-community: a campground, a rest area, a
tavern. Allen County has eight active community systems and thirty-three of the other two kinds, and
counting all forty-one as "the county's water systems" would be a category error.

**`gw_sw_code` is a summary and the facility table is the fact.** Bluffton is coded `SW` and every
source it owns is a well; the surface water is bought from Ottawa Village in Putnam County. Elida is
coded `SW` and owns no source at all — its one supply facility is a consecutive connection from Lima
City. Read the facilities before saying where a town's water comes from.

**Violations are mostly paperwork and the file says which.** `is_health_based_ind` separates the
twenty violations in this county that concern the water from the hundred and seventy-seven that
concern monitoring, reporting or record-keeping. Every health-based violation here is either an
`MCL` — a measured exceedance of a standard — or a `TT`, a failure of a required treatment step.

**`rtc_date` is the field that turns rows into problems.** Ten of Bluffton's violations, one per
quarter from October 2013 to March 2016, share a single return-to-compliance date. They are one
problem lasting two and a half years. See
[the file carries its own grouping key](../decisions/the-file-carries-its-own-grouping-key.yml).

**What it will not tell you.** What a contaminant code means. `REF_CODE_VALUES`, the reference table
that would resolve `2950` to a substance name, answers "the table is not available" to every query
tried. The violation rows carry the measured value, its unit and the standard, so the corpus can say
by how much a system exceeded a limit without being able to say what the limit is on.
