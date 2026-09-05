---
name: USGS NHDPlus High Resolution, via The National Map
description: >-
  The federal flow network — every mapped channel in the country with the reach below it named,
  its drainage area, its stream order, and its distance to the sea. It is the source this corpus
  named in three open questions and a crate stub and did not have.
type: api
obtained: true
retrieved: 2026-09-04
ttl_days: 1825
location:
  - kind: url
    value: https://hydro.nationalmap.gov/arcgis/rest/services/NHDPlus_HR/MapServer
    description: >-
      Thirteen layers. 3 `NetworkNHDFlowline` is the one that matters — 88 fields, the whole
      NHDPlus value-added attribute table exposed on a keyless service. 1 `NHDPlusSink`,
      2 `NHDPoint`, 8 `NHDArea`, 9 `NHDWaterbody`, 12 `WBDHU12`. `maxRecordCount` is 2,000.
used-by:
  - ../corpus/measure/allen-county-stream-network-2026.yml
  - ../corpus/measure/allen-county-standing-water-2026.yml
  - ../corpus/measure/allen-county-distance-to-the-sea-2026.yml
  - ../corpus/natural-feature/blanchard-river.yml
  - ../corpus/natural-feature/riley-creek.yml
---

Queried exactly like [the Watershed Boundary Dataset](usgs-watershed-boundary.md) and
[TIGERweb](tigerweb-census2020.md): an envelope goes in and the features intersecting it come
back. No key, no registration, no rate limit met across the two hundred or so queries this phase
made. The county's flowlines were taken as GeoJSON over the envelope
`-84.42,40.62,-83.86,40.94` in two pages of a thousand, then clipped to the TIGER county polygon
in EPSG:26916 — the same projection [the canal](../corpus/measure/miami-and-erie-canal-in-allen-county-2026.yml)
was measured in, so the two are comparable.

**Take the envelope from the polygon and not from memory.** The first pass used
`-84.30,40.60,-83.85,40.95` and returned 1,539 flowlines; the county's western edge is at
−84.39738 and the correct envelope returns 1,792. A bounding box that looks generous can still
be inside the thing it is meant to contain.

**What it gives that the WBD cannot.** The WBD says which basin a stream's water reaches. This
says which reach it enters, by identifier:

    hydroseq        this flowline's position in the sort from mouth to headwater
    dnhydroseq      the flowline immediately downstream — the `downstream_comid` field
                    three nodes of this corpus named as the instrument they needed
    uphydroseq      the flowline immediately upstream on the main path
    levelpathi      the main stem this flowline belongs to
    terminalpa      the level path it ends on — the ocean, in effect
    pathlength      kilometres from the bottom of this flowline to that terminal
    totdasqkm       drainage area above it
    streamorde      Strahler order
    qema            estimated mean annual flow, cubic feet a second

**The service labels its own codes and this corpus takes the labels from it.** `FCode` has no
coded-value domain on the layer, but the renderer carries one: 46003 Intermittent, 46006
Perennial, 55800 Artificial Path, 33600 Canal Ditch, 33400 Connector; and on the waterbody layer
390 Lake Pond, 436 Reservoir, 466 Swamp Marsh. [verified] — the layer's `drawingInfo.renderer`,
`uniqueValueInfos`. That is the file describing itself, which is better than a data dictionary
this corpus would have had to find elsewhere.

**`qema` is a model and is recorded here so it is never read as a gauge.** It is an estimate of
mean annual flow from runoff, and the field list carries five more like it — `qbma` through
`qfma`, each a different adjustment. Allen County has two gauges with a peak-flow record and this
service puts a number on 1,150 reaches. The two are not the same kind of thing and this corpus
does not put them in one table; see [the peak flows](../corpus/measure/ottawa-river-peak-flows-1924-2025.yml).

**A caution about clipping.** A flowline that merely touches a county boundary is returned by an
intersects query, and its `LengthKm` is the whole reach. Every length in the nodes below is the
clipped length — the part inside Allen County — and the two differ by a fifth: 1,792 flowlines
intersect the envelope, 1,150 touch the county, and their full length is not the county's.

**What it cannot support.** It is a network of channels, not of water. It says which reach lies
below which; it does not say that water flows through them today, and its intermittent class is
drawn from mapping rather than from measurement. Nothing here dates a channel or says who dug it,
which matters in a county where most of the network is ditch; see
[what the ground is](../corpus/measure/allen-county-soils-2026.yml). And the level path is a
computed main stem rather than a river — see
[a level path is not a river](../decisions/a-level-path-is-not-a-river.yml).
