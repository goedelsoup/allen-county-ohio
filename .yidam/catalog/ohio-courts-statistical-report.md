---
name: Ohio Courts Statistical Report (Supreme Court of Ohio)
description: >-
  Every case filed, terminated and left pending in every Ohio court, county by county and court by
  court, reported to the Supreme Court by the courts themselves. It is the first source this corpus
  holds that says what work the county's own courts do.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://www.supremecourt.ohio.gov/courts/judicial-system/supreme-court-of-ohio/reports-publications/
    description: >-
      The publications index, which links every annual report from 1987 and every statistical
      summary from 1999.
  - kind: url
    value: https://www.supremecourt.ohio.gov/docs/Publications/annrep/17OCSR/2017OCSR.pdf
    description: >-
      *Ohio Courts Statistical Report for 2017*, 293 pages, 2.2 MB. One row per county for each of
      the four common pleas divisions and one row per court for the municipal and county courts,
      across overall caseloads and every case type. The eleven detailed reports for 2007 through
      2017 follow this shape; the file is `{YY}OCS.pdf` under `annrep/{YY}OCS/` for 2007–2013 and
      `{YY}OCSR.pdf` under `annrep/{YY}OCSR/` for 2014–2017.
  - kind: url
    value: https://www.supremecourt.ohio.gov/docs/Publications/annrep/20OCSR/2020OCS.pdf
    description: >-
      *2020 Ohio Courts Statistical Summary*, 68 pages. The 2018, 2019 and 2020 releases are
      summaries: charts, maps and statewide totals, with **no county tables at all**. The detailed
      report stops at 2017.
used-by:
  - ../corpus/measure/allen-county-court-caseloads-2007-2017.yml
  - ../corpus/measure/allen-county-foreclosures-and-evictions-2007-2017.yml
---

**It is the courts reporting on themselves, on a form the Supreme Court writes.** Rule 37 of the
Rules of Superintendence requires every court to file monthly and annual case statistics; this
report is the compilation. A filing here is a case opened on a docket, not a dispute in a county,
and a court that changes how it counts changes the series without anything changing in the county.
[inference]

**The PDFs carry a text layer and the tables survive `pdftotext -layout` intact.** Each county or
court is one line, columns aligned, with the population the court serves in the second field. Allen
County appears once per table under `Allen`; its municipal court appears under `Lima`. [verified] —
the eleven reports, read here.

**The Lima Municipal Court serves the whole county.** Its population column reads 108,473 through
2009 and 106,331 from 2010 — the county's 2000 and 2010 census counts, unchanged in between — so
its caseload is the county's caseload and not a city's. [verified] — the same reports against
[the census](census-2020-redistricting-file.md).

## Two checks it passes

**The pending balances chain across eleven separately published documents.** For forcible entry and
detainer at Lima, every year's *cases pending December 31* equals the next year's *cases pending
January 1*, in all ten transitions from 2007 to 2017. [verified] — the eleven reports. That is a
test of the compiler rather than of the county; see
[arithmetic that closes is about the compiler](../decisions/arithmetic-that-closes-is-about-the-compiler.yml).

**Its eviction counts match a second publisher's almost exactly.** For 2008, 2011 and 2014 the new
forcible-entry filings here — 982, 922 and 863 — are the same figures to the unit that
[the Eviction Lab](eviction-lab-county-data.md) publishes for this county; 2017 differs by four,
802 against 806. [verified] — both sources. Both chains begin at the same court's docket, so what
this establishes is that neither pipeline mangles the count; see
[an exact match is a question](../decisions/an-exact-match-is-a-question.yml).

**What it is not.** It is not a record of crime, of debt, or of housing loss: it counts cases
brought. It is not comparable across a court's own reorganisations without checking the judge
column. And it has no county detail after 2017, so the most recent year it can describe is nine
years old.
