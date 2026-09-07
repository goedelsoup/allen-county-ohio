---
name: NOAA National Water Prediction Service — gauge records
description: >-
  The National Weather Service's register of river forecast points. For each gauge it publishes the
  four flood categories in feet — action, minor, moderate, major — a list of impact statements
  describing in prose what happens at a given stage, the historic crests it has on file, and whether
  inundation mapping is enabled. It is the only federal source that says what a stage *means*, and
  so the only one that turns a discharge into a flood.
type: dataset
obtained: true
retrieved: 2026-09-06
ttl_days: 365
location:
  - kind: url
    value: https://api.water.noaa.gov/nwps/v1/gauges/FTJO1
    description: >-
      One gauge, by its five-character NWS location id. `FTJO1` is the Auglaize River at Ft.
      Jennings, in Putnam County, and it is the gauge whose minor-flood impact statement names
      Allen County. The same path with the id changed serves any forecast point.
  - kind: url
    value: https://api.water.noaa.gov/nwps/v1/gauges/ORLO1
    description: >-
      The Ottawa River at Lima — USGS 04187100, the only NWS gauge inside Allen County. It answers
      200 with every flood category set to the sentinel −9999, an empty impact list, a null
      record-of-observation block and inundation mapping disabled. A gauge with no thresholds is
      not an error and must not be read as one.
  - kind: url
    value: https://api.water.noaa.gov/nwps/v1/gauges?bbox.xmin=-84.5&bbox.ymin=40.5&bbox.xmax=-83.8&bbox.ymax=41.0
    description: >-
      A bounding-box query over the gauge register. It returns `{"gauges": []}` for a box covering
      Allen County and its margins, because the one gauge inside the county carries no thresholds
      and the query returns forecast points. An empty array here is a statement about thresholds,
      not about instruments.
used-by:
  - ../corpus/event/the-flood-of-22-august-2007.yml
  - ../corpus/event/the-flood-of-28-february-2011.yml
  - ../corpus/event/the-flood-of-16-june-2015.yml
  - ../corpus/measure/ottawa-river-peak-flows-1924-2025.yml
---

**What this source adds that USGS does not.** The USGS peak file says a river carried 5,700 cubic
feet a second on a day. It does not say whether that was a flood. This file carries the four
thresholds a forecast office set for the gauge and, at most of them, a sentence describing what the
water reaches at that stage — "Water begins to cover State Route 115 near the south village limits
of Kalida, and the road is closed." That sentence is the bridge between a discharge and a flood, and
this corpus had no source for it before.

**A threshold is a local administrative judgement, not a physical constant.** On the Auglaize at Ft.
Jennings, minor flood stage is 13 feet and 50 of the gauge's 97 annual peaks that carry a height
stand between 13 and 16; a further 24 are between 16 and 20. Three peaks in four reach minor flood
stage, and the impact statement at 13 feet is about farm land. Read as "how often does this river
flood", that produces nonsense; read as "at what stage does water leave the channel onto the
lowest ground", it is exactly right. The category names are the NWS's own and carry the NWS's
meaning.

**The sentinel is −9999 and it is not a stage.** Every category on a gauge with no thresholds reads
`{"stage": -9999, "flow": -9999}`. A consumer that treats that as a number will find every river in
the country permanently above major flood stage.

**It speaks in stage where USGS speaks in flow, and at one gauge here the two rank differently.**
The Ottawa River at Lima's highest stage on record is 21.20 feet on 16 June 2015 and its highest
discharge is 5,700 cubic feet a second on 28 February 2011, which stood at 20.81. On a single stable
rating a higher stage cannot carry less water, so either the rating changed between the two or one
peak is mis-rated, and **which event was "largest" depends on which quantity is asked for**. Nothing
fetched here settles it. See
[the intersection, not the union](../decisions/the-intersection-not-the-union.yml).

**Its stages are on each gauge's own datum, and an old one may be on a superseded datum.** A crest
carrying `olddatum: true` is not comparable with modern readings even at the same gauge.

**What it does not have.** Allen County. The county contains one forecast point and that point has
no thresholds, no impacts and no inundation mapping, so every question of the form *was this a
flood in Allen County* has to be answered from a gauge outside it — which is a fact about the
county's instrumentation and is recorded as one rather than worked around.
