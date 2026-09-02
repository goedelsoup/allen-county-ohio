---
name: Cropland Data Layer, county statistics (USDA NASS, via CropScape)
description: >-
  Every thirty-metre pixel in Allen County classified and counted, once a year from 2008 — a
  hundred and thirty crop classes plus developed, forest, water and wetland, with acreage. It is
  the first source in this corpus that says what the county's ground is now rather than how much of
  it a farmer reports operating.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url_template
    value: https://nassgeodata.gmu.edu/axis2/services/CDLService/GetCDLStat?year={year}&fips=39003&format=csv
    description: >-
      The request. It does **not** return the statistics; it returns a `returnURL` element naming a
      CSV to fetch on a second call. A caller that parses the first response as data gets one line
      of XML.
  - kind: url_template
    value: https://nassgeodata.gmu.edu/webservice/nass_data_cache/byfips/CDL_{year}_39003.csv
    description: >-
      The CSV the call above names. Four columns — class value, class name, pixel count, acreage.
      Seventeen years were taken, 2008 through 2024, between 927 and 1,421 bytes each.
used-by:
  - ../corpus/measure/allen-county-land-cover-2008-2024.yml
---

**The pixel grid is fixed and the total is a control.** Every year from 2010 the classified acreage
for this county is 260,324 to the acre; 2008 and 2009 differ by three and four acres. The county's
land area from an unrelated federal file is about 259,840 acres, so the classification covers the
county and 484 acres of something adjacent or of rounding. Any year whose total moves has been
misread. [verified] — the seventeen files, summed here, against
[the county's land area](../corpus/measure/allen-county-land-area-2020.yml).

**Half of this file can be differenced year to year and half cannot.** The crop classes are
classified from that year's imagery and they move as farming moves: winter wheat falls from 21,483
acres to 6,688 between 2008 and 2012 and stays down, and 2019 puts 37,726 acres in
`Fallow/Idle Cropland` against a seventeen-year median of 55. The non-agricultural classes do not
behave that way. Developed land ranges over 3,226 acres — 6.9 per cent of the class — with a fitted
slope of **minus 35 acres a year**, in a county that has annexed and built continuously over the
same period; forest ranges over 21 per cent of its own size with a slope of plus 253. Those are the
classifier, not the ground. [verified] — same files, computed here. See
[one file, two reliabilities](../decisions/one-file-two-reliabilities.yml).

**A single year's class is still worth quoting even where the series is not.** The 2024 shares —
70.3 per cent crops and pasture, 17.8 developed, 10.4 forest — are one classification of one image
year, and they agree with the Census of Agriculture's 69.4 per cent in farms for 2022 to within a
point, from a wholly different definition. [verified] — same source against
[the census of agriculture](usda-census-of-agriculture.md).

**The class names are the file's own and some of them are not what they sound like.**
`Grass/Pasture` includes hay ground, lawns and road verge; `Developed/Open Space` is mostly the
grass between buildings rather than buildings. Nothing here counts farms, farmers, ownership or
tenure, and a crop class is what grew, not who grew it. [verified] — the class list.

**Double-cropped ground is one pixel and two crops.** `Dbl Crop WinWht/Soybeans` is its own class
and its acreage appears under neither wheat nor soybeans, so the sum of the single-crop classes
understates each. In this county the double-crop classes are under 900 acres in every year taken.
[verified] — same files.
