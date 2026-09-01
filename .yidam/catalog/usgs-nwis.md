---
name: USGS National Water Information System
description: >-
  The United States Geological Survey's register of water-monitoring sites and its record of annual
  peak streamflow at each of them. The site service lists every gauge, well and lake station in a
  county with its coordinates, hydrologic unit and drainage area; the peak service gives one row per
  water year, the highest flow of that year and the day it happened.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 365
location:
  - kind: url
    value: https://waterservices.usgs.gov/nwis/site/?format=rdb&countyCd=39003&siteStatus=all&siteOutput=expanded
    description: >-
      Allen County's monitoring sites, 184 of them, tab-delimited with comment lines prefixed `#`
      and a units row beneath the header that must be discarded. `siteStatus=all` is needed or
      discontinued gauges vanish, and the discontinued ones are most of the county's long record.
  - kind: url
    value: https://nwis.waterdata.usgs.gov/nwis/peak?site_no=04187500&agency_cd=USGS&format=rdb
    description: >-
      Annual peak streamflow, one row per water year, for the Ottawa River at Allentown. The same
      path with `site_no` changed serves any gauge; a gauge with no peak record returns a 200 with
      a 60-byte body and no rows, which is how three of the county's larger stream sites answered.
used-by:
  - ../corpus/event/the-ottawa-river-flood-of-1959.yml
  - ../corpus/measure/allen-county-water-gauges-2026.yml
  - ../corpus/measure/ottawa-river-peak-flows-1924-2025.yml
---

**What a site is and what a record is.** The two are not the same thing and the difference is the
whole of what this source taught. Allen County has 184 monitoring sites; **two** of them have a
usable record of annual peak flow, and the two do not overlap in time. A county can be thick with
instruments and thin with measurements.

**Water years, not calendar years.** A water year runs 1 October to 30 September and is named for
the calendar year it ends in, so a peak dated 30 December 1990 belongs to water year 1991. Counting
peaks by calendar year gives some years two and some none, which is what the raw file looks like
until the convention is applied.

**Its silences are legible.** The Allentown gauge, 1924 to 1981, is missing water years 1936 to 1938
and 1940 to 1942. The Lima gauge, 1989 to 2025, is missing 2000 to 2009 exactly. A gap of that shape
is a gauge that was not funded, not a river that did not flood, and the file does not distinguish
them — it simply has no row. See
[the intersection, not the union](../decisions/the-intersection-not-the-union.yml).

**Hydrologic units come free with the site list.** Every site carries an eight-digit `huc_cd`, which
is how this phase confirmed that 168 of the county's 184 sites are in the Auglaize subbasin
(`04100007`), 14 in the Blanchard (`04100008`), and the remaining two in other Maumee units — and
that not one of them sits on the Ohio River side of the divide the corpus found in
[Auglaize Township](../corpus/natural-feature/scioto-river-basin.yml). That is an absence of
instruments and not evidence about the divide.

**What it will not answer.** Anything about water quality, which is a different service; anything
about flood stage or the elevation at which a river leaves its banks, which is the National Weather
Service's; and how much water flows in an ordinary year, which is the daily-values service and was
not fetched. Peak flow is the highest instant of each year and nothing else.
