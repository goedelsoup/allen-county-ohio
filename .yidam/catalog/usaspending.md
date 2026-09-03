---
name: USAspending.gov API
description: >-
  The federal government's own account of every dollar it awards, queryable by county with no key.
  It is the first source in this catalog that can be asked how much the United States spends in
  Allen County, and the first that answers the question twice with figures seven times apart.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 180
location:
  - kind: url
    value: https://api.usaspending.gov/api/v2/search/spending_by_geography/
    description: >-
      POST with `geo_layer: county` and `geo_layer_filters: ["39003"]`. **The `scope` field takes
      either `place_of_performance` or `recipient_location`, and the answer changes by a factor of
      seven.** Taken here for every federal year from 2008 to 2025 and for each of five award-type
      groups, which is 180 requests. The service replies that its searches reach no earlier than
      1 October 2007.
  - kind: url
    value: https://api.usaspending.gov/api/v2/search/spending_by_category/recipient/
    description: >-
      The same filters with `place_of_performance_locations` or `recipient_locations` set to
      `{country: USA, state: OH, county: "003"}`, which is how the two geographies are asked for
      here. Also served for `awarding_agency` and `cfda`, the assistance listing.
  - kind: url
    value: https://api.usaspending.gov/api/v2/search/spending_by_award/
    description: >-
      Award-level rows with description, recipient, sub-agency and period of performance. This is
      where a county-level total becomes a named contract for a named thing.
used-by:
  - ../corpus/measure/federal-money-in-allen-county-2008-2025.yml
  - ../corpus/measure/federal-contracts-in-allen-county-2008-2025.yml
  - ../corpus/site/lima-army-tank-plant.yml
  - ../corpus/site/lima-refinery.yml
---

**Two geographies, one file, and the difference is not small.** Across federal years 2008 to 2025
the contract dollars whose *place of performance* is Allen County come to **$1,088,368,417**; the
contract dollars whose *recipient* is located in Allen County come to **$154,761,289**. Over all
award types the two totals are $11,102,210,910 and $5,634,911,850. [verified] — the geography
endpoint, both scopes, summed here. Neither is wrong and neither is the other's error; see
[a dollar has two addresses](../decisions/a-dollar-has-two-addresses.yml).

**The recipient-location series has a break at federal year 2017 and the other scope is the
control.** By recipient location the county's total goes $51,043,397 in 2016, $365,279,284 in 2017
and $475,829,981 in 2018. By place of performance across the same three years it goes $476,140,899,
$464,299,438 and $520,037,772 — no step at all. Direct payments to individuals began being
attributed to a county in the recipient view, and nothing about the county changed. [verified] —
both scopes, computed here. The recipient series is not usable before 2018; see
[a reporting change has a date and a control](../decisions/a-reporting-change-has-a-date-and-a-control.yml).

**Its award descriptions are the operative text of a contract and read like it.** `TURBINE FUEL,
JP8`. `ENERGY CONSERVATION MEASURES AT JSMC LIMA, OH IGF::OT::IGF`. `(42) MATERIAL SETS FOR THE
CONVERSION OF M1A2 TANKS TO M1A2S CONFIGURATION FOR THE KINGDOM OF SAUDI ARABIA.` One 1996 award's
description is an undelimited string of internal accounting fields —
`199710!2100!0342!AE07 !USA TANK-AUTOMOTIVE & ARMAMENTS !DAAE0790EA001` — so the field is not
uniformly prose and cannot be parsed. [verified] — the award endpoint.

**It names companies and public bodies, and this corpus quotes those.** Every recipient reproduced
from this source in the corpus is a corporation, a government or a public agency. Award data also
carries individual recipients for assistance programmes; those are aggregated here by programme and
by county and no individual is named or counted separately, which is the same line
[the tract books](../decisions/what-crosses-from-the-recorder.yml) are read under. [verified] — the queries run,
which are aggregate for every assistance figure in this corpus.
