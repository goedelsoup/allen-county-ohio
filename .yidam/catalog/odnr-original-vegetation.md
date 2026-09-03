---
name: Original Natural Vegetation of Ohio (ODNR, after Gordon 1966)
description: >-
  What grew on this ground before the surveyors came, mapped. It is the source the corpus named in
  an open question and did not fetch: the polygon layer that answers where the Great Black Swamp
  actually was, as against the hydric soil the corpus had been using as its proxy.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 1825
location:
  - kind: url
    value: https://gis.ohiodnr.gov/geodata/Statewide/OriginalVegetationOhio.zip
    description: >-
      4.4 MB. One shapefile, `or_veg_spn83`, 1,632 polygons statewide in NAD83 Ohio North (US survey
      feet), plus the published map as a PDF. Digitized by ODNR in 2003 from Robert B. Gordon,
      *The Natural Vegetation of Ohio at the Time of the Earliest Land Surveys*, Ohio Biological
      Survey, 1966.
  - kind: url
    value: https://www.arcgis.com/sharing/rest/content/items/e1f5b7dd508c44b98c521d46b52d4a9f
    description: >-
      The item record that points at the zip. The download URL is not linked from any ODNR services
      directory this corpus could reach — `gis.ohiodnr.gov/arcgis/rest/services` lists sixteen
      folders and one map service, none of them this — so the item record is how the file is found.
used-by:
  - ../corpus/natural-feature/great-black-swamp.yml
  - ../corpus/measure/allen-county-original-vegetation.yml
---

**The legend is inside the shapefile and nowhere else in it.** The attribute table carries
`VEG_CDE` as a bare integer; the thirteen names are in the FGDC metadata that ships beside the
`.shp`, as an enumerated domain attributed to the Ohio Biological Survey. [verified] — the file's
own `.shp.xml`. Code 4 is *Elm-Ash Swamp Forests* and code 1 is *Beech Forests*, which are the only
two that occur in Allen County.

**Several re-hosts of this layer exist with the names already joined and this corpus does not use
them.** A search of the public item catalogue returns feature services owned by individuals and by
an Audubon chapter carrying the same geometry with a `VegName` column added. The agency's own zip
carries the same information in its metadata, so a re-host would buy nothing and cost a witness;
see [a reprint is not a second witness](../decisions/a-reprint-is-not-a-second-witness.yml).

**It is a 1966 interpretation of 1800s field notes, digitized in 2003, and each of those three
dates matters.** Gordon read the original land survey records; ODNR traced his printed map; the
polygons are therefore as precise as a pen on a state-scale sheet and no more. [verified] — the
item's own description. A township percentage computed from it is good to a point or two and a
parcel-level claim would be a misuse.

**It partitions the county exactly.** Clipped to Allen County the polygons sum to 260,358.7 acres
against a county area of 260,358.7 computed from the same boundary — no gaps and no overlaps, which
is the check that the whole county is accounted for rather than most of it. [verified] — computed
here against [TIGER's county polygon](census-tiger-roads.md).

**ODNR's liability statement is on the item and is the ordinary one.** The department provides the
data as is, with no warranty as to accuracy or fitness. [verified] — the item record.
