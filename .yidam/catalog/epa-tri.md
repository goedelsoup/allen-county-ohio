---
name: EPA Toxics Release Inventory, via Envirofacts
description: >-
  Every US industrial facility that has reported a listed chemical since 1987, with its address, its
  corporate parent, and one form per chemical per year. It is the only source this corpus has that
  names the county's manufacturers as a set, and the only one that says how long each has been here.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://data.epa.gov/efservice/tri_facility/county_name/ALLEN/state_abbr/OH/CSV
    description: >-
      The facility register. 49 rows for Allen County — the facilities that have ever reported,
      with current name, address, coordinates, closure flag and parent company.
  - kind: url
    value: https://data.epa.gov/efservice/tri_facility/county_name/ALLEN/state_abbr/OH/tri_reporting_form/CSV
    description: >-
      The register joined to the forms. 5,372 rows for Allen County across reporting years 1987 to
      2024. Envirofacts pages at 1,000 rows; append `/rows/0:999` and step.
---

**Twenty facilities reported for 2024 and the register holds forty-nine.** Twenty-eight of the
forty-eight that have ever filed did not file for 2024, and three of those twenty-eight carry the
closure flag. [verified] The flag catches about one departure in nine, so it is not a currency
test and this corpus does not use it as one. What answers *who is here now* is whether a facility
filed a form for the latest year. See
[a register is not a census](../decisions/a-register-is-not-a-census.yml) and
[the reporters](../corpus/measure/allen-county-tri-reporters-1987-2024.yml).

**Its facility attributes are current and its forms are historical, and joining them is a trap.**
`parent_co_name` and `facility_name` live in the register and describe today. Joined to a 1987 form
they print today's owner against a thirty-eight-year-old filing. The Lima refinery's 1987 row reads
`CENOVUS ENERGY INC.`, and [EIA](eia-refinery-capacity.md) has BP operating that refinery until
1997 and Cenovus not appearing until 2022. [verified] Every parent named from this source is a
parent *now*.

**Its coordinates must not be used for containment.** They are stated with an accuracy field, and
the field is 150 metres for the refinery and the Ford engine plant and **11,000 metres** for the
Joint Systems Manufacturing Center. [verified] The refinery's point falls 0.96 miles west of the
one [the county's own address file](allen-county-gis-downloads.md) gives, and the two land in
different jurisdictions. The corpus takes the county's point, which is address-matched by the
authority that maintains the addresses, and does not ask this source where anything is.

**Two records at one address, and the difference is a reporter and not an operator.** The tank plant
appears twice: `U.S. ARMY JSMC GENERAL DYNAMICS LAND SYSTEMS`, parent General Dynamics Corp,
reporting 1987 to 1993; and `U S ARMY JOINT SYSTEMS MANUFACTURING CENTER`, parent US Department of
Defense, reporting 1994 to 2024. Same street address, same coordinates. [verified] Who filed
changed in 1994. Whether who *operated* changed, this source cannot say. [open]

**What else is in it, unread.** Release and transfer quantities per chemical per year — the actual
tonnage, which is what the inventory is for — plus waste management, source reduction and the
chemical identities themselves. This corpus has taken names, parents, addresses and which years each
facility filed.
