---
name: GHCN-Daily, Lima and Van Wert co-operative stations
description: >-
  The daily weather record of one station inside Allen County, kept since 1901, and a second
  station twenty-five miles west that exists here to check it. Every source of weather this corpus
  held before was monthly, divisional or event-driven; this is the first that records what happened
  on a named day, and it is the only one that separates snow from rain.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 180
location:
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/ghcn/daily/all/USC00334551.dly
    description: >-
      LIMA WWTP, the city's water pollution control plant, at 40.7247, -84.1294 and 259.1 m. Daily
      maximum and minimum temperature, precipitation, snowfall and snow depth from 1 April 1901 to
      26 August 2026 — 45,601 days carrying at least one observation, of which 45,022 carry a
      maximum temperature. 2.4 MB, fixed-width.
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/ghcn/daily/all/USC00338609.dly
    description: >-
      VAN WERT 1 S, 40.5 km west in Van Wert County, from 1893. Not in Allen County and not used
      for any figure about it — it is here to answer one question the Lima station cannot answer
      about itself, which is whether its own warming is the city it stands in.
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/ghcn/daily/readme.txt
    description: >-
      The format and the flags. Units are the thing to get right: temperature in tenths of degrees
      Celsius, snowfall and snow depth in whole millimetres, and **precipitation in tenths of a
      millimetre** — a factor of ten between two columns of the same record.
  - kind: url
    value: https://www.ncei.noaa.gov/pub/data/ghcn/daily/ghcnd-inventory.txt
    description: >-
      Which element each station holds and between which years. It is what found the two stations
      above out of 132,501, and it is not a statement that the years between are populated.
used-by:
  - ../corpus/measure/allen-county-frost-free-season-1902-2025.yml
  - ../corpus/measure/allen-county-temperature-extremes-1901-2026.yml
  - ../corpus/measure/lima-snowfall-1901-2025.yml
  - ../corpus/event/the-ottawa-river-flood-of-1959.yml
---

**Six hundred and seventy-two values were dropped before anything was computed, and the file said
which.** Every observation carries a quality flag, and 672 of about 206,000 in the Lima record
carry a non-blank one — 599 failing an internal consistency check, thirteen a spatial one, and one
a bounds check. That last is 6 March 1930, where the snowfall column reads 4,079 millimetres, or a
hundred and sixty inches in a day. Read without the flag it makes the winter of 1929–30 the
snowiest in the county's history by a factor of three. [verified] — the file's own `QFLAG` column,
2 September 2026.

**The station agrees with the divisional record this corpus already holds, on five figures that
were computed from different data.** Its 1991–2020 annual mean is 51.37 °F against the county
division's 51.19; its 1996–2025 mean is 51.64 against 51.63; its mean daily maximum for June to
August 1934 is 88.9 °F against 88.8; its driest year is 1963 at 22.48 inches against 22.70; and its
wettest is 2011 at 54.49 against 52.56. One thermometer in one yard and an average over an entire
climate division are not the same instrument, and on the county's warmest, driest and wettest years
they do not disagree. [verified] — computed here against
[the county's temperature](../corpus/measure/allen-county-temperature-1895-2025.yml) and
[its precipitation](../corpus/measure/allen-county-precipitation-1895-2025.yml).

**The station is in a city, and that is a reason to distrust it about frost.** A minimum
temperature taken inside Lima is the one measurement here most exposed to the warmth of the ground
it stands on, and the frost-free season is computed entirely from minima. Van Wert 1 S — a
different county, a town a third of Lima's size, forty kilometres away — is in this entry for that
reason and no other. [inference]

**What it does not contain.** No station history: whether the Lima gauge moved, changed observer
or changed observation hour across a hundred and twenty-five years is not in this file, and a
change in the hour of observation is known to shift daily maxima and minima. The corpus reads this
record for thresholds and dates rather than for tenths. [verified] — the file has no metadata
beyond a single coordinate.

**And it contains a hole that looks like data.** From the 1978–79 season through 2005–06 the Lima
station reported snowfall on two days out of twenty-eight winters, while reporting hundreds of days
of precipitation falling at or below 34 °F; with the seasons on either side that are short of
observations, thirty consecutive winters drop out. Those seasons are not snowless; they are unmeasured,
and the file records the absence as a column of zeros rather than as blanks. [verified] — the
record itself; see [a zero is not a blank](../decisions/a-zero-is-not-a-blank.yml).
