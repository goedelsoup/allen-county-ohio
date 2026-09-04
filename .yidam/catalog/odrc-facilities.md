---
name: Ohio Department of Rehabilitation and Correction, facilities page
description: >-
  The operating agency's own list of its institutions. Almost all of it is loaded by JavaScript and
  invisible to a plain client; the one table that is in the served HTML is the list of closed
  facilities, and that is the table this corpus needed.
type: reference
obtained: true
retrieved: 2026-08-31
ttl_days: 180
location:
  - kind: url
    value: https://drc.ohio.gov/about/facilities
    description: >-
      Returns 358 KB of HTML to a browser user-agent. The current institutions are rendered
      client-side and are not in it. The closed-facilities list is.
used-by:
  - ../corpus/site/allen-correctional-institution.yml
  - ../corpus/question/who-lives-in-the-county-without-housing.yml
  - ../corpus/measure/allen-county-shortage-designations-1985-2026.yml
---

**What it gave, in one line.** The page carries a list headed "The following previously served as
correctional facilities but have since closed", and in it:

> Ohio State Reformatory | Closed 1990 · Orient Correctional Institution | Closed 2002 · **Lima
> Correctional Institution | Closed 2004** · Montgomery Education and Pre-Release Center | Closed
> 2004 · Hocking Correctional Facility | Closed 2016

Allen County's address file lists `LIMA CORRECTIONAL INSTITUTE` at 2350 N West St alongside two
institutions that still operate. It has been carrying a prison that closed twenty-two years ago,
which is the kind of thing a county's own file is least likely to notice about itself and a state
agency states in five words.

**The retrieval trap, recorded so the next phase does not repeat it.** `drc.ohio.gov` returns HTTP
200 and a full-looking page to `curl` with a browser user-agent, and the page contains no
institution names, no addresses, no populations and no links to any — every one of those is
injected by scripts. Searching the served HTML for "Allen" or "Oakwood" returns only the closed
list. The site's `/reports` and `/reports/institution-reports` paths behave the same way and expose
no PDF links at all. This is a different failure from the 403s this corpus has met before: nothing
is refused, and what arrives is empty in a way that reads as an answer.

**What would be needed instead.** The department's monthly institution population report, which is
published as a PDF and whose URL is not discoverable from the served HTML; or the facility roll in
its annual report. Both are named here so a later phase can look for them by another route. What
this corpus has in the meantime is better for the question it was asking anyway:
[the Census Bureau's landmark file](census-tiger-landmarks.md) names the institutions and locates
them, and [the redistricting file](census-2020-redistricting-file.md) counts the people in them.
