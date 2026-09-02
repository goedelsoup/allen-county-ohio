---
name: Common Core of Data (NCES), via the Urban Institute Education Data API
description: >-
  The federal roll of every public school in the country, read through the Urban Institute's
  API rather than from NCES's own files. It is here for one thing the state's own enrolment
  figures cannot show: which of Allen County's public schools are community schools, and how
  many children are in them.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://educationdata.urban.org/api/v1/schools/ccd/directory/2022/?fips=39
    description: >-
      All 3,695 Ohio public schools for 2022, paginated. `county_code` is a proper five-digit FIPS
      here — `39003` — and `charter` is 1 or 0. **The query parameter `county_code` is accepted and
      ignored**: adding it returns all 3,695 rows unchanged, so the filter was applied after the
      download and not by the server.
  - kind: url
    value: https://educationdata.urban.org/api/v1/api-endpoints/
    description: >-
      The endpoint index, 129 of them. It is how this corpus established that the API serves the
      Common Core of Data from 1986 and **does not serve the Private School Universe Survey at
      all**, which is why that one was taken from
      [NCES directly](nces-private-school-universe.md).
used-by:
  - ../corpus/measure/allen-county-private-schools-2013-2021.yml
  - ../corpus/question/why-one-child-in-five-is-not-in-these-districts.yml
---

**Why an intermediary rather than the agency.** The Urban Institute republishes NCES's files under
one schema with one pagination scheme, and the agency's own downloads are per-year fixed-width
archives with a layout document each. For a question that needed one column — is this school a
community school — the intermediary is the shorter road, and the figures below can be checked
against NCES whenever the answer stops being one column. [inference] This is the same standing as
[OpenElections](openelections-ohio.md): a compiler, useful, and not the publisher of record.

**Two community schools stand in Allen County, and they hold 313 children.** Heir Force Community
School with 238 and West Central Learning Academy II with 75, both open in 2022, out of 36 public
schools located in the county. [verified] — the 2022 directory, filtered here.

**And that count is the wrong shape for the question it was fetched for.** Ohio's largest community
schools are statewide online schools, and this file locates a school at its administrative address.
Ohio Virtual Academy enrols 14,334 children from a building in Wood County; Alternative Education
Academy 5,783; Ohio Connections Academy 5,380. Every one of those pupils lives somewhere, some of
them here, and none of them is in the 313 above. [verified] — the same directory. A county-located
count of community schools is a count of buildings and not of children. [inference]
