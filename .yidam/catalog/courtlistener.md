---
name: CourtListener (Free Law Project)
description: >-
  A search index over American published opinions, federal and state, with an open API that needs
  no key for search. It is the first source in this catalog that finds a place by looking for it in
  the reports rather than in a register, and the first that measures an institution by the number
  of times people had to sue it.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://www.courtlistener.com/api/rest/v4/search/?q=%22Lima+State+Hospital%22&type=o&order_by=dateFiled+asc
    description: >-
      The phrase search that returns **187 published opinions**, 21 December 1920 to 1 November
      2021, in pages of twenty. Each row carries the case name, the filing date, the court, every
      citation, the docket number, the judges, counsel, and a syllabus — but not the opinion.
  - kind: url
    value: https://www.courtlistener.com/api/rest/v4/opinions/1370571/
    description: >-
      **This endpoint answers 401 without a token, and no token was obtained.** Registering for one
      is a form, and the corpus took the opinions it needed from
      [the Caselaw Access Project](caselaw-access-project.md) instead. The public web page for the
      same opinion answers HTTP 202 with an empty body to this client, which is a challenge and was
      also not worked around.
used-by:
  - ../corpus/measure/lima-state-hospital-in-the-reports-1920-2021.yml
  - ../corpus/site/lima-state-hospital.yml
---

**What the phrase search finds, by decade.** [verified] — the query above, counted here.

    1920s   8      1970s  46
    1930s  10      1980s  22
    1940s  15      1990s   8
    1950s  28      2000s  10
    1960s  39      2020s   1

**And by court.** Ohio's court of appeals 101, the Ohio Supreme Court 37, the Sixth Circuit 13, the
Northern District of Ohio 6, the Southern District 3, three courts of common pleas 6, two probate
courts 2, and one each from Oklahoma and Mississippi. [verified] — same source, counted here.

**Eight of the 101 appellate opinions carry this county's own docket form.** Ohio's Third District
numbers a case with a county code first and Allen County's is 1; eight of the 101 open `1-`, from
*Burton v. Reshetylo* in 1973 to *State ex rel. AFSCME v. Taft* in 2004, and two of them —
*Holderbaum v. Watkins* and *Wolonsky v. Balson* — name the same superintendents as the federal
class action. The remaining ninety-three carry Hamilton County's `C-`, Franklin's `06AP-`,
Mahoning's `07 MA`, or a bare sequence number. An institution standing in Allen County generated
most of its litigation in other people's counties. [verified] — the docket numbers, parsed here;
the reading is this corpus's. [inference]

**Four names in sequence, and each date is a floor.** The first published appearance of each name
in this index is: Lima State Hospital 21 December 1920, Lima Correctional Institution 22 February
1988, Oakwood Forensic Center 9 August 1989, Allen Correctional Institution 3 March 1992, Oakwood
Correctional Facility 25 February 2002. [verified] — five phrase searches, counted here. A first
appearance in the reports is not a founding date: litigation reaches publication years after the
events it is about, so each of these bounds the name's existence from one side only.

**It rate-limits an anonymous client and says so.** Paging the 187 results returned HTTP 429 twice
and succeeded on retry with a delay. Requests here are spaced by a second and carry a user agent
naming this corpus. [verified] — the responses themselves.
