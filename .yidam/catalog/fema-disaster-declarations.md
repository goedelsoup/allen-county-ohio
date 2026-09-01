---
name: OpenFEMA Disaster Declarations Summaries
description: >-
  Every federal disaster and emergency declaration since 1953, one row per declaration per county,
  from the Federal Emergency Management Agency's open API. It carries the declaration date, the
  incident window, the incident type, and which of the three assistance programs the county was
  designated for. It is the corpus's first source of dated singular happenings after 1955.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 180
location:
  - kind: url
    value: "https://www.fema.gov/api/open/v2/DisasterDeclarationsSummaries?$filter=state eq 'OH' and fipsCountyCode eq '003'"
    description: >-
      Allen County's rows, ten of them, 9.6 KB of JSON. No key and no registration. The filter is
      OData; `fipsCountyCode` is the three-digit county code within `fipsStateCode` 39, and
      `designatedArea` reads "Allen (County)" on every row, which is the check that the filter did
      what was intended.
used-by:
  - ../corpus/event/allen-county-declared-for-covid-19-2020.yml
  - ../corpus/event/allen-county-declared-for-hurricane-katrina-2005.yml
  - ../corpus/event/the-tornado-of-11-april-1965.yml
  - ../corpus/event/the-tornadoes-of-april-1965.yml
  - ../corpus/measure/allen-county-disaster-declarations-1965-2020.yml
  - ../corpus/measure/ottawa-river-peak-flows-1924-2025.yml
---

**What a row is.** Not a disaster — a **declaration**, which is a presidential act taken on a state
governor's request after a damage assessment. A county appears in this file when it is named in
such a declaration, and it appears once per declaration, so one storm can produce two rows and one
row can name a county that suffered nothing.

**Its two declaration types are different instruments.** `EM` is an emergency declaration, which is
faster and narrower; `DR` is a major disaster declaration. Allen County has four of the first and six
of the second, and the June 2012 storm produced one of each seven weeks apart.

**Its three program flags are what the money was for.** `iaProgramDeclared` is Individual
Assistance — aid to households; `paProgramDeclared` is Public Assistance — aid to state and local
governments for debris and public works; `hmProgramDeclared` is Hazard Mitigation. Only two of Allen
County's ten declarations carried Individual Assistance, in 1965 and 2007.

**Its incident dates are not reliable for old records.** The 1965 tornado declaration and the 1978
blizzard declaration both give an incident that began and ended on the day the declaration was
signed. A tornado outbreak and a blizzard did not last zero days; those fields were filled with the
declaration date. Every record from 2004 on carries a real window. See
[a file has more than one date](../decisions/a-file-has-more-than-one-date.yml), which this file is
the second instance of in two phases.

**What it cannot be read as.** A hazard series. Allen County has two declarations in the thirty-nine
years to 2004 and eight in the sixteen after, and the far likelier explanation is that the federal
programme changed — in what it covers, in the threshold for designating a county, and in how readily
governors request — than that north-west Ohio's weather changed by a factor of ten. The file
measures an administrative act.

**What else is in it, unread.** The declaration request date and number, the closeout date, the
region, and a `designatedIncidentTypes` code. The API also serves related endpoints — public
assistance funded projects, individual assistance by county, hazard mitigation grants — which would
put dollars beside these dates and were not fetched.
