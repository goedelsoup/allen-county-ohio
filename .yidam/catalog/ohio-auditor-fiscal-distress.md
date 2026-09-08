---
name: Local Governments in Fiscal Distress (Auditor of State of Ohio)
description: >-
  Every Ohio local government the state has placed in fiscal emergency since the law was written in
  1979, with the statutory condition that put it there, the date it was declared, the date it was
  released and how long that took — and the current fiscal watch and fiscal caution lists beside it.
  It is the only source in this corpus that says when a state supervision of a local government's
  finances ended, which the audit search cannot: a termination is a determination and not a report.
type: dataset
obtained: true
retrieved: 2026-09-08
ttl_days: 180
location:
  - kind: url
    value: https://ohioauditor.gov/fiscal/_docs/LocalGov_FD_Chapter_118.pdf
    description: >-
      The report itself, 14 pages, 386 KB, dated 28 July 2026 and reissued on a schedule the page
      calls "Updated July 2026". Four tables: 15 governments currently in fiscal emergency, 75 whose
      emergencies have terminated, and the current fiscal watch and fiscal caution lists. Rows carry
      county, population, whether the analysis came `By` request or by the Auditor's own initiative,
      the dollar figure under each statutory condition, and for the terminated table two dates and a
      duration in years. Earliest declaration in the file is 1980.
  - kind: url
    value: https://ohioauditor.gov/fiscal/local.html
    description: >-
      The page the report hangs off, and a source in its own right: it sets out the declaring
      conditions for caution, watch and emergency, the termination test for each, and the membership
      of a Financial Planning and Supervision Commission by class of government and by population
      threshold. The commission roster is here and in no dataset.
  - kind: url
    value: https://ohioauditor.gov/fiscaldistress.html
    description: >-
      The parent page, which links `fiscal/local.html`, `fiscal/schools.html` and
      `fiscal/receivership.html`. School districts are a separate statute and a separate list.
used-by:
  - ../corpus/event/fort-shawnee-declared-in-fiscal-emergency-2010.yml
---

**This host answers 200 for a page that does not exist.** `ohioauditor.gov/there-is-no-such-page-here.html`
returns HTTP 200 and a 34,330-byte body whose text says the page was not found, and so does any
other wrong path under any directory. [verified] — three requests, one good path and two invented.
A fetch here is checked by reading the body or by its length, never by its status code. This is the
second host in two phases to do it — see [the Lima charter](city-of-lima-official-site.md), where
Municode returns 200 for a city that does not exist — and the two are unrelated vendors, so the
habit is the web's and not one publisher's.

**A termination has a date here and nowhere else in this corpus.** The
[audit search](ohio-auditor-of-state-audit-search.md) carries the Village of Fort Shawnee's fiscal
emergency *declaration* as a released report and carries nothing for its release, because ending a
fiscal emergency is a determination the Auditor makes and not a document the Auditor publishes as an
audit. [verified] — the two sources, searched for the same entity. A corpus that asks only the audit
search will conclude that an emergency never ended.

**The population column decides how the government was supervised, not just how big it was.** Under
Chapter 118 a village or township of fewer than a thousand people gets no commission at all: the
Auditor of State serves as Financial Supervisor with all of a commission's powers. At a thousand or
more, three further members are appointed within fifteen days of the declaration. [verified] — the
`fiscal/local.html` page, commission membership. Two governments with identical books and different
populations are under different institutions, and the report's population column is the only place
the file says which.

**The condition columns are a summary and are not the declaration's own schedules.** The report
gives Fort Shawnee $4,040 of accounts payable where the certified analysis gives $761 for the
general fund; the statute has an all-funds test and a general-fund test and the summary column does
not say which it is reporting. [verified] — the report against the declaration itself. Where the two
disagree the declaration is the record and this file is the index to it.

**It is a list of governments and not of distress.** A government appears once it has been declared
and stays for good, so the terminated table is a history and the other three are snapshots. Reading
the caution or watch lists for "has this county ever" answers a question they were not asked; only
the emergency list has a past tense. [inference]
