---
name: TIGER/Line Roads (Census Bureau)
description: >-
  The Census Bureau's road linework for one county, published as a shapefile every year since the
  2010 census. It carries a name and a feature class for every segment and nothing else: no owner,
  no traffic, no surface, no condition. It is the only file this corpus has found that draws every
  public road in Allen County, and the first thing it establishes is that a mile of linework is not
  a mile of road.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/geo/tiger/TIGER2024/ROADS/tl_2024_39003_roads.zip
    description: >-
      The 2024 edition for Allen County, 1.26 MB, **8,360 segments**. Geographic coordinates on
      NAD83; lengths here are computed after projection to EPSG:6549, Ohio North, whose units are
      US survey **feet** and not metres. The editions for 2010, 2013, 2016, 2019 and 2022 are held
      as well, at the same path with the year changed in three places.
  - kind: url
    value: https://www2.census.gov/geo/tiger/TIGER2024/COUNTY/tl_2024_us_county.zip
    description: >-
      The county boundary, used to check that the road file is clipped to the county it is named
      for. It is: both files have the identical envelope, 40.643069 to 40.920429 north and
      −84.397380 to −83.879828 west, and the polygon's projected area comes to 1,053,638,605 square
      metres against a stated land-plus-water area of 1,053,739,452 — agreement to 0.01 per cent,
      which is what established that the projection and its units were right.
  - kind: url
    value: https://www2.census.gov/geo/tiger/TIGER2013/PLACE/tl_2013_39_place.zip
    description: >-
      The Ohio place file, taken for 2011 to 2014 to date one boundary change. Carries `LSAD`,
      `CLASSFP` and `MTFCC`, which is how a village is told from a census designated place.
used-by:
  - ../corpus/measure/allen-county-roads-2010-2024.yml
  - ../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml
---

**What it draws, and how much of it.** Nine feature classes appear in Allen County's 2024 file. Six
of them are things a car can be driven on and they come to **2,225.6 miles**; the whole file,
including alleys, service drives, parking-lot roads and private service roads, comes to 2,232.3.
[verified] — the 2024 file, computed here.

    MTFCC   segments    miles   what it is
    S1400       8099   1814.9   local road, rural road or city street
    S1200        154    343.9   secondary road
    S1100          4     50.0   primary road
    S1630         42     16.0   ramp
    S1740         29      3.8   private road for service vehicles
    S1730         13      2.3   alley
    S1640         14      0.8   service drive
    S1780          4      0.5   parking lot road
    S1750          1      0.1   internal Census use

**Four segments are the whole of the county's primary road, and there are only two roads in them.**
I-75 is drawn as two lines of 23.167 and 23.139 miles, and US 30 as two of 1.952 and 1.737.
[verified] — the same file, by segment. A divided highway is two carriageways and this file draws
one line per carriageway, so its mileage counts pavement and not route; see
[a centerline is not a road](../decisions/a-centerline-is-not-a-road.yml).

**The check that proves it is against another agency.** The Federal Highway Administration's
[public release](fhwa-hpms-public-release.md) gives I-75 in this county as 23.12 route miles and
US 30 as 24.06. This file gives 46.31 and 48.098. The ratios are 2.0030 and 1.9991. [verified] —
the two files, computed here.

**Its editions cannot be differenced, and the flat part is as much a fact about the Bureau as the
moves.** The county's driveable mileage reads 2,358.5 in 2010, 2,299.1 in 2013, 2,304.2 in 2016,
2,226.7 in 2019, 2,225.6 in 2022 and 2,225.6 in 2024 — identical across the last three editions to
a tenth of a mile, in a county that authorised new houses and annexed ground in every one of those
years. [verified] — the six editions, computed here.

**And one of the moves is traceable to a label.** US 30's total in this county is **48.098 miles in
both the 2016 and the 2019 edition**, to a thousandth. In 2016 all of it is secondary road; in 2019,
3.689 miles of it is primary and 44.409 is secondary, and the county's secondary total falls by
3.743 in the same step. Nothing was built. A class moved on 3.689 miles of existing pavement.
[verified] — the two editions, computed here. This is
[a column can empty into its neighbour](../decisions/a-column-can-empty-into-its-neighbour.yml)
in a map instead of a table.

**It says nothing about who owns a road.** There is no ownership, maintenance, jurisdiction or
funding column in it, and no way to derive one: a county road that becomes a city street at a
corporation line is one continuous feature with one name. [verified] — the file's own schema.
