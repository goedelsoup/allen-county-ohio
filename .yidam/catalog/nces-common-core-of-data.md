---
name: Common Core of Data (NCES), via the Urban Institute Education Data API
description: >-
  The federal roll of every public school in the country, read through the Urban Institute's
  API rather than from NCES's own files. It is here for one thing the state's own enrolment
  figures cannot show: which of Allen County's public schools are community schools, and how
  many children are in them.
type: dataset
obtained: true
retrieved: 2026-09-03
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
    value: https://educationdata.urban.org/api/v1/school-districts/ccd/enrollment/2024/grade-99/race/?fips=39
    description: >-
      District enrolment by race, one request per year, 1986 to 2024. Ohio is 9,432 rows in 2023 —
      1,048 districts by nine race codes — and arrives unpaginated in 800 KB. The `{grade}` segment
      must be written `grade-99`; the bare `99` the endpoint index shows returns HTTP 500.
  - kind: url
    value: https://educationdata.urban.org/api/v1/school-districts/ccd/directory/2023/?fips=39
    description: >-
      The district directory, 1,069 Ohio rows with 70 columns — teachers, counsellors,
      psychologists, librarians and administrators as full-time equivalents, plus special-education
      and English-learner counts and the number of schools. None of it is in the finance survey.
      **Now taken as a series**, one request per year from 1986 to 2024: 39 files, 797 Ohio rows in
      the first and 1,073 in the last, unpaginated. The staffing columns are empty before 1992 and
      the count of schools is populated from 1986.
  - kind: url
    value: https://educationdata.urban.org/api/v1/api-endpoints/
    description: >-
      The endpoint index, 129 of them. It is how this corpus established that the API serves the
      Common Core of Data from 1986 and **does not serve the Private School Universe Survey at
      all**, which is why that one was taken from
      [NCES directly](nces-private-school-universe.md).
used-by:
  - ../corpus/measure/allen-county-school-staffing-1992-2024.yml
  - ../corpus/measure/allen-county-school-buildings-1986-2024.yml
  - ../corpus/measure/allen-county-private-schools-2013-2021.yml
  - ../corpus/measure/allen-county-school-enrolment-1988-2024.yml
  - ../corpus/measure/allen-county-school-enrolment-by-race-1988-2024.yml
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

**Its year is the fall of a school year, and the finance survey's is not.** This file's `year`
column for enrolment names the autumn the children were counted; the
[school system finance survey](census-school-system-finances.md) labels the same count by the
fiscal year that autumn opens. The two therefore differ by one, and it is not a rounding difference
or a coverage difference: across fourteen years the finance survey's figure for Allen County's
twelve districts equals this file's figure for the **preceding** year in fourteen years of fourteen
and in twelve districts of twelve at both ends of the run, and equals its same-year figure in none.
[verified] — the two sources, compared here. See
[a file has more than one date](../decisions/a-file-has-more-than-one-date.yml).

**"Two or more races" opens in 2010 and holds 1,182 of this county's children in its first year.**
Before 2010 the category is absent from every Ohio district row; from 2010 it is populated and by
2024 it holds 1,514 of Allen County's 15,850 pupils, 711 of them in Lima City alone. [verified] —
the annual files. Every race share differenced across that boundary is differenced across a change
in the roll; see [a category has a birthday](../decisions/a-category-has-a-birthday.yml) and
[a revision that changes the roll](../decisions/a-revision-that-changes-the-roll.yml).

**It reaches back to 1986 and has now been read as a series twice.** The endpoint index was
consulted once, to establish which schools were community schools; then the district enrolment
series was taken year by year; and the directory beside it — the same thirty-nine files, carrying
teachers, aides, counsellors, librarians and administrators — was taken after that. [verified] —
the same index this entry already cites. Two readings of one file, a day apart, for two
questions that were both in it the first time; see
[a source is not exhausted by one question](../decisions/a-source-is-not-exhausted-by-one-question.yml).

**Its staffing has one whole year missing and one year missing two districts.** Every staffing
column reads `-1` for all twelve of Allen County's districts in 2014 — a sentinel and not a zero,
with the enrolment and school counts either side of it intact. In 2022 Bath and Lima report no
staff at all, so a county total for that year covers 68.44 per cent of the county's children and
reads like a collapse. [verified] — the annual directory files, filtered here. See
[a zero is not a blank](../decisions/a-zero-is-not-a-blank.yml).

**Its counsellor components are zeroed and its counsellor total is not.** Every Allen County
district reports `0` elementary and `0` secondary guidance counsellors in 2024 and a total of 43
between them. The total is the figure that can be used and the split cannot. [verified] — the 2024
directory.

**Its largest staffing column has no name.** `support_staff_other_fte` holds 1,114 of the 2,938
full-time equivalents the twelve districts report in 2024 — 37.92 per cent, more than teachers,
aides, counsellors, librarians and administrators of every grade combined. The file itemises the
professions it has an interest in and puts the rest of the payroll in one cell. [verified] — the
same directory. The total column itself begins in 2015, so the file cannot be asked how many people
worked in these schools before then. [verified] — the annual files.

**Staffing is a full-time-equivalent count and some of its zeros are real.** Nine of the county's
twelve districts report `0.0` school psychologists in 2023 and three report a positive number; the
column is populated for every district, so these are reported zeros rather than missing cells.
[verified] — the 2023 directory. Whether a district with no psychologist of its own buys the
service from the [educational service center](../corpus/jurisdiction/allen-county-educational-service-center.yml)
is not in this file. [open]
