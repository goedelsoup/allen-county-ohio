---
name: Allen County Engineer — ditch maintenance map service
description: >-
  The county engineer's live map of every petitioned drainage improvement on permanent
  maintenance: 249 line features under 169 petition numbers, and the LiDAR-derived watershed
  each one is assessed against. It is the only source this corpus holds that says which of the
  county's channels somebody dug.
type: api
obtained: true
retrieved: 2026-09-07
ttl_days: 365
location:
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AGOL/AGOL_DitchMaintenance/MapServer?f=json
    description: >-
      Service root. Eight layers, `maxRecordCount` 1000, native spatial reference EPSG:3734
      (NAD83 / Ohio North, US survey feet) — so lengths and areas come back in feet without a
      reprojection.
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AGOL/AGOL_DitchMaintenance/MapServer/0/query?where=1%3D1&outFields=*&returnGeometry=true&outSR=3734&f=geojson
    description: >-
      Layer 0, Petition Ditches — 249 polylines with `PWID` (the petition number), `PWName` and
      `LAYER`. The whole layer returns in one request.
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AGOL/AGOL_DitchMaintenance/MapServer/1/query?where=1%3D1&outFields=*&returnGeometry=true&outSR=3734&f=geojson
    description: >-
      Layer 1, Watersheds — 175 polygons carrying only `LAYER`, of the form `#1321-Lidarws`.
      These are the assessed areas, and they are nested; see below.
used-by:
  - ../corpus/measure/allen-county-petitioned-ditches-2026.yml
  - ../corpus/measure/allen-county-stream-network-2026.yml
  - ../corpus/natural-feature/hog-creek.yml
  - ../corpus/natural-feature/little-ottawa-river.yml
  - ../corpus/natural-feature/ottawa-river.yml
---

**This is a different service from [the county's main GIS](allen-county-gis-rest.md).** That one
is `AllenCountyGIS/MapServer` at the service root; this one sits in the `AGOL` folder, which holds
twenty-eight further services the root listing does not name. The root's `?f=json` returns eight
services and seven *folders*, and this corpus read the services and not the folders.

## What the two layers are

`PWID` is the petition number assigned by the Drainage Engineering Department when the petition
is filed, and it is the join key to everything the office publishes about a project. `PWName` is
the petition's name, which is nearly always **the name of whoever petitioned for it** — a
landowner, a group of them, a township's trustees, a developer, a factory. The 249 line features
carry 169 distinct petition numbers between 1036 and 1348, so a petition is one legal instrument
drawn as several lines; #1198 is fifteen of them. **Two features carry no `PWID`, no `PWName` and
no `LAYER`** — 0.19 miles of line that the file cannot attribute to a petition.

## The watershed polygons cannot be summed

Layer 1's 175 polygons total **403,626 acres** by their own `Shape.STArea()`, in a county of
260,359. They are nested: **136 of the 175 lie ≥99 per cent inside another one**, because a trunk
ditch's assessed watershed contains the watersheds of the laterals that drain into it. Their union
is 253,118 acres and the union clipped to the county is **203,261**. Nothing in the layer says
which polygons are contained in which, and the only field it carries is `LAYER`.

The largest is `#1239-Lidarws` at 137,482 acres — over half the county, and the assessed watershed
of the Ottawa River.

## One polygon has no area

`#1231-Lidarws` returns a ring whose shoelace area is zero. It is one of 175 and it is reported
here because a count of watersheds is not a count of watersheds that bound anything.

## The service does not date itself

There is no edit date, no vintage field and no service description. What fixes it in time is
[the engineer's own annual report](allen-county-engineer-annual-reports.md), whose April 2025
issue states a maintained mileage that this layer's geometry reproduces to about one per cent.
