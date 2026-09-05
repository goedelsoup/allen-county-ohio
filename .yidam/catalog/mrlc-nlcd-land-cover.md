---
name: National Land Cover Database (MRLC / USGS)
description: >-
  Nine epochs of thirty-metre land cover for the conterminous United States, 2001 through 2021,
  plus impervious surface and tree canopy, served as a keyless coverage service that will clip to
  a county. It is the source the Cropland Data Layer's non-agricultural classes are made from, and
  the source the Cropland Data Layer's own documentation tells you to use instead of them.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://www.mrlc.gov/geoserver/mrlc_download/wcs?service=WCS&version=2.0.1&request=GetCapabilities
    description: >-
      102 coverages. Land cover for 2001, 2004, 2006, 2008, 2011, 2013, 2016, 2019 and 2021;
      impervious surface for the same nine; tree canopy annually 2011–2021; change count and change
      index for 2001–2021; and an annual 1985–2023 product not read here.
  - kind: url
    value: https://www.mrlc.gov/geoserver/mrlc_download/wcs?service=WCS&version=2.0.1&request=GetCoverage&coverageId=mrlc_download__NLCD_2021_Land_Cover_L48&format=image/tiff&subset=X(964400,1017700)&subset=Y(2013300,2059400)
    description: >-
      One county, clipped. The coverages are in **EPSG:5070**, NAD83 Conus Albers, with axis labels
      `X Y`, so the subset must be given in metres in that projection and not in degrees. Allen
      County's envelope is X 964,400 to 1,017,700 and Y 2,013,300 to 2,059,400; the response is a
      1777 × 1537 byte GeoTIFF of 4.2 MB, which is the whole county at native resolution.
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

**What is not taken here.** The annual 1985–2023 product, which would give thirty-nine years rather
than nine and is a different lineage with its own accuracy statement; tree canopy; and the Alaska,
Hawaii and Puerto Rico coverages, which are on the same service and are not this county.
