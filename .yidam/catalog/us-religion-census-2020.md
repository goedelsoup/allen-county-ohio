---
name: 2020 U.S. Religion Census (ASARB)
description: >-
  The Association of Statisticians of American Religious Bodies' decennial count of congregations
  and adherents for every county in the United States, by religious group. It is the first source
  in this corpus about religion in Allen County, and the only one that counts what a hundred and
  sixty congregations amount to rather than telling the story of one of them.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 3650
location:
  - kind: url
    value: https://www.usreligioncensus.org/sites/default/files/2023-06/2020_USRC_Group_Detail.xlsx
    description: >-
      Group by nation, state, county and metro. The county sheet is 80,682 rows — one per group per
      county that reports any — with congregations, adherents, and adherents as a share of total
      adherents and of population. Allen County has 49 of them.
  - kind: url
    value: https://www.usreligioncensus.org/sites/default/files/2023-06/2020_USRC_Summaries.xlsx
    description: >-
      The totals sheet, one row per county, with population, congregations, adherents, congregations
      per 100,000 and adherents as a share of population, each with its national rank among 3,146
      counties.
used-by:
  - ../corpus/measure/allen-county-congregations-2020.yml
---

**Its "adherents" are what religious bodies report, and the bodies do not all count the same
thing.** A denomination that counts baptized infants and a denomination that counts confessing
adults over eighteen both appear in this file in one column headed adherents. The compilers
estimate for some groups — the county's Muslim figure is labelled "Muslim Estimate" in the group
name itself — and take others as filed. Nothing here is a survey of individuals; it is a survey of
organizations about their membership. [verified] — the group names and file structure.

**Nine of Allen County's 160 congregations report no adherents at all.** Community of Christ, the
Pentecostal Church of God and the United Pentecostal Church International report one congregation
each and a blank; the Evangelical Free Church of America and the General Association of Regular
Baptist Churches report three each and a blank. The reverse also happens: the Baha'i Faith USA
reports 23 adherents and no congregation. A congregation count and an adherent count are separate
returns and a group may make one and not the other. [verified] — every Allen County row read.

**The county population it prints is the 2020 census count exactly.** 102,206, which is the figure
this corpus already holds from
[the redistricting file](census-2020-redistricting-file.md), so the share-of-population column can
be checked and is not a modelled denominator. [verified] — compared.

**A share of population is not a rate of belief and not a rate of attendance.** 57.4 per cent of
Allen County appears in these returns. The remaining 42.6 per cent are people no religious body
reported, which includes those belonging to a body that did not file, those in a body outside the
compilers' frame, and those in none. The file does not distinguish them and neither does this
corpus. [verified] — the file's own definitions.

**The xlsx has no CSV counterpart.** Both files are OOXML and were read with the standard library —
`zipfile` for the archive, `xml.etree` for `xl/worksheets/sheet*.xml` and `xl/sharedStrings.xml` —
because no spreadsheet library is installed here. The sheet a caller wants is not the first: the
county data is on `2020 County Summary` and `2020 Group by County`, resolved through
`xl/_rels/workbook.xml.rels`. [verified] — the retrieval as performed.
