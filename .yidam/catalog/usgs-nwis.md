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
    value: https://api.waterdata.usgs.gov/ogcapi/v0/collections/field-measurements/items?county_code=003&state_code=39&site_type_code=GW&parameter_code=72019&limit=10000&f=json
    description: >-
      Every depth-to-water reading ever taken in the county's groundwater wells — 904 of them,
      1962 to 2026 — as GeoJSON, in one call and with no site list needed first. This is the
      replacement for the decommissioned `gwlevels` service; see the note below.
  - kind: url
    value: https://api.waterdata.usgs.gov/ogcapi/v0/collections/monitoring-locations/items?county_code=003&state_code=39&limit=500&f=json
    description: >-
      The same 185 sites the legacy `site` service gives, with aquifer code, national aquifer,
      well and hole depth, altitude and construction date on each. The legacy service's expanded
      output carries these too; this one is GeoJSON and needs no units row discarded.
  - kind: url
    value: https://nwis.waterdata.usgs.gov/nwis/peak?site_no=04187500&agency_cd=USGS&format=rdb
    description: >-
      Annual peak streamflow, one row per water year, for the Ottawa River at Allentown. The same
      path with `site_no` changed serves any gauge; a gauge with no peak record returns a 200 with
      a 60-byte body and no rows, which is how three of the county's larger stream sites answered.
used-by:
  - ../corpus/measure/allen-county-groundwater-1962-2026.yml
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

**The service this corpus needed for groundwater was decommissioned before the corpus asked for
it.** `waterservices.usgs.gov/nwis/gwlevels/` returned discrete field measurements of water level in
a well. It was frozen on 1 November 2025, began returning a 301 to a blog post on 1 February 2026,
and was to start returning errors on 1 June 2026; it still redirects. This catalog entry was written
on 1 September 2026 and the node it fed opened a question — *whether any of the ninety groundwater
wells has a water-level record* — against an endpoint that had already been dead for seven months.
Nothing in the entry said so, because the two services this corpus did use, `site` and `peak`, both
answered. See [a live url is not a live file](../decisions/a-live-url-is-not-a-live-file.yml).

**The replacement is an OGC API on a different host and it answers the question in one call.**
`https://api.waterdata.usgs.gov/ogcapi/v0/` serves thirty-seven collections as GeoJSON, of which
four matter here:

    monitoring-locations       every site, with aquifer, well depth, altitude, construction date
    field-measurements         discrete readings, incl. groundwater level, filterable by county
    peaks                      annual peak streamflow, the successor to the `peak` service
    aquifer-codes              what a code like `350SLRN` or `N400SLRDVN` is called

Filtering is by query parameter — `county_code=003&state_code=39&site_type_code=GW` returns this
county's ninety wells, and adding `parameter_code=72019` to `field-measurements` returns every
depth-to-water reading ever taken in them. The legacy service could not have been asked that
question this way; it needed a site list first.

**The site list has moved by one and the classes are the same.** The new service gives 185 sites
where the old gave 184, with the same shape: 90 groundwater, 31 lake, 29 stream, 19 agricultural,
13 atmospheric, 2 atmospheric-other, 1 spring. A count this corpus published as 184 is now 185 and
neither number is wrong; the file gained a site.

**Its groundwater readings come in three rows, not one.** Every visit is published three times —
parameter `72019` as depth to water below land surface, `62610` as the level above NGVD29, and
`62611` above NAVD88 — so a naive count of rows triples the number of measurements. This corpus
counts visits by `72019` and takes elevations from `62611`. [verified] — the collection, read for
this county.

**What it will not answer.** Anything about water quality, which is a different service; anything
about flood stage or the elevation at which a river leaves its banks, which is the National Weather
Service's; and how much water flows in an ordinary year, which is the daily-values service and was
not fetched. Peak flow is the highest instant of each year and nothing else. Nor does it say *why*
a well was drilled or a reading taken: it carries an observing procedure and a measuring agency and
no project.
