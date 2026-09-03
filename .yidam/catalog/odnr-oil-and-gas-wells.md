---
name: Oil and gas wells of Ohio (ODNR Division of Oil and Gas Resources Management)
description: >-
  Every oil or gas well the state of Ohio has a record of, by API number, with its status, its
  operator, its township, its wellhead coordinate and — for a small minority — its depth, its permit
  date, its completion date and the date it was plugged. Allen County has 4,849 of them, which is
  the first quantity this corpus has held about the field that made the place.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://gis.ohiodnr.gov/arcgis/rest/services/DOG_Services/Oilgas_Wells_public/MapServer/0/query
    description: >-
      The public map service, layer 0, *Wells by Type (Symbol)*. Filtering `WL_CNTY='ALLEN'` and
      paging 1,000 at a time returns 4,849 rows with wellhead latitude and longitude, township,
      operator, orphan-well status and a map symbol. Every one of them is vertical.
  - kind: url
    value: https://gis.ohiodnr.gov/arcgis/rest/services/DOG_Services/ogwells_offline/FeatureServer/0/query
    description: >-
      The feature service behind it, which the map service does not expose. It carries
      `PERMIT_ISSUED_DATE`, `COMP_DATE`, `PLUG_DATE`, `TOTAL_DEPTH` and a different status
      vocabulary. It has no county column; `API_NO LIKE '34003%'` is the filter, and it returns
      4,834 rows.
used-by:
  - ../corpus/measure/allen-county-oil-and-gas-wells-2026.yml
---

**The two services disagree about the status of three wells in five.** Of the 4,831 wells in both,
2,933 — 60.7 per cent — carry different status words. The largest single disagreement is 2,821 wells
the map service calls *Unknown status* and the feature service calls *Historical Production Well*.
[verified] — the two services joined on API number here. The map service also calls 45 wells *Permit
Expired* and 25 *Expired Permit*, two spellings of one thing, where the feature service calls all 70
*Not Drilled*. See [the legend is not the record](../decisions/the-legend-is-not-the-record.yml).

**Almost nothing in it is dated.** A permit date exists for 153 of 4,834 Allen County wells, a
completion date for 105 and a plug date for 166 — 3.2, 2.2 and 3.4 per cent. [verified] — the
feature service. Of the 1,715 wells recorded as plugged and abandoned, 84 carry the date they were
plugged.

**The register does not reach the field it records.** The earliest completion date in it for this
county is 1890 and this corpus dates the strike to 9 May 1885; thirty wells in the county carry a
completion date before 1920 and the boom drilled thousands. [verified] — the feature service against
[the 1906 history](miller-allen-county-1906.md). What survives is the well, not the paperwork.

**Dates are epoch milliseconds and some are negative.** `PLUG_DATE` of `-542160000000` is 1952.
[verified] — the same service. A client that treats the field as seconds, or as unsigned, puts the
nineteenth century in the twenty-first.

**Depth is the tell and it is present for 290 wells.** The median is 1,332 feet and 230 of the 290
fall between 1,200 and 1,399. [verified] — the same service. Only 43 wells in the county name a
producing formation and 39 of those name the Trenton Limestone.

**The counts differ by nineteen and neither service is a superset.** 18 wells appear only in the map
service and 1 only in the feature service. [verified] — the two joined here. Any total from this
source has to say which service it came from.

**The operator column is a single value for 98 per cent of them.** 4,761 of 4,849 read
`HISTORIC OWNER`, and the 88 that do not are shared among 29 named companies. [verified] — the map
service. It is a placeholder and not a company, and a count of operators taken from it would give
this county thirty.
