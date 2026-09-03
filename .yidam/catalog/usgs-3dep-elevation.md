---
name: USGS 3D Elevation Program, point service and image service
description: >-
  How high the ground is, at one-metre resolution, keyless. It is the first source in this catalog
  that can say how high anything in Allen County is — the corpus has held rivers, watersheds,
  quarries and a continental divide without a single elevation attached to any of them.
type: api
obtained: true
retrieved: 2026-09-03
ttl_days: 1825
location:
  - kind: url
    value: https://epqs.nationalmap.gov/v1/json
    description: >-
      One point in, one elevation out. `x`, `y`, `wkid=4326`, `units=Feet`. Used here for the 98
      samples spaced 500 feet along the continental divide inside this county. A bare request is
      served, but this corpus sends a user agent naming itself and spaces its requests; the service
      answers about one a second and returns HTTP 500 rather than a queue when pushed.
  - kind: url
    value: https://elevation.nationalmap.gov/arcgis/rest/services/3DEPElevation/ImageServer/exportImage
    description: >-
      The whole surface as a raster. A bounding box, a size and `format=tiff&pixelType=F32` return a
      GeoTIFF; 1,600 × 880 over this county is 6 MB and roughly 30 metres a pixel. Elevations come
      back in **metres** whatever the point service was asked for.
used-by:
  - ../corpus/measure/allen-county-elevation-2026.yml
---

**It is a model of the current surface, and the current surface includes what has been dug.** The
lowest elevation anywhere in Allen County is 474.7 feet, 258.8 feet below any natural ground in the
county, and it is the floor of a working limestone quarry. [verified] — the image service, clipped
to the county. A minimum or a maximum read off this service is a statement about a place and has to
be visited before it is quoted; see
[an extreme in a surface has an address](../decisions/an-extreme-in-a-surface-has-an-address.yml).

**The two endpoints do not return the same units and neither says so loudly.** The point service
takes a `units` parameter and honours it. The image service returns whatever the underlying raster
holds, which for 3DEP is metres, and the request has no unit to ask for. [verified] — both, against
each other at the same coordinate.

**Its date is a mosaic and not a date.** 3DEP is assembled from lidar and older photogrammetric
sources acquired over decades and republished as one seamless surface, so a pit measured here is
the pit as of whenever that tile was flown, which the service reports as a range rather than a day.
[verified] — the service's own catalog. Any volume or depth taken from it is stated with that
caveat and never as a measurement of the present.

**Two ways of asking cost very differently.** Ninety-eight point queries took about two minutes;
one raster export covering the whole county took nine seconds and answered questions the points
could not. Where a question is about a region rather than a line, the raster is the instrument.
[verified]
