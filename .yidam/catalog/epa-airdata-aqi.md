---
name: Annual AQI by county (EPA AirData)
description: >-
  Forty-five years of what the air over Allen County actually measured, day by day, reduced to one
  row a year: how many days were monitored, how many were good, how many reached unhealthy, and
  which pollutants the monitors were watching. Measurement, where the release inventory is
  estimate.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://aqs.epa.gov/aqsweb/airdata/annual_aqi_by_county_2024.zip
    description: >-
      One zipped CSV a year, every year from 1980 to 2024, roughly 20 KB each and every county in
      the country in each. Allen County, Ohio has a row in all forty-five. The columns read here
      are `Days with AQI`, `Good Days`, `Moderate Days`, the four unhealthy columns, `90th
      Percentile AQI`, `Median AQI`, `Max AQI` and the `Days <pollutant>` columns.
used-by:
  - ../corpus/measure/allen-county-air-quality-1980-2024.yml
---

**The denominator is the number of days a monitor reported, and it changes.** Allen County has 31
monitored days in 1980, around 210 through the 1980s and 1990s, and 360-odd from 2010. A count of
good days is therefore not comparable across the series and a share of monitored days is.
[verified] —
[the annual county files](https://aqs.epa.gov/aqsweb/airdata/annual_aqi_by_county_2024.zip),
`Days with AQI`, 1980 through 2024.

**The break at 2010 is a monitor arriving, not the air changing.** Before 2010 this county's AQI
is an ozone measurement, and ozone is monitored in season; from 2010 a PM2.5 monitor reports
year-round, and the monitored days go from 232 to 365 in one step. The count of good days barely
moves — 189 to 194 — while the year it is counted out of nearly doubles, so the share falls from
81.5 per cent of monitored days to 53.2. That reads as a collapse and is a second pollutant being
watched. [verified] — same file, `Days Ozone` and `Days PM2.5`.

**The percentile columns are computed over whatever was monitored that year**, so the 90th
percentile before and after 2010 are summaries of different mixtures and the comparison across
that line is weaker than it looks. This corpus reads the percentile within an era and the
unhealthy-day share across the whole. [inference] — from the file's own structure.

**Most of Ohio is not measured at all.** 37 of the state's 88 counties have a row in the 2024 file;
46 did in 2000. A county rank computed here is a rank among the monitored, and which counties those
are changes from year to year. [verified] — same file, rows with `State` of Ohio.

**A day is attributed to one pollutant — the one that set the index that day** — so `Days Ozone`
and `Days PM2.5` are a partition of the monitored days and not a count of days each was measured.
[verified] — same file; the pollutant columns sum to `Days with AQI` in all forty-five Allen County
rows, as do the six severity columns, which is how that partition was confirmed rather than
assumed.
