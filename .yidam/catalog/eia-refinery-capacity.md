---
name: EIA Refinery Capacity Report
description: >-
  The Energy Information Administration's annual census of every operating US refinery — its
  operating company, its corporate parent since 2010, and its capacity unit by unit, as of 1 January.
  It is the source that closes a question this corpus has carried since genesis: who operates the
  Lima refinery now, and who has operated it since.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://www.eia.gov/petroleum/refinerycapacity/refcap26.xlsx
    description: >-
      The current report, as of 1 January 2026. 3,336 rows: one per refinery per unit per capacity
      measure. Allen County is `SITE` LIMA, `STATE_NAME` Ohio.
  - kind: url
    value: https://www.eia.gov/petroleum/refinerycapacity/archive/2005/refcap05.xls
    description: >-
      The archive pattern — `archive/<year>/refcap<yy>.xls` for 1994 to 2023, `.xlsx` for 2024 and
      2025. Thirty-one reports were taken. 1996 and 1998 are not published and were not found.
---

**It names the operator of the Lima refinery in every report from 1994.** BP Oil Corp. through
1997; Clark Refining & Marketing in 1999 and 2000; Premcor Refg Group Inc from 2001 through 2007;
Lima Refining Company from 2008 to now. [verified] See
[the capacity series](../corpus/measure/lima-refinery-capacity-1994-2026.yml) and
[the refinery](../corpus/site/lima-refinery.yml), whose ownership chain had been `[open]` since
genesis.

**It names the operating company, which is not the owner, and the file says so by its own silence.**
`COMPANY_NAME` is the entity that files the survey. A refinery can change hands without changing
that name, and this one did: the county's own address file labels the site "VALERO LIMA REFINERY"
while EIA never prints the word Valero in any year of any report. [verified] —
[Allen County GIS](allen-county-gis-downloads.md). So a change in this column dates a change of
filing entity, and an unchanged column is not evidence that nothing happened. [inference]

**The `CORPORATION` column exists only from the 2010 report.** Before that the survey did not
collect a parent, so for 1994 to 2009 this source can see a change of operator and cannot see a
change of owner behind an unchanged one. From 2010 it reads Husky Energy Inc, then Cenovus Energy
Inc in 2022, then Cenovus Marketing (USA) Inc from 2023. [verified]

**Two report years are missing and one of them matters.** 1996 and 1998 are absent from the archive.
The BP-to-Clark transition falls in the 1998 gap, so this source bounds it to after the 1997 report
and no later than the 1999 one, and cannot date it. [verified]

**What it gives besides ownership.** Atmospheric crude distillation capacity in barrels per calendar
day and per stream day, and charge or production capacity for every downstream unit — cat cracking,
hydrocracking, reforming, coking, vacuum distillation, four desulfurization streams, isomerization,
aromatics, sulfur in short tons. Thirty-seven rows for Lima in the current report.

**What else is in it, unread.** The same for all 124 US refineries, every year for thirty-one years,
with refining district and PADD. This corpus has taken one site's operator, one site's crude
capacity, and the four Ohio refineries' capacities once.
