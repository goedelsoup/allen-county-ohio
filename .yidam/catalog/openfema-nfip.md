---
name: National Flood Insurance Program claims, policies and repeat-loss properties (OpenFEMA)
description: >-
  Every flood insurance claim paid in a county since 1978, every policy term written in a postal
  area since 2009, and the buildings that have been paid on more than once. It is the only source
  this corpus holds that records where water actually damaged a building, as against
  [where a map says it will](fema-nfhl.md).
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 180
location:
  - kind: url
    value: https://www.fema.gov/api/open/v3/NfipClaims
    description: >-
      Redacted claims, 2.72 million nationally. `$filter=countyCode eq '39003'` returns **259** for
      Allen County, losses dated 1978 to 2023, with 84 fields including the date and cause of loss,
      the rated and current flood zones, the amounts paid, the community and the occupancy type.
  - kind: url
    value: https://www.fema.gov/api/open/v3/NfipPolicies
    description: >-
      Policy terms, 74.3 million nationally and one row per term rather than per building. There is
      **no county field**: `countyCode` does not exist here, `propertyState eq 'OH'` times out into
      a 503, and `startswith(censusGeoid,'39003')` returns a 503 immediately. `reportedZipCode`
      works and answers in about eleven seconds, which is why the policy side of this corpus's
      reading is built on [the county's postal areas](../corpus/measure/allen-county-zip-codes-2020.yml).
  - kind: url
    value: https://www.fema.gov/api/open/v1/NfipMultipleLossProperties
    description: >-
      Buildings paid on twice or more. `$filter=fipsCountyCode eq '39003'` returns **27**, with the
      number of losses, the flood zone, whether the property is still insured and whether it has
      been mitigated.
  - kind: url
    value: https://www.fema.gov/api/open/v1/OpenFemaDataSetFields
    description: >-
      The data dictionary as data. `$filter=openFemaDataSet eq 'NfipClaims'` returns all 84 field
      descriptions, and it is the only place the numeric code lists are written down — the cause of
      damage, the occupancy type, and what each letter of a flood zone means.
used-by:
  - ../corpus/measure/allen-county-flood-insurance-1978-2023.yml
  - ../corpus/measure/allen-county-flood-hazard-2026.yml
---

**It carries a coordinate for every damaged house, and this corpus does not publish one.** Each
claim row has a latitude and longitude rounded to a tenth of a degree, a census tract, a reported
city and a postal code; the repeat-loss file carries a block group. No name appears anywhere in
either. What is taken here is counts, sums and shares by community and by year, and the buildings
are described rather than placed — the same rule this corpus applied to
[a tract page](../decisions/what-a-tract-page-may-be-quoted-for.yml) and to
[the county's donors](../corpus/measure/allen-county-federal-contributions-1980-2024.yml).

**The claim record and the policy record do not cover the same years.** Claims begin in 1978 and
policies in 2009, so any rate that divides one by the other has to be cut to the overlap first, and
every such rate here is. [verified] — both files, their date ranges read. See
[a compulsory denominator is not a voluntary one](../decisions/a-compulsory-denominator-is-not-a-voluntary-one.yml).

**A county code is not a county.** The field's own description says it "may not reflect the
individual county the property is located", and three of Allen County's 259 rows name communities
in Hancock, Auglaize and Fairfield. [verified] — the same file and its dictionary. Three in 259 is
1.2 per cent, and it is reported rather than corrected because there is no rule here that would fix
one row without moving another.

**The v2 endpoints are deprecated and answer anyway.** `FimaNfipClaims` and `FimaNfipPolicies`
return data, frozen as of 1 June 2026, and will be withdrawn on 15 October 2026; the deprecation
notice arrives inside the response's own metadata rather than as a status code. [verified] — the
v2 metadata block. Everything here is taken from v3 or v1 as listed above.
