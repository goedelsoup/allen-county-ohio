---
name: National Levee Database (USACE)
description: >-
  The Corps of Engineers' inventory of the nation's levee systems — embankments, floodwalls,
  closure structures and the areas they protect. It is in this catalog for a zero: it holds no
  feature of any kind in Allen County, or anywhere in a box of north-west Ohio nine counties wide.
type: api
obtained: true
retrieved: 2026-09-07
ttl_days: 365
location:
  - kind: url
    value: https://geospatial.sec.usace.army.mil/dls/rest/services/NLD/Public/MapServer?f=json
    description: >-
      Eighteen layers. 16 Leveed Areas is the polygon layer; 15 System Routes, 11 Embankments,
      12 Floodwalls and 7 Alignment Lines are the linework. Keyless.
  - kind: url
    value: https://levees.sec.usace.army.mil/data-services/services/
    description: >-
      Where the service URLs are published. The host is `geospatial.sec.usace.army.mil` and not
      `levees.sec.usace.army.mil`; the JSON API advertised at `/api/` answers `not found` to the
      paths a reader would guess.
used-by:
  - ../corpus/measure/allen-county-watershed-boundaries-2026.yml
---

## The query, and its control

Layers 16, 15, 11, 12 and 7 each return `{"count":0}` for the envelope
`-84.42,40.62,-83.86,40.94`, which is Allen County. [verified] — queried here, 7 September 2026.

**A zero from a service is worth nothing without a control**, and this corpus has been wrong that
way before. Widening to `-85.0,38.3,-80.5,42.4` — Ohio and its edges — layer 16 returns **169
leveed areas**, so the query works and the layer is populated in this state. Widening only to
`-85.0,40.2,-83.5,41.5`, which covers nine counties of north-west Ohio, returns **zero** again.

The absence is regional, not a property of Allen County, and it is what
[the flood insurance study](fema-flood-insurance-study.md) says in prose and
[the NFHL](fema-nfhl.md) says by returning no feature on layer 23.

## What the database is scoped to, which is why the zero is not a surprise

The NLD is the inventory behind the USACE Levee Safety Program. It holds levee *systems* — works
with an identified sponsor, an alignment and a leveed area behind them. An artificial bank that
is not a levee system in that sense is not in it: a reservoir embankment, a ditch spoil bank, a
road on fill. Reading the zero as "there is no artificial bank in Allen County" would be reading
a programme's roster as a description of the ground. See
[a presence flag is not an extent](../decisions/a-presence-flag-is-not-an-extent.yml), which is
the same error from the other side.
