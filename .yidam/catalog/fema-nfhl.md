---
name: National Flood Hazard Layer (FEMA)
description: >-
  The effective flood insurance rate maps as a queryable map service — every flood zone polygon,
  floodway, base flood elevation and printed panel FEMA has issued for a county. It is the first
  source in this catalog that says which ground in Allen County is expected to flood, and the first
  federal map of the county that is a regulatory instrument rather than a description.
type: api
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer
    description: >-
      Thirty-two layers. Layer **28, Flood Hazard Zones**, is the one that carries the polygons;
      layer 3 is FIRM Panels, 27 is Flood Hazard Boundaries, 23 is Levees and 16 is Base Flood
      Elevations. Everything here is keyed on `DFIRM_ID`, which for this county is `39003C`.
  - kind: url
    value: https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/28/query
    description: >-
      `where=DFIRM_ID='39003C'` returns **1,625 polygons** for Allen County. The useful fields are
      `FLD_ZONE`, `ZONE_SUBTY`, `SFHA_TF` — the flag that says whether a polygon is in the special
      flood hazard area — `STATIC_BFE` and `DEPTH`. Ask for `outSR=26916` and areas can be summed
      without a further projection step.
  - kind: url
    value: https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/3/query
    description: >-
      The printed panels. Allen County has 48, of which 43 are "Countywide, Panel Printed" and 5
      are "Countywide, Not Printed", at scales of 1:6,000, 1:12,000 and 1:24,000.
used-by:
  - ../corpus/measure/allen-county-flood-hazard-2026.yml
  - ../corpus/measure/allen-county-flood-insurance-1978-2023.yml
---

**The county's map is three maps.** Thirty-seven of the 48 panels took effect on **20 June 2024**,
ten on **2 May 2013** and one on **4 May 2015**. [verified] — layer 3, `EFF_DATE` read for every
panel. A flood zone in this county is therefore as of one of three dates, and a claim from 2011 was
rated against a map that no longer exists.

**`SFHA_TF` is the field that matters and `FLD_ZONE` is not enough.** Zone X appears twice in this
county under two subtypes — `AREA OF MINIMAL FLOOD HAZARD` and `0.2 PCT ANNUAL CHANCE FLOOD HAZARD`
— and only the flag distinguishes the ground outside all mapped hazard from the ground inside the
five-hundred-year band. [verified] — the same source, its own schema and values.

**Large geometry pages fail without an error message.** A request for 500 polygons with geometry
returns HTTP 000 and "connection reset by peer" perhaps half the time; 250 at a time with a retry
succeeds. The failure is at the transport layer, so a client that checks only the status code of
what it received sees nothing wrong and quietly holds a short file. [verified] — the seven pages
taken here, three of which needed a second or third attempt.

**What it does not carry.** No population, no buildings, no addresses, no depth of past flooding and
no damage. It is a statement about ground. Counting the people on that ground means overlaying it
on something else, and the finest thing this corpus holds is the census block — see
[the floodplain](../corpus/measure/allen-county-flood-hazard-2026.yml), which brackets rather than
estimates for that reason.
