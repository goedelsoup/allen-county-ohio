---
name: Audit Search (Auditor of State of Ohio)
description: >-
  Every audit the state has released on a public body in Allen County since 1999 — 1,260 reports on
  118 distinct bodies, each with its entity type, report type, audited period, release date and a
  downloadable PDF. It is the only source in this corpus that describes how the county's governments
  keep their books rather than what they spend, and the only one that names a public body because
  money went missing from it.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 180
location:
  - kind: url
    value: https://ohioauditor.gov/auditsearch/Search.aspx
    description: >-
      An ASP.NET WebForms page, so the search is a POST carrying `__VIEWSTATE`,
      `__VIEWSTATEGENERATOR` and `__EVENTVALIDATION` back from a GET of the same page. The county
      filter is `ddlCounty=Allen`; the three "all" dropdowns must be sent with their exact option
      text — `All Fiscal Years`, `All Release Months`, `All Release Years` — and sending `All Months`
      instead returns HTTP 500. One request returns every row for the county unpaginated, 2 MB.
  - kind: url
    value: https://ohioauditor.gov/auditsearch/detail.aspx?ReportID=941547cb-c03b-46f7-9a1f-bd2e6d02c24d
    description: >-
      The detail page for one report, linked from every result row. The PDF behind it is not an
      `href`: the download is `__doPostBack('lbReport','')` against the same URL, which returns
      `application/pdf` with the filename in `Content-Disposition`. This one is the Village of Fort
      Shawnee's fiscal emergency declaration, 11 pages.
used-by:
  - ../corpus/measure/allen-county-audits-1999-2026.yml
  - ../corpus/measure/allen-county-findings-for-recovery-1999-2026.yml
  - ../corpus/event/fort-shawnee-declared-in-fiscal-emergency-2010.yml
---

**An asterisk on an entity name means a finding for recovery, and the legend is the only place the
file says so.** 62 of the 1,260 rows carry one. The `cbxFindingsForRecovery` checkbox on the search
form returns nothing at all when it is sent, so the asterisk in the result table is the working
filter. [verified] — the result table and its footnote, "* Denotes Findings for Recovery".

**Counting distinct entities from this file overstates them, and by a lot.** The entity column holds
1,260 strings resolving to 141 spellings and 118 bodies: a report type is sometimes appended in
quotation marks to the name — `Village of Fort Shawnee "Fiscal Emergency Analysis - Declaration"` —
and four bodies appear under two spellings each, differing by a comma, a slash, an `Inc.` or the
word *County*. [verified] — the same file, normalised here. A naive count gives 174 and it is wrong.

**Its universe is not a universe of governments.** 25 of the 118 are Medicaid providers — private
companies audited because they take public money — and the list also holds two political parties, a
college foundation, a cemetery association, a placement agency and five special improvement
districts. [verified] — the same file, by entity type. Against the
[Census of Governments](census-of-governments-unit-lists.md), which counts 45 governments in this
county, the two are answering different questions and neither is wrong.

**And it is cumulative where the census is a snapshot.** The eight villages here include Fort
Shawnee, whose electors surrendered its corporate powers in November 2012; the three colleges
include Lima Technical College, which is Rhodes State College under an earlier name. [verified] — the same file
against [the government unit lists](census-of-governments-unit-lists.md). A body that has ceased to
exist does not leave this list, because the list is of reports and not of bodies.

**The release date is not the audited period and the two can be eight years apart.** The City of
Lima Municipal Court's special audit covers 1 January 2009 to 30 June 2012 and was released on
4 June 2020. [verified] — the same file. Any series built on this source must say which of the two
dates it is counting by; this corpus counts by release, because that is the date the public could
read it.

**What the search does not carry.** No amount. The finding-for-recovery flag is a yes or no, and the
sum is inside the PDF. [verified] — the same file. Reading 62 PDFs to total this county's findings
is possible and has not been done; three have been read and are cited where they are used.
