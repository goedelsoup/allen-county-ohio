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
  - kind: url
    value: https://waterservices.usgs.gov/nwis/dv/?format=rdb&sites=04187500&startDT=&endDT=&parameterCd=00060
    description: >-
      Daily mean discharge, one row per day per site. This is the file the peak service is a summary
      of, and it answers the questions the annual row cannot — how many floods a year held, and how
      broad a crest was. `startDT`/`endDT` here are real filters, unlike the collection search's.
  - kind: url
    value: https://waterservices.usgs.gov/nwis/site/?format=rdb&countyCd=39003&seriesCatalogOutput=true&outputDataTypeCd=dv&parameterCd=00060&siteStatus=all
    description: >-
      Which sites carry a daily discharge series and between which dates, one row per series. Ask
      this before assuming a gauge has only annual peaks, and before assuming this county's rivers
      are the sites already cited: it is what found the Auglaize near Kossuth, which no node of this
      corpus had read.
  - kind: url
    value: https://api.water.usgs.gov/nldi/linked-data/nwissite/USGS-04189000/basin
    description: >-
      The drainage basin above a gauge, as GeoJSON, in one request and with no site list first. This
      is the check that a gauge measures the ground a claim is about: the Blanchard-at-Findlay basin
      runs longitude −83.7533 to −83.3640 and this county's easternmost monitoring site is −83.8858,
      so the basin holds no part of Allen County. Swapping `basin` for
      `navigation/DM/nwissite?distance=500` lists what lies downstream — thirty-seven sites below
      the Ottawa at Allentown, down the Auglaize and the Maumee to Toledo, and the Blanchard is not
      one of them. See
      [a neighbour's gauge](../decisions/a-neighbours-gauge-is-not-this-countys-record.yml).
used-by:
  - ../corpus/measure/the-flood-studys-flood-history.yml
  - ../corpus/event/the-flood-of-29-june-1957.yml
  - ../corpus/event/the-flood-of-21-april-1964.yml
  - ../corpus/event/the-flood-of-14-march-1978.yml
  - ../corpus/event/the-flash-flood-at-cairo-28-may-2014.yml
  - ../corpus/event/the-flood-at-bluffton-26-april-2019.yml
  - ../corpus/event/the-flood-of-12-march-1939.yml
  - ../corpus/event/the-flood-of-13-february-1950.yml
  - ../corpus/event/the-flood-of-16-june-1946.yml
  - ../corpus/event/the-flood-of-16-june-2015.yml
  - ../corpus/event/the-flood-of-22-august-2007.yml
  - ../corpus/event/the-flood-of-28-february-2011.yml
  - ../corpus/event/the-flood-of-6-june-1947.yml
  - ../corpus/event/the-flood-of-9-february-1959.yml
  - ../corpus/event/the-ottawa-river-flood-of-1959.yml
  - ../corpus/event/the-storm-of-16-18-may-1943.yml
  - ../corpus/measure/allen-county-groundwater-1962-2026.yml
  - ../corpus/measure/allen-county-water-gauges-2026.yml
  - ../corpus/measure/allen-county-water-systems-2026.yml
  - ../corpus/measure/ottawa-river-peak-flows-1924-2025.yml
  - ../corpus/event/the-flood-of-13-14-june-1981.yml
  - ../corpus/measure/the-twenty-wettest-days-against-the-river.yml
---

**What a site is and what a record is.** The two are not the same thing and the difference is the
whole of what this source taught. Allen County has 184 monitoring sites; **six** carry an annual
peak record and **three** have ever carried a daily discharge series. A county can be thick with
instruments and thin with measurements.

**Ask the service, not the corpus.** This entry said before 2026-09-06 that two sites have a usable
peak record. Six do. The right query is the site service's own `hasDataTypeCd=pk` filter, or
`seriesCatalogOutput=true`, which returns one row per series per site with its parameter, its span
and its count; enumerating the gauges somebody had already read finds only those.

**Water years, not calendar years.** A water year runs 1 October to 30 September and is named for
the calendar year it ends in, so a peak dated 30 December 1990 belongs to water year 1991. Counting
peaks by calendar year gives some years two and some none, which is what the raw file looks like
until the convention is applied.

**Its silences are legible, and the daily record dates them better than the peak file does.** The
Allentown gauge, 1924 to 1981, is missing water years 1936 to 1938 and 1940 to 1942 — one continuous
outage, 31 December 1935 to 1 September 1943, that the water-year convention splits into two. The
Lima gauge, 1989 to 2025, is missing 2000 to 2009 exactly, which in the daily record is 22 November
1999 to 1 July 2009. A gap of that shape is a gauge that was not funded, not a river that did not
flood, and the file does not distinguish them — it simply has no row.

**The peak file is one row a water year, and the daily file underneath it has been there since
1923.** The Ottawa at Allentown carries **18,545 days of mean discharge, 1 October 1923 to 23 March
1982**; the Ottawa at Lima 10,338 days from 30 September 1988; the Auglaize near Kossuth from March
2017. [verified] — the site service with `seriesCatalogOutput=true&outputDataTypeCd=dv`, run here.
This corpus ranked that river's floods from fifty-two annual rows for four phases, and an annual row
cannot show a year's second flood or say whether a crest was broad or sharp. In water year 1959 the
difference is a month: the Blanchard's largest instantaneous reading is 11 February and its largest
daily mean is 22 January. See
[a peak is one row a year](../decisions/a-peak-is-one-row-a-year.yml).

**A peak may be a reconstruction rather than a reading, and the file says which.** `peak_cd` 7 is
"Discharge is an Historic Peak", 2 is an estimate, `Bd` is a day that is unknown or not exact. The
Ottawa's second and third largest peaks at Allentown — 6,160 cfs in March 1939 and 6,000 in May 1943
— both carry 7, both fall inside the outage above, and the 1943 one has no day. See
[the intersection, not the union](../decisions/the-intersection-not-the-union.yml).

**Across the two long records here, exactly two rows are dated to the month and no further, and
they are the two floods this corpus has spent the most on.** Allentown's 6,000 cfs of May 1943 reads
`1943-05-00` and carries `2,7,Bd` — an estimate, a historic peak, day unknown; the Blanchard near
Findlay's 22,000 cfs of March 1913, the largest in ninety-nine water years, reads `1913-03-00` with
`7,Bd`. 151 rows across the two files and 2 incomplete dates. [verified] — both peak files, counted
here. The 1943 one is now dated to a day or two by a newspaper and a rain gauge; see
[the storm of 16–18 May 1943](../corpus/event/the-storm-of-16-18-may-1943.yml) and
[an instrument dates it and a witness describes it](../decisions/an-instrument-dates-it-and-a-witness-describes-it.yml).
1913 has no such witness and this corpus has looked.

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

**A site's county code is a location and not a catchment, and the network service knows the
difference.** Site 04189000 is `county_cd` 063 — Hancock — and HUC 04100008, where this county's
gauges are 04100007. Its basin polygon, one request to the network-linked service, runs west only to
longitude −83.7533 against this county's easternmost site at −83.8858, and downstream-main navigation
from the Ottawa at Allentown returns thirty-seven sites without it among them. **The Blanchard and
the Ottawa are sibling tributaries of the Auglaize and neither carries the other's water.**
[verified] — the basin and navigation services, run here. Four nodes of this corpus cite 04189000 and
every one names Findlay in the same sentence, so none is in error; what the basin service settles is
that a gauge's county code cannot stand in for the ground it measures. See
[a neighbour's gauge](../decisions/a-neighbours-gauge-is-not-this-countys-record.yml).

**What it will not answer.** Anything about water quality, which is a different service; anything
about flood stage or the elevation at which a river leaves its banks, which is the National Weather
Service's; and how much water flows in an ordinary year, which is the daily-values service and was
not fetched. Peak flow is the highest instant of each year and nothing else. Nor does it say *why*
a well was drilled or a reading taken: it carries an observing procedure and a measuring agency and
no project.
