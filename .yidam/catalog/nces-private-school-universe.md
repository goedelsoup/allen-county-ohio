---
name: Private School Universe Survey (NCES)
description: >-
  The federal government's biennial roll of every private school in the country, with each
  school's name, town, county, religious affiliation and enrolment. It names the ten private
  schools in Allen County, which no source in this corpus had ever done, and it is the first
  file here whose roster of institutions is rebuilt from scratch every cycle.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 730
location:
  - kind: url
    value: https://nces.ed.gov/surveys/pss/zip/pss2122_pu_csv.zip
    description: >-
      The 2021–22 public-use file, 4.2 MB zipped, 22,345 schools and 459 columns. Allen County is
      `PSTABB` = `OH` with `PCNTY` = `003`; the composite `PCNTY22` is county-then-state, so this
      county reads `00339` rather than the `39003` every other file in this corpus uses.
      Enrolment is `NUMSTUDS`, the school's name `PINST`.
  - kind: url
    value: https://nces.ed.gov/surveys/pss/zip/pss1920_pu_csv.zip
    description: >-
      2019–20, and the reason the county's series cannot be differenced. The same four earlier
      editions were taken — `pss1314`, `pss1516`, `pss1718` and `pss1920` — giving five readings
      two years apart. The column names are not stable across them: 2015–16 lower-cases every
      header, so a reader that matched `NUMSTUDS` exactly found nothing and said so.
  - kind: url
    value: https://nces.ed.gov/surveys/pss/pssdata.asp
    description: >-
      The index of editions. It lists five public-use files back to 2013–14 and nothing after
      2021–22; `pss2223` and `pss2324` both return 404, so this survey's latest word on Allen
      County is four years old.
used-by:
  - ../corpus/measure/allen-county-private-schools-2013-2021.yml
  - ../corpus/question/why-one-child-in-five-is-not-in-these-districts.yml
---

**It is a universe survey and not a sample, which is a different promise from the one this corpus
usually reads.** There are no margins of error here of the kind
[the American Community Survey](census-acs-summary-file.md) publishes; the file intends to be every
private school there is. What it has instead is a **frame**, rebuilt each cycle from state lists,
religious-association lists and an area search — and the frame moves. [verified] — the programme's
own description.

**A fifth of Ohio's private schools turn over between two editions.** 185 of the 937 Ohio schools
in the 2019–20 file are absent from 2021–22, and 215 schools appear in 2021–22 that were not in
2019–20. [verified] — the two files, matched on name and town here. Statewide those two churns
roughly cancel and Ohio's private enrolment *rises* 5.6 per cent, from 145,882 to 154,033. At
county grain nothing cancels: Allen County's total falls 46.9 per cent over the same two editions,
and three schools account for 96 per cent of the fall. See
[a frame is not a panel](../decisions/a-frame-is-not-a-panel.yml).

**Two of the three are schools this county still has.** Delphos St John's, at 603 pupils the
largest private school ever recorded here, and Temple Christian School in Lima at 237, are absent
from the 2021–22 edition — not moved to another county, but absent from Ohio's file entirely, under
any spelling of either name. [verified] — a search of all 978 Ohio rows. **This file does not say
they closed and this corpus does not say it either.** A school missing from an edition is missing
from that edition's frame, which is a fact about the survey. [inference]

**It carries a duplicate the corpus could not resolve.** *The Center for Autism and Dyslexia* in
Lima appears as two records in both 2019–20 and 2021–22 — in 2019–20 with identical enrolments of
78 and 78, in 2021–22 with 62 and 59. Two campuses or one school listed twice; nothing in the file
distinguishes them, and every total here is given as published rather than de-duplicated.
[verified]

**What it does not carry.** No home instruction, which in Ohio is registered with the resident
district and not with any school. No pupil's county of residence — a school's county is where the
building is, so a child who crosses a county line to a private school is counted in the wrong one.
And no community schools: those are public and live in
[the Common Core of Data](nces-common-core-of-data.md).
