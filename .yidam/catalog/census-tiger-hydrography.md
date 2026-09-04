---
name: TIGER/Line Hydrography (Census Bureau)
description: >-
  The Census Bureau's water linework and water polygons for one county, the companion files to
  [the road file](census-tiger-roads.md) this corpus already reads. It is where the county's
  streams, ditches and canals are drawn, and its interest here is what it declines to draw: the
  Miami and Erie Canal is in it three times under two spellings, and the stretch of it that is a
  National Historic Landmark is in it under no name at all.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/geo/tiger/TIGER2024/LINEARWATER/tl_2024_39003_linearwater.zip
    description: >-
      Water as lines — 322,607 bytes, **272 features** for Allen County. Five fields and no more:
      `ANSICODE`, `LINEARID`, `FULLNAME`, `ARTPATH`, `MTFCC`. No width, no depth, no flow, no owner
      and no date. 260 features carry `MTFCC` H3010, stream or river; 12 carry H3020, canal, ditch
      or aqueduct. 203 of the 272 have no name.
  - kind: url
    value: https://www2.census.gov/geo/tiger/TIGER2024/AREAWATER/tl_2024_39003_areawater.zip
    description: >-
      Water as polygons — 559,922 bytes, 1,125 features, 1,091 of them unnamed. The named ones are
      the rivers, the reservoirs and eight lakes. No canal polygon exists in this county, which is
      why the canal below is measured as a line and then from the ground.
  - kind: url
    value: https://www2.census.gov/geo/tiger/TIGER2024/COUSUB/tl_2024_39_cousub.zip
    description: >-
      County subdivisions for the whole of Ohio, 6.45 MB. Taken statewide rather than by county
      because the question asked of it was which side of a county line a point falls on, and a file
      clipped to one county cannot answer that.
used-by:
  - ../corpus/measure/miami-and-erie-canal-in-allen-county-2026.yml
  - ../corpus/measure/miami-and-erie-canal-deep-cut-2026.yml
  - ../corpus/site/miami-and-erie-canal.yml
---

**One canal, three features, two spellings.** `Miami & Erie Cnl` appears on two of them and
`Miami-Erie Cnl` on the third, and the three together run 32,290.7 feet — 6.1157 miles — from
`LINEARID` 110457129607 north to 110457129744. [verified] — the file, projected to EPSG:26916 and
summed. A search on the full name returns nothing at all, and a search on either spelling returns
part of the canal; see [the canal](../corpus/measure/miami-and-erie-canal-in-allen-county-2026.yml).

**`MTFCC` is the only classification, and H3010 and H3020 are not drawn from different evidence.**
The first is stream or river and the second is canal, ditch or aqueduct — one code for the natural
channel and one for three kinds of dug channel that are not distinguished from each other.
[verified] — the file's own schema. A file with a class for canal-or-ditch cannot be asked which of
those a feature is, and this corpus has learnt not to ask it.

**A named feature can stop before the thing does.** The canal's most conspicuous surviving reach —
the 7,375 feet of it that is a National Historic Landmark — is not in this file under the canal's
name. Its northern 4,692 feet are in the file as an unnamed H3010, a stream; its southern 2,684
feet are not in the file at all. [verified] — the file against
[the elevation surface](usgs-3dep-elevation.md), compared point by point at hundred-metre spacing.

Absence from this file is not absence from the ground, and this corpus reads a gap in it as a
statement about the Bureau's water inventory rather than about the county. [inference]

**It shares the road file's units problem and its remedy.** Coordinates are geographic on NAD83, so
a length is meaningless until the geometry is projected. Lengths here are computed in EPSG:26916 —
UTM zone 16 north, metres — rather than in the Ohio North state plane used for
[the roads](../corpus/measure/allen-county-roads-2010-2024.yml), because the same corridor had to be
compared with an elevation raster that the national map serves in metres. The two projections agree
on this canal's length to 0.03 per cent. [verified] — both, computed here.
