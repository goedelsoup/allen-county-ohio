---
name: Allen County GIS — Ottawa River map service
description: >-
  A second county map service, built for the Ottawa River rather than for the county's business:
  thirty-six annotated points along the river, Ohio EPA's aquatic-life-use segments, and sixteen
  named subwatersheds. It is where the county's low-head dams are enumerated.
type: api
obtained: true
retrieved: 2026-09-07
ttl_days: 365
location:
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AGOL/OttawaRiver/MapServer?f=json
    description: Six layers, `maxRecordCount` 1000.
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AGOL/OttawaRiver/MapServer/0/query?where=1%3D1&outFields=OBJECTID,AreaName,Lat,Long,Comment&returnGeometry=false&f=json
    description: >-
      Layer 0, Points of Interest — 36 rows of `AreaName`, `Lat`, `Long` and a free-text
      `Comment`. Seven names end in "Lowhead Dam"; an eighth is "Remnant Lowhead Dam in OR near
      Lost Creek Reservoir".
  - kind: url
    value: https://gis.allencountyohio.com/arcgis/rest/services/AGOL/OttawaRiver/MapServer/2/query?where=1%3D1&outFields=*&returnGeometry=true&outSR=3734&f=geojson
    description: >-
      Layer 2, Aquatic Life Use Attainment — 41 segments, 244.84 miles, carrying `StreamNama`
      (the field is misspelled) and `AttainType` of Full, Partial or Non.
used-by:
  - ../corpus/measure/allen-county-dams-2026.yml
  - ../corpus/natural-feature/little-ottawa-river.yml
---

**It is not a register and it does not date itself.** Layer 0 is called *Points of Interest*, and
its rows are what somebody walking the river thought worth a pin: outfalls, wastewater plants,
a drinking-water intake, two school groups kick-seining for macroinvertebrates, a two-stage ditch,
a dewatered reach, and the dams. **The Allentown Lowhead Dam is one of the thirty-six and it was
removed in August 2020**, which is consistent with a point of interest and would be a defect in a
register. Nothing in the layer distinguishes a structure that stands from a place where one was.

Layer 2 covers the whole Ottawa basin rather than this county — 244.84 miles against the county's
735.22 of mapped channel — and carries no survey year. Ohio EPA's own designations and dates are
in [the TMDL report](ohio-epa-ottawa-river-tmdl.md); this layer is read here for which streams the
county thought worth drawing and not for an attainment finding.
