---
name: NOAA Storm Events Database
description: >-
  The National Weather Service's record of every severe weather event in the United States since
  1950, one row per event per county or forecast zone, published by NCEI as one gzipped CSV per
  year. It carries the event type, the begin and end times, the county, the reporter, the
  magnitude, the deaths, the injuries, the damage, the tornado's F-scale and track, and — from
  1996 — a narrative written by the forecast office.
type: dataset
obtained: true
retrieved: 2026-09-06
ttl_days: 180
location:
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/swdi/stormevents/csvfiles/
    description: >-
      The bulk directory. Seventy-seven detail files, `StormEvents_details-ftp_v1.0_d<year>_c<compiled>.csv.gz`,
      one per year 1950 through 2026, 300 MB compressed for the whole country. There is no county
      filter and no API: the whole run is downloaded and filtered locally on `STATE` = OHIO and
      `CZ_NAME` = ALLEN, which yields 390 rows. The `c` in the filename is the compile date and
      changes when a past year is corrected, so a cached file is not a stable citation.
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/swdi/stormevents/csvfiles/Storm-Data-Bulk-csv-Format.pdf
    description: >-
      The field dictionary, and the only part of the documentation that survives as text — the
      database's own landing page is a JavaScript shell that returns nothing to a fetch. It
      reproduces the event-type table of NWS Directive 10-1605, which assigns each of the 48
      permitted event types a County or Zone designator.
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/swdi/stormevents/csvfiles/StormEvents_fatalities-ftp_v1.0_d1965_c20260323.csv.gz
    description: >-
      The companion fatality file, keyed on `EVENT_ID`, with an age, a sex and a location code per
      death. Fetched for 1965 and it does not carry Allen County's eleven: the year holds eighteen
      fatality rows and none of them belongs to this county's event.
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/swdi/stormevents/csvfiles/StormEvents_locations-ftp_v1.0_d2006_c20260707.csv.gz
    description: >-
      The companion locations file, one per year on the same naming scheme as the details, keyed on
      `EVENT_ID` with a `LOCATION_INDEX`, a range and azimuth from a named place, and a latitude and
      longitude. Fetched for 1972 and for every year 1996 through 2026 — the whole of its non-empty
      run — and joined to this county's rows. What that found is below.
used-by:
  - ../corpus/event/the-tornado-of-11-april-1965.yml
  - ../corpus/event/the-tornado-of-19-july-1950.yml
  - ../corpus/event/the-tornadoes-of-april-1965.yml
  - ../corpus/event/the-windstorm-of-22-june-2006.yml
  - ../corpus/measure/allen-county-disaster-declarations-1965-2020.yml
  - ../corpus/measure/allen-county-storm-events-1950-2026.yml
  - ../corpus/measure/allen-county-tornadoes-1950-2026.yml
  - ../corpus/office/allen-county-sheriff.yml
---

**What a row is.** An event, not an episode and not a storm. One thunderstorm crossing this county
on 22 June 2006 produced nine Allen County rows sharing an `EPISODE_ID`, because nine separate
reports of damage reached the forecast office. The count is of reports that were written up.

**Its coverage begins at different dates for different weather.** In this county's file the first
tornado is 1950, the first thunderstorm wind 1955, the first hail 1959 — and then nothing new for
thirty-seven years, until four more event types first appear in 1996 and nine more after. A zero
before a category's first year is not a quiet year; it is a category that did not yet exist. See
[a category has a birthday](../decisions/a-category-has-a-birthday.yml).

**Its county and zone rows are different geometries.** `CZ_TYPE` is C or Z, and the directive fixes
which by event type: tornado, hail, thunderstorm wind, flash flood, lightning and heavy rain are
reported by county, while winter storm, heavy snow, high wind, ice storm, blizzard and cold are
reported by forecast zone. All 390 Allen County rows obey it — 287 county rows and 103 zone rows,
and no event type appears under both designators. Ohio's zones follow county lines, so both name
"ALLEN", but a zone row is a statement about a forecast area and not about ground.

**Who writes it.** Of the 293 rows carrying a `SOURCE`, 240 name a person — trained spotters,
emergency managers, law enforcement, broadcast media, amateur radio operators, newspapers, the
public — and 43 name an instrument. Eight are NWS storm surveys. This is a record made by the
county's own residents telephoning the forecast office, which is why its density tracks the number
of people willing to telephone.

**Its damage figures are estimates and its old ones are round.** 1950 and 1965 both read exactly
`2.5M`. The 2006 storm's four Lima rows read 50K, 200K, 1.6M and blank. Nothing here is an audited
loss, and the currency is nominal dollars of the year of the event with no deflator in the file.

**Its coordinates changed meaning about 2010.** Every tornado from 1998 to 2006 has an identical
begin and end coordinate against a stated track length of a tenth of a mile, on a five- or
sixtieth-of-a-degree grid: those are the nearest town's position, not the tornado's. Every tornado
from 2010 on has begin and end coordinates whose great-circle distance reproduces the stated track
length to two decimals. The field is the same; what fills it is not.

**What it cannot be read as.** A climate series, a hazard rate, or evidence of absence. Allen
County's count rises from four events in the 1950s to 121 in the 2010s, monotonically across seven
decades, and its tornado count — the one category counted throughout — goes 1, 2, 4, 3, 2, 3, 4
and does not. The 1978 blizzard that brought this county a federal emergency declaration is not in
this file at all.

**Its locations file gives this county nothing its detail rows do not already carry.** The
companion `StormEvents_locations` file, keyed on `EVENT_ID` with one row per position and a
`LOCATION_INDEX` to order them, is header-only for every year before 1996 — the sole exception is
1972, whose two rows are a Florida event under the placeholder identifier 990000001 — so neither
the 1950 tornado nor the 1965 one draws anything from it. From 1996 on it places 233 of this
county's 315 rows, and **every tornado and every thunderstorm wind among them carries one position
or two, never three.** Where it carries two, they are the begin and end the detail file already
states. This entry said before 2026-09-06 that the file gives a tornado's intermediate track
points; that is true of the file and not of this county, whose longest tornado in it ran 3.87 miles
and got two.

**Its only multi-point Allen rows are floods, and what they carry is an extent.** Six of them at
four and five positions — flash floods of May 2011 (two), May 2014 and July 2020, floods of June
2015 and April 2019 — and read as a path they are nonsense. Event 593454 runs from 1.25 miles WNW
of Elida through Needmore, Scotts Crossing and Gomer to 0.51 miles E of Elida, back beside where it
began; event 908785 is four positions inside four-tenths of a mile of Cairo. They outline where the
water stood. See [an extent is not a track](../decisions/an-extent-is-not-a-track.yml).

**What else is in it, unread.** The fatality file for every year but 1965; the episode narratives
for every event but the handful quoted here; and every other county in Ohio, already on the disk in
the same download.
