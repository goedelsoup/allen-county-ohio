---
name: National Register of Historic Places — NPS spatial dataset
description: >-
  The federal list of properties judged worth keeping, as a queryable map service: name,
  reference number, resource type, address, the date of listing, the count of contributing
  buildings, a National Historic Landmark flag, and a link to the nomination file at the National
  Archives. Allen County has twenty-nine listings and one Landmark.
type: api
obtained: true
retrieved: 2026-08-31
ttl_days: 365
location:
  - kind: url
    value: https://mapservices.nps.gov/arcgis/rest/services/cultural_resources/nrhp_locations/MapServer/0/query
    description: >-
      Layer 0, points — 26 of Allen County's 29. `where=State='OHIO' AND County='Allen'` returns
      them in one request; the record cap is 2,000. The state name is spelled out and in capitals,
      which is why a query on `'OH'` returns nothing rather than an error.
  - kind: url
    value: https://mapservices.nps.gov/arcgis/rest/services/cultural_resources/nrhp_locations/MapServer/1/query
    description: >-
      Layer 1, polygons — the other 3. A property is in one layer or the other and never both, so
      a county's list is the union and a query against the points alone silently loses the
      historic district.
used-by:
  - ../corpus/measure/allen-county-national-register.yml
  - ../corpus/site/allen-county-courthouse.yml
  - ../corpus/site/lima-memorial-hall.yml
  - ../corpus/site/lima-pennsylvania-railroad-depot.yml
  - ../corpus/site/miami-and-erie-canal.yml
  - ../corpus/place/delphos.yml
  - ../corpus/place/spencerville.yml
---

**What a row is.** `NRIS_Refnum` is the reference number a property carries for life; `CertDate`
is the day the Keeper of the Register signed it; `ResType` is one of building, structure, site,
object or district; `NumCBldg` and its siblings count the contributing features inside a listing,
which is how a single reference number can stand for thirty-five buildings. `MultiName` names the
multiple-property submission a listing arrived under, and for this county that one field explains
more than half the list.

**The coordinates are not survey.** Every Allen County point carries `BND_TYPE` "Arbitrary point"
and `MAP_METHOD` "Derived by XY event point or centroid generation", with the note that
coordinates were "extracted directly from NPS NRIS and processed from assumed coordinate systems".
The file claims `SRC_ACCU` of ±12 metres. The one listing this corpus can check against an
independently surveyed point — [the courthouse](../corpus/site/allen-county-courthouse.yml), whose
county-GIS coordinate the corpus already held — is **31.9 metres** away. Points from this file are
good enough to identify a township and a survey section and are not good enough to identify a
parcel, and this corpus uses them only for the first.

**`IS_EXTANT` is "Unknown" for all twenty-nine**, with the file's own explanation attached:
"Feature was created as part of batch process from NRIS and status needs to be confirmed
individually." A listing here is evidence that a building was standing when it was nominated. It
is not evidence that it is standing now.

**What it does not carry.** No removals — every Allen County row reads `STATUS: Listed`, and a
property delisted after demolition would simply be absent, so this source cannot be used to count
what has been lost. No nomination text, only a NARA link to the scanned file. No state or local
designations: Ohio's own inventory is a separate register and is not held here.
