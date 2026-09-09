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
  - ../corpus/event/the-flood-of-5-7-february-2008.yml
  - ../corpus/event/the-flood-of-8-11-march-2009.yml
  - ../corpus/event/the-flood-of-13-15-july-1992.yml
  - ../corpus/measure/the-largest-river-days-in-a-hundred-years.yml
  - ../corpus/measure/the-four-files-that-can-see-a-flood.yml
  - ../corpus/event/the-flood-of-27-february-1997.yml
  - ../corpus/event/the-flood-of-17-may-2019.yml
  - ../corpus/event/the-flood-of-14-april-1979.yml
  - ../corpus/measure/the-flood-studys-flood-history.yml
  - ../corpus/event/the-flood-of-14-march-1978.yml
  - ../corpus/measure/the-twenty-wettest-days-against-the-river.yml
  - ../corpus/event/the-flood-of-13-14-june-1981.yml
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

**Its cause-of-loss codes are not contemporary with its rows.** The dictionary served today reads
code `1` as *tidal water overflow*; in this landlocked county it is on twelve of thirteen claims
filed in the 1970s, fourteen in the 1980s, and one in the thirty-six years since — while `2`, stream
or river overflow, does not appear at all until 1980 and then never stops. [verified] — the claims
file, `countyCode` 39003, tabulated by decade here. A meaning does not migrate that way and a code
list does, so the early rows were coded under a scheme this API no longer serves. Counts and dollar
sums are unaffected; only the glosses are. `occupancyType` has the same shape declared openly — a
second, two-digit scheme for Risk Rating 2.0 sitting beside the first. See
[a code list is dated](../decisions/a-code-list-is-dated-and-the-rows-are-older.yml).

**`reportedZipCode` is populated where `reportedCity` is not**, and it is the finest geography this
file gives Allen County. Every one of the 259 rows reads "Currently Unavailable" for the city; every
one carries a postal code. Cut against
[the county's postal areas](../corpus/measure/allen-county-zip-codes-2020.yml) it located the flood
of June 1981 to Delphos and Bluffton — the county's two ends — where the node written from this file
in an earlier phase had reported that it gave no location at all. [verified] — the same file.

**The v2 endpoints are deprecated and answer anyway.** `FimaNfipClaims` and `FimaNfipPolicies`
return data, frozen as of 1 June 2026, and will be withdrawn on 15 October 2026; the deprecation
notice arrives inside the response's own metadata rather than as a status code. [verified] — the
v2 metadata block. Everything here is taken from v3 or v1 as listed above.

**`nfipCommunityName` is the community a claim was rated in when it was filed, and the community
status book is a register of who participates now.** The two disagree, and the disagreement is
informative rather than a defect: this county's 259 claims name one paid at **Fort Shawnee** on
11 March 1982 under community number 390611, and Fort Shawnee is in no current register because its
electors abolished the village in 2012. A corpus that reads the status book as a history will
conclude that a place never joined the programme when it did. See
[a blank field may be a value](../decisions/a-blank-field-may-be-a-value.yml).

**Four of the 259 rows name a community outside this county** — Findlay, Fairfield County, Auglaize
County and Shawnee — while carrying `countyCode` 39003. The rating community is not the location.
[verified] — the file itself.
