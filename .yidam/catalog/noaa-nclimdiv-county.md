---
name: NOAA nClimDiv, county monthly temperature and precipitation
description: >-
  The National Centers for Environmental Information's gridded climate record, aggregated to every
  county in the United States, monthly, from January 1895. It is the first source in this corpus
  that says what the weather is here, and the only one that says it for a hundred and thirty-one
  complete years.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 180
location:
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/cirs/climdiv/
    description: >-
      The directory. Filenames carry a version and a release date and the release date moves every
      month, so the URL must be read off the listing rather than remembered. The release taken here
      is `v1.0.0-20260806`.
  - kind: url_template
    value: https://www.ncei.noaa.gov/pub/data/cirs/climdiv/climdiv-{element}cy-v1.0.0-20260806
    description: >-
      `{element}` is `tmpc` average temperature, `tmax` maximum, `tmin` minimum, `pcpn`
      precipitation, `hddc` heating degree days, `cddc` cooling degree days. Six files were taken,
      39 MB each, every county in the country in every one of them.
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/cirs/climdiv/county-readme.txt
    description: The record layout and the state-code table. 8 KB, and it is not optional — see below.
used-by:
  - ../corpus/measure/allen-county-precipitation-1895-2025.yml
  - ../corpus/measure/allen-county-temperature-1895-2025.yml
---

**What it is, and what it is not.** It is not a weather station. County values are area-weighted
averages of a 5 km grid interpolated from Global Historical Climatology Network daily station data,
with bias adjustments for changes in observation time, station location and instrumentation. So the
figure for Allen County in 1901 is a reconstruction of the county's average from whatever stations
were reporting near it, not a reading anyone took at a courthouse. The file's own documentation
says so, and this corpus repeats it because a county-labelled number invites the other reading.

**Two things in the layout that will silently produce a wrong county.** The county files are
**eleven** characters wide in their first field where every other file in the same directory is ten,
because the two-digit state code and the three-digit county FIPS do not fit in the same space as a
climate-division number. And the state code is **not** the state FIPS: NOAA numbers the states
alphabetically, so **Ohio is 33**, not 39. Allen County's records begin `33003`. Reading `39003`
returns nothing and reading a ten-character key returns the wrong county.

    columns  1-2   state code (01-50, alphabetical, Ohio = 33)
             3-5   county FIPS
             6-7   element (01 precipitation, 02 average temperature,
                   25 heating degree days, 26 cooling degree days,
                   27 maximum temperature, 28 minimum temperature)
             8-11  year
            12-95  twelve monthly values, seven characters each

Missing months are `-99.99` for temperature and `-9.99` for precipitation, and the current year is
mostly missing, which is why every figure this corpus takes is computed over complete calendar
years only.

**Three controls were run before anything was written from it.**

The first is arithmetic and it is the strongest available: the average-temperature file and the
maximum and minimum files are separate downloads, and `(tmax + tmin) / 2` should be `tmpc`. Across
Allen County's 1,572 complete monthly cells it is, to within a rounding step — thirteen cells differ
by more than 0.05 °F and the largest disagreement in the whole record is 0.10. Three files that were
built and served independently agree cell by cell. [verified]

The second is structural: the Ohio block holds **88 counties and 132 years**, which is the number of
counties Ohio has and the number of years from 1895 through 2026. [verified]

The third is geographic. Allen's 1991–2020 mean is 51.19 °F; Auglaize next door is 51.15 and Hancock
51.13, while Hamilton County at the Ohio River is 54.37 and Cuyahoga on the lake is 50.82. Neighbours
agree to two hundredths of a degree and the state's north-south gradient is three and a half
degrees, which is what a gridded product should look like and what a mis-keyed county would not.
[verified]

**What it does not carry.** No daily values, no extremes, no snowfall, no wind, and nothing below
the county line. For a storm it is the wrong instrument entirely — that is
[NOAA's storm events database](noaa-storm-events.md), which this corpus already holds and which
begins in 1950. For what the rivers did with the rain, see
[USGS peak streamflow](usgs-nwis.md).
