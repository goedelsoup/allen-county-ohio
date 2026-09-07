---
name: Federal Standards and Procedures for the National Watershed Boundary Dataset (WBD)
description: >-
  The document that defines what every field in the Watershed Boundary Dataset means. It is in
  this catalog because two of its fields share a name and do not share a domain, and reading the
  county's watershed attributes without it produces confident statements about levees and roads
  that the file does not make.
type: document
obtained: true
retrieved: 2026-09-07
ttl_days: 3650
location:
  - kind: url
    value: https://pubs.usgs.gov/tm/11/a3/pdf/tm11-a3.pdf
    description: >-
      USGS Techniques and Methods 11-A3, fourth edition. 10.3 MB, text layer present. §6.3.7.4 is
      HUMod on the 12-digit polygon, §6.4.2.3 is HUMod on the boundary line, §6.4.2.4 is
      LineSource.
used-by:
  - ../corpus/measure/allen-county-watershed-boundaries-2026.yml
---

## Two fields called HUMod, two domains, and they are about different things

**§6.4.2.3, on the line**, is *"the modification to natural overland flow that alters the location
of the hydrologic unit boundary or special conditions that are applied to a specific boundary line
segment."* Its domain includes:

> **TF Transportation Feature**—A land transportation feature, for example, a road, railroad, dock,
> airport, etc., that alters the natural boundary location.

> **LE Levee**—An artificial bank to confine a stream channel or limit adjacent areas subject to
> flooding; alters the natural boundary location.

**§6.3.7.4, on the 12-digit polygon**, is *"the type of modification to natural overland flow that
alters the natural delineation of a 12-digit hydrologic unit."* Its stated domain is
`AW, GF, GL, IF, KA, LA, MA, NC, NM, OC, OF, PD, RC, RS, UA, WD` — and **`TF` and `LE` are not in
it.** Neither is `DD` or `IT`: both were folded into `AW Artificial Waterway`, whose definition
names the retired codes outright — *"(The previously included designations AD Aqueduct, DD Drainage
Ditch, GC General Canal/Ditch, ID Irrigation Ditch, IT Interbasin Transfer, SD Stormwater Ditch, SC
Stormwater Canal, and BC Barge Canal have now been grouped into this designation)."*

The standard says what to do about that in one sentence, and it is the sentence this corpus needed:

> Previous versions of this guideline did not provide the same number of modification choices—check
> metadata or contact the principal WBD In-State Steward for more information.

**Nothing in the code is a quantity.** Every definition in either domain is of the form *alters the
natural boundary location* or *alters the natural flow out of the hydrologic unit*. The field
records that a thing happened somewhere on or in the unit. It does not say where, how much, or how
often; see
[a presence flag is not an extent](../decisions/a-presence-flag-is-not-an-extent.yml).

## LineSource says what the boundary was traced from

§6.4.2.4: *"LineSource represents the code for the base data used for delineating hydrologic unit
boundaries."* Its domain runs from `DRG24` — *"Delineated from 1:24,000-scale Digital Raster
Graphics"*, which is a scanned paper topographic quadrangle — through `ORTHO "scale"`, `NED10`,
`DEM30` and `LIDAR`, *"Derived from LiDAR (light detection and ranging) data."*

§4.1 sets the floor: *"Delineations need to meet a 1:24,000 scale in the United States ... at a
minimum."* §4.2.1 makes the DRG the preferred base. So a boundary drawn from `DRG24` alone is not
below standard — it is the standard's baseline, and the field is how you find out that no elevation
model was involved.

## What this catalog entry does not contain

The domain for the **10-digit** polygon HUMod (§6.3.6.4), which is a third list again, and the
metadata form the LiDAR entry refers to. Neither was needed for the 12-digit units this corpus
reads.
