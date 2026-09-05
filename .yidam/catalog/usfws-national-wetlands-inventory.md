---
name: National Wetlands Inventory (U.S. Fish and Wildlife Service)
description: >-
  The federal map of every wetland and deepwater habitat in the United States, polygon by polygon,
  each carrying a Cowardin code that says what system it belongs to, what grows in it, how long it
  holds water — and whether somebody dug it. It is the fifth federal file this corpus holds that
  measures wetland on the same ground, and the only one whose classification distinguishes a marsh
  from a farm pond.
type: dataset
obtained: true
retrieved: 2026-09-05
ttl_days: 365
location:
  - kind: url
    value: https://fwspublicservices.wim.usgs.gov/wetlandsmapservice/rest/services/Wetlands/MapServer/0/query
    description: >-
      The queryable layer. An `esriGeometryEnvelope` around the county returns 5,345 polygons at
      1,000 a page with `resultOffset`; `outFields` carries `ATTRIBUTE` (the Cowardin code),
      `WETLAND_TYPE` (the five plain-English classes), and `ACRES` (the whole polygon's area,
      which is not its area inside the county). `f=geojson` and `outSR=4326` give geometry that
      can be clipped locally.
  - kind: url
    value: https://www.fws.gov/wetlands/downloads/Watershed/HU8_04100007_Watershed.zip
    description: >-
      The documented bulk download by hydrologic unit, and it 404s. The host it moved to,
      `documentst.ecosphere.fws.gov`, answers 403 to an automated client. Not a gate — the map
      service above serves the identical data without challenge — but recorded because the route
      the documentation names is not the route that works.
used-by:
  - ../corpus/measure/allen-county-wetlands-2026.yml
---

**The classification is the source, not the acreage.** A polygon's `ATTRIBUTE` is a Cowardin code:
system, subsystem, class, water regime, then modifiers. `PEM1C` is palustrine emergent persistent
seasonally-flooded — a marsh. `PUBGx` is palustrine unconsolidated-bottom intermittently-exposed
**excavated** — a dug pond. `L1UBHh` is lacustrine limnetic unconsolidated-bottom permanently
flooded **diked or impounded** — a reservoir. All three are "wetland" in the layer's own
`WETLAND_TYPE` rollup, and two of them are earthworks. See
[a modifier changes what the thing is](../decisions/a-modifier-changes-what-the-thing-is.yml).

**Its `ACRES` field is the polygon's area and not its area here.** A wetland on the county line
carries its whole acreage in every row, so a sum of `ACRES` over an envelope query overstates. The
figures this corpus publishes are computed by clipping each polygon to the county boundary and
measuring in EPSG:5070, which is the same method it used on the hydrography file. [verified] —
the layer's own field list against the clipped result.

**Its geometry needs repair before it can be intersected.** Nine polygons in and around this county
self-intersect, and one of them stops a naive clip with a GEOS topology exception. Running
`MakeValid` on both sides is what lets the clip complete; 4,539 of the 5,383 polygons fetched have
area inside Allen County and none is dropped. [verified] — the clip, counted.

**What it is authority for.** Where a wetland was mapped, of what class, in what water regime, and
whether it is excavated, diked or farmed. It is a photo-interpretation product with field checks,
not a jurisdictional determination, and the Service says so: a polygon here is not a statement that
the Clean Water Act applies to that ground.

**What it is not authority for.** When the mapping was done. The layer this corpus queried carries
no image date on the polygon, so the county's 7,361 acres are as of the compilation and not as of
2026, and nothing retrieved here dates them. That is the one thing this file needs and does not
carry. [verified] — the field list, read in full.

**It disagrees with every other file the corpus holds on the same ground, and the disagreements are
the finding.** The hydrography file draws 42.0 acres of swamp or marsh here; this one draws 425.9
acres of emergent wetland. The Cropland Data Layer finds 704 acres of wetland; the soil survey
rates 91,953 acres hydric; the original-vegetation map draws 22,433.9 acres of elm-ash swamp
forest. Five federal files, five numbers, spanning a factor of two thousand. See
[what is left of the water](../corpus/measure/allen-county-wetlands-2026.yml) and
[the gap between two universes is a measurement](../decisions/the-gap-between-two-universes-is-a-measurement.yml).
