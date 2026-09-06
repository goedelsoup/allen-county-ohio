---
name: National Land Cover Database (MRLC / USGS)
description: >-
  Nine epochs of thirty-metre land cover for the conterminous United States, 2001 through 2021,
  plus impervious surface and tree canopy, served as a keyless coverage service that will clip to
  a county. It is the source the Cropland Data Layer's non-agricultural classes are made from, and
  the source the Cropland Data Layer's own documentation tells you to use instead of them.
type: dataset
obtained: true
retrieved: 2026-09-06
ttl_days: 365
location:
  - kind: url
    value: https://www.mrlc.gov/geoserver/mrlc_download/wcs?service=WCS&version=2.0.1&request=GetCapabilities
    description: >-
      102 coverages. Land cover for 2001, 2004, 2006, 2008, 2011, 2013, 2016, 2019 and 2021;
      impervious surface for the same nine; tree canopy annually 2011–2021; change count, change
      index and first-disturbance date for 2001–2021; forest disturbance date 1984–2021; and two
      summaries of the annual 1985–2023 product, which are all of it this service carries.
  - kind: url
    value: https://www.mrlc.gov/geoserver/mrlc_download/wcs?service=WCS&version=2.0.1&request=GetCoverage&coverageId=mrlc_download__NLCD_2021_Land_Cover_L48&format=image/tiff&subset=X(964400,1017700)&subset=Y(2013300,2059400)
    description: >-
      One county, clipped. The coverages are in **EPSG:5070**, NAD83 Conus Albers, with axis labels
      `X Y`, so the subset must be given in metres in that projection and not in degrees. Allen
      County's envelope is X 964,400 to 1,017,700 and Y 2,013,300 to 2,059,400; the response is a
      1777 × 1537 byte GeoTIFF of 4.2 MB, which is the whole county at native resolution.
  - kind: url
    value: https://www.mrlc.gov/geoserver/wms?service=WMS&version=1.1.1&request=GetLegendGraphic&layer=mrlc_display:Annual_NLCD_Smy_LndCov_ChgIdx_1985_2023_CU_C1V0&format=application/json
    description: >-
      The legend for the change products, which has no documentation page at all. Every
      `/data/legends/` URL for them answers 404 and so does the legends index; the map service's
      own style endpoint returns the colormap with its labels — "Urban change (3)", "Agriculture
      within class change (6)", "Urban within class change (21)". Substitute any layer name in the
      `mrlc_display` workspace.
  - kind: url
    value: https://www.mrlc.gov/data/legends/national-land-cover-database-class-legend-and-description
    description: >-
      The legend, and the only place the class values are named. The GeoTIFF carries a 256-entry
      colour table and **no raster attribute table**, so a reader who does not fetch this page has
      sixteen integers and no names for them.
  - kind: url
    value: https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb/tigerWMS_Census2020/MapServer/82/query?where=GEOID='39003'&outSR=5070&f=geojson
    description: >-
      The county boundary in the same projection, which is what turns a rectangle into a county.
      Rasterised onto the coverage's own grid it burns 1,170,809 pixels — 260,382 acres, the
      Census Bureau's own land-plus-water figure for Allen County to the acre.
used-by:
  - ../corpus/measure/allen-county-land-cover-2001-2021.yml
  - ../corpus/measure/allen-county-developed-land-2001-2021.yml
---

**The producer of the corpus's existing land-cover figures says to use this one instead.** The
Cropland Data Layer's own frequently-asked-questions page reads: "The accuracy of the CDL
non-agricultural land cover classes are entirely dependent upon the NLCD. Thus, the USDA NASS
recommends that users consider the NLCD for studies involving non-agricultural land cover."
[verified] — [NASS's CDL FAQ](usda-cropscape-cdl.md), quoted. The CDL's developed, forest, water
and wetland classes are not a second opinion about this county; they are this file, recoded and
re-registered annually.

**A class that only moves one way.** Across the eight epoch steps here, 1,909 acres of Allen County
entered the developed classes and 0.7 acres left them; 2,260 acres moved to a denser developed
class and **not one acre moved to a thinner one**. Over the same steps forest gained 434 acres and
lost 442, cropland gained 968 and lost 2,425, wetland gained 92 and lost 62. Every other class in
this file moves in both directions and this one does not. [verified] — computed here, epoch by
epoch; see
[a class that only moves one way](../decisions/a-class-that-only-moves-one-way.yml).

**One epoch step is a mapping change and shows as one.** 183 acres of this county go from
cultivated crops to open water between 2011 and 2013 and essentially none does so in any other
step — eight acres across 2001–2011 and none at all after 2013. A change confined to one boundary
in a nine-epoch series is a property of the boundary. [verified] — computed here.

**The GDAL trap, recorded again.** `gdal.Open(...).GetRasterBand(1).ReadAsArray()` on one line
raises `TypeError: in method 'Band_XSize_get'`: the dataset is collected before the band is read.
Bind the dataset to a name. This is the same defect the
[NHDPlus work](usgs-nhdplus-high-resolution.md) recorded for `Layer_ResetReading`.

**What is not taken here.** The Alaska, Hawaii and Puerto Rico coverages, which are on the same
service and are not this county.

## The annual product, its two summaries, and the thirty-nine layers that are not served

**The annual 1985–2023 product is on this service as two summaries of itself and not as its
layers.** `Annual_NLCD_Smy_LndCov_ChgCnt_1985_2023_CU_C1V0` counts how many times each pixel
changed class across the thirty-nine years and `Annual_NLCD_Smy_LndCov_ChgIdx_1985_2023_CU_C1V0`
names the kind of change; both clip to the county at native resolution like every other coverage
here. [verified] The thirty-nine annual land-cover rasters they summarise are in neither the WCS
capabilities nor the WMS's — a search of both documents for `LndCov` returns those two names and
nothing else. [verified] — the two capabilities documents, retrieved together.

**Nor are they reachable in bulk.** `https://s3-us-west-2.amazonaws.com/mrlc/Annual_NLCD_LndCov_1985_CU_C1V0.tif`
answers 403, the `.zip` forms of the same names answer 403, and the 2023 `.tif` answers 200 with a
**forty-two-byte body** — a bare TIFF header and no image. Listing the bucket answers
`AccessDenied`. [verified] — requested and recorded, not worked around. So the product built to
replace the nine epochs can be asked how often this county changed and cannot be asked what it
looked like in any particular year.

**The two products do not agree about how much of this county moves, and the windows do not
explain it.** The nine-epoch file has 2.47 per cent of the county changing class between 2001 and
2021; the annual summary has 11.38 per cent changing between 1985 and 2023. [verified] The annual
window is roughly twice as long and the figure is more than four times as large, and the annual
product's change index says why: its largest single category is a class changing *inside itself*.
[verified]

**Tree canopy is eleven separately named coverages and one product.** `nlcd_tcc_conus_<year>_v2021-4`
for 2011 through 2021, per-cent cover per pixel, all eleven produced in the same v2021-4 run rather
than published a year at a time — which is what makes the series internally comparable and is also
why a year-to-year move in it is not an independent observation. [verified] Values run 0–100 with
254 as fill; one pixel of this county carries it in 2021 and none in any other year.
