---
name: SSURGO soil survey of Allen County, Ohio (USDA NRCS)
description: >-
  The field soil survey of this county, digitised: 80 map units over 260,340 acres, each broken into
  the soil series that make it up, with each one's drainage class, hydric rating, taxonomic order
  and prime-farmland class. It is the only source this corpus holds that describes the ground
  itself rather than what has been built on it or grown out of it.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 730
location:
  - kind: url
    value: https://sdmdataaccess.sc.egov.usda.gov/Tabular/post.rest
    description: >-
      Soil Data Access. A POST of `{"format":"JSON+COLUMNNAME","query":"<SQL>"}` returns a table.
      The county's composition is one join — `legend` to `mapunit` to `component` on
      `areasymbol = 'OH003'` — and returns 436 component rows in 79 KB.
  - kind: url
    value: https://sdmdataaccess.sc.egov.usda.gov/Tabular/post.rest#spatial
    description: >-
      The same endpoint answers spatial queries against `mupolygon`. Clipping map-unit polygons to a
      township boundary — `mupolygongeo.STIntersection(geometry::STGeomFromText(<wkt>,4326)).STArea()`
      grouped by `mukey`, with `areasymbol = 'OH003'` in the `WHERE` — is what makes a figure per
      township possible.
  - kind: url
    value: https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb/tigerWMS_Census2020/MapServer/20/query
    description: >-
      TIGERweb layer 20, county subdivisions, which supplies the thirteen boundary polygons the
      spatial queries are clipped to. Rings come back in ArcGIS orientation, so a hole is a
      counter-clockwise ring and has to be told from a second exterior part by its signed area.
used-by:
  - ../corpus/measure/allen-county-soils-2026.yml
  - ../corpus/natural-feature/great-black-swamp.yml
  - ../corpus/measure/allen-county-original-vegetation.yml
  - ../corpus/place/allen-county.yml
---

**A query without a county filter times out and a query with one does not.** The unfiltered
clipped-area aggregation over `mupolygon` returned *Your query timed out* against a box a tenth of a
degree across; adding `areasymbol = 'OH003'` to the same query returned in seconds. [verified] — the
retrievals here. The service reports the timeout as an OGC `ServiceExceptionReport` in XML with an
HTTP 400, so a client that assumes JSON gets a decode error rather than the message.

**The tabular acreage and the spatial acreage agree, which is the control.** Summing `muacres`
over the county's 80 map units gives 260,340; summing the clipped polygon areas over the thirteen
county subdivisions and re-deriving the same three shares gives hydric 35.3 per cent, very poorly
drained 34.5 and poorly drained or worse 74.8, against 35.3, 34.5 and 74.7 from the tabular join.
[verified] — the two computations here. They are different files and different arithmetic and they
land on the same county.

**And it agrees with the gazetteer to within a sixtieth of a square mile.** 402.545 square miles of
land and 4.306 of water is 260,385 acres; SSURGO maps 260,340, a difference of 45 acres or 0.017 per
cent. [verified] — this file against
[the 2020 Census Gazetteer](census-gazetteer-2020.md).

**A map unit is not a soil.** Each of the 80 is a named mixture — `Alvada loam, 0 to 1 percent
slopes` is 84 per cent Alvada, 8 per cent Pewamo, 5 per cent Blount and 3 per cent Hoytville — and
every rating hangs off the components, not the unit. [verified] — the same query. Weighting
`muacres` by `comppct_r` puts the county at 35.3 per cent hydric; taking each unit's dominant
component instead gives 34.3, and counting every unit that contains *any* hydric component gives
90.8. Only the first is a share of ground. The spread is not an accident of rounding: components
holding under half their unit are 41.7 per cent hydric against 34.2 for those holding half or more,
so the wet soil in this county sits disproportionately in the minor inclusions.

**The survey is old under the republication date.** SSURGO version 25 for this county was restored
on 27 August 2025, and the oldest source in its own lineage metadata is dated 1965 at a scale of
1:15,840; the digital revisions carry field dates from 1988 to 2009. [verified] — `sacatalog`, the
FGDC block. So the ratings describe soil as mapped across sixty years, which is the right vintage
for a property of the ground and the wrong one for anything a person did to it recently.

**`hydricrating` has four values and one of them is a blank.** *Yes*, *No*, *Unranked* and null, the
last two covering 3.3 per cent of the county between them — mostly water, pits and urban land.
[verified] — the same query. Read as *No* they would understate the wet ground.
