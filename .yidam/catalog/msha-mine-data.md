---
name: MSHA Mine Data Retrieval System — open data sets
description: >-
  Every mine and quarry in the United States that the Mine Safety and Health Administration
  regulates, with its operator, its coordinates, its quarterly employment and hours, and every
  reportable injury at it. It is how this corpus found the two holes still being worked in Allen
  County.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 180
location:
  - kind: url
    value: https://arlweb.msha.gov/OpenGovernmentData/DataSets/Mines.zip
    description: >-
      The mine master, 7.3 MB zipped, 91,985 rows, pipe-delimited with a `latin-1` encoding. One
      row per mine ever registered, with status, operator, controller, SIC, county FIPS,
      coordinates, shifts, and written directions to the gate.
  - kind: url
    value: https://arlweb.msha.gov/OpenGovernmentData/DataSets/MinesProdQuarterly.zip
    description: >-
      Employment and hours worked, one row per mine per subunit per calendar quarter, 2000 to the
      present. 55 MB zipped. `COAL_PRODUCTION` is empty for a limestone quarry, so this file gives
      labour and not tonnage.
  - kind: url
    value: https://arlweb.msha.gov/OpenGovernmentData/DataSets/Accidents.zip
    description: >-
      Every accident, injury and illness reported under Part 50, 52 MB zipped, with the degree of
      injury, the classification, the occupation, and the days lost and restricted.
  - kind: url
    value: https://arlweb.msha.gov/OpenGovernmentData/DataSets/ControllerOperatorHistory.zip
    description: >-
      Who controlled and operated each mine, and between which dates. 169,890 rows.
used-by:
  - ../corpus/measure/allen-county-quarries-2000-2025.yml
  - ../corpus/site/national-lime-and-stone-lima-quarry.yml
---

**It is a register of regulated operations, not a map of excavation.** A mine leaves this file when
its operator stops filing, and the hole stays where it is. Allen County's four abandoned entries
were abandoned in 1984, 1984, 1987 and 2009; nothing here says whether anything remains at those
places or what is in them now. [inference] See
[a register is not a census](../decisions/a-register-is-not-a-census.yml).

**No API key, no guestbook, no user-agent filter.** The zip files are served directly from
`arlweb.msha.gov`, which is the agency's own legacy host. [verified] — the four retrievals here.

## The date that means "before the file"

**A quarter of the controller-history rows begin on 1 January 1950, and nothing in the file begins
earlier.** 41,309 of 169,890, and 16,038 of the mine master's 91,985 rows carry the same date in
`CURRENT_CONTROLLER_BEGIN_DT`. [verified] — the two files, counted here. It is a floor rather than a
day: read as a date it says that a quarter of American mining changed hands on one New Year's
morning.

**The next two commonest dates are also administrative.** 1 January 1979 appears 1,702 times and
**9 March 1978** — the day the Federal Mine Safety and Health Act of 1977 took effect — appears 916.
[verified] — the same file. See
[a value shared by unrelated records is a default](../decisions/a-value-shared-by-unrelated-records-is-a-default.yml).

**Its injury measure is not OSHA's, and its denominator is.** MSHA collects reportable injuries
under 30 CFR Part 50 and this corpus computes the rate on 200,000 hours, the same base
[the OSHA summaries](osha-injury-tracking-application.md) use — so the two rates are comparable in
their arithmetic and rest on two agencies' definitions of what must be written down. A quarry rate
and a nursing home rate stand beside each other here with that said. [inference]
